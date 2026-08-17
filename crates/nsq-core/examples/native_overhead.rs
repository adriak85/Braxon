use std::collections::BTreeMap;
use std::time::Instant;

use nsq_core::{Charge, Dialect, NSQLever, NSQSlot, NativeNsqReflexor, NsqAddress, NsqInstruction};

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

fn main() {
    let count = 4096u64;
    let mut published = BTreeMap::new();
    for position in 1..=count {
        published.insert(address(position), slot(position));
    }
    let mut hardware = published.clone();
    hardware.insert(address(count), slot(count + 1));

    let full_start = Instant::now();
    let full: Vec<NsqInstruction> = published
        .iter()
        .map(|(address, value)| NsqInstruction::Set {
            address: address.clone(),
            value: value.clone(),
        })
        .collect();
    let full_elapsed = full_start.elapsed();

    let delta_start = Instant::now();
    let mut reflexor = NativeNsqReflexor::new();
    let dirty = vec![address(count)];
    let delta = reflexor.orbit_dirty(published, &hardware, &dirty);
    let delta_elapsed = delta_start.elapsed();

    println!("schema=nsq.native_overhead.v1");
    println!("resident_slots={count}");
    println!("full_operations={}", full.len());
    println!("delta_operations={}", delta.instructions.len());
    println!(
        "operation_reduction={}/{}",
        full.len().saturating_sub(delta.instructions.len()),
        full.len()
    );
    println!("full_plan_nanos={}", full_elapsed.as_nanos());
    println!("delta_plan_nanos={}", delta_elapsed.as_nanos());
    println!("hardware_cpu_claim=false");
}
