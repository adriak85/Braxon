#!/usr/bin/env node
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const root = path.resolve(process.argv[2] ?? process.cwd());
const outDir = path.join(root, 'state/full_android_language_toolchain/inventories');
const readText = (relative) => fs.readFileSync(path.join(root, relative), 'utf8');
const readJson = (relative) => JSON.parse(readText(relative));
const exists = (relative) => fs.existsSync(path.join(root, relative));
const sha256 = (value) => crypto.createHash('sha256').update(value).digest('hex');
const writeJson = (name, value) => {
  const serialized = `${JSON.stringify(value, null, 2)}\n`;
  fs.writeFileSync(path.join(outDir, name), serialized);
  return { path: `state/full_android_language_toolchain/inventories/${name}`, bytes: Buffer.byteLength(serialized), sha256: sha256(serialized) };
};
const writeText = (name, value) => {
  const serialized = value.endsWith('\n') ? value : `${value}\n`;
  fs.writeFileSync(path.join(outDir, name), serialized);
  return { path: `state/full_android_language_toolchain/inventories/${name}`, bytes: Buffer.byteLength(serialized), sha256: sha256(serialized) };
};

for (const required of [
  'config/nsq/bionic_gnu_compatibility_matrix.json',
  'config/nsq/language_functional_ingestion_matrix.json',
  'config/toolchains/source_availability_manifest.json',
  'config/toolchains/license_report.json',
  'config/toolchains/termux_android_aarch64_capacity_profile.json',
  'config/toolchains/rust_bootstrap_chain.json',
  'config/toolchains/source_built_build_graph.json',
  'scripts/toolchains/unified_android_libc_contract_overlay.sh',
  'scripts/toolchains/promote_rust_edge_nightly_aarch64.sh',
]) {
  if (!exists(required)) throw new Error(`required native inventory input is missing: ${required}`);
}

fs.mkdirSync(outDir, { recursive: true });
const matrix = readJson('config/nsq/bionic_gnu_compatibility_matrix.json');
const languages = readJson('config/nsq/language_functional_ingestion_matrix.json');
const sources = readJson('config/toolchains/source_availability_manifest.json');
const licenses = readJson('config/toolchains/license_report.json');
const capacity = readJson('config/toolchains/termux_android_aarch64_capacity_profile.json');
const rust = readJson('config/toolchains/rust_bootstrap_chain.json');
const buildGraph = readJson('config/toolchains/source_built_build_graph.json');
const overlayScript = readText('scripts/toolchains/unified_android_libc_contract_overlay.sh');
const promotionScript = readText('scripts/toolchains/promote_rust_edge_nightly_aarch64.sh');

const targetEnvironment = {
  declared_architecture: matrix.target.architecture,
  declared_platform: matrix.target.platform,
  declared_abi: matrix.target.abi,
  declared_android_api_floor: matrix.target.android_api_floor,
  controller_architecture: process.arch,
  controller_platform: process.platform,
  actual_target_probe_performed: false,
  target_build_claimed: false,
};

const sourceRoots = [
  { id: 'llvm_project', path: 'state/full_android_language_toolchain/src/llvm-project', indicators: ['llvm/CMakeLists.txt', 'clang/CMakeLists.txt', 'lld/CMakeLists.txt', 'compiler-rt/CMakeLists.txt', 'libcxx/CMakeLists.txt', 'libcxxabi/CMakeLists.txt', 'libunwind/CMakeLists.txt'] },
  { id: 'rust_stage1_source', path: 'state/full_android_language_toolchain/src/rust', indicators: ['x.py', 'compiler/rustc/Cargo.toml', 'library/std/Cargo.toml'] },
  { id: 'rust_edge_nightly_source_archive', path: 'state/full_android_language_toolchain/source_archives/rust-f7d782a3be46d6bb4b9792fe69a61db389ba1769.tar.gz', indicators: [] },
  { id: 'cpython', path: 'state/full_android_language_toolchain/src/cpython', indicators: ['configure.ac', 'Python/ceval.c', 'Modules/Setup.stdlib.in'] },
];

const sourceLikeName = (name) => /(^CMakeLists\.txt$|^Makefile(?:\.in)?$|^configure(?:\.ac)?$|^x\.py$|\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx|inc|def|td|rs|py|m|mm|s|S|asm|cmake|txt|toml|json|yml|yaml|mk|sh|in))$/i.test(name);
const scanSymbolReferences = (rootEntry, symbols) => {
  const rootAbsolute = path.join(root, rootEntry.path);
  const counts = Object.fromEntries(symbols.map((symbol) => [symbol, 0]));
  const samples = Object.fromEntries(symbols.map((symbol) => [symbol, []]));
  const summary = { root: rootEntry.path, traversed_regular_files: 0, source_or_build_files_scanned: 0, bytes_scanned: 0, non_source_or_build_files_skipped: 0, oversized_source_or_build_files_skipped: 0 };
  if (!fs.existsSync(rootAbsolute) || !fs.statSync(rootAbsolute).isDirectory()) return { ...summary, present: false, counts, samples };
  const walk = (absolute, relative) => {
    for (const entry of fs.readdirSync(absolute, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))) {
      const childAbsolute = path.join(absolute, entry.name);
      const childRelative = path.join(relative, entry.name);
      if (entry.isDirectory()) {
        walk(childAbsolute, childRelative);
        continue;
      }
      if (!entry.isFile()) continue;
      summary.traversed_regular_files += 1;
      if (!sourceLikeName(entry.name)) {
        summary.non_source_or_build_files_skipped += 1;
        continue;
      }
      const stat = fs.statSync(childAbsolute);
      if (stat.size > 16 * 1024 * 1024) {
        summary.oversized_source_or_build_files_skipped += 1;
        continue;
      }
      const content = fs.readFileSync(childAbsolute);
      if (content.includes(0)) {
        summary.non_source_or_build_files_skipped += 1;
        continue;
      }
      summary.source_or_build_files_scanned += 1;
      summary.bytes_scanned += content.length;
      for (const symbol of symbols) {
        let offset = content.indexOf(symbol);
        while (offset !== -1) {
          counts[symbol] += 1;
          if (samples[symbol].length < 8) samples[symbol].push({ path: childRelative, byte_offset: offset });
          offset = content.indexOf(symbol, offset + symbol.length);
        }
      }
    }
  };
  walk(rootAbsolute, rootEntry.path);
  return { ...summary, present: true, counts, samples };
};
const nativeSymbols = matrix.interfaces.map((item) => item.symbol);
const sourceReferenceScans = sourceRoots.filter((entry) => entry.indicators.length > 0).map((entry) => scanSymbolReferences(entry, nativeSymbols));
const observedReferenceCount = (symbol) => sourceReferenceScans.reduce((total, scan) => total + scan.counts[symbol], 0);
const observedReferenceSamples = (symbol) => sourceReferenceScans.flatMap((scan) => scan.samples[symbol].map((sample) => ({ source_root: scan.root, ...sample }))).slice(0, 16);

const toolchainInventory = {
  schema: 'braxon.native.toolchain_inventory.v1',
  generated_at: new Date().toISOString(),
  target_environment: targetEnvironment,
  source_roots: sourceRoots.map((entry) => ({
    ...entry,
    present: exists(entry.path),
    required_indicators_present: entry.indicators.map((indicator) => ({ indicator, present: exists(`${entry.path}/${indicator}`) })),
  })),
  llvm_components: ['llvm', 'clang', 'clang-tools-extra', 'lld', 'lldb', 'compiler-rt', 'libcxx', 'libcxxabi', 'libunwind', 'polly', 'bolt', 'AArch64 backend', 'ELF/linker infrastructure', 'DWARF/debugging infrastructure'],
  rust_components: ['rustc', 'cargo', 'rustdoc', 'rustfmt', 'clippy', 'compiler-builtins', 'core', 'alloc', 'std', 'proc-macro', 'target specification', 'Android platform support'],
  build_graph_nodes: buildGraph.nodes.map((node) => ({ id: node.id, depends_on: node.depends_on, proof: node.proof })),
  source_built_llvm_binding_required: promotionScript.includes('BIONIC_OVERLAY_PROOF') && promotionScript.includes('llvm-config'),
  repository_contained_source_only: !/\bcurl\b|\bwget\b|git clone|git fetch/.test(promotionScript),
  source_build_surface_reference_scans: sourceReferenceScans,
};

const missingSymbols = {
  schema: 'braxon.native.missing_symbols.v1',
  generated_at: new Date().toISOString(),
  target_environment: targetEnvironment,
  policy: 'Inventory status is source-derived. No symbol is represented as closed native until an actual target compile/link/run and downstream-consumer proof exists.',
  entries: matrix.interfaces.map((item) => {
    const sourceMentioned = overlayScript.includes(item.symbol);
    const syscallBacked = /syscall/i.test(item.aarch64_syscall_or_bridge);
    const bridgeBacked = /bridge/i.test(item.aarch64_syscall_or_bridge);
    return {
      symbol: item.symbol,
      header: item.header,
      signature_class: item.signature_class,
      interface_class: syscallBacked ? 'E_SYSCALL_BACKED_IMPLEMENTATION' : bridgeBacked ? 'G_ABI_LINKAGE_ADAPTATION' : 'D_NATIVE_IMPLEMENTATION_REQUIRED',
      source_authority: item.provenance,
      native_source_declared: sourceMentioned,
      staged_header_declared: sourceMentioned,
      overlay_route: matrix.overlay.header_generator,
      semantic_identity: item.universal_lexical,
      target_proof_requirement: item.proof_state,
      observed_source_build_reference_count: observedReferenceCount(item.symbol),
      observed_source_build_reference_samples: observedReferenceSamples(item.symbol),
      status: targetEnvironment.actual_target_probe_performed ? 'UNRESOLVED' : 'BLOCKED_TARGET',
      silent_ignore: false,
    };
  }),
};

const languageEntries = languages.languages.map((language) => ({
  id: language.id,
  family: language.family,
  target_environment: language.target_environment,
  nsq_capability: language.semantic_contract?.nsq_capability,
  reflexor_route: language.semantic_contract?.kinetic_reflexor_route,
  native_ingress: language.target_materialization?.native_ingress,
  required_local_tools: language.target_materialization?.required_local_tools ?? [],
  hidden_download_allowed: language.target_materialization?.hidden_download_allowed,
  declared_native_execution_status: language.target_materialization?.compiler_or_runtime_payload_state,
  closure_status: 'DECLARED_SEMANTIC_NOT_NATIVE_PROVEN',
}));

const contractClosure = {
  schema: 'braxon.native.contract_closure.v1',
  generated_at: new Date().toISOString(),
  target_environment: targetEnvironment,
  closure_requirement: ['source closure', 'ABI closure', 'link closure', 'runtime probe', 'downstream consumer closure', 'semantic registration', 'provenance', 'contract validation'],
  overlay_policy: {
    system_write_attempted: false,
    termux_prefix_overwrite_allowed: matrix.policy.direct_system_or_termux_prefix_overwrite_allowed,
    staged_overlay_only: matrix.policy.overlay_is_opt_in_and_staged,
  },
  interface_summary: {
    declared_total: missingSymbols.entries.length,
    target_proven_total: 0,
    blocked_target_total: missingSymbols.entries.filter((entry) => entry.status === 'BLOCKED_TARGET').length,
    silently_ignored_total: missingSymbols.entries.filter((entry) => entry.silent_ignore).length,
  },
  native_status: 'BLOCKED_TARGET',
  completion_claim_permitted: false,
};

const abiClosure = {
  schema: 'braxon.native.abi_closure.v1',
  generated_at: new Date().toISOString(),
  required_artifact_checks: ['AArch64 ELF machine', 'ELF class 64-bit', 'DT_NEEDED Android libc', 'exported symbol visibility', 'static archive symbols', 'consumer compile', 'consumer link', 'target execution', 'downstream consumer execution'],
  required_tools: ['llvm-nm', 'llvm-readelf', 'llvm-readobj', 'llvm-objdump', 'clang', 'clang++', 'ld.lld', 'llvm-ar', 'llvm-ranlib'],
  target_probe_performed: false,
  status: 'BLOCKED_TARGET',
};

const semanticReflexInventory = {
  schema: 'braxon.native.semantic_reflex_inventory.v1',
  generated_at: new Date().toISOString(),
  dialect_projection: matrix.dialect_projection,
  native_capabilities: missingSymbols.entries.map((entry) => ({
    canonical_identity: entry.semantic_identity,
    source_authority: entry.source_authority,
    semantic_intent: `request ${entry.symbol} Android Bionic compatibility contract`,
    execution_route: entry.overlay_route,
    verification_route: 'target compile/link/run plus downstream consumer probe',
    failure_closed_behavior: 'BLOCKED_TARGET until actual target proof exists',
  })),
  language_capability_total: languageEntries.length,
  resident_runtime_constructed: false,
};

const upstreamProvenance = {
  schema: 'braxon.native.upstream_provenance.v1',
  generated_at: new Date().toISOString(),
  sources: sources.sources.map((source) => ({
    id: source.id,
    upstream_project: source.upstream_project ?? null,
    source_url: source.source_url ?? null,
    source_status: source.source_status,
    license: source.license ?? null,
    reconstruction_status: source.reconstruction_status ?? null,
  })),
  normal_runtime_network_required: sources.policy.normal_runtime_network_required,
};

const licenseClassification = {
  schema: 'braxon.native.license_classification.v1',
  generated_at: new Date().toISOString(),
  classifications: licenses.components ?? licenses.entries ?? [],
  rule: 'Source provenance and license terms remain authoritative. Interface compatibility does not transfer upstream ownership or make derived code privately licensable.',
};

const targetPrerequisites = {
  schema: 'braxon.native.target_prerequisites.v1',
  generated_at: new Date().toISOString(),
  target_environment: targetEnvironment,
  capacity_blockers: capacity.current_capacity_blockers,
  rust_release_blockers: rust.current_release_state?.release_blockers ?? [],
  required_sequence: [
    'scripts/braxon_reconstruct.sh preflight',
    'scripts/braxon_reconstruct.sh source-edge',
    'BRAXON_SOURCE_BUILD_APPROVED=1 JOBS=1 scripts/braxon_reconstruct.sh source-build',
    'BRAXON_SOURCE_BUILD_APPROVED=1 JOBS=1 scripts/braxon_reconstruct.sh edge-nightly-build',
    'scripts/braxon_reconstruct.sh calibrate',
    'scripts/braxon_reconstruct.sh verify',
  ],
  completion_claim_permitted: false,
};

const humanReport = [
  '# Braxon Native Contract Closure Inventory',
  '',
  `Generated: ${new Date().toISOString()}`,
  '',
  '> This report is derived from retained source/build surfaces and canonical contracts. It does not claim a native Android build, ABI validation, consumer execution, or Rust nightly promotion unless the corresponding target receipt exists.',
  '',
  '## Target boundary',
  '',
  `The declared target is **${targetEnvironment.declared_abi}** at Android API ${targetEnvironment.declared_android_api_floor}. This generator executed on **${targetEnvironment.controller_architecture}/${targetEnvironment.controller_platform}**; therefore every native interface remains **BLOCKED_TARGET** until an actual target compile/link/run and downstream-consumer proof is retained.`,
  '',
  '## Declared native interfaces and observed source/build references',
  '',
  '| Interface | Class | Header | Source/build references | Current status |',
  '|---|---|---|---:|---|',
  ...missingSymbols.entries.map((entry) => `| ${entry.symbol} | ${entry.interface_class} | ${entry.header} | ${entry.observed_source_build_reference_count} | ${entry.status} |`),
  '',
  '## No-depth-limit retained source/build scans',
  '',
  '| Source root | Regular files traversed | Source/build files scanned | Bytes scanned | Oversized source/build files skipped |',
  '|---|---:|---:|---:|---:|',
  ...sourceReferenceScans.map((scan) => `| ${scan.root} | ${scan.traversed_regular_files} | ${scan.source_or_build_files_scanned} | ${scan.bytes_scanned} | ${scan.oversized_source_or_build_files_skipped} |`),
  '',
  '## Required native closure chain',
  '',
  'Each discovered requirement follows the established Braxon chain: source-derived inventory → classification → staged header when necessary → native implementation only when necessary → AArch64 compilation → archive/shared overlay → symbol inspection → consumer compile/link → target execution → downstream consumer probe → machine-readable evidence → NSQ/Reflexor registration → canonicality validation.',
  '',
  '## Required target sequence',
  '',
  ...targetPrerequisites.required_sequence.map((command, index) => `${index + 1}. \`${command}\``),
  '',
  '## Truth state',
  '',
  `Native contract family: **${contractClosure.native_status}**. Rust edge nightly: **REPOSITORY_CONTAINED_NATIVE_PROMOTION_PATH_IMPLEMENTED_TARGET_BUILD_PENDING**. CPython and LLVM source/build routes are repository-contained but are not target-complete until their own actual receipts exist. The NSQ Reflexor route is canonical and on-demand; no resident runtime is constructed.`,
  '',
  '## Evidence files',
  '',
  'The sibling JSON artifacts in this directory contain the complete toolchain, missing-symbol, contract, ABI, semantic Reflexor, provenance, licensing, prerequisite, and final-closure records.',
].join('\n');

const output = {};
output.toolchain_inventory = writeJson('toolchain_inventory.json', toolchainInventory);
output.missing_symbols = writeJson('missing_symbols.json', missingSymbols);
output.contract_closure = writeJson('contract_closure.json', contractClosure);
output.abi_closure = writeJson('abi_closure.json', abiClosure);
output.semantic_reflex_inventory = writeJson('semantic_reflex_inventory.json', semanticReflexInventory);
output.upstream_provenance = writeJson('upstream_provenance.json', upstreamProvenance);
output.license_classification = writeJson('license_classification.json', licenseClassification);
output.target_prerequisites = writeJson('target_prerequisites.json', targetPrerequisites);
output.human_report = writeText('NATIVE_CONTRACT_CLOSURE.md', humanReport);
const finalClosure = {
  schema: 'braxon.native.final_toolchain_closure.v1',
  generated_at: new Date().toISOString(),
  target_environment: targetEnvironment,
  inventory_outputs: output,
  native_contract_family_status: 'SOURCE_DERIVED_INVENTORY_COMPLETE_TARGET_PROOF_PENDING',
  rust_edge_nightly_status: 'REPOSITORY_CONTAINED_NATIVE_PROMOTION_PATH_IMPLEMENTED_TARGET_BUILD_PENDING',
  cpython_status: 'REPOSITORY_CONTAINED_SOURCE_AND_DOWNSTREAM_PROBE_ROUTE_PRESENT_TARGET_BUILD_PENDING',
  llvm_status: 'RETAINED_SOURCE_AND_SOURCE_BUILD_ROUTE_PRESENT_TARGET_BUILD_PENDING',
  semantic_reflex_status: 'CANONICAL_NSQ_ROUTE_PRESENT_NATIVE_TARGET_CLOSURE_PENDING',
  completion_claim_permitted: false,
  unresolved_status_classes: ['BLOCKED_TARGET', 'EXTERNAL_PREREQUISITE', 'PROPRIETARY_NOT_REDISTRIBUTABLE', 'UNRESOLVED'],
};
output.final_toolchain_closure = writeJson('final_toolchain_closure.json', finalClosure);

console.log(JSON.stringify({
  schema: 'braxon.native.contract_inventory_generation.v1',
  valid: true,
  target_environment: targetEnvironment,
  native_interface_total: missingSymbols.entries.length,
  language_total: languageEntries.length,
  output,
}, null, 2));
