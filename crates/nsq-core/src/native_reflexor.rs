use std::collections::BTreeMap;

use crate::{NSQSlot, NsqAddress, NsqInstruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NsqReflexPhase {
    Publish,
    Reconcile,
    DeltaCommit,
}

impl Default for NsqReflexPhase {
    fn default() -> Self {
        Self::Publish
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NsqReflexReport {
    pub phase: NsqReflexPhase,
    pub published: usize,
    pub changed: usize,
    pub instructions: Vec<NsqInstruction>,
}

#[derive(Debug, Default, Clone)]
pub struct NativeNsqReflexor {
    watermark: BTreeMap<NsqAddress, NSQSlot>,
    phase: NsqReflexPhase,
}

impl NativeNsqReflexor {
    pub fn new() -> Self {
        Self {
            watermark: BTreeMap::new(),
            phase: NsqReflexPhase::Publish,
        }
    }
    pub fn phase(&self) -> NsqReflexPhase {
        self.phase
    }
    pub fn watermark(&self) -> &BTreeMap<NsqAddress, NSQSlot> {
        &self.watermark
    }

    pub fn orbit(
        &mut self,
        published: BTreeMap<NsqAddress, NSQSlot>,
        hardware: &BTreeMap<NsqAddress, NSQSlot>,
    ) -> NsqReflexReport {
        self.phase = NsqReflexPhase::Publish;
        let changed: Vec<_> = published
            .iter()
            .filter(|(address, value)| hardware.get(*address) != Some(*value))
            .map(|(address, value)| (address.clone(), value.clone()))
            .collect();
        self.phase = NsqReflexPhase::Reconcile;
        let instructions: Vec<_> = changed
            .iter()
            .map(|(address, value)| NsqInstruction::Set {
                address: address.clone(),
                value: value.clone(),
            })
            .collect();
        self.phase = NsqReflexPhase::DeltaCommit;
        self.watermark = published;
        NsqReflexReport {
            phase: self.phase,
            published: self.watermark.len(),
            changed: changed.len(),
            instructions,
        }
    }

    pub fn orbit_dirty(
        &mut self,
        published: BTreeMap<NsqAddress, NSQSlot>,
        hardware: &BTreeMap<NsqAddress, NSQSlot>,
        dirty: &[NsqAddress],
    ) -> NsqReflexReport {
        self.phase = NsqReflexPhase::Publish;
        let changed: Vec<_> = dirty
            .iter()
            .filter_map(|address| {
                published
                    .get(address)
                    .map(|value| (address.clone(), value.clone()))
            })
            .filter(|(address, value)| hardware.get(address) != Some(value))
            .collect();
        self.phase = NsqReflexPhase::Reconcile;
        let instructions: Vec<_> = changed
            .iter()
            .map(|(address, value)| NsqInstruction::Set {
                address: address.clone(),
                value: value.clone(),
            })
            .collect();
        self.phase = NsqReflexPhase::DeltaCommit;
        self.watermark = published;
        NsqReflexReport {
            phase: self.phase,
            published: self.watermark.len(),
            changed: changed.len(),
            instructions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Charge, Dialect, NSQLever};

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
    fn delta_orbit_reduces_native_operations_for_large_resident_frame() {
        let mut published = BTreeMap::new();
        for position in 1..=1024 {
            published.insert(address(position), slot(position));
        }
        let mut hardware = published.clone();
        let changed_address = address(1024);
        hardware.insert(changed_address.clone(), slot(2048));
        let mut reflexor = NativeNsqReflexor::new();
        let report = reflexor.orbit(published, &hardware);
        assert_eq!(report.published, 1024);
        assert_eq!(report.changed, 1);
        assert_eq!(report.instructions.len(), 1);
    }

    #[test]
    fn reflexor_emits_only_native_nsq_changes_and_refreshes_watermark() {
        let a = address(1);
        let b = address(2);
        let mut published = BTreeMap::new();
        published.insert(a.clone(), slot(10));
        published.insert(b.clone(), slot(20));
        let mut hardware = BTreeMap::new();
        hardware.insert(a.clone(), slot(10));
        let mut reflexor = NativeNsqReflexor::new();
        let report = reflexor.orbit(published.clone(), &hardware);
        assert_eq!(report.phase, NsqReflexPhase::DeltaCommit);
        assert_eq!(report.changed, 1);
        assert!(
            matches!(&report.instructions[0], NsqInstruction::Set { address, .. } if address == &b)
        );
        assert_eq!(reflexor.watermark(), &published);
    }
}
