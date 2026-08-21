#!/usr/bin/env node
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const root = process.argv[2] ? path.resolve(process.argv[2]) : path.resolve(path.dirname(new URL(import.meta.url).pathname), '../..');
const contracts = [
  'config/toolchains/contained_semantic_toolchain_inventory.json',
  'config/toolchains/source_availability_manifest.json',
  'config/toolchains/rust_bootstrap_chain.json',
  'config/toolchains/termux_android_aarch64_capacity_profile.json',
  'config/toolchains/termux_nsq_intercept_policy.json',
  'config/toolchains/source_built_build_graph.json',
  'config/toolchains/extended_repository_integration_manifest.json',
  'config/toolchains/license_report.json',
  'config/toolchains/gap_report.json',
  'config/nsq/feature_execution_registry.json',
  'config/nsq/language_functional_ingestion_matrix.json',
  'config/nsq/complete_semantic_extraction_contract.json',
  'config/nsq/semantic_corpus_manifest.json',
  'config/nsq/bionic_gnu_compatibility_matrix.json',
  'config/nsq/watermarked_file_operation_contract.json',
];

function readJson(relative) {
  const absolute = path.join(root, relative);
  if (!fs.existsSync(absolute)) throw new Error(`missing contract: ${relative}`);
  return JSON.parse(fs.readFileSync(absolute, 'utf8'));
}

const parsed = new Map(contracts.map((relative) => [relative, readJson(relative)]));
const language = parsed.get('config/nsq/language_functional_ingestion_matrix.json');
const feature = parsed.get('config/nsq/feature_execution_registry.json');
const repository = parsed.get('config/toolchains/extended_repository_integration_manifest.json');
const graph = parsed.get('config/toolchains/source_built_build_graph.json');
const semantic = parsed.get('config/nsq/semantic_corpus_manifest.json');
const extraction = parsed.get('config/nsq/complete_semantic_extraction_contract.json');
const watermark = parsed.get('config/nsq/watermarked_file_operation_contract.json');

if (language.language_total !== language.languages.length) throw new Error('language matrix total mismatch');
if (repository.repository_total !== repository.repositories.length) throw new Error('repository manifest total mismatch');
if (!feature.features.some((item) => item.id === 'feature:language.parameter_parse')) throw new Error('language intercept feature missing');
if (!feature.features.some((item) => item.id === 'feature:repository.operation')) throw new Error('repository intercept feature missing');
if (!feature.features.some((item) => item.id === 'feature:toolchain.bionic_compatibility')) throw new Error('Bionic feature missing');
if (!feature.features.some((item) => item.id === 'feature:watermark.file_operation')) throw new Error('functional watermark feature missing');
if (watermark.capability !== 'feature:watermark.file_operation' || !watermark.watermark_is_functional) throw new Error('functional watermark capability contract is invalid');
if (watermark.resident_runtime || watermark.native_execution_policy.allows_hidden_download || !watermark.native_execution_policy.requires_aarch64_android_target) throw new Error('functional watermark target boundary is invalid');
if (!graph.nodes.some((item) => item.id === 'complete_language_semantic_proofs')) throw new Error('semantic proof build-graph node missing');
if (!graph.nodes.some((item) => item.id === 'termux_nsq_calibration_and_recovery')) throw new Error('Termux calibration build-graph node missing');
if (!graph.nodes.some((item) => item.id === 'functional_watermark_file_operation')) throw new Error('functional watermark build-graph node missing');
if (semantic.corpora.length !== semantic.compaction_metrics.manifested_compact_artifact_count) throw new Error('semantic corpus count mismatch');
for (const corpus of semantic.corpora) {
  const absolute = path.join(root, corpus.path);
  if (!fs.existsSync(absolute)) throw new Error(`semantic corpus path is missing: ${corpus.path}`);
  const bytes = fs.readFileSync(absolute);
  if (bytes.length !== corpus.bytes) throw new Error(`semantic corpus byte count mismatch: ${corpus.path}`);
  const sha256 = crypto.createHash('sha256').update(bytes).digest('hex');
  if (sha256 !== corpus.sha256) throw new Error(`semantic corpus hash mismatch: ${corpus.path}`);
}
if (extraction.required_semantic_surfaces.length < 17) throw new Error('complete semantic surface contract is incomplete');
for (const item of repository.repositories) {
  if (item.nsq_capability !== `repository:${item.id}`) throw new Error(`noncanonical repository capability: ${item.id}`);
}
console.log(JSON.stringify({
  schema: 'braxon.toolchain.contract_validation.v1',
  valid: true,
  language_total: language.language_total,
  repository_total: repository.repository_total,
  semantic_surface_total: extraction.required_semantic_surfaces.length,
  compact_corpus_total: semantic.corpora.length,
  functional_watermark_routes: watermark.compiler_routes.length,
}, null, 2));
