use crate::{capital::build_capitals, coaching::CoachingMode, seed::IntentSeed};
use nsq_core::{
    Charge, Dialect, NSQLever, NSQSlot, NativeNsqMachine, NativeNsqOwnership, NativeNsqRuntime,
    NsqActuationReceipt, NsqAddress, NsqInstruction, NsqLeasePhase, NsqTensor, NsqTensorStore,
    CANONICAL_LEVER_MAX_POSITION, NSQ_TENSOR_SCHEMA,
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const CITADEL_MATERIALIZATION_SCHEMA: &str = "nsq.citadel.seed_materialization.v1";

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
}

impl std::fmt::Display for CitadelMaterializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSeed(message) => write!(f, "invalid Citadel seed: {message}"),
            Self::Tensor(error) => write!(f, "Citadel tensor error: {error}"),
            Self::Runtime(error) => write!(f, "Citadel runtime error: {error}"),
            Self::Ownership(error) => write!(f, "Citadel ownership error: {error}"),
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
}

impl CitadelNativeRuntime {
    pub fn new(coaching: CoachingMode) -> Self {
        Self {
            coaching,
            tensor_store: NsqTensorStore::default(),
            runtime: NativeNsqRuntime::new(NativeNsqMachine::default()),
            ownership: NativeNsqOwnership::default(),
            last_generation: 0,
        }
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
                if generation > 1 {
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
