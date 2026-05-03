use nsq_compose::compose_repo_surface;
use std::env;

fn main() {
    let out = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: nsq-compose <out.nsq>");
        std::process::exit(2);
    });

    let sample = vec![
        "triple repo.core -> has -> nsq.source :layer 1 :plane 1 :anchor 10 :weight 1 :flags 0"
            .to_string(),
        "triple repo.core -> has -> nsq.compile :layer 1 :plane 1 :anchor 20 :weight 1 :flags 0"
            .to_string(),
        "triple repo.core -> has -> nsq.inspect :layer 1 :plane 1 :anchor 30 :weight 1 :flags 0"
            .to_string(),
    ];

    compose_repo_surface(&sample, &out).unwrap_or_else(|e| {
        eprintln!("compose error: {}", e);
        std::process::exit(2);
    });

    println!("composed={}", out);
}
