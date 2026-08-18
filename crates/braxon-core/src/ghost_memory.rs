use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const GHOST_MEMORY_SCHEMA: &str = "braxon.nsq.ghost_memory.v1";
pub const FIRING_WINDOW_BYTES: u64 = 15 * 1024 * 1024;
pub const DEFAULT_PAGE_BYTES: u64 = FIRING_WINDOW_BYTES;
pub const CPU_ADDRESS_SPACE_LIMIT: u64 = 1 << 48;
pub const VIRTUAL_EXTENSION_BASE: u64 = CPU_ADDRESS_SPACE_LIMIT;
pub const VIRTUAL_EXTENSION_LIMIT: u64 = 1 << 56;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WireKind {
    Parameter,
    Weight,
    Tokenizer,
    Launcher,
    Fact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageState {
    OnWire,
    Firing,
    Mapped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WirePage {
    pub page_id: String,
    pub region_id: String,
    pub kind: WireKind,
    pub wire_address: u64,
    pub byte_len: u64,
    pub address_domain: String,
    pub state: PageState,
    pub owner: String,
    pub cpu_aperture: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiringLease {
    pub lease_id: String,
    pub intent_id: String,
    pub page_id: String,
    pub cpu_aperture: u64,
    pub phase: PistonPhase,
    pub generation: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PistonPhase {
    Acquire,
    Hold,
    Commit,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FireDecision {
    Accepted,
    Deferred,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FireReport {
    pub schema: String,
    pub decision: FireDecision,
    pub reason: String,
    pub lease: Option<FiringLease>,
    pub wire_bytes: u64,
    pub active_cpu_bytes: u64,
    pub ordinary_storage_bytes: u64,
    pub physical_cpu_resources_touched: bool,
}

#[derive(Debug)]
pub struct GhostMemoryBus {
    pages: BTreeMap<String, WirePage>,
    leases: BTreeMap<String, FiringLease>,
    aperture_base: u64,
    aperture_bytes: u64,
    active_cpu_bytes: u64,
    generation: u64,
}

impl GhostMemoryBus {
    pub fn new(aperture_bytes: u64) -> Self {
        Self {
            pages: BTreeMap::new(),
            leases: BTreeMap::new(),
            aperture_base: 0x4000_0000,
            aperture_bytes: aperture_bytes.min(FIRING_WINDOW_BYTES),
            active_cpu_bytes: 0,
            generation: 0,
        }
    }

    pub fn map_wire_region(
        &mut self,
        region_id: &str,
        kind: WireKind,
        wire_base: u64,
        byte_len: u64,
        owner: &str,
    ) -> Result<usize, String> {
        if region_id.trim().is_empty() || owner.trim().is_empty() || byte_len == 0 {
            return Err("wire region identity, owner, and nonzero length are required".to_string());
        }
        let end = wire_base
            .checked_add(byte_len)
            .ok_or_else(|| "wire address overflow".to_string())?;
        if wire_base < VIRTUAL_EXTENSION_BASE || end > VIRTUAL_EXTENSION_LIMIT {
            return Err("wire mapping must remain inside the NSQ virtual extension, outside ordinary CPU addresses".to_string());
        }
        let pages = byte_len.div_ceil(DEFAULT_PAGE_BYTES);
        for index in 0..pages {
            let offset = index * DEFAULT_PAGE_BYTES;
            let len = (byte_len - offset).min(DEFAULT_PAGE_BYTES);
            let page = WirePage {
                page_id: format!("{region_id}.page-{index}"),
                region_id: region_id.to_string(),
                kind,
                wire_address: wire_base
                    .checked_add(offset)
                    .ok_or_else(|| "wire address overflow".to_string())?,
                byte_len: len,
                address_domain: "nsq_virtual_extension".to_string(),
                state: PageState::OnWire,
                owner: owner.to_string(),
                cpu_aperture: None,
            };
            if self.pages.insert(page.page_id.clone(), page).is_some() {
                return Err("duplicate wire page".to_string());
            }
        }
        Ok(pages as usize)
    }

    pub fn fire(&mut self, intent_id: &str, page_id: &str) -> FireReport {
        let Some(page) = self.pages.get(page_id).cloned() else {
            return self.report(FireDecision::Rejected, "unknown wire page", None);
        };
        if self
            .leases
            .values()
            .any(|lease| lease.phase != PistonPhase::Release)
        {
            return self.report(
                FireDecision::Deferred,
                "CPU aperture is held by another piston; release before reuse",
                None,
            );
        }
        if self.active_cpu_bytes.saturating_add(page.byte_len) > self.aperture_bytes {
            return self.report(
                FireDecision::Deferred,
                "CPU aperture budget exhausted; wait for piston release",
                None,
            );
        }
        self.generation += 1;
        let aperture = self.aperture_base;
        if let Some(mapped) = self.pages.get_mut(page_id) {
            mapped.state = PageState::Firing;
            mapped.cpu_aperture = Some(aperture);
        }
        self.active_cpu_bytes = self.active_cpu_bytes.saturating_add(page.byte_len);
        let lease = FiringLease {
            lease_id: format!("ghost-piston-{}-{}", self.generation, page_id),
            intent_id: intent_id.to_string(),
            page_id: page_id.to_string(),
            cpu_aperture: aperture,
            phase: PistonPhase::Acquire,
            generation: self.generation,
        };
        self.leases.insert(lease.lease_id.clone(), lease.clone());
        self.report(
            FireDecision::Accepted,
            "wire page fired into CPU aperture",
            Some(lease),
        )
    }

    pub fn advance(&mut self, lease_id: &str, phase: PistonPhase) -> Result<(), String> {
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| "unknown ghost-memory piston lease".to_string())?;
        lease.phase = phase;
        if phase == PistonPhase::Commit {
            if let Some(page) = self.pages.get_mut(&lease.page_id) {
                page.state = PageState::Mapped;
            }
        }
        if phase == PistonPhase::Release {
            let page_id = lease.page_id.clone();
            self.leases.remove(lease_id);
            if let Some(page) = self.pages.get_mut(&page_id) {
                self.active_cpu_bytes = self.active_cpu_bytes.saturating_sub(page.byte_len);
                page.state = PageState::OnWire;
                page.cpu_aperture = None;
            }
        }
        Ok(())
    }

    pub fn page(&self, page_id: &str) -> Option<&WirePage> {
        self.pages.get(page_id)
    }
    pub fn wire_bytes(&self) -> u64 {
        self.pages.values().map(|page| page.byte_len).sum()
    }
    pub fn active_cpu_bytes(&self) -> u64 {
        self.active_cpu_bytes
    }
    pub fn ordinary_storage_bytes(&self) -> u64 {
        0
    }

    fn report(
        &self,
        decision: FireDecision,
        reason: &str,
        lease: Option<FiringLease>,
    ) -> FireReport {
        FireReport {
            schema: GHOST_MEMORY_SCHEMA.to_string(),
            decision,
            reason: reason.to_string(),
            lease,
            wire_bytes: self.wire_bytes(),
            active_cpu_bytes: self.active_cpu_bytes,
            ordinary_storage_bytes: self.ordinary_storage_bytes(),
            physical_cpu_resources_touched: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_parameter_space_stays_on_wire_while_one_page_fires() {
        let mut bus = GhostMemoryBus::new(DEFAULT_PAGE_BYTES);
        assert_eq!(
            bus.map_wire_region(
                "weights",
                WireKind::Parameter,
                VIRTUAL_EXTENSION_BASE + 0x8000,
                DEFAULT_PAGE_BYTES * 3,
                "council"
            )
            .unwrap(),
            3
        );
        let report = bus.fire("intent-1", "weights.page-1");
        assert_eq!(report.decision, FireDecision::Accepted);
        assert_eq!(report.wire_bytes, DEFAULT_PAGE_BYTES * 3);
        assert_eq!(report.active_cpu_bytes, DEFAULT_PAGE_BYTES);
        assert_eq!(report.ordinary_storage_bytes, 0);
        assert_eq!(bus.page("weights.page-0").unwrap().state, PageState::OnWire);
        assert_eq!(bus.page("weights.page-1").unwrap().state, PageState::Firing);
        bus.advance(&report.lease.unwrap().lease_id, PistonPhase::Commit)
            .unwrap();
        assert_eq!(bus.page("weights.page-1").unwrap().state, PageState::Mapped);
    }

    #[test]
    fn piston_rotation_releases_cpu_aperture_back_to_wire() {
        let mut bus = GhostMemoryBus::new(DEFAULT_PAGE_BYTES);
        bus.map_wire_region(
            "weights",
            WireKind::Parameter,
            VIRTUAL_EXTENSION_BASE + 0x8000,
            DEFAULT_PAGE_BYTES * 2,
            "council",
        )
        .unwrap();
        let lease = bus.fire("intent-1", "weights.page-0").lease.unwrap();
        bus.advance(&lease.lease_id, PistonPhase::Commit).unwrap();
        bus.advance(&lease.lease_id, PistonPhase::Release).unwrap();
        assert_eq!(bus.active_cpu_bytes(), 0);
        assert_eq!(bus.page("weights.page-0").unwrap().state, PageState::OnWire);
        let next = bus.fire("intent-2", "weights.page-1");
        assert_eq!(next.decision, FireDecision::Accepted);
    }

    #[test]
    fn wire_page_content_cannot_be_overridden_while_firing() {
        let mut bus = GhostMemoryBus::new(DEFAULT_PAGE_BYTES * 2);
        bus.map_wire_region(
            "weights",
            WireKind::Weight,
            VIRTUAL_EXTENSION_BASE + 0x9000,
            DEFAULT_PAGE_BYTES,
            "council",
        )
        .unwrap();
        assert_eq!(
            bus.fire("intent-1", "weights.page-0").decision,
            FireDecision::Accepted
        );
        assert_eq!(
            bus.fire("intent-2", "weights.page-0").decision,
            FireDecision::Deferred
        );
    }

    #[test]
    fn ordinary_cpu_addresses_are_not_wire_extension_addresses() {
        let mut bus = GhostMemoryBus::new(DEFAULT_PAGE_BYTES);
        assert!(bus
            .map_wire_region(
                "bad",
                WireKind::Parameter,
                0xA000,
                DEFAULT_PAGE_BYTES,
                "council"
            )
            .is_err());
    }

    #[test]
    fn aperture_pressure_is_fail_closed() {
        let mut bus = GhostMemoryBus::new(DEFAULT_PAGE_BYTES);
        bus.map_wire_region(
            "weights",
            WireKind::Parameter,
            VIRTUAL_EXTENSION_BASE + 0xA000,
            DEFAULT_PAGE_BYTES * 2,
            "council",
        )
        .unwrap();
        assert_eq!(
            bus.fire("intent-1", "weights.page-0").decision,
            FireDecision::Accepted
        );
        assert_eq!(
            bus.fire("intent-2", "weights.page-1").decision,
            FireDecision::Deferred
        );
    }

    #[test]
    fn firing_report_never_claims_physical_cpu_control() {
        let mut bus = GhostMemoryBus::new(DEFAULT_PAGE_BYTES);
        bus.map_wire_region(
            "weights",
            WireKind::Parameter,
            VIRTUAL_EXTENSION_BASE + 0xB000,
            DEFAULT_PAGE_BYTES,
            "council",
        )
        .unwrap();
        let report = bus.fire("intent-1", "weights.page-0");
        assert!(!report.physical_cpu_resources_touched);
    }
}
