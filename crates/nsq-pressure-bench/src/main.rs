//! DERIVED TRANSPORT / PRESSURE PATH ONLY
//! This crate measures packed transport, replay, and pressure behavior.
//! It must never be treated as canonical NSQ truth.

//! DERIVED ARTIFACT ONLY\n//! This crate is not canonical NSQ truth.\n//! Integer lanes, packed transport, and benchmark/index layouts here are\n//! disposable derivatives regenerated from preserved canonical NSQ artifacts.\n\nuse serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const MAGIC: &[u8; 8] = b"NSQPRM01";
const MODE_NOISE: u8 = 1;
const MODE_STRUCT: u8 = 2;

#[derive(Clone, Copy)]
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;
        self.0 = x;
        x
    }
    fn range(&mut self, max: usize) -> usize {
        if max == 0 {
            0
        } else {
            (self.next_u64() as usize) % max
        }
    }
}

#[derive(serde::Serialize)]
struct Score {
    lane: String,
    duration_secs: u64,
    native_bytes: u64,
    decoded_bytes: u64,
    decoded_lines: usize,
    decoded_records: usize,
    unique_symbols: usize,
    transitions: usize,
    class_counts: BTreeMap<String, usize>,
    compression_ratio: f64,
    records_per_sec: f64,
    decoded_bytes_per_sec: f64,
    native_sha_like: String,
    format_notes: Vec<String>,
}

fn hash64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

fn put_transport_u16<W: Write>(w: &mut W, v: u16) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn put_transport_u32<W: Write>(w: &mut W, v: u32) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn put_u64<W: Write>(w: &mut W, v: u64) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn get_transport_u16(buf: &[u8], off: &mut usize) -> io::Result<u16> {
    if *off + 2 > buf.len() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "u16"));
    }
    let v = u16::from_le_bytes([buf[*off], buf[*off + 1]]);
    *off += 2;
    Ok(v)
}
fn get_transport_u32(buf: &[u8], off: &mut usize) -> io::Result<u32> {
    if *off + 4 > buf.len() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "u32"));
    }
    let v = u32::from_le_bytes([buf[*off], buf[*off + 1], buf[*off + 2], buf[*off + 3]]);
    *off += 4;
    Ok(v)
}
fn get_u64(buf: &[u8], off: &mut usize) -> io::Result<u64> {
    if *off + 8 > buf.len() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "u64"));
    }
    let v = u64::from_le_bytes([
        buf[*off],
        buf[*off + 1],
        buf[*off + 2],
        buf[*off + 3],
        buf[*off + 4],
        buf[*off + 5],
        buf[*off + 6],
        buf[*off + 7],
    ]);
    *off += 8;
    Ok(v)
}

fn default_symbols() -> Vec<&'static str> {
    vec![
        "wake",
        "self",
        "semantic",
        "address",
        "bit",
        "lattice",
        "graph",
        "edge",
        "node",
        "delta",
        "macro",
        "switch",
        "lever",
        "cell",
        "membrane",
        "matrix",
        "pulse",
        "noise",
        "signal",
        "field",
        "anchor",
        "merge",
        "resolve",
        "route",
        "align",
        "stack",
        "core",
        "shard",
        "vector",
        "plane",
        "exception",
        "socio",
        "psych",
        "thread",
        "bridge",
        "gate",
        "state",
        "flux",
        "prime",
        "sovereign",
    ]
}

fn default_macros() -> Vec<&'static str> {
    vec![
        "wake:self",
        "semantic:address",
        "bit:lattice",
        "graph:edge:node",
        "macro:switch:lever",
        "cell:membrane:matrix",
        "noise:signal:field",
        "delta:route:align",
        "exception:socio:psych",
        "prime:flux:sovereign",
    ]
}

fn ensure_parent(path: &Path) -> io::Result<()> {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?;
    }
    Ok(())
}

fn write_header<W: Write>(
    w: &mut W,
    mode: u8,
    duration_secs: u64,
    symbols: &[&str],
    macros_: &[&str],
) -> io::Result<()> {
    w.write_all(MAGIC)?;
    w.write_all(&[mode])?;
    put_u64(w, duration_secs)?;
    put_transport_u16(w, symbols.len() as u16)?;
    put_transport_u16(w, macros_.len() as u16)?;
    for s in symbols {
        let b = s.as_bytes();
        put_transport_u16(w, b.len() as u16)?;
        w.write_all(b)?;
    }
    for m in macros_ {
        let b = m.as_bytes();
        put_transport_u16(w, b.len() as u16)?;
        w.write_all(b)?;
    }
    Ok(())
}

fn run_noise(out: &Path, seconds: u64) -> io::Result<()> {
    ensure_parent(out)?;
    let file = File::create(out)?;
    let mut w = BufWriter::new(file);
    let symbols = default_symbols();
    let macros_ = default_macros();
    write_header(&mut w, MODE_NOISE, seconds, &symbols, &macros_)?;

    let start = Instant::now();
    let dur = Duration::from_secs(seconds);
    let mut rng = Rng::new(0x4e53515f4e4f4953);
    let mut prev_sym: u16 = 0;
    let mut recs: u64 = 0;

    while start.elapsed() < dur {
        let sym = rng.range(symbols.len()) as u16;
        let delta_prev = sym.wrapping_sub(prev_sym);
        prev_sym = sym;

        let macro_id = rng.range(macros_.len()) as u16;
        let lever_a = (rng.next_u64() & 0x3f) as u8;
        let lever_b = (rng.next_u64() & 0x3f) as u8;
        let switch_pack = ((lever_a as u16) << 6) | lever_b as u16;
        let pos = rng.next_u64() & 0x00ff_ffff;
        let amp = (rng.next_u64() & 0xffff) as u16;

        put_transport_u16(&mut w, sym)?;
        put_transport_u16(&mut w, delta_prev)?;
        put_transport_u16(&mut w, macro_id)?;
        put_transport_u16(&mut w, switch_pack)?;
        put_transport_u32(&mut w, pos as u32)?;
        put_transport_u16(&mut w, amp)?;
        recs += 1;
    }

    w.flush()?;
    eprintln!("records={}", recs);
    Ok(())
}

fn run_struct(out: &Path, seconds: u64) -> io::Result<()> {
    ensure_parent(out)?;
    let file = File::create(out)?;
    let mut w = BufWriter::new(file);
    let symbols = default_symbols();
    let macros_ = default_macros();
    write_header(&mut w, MODE_STRUCT, seconds, &symbols, &macros_)?;

    let start = Instant::now();
    let dur = Duration::from_secs(seconds);
    let mut rng = Rng::new(0x4e53515f53545243);
    let mut prev_anchor: u32 = 0;
    let mut recs: u64 = 0;

    while start.elapsed() < dur {
        let subject = rng.range(symbols.len()) as u16;
        let relation = rng.range(macros_.len()) as u16;
        let object = rng.range(symbols.len()) as u16;
        let layer = (rng.next_u64() & 0x1f) as u8;
        let plane = (rng.next_u64() & 0x1f) as u8;
        let packed = ((layer as u16) << 5) | plane as u16;

        let anchor = (rng.next_u64() & 0x00ff_ffff) as u32;
        let delta_anchor = anchor.wrapping_sub(prev_anchor);
        prev_anchor = anchor;

        let weight = (rng.next_u64() & 0xffff) as u16;
        let flags = (rng.next_u64() & 0xff) as u8;

        put_transport_u16(&mut w, subject)?;
        put_transport_u16(&mut w, relation)?;
        put_transport_u16(&mut w, object)?;
        put_transport_u16(&mut w, packed)?;
        put_transport_u32(&mut w, delta_anchor)?;
        put_transport_u16(&mut w, weight)?;
        w.write_all(&[flags])?;
        recs += 1;
    }

    w.flush()?;
    eprintln!("records={}", recs);
    Ok(())
}

fn parse_strings(buf: &[u8], off: &mut usize, count: usize) -> io::Result<Vec<String>> {
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let len = get_transport_u16(buf, off)? as usize;
        if *off + len > buf.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "string bytes"));
        }
        let s = String::from_utf8_lossy(&buf[*off..*off + len]).to_string();
        *off += len;
        out.push(s);
    }
    Ok(out)
}

fn decode(input: &Path, decoded: &Path, score_json: &Path) -> io::Result<()> {
    let data = fs::read(input)?;
    let mut off = 0usize;

    if data.len() < 8 || &data[..8] != MAGIC {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "bad magic"));
    }
    off += 8;
    let mode = data[off];
    off += 1;
    let duration_secs = get_u64(&data, &mut off)?;
    let sym_count = get_transport_u16(&data, &mut off)? as usize;
    let macro_count = get_transport_u16(&data, &mut off)? as usize;
    let symbols = parse_strings(&data, &mut off, sym_count)?;
    let macros_ = parse_strings(&data, &mut off, macro_count)?;

    ensure_parent(decoded)?;
    ensure_parent(score_json)?;

    let mut decoded_out = BufWriter::new(File::create(decoded)?);
    let mut unique = BTreeSet::new();
    let mut classes: BTreeMap<String, usize> = BTreeMap::new();
    let mut decoded_records = 0usize;
    let mut transitions = 0usize;

    match mode {
        MODE_NOISE => {
            writeln!(decoded_out, "# lane=noise")?;
            while off < data.len() {
                if off + 14 > data.len() {
                    break;
                }
                let sym = get_transport_u16(&data, &mut off)?;
                let delta_prev = get_transport_u16(&data, &mut off)?;
                let macro_id = get_transport_u16(&data, &mut off)?;
                let switch_pack = get_transport_u16(&data, &mut off)?;
                let pos = get_transport_u32(&data, &mut off)?;
                let amp = get_transport_u16(&data, &mut off)?;

                let lever_a = (switch_pack >> 6) & 0x3f;
                let lever_b = switch_pack & 0x3f;

                let s = symbols
                    .get(sym as usize)
                    .map(String::as_str)
                    .unwrap_or("<?>");
                let m = macros_
                    .get(macro_id as usize)
                    .map(String::as_str)
                    .unwrap_or("<?>");

                writeln!(
                    decoded_out,
                    "noise sym={} macro={} leverA={} leverB={} pos={} amp={} dprev={}",
                    s, m, lever_a, lever_b, pos, amp, delta_prev
                )?;

                unique.insert(s.to_string());
                *classes.entry("symbol".to_string()).or_insert(0) += 1;
                if decoded_records > 0 {
                    transitions += 1;
                }
                decoded_records += 1;
            }
        }
        MODE_STRUCT => {
            writeln!(decoded_out, "# lane=structured")?;
            let mut anchor: u32 = 0;
            while off < data.len() {
                if off + 13 > data.len() {
                    break;
                }
                let subject = get_transport_u16(&data, &mut off)?;
                let relation = get_transport_u16(&data, &mut off)?;
                let object = get_transport_u16(&data, &mut off)?;
                let packed = get_transport_u16(&data, &mut off)?;
                let delta_anchor = get_transport_u32(&data, &mut off)?;
                let weight = get_transport_u16(&data, &mut off)?;
                let flags = data[off];
                off += 1;

                anchor = anchor.wrapping_add(delta_anchor);

                let layer = (packed >> 5) & 0x1f;
                let plane = packed & 0x1f;
                let s = symbols
                    .get(subject as usize)
                    .map(String::as_str)
                    .unwrap_or("<?>");
                let r = macros_
                    .get(relation as usize)
                    .map(String::as_str)
                    .unwrap_or("<?>");
                let o = symbols
                    .get(object as usize)
                    .map(String::as_str)
                    .unwrap_or("<?>");

                writeln!(
                    decoded_out,
                    "triple subject={} relation={} object={} layer={} plane={} anchor={} weight={} flags={}",
                    s, r, o, layer, plane, anchor, weight, flags
                )?;

                unique.insert(s.to_string());
                unique.insert(o.to_string());
                if o.chars().all(|c| c.is_ascii_digit()) {
                    *classes.entry("int".to_string()).or_insert(0) += 1;
                } else {
                    *classes.entry("symbol".to_string()).or_insert(0) += 2;
                }
                if decoded_records > 0 {
                    transitions += 1;
                }
                decoded_records += 1;
            }
        }
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "unknown mode")),
    }

    decoded_out.flush()?;
    let decoded_bytes = fs::metadata(decoded)?.len();
    let decoded_text = fs::read_to_string(decoded)?;
    let decoded_lines = if decoded_text.is_empty() {
        0
    } else {
        decoded_text.lines().count()
    };

    let native_bytes = data.len() as u64;
    let native_hash = format!("{:016x}", hash64(&data));

    let score = Score {
        lane: if mode == MODE_NOISE {
            "nsq-native-noise".to_string()
        } else {
            "nsq-native-structured".to_string()
        },
        duration_secs,
        native_bytes,
        decoded_bytes,
        decoded_lines,
        decoded_records,
        unique_symbols: unique.len(),
        transitions,
        class_counts: classes,
        compression_ratio: if native_bytes == 0 {
            0.0
        } else {
            decoded_bytes as f64 / native_bytes as f64
        },
        records_per_sec: if duration_secs == 0 {
            0.0
        } else {
            decoded_records as f64 / duration_secs as f64
        },
        decoded_bytes_per_sec: if duration_secs == 0 {
            0.0
        } else {
            decoded_bytes as f64 / duration_secs as f64
        },
        native_sha_like: native_hash,
        format_notes: vec![
            "symbol interning".to_string(),
            "macro bank".to_string(),
            "switch lever packing".to_string(),
            "delta-coded references".to_string(),
            "native binary lane".to_string(),
            "decoded human-readable export".to_string(),
        ],
    };

    fs::write(score_json, serde_json::to_string_pretty(&score).unwrap())?;
    Ok(())
}

fn usage() -> ! {
    eprintln!("usage:");
    eprintln!("  nsq-pressure-bench write-noise <seconds> <native_out>");
    eprintln!("  nsq-pressure-bench write-structured <seconds> <native_out>");
    eprintln!("  nsq-pressure-bench decode <native_in> <decoded_txt> <score_json>");
    std::process::exit(2);
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("write-noise") => {
            let secs = args
                .next()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or_else(|| usage());
            let out = args.next().map(PathBuf::from).unwrap_or_else(|| usage());
            run_noise(&out, secs).unwrap();
        }
        Some("write-structured") => {
            let secs = args
                .next()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or_else(|| usage());
            let out = args.next().map(PathBuf::from).unwrap_or_else(|| usage());
            run_struct(&out, secs).unwrap();
        }
        Some("decode") => {
            let input = args.next().map(PathBuf::from).unwrap_or_else(|| usage());
            let decoded = args.next().map(PathBuf::from).unwrap_or_else(|| usage());
            let score = args.next().map(PathBuf::from).unwrap_or_else(|| usage());
            decode(&input, &decoded, &score).unwrap();
        }
        _ => usage(),
    }
}
