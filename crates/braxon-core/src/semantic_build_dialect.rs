use crate::{
    execute_canonical_parameter_citadel_cycle, BusValue, CouncilSurface, HardwareWriteAck,
    IntentOutcome, KineticReflexor, NsqIntent, NsqIntentDecision, NsqNativeBus, PistonPhase,
    TokenizerBridge, TokenizerBridgeReceipt, ValueClass, Watermark,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub const SEMANTIC_BUILD_DIALECT_SCHEMA: &str = "braxon.nsq.semantic_build_dialect.v1";
pub const SEMANTIC_BUILD_DIALECT_CAPABILITY: &str = "feature:toolchain.semantic_build_dialect";
const CONTRACT_RELATIVE_PATH: &str = "config/nsq/semantic_build_dialect_contract.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SemanticBuildAction {
    Inspect,
    Prepare,
    Execute,
    Undo,
}

impl SemanticBuildAction {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value.trim().to_ascii_lowercase().as_str() {
            "inspect" => Ok(Self::Inspect),
            "prepare" => Ok(Self::Prepare),
            "execute" => Ok(Self::Execute),
            "undo" => Ok(Self::Undo),
            _ => Err("semantic build action must be inspect, prepare, execute, or undo".to_string()),
        }
    }

    fn as_contract_value(self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
            Self::Prepare => "prepare",
            Self::Execute => "execute",
            Self::Undo => "undo",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SemanticBuildStatus {
    Inspected,
    Prepared,
    TargetBuildPending,
    Executed,
    ExecutionFailed,
    Reverted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildPathState {
    pub path: String,
    pub present: bool,
    pub bytes: Option<u64>,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SharedBuildCell {
    pub address: String,
    pub port: String,
    pub value_sha256: String,
    pub byte_len: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildExecutorBoundary {
    pub dispatcher: Option<String>,
    pub selected_mode: Option<String>,
    pub target_environment_matches: bool,
    pub explicit_execute_requested: bool,
    pub executor_permitted: bool,
    pub execution_attempted: bool,
    pub execution_succeeded: bool,
    pub exit_code: Option<i32>,
    pub stdout_log: Option<String>,
    pub stderr_log: Option<String>,
    pub stdout_sha256: Option<String>,
    pub stderr_sha256: Option<String>,
    pub conditional_source_replacement_authorized: bool,
    pub exact_guidance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReversalState {
    pub restored: bool,
    pub restored_snapshot_path: Option<String>,
    pub restored_predecessor_watermark: Option<String>,
    pub artifact_mutation_performed: bool,
    pub physical_execution_performed: bool,
    pub exact_guidance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutomaticToolState {
    pub scope: String,
    pub tool_name: String,
    pub artifact_path: String,
    pub artifact_sha256: String,
    pub state: String,
    pub storage_path: String,
    pub runtime_activated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueuedChainReaction {
    pub upstream_scope: String,
    pub downstream_scope: String,
    pub condition: String,
    pub requested_action: SemanticBuildAction,
    pub automatic_physical_execution: bool,
    pub predecessor_watermark: String,
    pub storage_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistentChainState {
    schema: String,
    scope: String,
    action: SemanticBuildAction,
    status: SemanticBuildStatus,
    state_watermark: String,
    declared_outputs_ready: bool,
    physical_execution_attempted: bool,
    physical_execution_succeeded: bool,
    no_resident_runtime: bool,
    auto_execute_physical_compiler: bool,
    snapshot_path: String,
    predecessor_snapshot_path: Option<String>,
    automatic_tool_state: Vec<AutomaticToolState>,
    queued_chain_reactions: Vec<QueuedChainReaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticBuildDialectReport {
    pub schema: String,
    pub capability: String,
    pub scope: String,
    pub scope_kind: String,
    pub action: SemanticBuildAction,
    pub status: SemanticBuildStatus,
    pub nsq_dialect: String,
    pub semantic_subdialect: String,
    pub kinetic_reflexor_route: String,
    pub target_environment: String,
    pub target_state_before: String,
    pub tokenizer_receipt: TokenizerBridgeReceipt,
    pub parameter_generation: u64,
    pub parameter_invariants_passed: bool,
    pub virtual_shared_cells: Vec<SharedBuildCell>,
    pub nsq_intent_decision: NsqIntentDecision,
    pub state_watermark: Watermark,
    pub state_watermark_committed: bool,
    pub required_inputs: Vec<BuildPathState>,
    pub declared_outputs: Vec<BuildPathState>,
    pub proof_requirements: Vec<String>,
    pub optimization_profile: String,
    pub executor: BuildExecutorBoundary,
    pub semantic_boundary: String,
    pub no_resident_runtime: bool,
    pub hidden_download_allowed: bool,
    pub scope_is_bounded: bool,
    pub exact_next_action: String,
    pub storage_chain_state_path: String,
    pub queued_chain_reactions: Vec<QueuedChainReaction>,
    pub automatic_tool_state: Vec<AutomaticToolState>,
    pub reversal: Option<ReversalState>,
}

#[derive(Debug, Deserialize)]
struct BuildDialectContract {
    schema: String,
    authority: String,
    capability: String,
    target_environment: String,
    nsq_dialect: String,
    semantic_subdialect: String,
    execution_model: String,
    watermark_is_functional: bool,
    resident_runtime: bool,
    hidden_download_allowed: bool,
    reflexor_job_bound: bool,
    storage_chain_policy: StorageChainPolicy,
    automatic_tool_population: Vec<AutomaticToolPopulation>,
    shared_cell_policy: SharedCellPolicy,
    executor_policy: ExecutorPolicy,
    chain_reactions: Vec<ChainReaction>,
    semantic_boundary: SemanticBoundary,
    scopes: Vec<BuildScope>,
}

#[derive(Debug, Deserialize)]
struct StorageChainPolicy {
    storage_root: String,
    persist_declared_transition_state: bool,
    chain_reaction_mode: String,
    auto_execute_physical_compiler: bool,
    cycle_policy: String,
    propagation_requires: Vec<String>,
    downstream_effect: String,
    automatic_tool_state_population: String,
    automatic_runtime_activation: bool,
    undo_available_for_every_declared_scope: bool,
    undo_mode: String,
    undo_may_execute_physical_compiler: bool,
    undo_may_delete_or_rewrite_artifacts: bool,
}

#[derive(Debug, Deserialize)]
struct AutomaticToolPopulation {
    scope: String,
    output_root: String,
    tool_names: Vec<String>,
    state: String,
}

#[derive(Debug, Deserialize)]
struct ChainReaction {
    upstream_scope: String,
    downstream_scope: String,
    condition: String,
    effect: String,
}

#[derive(Debug, Deserialize)]
struct SharedCellPolicy {
    address_namespace: String,
    piston_lifecycle: Vec<String>,
    same_address_concurrent_write: String,
    source_mutation: String,
    artifact_mutation: String,
}

#[derive(Debug, Deserialize)]
struct ExecutorPolicy {
    dispatcher: String,
    allowlisted_modes: Vec<String>,
    requires_explicit_execute: bool,
    requires_aarch64_android_target: bool,
    preserved_environment: Vec<String>,
    physical_default_jobs: u32,
    controlled_environment: BTreeMap<String, String>,
    allows_arbitrary_arguments: bool,
    allows_hidden_download: bool,
    failure_mode: String,
}

#[derive(Debug, Deserialize)]
struct SemanticBoundary {
    llvm_authority: String,
    javascript_and_java_runtime_semantics: String,
    jni: String,
    activation_rule: String,
}

#[derive(Debug, Clone, Deserialize)]
struct BuildScope {
    id: String,
    kind: String,
    languages: Vec<String>,
    allowed_actions: Vec<String>,
    executor_mode: Option<String>,
    target_state: String,
    required_paths: Vec<String>,
    proof_requirements: Vec<String>,
    #[serde(default)]
    output_paths: Vec<String>,
    optimization_profile: String,
}

/// Execute one bounded, on-demand build transaction. The KSR owns intent admission,
/// virtual shared-cell coordination, functional watermarks, and artifact acceptance.
/// The shell dispatcher remains a narrow physical Android executor and can never decide
/// a scope, add arguments, or activate a runtime lane on its own.
pub fn execute_semantic_build_dialect(
    start: impl AsRef<Path>,
    scope_id: &str,
    action: SemanticBuildAction,
    authorize_incomplete_llvm_replacement: bool,
) -> Result<SemanticBuildDialectReport, String> {
    let root = resolve_root(start)?;
    let contract: BuildDialectContract = read_json(&root.join(CONTRACT_RELATIVE_PATH))?;
    validate_contract(&contract)?;
    let scope = contract
        .scopes
        .iter()
        .find(|scope| scope.id == scope_id.trim().to_ascii_lowercase())
        .ok_or_else(|| format!("semantic build scope '{}' is not declared", scope_id.trim()))?;
    if authorize_incomplete_llvm_replacement
        && (scope.id != "llvm-source-edge" || action != SemanticBuildAction::Execute)
    {
        return Err("--replace-incomplete-llvm is only valid for the explicit llvm-source-edge execute transition".into());
    }
    if !scope
        .allowed_actions
        .iter()
        .any(|allowed| allowed == action.as_contract_value())
    {
        return Err(format!(
            "semantic build action '{}' is not declared for scope '{}'; allowed actions: {}",
            action.as_contract_value(),
            scope.id,
            scope.allowed_actions.join(", ")
        ));
    }
    validate_scope(scope, &contract.executor_policy)?;

    let required_inputs = path_states(&root, &scope.required_paths)?;
    let mut declared_outputs = path_states(&root, &scope.output_paths)?;
    let missing_inputs = required_inputs
        .iter()
        .filter(|entry| !entry.present)
        .map(|entry| entry.path.clone())
        .collect::<Vec<_>>();
    let outputs_ready_before = scope.output_paths.is_empty()
        || declared_outputs.iter().all(|entry| entry.present && entry.sha256.is_some());

    let tokenizer = TokenizerBridge::from_root(&root, "braxon_native")?;
    let intent_text = format!(
        "capability={};scope={};kind={};action={};target={};languages={};inputs={};outputs={}",
        contract.capability,
        scope.id,
        scope.kind,
        action.as_contract_value(),
        contract.target_environment,
        scope.languages.join(","),
        summarize_path_states(&required_inputs),
        summarize_path_states(&declared_outputs)
    );
    let tokenizer_receipt = tokenizer.encode_translate_round_trip(&intent_text);
    if !tokenizer_receipt.all_required_mappings_resolved() {
        return Err("semantic build intent cannot enter KSR because tokenizer mappings are unresolved".into());
    }
    let signal = i64::try_from(intent_text.len())
        .map_err(|_| "semantic build intent exceeds bounded parameter signal range".to_string())?;
    let context = build_context(&required_inputs, &declared_outputs)?;
    let parameter = execute_canonical_parameter_citadel_cycle(signal, context)?;
    if !parameter.invariants.all_pass() {
        return Err("semantic build intent was rejected by Parameter-Citadel invariants".into());
    }

    let mut native_bus = build_native_bus(&contract.shared_cell_policy)?;
    let cells = build_cells(
        &contract.shared_cell_policy.address_namespace,
        &scope.id,
        &intent_text,
        &required_inputs,
        &declared_outputs,
    );
    let nsq_intent = NsqIntent {
        schema: crate::NSQ_NATIVE_INTENT_SCHEMA.to_string(),
        intent_id: format!("semantic-build:{}:{}", scope.id, action.as_contract_value()),
        source_surface: "toolchain.semantic_build_dialect".to_string(),
        capability: contract.capability.clone(),
        gradient: [0.0; 8],
        target_addresses: cells.iter().map(|cell| cell.address.clone()).collect(),
        provenance: "system".to_string(),
        narrative: false,
    };
    let nsq_intent_decision = native_bus.decide(&nsq_intent);
    if nsq_intent_decision.outcome != IntentOutcome::Accepted {
        return Err(format!(
            "semantic build intent was not admitted to its virtual shared cells: {}",
            nsq_intent_decision.reason
        ));
    }
    native_bus.advance_piston(&nsq_intent.intent_id, PistonPhase::Hold)?;

    let mut reflexor = KineticReflexor::new();
    reflexor.publish(cells.iter().map(|cell| BusValue {
        key: cell.address.clone(),
        class: ValueClass::Fact,
        value_hash: cell.value_sha256.clone(),
        byte_len: cell.byte_len,
    }))?;
    reflexor.reconcile()?;
    let state_keys = reflexor
        .pending_delta()
        .iter()
        .map(|delta| delta.key.clone())
        .collect::<Vec<_>>();
    let generation = reflexor.generation();
    let state_commit = reflexor.commit_hardware(HardwareWriteAck {
        adapter_id: "nsq_semantic_build_shared_cell_adapter".to_string(),
        generation,
        accepted: true,
        written_keys: state_keys,
    })?;
    if !state_commit.hardware_write_acknowledged {
        return Err("semantic build shared-cell state was not acknowledged by KSR".into());
    }
    native_bus.advance_piston(&nsq_intent.intent_id, PistonPhase::Commit)?;

    let target_matches = cfg!(all(target_arch = "aarch64", target_os = "android"));
    let mut status = match action {
        SemanticBuildAction::Inspect => SemanticBuildStatus::Inspected,
        SemanticBuildAction::Prepare => {
            if missing_inputs.is_empty() {
                SemanticBuildStatus::Prepared
            } else {
                SemanticBuildStatus::TargetBuildPending
            }
        }
        SemanticBuildAction::Execute => SemanticBuildStatus::TargetBuildPending,
        SemanticBuildAction::Undo => SemanticBuildStatus::TargetBuildPending,
    };
    let mut executor = executor_boundary(
        &root,
        &contract,
        scope,
        action,
        target_matches,
        missing_inputs.as_slice(),
        outputs_ready_before,
        authorize_incomplete_llvm_replacement,
    )?;

    let mut reversal = None;
    if action == SemanticBuildAction::Undo {
        let restored = restore_prior_materialization(
            &root,
            &contract.storage_chain_policy,
            scope,
            &state_commit.watermark,
        )?;
        status = SemanticBuildStatus::Reverted;
        executor.executor_permitted = false;
        executor.exact_guidance = restored.exact_guidance.clone();
        reversal = Some(restored);
    }

    if action == SemanticBuildAction::Execute
        && missing_inputs.is_empty()
        && target_matches
        && scope.executor_mode.is_some()
    {
        let executed = run_declared_executor(
            &root,
            &contract.executor_policy,
            scope,
            authorize_incomplete_llvm_replacement,
            &state_commit.watermark,
        )?;
        executor = executed;
        if executor.execution_succeeded {
            let output_states_after = path_states(&root, &scope.output_paths)?;
            let output_missing = output_states_after
                .iter()
                .filter(|entry| !entry.present || entry.sha256.is_none())
                .map(|entry| entry.path.clone())
                .collect::<Vec<_>>();
            if output_missing.is_empty() {
                let artifact_watermark = commit_artifact_watermark(&scope.id, &output_states_after)?;
                executor.exact_guidance = format!(
                    "declared physical executor succeeded and KSR committed the output artifact watermark {}. Complete every declared target proof before activation.",
                    artifact_watermark.state_hash
                );
                status = SemanticBuildStatus::Executed;
                declared_outputs = output_states_after;
            } else {
                executor.execution_succeeded = false;
                executor.exact_guidance = format!(
                    "the declared executor exited successfully but required outputs are absent or unhashed: {}. Target proof is rejected.",
                    output_missing.join(", ")
                );
                status = SemanticBuildStatus::ExecutionFailed;
            }
        } else {
            status = SemanticBuildStatus::ExecutionFailed;
        }
    }
    native_bus.advance_piston(&nsq_intent.intent_id, PistonPhase::Release)?;
    let outputs_ready_after = scope.output_paths.is_empty()
        || declared_outputs.iter().all(|entry| entry.present && entry.sha256.is_some());
    let automatic_tool_state = populate_verified_tool_state(
        &root,
        &contract.storage_chain_policy,
        &contract.automatic_tool_population,
        scope,
        &status,
        &declared_outputs,
    )?;
    let queued_chain_reactions = queue_declared_chain_reactions(
        &root,
        &contract,
        scope,
        &status,
        outputs_ready_after,
        &state_commit.watermark,
    )?;
    let storage_chain_state_path = if action == SemanticBuildAction::Undo {
        reversal
            .as_ref()
            .and_then(|state| state.restored_snapshot_path.clone())
            .ok_or("KSR undo did not return a restored storage snapshot")?
    } else {
        persist_chain_state(
            &root,
            &contract.storage_chain_policy,
            scope,
            action,
            &status,
            &state_commit.watermark,
            outputs_ready_after,
            &executor,
            &automatic_tool_state,
            &queued_chain_reactions,
        )?
    };

    let exact_next_action = next_action(
        scope,
        action,
        &missing_inputs,
        target_matches,
        outputs_ready_before,
        &executor,
        status.clone(),
    );
    Ok(SemanticBuildDialectReport {
        schema: SEMANTIC_BUILD_DIALECT_SCHEMA.to_string(),
        capability: contract.capability.clone(),
        scope: scope.id.clone(),
        scope_kind: scope.kind.clone(),
        action,
        status,
        nsq_dialect: contract.nsq_dialect.clone(),
        semantic_subdialect: contract.semantic_subdialect.clone(),
        kinetic_reflexor_route: SEMANTIC_BUILD_DIALECT_CAPABILITY.to_string(),
        target_environment: contract.target_environment.clone(),
        target_state_before: scope.target_state.clone(),
        tokenizer_receipt,
        parameter_generation: parameter.generation,
        parameter_invariants_passed: true,
        virtual_shared_cells: cells,
        nsq_intent_decision,
        state_watermark: state_commit.watermark,
        state_watermark_committed: true,
        required_inputs,
        declared_outputs,
        proof_requirements: scope.proof_requirements.clone(),
        optimization_profile: scope.optimization_profile.clone(),
        executor,
        semantic_boundary: format!(
            "LLVM={}; JavaScript/Java={}; JNI={}; activation={}",
            contract.semantic_boundary.llvm_authority,
            contract.semantic_boundary.javascript_and_java_runtime_semantics,
            contract.semantic_boundary.jni,
            contract.semantic_boundary.activation_rule
        ),
        no_resident_runtime: !contract.resident_runtime,
        hidden_download_allowed: contract.hidden_download_allowed,
        scope_is_bounded: true,
        exact_next_action,
        storage_chain_state_path,
        queued_chain_reactions,
        automatic_tool_state,
        reversal,
    })
}

fn persist_chain_state(
    root: &Path,
    policy: &StorageChainPolicy,
    scope: &BuildScope,
    action: SemanticBuildAction,
    status: &SemanticBuildStatus,
    watermark: &Watermark,
    declared_outputs_ready: bool,
    executor: &BuildExecutorBoundary,
    automatic_tool_state: &[AutomaticToolState],
    queued_chain_reactions: &[QueuedChainReaction],
) -> Result<String, String> {
    if !policy.persist_declared_transition_state {
        return Err("semantic build storage-chain persistence is disabled by contract".into());
    }
    let active_path = normalized_relative_path(
        root,
        &format!("{}/active/{}.json", policy.storage_root, scope.id),
    )?;
    let predecessor_snapshot_path = if active_path.is_file() {
        Some(read_json::<PersistentChainState>(&active_path)?.snapshot_path)
    } else {
        None
    };
    let snapshot_path = normalized_relative_path(
        root,
        &format!(
            "{}/snapshots/{}/{}.json",
            policy.storage_root, scope.id, watermark.state_hash
        ),
    )?;
    let state = PersistentChainState {
        schema: "braxon.nsq.semantic_build_chain_state.v1".to_string(),
        scope: scope.id.clone(),
        action,
        status: status.clone(),
        state_watermark: watermark.state_hash.clone(),
        declared_outputs_ready,
        physical_execution_attempted: executor.execution_attempted,
        physical_execution_succeeded: executor.execution_succeeded,
        no_resident_runtime: true,
        auto_execute_physical_compiler: policy.auto_execute_physical_compiler,
        snapshot_path: display_relative(root, &snapshot_path),
        predecessor_snapshot_path,
        automatic_tool_state: automatic_tool_state.to_vec(),
        queued_chain_reactions: queued_chain_reactions.to_vec(),
    };
    write_json_atomically(&snapshot_path, &state)?;
    write_json_atomically(&active_path, &state)?;
    Ok(display_relative(root, &active_path))
}

fn restore_prior_materialization(
    root: &Path,
    policy: &StorageChainPolicy,
    scope: &BuildScope,
    reversal_watermark: &Watermark,
) -> Result<ReversalState, String> {
    if !policy.undo_available_for_every_declared_scope
        || policy.undo_mode
            != "atomically_restore_prior_verified_storage_materialization_and_record_new_reversal_watermark"
        || policy.undo_may_execute_physical_compiler
        || policy.undo_may_delete_or_rewrite_artifacts
    {
        return Err("semantic build undo policy is invalid or permits physical/artifact mutation".into());
    }
    let active_path = normalized_relative_path(
        root,
        &format!("{}/active/{}.json", policy.storage_root, scope.id),
    )?;
    let active: PersistentChainState = read_json(&active_path).map_err(|_| {
        format!(
            "KSR undo has no active materialization for scope '{}'; execute inspect or prepare first",
            scope.id
        )
    })?;
    let previous_path = active.predecessor_snapshot_path.clone().ok_or_else(|| {
        format!(
            "KSR undo has no prior verified materialization for scope '{}'; no state was changed",
            scope.id
        )
    })?;
    let previous_absolute = normalized_relative_path(root, &previous_path)?;
    let previous: PersistentChainState = read_json(&previous_absolute)?;
    if previous.scope != scope.id || previous.snapshot_path != previous_path {
        return Err("KSR undo rejected an invalid prior materialization snapshot".into());
    }
    for state in &active.automatic_tool_state {
        let path = normalized_relative_path(root, &state.storage_path)?;
        if path.is_file() {
            fs::remove_file(&path).map_err(|error| {
                format!("failed to clear superseded KSR tool state '{}': {error}", path.display())
            })?;
        }
    }
    for state in &previous.automatic_tool_state {
        let path = normalized_relative_path(root, &state.storage_path)?;
        write_json_atomically(&path, state)?;
    }
    for queue in &active.queued_chain_reactions {
        let path = normalized_relative_path(root, &queue.storage_path)?;
        if path.is_file() {
            fs::remove_file(&path).map_err(|error| {
                format!("failed to clear superseded KSR queue state '{}': {error}", path.display())
            })?;
        }
    }
    for queue in &previous.queued_chain_reactions {
        let path = normalized_relative_path(root, &queue.storage_path)?;
        write_json_atomically(&path, queue)?;
    }
    write_json_atomically(&active_path, &previous)?;
    let reversal_path = normalized_relative_path(
        root,
        &format!(
            "{}/reversals/{}/{}.json",
            policy.storage_root, scope.id, reversal_watermark.state_hash
        ),
    )?;
    let reversal = ReversalState {
        restored: true,
        restored_snapshot_path: Some(previous_path),
        restored_predecessor_watermark: Some(previous.state_watermark),
        artifact_mutation_performed: false,
        physical_execution_performed: false,
        exact_guidance: "KSR atomically restored the prior verified storage materialization, tool-state eligibility, and declared downstream queue. Compiled artifacts were neither deleted nor rewritten, and no physical compiler was executed.".to_string(),
    };
    write_json_atomically(&reversal_path, &reversal)?;
    Ok(reversal)
}

fn populate_verified_tool_state(
    root: &Path,
    policy: &StorageChainPolicy,
    populations: &[AutomaticToolPopulation],
    scope: &BuildScope,
    status: &SemanticBuildStatus,
    outputs: &[BuildPathState],
) -> Result<Vec<AutomaticToolState>, String> {
    if *status != SemanticBuildStatus::Executed {
        return Ok(Vec::new());
    }
    let Some(population) = populations.iter().find(|population| population.scope == scope.id) else {
        return Ok(Vec::new());
    };
    let mut populated = Vec::new();
    for tool_name in &population.tool_names {
        let artifact_path = format!("{}/{}", population.output_root, tool_name);
        let artifact = outputs
            .iter()
            .find(|output| output.path == artifact_path)
            .ok_or_else(|| format!("declared automatic tool '{}' lacks a matching build output", tool_name))?;
        let artifact_sha256 = artifact.sha256.clone().ok_or_else(|| {
            format!("declared automatic tool '{}' has no artifact hash", tool_name)
        })?;
        let storage_path = normalized_relative_path(
            root,
            &format!("{}/tools/{}/{}.json", policy.storage_root, scope.id, tool_name),
        )?;
        let state = AutomaticToolState {
            scope: scope.id.clone(),
            tool_name: tool_name.clone(),
            artifact_path,
            artifact_sha256,
            state: population.state.clone(),
            storage_path: display_relative(root, &storage_path),
            runtime_activated: false,
        };
        write_json_atomically(&storage_path, &state)?;
        populated.push(state);
    }
    Ok(populated)
}

fn queue_declared_chain_reactions(
    root: &Path,
    contract: &BuildDialectContract,
    scope: &BuildScope,
    status: &SemanticBuildStatus,
    outputs_ready: bool,
    watermark: &Watermark,
) -> Result<Vec<QueuedChainReaction>, String> {
    let eligible = contract.chain_reactions.iter().filter(|reaction| {
        reaction.upstream_scope == scope.id
            && ((reaction.condition == "prepared_or_executed"
                && matches!(*status, SemanticBuildStatus::Prepared | SemanticBuildStatus::Executed))
                || (reaction.condition == "executed_with_declared_outputs_watermarked"
                    && *status == SemanticBuildStatus::Executed
                    && outputs_ready))
    });
    let mut queued = Vec::new();
    for reaction in eligible {
        let downstream = contract
            .scopes
            .iter()
            .find(|candidate| candidate.id == reaction.downstream_scope)
            .ok_or("semantic build chain reaction references a missing downstream scope")?;
        let storage_path = normalized_relative_path(
            root,
            &format!(
                "{}/queue/{}/from-{}.json",
                contract.storage_chain_policy.storage_root, downstream.id, scope.id
            ),
        )?;
        let queued_state = QueuedChainReaction {
            upstream_scope: scope.id.clone(),
            downstream_scope: downstream.id.clone(),
            condition: reaction.condition.clone(),
            requested_action: SemanticBuildAction::Prepare,
            automatic_physical_execution: false,
            predecessor_watermark: watermark.state_hash.clone(),
            storage_path: display_relative(root, &storage_path),
        };
        if storage_path.is_file() {
            let existing: QueuedChainReaction = read_json(&storage_path)?;
            if existing.predecessor_watermark != queued_state.predecessor_watermark {
                return Err(format!(
                    "KSR rejected conflicting predecessor watermark for queued downstream scope '{}'",
                    downstream.id
                ));
            }
        } else {
            write_json_atomically(&storage_path, &queued_state)?;
        }
        queued.push(queued_state);
    }
    Ok(queued)
}

fn write_json_atomically<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or("semantic build storage path has no parent directory")?;
    fs::create_dir_all(parent).map_err(|error| {
        format!(
            "failed to create semantic build storage directory '{}': {error}",
            parent.display()
        )
    })?;
    let temporary = path.with_extension("json.tmp");
    let content = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("failed to serialize semantic build storage state: {error}"))?;
    fs::write(&temporary, content).map_err(|error| {
        format!(
            "failed to write semantic build storage state '{}': {error}",
            temporary.display()
        )
    })?;
    fs::rename(&temporary, path).map_err(|error| {
        format!(
            "failed to atomically commit semantic build storage state '{}': {error}",
            path.display()
        )
    })
}

fn validate_contract(contract: &BuildDialectContract) -> Result<(), String> {
    if contract.schema != "braxon.nsq.semantic_build_dialect_contract.v1"
        || contract.authority != "NSQ_KINETIC_SEMANTIC_REFLEXOR"
        || contract.capability != SEMANTIC_BUILD_DIALECT_CAPABILITY
        || contract.target_environment != "aarch64-linux-android"
        || contract.nsq_dialect != "control"
        || contract.semantic_subdialect != "kinetic_build"
        || contract.execution_model
            != "semantic_intent_to_tokenizer_to_parameter_citadel_to_virtual_shared_cells_to_kinetic_reflexor_to_functional_watermark_to_allowlisted_physical_executor_to_artifact_proof"
        || !contract.watermark_is_functional
        || contract.resident_runtime
        || contract.hidden_download_allowed
        || contract.reflexor_job_bound
        || contract.storage_chain_policy.storage_root
            != "state/full_android_language_toolchain/ksr_semantic_chain"
        || !contract.storage_chain_policy.persist_declared_transition_state
        || contract.storage_chain_policy.chain_reaction_mode
            != "watermark_governed_declared_dependency_propagation"
        || contract.storage_chain_policy.auto_execute_physical_compiler
        || contract.storage_chain_policy.cycle_policy
            != "reject_contract_cycles_and_reject_stale_or_conflicting_predecessor_watermarks"
        || contract.storage_chain_policy.propagation_requires
            != vec![
                "committed_ksr_state_watermark".to_string(),
                "declared_scope_dependency".to_string(),
                "declared_output_or_preparation_state".to_string(),
            ]
        || contract.storage_chain_policy.downstream_effect
            != "persist_queued_semantic_preparation_only_until_a_separate_explicit_execute_action"
        || contract.storage_chain_policy.automatic_tool_state_population
            != "persist_verified_repository_built_tool_state_from_declared_output_hashes_only"
        || contract.storage_chain_policy.automatic_runtime_activation
        || !contract.storage_chain_policy.undo_available_for_every_declared_scope
        || contract.storage_chain_policy.undo_mode
            != "atomically_restore_prior_verified_storage_materialization_and_record_new_reversal_watermark"
        || contract.storage_chain_policy.undo_may_execute_physical_compiler
        || contract.storage_chain_policy.undo_may_delete_or_rewrite_artifacts
        || contract.shared_cell_policy.address_namespace != "council/0/build"
        || contract.shared_cell_policy.piston_lifecycle
            != vec![
                "acquire".to_string(),
                "hold".to_string(),
                "commit".to_string(),
                "release".to_string(),
            ]
        || contract.shared_cell_policy.same_address_concurrent_write != "queued"
        || contract.shared_cell_policy.source_mutation
            != "forbidden_except_declared_repository_source_materialization"
        || contract.shared_cell_policy.artifact_mutation
            != "allowed_only_through_declared_executor"
        || contract.executor_policy.allowlisted_modes.is_empty()
        || !contract.executor_policy.requires_explicit_execute
        || !contract.executor_policy.requires_aarch64_android_target
        || contract.executor_policy.physical_default_jobs != 1
        || contract.executor_policy.allows_arbitrary_arguments
        || contract.executor_policy.allows_hidden_download
        || contract.executor_policy.failure_mode
            != "fail_closed_with_exact_target_or_materialization_guidance"
        || contract.scopes.is_empty()
    {
        return Err("semantic build dialect contract is invalid or weakens KSR authority".into());
    }
    if contract.scopes.iter().any(|scope| !scope.allowed_actions.iter().any(|action| action == "undo")) {
        return Err("semantic build undo is not available for every declared scope".into());
    }
    if contract.automatic_tool_population.is_empty()
        || contract.automatic_tool_population.iter().any(|population| {
            population.scope.trim().is_empty()
                || population.output_root.trim().is_empty()
                || population.tool_names.is_empty()
                || population.state != "verified_repository_built_pending_target_proof_completion"
        })
    {
        return Err("semantic build automatic tool-state population contract is invalid".into());
    }
    validate_chain_reactions(contract)?;
    Ok(())
}

fn validate_chain_reactions(contract: &BuildDialectContract) -> Result<(), String> {
    let declared = contract
        .scopes
        .iter()
        .map(|scope| scope.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut graph = BTreeMap::<String, Vec<String>>::new();
    for reaction in &contract.chain_reactions {
        if !declared.contains(reaction.upstream_scope.as_str())
            || !declared.contains(reaction.downstream_scope.as_str())
            || reaction.upstream_scope == reaction.downstream_scope
            || !matches!(
                reaction.condition.as_str(),
                "prepared_or_executed" | "executed_with_declared_outputs_watermarked"
            )
            || reaction.effect != "queue_prepare"
        {
            return Err("semantic build chain reaction is undeclared, cyclic, or unsafe".into());
        }
        graph
            .entry(reaction.upstream_scope.clone())
            .or_default()
            .push(reaction.downstream_scope.clone());
    }
    let mut visited = BTreeSet::new();
    let mut visiting = BTreeSet::new();
    for scope in &declared {
        validate_chain_node(*scope, &graph, &mut visiting, &mut visited)?;
    }
    Ok(())
}

fn validate_chain_node(
    scope: &str,
    graph: &BTreeMap<String, Vec<String>>,
    visiting: &mut BTreeSet<String>,
    visited: &mut BTreeSet<String>,
) -> Result<(), String> {
    if visited.contains(scope) {
        return Ok(());
    }
    if !visiting.insert(scope.to_string()) {
        return Err("semantic build chain reaction graph contains a cycle".into());
    }
    if let Some(downstream) = graph.get(scope) {
        for next in downstream {
            validate_chain_node(next, graph, visiting, visited)?;
        }
    }
    visiting.remove(scope);
    visited.insert(scope.to_string());
    Ok(())
}

fn validate_scope(scope: &BuildScope, policy: &ExecutorPolicy) -> Result<(), String> {
    if scope.id.trim().is_empty()
        || scope.kind.trim().is_empty()
        || scope.languages.is_empty()
        || scope.allowed_actions.is_empty()
        || scope.target_state.trim().is_empty()
        || scope.required_paths.is_empty()
        || scope.proof_requirements.is_empty()
        || scope.optimization_profile.trim().is_empty()
    {
        return Err("semantic build scope is incomplete".into());
    }
    if let Some(mode) = &scope.executor_mode {
        if !policy.allowlisted_modes.iter().any(|allowed| allowed == mode) {
            return Err(format!(
                "semantic build scope '{}' selects an executor outside the KSR allowlist",
                scope.id
            ));
        }
    }
    Ok(())
}

fn path_states(root: &Path, paths: &[String]) -> Result<Vec<BuildPathState>, String> {
    paths
        .iter()
        .map(|relative| {
            let absolute = normalized_relative_path(root, relative)?;
            let state = if absolute.is_file() {
                let bytes = fs::read(&absolute).map_err(|error| {
                    format!("failed to read declared build path '{}': {error}", relative)
                })?;
                BuildPathState {
                    path: relative.clone(),
                    present: true,
                    bytes: Some(u64::try_from(bytes.len()).map_err(|_| {
                        format!("declared build path '{}' byte count overflows u64", relative)
                    })?),
                    sha256: Some(sha256_hex(&bytes)),
                }
            } else if absolute.is_dir() {
                BuildPathState {
                    path: relative.clone(),
                    present: true,
                    bytes: None,
                    sha256: None,
                }
            } else {
                BuildPathState {
                    path: relative.clone(),
                    present: false,
                    bytes: None,
                    sha256: None,
                }
            };
            Ok(state)
        })
        .collect()
}

fn normalized_relative_path(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let path = Path::new(relative);
    if relative.trim().is_empty()
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir
                    | std::path::Component::RootDir
                    | std::path::Component::Prefix(_)
            )
        })
    {
        return Err("semantic build contract contains a non-normalized repository-relative path".into());
    }
    Ok(root.join(path))
}

fn build_native_bus(policy: &SharedCellPolicy) -> Result<NsqNativeBus, String> {
    NsqNativeBus::new((0..10).map(|index| CouncilSurface {
        surface_id: format!("semantic-build-surface-{index}"),
        role: if index == 0 { "build" } else { "reserved" }.to_string(),
        address_prefix: format!("council/{index}/"),
        active: index == 0 && policy.address_namespace.starts_with("council/0/"),
    }))
}

fn build_cells(
    namespace: &str,
    scope: &str,
    intent_text: &str,
    inputs: &[BuildPathState],
    outputs: &[BuildPathState],
) -> Vec<SharedBuildCell> {
    let prefix = format!("{namespace}/{scope}");
    let values = [
        ("intent", intent_text.to_string()),
        ("source", summarize_path_states(inputs)),
        ("executor", scope.to_string()),
        ("artifact", summarize_path_states(outputs)),
    ];
    values
        .into_iter()
        .map(|(port, value)| SharedBuildCell {
            address: format!("{prefix}/{port}"),
            port: port.to_string(),
            value_sha256: sha256_hex(value.as_bytes()),
            byte_len: u64::try_from(value.len()).unwrap_or(u64::MAX).max(1),
        })
        .collect()
}

fn build_context(inputs: &[BuildPathState], outputs: &[BuildPathState]) -> Result<i64, String> {
    let material = format!("{}|{}", summarize_path_states(inputs), summarize_path_states(outputs));
    let mut value = 0i64;
    for byte in material.bytes() {
        value = value
            .wrapping_mul(257)
            .wrapping_add(i64::from(byte).saturating_add(1));
    }
    if value == i64::MIN {
        Err("semantic build context reached the reserved minimum parameter value".to_string())
    } else {
        Ok(value)
    }
}

fn summarize_path_states(states: &[BuildPathState]) -> String {
    states
        .iter()
        .map(|entry| {
            format!(
                "{}:{}:{}",
                entry.path,
                entry.present,
                entry.sha256.as_deref().unwrap_or("unhashed")
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn executor_boundary(
    root: &Path,
    contract: &BuildDialectContract,
    scope: &BuildScope,
    action: SemanticBuildAction,
    target_matches: bool,
    missing_inputs: &[String],
    outputs_ready: bool,
    replacement_authorized: bool,
) -> Result<BuildExecutorBoundary, String> {
    let dispatcher = normalized_relative_path(root, &contract.executor_policy.dispatcher)?;
    let mode = scope.executor_mode.clone();
    let executor_permitted = mode
        .as_ref()
        .map(|selected| contract.executor_policy.allowlisted_modes.contains(selected))
        .unwrap_or(false);
    let exact_guidance = if action != SemanticBuildAction::Execute {
        "semantic intent, virtual shared cells, and functional state watermark are committed; execution remains explicit".to_string()
    } else if mode.is_none() {
        format!(
            "scope '{}' has no declared physical executor; it remains {} until a separately reviewed source-build route is added",
            scope.id, scope.target_state
        )
    } else if contract.executor_policy.requires_aarch64_android_target && !target_matches {
        format!(
            "semantic admission completed, but physical compilation is target-bound: rerun this exact command on {}",
            contract.target_environment
        )
    } else if !missing_inputs.is_empty() {
        format!(
            "execution is not admitted because only this scope's declared inputs are missing: {}",
            missing_inputs.join(", ")
        )
    } else if !dispatcher.is_file() {
        format!(
            "declared KSR physical dispatcher '{}' is unavailable",
            contract.executor_policy.dispatcher
        )
    } else if scope.id == "llvm-source-edge" && !outputs_ready && !replacement_authorized {
        "an incomplete LLVM source tree may require replacement. Preserve any local work, then rerun the same scope with --replace-incomplete-llvm; KSR will pass no other user-controlled environment or arguments".to_string()
    } else {
        "all bounded semantic and target conditions are present; the allowlisted physical executor may run exactly once for this declared scope".to_string()
    };
    Ok(BuildExecutorBoundary {
        dispatcher: Some(contract.executor_policy.dispatcher.clone()),
        selected_mode: mode,
        target_environment_matches: target_matches,
        explicit_execute_requested: action == SemanticBuildAction::Execute,
        executor_permitted,
        execution_attempted: false,
        execution_succeeded: false,
        exit_code: None,
        stdout_log: None,
        stderr_log: None,
        stdout_sha256: None,
        stderr_sha256: None,
        conditional_source_replacement_authorized: replacement_authorized,
        exact_guidance,
    })
}

fn run_declared_executor(
    root: &Path,
    policy: &ExecutorPolicy,
    scope: &BuildScope,
    replacement_authorized: bool,
    state_watermark: &Watermark,
) -> Result<BuildExecutorBoundary, String> {
    let mode = scope
        .executor_mode
        .as_ref()
        .ok_or("scope has no declared executor")?;
    if !policy.allowlisted_modes.contains(mode) {
        return Err("KSR rejected executor mode outside its allowlist".into());
    }
    let dispatcher = normalized_relative_path(root, &policy.dispatcher)?;
    if !dispatcher.is_file() {
        return Err(format!("declared physical dispatcher '{}' is absent", policy.dispatcher));
    }
    if scope.id == "llvm-source-edge" && llvm_tree_is_nonempty(root)? && !llvm_outputs_present(root)? && !replacement_authorized {
        return Err("KSR requires --replace-incomplete-llvm before it may replace an incomplete LLVM source tree".into());
    }
    let logs = root
        .join("state/full_android_language_toolchain/ksr_execution_logs")
        .join(&scope.id);
    fs::create_dir_all(&logs).map_err(|error| {
        format!(
            "failed to create bounded KSR execution evidence directory '{}': {error}",
            logs.display()
        )
    })?;
    let stdout_path = logs.join("stdout.log");
    let stderr_path = logs.join("stderr.log");
    let stdout = File::create(&stdout_path)
        .map_err(|error| format!("failed to create KSR stdout evidence: {error}"))?;
    let stderr = File::create(&stderr_path)
        .map_err(|error| format!("failed to create KSR stderr evidence: {error}"))?;
    let mut command = Command::new(&dispatcher);
    command.current_dir(root).arg(mode).env_clear();
    for variable in &policy.preserved_environment {
        if let Some(value) = env::var_os(variable) {
            command.env(variable, value);
        }
    }
    for (key, value) in &policy.controlled_environment {
        command.env(key, value);
    }
    command
        .env("BRAXON_KSR_SEMANTIC_BUILD_CAPABILITY", SEMANTIC_BUILD_DIALECT_CAPABILITY)
        .env("BRAXON_KSR_SEMANTIC_BUILD_SCOPE", &scope.id)
        .env("BRAXON_KSR_SEMANTIC_BUILD_WATERMARK", &state_watermark.state_hash)
        .env("BRAXON_KSR_SEMANTIC_BUILD_ACTION", "execute");
    if scope.id == "llvm-source-edge" && replacement_authorized {
        command.env("BRAXON_REPLACE_INCOMPLETE_LLVM_SOURCE", "1");
    }
    let status = command
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .status()
        .map_err(|error| format!("failed to launch KSR-selected physical executor: {error}"))?;
    let stdout_bytes = fs::read(&stdout_path)
        .map_err(|error| format!("failed to read KSR stdout evidence: {error}"))?;
    let stderr_bytes = fs::read(&stderr_path)
        .map_err(|error| format!("failed to read KSR stderr evidence: {error}"))?;
    let execution_succeeded = status.success();
    let exact_guidance = if execution_succeeded {
        "the allowlisted physical executor returned success; KSR will now require every declared output before it accepts artifact materialization".to_string()
    } else {
        format!(
            "the allowlisted physical executor exited with {:?}; inspect its bounded stdout/stderr evidence and repair only the declared missing target or source condition",
            status.code()
        )
    };
    Ok(BuildExecutorBoundary {
        dispatcher: Some(policy.dispatcher.clone()),
        selected_mode: Some(mode.clone()),
        target_environment_matches: true,
        explicit_execute_requested: true,
        executor_permitted: true,
        execution_attempted: true,
        execution_succeeded,
        exit_code: status.code(),
        stdout_log: Some(display_relative(root, &stdout_path)),
        stderr_log: Some(display_relative(root, &stderr_path)),
        stdout_sha256: Some(sha256_hex(&stdout_bytes)),
        stderr_sha256: Some(sha256_hex(&stderr_bytes)),
        conditional_source_replacement_authorized: replacement_authorized,
        exact_guidance,
    })
}

fn commit_artifact_watermark(scope: &str, outputs: &[BuildPathState]) -> Result<Watermark, String> {
    let values = outputs
        .iter()
        .filter_map(|output| {
            output.sha256.as_ref().map(|hash| BusValue {
                key: format!("nsq/build-artifact/{scope}/{}", output.path),
                class: ValueClass::Fact,
                value_hash: hash.clone(),
                byte_len: output.bytes.unwrap_or(1).max(1),
            })
        })
        .collect::<Vec<_>>();
    if values.is_empty() {
        return Err("KSR cannot watermark an empty declared build output set".into());
    }
    let mut reflexor = KineticReflexor::new();
    reflexor.publish(values)?;
    reflexor.reconcile()?;
    let keys = reflexor
        .pending_delta()
        .iter()
        .map(|delta| delta.key.clone())
        .collect::<Vec<_>>();
    let generation = reflexor.generation();
    let commit = reflexor.commit_hardware(HardwareWriteAck {
        adapter_id: "nsq_semantic_build_artifact_adapter".to_string(),
        generation,
        accepted: true,
        written_keys: keys,
    })?;
    if !commit.hardware_write_acknowledged {
        return Err("KSR did not acknowledge the declared artifact watermark".into());
    }
    Ok(commit.watermark)
}

fn llvm_tree_is_nonempty(root: &Path) -> Result<bool, String> {
    let source = root.join("state/full_android_language_toolchain/src/llvm-project");
    if !source.is_dir() {
        return Ok(false);
    }
    let mut entries = fs::read_dir(source)
        .map_err(|error| format!("failed to inspect LLVM source materialization boundary: {error}"))?;
    Ok(entries.next().transpose().map_err(|error| error.to_string())?.is_some())
}

fn llvm_outputs_present(root: &Path) -> Result<bool, String> {
    let paths = [
        "state/full_android_language_toolchain/src/llvm-project/llvm/CMakeLists.txt",
        "state/full_android_language_toolchain/src/llvm-project/llvm/lib/Demangle/CMakeLists.txt",
        "state/full_android_language_toolchain/src/llvm-project/llvm/lib/Support/CMakeLists.txt",
        "state/full_android_language_toolchain/src/llvm-project/llvm/lib/TableGen/CMakeLists.txt",
        "state/full_android_language_toolchain/source_receipts/llvm-project-eaab4d9841b9a8a12783d927b2df2291c1c79269.txt",
    ];
    Ok(paths.iter().all(|relative| root.join(relative).is_file()))
}

fn next_action(
    scope: &BuildScope,
    action: SemanticBuildAction,
    missing_inputs: &[String],
    target_matches: bool,
    outputs_ready_before: bool,
    executor: &BuildExecutorBoundary,
    status: SemanticBuildStatus,
) -> String {
    if !missing_inputs.is_empty() {
        return format!(
            "Resolve only the declared inputs for '{}': {}. Then rerun `Braxon toolchain build-dialect {} prepare`.",
            scope.id,
            missing_inputs.join(", "),
            scope.id
        );
    }
    match status {
        SemanticBuildStatus::Inspected => format!(
            "The scope is semantically admitted but not prepared for physical mutation. Run `Braxon toolchain build-dialect {} prepare`.",
            scope.id
        ),
        SemanticBuildStatus::Prepared => {
            if scope.executor_mode.is_none() {
                format!(
                    "'{}' is bounded semantic preparation only and remains {}. No physical runtime build is declared or claimed.",
                    scope.id, scope.target_state
                )
            } else if !target_matches {
                format!(
                    "Preparation is valid, but execute only on native aarch64-linux-android: `Braxon toolchain build-dialect {} execute`.",
                    scope.id
                )
            } else if scope.id == "llvm-source-edge" && !outputs_ready_before {
                format!(
                    "On a fresh source tree run `Braxon toolchain build-dialect {} execute`. If KSR reports an incomplete existing LLVM tree, preserve local work and rerun with `--replace-incomplete-llvm`.",
                    scope.id
                )
            } else {
                format!(
                    "Run the explicitly authorized target transaction: `Braxon toolchain build-dialect {} execute`.",
                    scope.id
                )
            }
        }
        SemanticBuildStatus::TargetBuildPending => executor.exact_guidance.clone(),
        SemanticBuildStatus::Executed => format!(
            "'{}' emitted declared outputs through KSR's allowlisted executor. Perform the remaining target proof requirements before activation: {}.",
            scope.id,
            scope.proof_requirements.join(", ")
        ),
        SemanticBuildStatus::ExecutionFailed => executor.exact_guidance.clone(),
        SemanticBuildStatus::Reverted => "KSR restored the prior verified semantic materialization. Compiled artifacts remain immutable; run inspect to review the restored state or prepare to create a new declared transition.".to_string(),
    }
}

fn display_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    format!("{:x}", digest.finalize())
}

fn resolve_root(start: impl AsRef<Path>) -> Result<PathBuf, String> {
    let canonical = start
        .as_ref()
        .canonicalize()
        .map_err(|error| format!("unable to resolve semantic build dialect start: {error}"))?;
    canonical
        .ancestors()
        .find(|candidate| candidate.join(CONTRACT_RELATIVE_PATH).is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| "unable to locate semantic build dialect contract".to_string())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("failed to read '{}': {error}", path.display()))?;
    serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse '{}': {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repository root")
    }

    #[test]
    fn every_declared_scope_is_a_bounded_krs_semantic_transaction() {
        let root = repository_root();
        let contract: BuildDialectContract = read_json(&root.join(CONTRACT_RELATIVE_PATH)).unwrap();
        for scope in contract.scopes {
            let report = execute_semantic_build_dialect(&root, &scope.id, SemanticBuildAction::Inspect, false)
                .unwrap_or_else(|error| panic!("{}: {error}", scope.id));
            assert_eq!(report.scope, scope.id);
            assert!(report.parameter_invariants_passed);
            assert!(report.state_watermark_committed);
            assert!(report.scope_is_bounded);
            assert!(report.no_resident_runtime);
            assert!(!report.hidden_download_allowed);
            assert!(report.virtual_shared_cells.iter().all(|cell| cell.address.starts_with("council/0/build/")));
            assert!(root.join(&report.storage_chain_state_path).is_file());
            assert!(report.queued_chain_reactions.iter().all(|reaction| !reaction.automatic_physical_execution));
        }
    }

    #[test]
    fn host_execute_fails_closed_without_claiming_android_compilation() {
        let report = execute_semantic_build_dialect(
            repository_root(),
            "llvm-source-edge",
            SemanticBuildAction::Execute,
            false,
        )
        .unwrap();
        if !cfg!(all(target_arch = "aarch64", target_os = "android")) {
            assert_eq!(report.status, SemanticBuildStatus::TargetBuildPending);
            assert!(!report.executor.execution_attempted);
            assert!(!report.executor.execution_succeeded);
            assert!(report.executor.exact_guidance.contains("aarch64-linux-android"));
        }
    }

    #[test]
    fn runtime_candidates_are_semantic_boundaries_not_pure_llvm_claims() {
        let report = execute_semantic_build_dialect(
            repository_root(),
            "openjdk",
            SemanticBuildAction::Prepare,
            false,
        )
        .unwrap();
        assert_eq!(report.status, SemanticBuildStatus::Prepared);
        assert!(report.executor.selected_mode.is_none());
        assert!(report.semantic_boundary.contains("upstream_implementation_semantics"));
        assert!(report.exact_next_action.contains("No physical runtime build is declared"));
    }

    #[test]
    fn executed_artifact_hashes_reflexively_populate_tool_state_without_runtime_activation() {
        let temporary = env::temp_dir().join(format!(
            "braxon-semantic-build-tool-state-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&temporary);
        fs::create_dir_all(&temporary).unwrap();
        let policy = StorageChainPolicy {
            storage_root: "state/chain".to_string(),
            persist_declared_transition_state: true,
            chain_reaction_mode: "watermark_governed_declared_dependency_propagation".to_string(),
            auto_execute_physical_compiler: false,
            cycle_policy: "reject_contract_cycles_and_reject_stale_or_conflicting_predecessor_watermarks".to_string(),
            propagation_requires: vec![],
            downstream_effect: "persist_queued_semantic_preparation_only_until_a_separate_explicit_execute_action".to_string(),
            automatic_tool_state_population: "persist_verified_repository_built_tool_state_from_declared_output_hashes_only".to_string(),
            automatic_runtime_activation: false,
            undo_available_for_every_declared_scope: true,
            undo_mode: "atomically_restore_prior_verified_storage_materialization_and_record_new_reversal_watermark".to_string(),
            undo_may_execute_physical_compiler: false,
            undo_may_delete_or_rewrite_artifacts: false,
        };
        let scope = BuildScope {
            id: "llvm-aarch64-source-build".to_string(),
            kind: "compiler_materialization".to_string(),
            languages: vec!["llvm_ir".to_string()],
            allowed_actions: vec!["execute".to_string()],
            executor_mode: Some("source-build".to_string()),
            target_state: "TARGET_BUILD_PENDING".to_string(),
            required_paths: vec!["source".to_string()],
            proof_requirements: vec!["artifact_watermark".to_string()],
            output_paths: vec![],
            optimization_profile: "release".to_string(),
        };
        let population = AutomaticToolPopulation {
            scope: scope.id.clone(),
            output_root: "state/install/llvm/bin".to_string(),
            tool_names: vec!["clang".to_string()],
            state: "verified_repository_built_pending_target_proof_completion".to_string(),
        };
        let outputs = vec![BuildPathState {
            path: "state/install/llvm/bin/clang".to_string(),
            present: true,
            bytes: Some(1),
            sha256: Some("a".repeat(64)),
        }];
        let populated = populate_verified_tool_state(
            &temporary,
            &policy,
            &[population],
            &scope,
            &SemanticBuildStatus::Executed,
            &outputs,
        )
        .unwrap();
        assert_eq!(populated.len(), 1);
        assert!(!populated[0].runtime_activated);
        assert!(temporary.join(&populated[0].storage_path).is_file());
        let _ = fs::remove_dir_all(&temporary);
    }

    #[test]
    fn storage_chain_is_unbounded_by_job_count_but_never_auto_executes_compilers() {
        let contract: BuildDialectContract =
            read_json(&repository_root().join(CONTRACT_RELATIVE_PATH)).unwrap();
        assert!(!contract.reflexor_job_bound);
        assert!(!contract.storage_chain_policy.auto_execute_physical_compiler);
        assert!(!contract.storage_chain_policy.automatic_runtime_activation);
        assert_eq!(contract.executor_policy.physical_default_jobs, 1);
        validate_chain_reactions(&contract).unwrap();
    }

    #[test]
    fn universal_undo_is_krs_routed_and_never_grants_physical_or_artifact_mutation() {
        let contract: BuildDialectContract =
            read_json(&repository_root().join(CONTRACT_RELATIVE_PATH)).unwrap();
        assert_eq!(SemanticBuildAction::parse("undo").unwrap(), SemanticBuildAction::Undo);
        assert!(contract.scopes.iter().all(|scope| scope.allowed_actions.contains(&"undo".to_string())));
        assert!(contract.storage_chain_policy.undo_available_for_every_declared_scope);
        assert!(!contract.storage_chain_policy.undo_may_execute_physical_compiler);
        assert!(!contract.storage_chain_policy.undo_may_delete_or_rewrite_artifacts);
    }

    #[test]
    fn unknown_scope_and_undeclared_action_fail_closed() {
        let unknown = execute_semantic_build_dialect(
            repository_root(),
            "not-a-scope",
            SemanticBuildAction::Inspect,
            false,
        )
        .expect_err("unknown scope must fail");
        assert!(unknown.contains("not declared"));
        let action = execute_semantic_build_dialect(
            repository_root(),
            "quickjs",
            SemanticBuildAction::Execute,
            false,
        )
        .expect_err("undeclared execute must fail");
        assert!(action.contains("not declared"));
    }
}
