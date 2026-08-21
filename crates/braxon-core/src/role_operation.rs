use crate::{
    bootstrap_live_bus, execute_operator_intelligence, IntelligentOperation, LiveBusBootstrapReport,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const ROLE_OPERATION_SCHEMA: &str = "braxon.nsq.role_operation.v1";
pub const ROLE_OPERATION_CAPABILITY: &str = "feature:role.operation";
pub const ROLE_OPERATION_CONTRACT_RELATIVE_PATH: &str = "config/nsq/role_operation_contract.json";
pub const ROLE_OPERATION_STATE_RELATIVE_DIR: &str = "state/reflex/role_operations";

#[derive(Debug, Clone, Deserialize)]
struct RoleOperationContract {
    schema: String,
    authority: String,
    owner: String,
    execution_policy: String,
    state_policy: String,
    modes: Vec<RoleModeContract>,
}

#[derive(Debug, Clone, Deserialize)]
struct RoleModeContract {
    id: String,
    label: String,
    lead_office: String,
    supporting_offices: Vec<String>,
    authorized_features: Vec<String>,
    purpose: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CourtConfig {
    offices: BTreeMap<String, CourtOffice>,
}

#[derive(Debug, Clone, Deserialize)]
struct CourtOffice {
    title: String,
    #[serde(rename = "class")]
    durability_class: String,
    authority_domain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtOfficeBinding {
    pub id: String,
    pub title: String,
    pub durability_class: String,
    pub authority_domain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleOperation {
    pub schema: String,
    pub capability: String,
    pub owner: String,
    pub mode: String,
    pub mode_label: String,
    pub request: String,
    pub purpose: String,
    pub lead_office: CourtOfficeBinding,
    pub supporting_offices: Vec<CourtOfficeBinding>,
    pub authorized_features: Vec<String>,
    pub execution_policy: String,
    pub state_policy: String,
    pub live_bus_bootstrap: LiveBusBootstrapReport,
    pub intelligent_operation: IntelligentOperation,
    pub completed: bool,
    pub state_path: String,
}

/// Execute one role-bound operation. The court contract determines who may perform
/// the operation; the live bus and operator intelligence engine perform the actual
/// bounded NSQ transaction. State is emitted only after the transaction has released
/// all address leases successfully.
pub fn execute_role_operation(
    root: impl AsRef<Path>,
    mode_id: &str,
    request: impl AsRef<str>,
) -> Result<RoleOperation, String> {
    let root = root.as_ref();
    let request = request.as_ref().trim();
    if request.is_empty() {
        return Err("role operation requires a nonempty request".to_string());
    }

    let contract = load_role_operation_contract(root)?;
    let mode = contract
        .modes
        .iter()
        .find(|mode| mode.id == mode_id)
        .ok_or_else(|| format!("unknown declared role operation mode: {mode_id}"))?;
    validate_mode(mode)?;

    let court = load_court_config(root)?;
    let lead_office = resolve_office(&court, &mode.lead_office)?;
    let supporting_offices = mode
        .supporting_offices
        .iter()
        .map(|office| resolve_office(&court, office))
        .collect::<Result<Vec<_>, _>>()?;

    let routing_intent = format!(
        "{} role {} via {}: {}",
        mode.label, mode.id, lead_office.id, request
    );
    let live_bus_bootstrap = bootstrap_live_bus(root, &routing_intent)?;
    let intelligent_operation = execute_operator_intelligence(&routing_intent)?;
    if !intelligent_operation.action.completed
        || !intelligent_operation.lease_released
        || intelligent_operation.native_fired_count == 0
    {
        return Err(format!(
            "role operation failed to complete its NSQ transaction: completed={} fired={} released={}",
            intelligent_operation.action.completed,
            intelligent_operation.native_fired_count,
            intelligent_operation.lease_released
        ));
    }

    let state_path = role_operation_state_path(root, mode_id, request);
    let operation = RoleOperation {
        schema: ROLE_OPERATION_SCHEMA.to_string(),
        capability: ROLE_OPERATION_CAPABILITY.to_string(),
        owner: contract.owner,
        mode: mode.id.clone(),
        mode_label: mode.label.clone(),
        request: request.to_string(),
        purpose: mode.purpose.clone(),
        lead_office,
        supporting_offices,
        authorized_features: mode.authorized_features.clone(),
        execution_policy: contract.execution_policy,
        state_policy: contract.state_policy,
        live_bus_bootstrap,
        intelligent_operation,
        completed: true,
        state_path: state_path.display().to_string(),
    };
    write_role_operation_state(&state_path, &operation)?;
    Ok(operation)
}

pub fn available_role_modes(root: impl AsRef<Path>) -> Result<Vec<String>, String> {
    let contract = load_role_operation_contract(root.as_ref())?;
    let court = load_court_config(root.as_ref())?;
    contract
        .modes
        .iter()
        .map(|mode| {
            validate_mode(mode)?;
            resolve_office(&court, &mode.lead_office)?;
            for office in &mode.supporting_offices {
                resolve_office(&court, office)?;
            }
            Ok(mode.id.clone())
        })
        .collect()
}

fn load_role_operation_contract(root: &Path) -> Result<RoleOperationContract, String> {
    let path = root.join(ROLE_OPERATION_CONTRACT_RELATIVE_PATH);
    let raw = fs::read_to_string(&path).map_err(|error| {
        format!(
            "unable to read role operation contract {}: {error}",
            path.display()
        )
    })?;
    let contract: RoleOperationContract = serde_json::from_str(&raw).map_err(|error| {
        format!(
            "invalid role operation contract {}: {error}",
            path.display()
        )
    })?;
    if contract.schema != "braxon.nsq.role_operation_contract.v1" {
        return Err(format!(
            "unsupported role operation contract schema: {}",
            contract.schema
        ));
    }
    if contract.authority != "NSQ kinetic semantic reflexor" {
        return Err(
            "role operation contract must declare NSQ kinetic semantic reflexor authority"
                .to_string(),
        );
    }
    if contract.owner.trim().is_empty()
        || contract.execution_policy.trim().is_empty()
        || contract.state_policy.trim().is_empty()
    {
        return Err("role operation contract lacks owner or execution/state policy".to_string());
    }
    Ok(contract)
}

fn load_court_config(root: &Path) -> Result<CourtConfig, String> {
    let path = root.join("config/braxon_court.json");
    let raw = fs::read_to_string(&path).map_err(|error| {
        format!(
            "unable to read court configuration {}: {error}",
            path.display()
        )
    })?;
    let court: CourtConfig = serde_json::from_str(&raw)
        .map_err(|error| format!("invalid court configuration {}: {error}", path.display()))?;
    if court.offices.is_empty() {
        return Err("court configuration declares no offices".to_string());
    }
    Ok(court)
}

fn validate_mode(mode: &RoleModeContract) -> Result<(), String> {
    if mode.id.trim().is_empty()
        || mode.label.trim().is_empty()
        || mode.lead_office.trim().is_empty()
        || mode.purpose.trim().is_empty()
    {
        return Err("role operation mode has an empty required field".to_string());
    }
    for required in [
        ROLE_OPERATION_CAPABILITY,
        "feature:operator.intelligence",
        "feature:live_bus.bootstrap",
    ] {
        if required != ROLE_OPERATION_CAPABILITY
            && !mode
                .authorized_features
                .iter()
                .any(|feature| feature == required)
        {
            return Err(format!(
                "role mode {} does not authorize required feature {required}",
                mode.id
            ));
        }
    }
    if mode
        .supporting_offices
        .iter()
        .any(|office| office == &mode.lead_office)
    {
        return Err(format!(
            "role mode {} repeats its lead office as a support office",
            mode.id
        ));
    }
    Ok(())
}

fn resolve_office(court: &CourtConfig, office_id: &str) -> Result<CourtOfficeBinding, String> {
    let office = court
        .offices
        .get(office_id)
        .ok_or_else(|| format!("role operation references unknown court office: {office_id}"))?;
    if office.title.trim().is_empty()
        || office.durability_class.trim().is_empty()
        || office.authority_domain.is_empty()
    {
        return Err(format!(
            "court office {office_id} lacks executable authority metadata"
        ));
    }
    Ok(CourtOfficeBinding {
        id: office_id.to_string(),
        title: office.title.clone(),
        durability_class: office.durability_class.clone(),
        authority_domain: office.authority_domain.clone(),
    })
}

fn role_operation_state_path(root: &Path, mode: &str, request: &str) -> PathBuf {
    root.join(ROLE_OPERATION_STATE_RELATIVE_DIR).join(format!(
        "{}-{}.json",
        sanitize_identifier(mode),
        stable_hash(request)
    ))
}

fn write_role_operation_state(path: &Path, operation: &RoleOperation) -> Result<(), String> {
    let parent = path.parent().ok_or_else(|| {
        format!(
            "role operation state path has no parent: {}",
            path.display()
        )
    })?;
    fs::create_dir_all(parent).map_err(|error| {
        format!(
            "unable to create role operation state directory {}: {error}",
            parent.display()
        )
    })?;
    let body = serde_json::to_string_pretty(operation)
        .map_err(|error| format!("unable to serialize role operation state: {error}"))?;
    fs::write(path, body).map_err(|error| {
        format!(
            "unable to write role operation state {}: {error}",
            path.display()
        )
    })
}

fn sanitize_identifier(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_role_modes_resolve_to_real_court_offices() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let modes = available_role_modes(&root).unwrap();
        assert_eq!(
            modes,
            vec!["assistant", "designer", "agent", "worker", "personal"]
        );
    }

    #[test]
    fn role_state_path_is_deterministic_and_request_specific() {
        let root = Path::new("/tmp/braxon-role-test");
        let first = role_operation_state_path(root, "assistant", "inspect root");
        let second = role_operation_state_path(root, "assistant", "inspect root");
        let third = role_operation_state_path(root, "assistant", "repair root");
        assert_eq!(first, second);
        assert_ne!(first, third);
    }
}
