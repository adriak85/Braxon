use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const PISTON_MEMORY_SCHEMA: &str = "braxon.nsq.piston_memory.v1";
pub const CPU_ADDRESS_SPACE_BYTES: u64 = 1 << 48;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionKind {
    Parameter,
    Weight,
    KvCache,
    Activation,
    Tokenizer,
    Launcher,
    Fact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Residency {
    Virtual,
    Resident,
    Evictable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub region_id: String,
    pub kind: RegionKind,
    pub base_address: u64,
    pub byte_len: u64,
    pub cpu_visible: bool,
    pub residency: Residency,
    pub address_owner: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PistonPhase {
    Acquire,
    Hold,
    Commit,
    Release,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryLease {
    pub lease_id: String,
    pub intent_id: String,
    pub region_id: String,
    pub phase: PistonPhase,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryDecision {
    Accepted,
    Deferred,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryDecisionReport {
    pub schema: String,
    pub decision: MemoryDecision,
    pub reason: String,
    pub lease: Option<MemoryLease>,
    pub resident_bytes: u64,
    pub virtual_bytes: u64,
}

#[derive(Debug, Default)]
pub struct PistonMemory {
    regions: BTreeMap<String, MemoryRegion>,
    leases: BTreeMap<String, MemoryLease>,
    next_dynamic_address: u64,
    ram_budget_bytes: u64,
    resident_bytes: u64,
    generation: u64,
}

impl PistonMemory {
    pub fn new(ram_budget_bytes: u64) -> Self {
        Self {
            regions: BTreeMap::new(),
            leases: BTreeMap::new(),
            next_dynamic_address: 0x1000_0000,
            ram_budget_bytes,
            resident_bytes: 0,
            generation: 0,
        }
    }

    pub fn map_fixed(&mut self, region: MemoryRegion) -> Result<(), String> {
        if region.region_id.trim().is_empty()
            || region.address_owner.trim().is_empty()
            || region.byte_len == 0
        {
            return Err(
                "memory region identity, owner, and nonzero length are required".to_string(),
            );
        }
        let end = region
            .base_address
            .checked_add(region.byte_len)
            .ok_or_else(|| "memory region address overflow".to_string())?;
        if end > CPU_ADDRESS_SPACE_BYTES {
            return Err("memory region exceeds CPU virtual address space".to_string());
        }
        if self
            .regions
            .values()
            .any(|existing| overlaps(&region, existing))
        {
            return Err("memory region overlaps an existing address mapping".to_string());
        }
        if self
            .regions
            .insert(region.region_id.clone(), region)
            .is_some()
        {
            return Err("duplicate memory region".to_string());
        }
        self.recompute_resident_bytes();
        Ok(())
    }

    pub fn allocate_dynamic(
        &mut self,
        region_id: &str,
        kind: RegionKind,
        byte_len: u64,
        owner: &str,
    ) -> Result<MemoryRegion, String> {
        if !matches!(kind, RegionKind::KvCache | RegionKind::Activation) {
            return Err(
                "only KV cache and activation regions may be dynamically allocated".to_string(),
            );
        }
        let base = self.next_dynamic_address;
        let end = base
            .checked_add(byte_len)
            .ok_or_else(|| "dynamic memory address overflow".to_string())?;
        if end > CPU_ADDRESS_SPACE_BYTES {
            return Err("dynamic allocation exceeds CPU virtual address space".to_string());
        }
        let region = MemoryRegion {
            region_id: region_id.to_string(),
            kind,
            base_address: base,
            byte_len,
            cpu_visible: true,
            residency: Residency::Virtual,
            address_owner: owner.to_string(),
        };
        self.map_fixed(region.clone())?;
        self.next_dynamic_address = end.next_multiple_of(0x1000);
        Ok(region)
    }

    pub fn acquire(&mut self, intent_id: &str, region_id: &str) -> MemoryDecisionReport {
        let Some(region) = self.regions.get(region_id).cloned() else {
            return self.report(MemoryDecision::Rejected, "unknown memory region", None);
        };
        if !region.cpu_visible {
            return self.report(MemoryDecision::Rejected, "region is not CPU-visible", None);
        }
        if self.leases.values().any(|lease| {
            lease.region_id == region_id
                && lease.intent_id != intent_id
                && lease.phase != PistonPhase::Release
        }) {
            return self.report(
                MemoryDecision::Deferred,
                "region is held by another piston",
                None,
            );
        }
        let additional = if region.residency == Residency::Resident {
            0
        } else {
            region.byte_len
        };
        if self.resident_bytes.saturating_add(additional) > self.ram_budget_bytes {
            return self.report(
                MemoryDecision::Deferred,
                "resident memory budget exhausted; no implicit eviction",
                None,
            );
        }
        self.generation += 1;
        if let Some(mapped) = self.regions.get_mut(region_id) {
            mapped.residency = Residency::Resident;
        }
        self.resident_bytes = self.resident_bytes.saturating_add(additional);
        let lease = MemoryLease {
            lease_id: format!("piston-{}-{}", self.generation, region_id),
            intent_id: intent_id.to_string(),
            region_id: region_id.to_string(),
            phase: PistonPhase::Acquire,
            generation: self.generation,
        };
        self.leases.insert(lease.lease_id.clone(), lease.clone());
        self.report(
            MemoryDecision::Accepted,
            "CPU-visible piston lease acquired",
            Some(lease),
        )
    }

    pub fn advance(&mut self, lease_id: &str, phase: PistonPhase) -> Result<(), String> {
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| "unknown piston lease".to_string())?;
        lease.phase = phase.clone();
        if phase == PistonPhase::Release {
            let region_id = lease.region_id.clone();
            self.leases.remove(lease_id);
            if let Some(region) = self.regions.get_mut(&region_id) {
                if matches!(region.kind, RegionKind::KvCache | RegionKind::Activation) {
                    region.residency = Residency::Evictable;
                }
            }
            self.recompute_resident_bytes();
        }
        Ok(())
    }

    pub fn region(&self, region_id: &str) -> Option<&MemoryRegion> {
        self.regions.get(region_id)
    }
    pub fn resident_bytes(&self) -> u64 {
        self.resident_bytes
    }
    pub fn virtual_bytes(&self) -> u64 {
        self.regions
            .values()
            .filter(|r| r.residency != Residency::Resident)
            .map(|r| r.byte_len)
            .sum()
    }

    fn recompute_resident_bytes(&mut self) {
        self.resident_bytes = self
            .regions
            .values()
            .filter(|r| r.residency == Residency::Resident)
            .map(|r| r.byte_len)
            .sum();
    }
    fn report(
        &self,
        decision: MemoryDecision,
        reason: &str,
        lease: Option<MemoryLease>,
    ) -> MemoryDecisionReport {
        MemoryDecisionReport {
            schema: PISTON_MEMORY_SCHEMA.to_string(),
            decision,
            reason: reason.to_string(),
            lease,
            resident_bytes: self.resident_bytes,
            virtual_bytes: self.virtual_bytes(),
        }
    }
}

fn overlaps(left: &MemoryRegion, right: &MemoryRegion) -> bool {
    let left_end = left.base_address.saturating_add(left.byte_len);
    let right_end = right.base_address.saturating_add(right.byte_len);
    left.base_address < right_end && right.base_address < left_end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixed(id: &str, kind: RegionKind, address: u64, bytes: u64) -> MemoryRegion {
        MemoryRegion {
            region_id: id.to_string(),
            kind,
            base_address: address,
            byte_len: bytes,
            cpu_visible: true,
            residency: Residency::Virtual,
            address_owner: "nsq-council".to_string(),
        }
    }

    #[test]
    fn fixed_parameter_mapping_is_cpu_visible_and_piston_leased() {
        let mut memory = PistonMemory::new(1024);
        memory
            .map_fixed(fixed("weights.layer0", RegionKind::Parameter, 0x1000, 512))
            .unwrap();
        let report = memory.acquire("intent-1", "weights.layer0");
        assert_eq!(report.decision, MemoryDecision::Accepted);
        assert_eq!(report.resident_bytes, 512);
        assert_eq!(
            memory.region("weights.layer0").unwrap().residency,
            Residency::Resident
        );
    }

    #[test]
    fn piston_prevents_same_region_override() {
        let mut memory = PistonMemory::new(2048);
        memory
            .map_fixed(fixed("kv.0", RegionKind::KvCache, 0x2000, 512))
            .unwrap();
        assert_eq!(
            memory.acquire("intent-1", "kv.0").decision,
            MemoryDecision::Accepted
        );
        assert_eq!(
            memory.acquire("intent-2", "kv.0").decision,
            MemoryDecision::Deferred
        );
    }

    #[test]
    fn kv_cache_is_bounded_and_fails_closed_under_pressure() {
        let mut memory = PistonMemory::new(512);
        memory
            .allocate_dynamic("kv.0", RegionKind::KvCache, 512, "attention")
            .unwrap();
        assert_eq!(
            memory.acquire("intent-1", "kv.0").decision,
            MemoryDecision::Accepted
        );
        memory
            .allocate_dynamic("kv.1", RegionKind::KvCache, 512, "attention")
            .unwrap();
        assert_eq!(
            memory.acquire("intent-2", "kv.1").decision,
            MemoryDecision::Deferred
        );
    }

    #[test]
    fn release_makes_dynamic_region_evictable_not_implicitly_resident() {
        let mut memory = PistonMemory::new(1024);
        memory
            .allocate_dynamic("activation.0", RegionKind::Activation, 512, "forward")
            .unwrap();
        let lease = memory.acquire("intent-1", "activation.0").lease.unwrap();
        memory
            .advance(&lease.lease_id, PistonPhase::Release)
            .unwrap();
        assert_eq!(memory.resident_bytes(), 0);
        assert_eq!(
            memory.region("activation.0").unwrap().residency,
            Residency::Evictable
        );
    }

    #[test]
    fn fixed_mappings_cannot_overlap() {
        let mut memory = PistonMemory::new(1024);
        memory
            .map_fixed(fixed("tokenizer", RegionKind::Tokenizer, 0x1000, 512))
            .unwrap();
        assert!(memory
            .map_fixed(fixed("launcher", RegionKind::Launcher, 0x1100, 512))
            .is_err());
    }
}
