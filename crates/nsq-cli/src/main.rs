use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use nsq_core::{
    register_reconstructed_tool_intents, Charge, Dialect, NSQLever, NSQSlot, NsqAddress,
    NsqSyntaxTree, RawNsqEngine, RawNsqEvent, RawNsqOutcome,
};
use BRAXON_core::NativeNsqStack;

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
    match NsqSyntaxTree::parse(input) {
        Ok(tree) => {
            println!("source_len: {}", tree.source_len);
            println!("root: {:?}", tree.root);
        }
        Err(reason) => {
            eprintln!("parse error: {reason}");
            std::process::exit(2);
        }
    }
}

fn cmd_eval(input: &str) {
    println!("NSQ eval");
    println!("input: {}", input);
    let mut stack = match native_stack() {
        Ok(stack) => stack,
        Err(reason) => {
            eprintln!("eval setup error: {reason}");
            std::process::exit(2);
        }
    };
    let mut values = BTreeMap::new();
    values.insert("input".to_string(), input.to_string());
    let outcome = stack.dispatch_raw_intent(RawNsqEvent::Invoke {
        capability_id: "guile.rebuild_intent".into(),
        input: values,
    });
    match outcome {
        RawNsqOutcome::Accepted {
            capability_id,
            state,
        } => {
            println!("capability: {capability_id}");
            println!(
                "result: {}",
                state.get("result").map(String::as_str).unwrap_or("missing")
            );
            println!("state: {:?}", state);
        }
        RawNsqOutcome::Corrected {
            capability_id,
            state,
        } => {
            println!("capability: {capability_id}");
            println!("result: corrected");
            println!("state: {:?}", state);
        }
        RawNsqOutcome::Rejected { reason } => {
            eprintln!("eval rejected: {reason}");
            std::process::exit(2);
        }
    }
}

fn cmd_select(input: &str) {
    println!("NSQ select");
    println!("input: {}", input);
    let stack = match native_stack() {
        Ok(stack) => stack,
        Err(reason) => {
            eprintln!("select setup error: {reason}");
            std::process::exit(2);
        }
    };
    let capabilities = stack.discover_raw_capabilities(input);
    if capabilities.is_empty() {
        eprintln!("selection rejected: no native capability matches '{input}'");
        std::process::exit(2);
    }
    for capability in capabilities {
        println!("capability: {}", capability.capability_id);
        println!("native_entry: {}", capability.native_entry);
    }
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
    let path = Path::new(target);
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) => {
            eprintln!("fetch error: {error}");
            std::process::exit(2);
        }
    };
    println!("target: {}", target);
    println!("bytes: {}", metadata.len());
    println!(
        "kind: {}",
        if metadata.is_dir() {
            "directory"
        } else {
            "file"
        }
    );
    if metadata.is_file() {
        match fs::read_to_string(path) {
            Ok(contents) => println!(
                "content_preview: {}",
                contents.chars().take(160).collect::<String>()
            ),
            Err(error) => println!("content_preview: unavailable ({error})"),
        }
    }
}

fn native_stack() -> Result<NativeNsqStack, String> {
    let council = (1..=10).map(native_address).collect::<Vec<_>>();
    let target = native_address(20);
    let desired = NSQSlot::new(Dialect::Intent, vec![NSQLever::new(Charge::Positive, 21)?]);
    NativeNsqStack::new(council, target, desired, 1)
}

fn native_address(position: u64) -> NsqAddress {
    NsqAddress::root(NSQSlot::new(
        Dialect::Control,
        vec![NSQLever::new(Charge::Positive, position).expect("positive NSQ lever")],
    ))
}

#[allow(dead_code)]
fn native_engine() -> Result<RawNsqEngine, String> {
    let mut engine = RawNsqEngine::default();
    register_reconstructed_tool_intents(&mut engine)?;
    Ok(engine)
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
