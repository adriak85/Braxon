#!/usr/bin/env node
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const root = process.argv[2]
  ? path.resolve(process.argv[2])
  : path.resolve(path.dirname(new URL(import.meta.url).pathname), '../..');
const manifestPath = path.join(root, 'config/nsq/semantic_corpus_manifest.json');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));

for (const corpus of manifest.corpora) {
  const absolute = path.join(root, corpus.path);
  if (!fs.existsSync(absolute) || !fs.statSync(absolute).isFile()) {
    throw new Error(`semantic corpus artifact is absent or non-file: ${corpus.path}`);
  }
  const bytes = fs.readFileSync(absolute);
  corpus.bytes = bytes.length;
  corpus.sha256 = crypto.createHash('sha256').update(bytes).digest('hex');
}
manifest.compaction_metrics.manifested_compact_artifact_count = manifest.corpora.length;
manifest.compaction_metrics.manifested_compact_bytes = manifest.corpora.reduce(
  (total, corpus) => total + corpus.bytes,
  0,
);
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`);
console.log(JSON.stringify({
  schema: 'braxon.nsq.semantic_corpus_manifest_refresh.v1',
  valid: true,
  corpus_total: manifest.corpora.length,
  compact_bytes: manifest.compaction_metrics.manifested_compact_bytes,
}, null, 2));
