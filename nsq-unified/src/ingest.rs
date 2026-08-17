use std::{env, fs, io, path::{Path, PathBuf}};

const SECTION_BYTES: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Kind {
    Implementation,
    Contract,
    Verification,
    Benchmark,
    Configuration,
    Evidence,
}

impl Kind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Implementation => "implementation",
            Self::Contract => "contract",
            Self::Verification => "verification",
            Self::Benchmark => "benchmark",
            Self::Configuration => "configuration",
            Self::Evidence => "evidence",
        }
    }
}

fn digest(bytes: &[u8]) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{h:016x}")
}

fn classify(path: &Path, text: &str) -> Kind {
    let name = path.file_name()
        .and_then(|x| x.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let lower = text.to_ascii_lowercase();

    if name.contains("before_")
        || name.contains("backup")
        || name.contains("deprecated")
        || name.contains("archive")
    {
        return Kind::Evidence;
    }
    if name.contains("bench") || lower.contains("criterion") || lower.contains("benchmark") {
        return Kind::Benchmark;
    }
    if name.contains("test")
        || lower.contains("#[test]")
        || lower.contains("#[cfg(test)]")
        || lower.contains("assert!(")
    {
        return Kind::Verification;
    }
    if name.contains("readme")
        || name.ends_with(".md")
        || name.contains("contract")
        || name.contains("law")
        || lower.contains("architecture law")
    {
        return Kind::Contract;
    }
    if matches!(
        path.extension().and_then(|x| x.to_str()),
        Some("toml" | "lock" | "json" | "yaml" | "yml" | "ini")
    ) {
        return Kind::Configuration;
    }
    Kind::Implementation
}

fn intent(path: &Path, text: &str) -> String {
    let lower = text.to_ascii_lowercase();
    let mut tags = Vec::new();

    for (needle, tag) in [
        ("intent", "intent"),
        ("gradient", "gradient"),
        ("lever", "state.lever"),
        ("seat", "state.seating"),
        ("stamp", "semantic.stamp"),
        ("watermark", "semantic.watermark"),
        ("runtime", "runtime"),
        ("compile", "build"),
        ("llvm", "toolchain"),
        ("ingest", "ingestion"),
        ("compose", "composition"),
        ("calibrat", "calibration"),
        ("compress", "compression"),
        ("archon", "orchestration"),
        ("query", "query"),
        ("decode", "decode"),
        ("encode", "encode"),
        ("pack", "packing"),
        ("proof", "proof"),
        ("preserve", "preservation"),
        ("registry", "registry"),
    ] {
        if lower.contains(needle) {
            tags.push(tag);
        }
    }

    if tags.is_empty() {
        tags.push(match classify(path, text) {
            Kind::Implementation => "implementation",
            Kind::Verification => "verification",
            Kind::Benchmark => "measurement",
            Kind::Contract => "contract",
            Kind::Configuration => "configuration",
            Kind::Evidence => "provenance",
        });
    }

    tags.sort_unstable();
    tags.dedup();
    tags.join("+")
}

fn language(path: &Path) -> &'static str {
    match path.extension().and_then(|x| x.to_str()).unwrap_or("") {
        "rs" => "rust",
        "c" => "c",
        "cc" | "cpp" | "cxx" | "hpp" | "h" => "cpp-c",
        "s" | "S" | "asm" => "assembly",
        "toml" => "toml",
        "json" => "json",
        "md" => "markdown",
        "yaml" | "yml" => "yaml",
        "sh" => "shell",
        _ => "other",
    }
}

fn excluded(rel: &Path, output: &Path) -> bool {
    if rel == output || rel.starts_with(output) {
        return true;
    }
    rel.components().any(|component| {
        matches!(
            component.as_os_str().to_str(),
            Some(".git" | "target" | "node_modules" | ".cache" | "build" | "dist")
        )
    })
}

fn collect(root: &Path, dir: &Path, output: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut entries = fs::read_dir(dir)?
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(&path);
        if excluded(rel, output) {
            continue;
        }
        if path.is_dir() {
            collect(root, &path, output, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn emit_file(root: &Path, path: &Path, out: &mut impl io::Write) -> io::Result<()> {
    let bytes = fs::read(path)?;
    let rel = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    let text = String::from_utf8_lossy(&bytes);
    let kind = classify(path, &text);
    let full_digest = digest(&bytes);
    let lines = text.lines().count();

    writeln!(out, "NODE source.file {{")?;
    writeln!(out, "  PATH = {:?};", rel)?;
    writeln!(out, "  SIZE = {};", bytes.len())?;
    writeln!(out, "  LINES = {};", lines)?;
    writeln!(out, "  DIGEST = {:?};", full_digest)?;
    writeln!(out, "  KIND = {:?};", kind.as_str())?;
    writeln!(out, "  INTENT = {:?};", intent(path, &text))?;
    writeln!(out, "  LANGUAGE = {:?};", language(path))?;
    writeln!(out, "  PROVENANCE = SOURCE;")?;
    writeln!(out, "}}\n")?;

    let mut offset = 0usize;
    let mut section = 0usize;
    while offset < bytes.len().max(1) {
        let end = (offset + SECTION_BYTES).min(bytes.len());
        let chunk = if bytes.is_empty() { &[][..] } else { &bytes[offset..end] };
        let chunk_text = String::from_utf8_lossy(chunk);

        writeln!(out, "NODE source.section {{")?;
        writeln!(out, "  PATH = {:?};", rel)?;
        writeln!(out, "  SECTION = {section};")?;
        writeln!(out, "  OFFSET = {offset};")?;
        writeln!(out, "  SIZE = {};", chunk.len())?;
        writeln!(out, "  DIGEST = {:?};", digest(chunk))?;
        writeln!(out, "  KIND = {:?};", kind.as_str())?;
        writeln!(out, "  INTENT = {:?};", intent(path, &chunk_text))?;
        writeln!(out, "  LANGUAGE = {:?};", language(path))?;
        writeln!(out, "}}\n")?;

        if bytes.is_empty() {
            break;
        }
        offset = end;
        section += 1;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let root = PathBuf::from(env::args().nth(1).unwrap_or_else(|| ".".into()));
    let output = PathBuf::from(
        env::args()
            .nth(2)
            .unwrap_or_else(|| "NSQ_SOURCE_STREAM.nsq".into()),
    );

    let output_abs = if output.is_absolute() {
        output.clone()
    } else {
        root.join(&output)
    };

    let mut files = Vec::new();
    collect(&root, &root, &output_abs.strip_prefix(&root).unwrap_or(&output_abs), &mut files)?;
    files.sort_by(|a, b| {
        a.strip_prefix(&root)
            .unwrap_or(a)
            .cmp(b.strip_prefix(&root).unwrap_or(b))
    });

    let mut f = fs::File::create(&output_abs)?;
    writeln!(f, "NSQ.SOURCE_STREAM {{ VERSION = 2; }}\n")?;
    writeln!(f, "META {{ FILES = {}; }}\n", files.len())?;

    for path in files {
        emit_file(&root, &path, &mut f)?;
    }
    Ok(())
}
