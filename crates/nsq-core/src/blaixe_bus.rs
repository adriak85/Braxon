use std::collections::BTreeSet;

use crate::{NativeNsqMachine, NativeNsqRuntime, NsqActuator, NsqAddress, NsqInstruction};

/// Direct Blaixe bus: endpoint identity is an NSQ address and execution is the
/// native NSQ transaction engine. A network MAC is deliberately not involved.
#[derive(Debug)]
pub struct BlaixeBus {
    runtime: NativeNsqRuntime<NativeNsqMachine>,
    endpoints: BTreeSet<NsqAddress>,
}

impl BlaixeBus {
    pub fn new<I>(endpoints: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = NsqAddress>,
    {
        let endpoints = endpoints.into_iter().collect::<BTreeSet<_>>();
        if endpoints.is_empty() {
            return Err("Blaixe requires at least one NSQ endpoint".into());
        }
        Ok(Self {
            runtime: NativeNsqRuntime::new(NativeNsqMachine::default()),
            endpoints,
        })
    }

    pub fn endpoints(&self) -> &BTreeSet<NsqAddress> {
        &self.endpoints
    }

    pub fn dispatch(
        &mut self,
        endpoint: &NsqAddress,
        stream: &[NsqInstruction],
    ) -> Result<(), String> {
        if !self.endpoints.contains(endpoint) {
            return Err("Blaixe endpoint is not registered".into());
        }
        self.runtime.execute(stream).map(|_| ())
    }

    pub fn snapshot(&self) -> std::collections::BTreeMap<NsqAddress, crate::NSQSlot> {
        self.runtime.actuator().snapshot()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Charge, Dialect, NSQLever, NSQSlot};

    fn address(position: u64) -> NsqAddress {
        NsqAddress::root(NSQSlot::new(
            Dialect::Control,
            vec![NSQLever::new(Charge::Positive, position).unwrap()],
        ))
    }

    #[test]
    fn blaixe_dispatches_directly_by_nsq_address() {
        let endpoint = address(1);
        let target = address(2);
        let value = NSQSlot::new(
            Dialect::Intent,
            vec![NSQLever::new(Charge::Positive, 3).unwrap()],
        );
        let mut bus = BlaixeBus::new([endpoint.clone()]).unwrap();
        bus.dispatch(
            &endpoint,
            &[NsqInstruction::Set {
                address: target.clone(),
                value: value.clone(),
            }],
        )
        .unwrap();
        assert_eq!(bus.snapshot().get(&target), Some(&value));
        assert!(bus.dispatch(&address(99), &[]).is_err());
    }
}
