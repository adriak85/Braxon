use crate::{capital::build_capitals, coaching::CoachingMode, seed::IntentSeed};
use nsq_core::{
    Charge, Dialect, NSQLever, NSQSlot, NativeNsqMachine, NativeNsqOwnership, NativeNsqRuntime,
    NsqActuationReceipt, NsqAddress, NsqInstruction, NsqLeasePhase, NsqTensor, NsqTensorStore,
    CANONICAL_LEVER_MAX_POSITION, NSQ_TENSOR_SCHEMA,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const CITADEL_MATERIALIZATION_SCHEMA: &str = "nsq.citadel.seed_materialization.v1";
pub const CITADEL_INVENTORY_SCHEMA: &str = "nsq.citadel.inventory.v1";
pub const CITADEL_DELTA_SCHEMA: &str = "nsq.citadel.delta.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CitadelDelta {
    pub schema: String,
    pub identity: String,
    pub target_tensor: String,
    pub base_generation: u64,
    pub delta_generation: u64,
    pub parent_materialization_hash: String,
    pub values: Vec<f32>,
    pub integrity_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitadelDeltaReceipt {
    pub identity: String,
    pub target_tensor: String,
    pub generation: u64,
    pub materialization_hash: String,
    pub activated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CitadelManifest {
    pub schema: String,
    pub lanes: Vec<CitadelManifestLane>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CitadelManifestLane {
    pub lane: String,
    pub model_id: String,
    pub source_repo: String,
    pub revision: String,
    pub artifact_family: String,
    pub bus_dialect: String,
    pub semantic_projection: String,
    #[serde(default)]
    pub independent_payload_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CitadelInventoryEntry {
    pub lane: String,
    pub pole_id: String,
    pub tensor_name: String,
    pub address: NsqAddressRecord,
    pub owner: NsqAddressRecord,
    pub source_seed_hash: String,
    pub materialization_hash: String,
    pub generation: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NsqAddressRecord {
    pub slots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CitadelInventory {
    pub schema: String,
    pub manifest_hash: String,
    pub generation: u64,
    pub entries: Vec<CitadelInventoryEntry>,
    pub inventory_hash: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CitadelTensorBody {
    pub pole_id: String,
    pub capital_id: usize,
    pub lane: usize,
    pub source_seed_id: String,
    pub source_seed_hash: String,
    pub tensor_name: String,
    pub shape: Vec<u64>,
    pub dtype: String,
    pub address: NsqAddress,
    pub owner: NsqAddress,
    pub generation: u64,
    pub materialization_hash: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CitadelMaterialization {
    pub schema: String,
    pub seed_id: String,
    pub seed_hash: String,
    pub generation: u64,
    pub bodies: Vec<CitadelTensorBody>,
    pub parameter_outputs: BTreeMap<String, f32>,
    pub receipt: NsqActuationReceipt,
}

#[derive(Debug)]
pub enum CitadelMaterializationError {
    InvalidSeed(String),
    Tensor(nsq_core::NativeTensorError),
    Runtime(String),
    Ownership(String),
    Manifest(String),
    Reconciliation(String),
    Delta(String),
}

impl std::fmt::Display for CitadelMaterializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSeed(message) => write!(f, "invalid Citadel seed: {message}"),
            Self::Tensor(error) => write!(f, "Citadel tensor error: {error}"),
            Self::Runtime(error) => write!(f, "Citadel runtime error: {error}"),
            Self::Ownership(error) => write!(f, "Citadel ownership error: {error}"),
            Self::Manifest(error) => write!(f, "Citadel manifest error: {error}"),
            Self::Reconciliation(error) => write!(f, "Citadel reconciliation error: {error}"),
            Self::Delta(error) => write!(f, "Citadel delta error: {error}"),
        }
    }
}

impl std::error::Error for CitadelMaterializationError {}

impl From<nsq_core::NativeTensorError> for CitadelMaterializationError {
    fn from(error: nsq_core::NativeTensorError) -> Self {
        Self::Tensor(error)
    }
}

pub struct CitadelNativeRuntime {
    pub coaching: CoachingMode,
    pub tensor_store: NsqTensorStore,
    pub runtime: NativeNsqRuntime<NativeNsqMachine>,
    pub ownership: NativeNsqOwnership,
    pub last_generation: u64,
    pub applied_deltas: BTreeSet<String>,
    pub active_delta_targets: BTreeMap<String, String>,
}

impl CitadelNativeRuntime {
    pub fn new(coaching: CoachingMode) -> Self {
        Self {
            coaching,
            tensor_store: NsqTensorStore::default(),
            runtime: NativeNsqRuntime::new(NativeNsqMachine::default()),
            ownership: NativeNsqOwnership::default(),
            last_generation: 0,
            applied_deltas: BTreeSet::new(),
            active_delta_targets: BTreeMap::new(),
        }
    }

    pub fn apply_delta(
        &mut self,
        delta: &CitadelDelta,
    ) -> Result<CitadelDeltaReceipt, CitadelMaterializationError> {
        if delta.schema != CITADEL_DELTA_SCHEMA {
            return Err(CitadelMaterializationError::Delta(
                "invalid delta schema".into(),
            ));
        }
        if delta.identity.trim().is_empty() || delta.target_tensor.trim().is_empty() {
            return Err(CitadelMaterializationError::Delta(
                "delta identity and target are required".into(),
            ));
        }
        if self.applied_deltas.contains(&delta.identity) {
            let tensor = self.tensor_store.get(&delta.target_tensor)?;
            return Ok(CitadelDeltaReceipt {
                identity: delta.identity.clone(),
                target_tensor: delta.target_tensor.clone(),
                generation: tensor.generation,
                materialization_hash: tensor.materialization_sha256.clone(),
                activated: false,
            });
        }
        if delta.base_generation != self.last_generation
            || delta.delta_generation != delta.base_generation.saturating_add(1)
        {
            return Err(CitadelMaterializationError::Delta(
                "delta generation does not match the live Citadel generation".into(),
            ));
        }
        if self.active_delta_targets.contains_key(&delta.target_tensor) {
            return Err(CitadelMaterializationError::Delta(
                "conflicting delta target is already active".into(),
            ));
        }
        let base = self.tensor_store.get(&delta.target_tensor)?.clone();
        if base.generation != delta.base_generation
            || base.materialization_sha256 != delta.parent_materialization_hash
        {
            return Err(CitadelMaterializationError::Delta(
                "delta parent does not match the target tensor".into(),
            ));
        }
        let encoded = delta
            .values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect::<Vec<_>>();
        if hash_bytes(&encoded) != delta.integrity_hash {
            return Err(CitadelMaterializationError::Delta(
                "delta integrity hash mismatch".into(),
            ));
        }
        let mut values = base
            .bytes
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect::<Vec<_>>();
        if values.len() != delta.values.len() {
            return Err(CitadelMaterializationError::Delta(
                "delta scope does not match tensor shape".into(),
            ));
        }
        for (value, change) in values.iter_mut().zip(&delta.values) {
            *value += change;
        }
        let bytes = values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect::<Vec<_>>();
        let next_hash = hash_bytes(&bytes);
        let next = NsqTensor {
            schema: NSQ_TENSOR_SCHEMA.into(),
            name: base.name,
            dtype: base.dtype,
            shape: base.shape,
            source_path: format!("delta://{}", delta.identity),
            source_sha256: delta.integrity_hash.clone(),
            materialization_sha256: next_hash.clone(),
            generation: delta.delta_generation,
            bytes,
        };
        self.tensor_store.insert(next)?;
        self.last_generation = delta.delta_generation;
        self.applied_deltas.insert(delta.identity.clone());
        self.active_delta_targets
            .insert(delta.target_tensor.clone(), delta.identity.clone());
        Ok(CitadelDeltaReceipt {
            identity: delta.identity.clone(),
            target_tensor: delta.target_tensor.clone(),
            generation: delta.delta_generation,
            materialization_hash: next_hash,
            activated: true,
        })
    }

    pub fn materialize_manifest(
        &mut self,
        manifest_json: &str,
    ) -> Result<(CitadelInventory, CitadelMaterialization), CitadelMaterializationError> {
        let manifest: CitadelManifest = serde_json::from_str(manifest_json)
            .map_err(|error| CitadelMaterializationError::Manifest(error.to_string()))?;
        validate_manifest(&manifest)?;
        let canonical = manifest
            .lanes
            .iter()
            .map(|lane| {
                format!(
                    "{}|{}|{}|{}",
                    lane.lane, lane.model_id, lane.revision, lane.semantic_projection
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let manifest_hash = hash_bytes(canonical.as_bytes());
        let intent = manifest
            .lanes
            .iter()
            .map(|lane| format!("{} {}", lane.lane, lane.model_id))
            .collect::<Vec<_>>()
            .join(" ");
        let seed = IntentSeed::new(&format!("citadel-manifest-{manifest_hash}"), &intent);
        let materialization = self.materialize_seed(&seed, 1)?;
        let mut by_pole = BTreeMap::new();
        for body in &materialization.bodies {
            if by_pole.insert(body.pole_id.clone(), body).is_some() {
                return Err(CitadelMaterializationError::Reconciliation(
                    "duplicate materialized pole".into(),
                ));
            }
        }
        let mut entries = Vec::with_capacity(manifest.lanes.len());
        for lane in &manifest.lanes {
            let pole_id = lane.lane.split('_').next().unwrap_or(&lane.lane);
            let body = by_pole.get(pole_id).ok_or_else(|| {
                CitadelMaterializationError::Reconciliation(format!(
                    "missing body for lane {}",
                    lane.lane
                ))
            })?;
            entries.push(CitadelInventoryEntry {
                lane: lane.lane.clone(),
                pole_id: body.pole_id.clone(),
                tensor_name: body.tensor_name.clone(),
                address: address_record(&body.address),
                owner: address_record(&body.owner),
                source_seed_hash: body.source_seed_hash.clone(),
                materialization_hash: body.materialization_hash.clone(),
                generation: body.generation,
            });
        }
        let inventory_hash = hash_bytes(
            serde_json::to_string(&entries)
                .map_err(|error| CitadelMaterializationError::Reconciliation(error.to_string()))?
                .as_bytes(),
        );
        Ok((
            CitadelInventory {
                schema: CITADEL_INVENTORY_SCHEMA.into(),
                manifest_hash,
                generation: materialization.generation,
                entries,
                inventory_hash,
            },
            materialization,
        ))
    }

    pub fn reconcile_inventory(
        &self,
        previous: &CitadelInventory,
        next: &CitadelInventory,
    ) -> Result<(), CitadelMaterializationError> {
        if previous.schema != CITADEL_INVENTORY_SCHEMA || next.schema != CITADEL_INVENTORY_SCHEMA {
            return Err(CitadelMaterializationError::Reconciliation(
                "invalid inventory schema".into(),
            ));
        }
        if next.entries.len() != 10 || unique_lanes(&next.entries) != 10 {
            return Err(CitadelMaterializationError::Reconciliation(
                "inventory is incomplete or duplicated".into(),
            ));
        }
        if previous.manifest_hash != next.manifest_hash {
            return Err(CitadelMaterializationError::Reconciliation(
                "manifest identity changed".into(),
            ));
        }
        if next.generation < previous.generation {
            return Err(CitadelMaterializationError::Reconciliation(
                "generation moved backwards".into(),
            ));
        }
        Ok(())
    }

    pub fn materialize_seed(
        &mut self,
        seed: &IntentSeed,
        generation: u64,
    ) -> Result<CitadelMaterialization, CitadelMaterializationError> {
        if seed.identity.trim().is_empty() || seed.intent.trim().is_empty() {
            return Err(CitadelMaterializationError::InvalidSeed(
                "identity and intent are required".into(),
            ));
        }
        if seed.coordinates.is_empty() {
            return Err(CitadelMaterializationError::InvalidSeed(
                "seed must contain at least one coordinate".into(),
            ));
        }
        let seed_hash = hash_bytes(
            format!("{}\n{}\n{:?}", seed.identity, seed.intent, seed.coordinates).as_bytes(),
        );
        let mut bodies = Vec::with_capacity(10);
        let mut outputs = BTreeMap::new();
        let mut instructions = Vec::with_capacity(20);
        let mut caps = build_capitals(self.coaching);
        let source_slots = seed
            .coordinates
            .iter()
            .map(|position| {
                NSQSlot::new(
                    Dialect::Intent,
                    vec![NSQLever::new(
                        Charge::Positive,
                        (*position).clamp(1, CANONICAL_LEVER_MAX_POSITION),
                    )
                    .unwrap()],
                )
            })
            .collect::<Vec<_>>();
        for capital in &mut caps {
            let bits = capital.dispatch(source_slots.clone());
            let board = capital.drain_board();
            for (bit, message) in bits.into_iter().zip(board.into_iter()) {
                let values = seed
                    .coordinates
                    .iter()
                    .map(|coordinate| {
                        (*coordinate as f32 / CANONICAL_LEVER_MAX_POSITION as f32)
                            + (message.pole_lane as f32 / 100.0)
                    })
                    .collect::<Vec<_>>();
                let bytes = values
                    .iter()
                    .flat_map(|value| value.to_le_bytes())
                    .collect::<Vec<_>>();
                let tensor_name = format!("citadel.{}.{}", message.pole_id, seed.identity);
                let address = address_for(message.pole_lane, false);
                let owner = address_for(message.pole_lane, true);
                let tensor = NsqTensor {
                    schema: NSQ_TENSOR_SCHEMA.into(),
                    name: tensor_name.clone(),
                    dtype: "F32".into(),
                    shape: vec![values.len() as u64],
                    source_path: format!("seed://{}", seed.identity),
                    source_sha256: seed_hash.clone(),
                    materialization_sha256: hash_bytes(&bytes),
                    generation,
                    bytes,
                };
                self.tensor_store.insert(tensor.clone())?;
                let output = self
                    .tensor_store
                    .parameter_dot(&tensor_name, &vec![1.0; values.len()])?;
                outputs.insert(message.pole_id.clone(), output);
                // A rematerialization may occur in a fresh on-demand runtime after
                // the prior resident window was released. Only release a lease that
                // is actually owned in this runtime; generation remains monotonic in
                // the materialized record either way.
                if generation > 1
                    && self
                        .ownership
                        .leases()
                        .values()
                        .any(|lease| lease.owner == owner)
                {
                    self.ownership
                        .advance(&owner, NsqLeasePhase::Release)
                        .map_err(CitadelMaterializationError::Ownership)?;
                }
                self.ownership
                    .acquire(owner.clone(), std::slice::from_ref(&address))
                    .map_err(CitadelMaterializationError::Ownership)?;
                instructions.push(NsqInstruction::Set {
                    address: address.clone(),
                    value: value_to_slot(output),
                });
                instructions.push(NsqInstruction::Fire {
                    address: address.clone(),
                });
                bodies.push(CitadelTensorBody {
                    pole_id: message.pole_id,
                    capital_id: message.capital_id,
                    lane: bit.lane as usize,
                    source_seed_id: seed.identity.clone(),
                    source_seed_hash: seed_hash.clone(),
                    tensor_name,
                    shape: vec![values.len() as u64],
                    dtype: "F32".into(),
                    address,
                    owner,
                    generation,
                    materialization_hash: tensor.materialization_sha256,
                });
            }
        }
        let receipt = self
            .runtime
            .execute(&instructions)
            .map_err(CitadelMaterializationError::Runtime)?;
        self.last_generation = generation;
        Ok(CitadelMaterialization {
            schema: CITADEL_MATERIALIZATION_SCHEMA.into(),
            seed_id: seed.identity.clone(),
            seed_hash,
            generation,
            bodies,
            parameter_outputs: outputs,
            receipt,
        })
    }
}

pub fn delta_for_tensor(
    target_tensor: &NsqTensor,
    identity: &str,
    values: Vec<f32>,
) -> Result<CitadelDelta, CitadelMaterializationError> {
    if values.len() * 4 != target_tensor.bytes.len() {
        return Err(CitadelMaterializationError::Delta(
            "delta scope does not match tensor shape".into(),
        ));
    }
    let encoded = values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect::<Vec<_>>();
    Ok(CitadelDelta {
        schema: CITADEL_DELTA_SCHEMA.into(),
        identity: identity.into(),
        target_tensor: target_tensor.name.clone(),
        base_generation: target_tensor.generation,
        delta_generation: target_tensor.generation.saturating_add(1),
        parent_materialization_hash: target_tensor.materialization_sha256.clone(),
        values,
        integrity_hash: hash_bytes(&encoded),
    })
}

fn validate_manifest(manifest: &CitadelManifest) -> Result<(), CitadelMaterializationError> {
    if manifest.lanes.len() != 10 {
        return Err(CitadelMaterializationError::Manifest(
            "Council manifest must contain exactly ten lanes".into(),
        ));
    }
    let mut lanes = BTreeSet::new();
    for lane in &manifest.lanes {
        if lane.lane.trim().is_empty()
            || lane.model_id.trim().is_empty()
            || lane.revision.trim().is_empty()
        {
            return Err(CitadelMaterializationError::Manifest(
                "lane identity is incomplete".into(),
            ));
        }
        if !lanes.insert(lane.lane.clone()) {
            return Err(CitadelMaterializationError::Manifest(format!(
                "duplicate lane {}",
                lane.lane
            )));
        }
    }
    Ok(())
}

fn unique_lanes(entries: &[CitadelInventoryEntry]) -> usize {
    entries
        .iter()
        .map(|entry| entry.lane.as_str())
        .collect::<BTreeSet<_>>()
        .len()
}

fn address_record(address: &NsqAddress) -> NsqAddressRecord {
    NsqAddressRecord {
        slots: address
            .path
            .iter()
            .map(|slot| format!("{:?}", slot))
            .collect(),
    }
}

fn address_for(lane: usize, owner: bool) -> NsqAddress {
    let base = if owner { 50_000 } else { 25_000 };
    let position = (base + lane as u64).min(CANONICAL_LEVER_MAX_POSITION);
    NsqAddress::root(NSQSlot::new(
        Dialect::Control,
        vec![NSQLever::new(Charge::Positive, position).unwrap()],
    ))
}

fn value_to_slot(value: f32) -> NSQSlot {
    let position = value.abs().min(CANONICAL_LEVER_MAX_POSITION as f32).round() as u64;
    NSQSlot::new(
        Dialect::Numeric,
        vec![NSQLever::new(
            if value.is_sign_negative() {
                Charge::Negative
            } else {
                Charge::Positive
            },
            position.max(1),
        )
        .unwrap()],
    )
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(intent: &str) -> IntentSeed {
        IntentSeed::new("citadel-seed-v1", intent)
    }

    #[test]
    fn seed_materializes_all_ten_bodies_into_tensor_store_and_runtime() {
        let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let result = runtime
            .materialize_seed(&seed("willow stone remembers"), 1)
            .unwrap();
        assert_eq!(result.bodies.len(), 10);
        assert_eq!(result.parameter_outputs.len(), 10);
        assert_eq!(result.receipt.executed, 20);
        assert_eq!(result.receipt.fired, 10);
        assert_eq!(runtime.tensor_store.len(), 10);
    }

    #[test]
    fn seed_change_rematerializes_generation_and_changes_outputs() {
        let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let first = runtime
            .materialize_seed(&seed("willow stone remembers"), 1)
            .unwrap();
        let second = runtime
            .materialize_seed(&seed("stone willow changes"), 2)
            .unwrap();
        assert_ne!(first.seed_hash, second.seed_hash);
        assert_ne!(first.parameter_outputs, second.parameter_outputs);
        assert!(second.bodies.iter().all(|body| body.generation == 2));
        assert_eq!(runtime.last_generation, 2);
    }

    #[test]
    fn fresh_runtime_can_rematerialize_a_later_generation_without_prior_residency() {
        let seed = seed("on-demand reconstruction");
        let mut first_window = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let first = first_window.materialize_seed(&seed, 1).unwrap();
        assert_eq!(first.generation, 1);

        let mut reconstructed_window = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let second = reconstructed_window.materialize_seed(&seed, 2).unwrap();
        assert_eq!(second.generation, 2);
        assert_eq!(second.bodies.len(), 10);
        assert_eq!(second.receipt.fired, 10);
    }

    #[test]
    fn manifest_materializes_complete_deterministic_inventory() {
        let manifest = include_str!("../../../config/nsq/council_full_artifact_seed_manifest.json");
        let mut first_runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let mut second_runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let (first_inventory, first_materialization) =
            first_runtime.materialize_manifest(manifest).unwrap();
        let (second_inventory, second_materialization) =
            second_runtime.materialize_manifest(manifest).unwrap();
        assert_eq!(first_inventory, second_inventory);
        assert_eq!(first_materialization, second_materialization);
        assert_eq!(first_inventory.entries.len(), 10);
        assert_eq!(unique_lanes(&first_inventory.entries), 10);
        assert_eq!(first_inventory.generation, 1);
        assert!(first_inventory
            .entries
            .iter()
            .all(|entry| !entry.materialization_hash.is_empty()));
    }

    #[test]
    fn manifest_failures_close_without_partial_inventory() {
        let manifest = include_str!("../../../config/nsq/council_full_artifact_seed_manifest.json");
        let mut value: serde_json::Value = serde_json::from_str(manifest).unwrap();
        value["lanes"][0]["lane"] = value["lanes"][1]["lane"].clone();
        let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        assert!(matches!(
            runtime.materialize_manifest(&value.to_string()),
            Err(CitadelMaterializationError::Manifest(_))
        ));
        assert_eq!(runtime.tensor_store.len(), 0);
    }

    #[test]
    fn inventory_reconciliation_rejects_stale_and_incomplete_state() {
        let manifest = include_str!("../../../config/nsq/council_full_artifact_seed_manifest.json");
        let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let (inventory, _) = runtime.materialize_manifest(manifest).unwrap();
        let mut incomplete = inventory.clone();
        incomplete.entries.pop();
        assert!(matches!(
            runtime.reconcile_inventory(&inventory, &incomplete),
            Err(CitadelMaterializationError::Reconciliation(_))
        ));
        let mut stale = inventory.clone();
        stale.generation = 0;
        assert!(matches!(
            runtime.reconcile_inventory(&inventory, &stale),
            Err(CitadelMaterializationError::Reconciliation(_))
        ));
    }

    #[test]
    fn delta_contract_applies_once_and_changes_tensor_computation() {
        let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let seed_result = runtime.materialize_seed(&seed("delta base"), 1).unwrap();
        let target = seed_result.bodies[0].tensor_name.clone();
        let before = runtime
            .tensor_store
            .parameter_dot(&target, &vec![1.0; 2])
            .unwrap();
        let base = runtime.tensor_store.get(&target).unwrap().clone();
        let delta = delta_for_tensor(&base, "delta-1", vec![0.5; 2]).unwrap();
        let receipt = runtime.apply_delta(&delta).unwrap();
        let after = runtime
            .tensor_store
            .parameter_dot(&target, &vec![1.0; 2])
            .unwrap();
        assert!(receipt.activated);
        assert_eq!(receipt.generation, 2);
        assert_ne!(before, after);
        let replay = runtime.apply_delta(&delta).unwrap();
        assert!(!replay.activated);
        assert_eq!(replay.materialization_hash, receipt.materialization_hash);
    }

    #[test]
    fn delta_contract_rejects_stale_conflicting_and_corrupted_state() {
        let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let seed_result = runtime.materialize_seed(&seed("delta guards"), 1).unwrap();
        let target = seed_result.bodies[0].tensor_name.clone();
        let base = runtime.tensor_store.get(&target).unwrap().clone();
        let delta = delta_for_tensor(&base, "delta-guard", vec![0.25; 2]).unwrap();
        let mut corrupt = delta.clone();
        corrupt.integrity_hash = "corrupt".into();
        assert!(matches!(
            runtime.apply_delta(&corrupt),
            Err(CitadelMaterializationError::Delta(_))
        ));
        let mut stale = delta.clone();
        stale.base_generation = 0;
        stale.delta_generation = 1;
        assert!(matches!(
            runtime.apply_delta(&stale),
            Err(CitadelMaterializationError::Delta(_))
        ));
        runtime.apply_delta(&delta).unwrap();
        let next_base = runtime.tensor_store.get(&target).unwrap().clone();
        let conflict = delta_for_tensor(&next_base, "delta-conflict", vec![0.5; 2]).unwrap();
        assert!(matches!(
            runtime.apply_delta(&conflict),
            Err(CitadelMaterializationError::Delta(_))
        ));
    }

    #[test]
    fn seed_materialization_fails_closed_without_coordinates() {
        let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let mut seed = seed("intent");
        seed.coordinates.clear();
        assert!(matches!(
            runtime.materialize_seed(&seed, 1),
            Err(CitadelMaterializationError::InvalidSeed(_))
        ));
    }
}
