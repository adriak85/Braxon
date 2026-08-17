use std::{env, fs, io, path::{Path, PathBuf}};

const SECTION_BYTES: usize = 64 * 1024;

fn digest(bytes: &[u8]) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{h:016x}")
}

fn classify(path: &Path, text: &str) -> &'static str {
    let n = path.file_name().and_then(|x| x.to_str()).unwrap_or("").to_ascii_lowercase();
    if n.contains("bench") || text.contains("criterion") || text.contains("benchmark") { return "benchmark"; }
    if n.contains("test") || text.contains("#[test]") || text.contains("assert!") { return "verification"; }
    if n.contains("readme") || n.ends_with(".md") || n.contains("contract") || n.contains("law") { return "contract"; }
    if n.contains("before_") || n.contains("backup") || n.contains("deprecated") || n.contains("archive") { return "evidence"; }
    if n.ends_with(".toml") || n.ends_with(".lock") || n.ends_with(".json") || n.ends_with(".yaml") || n.ends_with(".yml") { return "configuration"; }
    "implementation"
}

fn intent(path: &Path, text: &str) -> String {
    let mut tags = Vec::new();
    let lower = text.to_ascii_lowercase();
    for (needle, tag) in [
        ("intent", "intent"), ("gradient", "gradient"), ("lever", "state.lever"),
        ("seat", "state.seating"), ("stamp", "semantic.stamp"), ("watermark", "semantic.watermark"),
        ("runtime", "runtime"), ("compile", "build"), ("llvm", "toolchain"),
        ("ingest", "ingestion"), ("compose", "composition"), ("calibrat", "calibration"),
        ("compress", "compression"), ("archon", "orchestration"), ("cli", "interface"),
    ] { if lower.contains(needle) { tags.push(tag); } }
    if tags.is_empty() {
        tags.push(match classify(path, text) {
            "implementation" => "implementation",
            "verification" => "verification",
            "benchmark" => "measurement",
            "contract" => "contract",
            "configuration" => "configuration",
            _ => "provenance",
        });
    }
    tags.sort_unstable(); tags.dedup(); tags.join("+")
}

fn emit_file(root: &Path, path: &Path, out: &mut impl io::Write) -> io::Result<()> {
    let bytes = fs::read(path)?;
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/");
    let total = bytes.len().max(1);
    let mut offset = 0usize;
    let mut section = 0usize;
    while offset < bytes.len().max(1) {
        let end = (offset + SECTION_BYTES).min(bytes.len());
        let chunk = if bytes.is_empty() { &[][..] } else { &bytes[offset..end] };
        let text = String::from_utf8_lossy(chunk);
        writeln!(out, "NODE source.section {{")?;
        writeln!(out, "  PATH = {:?};", rel)?;
        writeln!(out, "  SECTION = {section};")?;
        writeln!(out, "  OFFSET = {offset};")?;
        writeln!(out, "  SIZE = {};", chunk.len())?;
        writeln!(out, "  FRACTION = {:.9};", end as f64 / total as f64)?;
        writeln!(out, "  DIGEST = {:?};", digest(chunk))?;
        writeln!(out, "  KIND = {:?};", classify(path, &text))?;
        writeln!(out, "  INTENT = {:?};", intent(path, &text))?;
        writeln!(out, "  LANGUAGE = {:?};", path.extension().and_then(|x| x.to_str()).unwrap_or("unknown"))?;
        writeln!(out, "  PROVENANCE = SOURCE;\n}}\n")?;
        if bytes.is_empty() { break; }
        offset = end; section += 1;
    }
    Ok(())
}

fn walk(root: &Path, dir: &Path, out: &mut impl io::Write) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?; let p = entry.path();
        if p.file_name().and_then(|x| x.to_str()) == Some("target") { continue; }
        if p.is_dir() { walk(root, &p, out)?; }
        else if p.is_file() { emit_file(root, &p, out)?; }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let root = PathBuf::from(env::args().nth(1).unwrap_or_else(|| ".".into()));
    let output = env::args().nth(2).unwrap_or_else(|| "NSQ_SOURCE_STREAM.nsq".into());
    let mut f = fs::File::create(&output)?;
    writeln!(f, "NSQ.SOURCE_STREAM {{ VERSION = 1; ROOT = {:?}; }}\n", root.display().to_string())?;
    walk(&root, &root, &mut f)
}
