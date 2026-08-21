//! Bounded virtual-addressed live-bus bootstrap for Braxon front doors.
//!
//! This module intentionally provides *circulation*, not a permanent resident
//! model process. Every front-door turn can establish the same addressable NSQ
//! wire map for the tokenizer, Parameter-Citadel state, canonical Council Ten
//! seed, and all ten configured seed-body descriptors. Each page is acquired,
//! committed into the bounded CPU aperture, and released back to the virtual
//! wire before the operation returns. Learned model weights are not represented
//! as present or executable unless an independent capability proves them.

use crate::ghost_memory::PistonPhase as GhostPistonPhase;
use crate::{
    assess_donor_model_readiness, execute_canonical_parameter_citadel_cycle,
    DonorModelReadinessReport, FireDecision, GhostMemoryBus, PageState, TokenizerBridge,
    TokenizerBridgeReceipt, WireKind, DEFAULT_PAGE_BYTES, VIRTUAL_EXTENSION_BASE,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const LIVE_BUS_BOOTSTRAP_SCHEMA: &str = "braxon.nsq.live_bus_bootstrap.v1";
pub const LIVE_BUS_CAPABILITY: &str = "feature:live_bus.bootstrap";
const VIRTUAL_REGION_STRIDE: u64 = DEFAULT_PAGE_BYTES;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LiveBusWindow {
    pub logical_id: String,
    pub owner: String,
    pub kind: String,
    pub virtual_address: String,
    pub page_id: String,
    pub byte_len: u64,
    pub wire_resident_for_operation: bool,
    pub fired: bool,
    pub committed: bool,
    pub released: bool,
    pub state_after_release: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveBusBootstrapReport {
    pub schema: String,
    pub capability: String,
    pub intent: String,
    pub tokenizer: TokenizerBridgeReceipt,
    pub parameter_generation: u64,
    pub parameter_invariants_passed: bool,
    pub council_seed_readiness: DonorModelReadinessReport,
    pub virtual_address_domain: String,
    pub virtual_window_total: usize,
    pub virtual_wire_bytes: u64,
    pub circulation_cycle_total: usize,
    pub all_windows_resolved: bool,
    pub all_windows_released: bool,
    pub active_cpu_bytes_after_release: u64,
    pub ordinary_storage_bytes: u64,
    pub model_weight_execution_claimed: bool,
    pub resident_runtime_constructed: bool,
    pub windows: Vec<LiveBusWindow>,
    pub exact_next_operation: String,
}

/// Build a fresh, addressable live-bus window for one explicit front-door
/// operation. The returned report proves both address resolution and bounded
/// release; the GhostMemoryBus is intentionally dropped after this call so the
/// system never represents virtual metadata as a persistent model runtime.
pub fn bootstrap_live_bus(
    start: impl AsRef<Path>,
    intent: impl AsRef<str>,
) -> Result<LiveBusBootstrapReport, String> {
    let root = resolve_root(start.as_ref())?;
    let intent = intent.as_ref().trim();
    if intent.is_empty() {
        return Err("live bus bootstrap requires a nonempty front-door intent".into());
    }

    let tokenizer =
        TokenizerBridge::from_root(&root, "braxon_native")?.encode_translate_round_trip(intent);
    if !tokenizer.all_required_mappings_resolved() {
        return Err(format!(
            "live bus tokenizer bootstrap failed: unresolved_tokens={} collisions={}",
            tokenizer.unresolved_tokens.join(","),
            tokenizer.collision_count
        ));
    }
    let signal = i64::try_from(tokenizer.projections.len())
        .map_err(|_| "tokenizer projection count exceeds parameter signal range")?;
    let context = i64::try_from(intent.chars().count())
        .map_err(|_| "front-door intent length exceeds parameter context range")?;
    let parameter = execute_canonical_parameter_citadel_cycle(signal, context)?;
    if !parameter.invariants.all_pass() {
        return Err("live bus Parameter-Citadel bootstrap failed its invariants".into());
    }
    let donor = assess_donor_model_readiness(&root)?;
    if !donor.complete_ten_body_window_proven
        || !donor.donor_parameter_synchronization_live
        || donor.materialized_body_total != 10
        || donor.nsq_fire_instruction_total != 10
        || donor.nsq_release_instruction_total != 10
    {
        return Err(format!(
            "live bus Council Ten bootstrap is incomplete: bodies={} fires={} releases={} synchronized={}",
            donor.materialized_body_total,
            donor.nsq_fire_instruction_total,
            donor.nsq_release_instruction_total,
            donor.donor_parameter_synchronization_live
        ));
    }

    let tokenizer_bytes = file_bytes(&root.join(&tokenizer.tokenizer_path))?;
    let parameter_bytes = parameter
        .citadel_materialization
        .bodies
        .iter()
        .map(|body| {
            body.shape
                .iter()
                .copied()
                .product::<u64>()
                .saturating_mul(4)
        })
        .sum::<u64>()
        .max(1);
    let seed_descriptor_bytes = u64::try_from(
        donor.seed_id.len() + donor.seed_hash.len() + donor.authoritative_seed_contract_path.len(),
    )
    .map_err(|_| "Council Ten seed descriptor length exceeds u64")?
    .max(1);

    let mut definitions = vec![
        WindowDefinition::new("tokenizer.native", WireKind::Tokenizer, tokenizer_bytes),
        WindowDefinition::new("parameter.citadel", WireKind::Parameter, parameter_bytes),
        WindowDefinition::new(
            "council-ten.seed",
            WireKind::Parameter,
            seed_descriptor_bytes,
        ),
    ];
    for band in &donor.bands {
        let bytes = u64::try_from(
            band.model_id.len()
                + band.assigned_pole.len()
                + band.materialized_pole.len()
                + band.role.len(),
        )
        .map_err(|_| "Council Ten seed-body descriptor length exceeds u64")?
        .max(1);
        definitions.push(WindowDefinition::new(
            format!("council-ten.seed-body.{}", band.materialized_pole),
            WireKind::Parameter,
            bytes,
        ));
    }
    if definitions.len() != 13 {
        return Err(format!(
            "live bus requires tokenizer, Parameter-Citadel, Council seed, and ten seed-body descriptors; found {}",
            definitions.len()
        ));
    }

    let mut ghost = GhostMemoryBus::new(DEFAULT_PAGE_BYTES);
    let mut windows = Vec::with_capacity(definitions.len());
    for (index, definition) in definitions.iter().enumerate() {
        let index = u64::try_from(index).map_err(|_| "virtual window index exceeds u64")?;
        let virtual_address = VIRTUAL_EXTENSION_BASE
            .checked_add(index.saturating_mul(VIRTUAL_REGION_STRIDE))
            .ok_or("live bus virtual address overflow")?;
        let owner = format!("{}::{}", LIVE_BUS_CAPABILITY, definition.logical_id);
        let mapped = ghost.map_wire_region(
            &definition.logical_id,
            definition.kind,
            virtual_address,
            definition.byte_len,
            &owner,
        )?;
        if mapped != 1 {
            return Err(format!(
                "live bus descriptor '{}' unexpectedly spans {mapped} pages; split its descriptor before bootstrap",
                definition.logical_id
            ));
        }
        let page_id = format!("{}.page-0", definition.logical_id);
        let fired = ghost.fire(intent, &page_id);
        if fired.decision != FireDecision::Accepted {
            return Err(format!(
                "live bus virtual address '{}' could not fire: {}",
                definition.logical_id, fired.reason
            ));
        }
        let lease = fired
            .lease
            .ok_or("accepted live bus firing omitted its piston lease")?;
        ghost.advance(&lease.lease_id, GhostPistonPhase::Commit)?;
        let committed = ghost
            .page(&page_id)
            .map(|page| page.state == PageState::Mapped && page.cpu_aperture.is_some())
            .unwrap_or(false);
        if !committed {
            return Err(format!(
                "live bus virtual address '{}' did not resolve into the bounded CPU aperture",
                definition.logical_id
            ));
        }
        ghost.advance(&lease.lease_id, GhostPistonPhase::Release)?;
        let released = ghost
            .page(&page_id)
            .map(|page| page.state == PageState::OnWire && page.cpu_aperture.is_none())
            .unwrap_or(false);
        if !released {
            return Err(format!(
                "live bus virtual address '{}' did not return to the virtual wire after release",
                definition.logical_id
            ));
        }
        windows.push(LiveBusWindow {
            logical_id: definition.logical_id.clone(),
            owner,
            kind: wire_kind_name(definition.kind).to_string(),
            virtual_address: format!("nsq-virtual://{virtual_address:016x}"),
            page_id,
            byte_len: definition.byte_len,
            wire_resident_for_operation: true,
            fired: true,
            committed,
            released,
            state_after_release: "on_wire_virtual_address_resolvable_for_next_piston_cycle".into(),
        });
    }

    let all_windows_resolved = windows
        .iter()
        .all(|window| window.fired && window.committed);
    let all_windows_released =
        windows.iter().all(|window| window.released) && ghost.active_cpu_bytes() == 0;
    if !all_windows_resolved || !all_windows_released {
        return Err(
            "live bus did not complete its virtual address resolution and release cycle".into(),
        );
    }

    Ok(LiveBusBootstrapReport {
        schema: LIVE_BUS_BOOTSTRAP_SCHEMA.into(),
        capability: LIVE_BUS_CAPABILITY.into(),
        intent: intent.into(),
        tokenizer,
        parameter_generation: parameter.generation,
        parameter_invariants_passed: parameter.invariants.all_pass(),
        council_seed_readiness: donor,
        virtual_address_domain: "nsq_virtual_extension_outside_ordinary_cpu_addresses".into(),
        virtual_window_total: windows.len(),
        virtual_wire_bytes: ghost.wire_bytes(),
        circulation_cycle_total: windows.len(),
        all_windows_resolved,
        all_windows_released,
        active_cpu_bytes_after_release: ghost.active_cpu_bytes(),
        ordinary_storage_bytes: ghost.ordinary_storage_bytes(),
        model_weight_execution_claimed: false,
        resident_runtime_constructed: false,
        windows,
        exact_next_operation: "The virtual wire map is proven for this front-door operation. A later operation creates a fresh bounded Piston/Ghost circulation cycle; no learned model-weight residency is claimed.".into(),
    })
}

#[derive(Debug, Clone)]
struct WindowDefinition {
    logical_id: String,
    kind: WireKind,
    byte_len: u64,
}

impl WindowDefinition {
    fn new(logical_id: impl Into<String>, kind: WireKind, byte_len: u64) -> Self {
        Self {
            logical_id: logical_id.into(),
            kind,
            byte_len,
        }
    }
}

fn file_bytes(path: &Path) -> Result<u64, String> {
    fs::metadata(path)
        .map_err(|error| format!("live bus cannot stat '{}': {error}", path.display()))
        .map(|metadata| metadata.len())
        .and_then(|bytes| {
            if bytes == 0 {
                Err(format!(
                    "live bus cannot map empty artifact '{}",
                    path.display()
                ))
            } else {
                Ok(bytes)
            }
        })
}

fn wire_kind_name(kind: WireKind) -> &'static str {
    match kind {
        WireKind::Parameter => "parameter",
        WireKind::Weight => "weight",
        WireKind::Tokenizer => "tokenizer",
        WireKind::Launcher => "launcher",
        WireKind::Fact => "fact",
    }
}

fn resolve_root(start: &Path) -> Result<PathBuf, String> {
    let canonical = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    canonical
        .ancestors()
        .find(|candidate| {
            candidate
                .join("config/nsq/braxon_council_ten_stack.json")
                .exists()
        })
        .map(Path::to_path_buf)
        .ok_or_else(|| "unable to resolve the Braxon repository root for live bus bootstrap".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_bus_bootstrap_maps_resolves_and_releases_every_required_window() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let report = bootstrap_live_bus(root, "verify live bus addressability").unwrap();
        assert_eq!(report.virtual_window_total, 13);
        assert_eq!(report.circulation_cycle_total, 13);
        assert!(report.all_windows_resolved);
        assert!(report.all_windows_released);
        assert_eq!(report.active_cpu_bytes_after_release, 0);
        assert!(report.windows.iter().all(|window| {
            window.virtual_address.starts_with("nsq-virtual://")
                && window.wire_resident_for_operation
                && window.fired
                && window.committed
                && window.released
        }));
        assert!(!report.model_weight_execution_claimed);
        assert!(!report.resident_runtime_constructed);
    }
}
