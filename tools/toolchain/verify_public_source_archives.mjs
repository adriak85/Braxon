#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { readdir, readFile, stat } from 'node:fs/promises';
import { basename, join, resolve } from 'node:path';

const root = resolve(process.argv[2] ?? process.cwd());
const manifestPath = join(root, 'config/toolchains/source_availability_manifest.json');
const manifest = JSON.parse(await readFile(manifestPath, 'utf8'));

if (manifest.schema !== 'braxon.toolchain.source_availability.v1') {
  throw new Error(`unsupported source availability manifest schema: ${manifest.schema}`);
}

async function sha256Files(paths) {
  const hash = createHash('sha256');
  let bytes = 0;
  for (const sourcePath of paths) {
    for await (const chunk of createReadStream(sourcePath)) {
      hash.update(chunk);
      bytes += chunk.length;
    }
  }
  return { bytes, sha256: hash.digest('hex') };
}

async function verifyProvenance(relativePath, expectedSha256) {
  const absolutePath = join(root, relativePath);
  const provenance = JSON.parse(await readFile(absolutePath, 'utf8'));
  if (provenance.schema !== 'braxon.source_archive.provenance.v1') {
    throw new Error(`unsupported source archive provenance schema: ${relativePath}`);
  }
  if (provenance.sha256 !== expectedSha256) {
    throw new Error(`provenance hash mismatch: ${relativePath}`);
  }
  return relativePath;
}

const checks = [];
for (const source of manifest.sources) {
  const evidence = source.materialization_evidence;
  if (!evidence?.clone_contained_archive) continue;
  const archive = evidence.clone_contained_archive;
  const archivePath = join(root, archive.path);
  const result = await sha256Files([archivePath]);
  const provenance = await verifyProvenance(archive.provenance, archive.sha256);
  if (result.bytes !== archive.bytes || result.sha256 !== archive.sha256) {
    throw new Error(`archive identity mismatch: ${source.id}`);
  }
  checks.push({
    source_id: source.id,
    representation: 'single_archive',
    paths: [archive.path],
    bytes: result.bytes,
    sha256: result.sha256,
    provenance,
    valid: true,
  });

  if (evidence.nested_llvm_archive) {
    const nested = evidence.nested_llvm_archive;
    const chunkDirectory = join(root, nested.chunk_directory);
    const partSuffix = '.part';
    const chunks = (await readdir(chunkDirectory))
      .filter((entry) => entry.endsWith(partSuffix))
      .sort()
      .map((entry) => join(chunkDirectory, entry));
    if (chunks.length !== nested.chunk_count) {
      throw new Error(`nested LLVM chunk count mismatch: expected ${nested.chunk_count}, got ${chunks.length}`);
    }
    const largestChunkBytes = Math.max(...await Promise.all(chunks.map(async (chunk) => (await stat(chunk)).size)));
    if (largestChunkBytes > nested.maximum_chunk_bytes) {
      throw new Error(`nested LLVM chunk exceeds Git-safe maximum: ${largestChunkBytes}`);
    }
    const assembled = await sha256Files(chunks);
    const nestedProvenance = await verifyProvenance(nested.provenance, nested.sha256);
    if (assembled.bytes !== nested.bytes || assembled.sha256 !== nested.sha256) {
      throw new Error(`nested LLVM reassembly identity mismatch: ${source.id}`);
    }
    checks.push({
      source_id: `${source.id}_nested_llvm`,
      representation: 'git_safe_chunk_set',
      paths: chunks.map((chunk) => chunk.slice(root.length + 1)),
      bytes: assembled.bytes,
      sha256: assembled.sha256,
      provenance: nestedProvenance,
      valid: true,
    });
  }
}

if (checks.length === 0) {
  throw new Error('no clone-contained public source archives were declared');
}

console.log(JSON.stringify({
  schema: 'braxon.toolchain.public_source_archive_verification.v1',
  valid: true,
  root,
  source_check_total: checks.length,
  checks,
}, null, 2));
