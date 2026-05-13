use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        eprintln!("Usage: nsq-speak <text>");
        std::process::exit(1);
    };

    let coaching = nsq_citadel::load_coaching_mode(
        &std::env::current_dir().unwrap_or_default()
    );
    let bus = nsq_citadel::CitadelBus::new(coaching);
    let reply = bus.route(&input);

    println!("{}", reply.summary());
    for msg in reply.board_messages {
        println!(
            "[C{} L{} {}] slots={} pressure={} live={}",
            msg.capital_id,
            msg.pole_lane,
            msg.pole_id,
            msg.slot_count,
            msg.pressure_sum,
            msg.is_live
        );
    }
}
