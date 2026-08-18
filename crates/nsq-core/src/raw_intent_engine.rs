use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Raw NSQ is the authoritative internal representation for reconstructed behavior.
/// Text, source-language ASTs, and external tool APIs are boundary evidence only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawNsqIntent {
    pub intent_id: String,
    pub domain: String,
    pub operation: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub invariants: Vec<String>,
    pub correction_route: String,
}

impl RawNsqIntent {
    pub fn new(
        intent_id: impl Into<String>,
        domain: impl Into<String>,
        operation: impl Into<String>,
        inputs: Vec<String>,
        outputs: Vec<String>,
        invariants: Vec<String>,
        correction_route: impl Into<String>,
    ) -> Result<Self, String> {
        let value = Self {
            intent_id: intent_id.into(),
            domain: domain.into(),
            operation: operation.into(),
            inputs,
            outputs,
            invariants,
            correction_route: correction_route.into(),
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), String> {
        for (label, value) in [
            ("intent_id", &self.intent_id),
            ("domain", &self.domain),
            ("operation", &self.operation),
            ("correction_route", &self.correction_route),
        ] {
            if value.trim().is_empty() {
                return Err(format!("raw NSQ intent {label} cannot be empty"));
            }
        }
        if self.inputs.is_empty() {
            return Err("raw NSQ intent requires at least one input".into());
        }
        if self.outputs.is_empty() {
            return Err("raw NSQ intent requires at least one output".into());
        }
        if self.invariants.is_empty() {
            return Err("raw NSQ intent requires at least one invariant".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawNsqCapability {
    pub capability_id: String,
    pub intent: RawNsqIntent,
    pub surface: String,
    pub native_entry: String,
    pub external_reference: Option<String>,
}

impl RawNsqCapability {
    pub fn validate(&self) -> Result<(), String> {
        if self.capability_id.trim().is_empty() || self.surface.trim().is_empty() {
            return Err("raw NSQ capability identity cannot be empty".into());
        }
        if self.native_entry.trim().is_empty() {
            return Err("raw NSQ capability requires a native entry".into());
        }
        if self
            .external_reference
            .as_deref()
            .is_some_and(|reference| reference.trim().is_empty())
        {
            return Err("empty external references must be omitted".into());
        }
        self.intent.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RawNsqEvent {
    Invoke {
        capability_id: String,
        input: BTreeMap<String, String>,
    },
    Correct {
        capability_id: String,
        field: String,
        expected: String,
        observed: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RawNsqOutcome {
    Accepted {
        capability_id: String,
        state: BTreeMap<String, String>,
    },
    Corrected {
        capability_id: String,
        state: BTreeMap<String, String>,
    },
    Rejected {
        reason: String,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RawNsqEngine {
    capabilities: BTreeMap<String, RawNsqCapability>,
    state: BTreeMap<String, String>,
    sequence: u64,
}

impl RawNsqEngine {
    pub fn register(&mut self, capability: RawNsqCapability) -> Result<(), String> {
        capability.validate()?;
        if self.capabilities.contains_key(&capability.capability_id) {
            return Err(format!(
                "duplicate raw NSQ capability: {}",
                capability.capability_id
            ));
        }
        self.capabilities
            .insert(capability.capability_id.clone(), capability);
        Ok(())
    }

    pub fn discover(&self, query: &str) -> Vec<&RawNsqCapability> {
        let needle = query.trim().to_ascii_lowercase();
        self.capabilities
            .values()
            .filter(|capability| {
                needle.is_empty()
                    || capability
                        .capability_id
                        .to_ascii_lowercase()
                        .contains(&needle)
                    || capability.surface.to_ascii_lowercase().contains(&needle)
                    || capability
                        .intent
                        .domain
                        .to_ascii_lowercase()
                        .contains(&needle)
                    || capability
                        .intent
                        .operation
                        .to_ascii_lowercase()
                        .contains(&needle)
                    || capability
                        .intent
                        .invariants
                        .iter()
                        .any(|item| item.to_ascii_lowercase().contains(&needle))
            })
            .collect()
    }

    pub fn dispatch(&mut self, event: RawNsqEvent) -> RawNsqOutcome {
        match event {
            RawNsqEvent::Invoke {
                capability_id,
                input,
            } => {
                let Some(capability) = self.capabilities.get(&capability_id) else {
                    return RawNsqOutcome::Rejected {
                        reason: format!("unknown raw NSQ capability: {capability_id}"),
                    };
                };
                if let Err(reason) = validate_input(capability, &input) {
                    return RawNsqOutcome::Rejected { reason };
                }
                self.sequence = self.sequence.saturating_add(1);
                self.state
                    .insert("last_capability".into(), capability_id.clone());
                self.state
                    .insert("last_sequence".into(), self.sequence.to_string());
                for (key, value) in input {
                    self.state.insert(format!("input:{key}"), value);
                }
                RawNsqOutcome::Accepted {
                    capability_id,
                    state: self.state.clone(),
                }
            }
            RawNsqEvent::Correct {
                capability_id,
                field,
                expected,
                observed,
            } => {
                if !self.capabilities.contains_key(&capability_id) {
                    return RawNsqOutcome::Rejected {
                        reason: format!("unknown raw NSQ capability: {capability_id}"),
                    };
                }
                if expected == observed {
                    return RawNsqOutcome::Rejected {
                        reason: "correction event has no divergence".into(),
                    };
                }
                self.state.insert(format!("corrected:{field}"), expected);
                self.state
                    .insert("correction_capability".into(), capability_id.clone());
                self.sequence = self.sequence.saturating_add(1);
                self.state
                    .insert("last_sequence".into(), self.sequence.to_string());
                RawNsqOutcome::Corrected {
                    capability_id,
                    state: self.state.clone(),
                }
            }
        }
    }

    pub fn state(&self) -> &BTreeMap<String, String> {
        &self.state
    }

    pub fn capability_count(&self) -> usize {
        self.capabilities.len()
    }
}

/// Register the reconstructed tool intents. These are behavioral contracts, not ports
/// of Tree-sitter, BOLT, JIT, Zig, DWARF, Hammer, ORC, Archer, Guile, or apropos.
pub fn register_reconstructed_tool_intents(engine: &mut RawNsqEngine) -> Result<(), String> {
    let catalog = [
        ("tree_sitter.parse", "syntax", "parse", "tree-sitter"),
        ("bolt.optimize", "layout", "optimize", "BOLT"),
        ("jit.dispatch", "execution", "specialize", "JIT"),
        ("zig.build", "compile", "resolve", "Zig"),
        ("dwarf.provenance", "debug", "explain", "DWARF"),
        ("hammer.verify", "analysis", "verify", "Hammer"),
        ("orc.compile", "compile", "materialize", "ORC"),
        ("archer.inspect", "analysis", "inspect", "Archer"),
        ("guile.rebuild_intent", "language", "reconstruct", "Guile"),
        ("apropos.discover", "discovery", "discover", "apropos"),
        ("tokenizer.boundary", "boundary", "encode", "tokenizer"),
        ("correction.in_stream", "correction", "repair", "correction"),
    ];
    for (capability_id, domain, operation, reference) in catalog {
        let intent = RawNsqIntent::new(
            format!("intent.{capability_id}"),
            domain,
            operation,
            vec!["input".into()],
            vec!["state".into(), "result".into()],
            vec![
                "raw NSQ is authoritative".into(),
                "deterministic ordering".into(),
                "in-stream correction remains reachable".into(),
            ],
            "correction.in_stream",
        )?;
        engine.register(RawNsqCapability {
            capability_id: capability_id.into(),
            intent,
            surface: reference.into(),
            native_entry: format!("nsq-core::RawNsqEngine::{operation}"),
            external_reference: Some(format!("{reference} intent evidence only")),
        })?;
    }
    Ok(())
}

fn validate_input(
    capability: &RawNsqCapability,
    input: &BTreeMap<String, String>,
) -> Result<(), String> {
    for required in &capability.intent.inputs {
        if !input.contains_key(required) {
            return Err(format!(
                "missing raw NSQ input `{required}` for {}",
                capability.capability_id
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capability() -> RawNsqCapability {
        RawNsqCapability {
            capability_id: "tooling.apropos.discover".into(),
            intent: RawNsqIntent::new(
                "intent.apropos.discover",
                "discovery",
                "discover",
                vec!["query".into()],
                vec!["capabilities".into()],
                vec!["deterministic ordering".into(), "NSQ authority".into()],
                "correct.discovery.query",
            )
            .unwrap(),
            surface: "apropos".into(),
            native_entry: "nsq-core::RawNsqEngine::discover".into(),
            external_reference: Some("Guile apropos intent only".into()),
        }
    }

    #[test]
    fn discovery_is_native_and_deterministic() {
        let mut engine = RawNsqEngine::default();
        engine.register(capability()).unwrap();
        let ids: Vec<_> = engine
            .discover("apropos")
            .into_iter()
            .map(|item| item.capability_id.as_str())
            .collect();
        assert_eq!(ids, vec!["tooling.apropos.discover"]);
    }

    #[test]
    fn dispatch_and_correction_remain_in_one_state_machine() {
        let mut engine = RawNsqEngine::default();
        engine.register(capability()).unwrap();
        let mut input = BTreeMap::new();
        input.insert("query".into(), "reflexor".into());
        assert!(matches!(
            engine.dispatch(RawNsqEvent::Invoke {
                capability_id: "tooling.apropos.discover".into(),
                input,
            }),
            RawNsqOutcome::Accepted { .. }
        ));
        assert!(matches!(
            engine.dispatch(RawNsqEvent::Correct {
                capability_id: "tooling.apropos.discover".into(),
                field: "query".into(),
                expected: "reflexor".into(),
                observed: "reflexor-old".into(),
            }),
            RawNsqOutcome::Corrected { .. }
        ));
        assert_eq!(
            engine.state().get("corrected:query"),
            Some(&"reflexor".to_string())
        );
    }

    #[test]
    fn reconstructed_tool_catalog_is_native_and_complete() {
        let mut engine = RawNsqEngine::default();
        register_reconstructed_tool_intents(&mut engine).unwrap();
        assert_eq!(engine.capability_count(), 12);
        assert_eq!(
            engine.discover("dwarf")[0].native_entry,
            "nsq-core::RawNsqEngine::explain"
        );
        assert_eq!(engine.discover("tree-sitter")[0].intent.operation, "parse");
    }

    #[test]
    fn abstraction_only_capabilities_are_rejected() {
        let mut engine = RawNsqEngine::default();
        let mut value = capability();
        value.native_entry.clear();
        assert!(engine.register(value).is_err());
    }
}
