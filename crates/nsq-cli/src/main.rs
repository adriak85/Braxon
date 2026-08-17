use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "nsq")]
#[command(version)]
#[command(about = "NSQ command surface")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Status,
    Repl,
    Parse { input: String },
    Eval { input: String },
    Select { input: String },
    Ingest { path: String },
    Fetch { target: String },
    Wake,
    Doctor,
}

fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Repl) {
        Command::Status => cmd_status(),
        Command::Repl => cmd_repl(),
        Command::Parse { input } => cmd_parse(&input),
        Command::Eval { input } => cmd_eval(&input),
        Command::Select { input } => cmd_select(&input),
        Command::Ingest { path } => cmd_ingest(&path),
        Command::Fetch { target } => cmd_fetch(&target),
        Command::Wake => cmd_wake(),
        Command::Doctor => cmd_doctor(),
    }
}

fn cmd_status() {
    println!("NSQ status: ready");
    println!("workspace: Braxon");
    println!("mode: prompt-chain front door");
}

fn cmd_parse(input: &str) {
    println!("NSQ parse");
    println!("input: {}", input);
    println!("tokens: {:?}", input.split_whitespace().collect::<Vec<_>>());
}

fn cmd_eval(input: &str) {
    println!("NSQ eval");
    println!("input: {}", input);
    println!("result: stub-ok");
}

fn cmd_select(input: &str) {
    println!("NSQ select");
    println!("input: {}", input);
    println!("selection: stub-ok");
}

fn cmd_ingest(path: &str) {
    let p = Path::new(path);
    if !p.exists() {
        eprintln!("ingest error: path does not exist: {}", path);
        std::process::exit(2);
    }

    match fs::metadata(p) {
        Ok(meta) => {
            if meta.is_dir() {
                println!("NSQ ingest");
                println!("path: {}", path);
                println!("type: directory");
            } else {
                println!("NSQ ingest");
                println!("path: {}", path);
                println!("type: file");
                println!("boundary_carrier_units: {}", meta.len());
            }
        }
        Err(e) => {
            eprintln!("ingest error: {}", e);
            std::process::exit(2);
        }
    }
}

fn cmd_fetch(target: &str) {
    println!("NSQ fetch");
    println!("target: {}", target);
    println!("result: stub-ok");
}

fn cmd_wake() {
    println!("NSQ wake");
    println!("state: active");
}

fn cmd_doctor() {
    println!("NSQ doctor");
    println!("cwd: {}", std::env::current_dir().unwrap().display());

    let cargo_toml = Path::new("Cargo.toml").exists();
    let braxon_core = Path::new("crates/braxon-core").exists();
    let nsq_cli = Path::new("crates/nsq-cli").exists();

    println!("check:cargo_toml={}", cargo_toml);
    println!("check:BRAXON_core={}", braxon_core);
    println!("check:nsq_cli={}", nsq_cli);

    match std::process::Command::new("rustc")
        .arg("--version")
        .output()
    {
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout);
            println!("rustc: {}", s.trim());
        }
        Err(e) => println!("rustc: unavailable ({})", e),
    }

    match std::process::Command::new("cargo")
        .arg("--version")
        .output()
    {
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout);
            println!("cargo: {}", s.trim());
        }
        Err(e) => println!("cargo: unavailable ({})", e),
    }

    let termux_prefix = std::env::var("PREFIX").unwrap_or_else(|_| "<unset>".to_string());
    println!("prefix: {}", termux_prefix);
}

fn cmd_repl() {
    println!("NSQ interactive prompt");
    println!("type: status | parse <text> | eval <text> | select <text> | ingest <path> | fetch <target> | wake | doctor | quit");

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("nsq> ");
        io::stdout().flush().unwrap();

        line.clear();
        let read = stdin.read_line(&mut line).unwrap_or(0);
        if read == 0 {
            println!();
            break;
        }

        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        if matches!(input, "quit" | "exit") {
            break;
        }

        if input == "status" {
            cmd_status();
            continue;
        }
        if input == "wake" {
            cmd_wake();
            continue;
        }
        if input == "doctor" {
            cmd_doctor();
            continue;
        }

        if let Some(rest) = input.strip_prefix("parse ") {
            cmd_parse(rest.trim());
            continue;
        }
        if let Some(rest) = input.strip_prefix("eval ") {
            cmd_eval(rest.trim());
            continue;
        }
        if let Some(rest) = input.strip_prefix("select ") {
            cmd_select(rest.trim());
            continue;
        }
        if let Some(rest) = input.strip_prefix("ingest ") {
            cmd_ingest(rest.trim());
            continue;
        }
        if let Some(rest) = input.strip_prefix("fetch ") {
            cmd_fetch(rest.trim());
            continue;
        }

        println!("unknown command: {}", input);
    }
}
