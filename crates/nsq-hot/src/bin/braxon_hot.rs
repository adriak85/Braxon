use nsq_hot::write_hot_state;
use std::env;
use std::path::PathBuf;

fn main() {
    let root = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());

    let out = env::args()
        .nth(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("state/braxon/hot/nsq_hot_state.json"));

    let state = write_hot_state(&root, &out).unwrap();

    println!("seed_count={}", state.seed_count);
    println!("seed_digest_chain={}", state.seed_digest_chain);
    println!("alphabet_reconstructed={}", state.alphabet_reconstructed);
    println!("intent_language_reconstructed={}", state.intent_language_reconstructed);
    println!("parameter_address_space_total={}", state.parameter_address_space_total);
    println!("positions_per_lever={}", state.positions_per_lever);
    println!("levers_per_unit={}", state.levers_per_unit);
    println!("states_per_unit_decimal={}", state.states_per_unit_decimal);
    println!("inserted_lane_count={}", state.inserted_lane_count);
    println!("hydrated_lane_count={}", state.hydrated_lane_count);
    println!("wake_framework_count={}", state.wake_framework_count);
    println!("hot_hot_hot={}", state.hot_hot_hot);
    println!("report={}", out.display());

    if !state.hot_hot_hot {
        std::process::exit(1);
    }
}
