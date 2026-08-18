use crate::{
    Charge, Dialect, NSQLever, NSQSlot, NsqAddress, NsqInstruction, NsqLeasePhase,
    CANONICAL_LEVER_MAX_POSITION,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const CONFINED_EXECUTION_SCHEMA: &str = "nsq.confined_execution.v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfinedOperation {
    Add,
    Multiply,
    Affine { scale: f64, bias: f64 },
    Clamp { min: f64, max: f64 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitiativeExpression {
    pub id: String,
    pub inputs: Vec<String>,
    pub output: String,
    pub operation: ConfinedOperation,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfinedBusBinding {
    pub semantic_name: String,
    pub address: NsqAddress,
    pub owner: NsqAddress,
    pub generation: u64,
    pub phase: NsqLeasePhase,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfinedExecution {
    pub schema: String,
    pub values: BTreeMap<String, f64>,
    pub bindings: Vec<ConfinedBusBinding>,
    pub instructions: Vec<NsqInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfinedExecutionError {
    EmptyExpression(String),
    DuplicateOutput(String),
    MissingInput(String),
    InvalidNumber(String),
    InvalidClamp(String),
    InvalidAddress(String),
}

impl std::fmt::Display for ConfinedExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyExpression(id) => write!(f, "expression {id} has no inputs"),
            Self::DuplicateOutput(name) => write!(f, "duplicate expression output: {name}"),
            Self::MissingInput(name) => write!(f, "missing confined input: {name}"),
            Self::InvalidNumber(name) => write!(f, "non-finite confined value: {name}"),
            Self::InvalidClamp(id) => write!(f, "invalid clamp bounds in expression: {id}"),
            Self::InvalidAddress(name) => write!(f, "unable to allocate NSQ address: {name}"),
        }
    }
}

impl std::error::Error for ConfinedExecutionError {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfinedProgram {
    pub expressions: Vec<InitiativeExpression>,
}

impl ConfinedProgram {
    pub fn validate(&self) -> Result<(), ConfinedExecutionError> {
        let mut outputs = BTreeSet::new();
        for expression in &self.expressions {
            if expression.id.trim().is_empty() || expression.output.trim().is_empty() {
                return Err(ConfinedExecutionError::EmptyExpression(
                    expression.id.clone(),
                ));
            }
            if expression.inputs.is_empty() {
                return Err(ConfinedExecutionError::EmptyExpression(
                    expression.id.clone(),
                ));
            }
            if !outputs.insert(expression.output.clone()) {
                return Err(ConfinedExecutionError::DuplicateOutput(
                    expression.output.clone(),
                ));
            }
            if let ConfinedOperation::Clamp { min, max } = expression.operation {
                if !min.is_finite() || !max.is_finite() || min > max {
                    return Err(ConfinedExecutionError::InvalidClamp(expression.id.clone()));
                }
            }
        }
        Ok(())
    }

    pub fn execute(
        &self,
        mut inputs: BTreeMap<String, f64>,
    ) -> Result<ConfinedExecution, ConfinedExecutionError> {
        self.validate()?;
        for (name, value) in &inputs {
            ensure_finite(name, *value)?;
        }
        let mut bindings = Vec::new();
        let mut instructions = Vec::new();
        for expression in &self.expressions {
            let mut values = Vec::with_capacity(expression.inputs.len());
            for input in &expression.inputs {
                let value = inputs
                    .get(input)
                    .copied()
                    .ok_or_else(|| ConfinedExecutionError::MissingInput(input.clone()))?;
                values.push(value);
            }
            let result = evaluate(&expression.operation, &values)?;
            ensure_finite(&expression.output, result)?;
            inputs.insert(expression.output.clone(), result);
            let owner = address_for(&format!("owner:{}", expression.id))?;
            let address = address_for(&expression.output)?;
            bindings.push(ConfinedBusBinding {
                semantic_name: expression.output.clone(),
                address: address.clone(),
                owner,
                generation: expression.generation,
                phase: NsqLeasePhase::Acquire,
            });
            instructions.push(NsqInstruction::Set {
                address: address.clone(),
                value: value_to_slot(result),
            });
            instructions.push(NsqInstruction::Fire { address });
        }
        Ok(ConfinedExecution {
            schema: CONFINED_EXECUTION_SCHEMA.into(),
            values: inputs,
            bindings,
            instructions,
        })
    }
}

fn evaluate(operation: &ConfinedOperation, values: &[f64]) -> Result<f64, ConfinedExecutionError> {
    let result = match operation {
        ConfinedOperation::Add => values.iter().sum(),
        ConfinedOperation::Multiply => values.iter().product(),
        ConfinedOperation::Affine { scale, bias } => {
            ensure_finite("scale", *scale)?;
            ensure_finite("bias", *bias)?;
            values.first().copied().unwrap_or(0.0) * scale + bias
        }
        ConfinedOperation::Clamp { min, max } => {
            values.first().copied().unwrap_or(0.0).clamp(*min, *max)
        }
    };
    Ok(result)
}

fn ensure_finite(name: &str, value: f64) -> Result<(), ConfinedExecutionError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(ConfinedExecutionError::InvalidNumber(name.into()))
    }
}

fn address_for(name: &str) -> Result<NsqAddress, ConfinedExecutionError> {
    let hash = name.bytes().fold(2_166_136_261u32, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(16_777_619)
    });
    let position = u64::from(hash) % CANONICAL_LEVER_MAX_POSITION + 1;
    let lever = NSQLever::new(Charge::Positive, position)
        .map_err(|_| ConfinedExecutionError::InvalidAddress(name.into()))?;
    Ok(NsqAddress::root(NSQSlot::new(Dialect::Intent, vec![lever])))
}

fn value_to_slot(value: f64) -> NSQSlot {
    let magnitude = value.abs().min(CANONICAL_LEVER_MAX_POSITION as f64).round() as u64;
    let position = magnitude.max(1);
    NSQSlot::new(
        Dialect::Numeric,
        vec![NSQLever::new(
            if value.is_sign_negative() {
                Charge::Negative
            } else {
                Charge::Positive
            },
            position,
        )
        .expect("clamped confined value always produces a canonical lever")],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_runtime::NsqActuator;
    use crate::{NativeNsqMachine, NativeNsqOwnership, NativeNsqRuntime};

    fn program() -> ConfinedProgram {
        ConfinedProgram {
            expressions: vec![
                InitiativeExpression {
                    id: "sum".into(),
                    inputs: vec!["x".into(), "y".into()],
                    output: "sum".into(),
                    operation: ConfinedOperation::Add,
                    generation: 1,
                },
                InitiativeExpression {
                    id: "scale".into(),
                    inputs: vec!["sum".into()],
                    output: "score".into(),
                    operation: ConfinedOperation::Affine {
                        scale: 2.0,
                        bias: 1.0,
                    },
                    generation: 1,
                },
            ],
        }
    }

    #[test]
    fn confined_expression_executes_and_emits_callable_bus_stream() {
        let mut inputs = BTreeMap::new();
        inputs.insert("x".into(), 3.0);
        inputs.insert("y".into(), 4.0);
        let execution = program().execute(inputs).unwrap();
        assert_eq!(execution.values["score"], 15.0);
        assert_eq!(execution.instructions.len(), 4);
        let mut runtime = NativeNsqRuntime::new(NativeNsqMachine::default());
        runtime.execute(&execution.instructions).unwrap();
        assert_eq!(runtime.actuator().snapshot().len(), 2);
    }

    #[test]
    fn confined_bus_bindings_are_leased_without_same_space_override() {
        let mut inputs = BTreeMap::new();
        inputs.insert("x".into(), 1.0);
        inputs.insert("y".into(), 2.0);
        let execution = program().execute(inputs).unwrap();
        let mut ownership = NativeNsqOwnership::default();
        for binding in &execution.bindings {
            ownership
                .acquire(
                    binding.owner.clone(),
                    std::slice::from_ref(&binding.address),
                )
                .unwrap();
        }
        assert_eq!(execution.bindings.len(), 2);
    }

    #[test]
    fn confined_rematerialization_changes_result_and_generation() {
        let mut inputs = BTreeMap::new();
        inputs.insert("x".into(), 2.0);
        inputs.insert("y".into(), 3.0);
        let first = program().execute(inputs.clone()).unwrap();
        inputs.insert("x".into(), 5.0);
        let mut changed = program();
        changed.expressions[0].generation = 2;
        let second = changed.execute(inputs).unwrap();
        assert_ne!(first.values["score"], second.values["score"]);
        assert_eq!(second.bindings[0].generation, 2);
    }

    #[test]
    fn confined_program_fails_closed_on_missing_input_and_duplicate_output() {
        let mut inputs = BTreeMap::new();
        inputs.insert("x".into(), 1.0);
        assert!(matches!(
            program().execute(inputs),
            Err(ConfinedExecutionError::MissingInput(_))
        ));
        let mut invalid = program();
        invalid.expressions[1].output = "sum".into();
        assert!(matches!(
            invalid.execute(BTreeMap::new()),
            Err(ConfinedExecutionError::DuplicateOutput(_))
        ));
    }
}

#[allow(dead_code)]
fn _lease_phase_is_serializable(phase: NsqLeasePhase) -> NsqLeasePhase {
    phase
}
