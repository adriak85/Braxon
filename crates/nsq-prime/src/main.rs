use nsq_prime::prime_report;

fn main() {
    let root = std::env::current_dir().unwrap();
    let rep = prime_report(root.to_string_lossy().as_ref());
    println!("{}", serde_json::to_string_pretty(&rep).unwrap());
    if !rep.ok {
        std::process::exit(1);
    }
}
