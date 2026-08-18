use std::collections::BTreeMap;
use std::path::Path;

use crate::{
    BraxonContextManifest, KineticReflexor, NativeNsqStack, ReflexorReport, SemanticPointer,
};
use nsq_core::{RawNsqEvent, RawNsqOutcome};

pub const SEMANTIC_LINK_SCHEMA: &str = "braxon.nsq.semantic_link.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticLinkRequest {
    pub pointer_id: String,
    pub query: String,
    pub input: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticLinkResolution {
    pub pointer: SemanticPointer,
    pub capability_id: String,
    pub route_is_authorized: bool,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticLinkReceipt {
    pub schema: String,
    pub pointer_id: String,
    pub path: String,
    pub capability_id: String,
    pub outcome: RawNsqOutcome,
    pub reflexor_report: ReflexorReport,
    pub materialized_bytes: u64,
    pub resident_client_bytes: u64,
    pub released: bool,
}

#[derive(Debug, Default)]
pub struct SemanticLinkSurface {
    reflexor: KineticReflexor,
}

impl SemanticLinkSurface {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn discover<'a>(
        manifest: &'a BraxonContextManifest,
        query: &str,
    ) -> Vec<&'a SemanticPointer> {
        let needle = query.trim().to_ascii_lowercase();
        manifest
            .semantic_pointers
            .iter()
            .filter(|pointer| {
                needle.is_empty()
                    || pointer.id.to_ascii_lowercase().contains(&needle)
                    || pointer.kind.to_ascii_lowercase().contains(&needle)
                    || pointer.path.to_ascii_lowercase().contains(&needle)
                    || pointer
                        .route
                        .iter()
                        .any(|route| route.to_ascii_lowercase().contains(&needle))
            })
            .collect()
    }

    pub fn resolve(
        &self,
        manifest: &BraxonContextManifest,
        root: &Path,
        request: &SemanticLinkRequest,
        stack: &NativeNsqStack,
    ) -> Result<SemanticLinkResolution, String> {
        let pointer = manifest
            .semantic_pointers
            .iter()
            .find(|pointer| pointer.id == request.pointer_id)
            .cloned()
            .ok_or_else(|| format!("semantic pointer '{}' is unresolved", request.pointer_id))?;
        if !request.query.trim().is_empty()
            && !Self::discover(manifest, &request.query)
                .iter()
                .any(|candidate| candidate.id == pointer.id)
        {
            return Err("semantic pointer does not satisfy the requested query".into());
        }
        if pointer.route.is_empty() {
            return Err(format!(
                "semantic pointer '{}' has no dispatch route",
                pointer.id
            ));
        }
        if !root.join(&pointer.path).exists() {
            return Err(format!(
                "semantic pointer path is unavailable: {}",
                pointer.path
            ));
        }
        let capability_id = pointer.route[0].clone();
        if stack.discover_raw_capabilities(&capability_id).is_empty() {
            return Err(format!("capability route is unavailable: {capability_id}"));
        }
        Ok(SemanticLinkResolution {
            pointer,
            capability_id,
            route_is_authorized: true,
            reason: "repository-addressed route resolved through NSQ capability authority".into(),
        })
    }

    pub fn invoke(
        &mut self,
        stack: &mut NativeNsqStack,
        resolution: SemanticLinkResolution,
        input: BTreeMap<String, String>,
    ) -> Result<SemanticLinkReceipt, String> {
        if !resolution.route_is_authorized {
            return Err("semantic link route is not authorized".into());
        }
        let byte_len = input
            .values()
            .map(|value| value.len() as u64)
            .sum::<u64>()
            .max(1);
        let value_hash = stable_input_hash(&input);
        let bus_value = crate::BusValue {
            key: resolution.pointer.id.clone(),
            class: crate::ValueClass::Fact,
            value_hash,
            byte_len,
        };
        self.reflexor.publish([bus_value])?;
        self.reflexor.reconcile()?;
        let outcome = stack.dispatch_raw_intent(RawNsqEvent::Invoke {
            capability_id: resolution.capability_id.clone(),
            input,
        });
        if !matches!(
            outcome,
            RawNsqOutcome::Accepted { .. } | RawNsqOutcome::Corrected { .. }
        ) {
            return Err(format!("semantic link dispatch failed: {outcome:?}"));
        }
        let expected: Vec<String> = self
            .reflexor
            .pending_delta()
            .iter()
            .map(|delta| delta.key.clone())
            .collect();
        let generation = self.reflexor.generation();
        let report = self.reflexor.commit_hardware(crate::HardwareWriteAck {
            adapter_id: "semantic-link-release".into(),
            generation,
            accepted: true,
            written_keys: expected,
        })?;
        Ok(SemanticLinkReceipt {
            schema: SEMANTIC_LINK_SCHEMA.into(),
            pointer_id: resolution.pointer.id,
            path: resolution.pointer.path,
            capability_id: resolution.capability_id,
            outcome,
            reflexor_report: report,
            materialized_bytes: byte_len,
            resident_client_bytes: 0,
            released: true,
        })
    }

    pub fn reflexor(&self) -> &KineticReflexor {
        &self.reflexor
    }
}

fn stable_input_hash(input: &BTreeMap<String, String>) -> String {
    let mut acc = 0xcbf29ce484222325_u128;
    for (key, value) in input {
        for byte in key.as_bytes().iter().chain(value.as_bytes()) {
            acc ^= *byte as u128;
            acc = acc.wrapping_mul(0x100000001b3);
        }
    }
    format!("{acc:032x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NativeNsqStack;
    use nsq_core::{Charge, Dialect, NSQLever, NSQSlot};

    fn address(position: u64) -> nsq_core::NsqAddress {
        nsq_core::NsqAddress::root(NSQSlot::new(
            Dialect::Control,
            vec![NSQLever::new(Charge::Positive, position).unwrap()],
        ))
    }

    fn stack() -> NativeNsqStack {
        NativeNsqStack::new(
            (1..=10).map(address),
            address(20),
            NSQSlot::new(
                Dialect::Intent,
                vec![NSQLever::new(Charge::Positive, 21).unwrap()],
            ),
            1,
        )
        .unwrap()
    }

    fn manifest(route: &str) -> BraxonContextManifest {
        BraxonContextManifest {
            schema: "test".into(),
            generated_at: "test".into(),
            identity: "test".into(),
            canonical_semantics: "base8_switch_topology".into(),
            private_license: true,
            offline_context_api: "test".into(),
            semantic_pointers: vec![SemanticPointer {
                id: "syntax-source".into(),
                kind: "syntax".into(),
                path: "Cargo.toml".into(),
                required: true,
                relationship: "semantic-route".into(),
                route: vec![route.into()],
            }],
            known_left_out: Vec::new(),
            left_out_policy: crate::context_manifest::LeftOutPolicy {
                must_call_out_omissions: true,
                missing_required_pointer_action: "fail".into(),
                missing_optional_pointer_action: "report".into(),
                citadel_surfaces_must_be_named: true,
            },
            wake_triggers: serde_json::from_str("{\"enabled_by_env\":\"TEST\",\"changed_files_env\":\"TEST_FILES\",\"default_mode\":\"test\",\"overhead_policy\":\"bounded\",\"surface_match_mode\":\"path\",\"suggest_linked_changes_for_each_changed_surface\":true,\"pipe_and_chain_identification\":true}").unwrap(),
        }
    }

    #[test]
    fn semantic_link_discovers_and_dispatches_without_resident_client_state() {
        let manifest = manifest("tree_sitter.parse");
        assert_eq!(SemanticLinkSurface::discover(&manifest, "syntax").len(), 1);
        let request = SemanticLinkRequest {
            pointer_id: "syntax-source".into(),
            query: "syntax".into(),
            input: [("input".into(), "(x)".into())].into_iter().collect(),
        };
        let mut surface = SemanticLinkSurface::new();
        let mut stack = stack();
        let resolution = surface
            .resolve(&manifest, Path::new("."), &request, &stack)
            .unwrap();
        let receipt = surface
            .invoke(&mut stack, resolution, request.input)
            .unwrap();
        assert!(matches!(receipt.outcome, RawNsqOutcome::Accepted { .. }));
        assert_eq!(receipt.resident_client_bytes, 0);
        assert!(receipt.released);
        assert_eq!(surface.reflexor().pending_delta().len(), 0);
    }

    #[test]
    fn unresolved_capability_fails_closed() {
        let manifest = manifest("missing.capability");
        let request = SemanticLinkRequest {
            pointer_id: "syntax-source".into(),
            query: "".into(),
            input: BTreeMap::new(),
        };
        assert!(SemanticLinkSurface::new()
            .resolve(&manifest, Path::new("."), &request, &stack())
            .is_err());
    }
}
