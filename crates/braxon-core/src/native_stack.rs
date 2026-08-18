use std::collections::BTreeMap;

use nsq_core::{
    register_reconstructed_tool_intents, BlaixeBus, NSQSlot, NativeNsqGhostWindow,
    NativeNsqMachine, NativeNsqReflexor, NativeNsqRuntime, NativeNsqTargetField, NsqAddress,
    NsqInstruction, RawNsqCapability, RawNsqEngine, RawNsqEvent, RawNsqOutcome,
};

use crate::NativeNsqBus;

#[derive(Debug)]
pub struct NativeNsqStack {
    pub runtime: NativeNsqRuntime<NativeNsqMachine>,
    pub bus: NativeNsqBus,
    pub direct_bus: BlaixeBus,
    pub ghost: NativeNsqGhostWindow,
    pub reflexor: NativeNsqReflexor,
    pub target: NativeNsqTargetField,
    pub raw_intent: RawNsqEngine,
}

impl NativeNsqStack {
    pub fn new(
        council: impl IntoIterator<Item = NsqAddress>,
        target: NsqAddress,
        desired: NSQSlot,
        ghost_capacity: usize,
    ) -> Result<Self, String> {
        let council = council.into_iter().collect::<Vec<_>>();
        let mut raw_intent = RawNsqEngine::default();
        register_reconstructed_tool_intents(&mut raw_intent)?;
        Ok(Self {
            runtime: NativeNsqRuntime::new(NativeNsqMachine::default()),
            bus: NativeNsqBus::new(council.clone())?,
            direct_bus: BlaixeBus::new(council)?,
            ghost: NativeNsqGhostWindow::new(ghost_capacity)?,
            reflexor: NativeNsqReflexor::new(),
            target: NativeNsqTargetField::new(target, desired),
            raw_intent,
        })
    }

    pub fn dispatch_direct(
        &mut self,
        endpoint: &NsqAddress,
        stream: &[NsqInstruction],
    ) -> Result<(), String> {
        self.direct_bus.dispatch(endpoint, stream)
    }

    pub fn execute_target(&mut self, observed: Option<&NSQSlot>) -> Result<bool, String> {
        let instruction = self.target.reconcile(observed);
        if let Some(instruction) = instruction {
            self.runtime.execute(std::slice::from_ref(&instruction))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn discover_raw_capabilities(&self, query: &str) -> Vec<&RawNsqCapability> {
        self.raw_intent.discover(query)
    }

    pub fn dispatch_raw_intent(&mut self, event: RawNsqEvent) -> RawNsqOutcome {
        self.raw_intent.dispatch(event)
    }

    pub fn execute_reflex_delta(
        &mut self,
        published: BTreeMap<NsqAddress, NSQSlot>,
        hardware: &BTreeMap<NsqAddress, NSQSlot>,
    ) -> Result<usize, String> {
        let report = self.reflexor.orbit(published, hardware);
        let count = report.instructions.len();
        if count > 0 {
            self.runtime.execute(&report.instructions)?;
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nsq_core::{Charge, Dialect, NSQLever};

    fn address(position: u64) -> NsqAddress {
        NsqAddress::root(NSQSlot::new(
            Dialect::Control,
            vec![NSQLever::new(Charge::Positive, position).unwrap()],
        ))
    }

    fn slot(position: u64) -> NSQSlot {
        NSQSlot::new(
            Dialect::Intent,
            vec![NSQLever::new(Charge::Positive, position).unwrap()],
        )
    }

    #[test]
    fn native_stack_routes_target_and_reflex_deltas_to_one_runtime() {
        let council = (1..=10).map(address).collect::<Vec<_>>();
        let mut stack = NativeNsqStack::new(council, address(20), slot(21), 1).unwrap();
        assert_eq!(stack.discover_raw_capabilities("tree-sitter").len(), 1);
        let mut raw_input = BTreeMap::new();
        raw_input.insert("input".into(), "source-unit".into());
        assert!(matches!(
            stack.dispatch_raw_intent(nsq_core::RawNsqEvent::Invoke {
                capability_id: "tree_sitter.parse".into(),
                input: raw_input,
            }),
            nsq_core::RawNsqOutcome::Accepted { .. }
        ));
        assert!(matches!(
            stack.dispatch_raw_intent(nsq_core::RawNsqEvent::Correct {
                capability_id: "tree_sitter.parse".into(),
                field: "input".into(),
                expected: "source-unit-v2".into(),
                observed: "source-unit".into(),
            }),
            nsq_core::RawNsqOutcome::Corrected { .. }
        ));
        assert!(stack.execute_target(None).unwrap());
        assert!(!stack.execute_target(Some(&slot(21))).unwrap());
        let endpoint = address(1);
        let direct_target = address(40);
        let direct_value = slot(41);
        stack
            .dispatch_direct(
                &endpoint,
                &[NsqInstruction::Set {
                    address: direct_target.clone(),
                    value: direct_value.clone(),
                }],
            )
            .unwrap();
        assert_eq!(
            stack.direct_bus.snapshot().get(&direct_target),
            Some(&direct_value)
        );
        let mut published = BTreeMap::new();
        published.insert(address(30), slot(31));
        let mut hardware = BTreeMap::new();
        hardware.insert(address(30), slot(32));
        assert_eq!(stack.execute_reflex_delta(published, &hardware).unwrap(), 1);
    }
}
