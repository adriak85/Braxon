use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Dialect {
    Canonical,
    SExpr,
    LuaShape,
    PythonShape,
}

impl Dialect {
    fn as_str(self) -> &'static str {
        match self {
            Dialect::Canonical => "canonical",
            Dialect::SExpr => "sexpr",
            Dialect::LuaShape => "lua_shape",
            Dialect::PythonShape => "python_shape",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CalibrationLock {
    selected_profile: String,
    promoted_macros: Vec<String>,
    hot_targets: Vec<String>,
    threshold_macro_promotion: usize,
    threshold_expansion: usize,
    representation_lock: serde_json::Value,
    rebalance_actions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NativeArtifact {
    artifact_version: String,
    source_path: String,
    source_hash_sha256: String,
    source_dialect: String,
    calibration_lock: CalibrationLock,
    records: Vec<PreservedRecord>,
    provenance: Provenance,
}

#[derive(Debug, Serialize, Deserialize)]
struct Provenance {
    compiler: String,
    mode: String,
    preservation: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PreservedRecord {
    dialect: String,
    source_line: String,
    semantic: SemanticRecord,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum SemanticRecord {
    Noise {
        symbol: String,
        macro_name: String,
        a: String,
        b: String,
        pos: String,
        amp: String,
    },
    Triple {
        subject: String,
        relation: String,
        object: String,
        layer: String,
        plane: String,
        anchor: String,
        weight: String,
        flags: String,
    },
    Membrane {
        cell: String,
        state: String,
        flux: String,
        gate: String,
        phase: String,
    },
}

fn split_tokens_preserving_quotes(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if let Some(q) = quote {
            if ch == '\\' {
                if let Some(n) = chars.next() {
                    cur.push(n);
                }
            } else if ch == q {
                quote = None;
            } else {
                cur.push(ch);
            }
            continue;
        }

        match ch {
            '"' | '\'' => quote = Some(ch),
            c if c.is_whitespace() => {
                if !cur.is_empty() {
                    out.push(cur.clone());
                    cur.clear();
                }
            }
            _ => cur.push(ch),
        }
    }

    if !cur.is_empty() {
        out.push(cur);
    }

    out
}

fn detect_dialect(text: &str) -> Dialect {
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let lower = line.to_ascii_lowercase();
        if lower == "@dialect sexpr" || lower == "!dialect sexpr" || lower == "dialect sexpr" {
            return Dialect::SExpr;
        }
        if lower == "@dialect lua_shape"
            || lower == "!dialect lua_shape"
            || lower == "dialect lua_shape"
        {
            return Dialect::LuaShape;
        }
        if lower == "@dialect python_shape"
            || lower == "!dialect python_shape"
            || lower == "dialect python_shape"
        {
            return Dialect::PythonShape;
        }
        if line.starts_with('(') && line.ends_with(')') {
            return Dialect::SExpr;
        }
        if line.contains('=')
            && (line.starts_with("noise ")
                || line.starts_with("triple ")
                || line.starts_with("membrane "))
        {
            if line.contains('(') && line.ends_with(')') {
                return Dialect::PythonShape;
            }
            return Dialect::LuaShape;
        }
        if line.starts_with("noise ")
            || line.starts_with("triple ")
            || line.starts_with("membrane ")
        {
            return Dialect::Canonical;
        }
    }
    Dialect::Canonical
}

fn strip_outer_parens(s: &str) -> &str {
    let t = s.trim();
    if t.starts_with('(') && t.ends_with(')') && t.len() >= 2 {
        &t[1..t.len() - 1]
    } else {
        t
    }
}

fn kv_map(tokens: &[String]) -> (Vec<String>, BTreeMap<String, String>) {
    let mut positional = Vec::new();
    let mut kv = BTreeMap::new();
    for tok in tokens {
        if let Some(eq) = tok.find('=') {
            let k = tok[..eq].trim().trim_end_matches(':').to_string();
            let v = tok[eq + 1..].trim().to_string();
            kv.insert(k, v);
        } else {
            positional.push(tok.clone());
        }
    }
    (positional, kv)
}

fn parse_canonical(line: &str) -> Option<SemanticRecord> {
    let toks: Vec<&str> = line.split_whitespace().collect();
    if toks.is_empty() {
        return None;
    }

    match toks[0] {
        "noise" => {
            let symbol = toks.get(1)?.to_string();
            let mut macro_name = String::new();
            let mut a = String::new();
            let mut b = String::new();
            let mut pos = String::new();
            let mut amp = String::new();

            let mut i = 2usize;
            while i + 1 < toks.len() {
                match toks[i] {
                    ":macro" => macro_name = toks[i + 1].to_string(),
                    ":a" => a = toks[i + 1].to_string(),
                    ":b" => b = toks[i + 1].to_string(),
                    ":pos" => pos = toks[i + 1].to_string(),
                    ":amp" => amp = toks[i + 1].to_string(),
                    _ => {}
                }
                i += 2;
            }

            Some(SemanticRecord::Noise {
                symbol,
                macro_name,
                a,
                b,
                pos,
                amp,
            })
        }
        "triple" => {
            let subject = toks.get(1)?.to_string();
            let relation = toks.get(3)?.to_string();
            let object = toks.get(5)?.to_string();

            let mut layer = String::new();
            let mut plane = String::new();
            let mut anchor = String::new();
            let mut weight = String::new();
            let mut flags = String::new();

            let mut i = 6usize;
            while i + 1 < toks.len() {
                match toks[i] {
                    ":layer" => layer = toks[i + 1].to_string(),
                    ":plane" => plane = toks[i + 1].to_string(),
                    ":anchor" => anchor = toks[i + 1].to_string(),
                    ":weight" => weight = toks[i + 1].to_string(),
                    ":flags" => flags = toks[i + 1].to_string(),
                    _ => {}
                }
                i += 2;
            }

            Some(SemanticRecord::Triple {
                subject,
                relation,
                object,
                layer,
                plane,
                anchor,
                weight,
                flags,
            })
        }
        "membrane" => {
            let cell = toks.get(1)?.to_string();

            let mut state = String::new();
            let mut flux = String::new();
            let mut gate = String::new();
            let mut phase = String::new();

            let mut i = 2usize;
            while i + 1 < toks.len() {
                match toks[i] {
                    ":state" => state = toks[i + 1].to_string(),
                    ":flux" => flux = toks[i + 1].to_string(),
                    ":gate" => gate = toks[i + 1].to_string(),
                    ":phase" => phase = toks[i + 1].to_string(),
                    _ => {}
                }
                i += 2;
            }

            Some(SemanticRecord::Membrane {
                cell,
                state,
                flux,
                gate,
                phase,
            })
        }
        _ => None,
    }
}

fn parse_sexpr(line: &str) -> Option<SemanticRecord> {
    let inner = strip_outer_parens(line);
    let toks = split_tokens_preserving_quotes(inner);
    if toks.is_empty() {
        return None;
    }

    match toks[0].as_str() {
        "noise" => {
            let symbol = toks.get(1)?.clone();
            let mut macro_name = String::new();
            let mut a = String::new();
            let mut b = String::new();
            let mut pos = String::new();
            let mut amp = String::new();

            let mut i = 2usize;
            while i + 1 < toks.len() {
                match toks[i].as_str() {
                    "macro" => macro_name = toks[i + 1].clone(),
                    "a" => a = toks[i + 1].clone(),
                    "b" => b = toks[i + 1].clone(),
                    "pos" => pos = toks[i + 1].clone(),
                    "amp" => amp = toks[i + 1].clone(),
                    _ => {}
                }
                i += 2;
            }

            Some(SemanticRecord::Noise {
                symbol,
                macro_name,
                a,
                b,
                pos,
                amp,
            })
        }
        "triple" => {
            let subject = toks.get(1)?.clone();
            let relation = toks.get(2)?.clone();
            let object = toks.get(3)?.clone();

            let mut layer = String::new();
            let mut plane = String::new();
            let mut anchor = String::new();
            let mut weight = String::new();
            let mut flags = String::new();

            let mut i = 4usize;
            while i + 1 < toks.len() {
                match toks[i].as_str() {
                    "layer" => layer = toks[i + 1].clone(),
                    "plane" => plane = toks[i + 1].clone(),
                    "anchor" => anchor = toks[i + 1].clone(),
                    "weight" => weight = toks[i + 1].clone(),
                    "flags" => flags = toks[i + 1].clone(),
                    _ => {}
                }
                i += 2;
            }

            Some(SemanticRecord::Triple {
                subject,
                relation,
                object,
                layer,
                plane,
                anchor,
                weight,
                flags,
            })
        }
        "membrane" => {
            let cell = toks.get(1)?.clone();

            let mut state = String::new();
            let mut flux = String::new();
            let mut gate = String::new();
            let mut phase = String::new();

            let mut i = 2usize;
            while i + 1 < toks.len() {
                match toks[i].as_str() {
                    "state" => state = toks[i + 1].clone(),
                    "flux" => flux = toks[i + 1].clone(),
                    "gate" => gate = toks[i + 1].clone(),
                    "phase" => phase = toks[i + 1].clone(),
                    _ => {}
                }
                i += 2;
            }

            Some(SemanticRecord::Membrane {
                cell,
                state,
                flux,
                gate,
                phase,
            })
        }
        _ => None,
    }
}

fn parse_kv_shape(line: &str) -> Option<SemanticRecord> {
    let normalized = line.replace(['(', ')', ','], " ");
    let toks = split_tokens_preserving_quotes(&normalized);
    if toks.is_empty() {
        return None;
    }

    let head = toks[0].as_str();
    let (positional, kv) = kv_map(&toks[1..]);

    match head {
        "noise" => {
            let symbol = positional.first()?.clone();
            Some(SemanticRecord::Noise {
                symbol,
                macro_name: kv.get("macro").cloned().unwrap_or_default(),
                a: kv.get("a").cloned().unwrap_or_default(),
                b: kv.get("b").cloned().unwrap_or_default(),
                pos: kv.get("pos").cloned().unwrap_or_default(),
                amp: kv.get("amp").cloned().unwrap_or_default(),
            })
        }
        "triple" => {
            let subject = positional
                .first()
                .cloned()
                .or_else(|| kv.get("subject").cloned())?;
            let relation = kv
                .get("rel")
                .cloned()
                .or_else(|| kv.get("relation").cloned())
                .or_else(|| positional.get(1).cloned())?;
            let object = kv
                .get("obj")
                .cloned()
                .or_else(|| kv.get("object").cloned())
                .or_else(|| positional.get(2).cloned())?;

            Some(SemanticRecord::Triple {
                subject,
                relation,
                object,
                layer: kv.get("layer").cloned().unwrap_or_default(),
                plane: kv.get("plane").cloned().unwrap_or_default(),
                anchor: kv.get("anchor").cloned().unwrap_or_default(),
                weight: kv.get("weight").cloned().unwrap_or_default(),
                flags: kv.get("flags").cloned().unwrap_or_default(),
            })
        }
        "membrane" => {
            let cell = positional
                .first()
                .cloned()
                .or_else(|| kv.get("cell").cloned())?;
            Some(SemanticRecord::Membrane {
                cell,
                state: kv.get("state").cloned().unwrap_or_default(),
                flux: kv.get("flux").cloned().unwrap_or_default(),
                gate: kv.get("gate").cloned().unwrap_or_default(),
                phase: kv.get("phase").cloned().unwrap_or_default(),
            })
        }
        _ => None,
    }
}

fn parse_preserving(text: &str) -> (Dialect, Vec<PreservedRecord>) {
    let dialect = detect_dialect(text);
    let mut out = Vec::new();

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let lower = line.to_ascii_lowercase();
        if lower.starts_with("@dialect ")
            || lower.starts_with("!dialect ")
            || lower.starts_with("dialect ")
        {
            continue;
        }

        let semantic = match dialect {
            Dialect::Canonical => parse_canonical(line),
            Dialect::SExpr => parse_sexpr(line),
            Dialect::LuaShape | Dialect::PythonShape => parse_kv_shape(line),
        };

        if let Some(semantic) = semantic {
            out.push(PreservedRecord {
                dialect: dialect.as_str().to_string(),
                source_line: line.to_string(),
                semantic,
            });
        }
    }

    (dialect, out)
}

fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    let bytes = h.finalize();
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: nsq-preserve <input.nsq> <calibration_lock.json> <output.native.json>");
        std::process::exit(2);
    }

    let input = &args[1];
    let lock_path = &args[2];
    let output = &args[3];

    let source_bytes = fs::read(input)?;
    let text = String::from_utf8_lossy(&source_bytes).to_string();
    let lock_raw = fs::read_to_string(lock_path)?;
    let lock: CalibrationLock = serde_json::from_str(&lock_raw)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let (dialect, records) = parse_preserving(&text);

    let artifact = NativeArtifact {
        artifact_version: "nsq-native-0".to_string(),
        source_path: input.to_string(),
        source_hash_sha256: sha256_hex(&source_bytes),
        source_dialect: dialect.as_str().to_string(),
        calibration_lock: lock,
        records,
        provenance: Provenance {
            compiler: "nsq-preserve".to_string(),
            mode: "canonical-preservation".to_string(),
            preservation: "dialect+source+semantic".to_string(),
        },
    };

    if let Some(parent) = Path::new(output).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(output, serde_json::to_vec_pretty(&artifact)?)?;
    println!("preserved={}", output);
    Ok(())
}
