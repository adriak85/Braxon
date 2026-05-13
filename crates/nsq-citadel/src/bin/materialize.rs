use std::path::Path;

fn main() {
    let root = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    match nsq_citadel::write_materialization(&root) {
        Ok(_) => {
            println!("Citadel699 materialization complete.");
            println!("Proof written to: state/nsq/proofs/citadel699_current_rebuild.json");
            println!("Council config written to: state/nsq/citadel699/current_rebuild/council_ten.materialization.json");
        }
        Err(e) => {
            eprintln!("Materialization failed: {}", e);
            std::process::exit(1);
        }
    }
}
