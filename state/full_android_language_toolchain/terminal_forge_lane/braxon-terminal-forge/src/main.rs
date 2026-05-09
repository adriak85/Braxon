use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const STRATEGIES: [&str; 10] = [
    "current_config_path",
    "tool_config_path",
    "pkg_config_path",
    "overlay_include_path",
    "adoption_include_path",
    "dereferenced_integrated_prefix",
    "copied_native_header_prefix",
    "patched_sysconfig_or_metadata",
    "env_override_flags",
    "generated_config_shim",
];

const TOOLS: [&str; 42] = [
    "braxon-python", "braxon-rustc", "braxon-cargo",
    "python3", "perl", "ruby", "node", "npm", "go", "lua",
    "rustc", "cargo", "clang", "clang++", "ld.lld", "llvm-ar", "llvm-ranlib",
    "zig", "zls", "tree-sitter",
    "java", "javac", "gradle", "kotlinc", "kotlin",
    "aapt", "apksigner", "dx", "zipalign",
    "cmake", "ninja", "make", "pkg-config",
    "git", "curl", "jq", "rg", "fd", "file",
    "proot", "fakeroot", "tsu",
];

#[derive(Debug)]
struct Probe {
    name: String,
    found: bool,
    path: String,
    version: String,
}

fn run(args: &[&str]) -> String {
    if args.is_empty() {
        return String::new();
    }
    let mut cmd = Command::new(args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    match cmd.output() {
        Ok(out) => {
            let mut s = String::new();
            s.push_str(&String::from_utf8_lossy(&out.stdout));
            s.push_str(&String::from_utf8_lossy(&out.stderr));
            s.trim().to_string()
        }
        Err(e) => format!("ERR: {e}"),
    }
}

fn which(name: &str) -> Option<String> {
    let out = run(&["sh", "-c", &format!("command -v {name}")]);
    if out.is_empty() || out.starts_with("ERR:") {
        None
    } else {
        Some(out.lines().next().unwrap_or("").to_string())
    }
}

fn safe_version(name: &str) -> String {
    match name {
        "braxon-python" | "python3" => run(&[name, "--version"]),
        "braxon-rustc" | "rustc" => run(&[name, "--version"]),
        "braxon-cargo" | "cargo" => run(&[name, "--version"]),
        "perl" => run(&["perl", "-v"]),
        "ruby" => run(&["ruby", "--version"]),
        "node" => run(&["node", "--version"]),
        "npm" => run(&["npm", "--version"]),
        "go" => run(&["go", "version"]),
        "lua" => run(&["lua", "-v"]),
        "clang" | "clang++" => run(&[name, "--version"]),
        "zig" => run(&["zig", "version"]),
        "zls" => run(&["zls", "--version"]),
        "tree-sitter" => run(&["tree-sitter", "--version"]),
        "java" => run(&["java", "-version"]),
        "javac" => run(&["javac", "-version"]),
        "gradle" => run(&["gradle", "--version"]),
        "kotlinc" => run(&["kotlinc", "-version"]),
        "kotlin" => run(&["kotlin", "-version"]),
        "aapt" => run(&["aapt", "version"]),
        "apksigner" => run(&["apksigner", "--version"]),
        "dx" => run(&["dx", "--version"]),
        "zipalign" => run(&["sh", "-c", "zipalign -h | head -n 5"]),
        "pkg-config" => run(&["pkg-config", "--version"]),
        "cmake" => run(&["cmake", "--version"]),
        "ninja" => run(&["ninja", "--version"]),
        "make" => run(&["make", "--version"]),
        "rg" => run(&["rg", "--version"]),
        "fd" => run(&["fd", "--version"]),
        "jq" => run(&["jq", "--version"]),
        "proot" => run(&["proot", "--version"]),
        "fakeroot" => run(&["fakeroot", "--version"]),
        "tsu" => run(&["tsu", "-v"]),
        _ => String::new(),
    }
}

fn write_report(path: &Path, probes: &[Probe]) -> io::Result<()> {
    let mut f = File::create(path)?;
    writeln!(f, "BRAXON TERMINAL FORGE REPORT")?;
    writeln!(f, "jobs: {}", env::var("JOBS").unwrap_or_else(|_| "7".to_string()))?;
    writeln!(f)?;
    writeln!(f, "resolver strategies:")?;
    for (i, s) in STRATEGIES.iter().enumerate() {
        writeln!(f, "{:02}. {}", i + 1, s)?;
    }
    writeln!(f)?;
    writeln!(f, "tool surface:")?;
    for p in probes {
        writeln!(f, "=== {} ===", p.name)?;
        writeln!(f, "found: {}", p.found)?;
        writeln!(f, "path: {}", p.path)?;
        writeln!(f, "version: {}", p.version.lines().take(6).collect::<Vec<_>>().join(" | "))?;
        writeln!(f)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let outdir = env::var("BRAXON_TERMINAL_FORGE_OUT")
        .unwrap_or_else(|_| "terminal_forge_report".to_string());
    let outdir = PathBuf::from(outdir);
    fs::create_dir_all(&outdir)?;

    let mut probes = Vec::new();
    for t in TOOLS {
        let path = which(t).unwrap_or_default();
        let found = !path.is_empty();
        let version = if found { safe_version(t) } else { String::new() };
        probes.push(Probe {
            name: t.to_string(),
            found,
            path,
            version,
        });
    }

    write_report(&outdir.join("BRAXON_TERMINAL_FORGE_REPORT.txt"), &probes)?;

    println!("BRAXON_TERMINAL_FORGE_OK");
    println!("report: {}", outdir.display());
    Ok(())
}
