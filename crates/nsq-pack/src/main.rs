use nsq_pack::pack_files;
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let out = args.next().unwrap_or_else(|| {
        eprintln!("usage: nsq-pack <out.pack> <input1> [input2 ...]");
        std::process::exit(2);
    });
    let inputs: Vec<String> = args.collect();
    if inputs.is_empty() {
        eprintln!("usage: nsq-pack <out.pack> <input1> [input2 ...]");
        std::process::exit(2);
    }

    let manifest = pack_files(&inputs, &out).unwrap_or_else(|e| {
        eprintln!("pack error: {}", e);
        std::process::exit(2);
    });

    println!("{}", serde_json::to_string_pretty(&manifest).unwrap());
}
