#!/usr/bin/env node
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const root = process.argv[2] ? path.resolve(process.argv[2]) : path.resolve(path.dirname(new URL(import.meta.url).pathname), '../..');
const contracts = [
  'config/braxon_identity.json',
  'config/toolchains/contained_semantic_toolchain_inventory.json',
  'config/toolchains/source_availability_manifest.json',
  'config/toolchains/rust_bootstrap_chain.json',
  'config/toolchains/current_phone_custom_rust_nightly.json',
  'config/toolchains/termux_android_aarch64_capacity_profile.json',
  'config/toolchains/termux_nsq_intercept_policy.json',
  'config/toolchains/source_built_build_graph.json',
  'config/toolchains/extended_repository_integration_manifest.json',
  'config/toolchains/license_report.json',
  'config/toolchains/gap_report.json',
  'config/nsq/feature_execution_registry.json',
  'config/nsq/canonical_surface_registry.json',
  'config/nsq/language_functional_ingestion_matrix.json',
  'config/nsq/complete_semantic_extraction_contract.json',
  'config/nsq/semantic_corpus_manifest.json',
  'config/nsq/bionic_gnu_compatibility_matrix.json',
  'config/nsq/watermarked_file_operation_contract.json',
  'config/nsq/semantic_build_dialect_contract.json',
  'config/nsq/gguf_semantic_flow_contract.json',
  'config/nsq/live_bus_refresh_benchmark_contract.json',
  'config/nsq/live_bus_bootstrap_contract.json',
  'config/nsq/native_contract_closure_inventory_contract.json',
  'config/nsq/js_java_jni_llvm_candidate_contract.json',
  'config/nsq/full_file_watermark_reflex_contract.json',
  'config/nsq/blank_surface_registry.json',
];

function readJson(relative) {
  const absolute = path.join(root, relative);
  if (!fs.existsSync(absolute)) throw new Error(`missing contract: ${relative}`);
  return JSON.parse(fs.readFileSync(absolute, 'utf8'));
}

const parsed = new Map(contracts.map((relative) => [relative, readJson(relative)]));
const identity = parsed.get('config/braxon_identity.json');
const language = parsed.get('config/nsq/language_functional_ingestion_matrix.json');
const feature = parsed.get('config/nsq/feature_execution_registry.json');
const canonicalSurface = parsed.get('config/nsq/canonical_surface_registry.json');
const repository = parsed.get('config/toolchains/extended_repository_integration_manifest.json');
const graph = parsed.get('config/toolchains/source_built_build_graph.json');
const rustBootstrap = parsed.get('config/toolchains/rust_bootstrap_chain.json');
const currentPhoneRust = parsed.get('config/toolchains/current_phone_custom_rust_nightly.json');
const semantic = parsed.get('config/nsq/semantic_corpus_manifest.json');
const extraction = parsed.get('config/nsq/complete_semantic_extraction_contract.json');
const watermark = parsed.get('config/nsq/watermarked_file_operation_contract.json');
const semanticBuild = readJson('config/nsq/semantic_build_dialect_contract.json');
const ggufFlow = readJson('config/nsq/gguf_semantic_flow_contract.json');
const liveBusBenchmark = readJson('config/nsq/live_bus_refresh_benchmark_contract.json');
const liveBus = parsed.get('config/nsq/live_bus_bootstrap_contract.json');
const nativeClosure = parsed.get('config/nsq/native_contract_closure_inventory_contract.json');
const runtimeCandidates = parsed.get('config/nsq/js_java_jni_llvm_candidate_contract.json');
const sourceAvailability = parsed.get('config/toolchains/source_availability_manifest.json');
const fullFileWatermark = parsed.get('config/nsq/full_file_watermark_reflex_contract.json');
const blankRegistry = parsed.get('config/nsq/blank_surface_registry.json');
const fullTreeAudit = readJson('state/braxon/full_tree_audit.json');
const fullFileWatermarkInventory = readJson('state/braxon/full_file_watermark_reflex_inventory.json');
const interceptPolicy = parsed.get('config/toolchains/termux_nsq_intercept_policy.json');

if (identity.canonical_project_name !== 'Braxon' || identity.canonical_project_name_case_sensitive !== 'Braxon') throw new Error('canonical Braxon project identity is invalid');
if (identity.legal_owner !== 'Michael David Norris' || identity.canonical_license !== 'LicenseRef-Braxon-Private') throw new Error('first-party owner or license identity is invalid');
for (const required of ['LICENSE', 'NOTICE', identity.first_party_component_scope, 'scripts/toolchains/resolve_braxon_repository_tool.sh', 'scripts/toolchains/write_braxon_repository_tool_dispatch.sh']) {
  if (!fs.existsSync(path.join(root, required))) throw new Error(`required first-party identity or dispatch surface is missing: ${required}`);
}
const licenseText = fs.readFileSync(path.join(root, 'LICENSE'), 'utf8');
const noticeText = fs.readFileSync(path.join(root, 'NOTICE'), 'utf8');
const componentScopeText = fs.readFileSync(path.join(root, identity.first_party_component_scope), 'utf8');
for (const required of ['Michael David Norris', 'LicenseRef-Braxon-Private']) {
  if (!licenseText.includes(required) || !noticeText.includes(required) || !componentScopeText.includes(required)) throw new Error(`first-party license notice lacks required identity: ${required}`);
}
if (interceptPolicy.schema !== 'braxon.termux.nsq_intercept_policy.v2' || !interceptPolicy.policy.repository_built_tool_required_for_declared_execution || interceptPolicy.policy.ambient_termux_tool_execution_allowed_after_calibration || interceptPolicy.policy.ambient_termux_tool_role !== 'bootstrap_only_for_rebuilding_repository_owned_toolchains_before_a_verified_local_artifact_exists') throw new Error('repository-built tool dispatch policy is invalid');
if (interceptPolicy.calibration.repository_tool_resolver !== 'scripts/toolchains/resolve_braxon_repository_tool.sh' || interceptPolicy.calibration.repository_tool_dispatch_manifest !== 'state/full_android_language_toolchain/install/braxon_repository_tool_dispatch.json') throw new Error('repository-tool resolver binding is invalid');
if (interceptPolicy.intercepted_tools.some((tool) => tool.real_tool_source !== 'verified_repository_tool_dispatch_manifest')) throw new Error('ambient tool discovery remains in the declared intercept policy');
const resolverScript = fs.readFileSync(path.join(root, 'scripts/toolchains/resolve_braxon_repository_tool.sh'), 'utf8');
if (!resolverScript.includes('ambient Termux fallback is prohibited') || !resolverScript.includes('verified_repository_built')) throw new Error('repository-tool resolver does not fail closed');

if (language.language_total !== language.languages.length) throw new Error('language matrix total mismatch');
if (repository.repository_total !== repository.repositories.length) throw new Error('repository manifest total mismatch');
if (!feature.features.some((item) => item.id === 'feature:language.parameter_parse')) throw new Error('language intercept feature missing');
if (!feature.features.some((item) => item.id === 'feature:repository.operation')) throw new Error('repository intercept feature missing');
if (!feature.features.some((item) => item.id === 'feature:toolchain.bionic_compatibility')) throw new Error('Bionic feature missing');
if (!feature.features.some((item) => item.id === 'feature:watermark.file_operation')) throw new Error('functional watermark feature missing');
const semanticBuildFeature = feature.features.find((item) => item.id === 'feature:toolchain.semantic_build_dialect');
if (!semanticBuildFeature || semanticBuildFeature.canonicality !== 'canonical_active' || semanticBuildFeature.resident_runtime || !semanticBuildFeature.on_demand || semanticBuildFeature.nsq_dialect !== 'control') throw new Error('canonical semantic build dialect feature is invalid');
if (!feature.features.some((item) => item.id === 'feature:live_bus.bootstrap' && item.canonicality === 'canonical_active')) throw new Error('canonical live bus bootstrap feature missing');
const ggufSemanticFlowFeature = feature.features.find((item) => item.id === 'feature:gguf.semantic_flow');
if (!ggufSemanticFlowFeature || ggufSemanticFlowFeature.source !== 'crates/braxon-core/src/gguf_semantic_flow.rs' || ggufSemanticFlowFeature.resident_runtime || !ggufSemanticFlowFeature.required_artifacts?.includes('config/nsq/gguf_semantic_flow_contract.json')) throw new Error('GGUF semantic-flow feature is invalid');
const liveBusBenchmarkFeature = feature.features.find((item) => item.id === 'feature:live_bus.refresh_benchmark');
if (!liveBusBenchmarkFeature || liveBusBenchmarkFeature.source !== 'crates/braxon-core/examples/region_fire_probe.rs' || liveBusBenchmarkFeature.resident_runtime || !liveBusBenchmarkFeature.required_artifacts?.includes('config/nsq/live_bus_refresh_benchmark_contract.json')) throw new Error('live-bus refresh benchmark feature is invalid');
if (!feature.features.every((item) => item.canonicality === 'canonical_active' || item.canonicality === 'deprecated_historical_only')) throw new Error('feature canonicality classification is incomplete');
for (const item of feature.features.filter((item) => item.canonicality === 'deprecated_historical_only')) {
  const successor = feature.features.find((candidate) => candidate.id === item.deprecated_replaced_by);
  if (!successor || successor.canonicality !== 'canonical_active') throw new Error(`deprecated feature lacks canonical successor: ${item.id}`);
}
if (!canonicalSurface.canonical_routes.some((route) => route.id === 'route:donor.citadel_seed' && route.classification === 'canonical_active')) throw new Error('canonical Council Ten Citadel donor route missing');
if (!canonicalSurface.removed_surface_purposes.some((surface) => surface.purpose === 'Active conventional safetensors index as the Braxon donor readiness authority' && surface.classification === 'removed')) throw new Error('removed conventional donor-index purpose declaration missing');
if (!canonicalSurface.canonical_routes.some((route) => route.id === 'route:live_bus.virtual_piston_ghost_bootstrap' && route.classification === 'canonical_active')) throw new Error('canonical live Piston/Ghost route missing');
const ggufSemanticFlowRoute = canonicalSurface.canonical_routes.find((route) => route.id === 'route:gguf.semantic_flow');
if (!ggufSemanticFlowRoute || ggufSemanticFlowRoute.feature !== 'feature:gguf.semantic_flow' || !ggufSemanticFlowRoute.front_door?.includes('Braxon gguf extract')) throw new Error('GGUF semantic-flow route is invalid');
const liveBusBenchmarkRoute = canonicalSurface.canonical_routes.find((route) => route.id === 'route:live_bus.refresh_benchmark');
if (!liveBusBenchmarkRoute || liveBusBenchmarkRoute.feature !== 'feature:live_bus.refresh_benchmark' || !liveBusBenchmarkRoute.front_door?.includes('region_fire_probe')) throw new Error('live-bus refresh benchmark route is invalid');
if (!canonicalSurface.canonical_routes.some((route) => route.id === 'route:watermark.full_file_reflex_inventory' && route.classification === 'canonical_active')) throw new Error('full-file watermark Reflexor route missing');
if (!canonicalSurface.canonical_routes.some((route) => route.id === 'route:toolchain.native_contract_closure_inventory' && route.feature === 'feature:toolchain.bionic_compatibility' && route.classification === 'canonical_active')) throw new Error('native contract closure canonical route missing');
if (!canonicalSurface.canonical_routes.some((route) => route.id === 'route:toolchain.js_java_jni_llvm_candidates' && route.feature === 'feature:toolchain.semantic_build_dialect' && route.classification === 'canonical_active')) throw new Error('JavaScript/Java/JNI candidate canonical route missing');
const semanticBuildRoute = canonicalSurface.canonical_routes.find((route) => route.id === 'route:toolchain.semantic_build_dialect');
if (!semanticBuildRoute || semanticBuildRoute.feature !== 'feature:toolchain.semantic_build_dialect' || semanticBuildRoute.classification !== 'canonical_active' || !semanticBuildRoute.front_doors.includes('Braxon toolchain build-dialect <scope> execute')) throw new Error('semantic build dialect canonical route is invalid');
if (runtimeCandidates.capability !== 'feature:toolchain.semantic_build_dialect' || runtimeCandidates.execution_model.resident_runtime || runtimeCandidates.execution_model.ambient_termux_runtime_fallback_allowed || !runtimeCandidates.execution_model.repository_built_tool_dispatch_required_before_calibration || !runtimeCandidates.execution_model.semantic_build_authority?.includes('feature:toolchain.semantic_build_dialect')) throw new Error('JavaScript/Java/JNI candidate execution boundary is invalid');
for (const laneId of ['quickjs_2026_06_04', 'nodejs_v26_7_0', 'openjdk_jdk26u_26_0_2_1', 'jni_bridge']) {
  const lane = runtimeCandidates.lanes.find((item) => item.id === laneId);
  if (!lane || lane.activation_status !== 'TARGET_BUILD_PENDING') throw new Error(`runtime candidate lane is missing or prematurely activated: ${laneId}`);
}
if (runtimeCandidates.version_channel_policy.automatic_latest_selection_allowed || runtimeCandidates.version_channel_policy.default_channel_before_target_proof !== 'none') throw new Error('runtime candidate version-channel activation boundary is invalid');
for (const sourceId of ['quickjs_2026_06_04', 'nodejs_v26_7_0', 'openjdk_jdk26u_26_0_2_1']) {
  const source = sourceAvailability.sources.find((item) => item.id === sourceId);
  if (!source || !source.source_status.includes('source_archive_clone_contained') || !source.source_status.includes('unexecuted_not_activated')) throw new Error(`runtime candidate source truth is invalid: ${sourceId}`);
}
const nativeSourceVerifierPath = path.join(root, 'scripts/toolchains/verify_public_source_archives.sh');
if (!fs.existsSync(nativeSourceVerifierPath)) throw new Error('native POSIX source archive verifier is missing');
const nativeSourceVerifier = fs.readFileSync(nativeSourceVerifierPath, 'utf8');
for (const sourceId of ['quickjs_2026_06_04', 'nodejs_v26_7_0', 'openjdk_jdk26u_26_0_2_1', 'source_check_total=7']) {
  if (!nativeSourceVerifier.includes(sourceId)) throw new Error(`native source archive verifier lacks candidate evidence: ${sourceId}`);
}
if (/^#!.*\b(node|python)\b/m.test(nativeSourceVerifier) || /^\s*(node|python)(?:\s|$)/m.test(nativeSourceVerifier) || /(?:;|&&|\|\|)\s*(node|python)(?:\s|$)/.test(nativeSourceVerifier)) throw new Error('native source archive verifier has an undeclared non-POSIX runtime dependency');
if (nativeClosure.capability !== 'feature:toolchain.bionic_compatibility' || nativeClosure.execution_model.resident_runtime || nativeClosure.execution_model.hidden_download_allowed || nativeClosure.execution_model.system_or_termux_prefix_write_allowed) throw new Error('native contract closure execution boundary is invalid');
for (const classification of ['CLOSED_NATIVE', 'CLOSED_HEADER', 'CLOSED_EXISTING_ANDROID', 'CLOSED_VIA_SYSCALL', 'CLOSED_VIA_FALLBACK', 'CLOSED_UPSTREAM', 'BLOCKED_TARGET', 'EXTERNAL_PREREQUISITE', 'PROPRIETARY_NOT_REDISTRIBUTABLE', 'HISTORICAL', 'UNRESOLVED']) {
  if (!nativeClosure.classification_set.includes(classification)) throw new Error(`native contract closure classification is missing: ${classification}`);
}
if (!nativeClosure.failure_closed_behavior.includes('No inventory entry')) throw new Error('native contract closure fail-closed boundary is invalid');
const nativeInventoryScript = path.join(root, 'tools/toolchain/generate_native_contract_closure_inventory.mjs');
if (!fs.existsSync(nativeInventoryScript)) throw new Error('native contract closure inventory generator is missing');
if (watermark.capability !== 'feature:watermark.file_operation' || !watermark.watermark_is_functional) throw new Error('functional watermark capability contract is invalid');
if (watermark.resident_runtime || watermark.native_execution_policy.allows_hidden_download || !watermark.native_execution_policy.requires_aarch64_android_target) throw new Error('functional watermark target boundary is invalid');
if (ggufFlow.schema !== 'braxon.nsq.gguf_semantic_flow_contract.v1' || ggufFlow.capability !== 'feature:gguf.semantic_flow' || ggufFlow.external_gguf_runtime_authority || ggufFlow.tensor_payload_loaded || !ggufFlow.parameter_cells_persistently_live || ggufFlow.container_reloaded_during_piston_refresh || ggufFlow.piston_refresh_policy?.address_namespace !== 'council/0/gguf' || !ggufFlow.piston_refresh_policy?.addresses_are_stable_across_refresh || ggufFlow.piston_refresh_policy?.duty_cycle !== 'address_stable_round_robin_piston_refresh_with_changed_cell_priority' || ggufFlow.piston_refresh_policy?.environment_binding?.authority !== 'KSR_validated_allowlist_only' || !ggufFlow.piston_refresh_policy?.environment_binding?.reject_unlisted_environment_variables || !ggufFlow.piston_refresh_policy?.environment_binding?.reject_binding_owner_or_watermark_mismatch || ggufFlow.stable_address_bindings?.length !== ggufFlow.semantic_cells?.length || new Set(ggufFlow.stable_address_bindings.map((binding) => binding.virtual_address)).size !== ggufFlow.semantic_cells.length || new Set(ggufFlow.stable_address_bindings.map((binding) => binding.environment_variable)).size !== ggufFlow.semantic_cells.length) throw new Error('GGUF stable-address live-bus contract is invalid');
if (liveBusBenchmark.schema !== 'braxon.nsq.live_bus_refresh_benchmark_contract.v1' || liveBusBenchmark.capability !== 'feature:live_bus.refresh_benchmark' || liveBusBenchmark.target_state !== 'TARGET_MEASUREMENT_PENDING' || liveBusBenchmark.comparative_modes?.length !== 2 || !liveBusBenchmark.comparative_modes.some((mode) => mode.id === 'stable_address_piston_refresh' && mode.piston_firing && mode.stable_virtual_addresses && !mode.bus_reconstructed_between_cycles) || !liveBusBenchmark.comparative_modes.some((mode) => mode.id === 'non_piston_bus_reconstruction' && !mode.piston_firing && mode.stable_virtual_addresses && mode.bus_reconstructed_between_cycles) || !liveBusBenchmark.semantic_descriptors?.includes('image.semantic-descriptor') || !liveBusBenchmark.truth_boundary.includes('does not measure or claim external model-weight execution')) throw new Error('live-bus refresh benchmark contract is invalid');
if (semanticBuild.schema !== 'braxon.nsq.semantic_build_dialect_contract.v1' || semanticBuild.authority !== 'NSQ_KINETIC_SEMANTIC_REFLEXOR' || semanticBuild.capability !== 'feature:toolchain.semantic_build_dialect' || semanticBuild.resident_runtime || semanticBuild.hidden_download_allowed || semanticBuild.reflexor_job_bound || semanticBuild.nsq_dialect !== 'control' || semanticBuild.semantic_subdialect !== 'kinetic_build') throw new Error('semantic build dialect contract identity is invalid');
if (semanticBuild.shared_cell_policy.address_namespace !== 'council/0/build' || semanticBuild.shared_cell_policy.same_address_concurrent_write !== 'queued' || semanticBuild.shared_cell_policy.source_mutation !== 'forbidden_except_declared_repository_source_materialization' || semanticBuild.shared_cell_policy.artifact_mutation !== 'allowed_only_through_declared_executor' || semanticBuild.shared_cell_policy.piston_lifecycle.join(',') !== 'acquire,hold,commit,release') throw new Error('semantic build shared-cell policy is invalid');
if (semanticBuild.executor_policy.dispatcher !== 'scripts/braxon_reconstruct.sh' || semanticBuild.executor_policy.allows_arbitrary_arguments || semanticBuild.executor_policy.allows_hidden_download || !semanticBuild.executor_policy.requires_explicit_execute || !semanticBuild.executor_policy.requires_aarch64_android_target || semanticBuild.executor_policy.physical_default_jobs !== 1 || semanticBuild.executor_policy.controlled_environment.JOBS !== '1') throw new Error('semantic build physical executor policy is invalid');
if (semanticBuild.storage_chain_policy.storage_root !== 'state/full_android_language_toolchain/ksr_semantic_chain' || !semanticBuild.storage_chain_policy.persist_declared_transition_state || semanticBuild.storage_chain_policy.chain_reaction_mode !== 'watermark_governed_declared_dependency_propagation' || semanticBuild.storage_chain_policy.auto_execute_physical_compiler || semanticBuild.storage_chain_policy.automatic_runtime_activation || semanticBuild.storage_chain_policy.automatic_tool_state_population !== 'persist_verified_repository_built_tool_state_from_declared_output_hashes_only' || !semanticBuild.storage_chain_policy.undo_available_for_every_declared_scope || semanticBuild.storage_chain_policy.undo_mode !== 'atomically_restore_prior_verified_storage_materialization_and_record_new_reversal_watermark' || semanticBuild.storage_chain_policy.undo_may_execute_physical_compiler || semanticBuild.storage_chain_policy.undo_may_delete_or_rewrite_artifacts) throw new Error('semantic build storage propagation or undo policy is invalid');
if (!Array.isArray(semanticBuild.automatic_tool_population) || semanticBuild.automatic_tool_population.length < 2 || semanticBuild.automatic_tool_population.some((entry) => entry.state !== 'verified_repository_built_pending_target_proof_completion' || !entry.scope || !entry.output_root || !entry.tool_names?.length)) throw new Error('automatic KSR tool-state population is invalid');
const llvmToolPopulation = semanticBuild.automatic_tool_population.find((entry) => entry.scope === 'llvm-aarch64-source-build');
if (!llvmToolPopulation || !['llvm-dwarfdump', 'llvm-jitlink', 'llvm-mc', 'llvm-bolt', 'perf2bolt'].every((tool) => llvmToolPopulation.tool_names.includes(tool))) throw new Error('automatic LLVM DWARF, ORC, or BOLT tool-state population is missing');
if (!Array.isArray(semanticBuild.chain_reactions) || semanticBuild.chain_reactions.some((reaction) => reaction.effect !== 'queue_prepare' || !['prepared_or_executed', 'executed_with_declared_outputs_watermarked'].includes(reaction.condition))) throw new Error('semantic build chain reaction policy is invalid');
for (const [featureId, disposition] of [['aarch64_aot_clang_lld', 'selected_by_llvm-aarch64-source-build'], ['compiler_rt_and_cxx_runtimes', 'selected_by_llvm-aarch64-source-build'], ['dwarf_elf_debug_proof', 'selected_by_llvm-aarch64-source-build'], ['mc_hammer_aarch64_conformance', 'TARGET_CONFORMANCE_PROOF_PENDING'], ['orc_jit', 'built_by_llvm-aarch64-source-build_target_proof_pending'], ['bolt_post_link_optimization', 'built_by_llvm-aarch64-source-build_target_profile_and_platform_proof_pending'], ['thin_lto', 'TARGET_STABILITY_PROOF_PENDING'], ['profile_guided_optimization', 'TARGET_WORKLOAD_PROOF_PENDING']]) {
  const feature = semanticBuild.llvm_native_features?.find((item) => item.id === featureId);
  if (!feature || feature.disposition !== disposition || !feature.components?.length || !feature.proof?.includes('artifact_watermark')) throw new Error(`LLVM native feature boundary is invalid: ${featureId}`);
}
for (const scopeId of ['llvm-source-edge', 'llvm-aarch64-source-build', 'rust-edge-nightly', 'quickjs', 'nodejs', 'openjdk', 'jni-bridge', 'aarch64-assembly', 'optimization-proof']) {
  const scope = semanticBuild.scopes.find((item) => item.id === scopeId);
  if (!scope || !scope.allowed_actions.includes('inspect') || !scope.allowed_actions.includes('undo') || !scope.required_paths.length || !scope.proof_requirements.length) throw new Error(`semantic build scope is incomplete: ${scopeId}`);
}
for (const scopeId of ['quickjs', 'nodejs', 'openjdk', 'jni-bridge']) {
  const scope = semanticBuild.scopes.find((item) => item.id === scopeId);
  if (scope.executor_mode !== null || scope.target_state !== 'TARGET_BUILD_PENDING' || scope.allowed_actions.includes('execute')) throw new Error(`runtime candidate scope is prematurely executable: ${scopeId}`);
}
if (liveBus.capability !== 'feature:live_bus.bootstrap' || !liveBus.execution_model.includes('no background daemon') || liveBus.truth_boundary.model_weight_execution_claimed || liveBus.truth_boundary.resident_runtime_constructed) throw new Error('live bus truth boundary is invalid');
if (liveBus.required_windows.length !== 4 || !liveBus.required_windows.some((window) => window.logical_id === 'council-ten.seed-body.*' && window.required_count === 10)) throw new Error('live bus required virtual window topology is invalid');
if (fullFileWatermark.capability !== 'feature:watermark.file_operation' || !fullFileWatermark.scope.includes('every tracked regular executable source') || !fullFileWatermark.fail_closed_conditions.includes('missing_line_merkle_root')) throw new Error('full-file watermark Reflexor contract is invalid');
if (!blankRegistry.valid || blankRegistry.entries.some((entry) => entry.classification === 'remove_or_materialize_required' || entry.active_reflexor_routable)) throw new Error('blank-surface registry is invalid');
if (!fullTreeAudit.valid || fullTreeAudit.unclassified.length !== 0 || fullTreeAudit.unregistered_blank_required_content.length !== 0 || fullTreeAudit.unresolved_environment_links.length !== 0) throw new Error('full tracked-tree audit is invalid');
if (!fullFileWatermarkInventory.valid || fullFileWatermarkInventory.unbound.length !== 0 || fullFileWatermarkInventory.unregistered_blank_executable_files.length !== 0 || fullFileWatermarkInventory.tree_audit_merkle_sha256 !== fullTreeAudit.merkle_sha256) throw new Error('full-file watermark Reflexor inventory is invalid or stale');
if (!graph.nodes.some((item) => item.id === 'complete_language_semantic_proofs')) throw new Error('semantic proof build-graph node missing');
const repositoryToolDispatch = graph.nodes.find((item) => item.id === 'repository_built_tool_dispatch');
if (!repositoryToolDispatch || !repositoryToolDispatch.depends_on.includes('edge_nightly_1_100_0_promotion')) throw new Error('repository-built tool dispatch build-graph node is invalid');
const calibrationNode = graph.nodes.find((item) => item.id === 'termux_nsq_calibration_and_recovery');
if (!calibrationNode || !calibrationNode.depends_on.includes('repository_built_tool_dispatch')) throw new Error('Termux calibration is not bound to repository-built tool dispatch');
if (!graph.nodes.some((item) => item.id === 'functional_watermark_file_operation')) throw new Error('functional watermark build-graph node missing');
const semanticBuildNode = graph.nodes.find((item) => item.id === 'ksr_semantic_build_dialect');
if (!semanticBuildNode || !semanticBuildNode.depends_on.includes('functional_watermark_file_operation') || !semanticBuildNode.operation.includes('Braxon toolchain build-dialect')) throw new Error('semantic build KSR graph node is invalid');
const llvmBuildNode = graph.nodes.find((item) => item.id === 'llvm_clang_lld_source_build');
if (!llvmBuildNode || !llvmBuildNode.depends_on.includes('ksr_semantic_build_dialect') || !llvmBuildNode.operation.includes('Braxon toolchain build-dialect llvm-aarch64-source-build execute')) throw new Error('LLVM source build is not bound to KSR semantic execution');
const edgePromotion = rustBootstrap.lanes.find((item) => item.id === 'edge_candidate_1_100_0_nightly');
if (!edgePromotion || edgePromotion.git_commit !== 'f7d782a3be46d6bb4b9792fe69a61db389ba1769' || edgePromotion.bootstrap_dependency !== 'bootstrap_termux_1_97_1') throw new Error('edge nightly bootstrap authority is invalid');
if (edgePromotion.native_promotion_command !== 'Braxon toolchain build-dialect rust-edge-nightly execute' || edgePromotion.selection_policy !== 'never selected by default before all required native target evidence is present') throw new Error('edge nightly promotion command or activation boundary is invalid');
if (currentPhoneRust.bootstrap_authority?.expected_release !== '1.97.1' || currentPhoneRust.bootstrap_authority?.expected_host !== 'aarch64-linux-android' || !currentPhoneRust.bootstrap_authority?.may_not_be_overwritten) throw new Error('preserved Rust 1.97.1 bootstrap contract is invalid');
if (currentPhoneRust.edge_promotion_candidate?.git_commit !== edgePromotion.git_commit || currentPhoneRust.edge_promotion_candidate?.rustup_allowed || currentPhoneRust.edge_promotion_candidate?.external_compiler_dependency_allowed) throw new Error('edge nightly no-rustup or no-external-compiler policy is invalid');
const edgeGraph = graph.nodes.find((item) => item.id === 'edge_nightly_1_100_0_promotion');
if (!edgeGraph || !edgeGraph.depends_on.includes('termux_stage0_bootstrap') || !edgeGraph.depends_on.includes('bionic_compatibility_overlay')) throw new Error('edge nightly build graph is not bound to bootstrap and Bionic overlay');
const promotionScriptPath = path.join(root, 'scripts/toolchains/promote_rust_edge_nightly_aarch64.sh');
if (!fs.existsSync(promotionScriptPath)) throw new Error('native edge nightly promotion script is missing');
const promotionScript = fs.readFileSync(promotionScriptPath, 'utf8');
for (const required of ['EDGE_COMMIT="f7d782a3be46d6bb4b9792fe69a61db389ba1769"', 'EDGE_RELEASE="1.100.0-nightly"', 'release: 1\\.97\\.1', 'BIONIC_OVERLAY_PROOF', 'stage_equivalent_output', 'promoted_cargo_version', 'promoted_rustdoc_version', 'promoted_rustfmt_version', 'promoted_clippy_version', 'edge_proc_macro.rs', 'proc_macro_aarch64_elf_verified', 'rustup_used": false']) {
  if (!promotionScript.includes(required)) throw new Error(`native edge promotion script lacks required invariant: ${required}`);
}
if (/\brustup\b(?!_used)/.test(promotionScript) || /\bcurl\b|\bwget\b|git clone|git fetch/.test(promotionScript)) throw new Error('native edge promotion script permits forbidden external compiler acquisition');
if (promotionScript.includes('$(command -v rustc') || promotionScript.includes('$(command -v cargo')) throw new Error('native edge promotion script silently discovers ambient Rust bootstrap tools');
const baseSourceBuildScript = fs.readFileSync(path.join(root, 'scripts/toolchains/rebuild_full_android_language_toolchain.sh'), 'utf8');
if (baseSourceBuildScript.includes('$(command -v rustc') || baseSourceBuildScript.includes('$(command -v cargo')) throw new Error('base source-build script silently discovers ambient Rust bootstrap tools');
if (!baseSourceBuildScript.includes('write_braxon_repository_tool_dispatch.sh')) throw new Error('base source-build script does not publish repository-built normal tool dispatch');
for (const required of ['verify_repository_contained_llvm_source', 'llvm/lib/Demangle/CMakeLists.txt', 'llvm/lib/Support/CMakeLists.txt', 'llvm/lib/TableGen/CMakeLists.txt', 'bolt/CMakeLists.txt', 'llvm/tools/llvm-jitlink/CMakeLists.txt', 'llvm-dwarfdump', 'llvm-mc', 'llvm-bolt', 'perf2bolt', 'source_receipts/llvm-project-eaab4d9841b9a8a12783d927b2df2291c1c79269.txt', 'verify_public_source_archives.sh']) {
  if (!baseSourceBuildScript.includes(required)) throw new Error(`base source-build script lacks complete LLVM source invariant: ${required}`);
}
const reconstructScript = fs.readFileSync(path.join(root, 'scripts/braxon_reconstruct.sh'), 'utf8');
for (const required of ['materialize_chunked_llvm_source', 'llvm_source_complete', 'bolt/CMakeLists.txt', 'llvm/tools/llvm-jitlink/CMakeLists.txt', 'BRAXON_REPLACE_INCOMPLETE_LLVM_SOURCE=1', 'accepted_verified_complete_llvm_source', 'materialized_verified_complete_llvm_source', 'verify_public_source_archives.sh', 'require_ksr_build_authorization', 'BRAXON_KSR_SEMANTIC_BUILD_CAPABILITY', 'BRAXON_KSR_SEMANTIC_BUILD_WATERMARK']) {
  if (!reconstructScript.includes(required)) throw new Error(`reconstruction dispatcher lacks complete LLVM source-edge invariant: ${required}`);
}
const sourceEdgeStart = reconstructScript.indexOf('source_edge() {');
const sourceEdgeEnd = reconstructScript.indexOf('\noffline_verify()', sourceEdgeStart);
const sourceEdgeBody = reconstructScript.slice(sourceEdgeStart, sourceEdgeEnd);
if (sourceEdgeBody.includes('verify_public_source_archives.mjs') || /(^|[^[:alnum:]_])node([^[:alnum:]_]|$)/.test(sourceEdgeBody)) throw new Error('phone source-edge still has a Node runtime dependency');
if (semantic.corpora.length !== semantic.compaction_metrics.manifested_compact_artifact_count) throw new Error('semantic corpus count mismatch');
if (semantic.corpora.reduce((total, corpus) => total + corpus.bytes, 0) !== semantic.compaction_metrics.manifested_compact_bytes) throw new Error('semantic corpus byte aggregate mismatch');
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
  canonical_surface_route_total: canonicalSurface.canonical_routes.length,
  deprecated_historical_surface_total: canonicalSurface.deprecated_historical_surfaces.length,
  tracked_path_total: fullTreeAudit.tracked_path_total,
  executable_line_total: fullFileWatermarkInventory.line_total,
  virtual_live_window_total: liveBus.required_windows.length,
  canonical_project_name: identity.canonical_project_name,
  legal_owner: identity.legal_owner,
  repository_built_tool_dispatch_required: interceptPolicy.policy.repository_built_tool_required_for_declared_execution,
}, null, 2));
