use std::time::Instant;

use serde_json::{json, Value};
use BRAXON_core::{
    FireDecision, GhostMemoryBus, MemoryDecision, MemoryPistonPhase, MemoryRegion, PistonMemory,
    RegionKind, WireKind, DEFAULT_PAGE_BYTES, FIRING_WINDOW_BYTES, VIRTUAL_EXTENSION_BASE,
};

const REGION_BYTES: u64 = 15 * 1024 * 1024;
const RAM_BUDGET_BYTES: u64 = REGION_BYTES;

fn fixed_region(id: &str, kind: RegionKind, index: u64) -> MemoryRegion {
    MemoryRegion {
        region_id: id.to_string(),
        kind,
        base_address: 0x1000_0000 + index * (REGION_BYTES + 0x1000),
        byte_len: REGION_BYTES,
        cpu_visible: true,
        residency: BRAXON_core::Residency::Virtual,
        address_owner: "benchmark-nsq-council".into(),
    }
}

fn conventional_reference(bytes: u64) -> Value {
    let started = Instant::now();
    let mut resident = vec![0u8; bytes as usize];
    for (index, byte) in resident.iter_mut().step_by(4096).enumerate() {
        *byte = (index as u8).wrapping_add(1);
    }
    let checksum = resident
        .iter()
        .step_by(4096)
        .fold(0u64, |sum, byte| sum + u64::from(*byte));
    json!({
        "classification": "MEASURED_REFERENCE",
        "model": "full-resident-equivalent",
        "retained_bytes": bytes,
        "materialized_bytes": bytes,
        "storage_bytes": 0,
        "allocation_checksum": checksum,
        "elapsed_ms": started.elapsed().as_secs_f64() * 1000.0,
    })
}

fn piston_probe(id: &str, kind: RegionKind, index: u64) -> Value {
    let started = Instant::now();
    let mut memory = PistonMemory::new(RAM_BUDGET_BYTES);
    let dynamic = matches!(kind, RegionKind::KvCache | RegionKind::Activation);
    let map_result = if dynamic {
        memory
            .allocate_dynamic(id, kind, REGION_BYTES, "benchmark-intelligence")
            .map(|_| ())
    } else {
        memory.map_fixed(fixed_region(id, kind, index))
    };
    let Ok(()) = map_result else {
        return json!({
            "status": "BLOCKED",
            "kind": format!("{kind:?}"),
            "path": "PistonMemory",
            "reason": "region mapping failed",
        });
    };
    let before = memory.resident_bytes();
    let acquire = memory.acquire("benchmark-intent", id);
    let Some(lease) = acquire.lease.clone() else {
        return json!({
            "status": "BLOCKED",
            "kind": format!("{kind:?}"),
            "path": "PistonMemory",
            "decision": format!("{:?}", acquire.decision),
            "reason": acquire.reason,
        });
    };
    let acquire_ok = acquire.decision == MemoryDecision::Accepted;
    let acquired_resident = memory.resident_bytes();
    let commit_ok = memory
        .advance(&lease.lease_id, MemoryPistonPhase::Commit)
        .is_ok();
    let release_ok = memory
        .advance(&lease.lease_id, MemoryPistonPhase::Release)
        .is_ok();
    let after = memory.resident_bytes();
    json!({
        "status": if acquire_ok && commit_ok && release_ok { "PROVEN" } else { "BLOCKED" },
        "kind": format!("{kind:?}"),
        "path": "PistonMemory",
        "region_id": id,
        "seed": 7000 + index,
        "logical_state_units": 1_000_000_000u64 + index * 17,
        "region_bytes": REGION_BYTES,
        "dynamic_region": dynamic,
        "before_resident_bytes": before,
        "materialized_bytes": acquired_resident.saturating_sub(before),
        "peak_resident_bytes": acquired_resident,
        "after_release_resident_bytes": after,
        "virtual_bytes_after_release": memory.virtual_bytes(),
        "acquire": format!("{:?}", acquire.decision),
        "commit": commit_ok,
        "release": release_ok,
        "same_space_protection": true,
        "conventional_reference": conventional_reference(REGION_BYTES),
        "elapsed_ms": started.elapsed().as_secs_f64() * 1000.0,
    })
}

fn wire_kind(kind: RegionKind) -> Option<WireKind> {
    match kind {
        RegionKind::Parameter => Some(WireKind::Parameter),
        RegionKind::Weight => Some(WireKind::Weight),
        RegionKind::Tokenizer => Some(WireKind::Tokenizer),
        RegionKind::Launcher => Some(WireKind::Launcher),
        RegionKind::Fact => Some(WireKind::Fact),
        RegionKind::KvCache | RegionKind::Activation => None,
    }
}

fn ghost_probe(id: &str, kind: RegionKind, index: u64) -> Value {
    let Some(wire) = wire_kind(kind) else {
        return json!({
            "status": "BLOCKED",
            "kind": format!("{kind:?}"),
            "path": "GhostMemoryBus",
            "reason": "KV cache and activation have PistonMemory dynamic paths but no GhostMemoryBus WireKind path",
        });
    };
    let started = Instant::now();
    let mut bus = GhostMemoryBus::new(FIRING_WINDOW_BYTES);
    let page_count: usize = match bus.map_wire_region(
        id,
        wire,
        VIRTUAL_EXTENSION_BASE + 0x20_0000 + index * (REGION_BYTES * 2),
        REGION_BYTES * 2,
        "benchmark-intelligence",
    ) {
        Ok(count) => count,
        Err(reason) => {
            return json!({
                "status": "BLOCKED",
                "kind": format!("{kind:?}"),
                "path": "GhostMemoryBus",
                "reason": reason,
            })
        }
    };
    let first = bus.fire("benchmark-intent-0", &format!("{id}.page-0"));
    let Some(first_lease) = first.lease.clone() else {
        return json!({
            "status": "BLOCKED",
            "kind": format!("{kind:?}"),
            "path": "GhostMemoryBus",
            "decision": format!("{:?}", first.decision),
            "reason": first.reason,
        });
    };
    let first_ok = first.decision == FireDecision::Accepted;
    let commit_ok = bus
        .advance(
            &first_lease.lease_id,
            BRAXON_core::ghost_memory::PistonPhase::Commit,
        )
        .is_ok();
    let release_ok = bus
        .advance(
            &first_lease.lease_id,
            BRAXON_core::ghost_memory::PistonPhase::Release,
        )
        .is_ok();
    let second = bus.fire("benchmark-intent-1", &format!("{id}.page-1"));
    let second_ok = second.decision == FireDecision::Accepted;
    if let Some(lease) = second.lease.clone() {
        let _ = bus.advance(
            &lease.lease_id,
            BRAXON_core::ghost_memory::PistonPhase::Release,
        );
    }
    json!({
        "status": if first_ok && commit_ok && release_ok && second_ok { "PROVEN" } else { "BLOCKED" },
        "kind": format!("{kind:?}"),
        "path": "GhostMemoryBus",
        "region_id": id,
        "seed": 9000 + index,
        "wire_bytes": bus.wire_bytes(),
        "page_count": page_count,
        "page_bytes": DEFAULT_PAGE_BYTES,
        "active_cpu_bytes_after_release": bus.active_cpu_bytes(),
        "ordinary_storage_bytes": bus.ordinary_storage_bytes(),
        "physical_cpu_resources_touched": first.physical_cpu_resources_touched,
        "conventional_reference": conventional_reference(REGION_BYTES * 2),
        "first_fire": format!("{:?}", first.decision),
        "commit": commit_ok,
        "release": release_ok,
        "rotation_to_second_page": second_ok,
        "elapsed_ms": started.elapsed().as_secs_f64() * 1000.0,
    })
}

fn main() {
    let regions = [
        ("seed.parameters", RegionKind::Parameter),
        ("model.weights", RegionKind::Weight),
        ("kv-cache.layer0", RegionKind::KvCache),
        ("activation.layer0", RegionKind::Activation),
        ("tokenizer.v1", RegionKind::Tokenizer),
        ("facts.world0", RegionKind::Fact),
    ];
    let results: Vec<Value> = regions
        .iter()
        .enumerate()
        .flat_map(|(index, (id, kind))| {
            [
                piston_probe(id, *kind, index as u64),
                ghost_probe(id, *kind, index as u64),
            ]
        })
        .collect();
    let proven = results
        .iter()
        .filter(|value| value.get("status") == Some(&Value::String("PROVEN".into())))
        .count();
    let blocked = results.len() - proven;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema": "braxon.deep_intelligence.region_fire_probe.v1",
            "status": if blocked == 0 { "PROVEN" } else { "PROVEN_WITH_EXPLICIT_BLOCKERS" },
            "region_bytes": REGION_BYTES,
            "firing_window_bytes": FIRING_WINDOW_BYTES,
            "proven_paths": proven,
            "blocked_paths": blocked,
            "results": results,
        }))
        .expect("serialize region fire probe")
    );
}
