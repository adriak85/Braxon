use nsq_citadel::{CitadelBus, CoachingMode, IntentSeed};

fn main() {
    let input = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let input = if input.trim().is_empty() {
        "What has not been seen yet?"
    } else {
        input.as_str()
    };
    let seed = IntentSeed::new("braxon.citadel.seed.v1", input);
    let materialized = seed.materialize(0, 8);
    let reply = CitadelBus::new(CoachingMode::Balanced).route(input);
    println!("intent={}", input);
    println!("input_slots={}", reply.input_slot_count);
    println!(
        "capitals={} poles={}",
        reply.capital_count, reply.pole_count
    );
    println!(
        "lead_pole={} priority={}",
        reply.lead_pole, reply.lead_priority
    );
    println!("pressure={}", reply.total_pressure);
    println!(
        "logical_complete={} resident_slots={}",
        materialized.logical_complete,
        materialized.slots.len()
    );
}
