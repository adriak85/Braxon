use nsq_inspect::inspect_file;
use std::env;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: nsq-inspect <artifact.nsqb>");
        std::process::exit(2);
    });

    let rep = inspect_file(&path).unwrap_or_else(|e| {
        eprintln!("inspect error: {}", e);
        std::process::exit(2);
    });

    println!("{}", serde_json::to_string_pretty(&rep).unwrap());
}
