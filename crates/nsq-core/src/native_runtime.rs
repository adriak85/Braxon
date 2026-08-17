use std::collections::{BTreeMap, BTreeSet};

use crate::{Charge, Dialect, NSQLever, NSQSlot};

pub const NSQ_NATIVE_RUNTIME_SCHEMA: &str = "nsq.native_runtime.v1";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NsqAddress {
    pub path: Vec<NSQSlot>,
}

impl NsqAddress {
    pub fn new(path: Vec<NSQSlot>) -> Result<Self, String> {
        if path.is_empty() {
            return Err("NSQ address requires at least one slot".into());
        }
        Ok(Self { path })
    }
    pub fn root(slot: NSQSlot) -> Self {
        Self { path: vec![slot] }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NsqLeasePhase {
    Acquire,
    Hold,
    Commit,
    Release,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NsqLease {
    pub target: NsqAddress,
    pub owner: NsqAddress,
    pub phase: NsqLeasePhase,
    pub generation: u64,
}

#[derive(Debug, Default, Clone)]
pub struct NativeNsqOwnership {
    leases: BTreeMap<NsqAddress, NsqLease>,
    generation: u64,
}

impl NativeNsqOwnership {
    pub fn acquire(&mut self, owner: NsqAddress, targets: &[NsqAddress]) -> Result<u64, String> {
        if targets.is_empty() {
            return Err("NSQ ownership requires at least one target".into());
        }
        if targets
            .iter()
            .any(|target| self.leases.contains_key(target))
        {
            return Err("NSQ target is already owned".into());
        }
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or("NSQ ownership generation overflow")?;
        for target in targets {
            self.leases.insert(
                target.clone(),
                NsqLease {
                    target: target.clone(),
                    owner: owner.clone(),
                    phase: NsqLeasePhase::Acquire,
                    generation: self.generation,
                },
            );
        }
        Ok(self.generation)
    }
    pub fn advance(&mut self, owner: &NsqAddress, phase: NsqLeasePhase) -> Result<(), String> {
        let owned: Vec<NsqAddress> = self
            .leases
            .values()
            .filter(|lease| &lease.owner == owner)
            .map(|lease| lease.target.clone())
            .collect();
        if owned.is_empty() {
            return Err("NSQ owner has no active leases".into());
        }
        for target in &owned {
            if let Some(lease) = self.leases.get_mut(target) {
                lease.phase = phase;
            }
        }
        if phase == NsqLeasePhase::Release {
            for target in owned {
                self.leases.remove(&target);
            }
        }
        Ok(())
    }
    pub fn leases(&self) -> &BTreeMap<NsqAddress, NsqLease> {
        &self.leases
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NsqInstruction {
    Set { address: NsqAddress, value: NSQSlot },
    Release { address: NsqAddress },
    Fire { address: NsqAddress },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NsqActuationReceipt {
    pub generation: u64,
    pub executed: usize,
    pub released: usize,
    pub fired: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NsqProvenance {
    System,
    Narrative,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NsqIntentFrame {
    pub intent: NsqAddress,
    pub source: NsqAddress,
    pub capability: NsqAddress,
    pub gradient: [NSQSlot; 8],
    pub targets: Vec<NsqAddress>,
    pub provenance: NsqProvenance,
}

impl NsqIntentFrame {
    pub fn validate(&self) -> Result<(), String> {
        if self.targets.is_empty() {
            return Err("NSQ intent requires at least one target".into());
        }
        if self.provenance == NsqProvenance::Narrative && self.capability == self.source {
            return Err("narrative source cannot equal system capability owner".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NsqIntentOutcome {
    Accepted,
    Queued,
    Deferred,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NsqIntentDecision {
    pub outcome: NsqIntentOutcome,
    pub generation: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NsqGhostReceipt {
    pub active: usize,
    pub wire: usize,
    pub released: bool,
}

#[derive(Debug, Default, Clone)]
pub struct NativeNsqGhostWindow {
    wire: BTreeMap<NsqAddress, NSQSlot>,
    active: BTreeSet<NsqAddress>,
    capacity: usize,
}

impl NativeNsqGhostWindow {
    pub fn new(capacity: usize) -> Result<Self, String> {
        if capacity == 0 {
            return Err("native NSQ ghost window capacity must be nonzero".into());
        }
        Ok(Self {
            wire: BTreeMap::new(),
            active: BTreeSet::new(),
            capacity,
        })
    }
    pub fn publish(&mut self, address: NsqAddress, value: NSQSlot) {
        self.wire.insert(address, value);
    }
    pub fn fire(&mut self, address: &NsqAddress) -> Result<NsqGhostReceipt, String> {
        if !self.wire.contains_key(address) {
            return Err("cannot fire an NSQ page absent from the wire".into());
        }
        if !self.active.contains(address) && self.active.len() >= self.capacity {
            return Err("native NSQ ghost aperture is occupied".into());
        }
        self.active.insert(address.clone());
        Ok(NsqGhostReceipt {
            active: self.active.len(),
            wire: self.wire.len(),
            released: false,
        })
    }
    pub fn release(&mut self, address: &NsqAddress) -> Result<NsqGhostReceipt, String> {
        if !self.active.remove(address) {
            return Err("cannot release an inactive NSQ page".into());
        }
        Ok(NsqGhostReceipt {
            active: self.active.len(),
            wire: self.wire.len(),
            released: true,
        })
    }
    pub fn active(&self) -> &BTreeSet<NsqAddress> {
        &self.active
    }
    pub fn wire(&self) -> &BTreeMap<NsqAddress, NSQSlot> {
        &self.wire
    }
}

pub trait NsqActuator {
    fn set(&mut self, address: &NsqAddress, value: &NSQSlot, generation: u64)
    -> Result<(), String>;
    fn release(&mut self, address: &NsqAddress, generation: u64) -> Result<(), String>;
    fn fire(&mut self, address: &NsqAddress, generation: u64) -> Result<(), String>;
    fn snapshot(&self) -> BTreeMap<NsqAddress, NSQSlot>;
}

#[derive(Debug, Default, Clone)]
pub struct NativeNsqMachine {
    state: BTreeMap<NsqAddress, NSQSlot>,
    fired: Vec<NsqAddress>,
    last_generation: u64,
}

impl NsqActuator for NativeNsqMachine {
    fn set(
        &mut self,
        address: &NsqAddress,
        value: &NSQSlot,
        generation: u64,
    ) -> Result<(), String> {
        if generation < self.last_generation {
            return Err("stale NSQ generation rejected".into());
        }
        self.state.insert(address.clone(), value.clone());
        self.last_generation = generation;
        Ok(())
    }
    fn release(&mut self, address: &NsqAddress, generation: u64) -> Result<(), String> {
        if generation < self.last_generation {
            return Err("stale NSQ release rejected".into());
        }
        self.state.remove(address);
        self.last_generation = generation;
        Ok(())
    }
    fn fire(&mut self, address: &NsqAddress, generation: u64) -> Result<(), String> {
        if generation < self.last_generation {
            return Err("stale NSQ fire rejected".into());
        }
        if !self.state.contains_key(address) {
            return Err("cannot fire an unmapped NSQ address".into());
        }
        self.fired.push(address.clone());
        self.last_generation = generation;
        Ok(())
    }
    fn snapshot(&self) -> BTreeMap<NsqAddress, NSQSlot> {
        self.state.clone()
    }
}

#[derive(Debug)]
pub struct NativeNsqRuntime<A: NsqActuator> {
    actuator: A,
    generation: u64,
    active: BTreeMap<NsqAddress, NSQSlot>,
}

impl<A: NsqActuator> NativeNsqRuntime<A> {
    pub fn new(actuator: A) -> Self {
        Self {
            actuator,
            generation: 0,
            active: BTreeMap::new(),
        }
    }
    pub fn actuator(&self) -> &A {
        &self.actuator
    }
    pub fn actuator_mut(&mut self) -> &mut A {
        &mut self.actuator
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn execute(
        &mut self,
        instructions: &[NsqInstruction],
    ) -> Result<NsqActuationReceipt, String> {
        if instructions.is_empty() {
            return Err("empty NSQ instruction stream rejected".into());
        }
        let next_generation = self
            .generation
            .checked_add(1)
            .ok_or("NSQ generation overflow")?;
        let mut shadow = self.active.clone();
        for instruction in instructions {
            match instruction {
                NsqInstruction::Set { address, value } => {
                    shadow.insert(address.clone(), value.clone());
                }
                NsqInstruction::Release { address } => {
                    if shadow.remove(address).is_none() {
                        return Err("cannot release an unmapped NSQ address".into());
                    }
                }
                NsqInstruction::Fire { address } => {
                    if !shadow.contains_key(address) {
                        return Err("cannot fire an unmapped NSQ address".into());
                    }
                }
            }
        }
        self.generation = next_generation;
        let mut receipt = NsqActuationReceipt {
            generation: self.generation,
            executed: 0,
            released: 0,
            fired: 0,
        };
        for instruction in instructions {
            match instruction {
                NsqInstruction::Set { address, value } => {
                    self.actuator.set(address, value, self.generation)?;
                    self.active.insert(address.clone(), value.clone());
                }
                NsqInstruction::Release { address } => {
                    self.actuator.release(address, self.generation)?;
                    self.active.remove(address);
                    receipt.released += 1;
                }
                NsqInstruction::Fire { address } => {
                    self.actuator.fire(address, self.generation)?;
                    receipt.fired += 1;
                }
            }
            receipt.executed += 1;
        }
        Ok(receipt)
    }
}

pub fn test_slot(position: u64) -> NSQSlot {
    NSQSlot::new(
        Dialect::Control,
        vec![NSQLever::new(Charge::Positive, position).expect("test position is canonical")],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_ghost_window_rotates_ns_q_pages_without_storage_payloads() {
        let a = NsqAddress::root(test_slot(10));
        let b = NsqAddress::root(test_slot(11));
        let c = NsqAddress::root(test_slot(12));
        let value = test_slot(77);
        let mut window = NativeNsqGhostWindow::new(1).unwrap();
        window.publish(a.clone(), value.clone());
        window.publish(b.clone(), value.clone());
        window.publish(c.clone(), value);
        assert_eq!(window.wire().len(), 3);
        assert_eq!(window.fire(&a).unwrap().active, 1);
        assert!(window.fire(&b).is_err());
        window.release(&a).unwrap();
        assert_eq!(window.fire(&b).unwrap().active, 1);
    }

    #[test]
    fn native_ns_q_executes_set_fire_release_without_binary_payload() {
        let address = NsqAddress::root(test_slot(7));
        let value = test_slot(42);
        let mut runtime = NativeNsqRuntime::new(NativeNsqMachine::default());
        let receipt = runtime
            .execute(&[
                NsqInstruction::Set {
                    address: address.clone(),
                    value: value.clone(),
                },
                NsqInstruction::Fire {
                    address: address.clone(),
                },
            ])
            .unwrap();
        assert_eq!(receipt.executed, 2);
        assert_eq!(receipt.fired, 1);
        assert_eq!(runtime.actuator().snapshot().get(&address), Some(&value));
        runtime
            .execute(&[NsqInstruction::Release {
                address: address.clone(),
            }])
            .unwrap();
        assert!(!runtime.actuator().snapshot().contains_key(&address));
    }

    #[test]
    fn cannot_fire_unmapped_address_or_execute_empty_stream() {
        let address = NsqAddress::root(test_slot(8));
        let mut runtime = NativeNsqRuntime::new(NativeNsqMachine::default());
        assert!(runtime.execute(&[]).is_err());
        assert!(
            runtime
                .execute(&[NsqInstruction::Fire { address }])
                .is_err()
        );
    }

    #[test]
    fn invalid_native_stream_does_not_partially_mutate_state() {
        let address = NsqAddress::root(test_slot(6));
        let value = test_slot(60);
        let missing = NsqAddress::root(test_slot(61));
        let mut runtime = NativeNsqRuntime::new(NativeNsqMachine::default());
        runtime
            .execute(&[NsqInstruction::Set {
                address: address.clone(),
                value: value.clone(),
            }])
            .unwrap();
        let before = runtime.actuator().snapshot();
        assert!(
            runtime
                .execute(&[
                    NsqInstruction::Set {
                        address: address.clone(),
                        value: test_slot(61)
                    },
                    NsqInstruction::Fire { address: missing },
                ])
                .is_err()
        );
        assert_eq!(runtime.actuator().snapshot(), before);
    }

    #[test]
    fn native_ownership_prevents_same_space_override_until_release() {
        let owner_a = NsqAddress::root(test_slot(1));
        let owner_b = NsqAddress::root(test_slot(2));
        let target = NsqAddress::new(vec![test_slot(3), test_slot(4)]).unwrap();
        let mut ownership = NativeNsqOwnership::default();
        assert_eq!(
            ownership
                .acquire(owner_a.clone(), std::slice::from_ref(&target))
                .unwrap(),
            1
        );
        assert!(
            ownership
                .acquire(owner_b.clone(), std::slice::from_ref(&target))
                .is_err()
        );
        ownership.advance(&owner_a, NsqLeasePhase::Release).unwrap();
        assert_eq!(
            ownership
                .acquire(owner_b, std::slice::from_ref(&target))
                .unwrap(),
            2
        );
    }

    #[test]
    fn nsq_state_is_addressed_by_slots() {
        let address_a = NsqAddress::new(vec![test_slot(1), test_slot(2)]).unwrap();
        let address_b = NsqAddress::new(vec![test_slot(1), test_slot(3)]).unwrap();
        let value = test_slot(99);
        let mut runtime = NativeNsqRuntime::new(NativeNsqMachine::default());
        runtime
            .execute(&[
                NsqInstruction::Set {
                    address: address_a.clone(),
                    value: value.clone(),
                },
                NsqInstruction::Set {
                    address: address_b.clone(),
                    value: value.clone(),
                },
            ])
            .unwrap();
        assert_eq!(runtime.actuator().snapshot().len(), 2);
        assert!(address_a < address_b);
    }
}
