use std::collections::BTreeSet;

use nsq_core::{
    NativeNsqOwnership, NsqAddress, NsqIntentDecision, NsqIntentFrame, NsqIntentOutcome,
    NsqLeasePhase,
};

pub const NATIVE_BRAXON_BUS_SCHEMA: &str = "braxon.native_nsq_bus.v1";

#[derive(Debug, Default)]
pub struct NativeNsqBus {
    council: BTreeSet<NsqAddress>,
    ownership: NativeNsqOwnership,
}

impl NativeNsqBus {
    pub fn new(council: impl IntoIterator<Item = NsqAddress>) -> Result<Self, String> {
        let council: BTreeSet<NsqAddress> = council.into_iter().collect();
        if council.len() != 10 {
            return Err("native NSQ council must contain exactly ten distinct addresses".into());
        }
        Ok(Self {
            council,
            ownership: NativeNsqOwnership::default(),
        })
    }

    pub fn council(&self) -> impl Iterator<Item = &NsqAddress> {
        self.council.iter()
    }
    pub fn ownership(&self) -> &NativeNsqOwnership {
        &self.ownership
    }

    pub fn decide(&mut self, frame: &NsqIntentFrame) -> NsqIntentDecision {
        if let Err(_) = frame.validate() {
            return NsqIntentDecision {
                outcome: NsqIntentOutcome::Rejected,
                generation: None,
            };
        }
        if !self.council.contains(&frame.capability) {
            return NsqIntentDecision {
                outcome: NsqIntentOutcome::Deferred,
                generation: None,
            };
        }
        if frame
            .targets
            .iter()
            .any(|target| self.ownership.leases().contains_key(target))
        {
            return NsqIntentDecision {
                outcome: NsqIntentOutcome::Queued,
                generation: None,
            };
        }
        match self
            .ownership
            .acquire(frame.capability.clone(), &frame.targets)
        {
            Ok(generation) => NsqIntentDecision {
                outcome: NsqIntentOutcome::Accepted,
                generation: Some(generation),
            },
            Err(_) => NsqIntentDecision {
                outcome: NsqIntentOutcome::Queued,
                generation: None,
            },
        }
    }

    pub fn advance(&mut self, owner: &NsqAddress, phase: NsqLeasePhase) -> Result<(), String> {
        self.ownership.advance(owner, phase)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nsq_core::{Charge, Dialect, NSQLever, NSQSlot, NsqProvenance};

    fn address(position: u64) -> NsqAddress {
        NsqAddress::root(NSQSlot::new(
            Dialect::Control,
            vec![NSQLever::new(Charge::Positive, position).unwrap()],
        ))
    }
    fn frame(owner: NsqAddress, target: NsqAddress) -> NsqIntentFrame {
        let slot = NSQSlot::new(
            Dialect::Intent,
            vec![NSQLever::new(Charge::Positive, 1).unwrap()],
        );
        NsqIntentFrame {
            intent: address(100),
            source: address(101),
            capability: owner,
            gradient: std::array::from_fn(|_| slot.clone()),
            targets: vec![target],
            provenance: NsqProvenance::System,
        }
    }

    #[test]
    fn native_bus_uses_ns_q_address_ownership_and_piston_release() {
        let seats: Vec<_> = (1..=10).map(address).collect();
        let owner = seats[0].clone();
        let target = address(200);
        let mut bus = NativeNsqBus::new(seats).unwrap();
        assert_eq!(
            bus.decide(&frame(owner.clone(), target.clone())).outcome,
            NsqIntentOutcome::Accepted
        );
        assert_eq!(
            bus.decide(&frame(owner.clone(), target.clone())).outcome,
            NsqIntentOutcome::Queued
        );
        bus.advance(&owner, NsqLeasePhase::Release).unwrap();
        assert_eq!(
            bus.decide(&frame(owner, target)).outcome,
            NsqIntentOutcome::Accepted
        );
    }
}
