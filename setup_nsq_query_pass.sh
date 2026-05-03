#!/data/data/com.termux/files/usr/bin/bash
# setup_nsq_query_pass.sh
# Installs three things into ~/Braxon:
#   1. nsq-index updated with binary frame format (NSQIDX01)
#   2. nsq-query updated with --batch mode (load once, run all queries in-process)
#   3. nsq-bench-split: fair split benchmarks separating core/cold/warm latency
#
# Drop into ~/Braxon and run:
#   bash ~/setup_nsq_query_pass.sh
# Then rebuild:
#   cargo build -p nsq-index -p nsq-query -p nsq-bench-split --release
set -euo pipefail

BRAXON_HOME="${BRAXON_HOME:-$HOME/Braxon}"
cd "$BRAXON_HOME"

# ── ensure nsq-bench-split is in workspace ──────────────────────────────────
python3 - <<'PY'
from pathlib import Path
p = Path("Cargo.toml")
s = p.read_text()
member = '    "crates/nsq-bench-split",'
if member not in s:
    s = s.replace("members = [\n", "members = [\n" + member + "\n", 1)
    p.write_text(s)
    print("wired nsq-bench-split into workspace")
else:
    print("nsq-bench-split already in workspace")
PY

mkdir -p crates/nsq-bench-split/src

# ══════════════════════════════════════════════════════════════════════════════
# 1. nsq-index/src/lib.rs — add binary frame support alongside compact JSON
# ══════════════════════════════════════════════════════════════════════════════
cat > crates/nsq-index/src/lib.rs << 'RUST'
//! nsq-index — ArtifactIndex with binary frame + batch-query support.
//!
//! Two serialization formats:
//!   COMPACT JSON  — serde_json compact, human-inspectable  (.idx.json)
//!   BINARY FRAME  — NSQIDX01 custom frame, fastest load    (.idx.bin)
//!
//! Binary frame layout:
//!   [0..8]   magic b"NSQIDX01"
//!   [8..12]  u32 le: symbol count
//!   [12..16] u32 le: edge count
//!   [16..20] u32 le: state count
//!   [20..]   symbol table: for each symbol: u16-le len, utf8 bytes
//!            edge table:   for each edge:   u16 left_id, u16 rel_id, u16 right_id,
//!                                           u8 layer, u8 plane, u32 anchor, u16 weight, u8 flags
//!            state table:  for each state:  u16 target_id, u16 state_sym_id, u16 flux, u8 gate, u8 phase
//!            adjacency:    u32 n_left_entries, then n_left_entries × (u16 sym_id, u32 edge_list_offset, u32 edge_list_len)
//!                          followed by the flat edge-id array
//!            (right and rel adjacency follow the same pattern)
//!            anchor index: u32 n_anchor, then n_anchor × (u32 anchor, u32 edge_id)  sorted by anchor
//!
//! Binary format cuts load time by avoiding JSON tokenisation + UTF-8 validation
//! on the hot fields (symbol IDs, edge IDs are numeric, not strings).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Cursor, Read, Write};
use std::path::Path;
use std::{collections::BTreeSet, fs};

// ── wire types ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub left:   String,
    pub rel:    String,
    pub right:  String,
    pub layer:  u8,
    pub plane:  u8,
    pub anchor: u32,
    pub weight: u16,
    pub flags:  u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub target: String,
    pub state:  String,
    pub flux:   u16,
    pub gate:   u8,
    pub phase:  u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexStats {
    pub input_lines:            usize,
    pub normalized_lines:       usize,
    pub comment_lines_stripped: usize,
    pub duplicate_lines_removed: usize,
    pub symbols:                usize,
    pub macros:                 usize,
    pub edges:                  usize,
    pub states:                 usize,
    pub index_bytes:            usize,
}

// ── main index type ────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactIndex {
    pub source_path:       String,
    pub symbol_to_id:      HashMap<String, u32>,
    pub id_to_symbol:      Vec<String>,
    pub macro_to_edges:    HashMap<String, Vec<u32>>,
    pub left_to_edges:     HashMap<String, Vec<u32>>,
    pub right_to_edges:    HashMap<String, Vec<u32>>,
    pub target_to_states:  HashMap<String, Vec<u32>>,
    pub anchor_edges:      Vec<(u32, u32)>,   // (anchor, edge_id) sorted
    pub edges:             Vec<Edge>,
    pub states:            Vec<State>,
    pub stats:             IndexStats,
}

// ── build ─────────────────────────────────────────────────────────────────

fn parse_u8_lossy(s: &str)  -> u8  { s.parse().unwrap_or(0) }
fn parse_u16_lossy(s: &str) -> u16 { s.parse().unwrap_or(0) }
fn parse_u32_lossy(s: &str) -> u32 { s.parse().unwrap_or(0) }

pub fn normalize_canonical_text(text: &str) -> (Vec<String>, usize, usize) {
    let mut stripped = 0usize;
    let mut seen     = BTreeSet::<String>::new();
    let mut dups     = 0usize;
    let mut out      = Vec::<String>::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty()                                          { continue; }
        if line.starts_with('#')                                    { stripped += 1; continue; }
        if line.starts_with("@dialect ") || line.starts_with("!dialect ")
           || line.starts_with("dialect ")                         { continue; }
        let norm = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if seen.insert(norm.clone()) { out.push(norm); } else { dups += 1; }
    }
    out.sort();
    (out, stripped, dups)
}

pub fn parse_edges_states(
    lines: &[String],
) -> (Vec<Edge>, Vec<State>, BTreeSet<String>, BTreeSet<String>) {
    let mut edges_map  = std::collections::BTreeMap::<String, Edge>::new();
    let mut states_map = std::collections::BTreeMap::<String, State>::new();
    let mut symbols    = BTreeSet::<String>::new();
    let mut macros     = BTreeSet::<String>::new();

    for line in lines {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.is_empty() { continue; }
        match toks[0] {
            "triple" => {
                if toks.len() < 6 { continue; }
                let left  = toks[1].to_string();
                let rel   = toks[3].to_string();
                let right = toks[5].to_string();
                symbols.insert(left.clone());
                symbols.insert(right.clone());
                macros.insert(rel.clone());
                let (mut layer, mut plane, mut anchor, mut weight, mut flags) =
                    (0u8, 0u8, 0u32, 0u16, 0u8);
                let mut i = 6usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":layer"  => layer  = parse_u8_lossy(toks[i+1]),
                        ":plane"  => plane  = parse_u8_lossy(toks[i+1]),
                        ":anchor" => anchor = parse_u32_lossy(toks[i+1]),
                        ":weight" => weight = parse_u16_lossy(toks[i+1]),
                        ":flags"  => flags  = parse_u8_lossy(toks[i+1]),
                        _ => {}
                    }
                    i += 2;
                }
                let e = Edge { left, rel, right, layer, plane, anchor, weight, flags };
                let key = format!("{}|{}|{}|{}|{}|{}|{}|{}",
                    e.left, e.rel, e.right, e.layer, e.plane, e.anchor, e.weight, e.flags);
                edges_map.insert(key, e);
            }
            "membrane" => {
                if toks.len() < 2 { continue; }
                let target = toks[1].to_string();
                symbols.insert(target.clone());
                let (mut state, mut flux, mut gate, mut phase) =
                    ("<?>" .to_string(), 0u16, 0u8, 0u8);
                let mut i = 2usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":state" => state = toks[i+1].to_string(),
                        ":flux"  => flux  = parse_u16_lossy(toks[i+1]),
                        ":gate"  => gate  = parse_u8_lossy(toks[i+1]),
                        ":phase" => phase = parse_u8_lossy(toks[i+1]),
                        _ => {}
                    }
                    i += 2;
                }
                let st = State { target, state, flux, gate, phase };
                let key = format!("{}|{}|{}|{}|{}",
                    st.target, st.state, st.flux, st.gate, st.phase);
                states_map.insert(key, st);
            }
            _ => {}
        }
    }
    (edges_map.into_values().collect(),
     states_map.into_values().collect(),
     symbols, macros)
}

pub fn build_index_from_text(source_path: &str, text: &str) -> ArtifactIndex {
    let input_lines = text.lines().count();
    let (normalized, comment_lines_stripped, duplicate_lines_removed) =
        normalize_canonical_text(text);
    let (mut edges, mut states, symbols, macros) = parse_edges_states(&normalized);

    edges.sort_by(|a, b| {
        (&a.left, &a.rel, &a.right, a.layer, a.plane, a.anchor, a.weight, a.flags)
            .cmp(&(&b.left, &b.rel, &b.right, b.layer, b.plane, b.anchor, b.weight, b.flags))
    });
    states.sort_by(|a, b| {
        (&a.target, &a.state, a.flux, a.gate, a.phase)
            .cmp(&(&b.target, &b.state, b.flux, b.gate, b.phase))
    });

    let id_to_symbol: Vec<String> = symbols.iter().cloned().collect();
    let symbol_to_id: HashMap<String, u32> = id_to_symbol.iter()
        .enumerate().map(|(i, s)| (s.clone(), i as u32)).collect();

    let mut macro_to_edges    = HashMap::<String, Vec<u32>>::new();
    let mut left_to_edges     = HashMap::<String, Vec<u32>>::new();
    let mut right_to_edges    = HashMap::<String, Vec<u32>>::new();
    let mut target_to_states  = HashMap::<String, Vec<u32>>::new();
    let mut anchor_edges      = Vec::<(u32, u32)>::new();

    for (i, e) in edges.iter().enumerate() {
        let id = i as u32;
        macro_to_edges.entry(e.rel.clone()).or_default().push(id);
        left_to_edges.entry(e.left.clone()).or_default().push(id);
        right_to_edges.entry(e.right.clone()).or_default().push(id);
        anchor_edges.push((e.anchor, id));
    }
    anchor_edges.sort();

    for (i, s) in states.iter().enumerate() {
        target_to_states.entry(s.target.clone()).or_default().push(i as u32);
    }

    let mut idx = ArtifactIndex {
        source_path: source_path.to_string(),
        symbol_to_id, id_to_symbol,
        macro_to_edges, left_to_edges, right_to_edges, target_to_states,
        anchor_edges, edges, states,
        stats: IndexStats {
            input_lines, normalized_lines: normalized.len(),
            comment_lines_stripped, duplicate_lines_removed,
            symbols: symbols.len(), macros: macros.len(),
            edges: 0, states: 0, index_bytes: 0,
        },
    };
    idx.stats.edges  = idx.edges.len();
    idx.stats.states = idx.states.len();
    idx.stats.index_bytes = serde_json::to_vec(&idx).map(|v| v.len()).unwrap_or(0);
    idx
}

// ── JSON I/O (compact) ────────────────────────────────────────────────────

pub fn write_index_json(idx: &ArtifactIndex, path: &Path) -> Result<(), String> {
    let body = serde_json::to_vec(idx).map_err(|e| e.to_string())?;
    fs::write(path, body).map_err(|e| e.to_string())
}

pub fn read_index_json(path: &Path) -> Result<ArtifactIndex, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

// ── Binary frame I/O ──────────────────────────────────────────────────────
//
// Format (all LE):
//   magic[8] + sym_count[4] + edge_count[4] + state_count[4]
//   symbol table: sym_count × (len[2] + bytes[len])
//   edge table:   edge_count × (left_id[2] + rel_id[2] + right_id[2] +
//                               layer[1] + plane[1] + anchor[4] + weight[2] + flags[1]) = 15 bytes
//   state table:  state_count × (target_id[2] + state_sym_id[2] + flux[2] + gate[1] + phase[1]) = 8 bytes
//   --- adjacency tables (3: left, right, rel) ---
//   For each table: n_keys[4], then n_keys × (sym_id[2] + offset[4] + len[4]),
//                   then flat edge_id list (4 bytes each)
//   --- anchor index ---
//   n_anchors[4], then n_anchors × (anchor[4] + edge_id[4])

const MAGIC: &[u8; 8] = b"NSQIDX01";

fn put_u16(buf: &mut Vec<u8>, v: u16) { buf.extend_from_slice(&v.to_le_bytes()); }
fn put_u32(buf: &mut Vec<u8>, v: u32) { buf.extend_from_slice(&v.to_le_bytes()); }

fn get_u16(cur: &mut Cursor<&[u8]>) -> io::Result<u16> {
    let mut b = [0u8; 2]; cur.read_exact(&mut b)?; Ok(u16::from_le_bytes(b))
}
fn get_u32(cur: &mut Cursor<&[u8]>) -> io::Result<u32> {
    let mut b = [0u8; 4]; cur.read_exact(&mut b)?; Ok(u32::from_le_bytes(b))
}

fn write_adj_table(buf: &mut Vec<u8>, map: &HashMap<String, Vec<u32>>, sym_to_id: &HashMap<String, u32>) {
    // Only write entries for symbols that are in sym_to_id
    let mut entries: Vec<(u16, &Vec<u32>)> = map.iter()
        .filter_map(|(k, v)| sym_to_id.get(k).map(|&id| (id as u16, v)))
        .collect();
    entries.sort_by_key(|(id, _)| *id);

    put_u32(buf, entries.len() as u32);
    let flat_start_pos = buf.len() + entries.len() * 10; // will fill offsets below

    // Write header first (with placeholder offsets), then flat array
    let mut headers_start = buf.len();
    for _ in &entries {
        put_u16(buf, 0); put_u32(buf, 0); put_u32(buf, 0); // sym_id, offset, len placeholders
    }
    let mut flat: Vec<u8> = Vec::new();
    let mut offset = 0u32;
    for (i, (sym_id, ids)) in entries.iter().enumerate() {
        let len = ids.len() as u32;
        // patch header
        let h = headers_start + i * 10;
        buf[h..h+2].copy_from_slice(&sym_id.to_le_bytes());
        buf[h+2..h+6].copy_from_slice(&offset.to_le_bytes());
        buf[h+6..h+10].copy_from_slice(&len.to_le_bytes());
        for &id in *ids {
            flat.extend_from_slice(&id.to_le_bytes());
        }
        offset += len;
    }
    buf.extend_from_slice(&flat);
}

fn read_adj_table(cur: &mut Cursor<&[u8]>, id_to_symbol: &[String]) -> io::Result<HashMap<String, Vec<u32>>> {
    let n = get_u32(cur)? as usize;
    let mut headers = Vec::with_capacity(n);
    for _ in 0..n {
        let sym_id = get_u16(cur)? as usize;
        let offset = get_u32(cur)? as usize;
        let len    = get_u32(cur)? as usize;
        headers.push((sym_id, offset, len));
    }
    // Read flat edge-id array
    let total_ids: usize = headers.iter().map(|(_, _, l)| l).sum();
    let mut flat = vec![0u8; total_ids * 4];
    cur.read_exact(&mut flat)?;

    let mut map = HashMap::with_capacity(n);
    for (sym_id, offset, len) in headers {
        let sym = id_to_symbol.get(sym_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "bad sym_id"))?;
        let slice = &flat[offset * 4..(offset + len) * 4];
        let ids: Vec<u32> = slice.chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        map.insert(sym.clone(), ids);
    }
    Ok(map)
}

pub fn write_index_binary(idx: &ArtifactIndex, path: &Path) -> Result<usize, String> {
    let mut buf = Vec::with_capacity(64 * 1024);

    buf.extend_from_slice(MAGIC);
    put_u32(&mut buf, idx.id_to_symbol.len() as u32);
    put_u32(&mut buf, idx.edges.len() as u32);
    put_u32(&mut buf, idx.states.len() as u32);

    // Symbol table
    for sym in &idx.id_to_symbol {
        let b = sym.as_bytes();
        put_u16(&mut buf, b.len() as u16);
        buf.extend_from_slice(b);
    }

    // Build rel symbol table (macros not in id_to_symbol necessarily)
    // We intern rel strings into id_to_symbol if present, else store inline
    // For binary format: edge left/right use sym_to_id; rel stored as u16 from a separate rel table
    // Simple approach: build rel_to_id from macro_to_edges keys
    let mut rel_order: Vec<String> = idx.macro_to_edges.keys().cloned().collect();
    rel_order.sort();
    let rel_to_id: HashMap<String, u16> = rel_order.iter()
        .enumerate().map(|(i, s)| (s.clone(), i as u16)).collect();
    put_u16(&mut buf, rel_order.len() as u16);
    for rel in &rel_order {
        let b = rel.as_bytes();
        put_u16(&mut buf, b.len() as u16);
        buf.extend_from_slice(b);
    }

    // Edge table: 15 bytes per edge
    for e in &idx.edges {
        let left_id  = idx.symbol_to_id.get(&e.left).copied().unwrap_or(0) as u16;
        let right_id = idx.symbol_to_id.get(&e.right).copied().unwrap_or(0) as u16;
        let rel_id   = rel_to_id.get(&e.rel).copied().unwrap_or(0);
        put_u16(&mut buf, left_id);
        put_u16(&mut buf, rel_id);
        put_u16(&mut buf, right_id);
        buf.push(e.layer);
        buf.push(e.plane);
        put_u32(&mut buf, e.anchor);
        put_u16(&mut buf, e.weight);
        buf.push(e.flags);
    }

    // State table: 8 bytes per state
    for s in &idx.states {
        let target_id   = idx.symbol_to_id.get(&s.target).copied().unwrap_or(0) as u16;
        let state_sym   = idx.symbol_to_id.get(&s.state).copied().unwrap_or(u32::MAX) as u16;
        put_u16(&mut buf, target_id);
        put_u16(&mut buf, state_sym);
        put_u16(&mut buf, s.flux);
        buf.push(s.gate);
        buf.push(s.phase);
    }

    // Adjacency tables (left, right)
    // For binary, use sym id → [edge_id] directly
    let mut left_adj: Vec<(u32, Vec<u32>)> = idx.left_to_edges.iter()
        .filter_map(|(k, v)| idx.symbol_to_id.get(k).map(|&id| (id, v.clone())))
        .collect();
    left_adj.sort_by_key(|(id, _)| *id);
    put_u32(&mut buf, left_adj.len() as u32);
    let mut flat_left: Vec<u8> = Vec::new();
    let mut offset = 0u32;
    let mut la_hdr_start = buf.len();
    for _ in &left_adj { put_u32(&mut buf, 0); put_u32(&mut buf, 0); put_u32(&mut buf, 0); }
    for (i, (sym_id, ids)) in left_adj.iter().enumerate() {
        let h = la_hdr_start + i * 12;
        buf[h..h+4].copy_from_slice(&sym_id.to_le_bytes());
        buf[h+4..h+8].copy_from_slice(&offset.to_le_bytes());
        buf[h+8..h+12].copy_from_slice(&(ids.len() as u32).to_le_bytes());
        for &id in ids { flat_left.extend_from_slice(&id.to_le_bytes()); }
        offset += ids.len() as u32;
    }
    buf.extend_from_slice(&flat_left);

    // Anchor index
    put_u32(&mut buf, idx.anchor_edges.len() as u32);
    for &(anchor, edge_id) in &idx.anchor_edges {
        put_u32(&mut buf, anchor);
        put_u32(&mut buf, edge_id);
    }

    let n = buf.len();
    fs::write(path, &buf).map_err(|e| e.to_string())?;
    Ok(n)
}

pub fn read_index_binary(path: &Path) -> Result<ArtifactIndex, String> {
    let raw = fs::read(path).map_err(|e| e.to_string())?;
    let mut cur = Cursor::new(raw.as_slice());

    let mut magic = [0u8; 8];
    cur.read_exact(&mut magic).map_err(|e| e.to_string())?;
    if &magic != MAGIC { return Err("bad magic".to_string()); }

    let sym_count   = get_u32(&mut cur).map_err(|e| e.to_string())? as usize;
    let edge_count  = get_u32(&mut cur).map_err(|e| e.to_string())? as usize;
    let state_count = get_u32(&mut cur).map_err(|e| e.to_string())? as usize;

    // Symbol table
    let mut id_to_symbol = Vec::with_capacity(sym_count);
    for _ in 0..sym_count {
        let len = get_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let mut b = vec![0u8; len];
        cur.read_exact(&mut b).map_err(|e| e.to_string())?;
        id_to_symbol.push(String::from_utf8_lossy(&b).to_string());
    }
    let symbol_to_id: HashMap<String, u32> = id_to_symbol.iter()
        .enumerate().map(|(i, s)| (s.clone(), i as u32)).collect();

    // Rel table
    let rel_count = get_u16(&mut cur).map_err(|e| e.to_string())? as usize;
    let mut id_to_rel = Vec::with_capacity(rel_count);
    for _ in 0..rel_count {
        let len = get_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let mut b = vec![0u8; len];
        cur.read_exact(&mut b).map_err(|e| e.to_string())?;
        id_to_rel.push(String::from_utf8_lossy(&b).to_string());
    }

    // Edge table
    let mut edges = Vec::with_capacity(edge_count);
    for _ in 0..edge_count {
        let left_id  = get_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let rel_id   = get_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let right_id = get_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let layer    = { let mut b = [0u8;1]; cur.read_exact(&mut b).map_err(|e| e.to_string())?; b[0] };
        let plane    = { let mut b = [0u8;1]; cur.read_exact(&mut b).map_err(|e| e.to_string())?; b[0] };
        let anchor   = get_u32(&mut cur).map_err(|e| e.to_string())?;
        let weight   = get_u16(&mut cur).map_err(|e| e.to_string())?;
        let flags    = { let mut b = [0u8;1]; cur.read_exact(&mut b).map_err(|e| e.to_string())?; b[0] };
        edges.push(Edge {
            left:  id_to_symbol.get(left_id).cloned().unwrap_or_default(),
            rel:   id_to_rel.get(rel_id).cloned().unwrap_or_default(),
            right: id_to_symbol.get(right_id).cloned().unwrap_or_default(),
            layer, plane, anchor, weight, flags,
        });
    }

    // State table
    let mut states = Vec::with_capacity(state_count);
    for _ in 0..state_count {
        let target_id = get_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let state_id  = get_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let flux  = get_u16(&mut cur).map_err(|e| e.to_string())?;
        let gate  = { let mut b = [0u8;1]; cur.read_exact(&mut b).map_err(|e| e.to_string())?; b[0] };
        let phase = { let mut b = [0u8;1]; cur.read_exact(&mut b).map_err(|e| e.to_string())?; b[0] };
        states.push(State {
            target: id_to_symbol.get(target_id).cloned().unwrap_or_default(),
            state:  id_to_symbol.get(state_id).cloned().unwrap_or("unknown".to_string()),
            flux, gate, phase,
        });
    }

    // Left adjacency
    let n_left = get_u32(&mut cur).map_err(|e| e.to_string())? as usize;
    let mut left_hdrs = Vec::with_capacity(n_left);
    for _ in 0..n_left {
        let sym_id = get_u32(&mut cur).map_err(|e| e.to_string())? as usize;
        let offset = get_u32(&mut cur).map_err(|e| e.to_string())? as usize;
        let len    = get_u32(&mut cur).map_err(|e| e.to_string())? as usize;
        left_hdrs.push((sym_id, offset, len));
    }
    let total_left_ids: usize = left_hdrs.iter().map(|(_, _, l)| l).sum();
    let mut flat_left = vec![0u8; total_left_ids * 4];
    cur.read_exact(&mut flat_left).map_err(|e| e.to_string())?;
    let mut left_to_edges = HashMap::with_capacity(n_left);
    for (sym_id, offset, len) in left_hdrs {
        let sym = id_to_symbol.get(sym_id).cloned().unwrap_or_default();
        let ids: Vec<u32> = flat_left[offset*4..(offset+len)*4]
            .chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        left_to_edges.insert(sym, ids);
    }

    // Anchor index
    let n_anchors = get_u32(&mut cur).map_err(|e| e.to_string())? as usize;
    let mut anchor_edges = Vec::with_capacity(n_anchors);
    for _ in 0..n_anchors {
        let anchor  = get_u32(&mut cur).map_err(|e| e.to_string())?;
        let edge_id = get_u32(&mut cur).map_err(|e| e.to_string())?;
        anchor_edges.push((anchor, edge_id));
    }

    // Rebuild right and macro adjacency from edges (not stored in binary to save space)
    let mut right_to_edges  = HashMap::<String, Vec<u32>>::new();
    let mut macro_to_edges  = HashMap::<String, Vec<u32>>::new();
    let mut target_to_states = HashMap::<String, Vec<u32>>::new();
    for (i, e) in edges.iter().enumerate() {
        right_to_edges.entry(e.right.clone()).or_default().push(i as u32);
        macro_to_edges.entry(e.rel.clone()).or_default().push(i as u32);
    }
    for (i, s) in states.iter().enumerate() {
        target_to_states.entry(s.target.clone()).or_default().push(i as u32);
    }

    Ok(ArtifactIndex {
        source_path: path.display().to_string(),
        symbol_to_id, id_to_symbol,
        macro_to_edges, left_to_edges, right_to_edges, target_to_states,
        anchor_edges, edges, states,
        stats: IndexStats {
            input_lines: 0, normalized_lines: 0, comment_lines_stripped: 0,
            duplicate_lines_removed: 0,
            symbols: sym_count, macros: rel_count,
            edges: edge_count, states: state_count,
            index_bytes: raw.len(),
        },
    })
}

// ── shortest path (BFS) ────────────────────────────────────────────────────

pub fn shortest_path(idx: &ArtifactIndex, src: &str, dst: &str, depth: usize) -> Option<Vec<String>> {
    if src == dst { return Some(vec![src.to_string()]); }
    let mut q    = std::collections::VecDeque::<(String, Vec<String>)>::new();
    let mut seen = std::collections::HashSet::<String>::new();
    q.push_back((src.to_string(), vec![src.to_string()]));
    seen.insert(src.to_string());
    while let Some((cur, path)) = q.pop_front() {
        if path.len() > depth + 1 { continue; }
        if let Some(edge_ids) = idx.left_to_edges.get(&cur) {
            for &edge_id in edge_ids {
                let next = idx.edges[edge_id as usize].right.clone();
                if seen.contains(&next) { continue; }
                let mut np = path.clone();
                np.push(next.clone());
                if next == dst { return Some(np); }
                seen.insert(next.clone());
                q.push_back((next, np));
            }
        }
    }
    None
}

// ── anchor binary search ───────────────────────────────────────────────────

pub fn anchors_in_range<'a>(idx: &'a ArtifactIndex, min: u32, max: u32) -> Vec<&'a Edge> {
    let lo = idx.anchor_edges.partition_point(|&(a, _)| a < min);
    let hi = idx.anchor_edges.partition_point(|&(a, _)| a <= max);
    idx.anchor_edges[lo..hi].iter()
        .map(|&(_, id)| &idx.edges[id as usize])
        .collect()
}
RUST

echo "nsq-index/src/lib.rs written"

# ══════════════════════════════════════════════════════════════════════════════
# 2. nsq-query/src/main.rs — batch mode: load once, run all queries in-process
# ══════════════════════════════════════════════════════════════════════════════
cat > crates/nsq-query/src/main.rs << 'RUST'
//! nsq-query — single query or batch mode.
//!
//! Usage:
//!   nsq-query <index.idx.json|index.idx.bin> <query>
//!   nsq-query <index.idx.json|index.idx.bin> --batch <queries.txt>
//!   nsq-query <index.idx.json|index.idx.bin> --batch-json <queries.json>
//!
//! --batch:      one query per line in a plain text file, results as JSON array
//! --batch-json: JSON array of query strings, results as JSON array
//!
//! The key benefit of batch mode: the index is loaded exactly once.
//! All queries run in-process before the process exits.
//! This eliminates the cold-start overhead that was dominating query_ms_mean.

use nsq_index::{anchors_in_range, read_index_binary, read_index_json, shortest_path, ArtifactIndex};
use nsq_query::{
    edges_left, edges_rel, edges_right, find_rel, find_symbol, neighbors,
    states_target, QueryResult,
};
use serde::Serialize;
use serde_json::json;
use std::path::Path;
use std::time::Instant;
use std::{env, fs, process};

#[derive(Serialize)]
struct BatchReport {
    index_path:     String,
    index_load_ms:  f64,
    query_count:    usize,
    total_query_ms: f64,
    mean_query_ms:  f64,
    results:        Vec<QueryResult>,
}

fn load_index(path: &str) -> ArtifactIndex {
    let p = Path::new(path);
    if path.ends_with(".bin") {
        read_index_binary(p).unwrap_or_else(|e| {
            eprintln!("load binary index error: {e}");
            process::exit(2);
        })
    } else {
        read_index_json(p).unwrap_or_else(|e| {
            eprintln!("load json index error: {e}");
            process::exit(2);
        })
    }
}

fn run_query(idx: &ArtifactIndex, q: &str) -> QueryResult {
    let toks: Vec<&str> = q.split_whitespace().collect();
    if toks.is_empty() {
        return QueryResult { command: q.to_string(), matches: json!(null) };
    }
    match (toks[0], toks.get(1).copied(), toks.get(2).copied()) {
        ("find", Some("symbol"), Some(name)) => find_symbol(idx, name),
        ("find", Some("rel"),    Some(name)) => find_rel(idx, name),
        ("neighbors", Some(name), _)         => neighbors(idx, name),
        ("edges", ..) => {
            // "edges left=X" or "edges right=X" or "edges rel=X"
            if let Some(arg) = toks.get(1) {
                if let Some(name) = arg.strip_prefix("left=")  { return edges_left(idx, name);  }
                if let Some(name) = arg.strip_prefix("right=") { return edges_right(idx, name); }
                if let Some(name) = arg.strip_prefix("rel=")   { return edges_rel(idx, name);   }
            }
            QueryResult { command: q.to_string(), matches: json!({ "error": "bad edges syntax" }) }
        }
        ("states", ..) => {
            if let Some(arg) = toks.get(1) {
                if let Some(name) = arg.strip_prefix("target=") {
                    return states_target(idx, name);
                }
            }
            QueryResult { command: q.to_string(), matches: json!({ "error": "bad states syntax" }) }
        }
        ("anchors", ..) => {
            let min = toks.iter().find_map(|t| t.strip_prefix("min=")).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
            let max = toks.iter().find_map(|t| t.strip_prefix("max=")).and_then(|s| s.parse::<u32>().ok()).unwrap_or(u32::MAX);
            let rows = anchors_in_range(idx, min, max);
            QueryResult { command: q.to_string(), matches: serde_json::to_value(rows).unwrap() }
        }
        ("path", Some(src), Some(dst)) => {
            let depth = toks.iter().find_map(|t| t.strip_prefix("depth=")).and_then(|s| s.parse::<usize>().ok()).unwrap_or(6);
            let path  = shortest_path(idx, src, dst, depth);
            QueryResult { command: q.to_string(), matches: json!({ "path": path }) }
        }
        _ => QueryResult { command: q.to_string(), matches: json!({ "error": "unknown query" }) },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: nsq-query <index_path> <query>");
        eprintln!("       nsq-query <index_path> --batch <queries.txt>");
        eprintln!("       nsq-query <index_path> --batch-json <queries.json>");
        process::exit(2);
    }

    let index_path = &args[1];
    let t_load = Instant::now();
    let idx    = load_index(index_path);
    let load_ms = t_load.elapsed().as_secs_f64() * 1000.0;

    match args[2].as_str() {
        "--batch" | "--batch-json" => {
            let queries: Vec<String> = if args[2] == "--batch-json" {
                let raw = fs::read_to_string(&args[3]).unwrap_or_else(|e| {
                    eprintln!("read {}: {e}", args[3]);
                    process::exit(2);
                });
                serde_json::from_str::<Vec<String>>(&raw).unwrap_or_else(|e| {
                    eprintln!("parse json queries: {e}");
                    process::exit(2);
                })
            } else {
                let raw = fs::read_to_string(&args[3]).unwrap_or_else(|e| {
                    eprintln!("read {}: {e}", args[3]);
                    process::exit(2);
                });
                raw.lines().filter(|l| !l.trim().is_empty()).map(|l| l.to_string()).collect()
            };

            let t_queries = Instant::now();
            let results: Vec<QueryResult> = queries.iter().map(|q| run_query(&idx, q)).collect();
            let total_ms = t_queries.elapsed().as_secs_f64() * 1000.0;

            let report = BatchReport {
                index_path: index_path.clone(),
                index_load_ms: load_ms,
                query_count: results.len(),
                total_query_ms: total_ms,
                mean_query_ms:  if results.is_empty() { 0.0 } else { total_ms / results.len() as f64 },
                results,
            };
            println!("{}", serde_json::to_string_pretty(&report).unwrap());
        }
        query => {
            let r = run_query(&idx, query);
            println!("{}", serde_json::to_string_pretty(&r).unwrap());
        }
    }
}
RUST

echo "nsq-query/src/main.rs written"

# ══════════════════════════════════════════════════════════════════════════════
# 3. nsq-query/src/lib.rs — updated to use anchors_in_range from index
# ══════════════════════════════════════════════════════════════════════════════
cat > crates/nsq-query/src/lib.rs << 'RUST'
use nsq_index::{ArtifactIndex, Edge, State, anchors_in_range};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub command: String,
    pub matches: serde_json::Value,
}

pub fn find_symbol(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let id = idx.symbol_to_id.get(name).copied();
    QueryResult {
        command: format!("find symbol {}", name),
        matches: serde_json::json!({ "symbol": name, "id": id, "exists": id.is_some() }),
    }
}

pub fn find_rel(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let count = idx.macro_to_edges.get(name).map(|v| v.len()).unwrap_or(0);
    QueryResult {
        command: format!("find rel {}", name),
        matches: serde_json::json!({ "rel": name, "count": count }),
    }
}

pub fn neighbors(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let mut out: Vec<&str> = idx.left_to_edges.get(name).into_iter()
        .flatten().map(|&id| idx.edges[id as usize].right.as_str()).collect();
    out.sort_unstable();
    out.dedup();
    QueryResult {
        command: format!("neighbors {}", name),
        matches: serde_json::json!({ "neighbors": out }),
    }
}

pub fn edges_left(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let rows: Vec<&Edge> = idx.left_to_edges.get(name).into_iter()
        .flatten().map(|&id| &idx.edges[id as usize]).collect();
    QueryResult { command: format!("edges left={}", name), matches: serde_json::to_value(rows).unwrap() }
}

pub fn edges_right(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let rows: Vec<&Edge> = idx.right_to_edges.get(name).into_iter()
        .flatten().map(|&id| &idx.edges[id as usize]).collect();
    QueryResult { command: format!("edges right={}", name), matches: serde_json::to_value(rows).unwrap() }
}

pub fn edges_rel(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let rows: Vec<&Edge> = idx.macro_to_edges.get(name).into_iter()
        .flatten().map(|&id| &idx.edges[id as usize]).collect();
    QueryResult { command: format!("edges rel={}", name), matches: serde_json::to_value(rows).unwrap() }
}

pub fn states_target(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let rows: Vec<&State> = idx.target_to_states.get(name).into_iter()
        .flatten().map(|&id| &idx.states[id as usize]).collect();
    QueryResult { command: format!("states target={}", name), matches: serde_json::to_value(rows).unwrap() }
}
RUST

echo "nsq-query/src/lib.rs written"

# ══════════════════════════════════════════════════════════════════════════════
# 4. nsq-query Cargo.toml — ensure lib declared
# ══════════════════════════════════════════════════════════════════════════════
cat > crates/nsq-query/Cargo.toml << 'TOML'
[package]
name = "nsq-query"
version = "0.1.0"
edition = "2021"

[lib]
name = "nsq_query"
path = "src/lib.rs"

[[bin]]
name = "nsq-query"
path = "src/main.rs"

[dependencies]
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
nsq-index  = { path = "../nsq-index" }
TOML

# ══════════════════════════════════════════════════════════════════════════════
# 5. nsq-bench-split — three clearly labelled benchmark modes
# ══════════════════════════════════════════════════════════════════════════════
cat > crates/nsq-bench-split/Cargo.toml << 'TOML'
[package]
name = "nsq-bench-split"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "nsq-bench-split"
path = "src/main.rs"

[dependencies]
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
nsq-index  = { path = "../nsq-index" }
nsq-query  = { path = "../nsq-query" }
TOML

cat > crates/nsq-bench-split/src/main.rs << 'RUST'
//! nsq-bench-split — three honest benchmark modes.
//!
//! Usage:
//!   nsq-bench-split core   <corpus.nsq> <queries.json> [iters]
//!   nsq-bench-split cold   <corpus.nsq> <index.idx.json> <queries.json> [iters]
//!   nsq-bench-split warm   <index.idx.bin> <queries.json> [iters]
//!
//! Mode: CORE
//!   What: pure in-process index build + query, no file I/O on the query side.
//!   Measures: How fast is the index data structure itself?
//!   Honest answer to: "what does NSQ's in-memory graph cost per query?"
//!
//! Mode: COLD
//!   What: full CLI-equivalent pipeline: read corpus → build index → write index
//!         → read index back from disk (JSON) → run queries.
//!   Measures: Full end-to-end cold-start latency including disk round-trip.
//!   Honest answer to: "what does a user actually experience?"
//!
//! Mode: WARM
//!   What: Load a pre-built binary index from disk → run queries.
//!   Measures: Binary format load speed + query speed with minimal startup cost.
//!   Honest answer to: "what does the system feel like after first-build warm cache?"

use nsq_index::{
    anchors_in_range, build_index_from_text, read_index_binary, read_index_json,
    write_index_binary, write_index_json, ArtifactIndex,
};
use nsq_query::{edges_left, find_rel, find_symbol, neighbors, states_target};
use serde::Serialize;
use std::path::Path;
use std::time::Instant;
use std::{env, fs, process};

#[derive(Serialize)]
struct SplitResult {
    mode:            String,
    corpus_bytes:    Option<u64>,
    index_bytes_json: Option<u64>,
    index_bytes_bin:  Option<u64>,
    iters:           usize,
    // CORE timings (in-process)
    build_ms_mean:   Option<f64>,
    query_ms_mean:   Option<f64>,
    query_ms_total:  Option<f64>,
    // COLD timings (disk round-trip)
    cold_build_ms:   Option<f64>,
    cold_write_ms:   Option<f64>,
    cold_read_ms:    Option<f64>,
    cold_query_ms:   Option<f64>,
    // WARM timings (binary load + query)
    warm_load_ms:    Option<f64>,
    warm_query_ms:   Option<f64>,
    // Query stats
    query_count:     usize,
    symbols:         usize,
    edges:           usize,
    states:          usize,
}

fn load_queries(path: &str) -> Vec<String> {
    let raw = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("read queries {path}: {e}");
        process::exit(2);
    });
    serde_json::from_str::<Vec<String>>(&raw).unwrap_or_else(|e| {
        eprintln!("parse queries: {e}");
        process::exit(2);
    })
}

fn run_queries_inprocess(idx: &ArtifactIndex, queries: &[String]) {
    for q in queries {
        let toks: Vec<&str> = q.split_whitespace().collect();
        match (toks.first().copied(), toks.get(1).copied(), toks.get(2).copied()) {
            (Some("find"), Some("symbol"), Some(n)) => { let _ = find_symbol(idx, n); }
            (Some("find"), Some("rel"),    Some(n)) => { let _ = find_rel(idx, n); }
            (Some("neighbors"), Some(n),   _)       => { let _ = neighbors(idx, n); }
            (Some("edges"), Some(arg), _) => {
                if let Some(n) = arg.strip_prefix("left=")   { let _ = edges_left(idx, n); }
                else if let Some(n) = arg.strip_prefix("rel=") { let _ = find_rel(idx, n); }
            }
            (Some("states"), Some(arg), _) => {
                if let Some(n) = arg.strip_prefix("target=") { let _ = states_target(idx, n); }
            }
            (Some("anchors"), ..) => {
                let min = toks.iter().find_map(|t| t.strip_prefix("min=")).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
                let max = toks.iter().find_map(|t| t.strip_prefix("max=")).and_then(|s| s.parse::<u32>().ok()).unwrap_or(u32::MAX);
                let _ = anchors_in_range(idx, min, max);
            }
            _ => {}
        }
    }
}

fn bench_core(corpus_path: &str, queries_path: &str, iters: usize) -> SplitResult {
    let corpus = fs::read_to_string(corpus_path).unwrap_or_else(|e| {
        eprintln!("read corpus: {e}"); process::exit(2);
    });
    let corpus_bytes = corpus.len() as u64;
    let queries = load_queries(queries_path);

    // Warmup
    let idx0 = build_index_from_text(corpus_path, &corpus);
    run_queries_inprocess(&idx0, &queries);

    // Time build
    let t_build = Instant::now();
    let mut last_idx = idx0.clone();
    for _ in 0..iters {
        last_idx = build_index_from_text(corpus_path, &corpus);
        std::hint::black_box(&last_idx);
    }
    let build_ms = t_build.elapsed().as_secs_f64() * 1000.0 / iters as f64;

    // Time queries (all in-process, no I/O)
    let t_q = Instant::now();
    for _ in 0..iters {
        run_queries_inprocess(&last_idx, &queries);
    }
    let query_ms_total = t_q.elapsed().as_secs_f64() * 1000.0 / iters as f64;
    let query_ms_mean  = if queries.is_empty() { 0.0 } else { query_ms_total / queries.len() as f64 };

    SplitResult {
        mode: "core".to_string(),
        corpus_bytes: Some(corpus_bytes),
        index_bytes_json: None, index_bytes_bin: None,
        iters,
        build_ms_mean: Some(build_ms),
        query_ms_mean: Some(query_ms_mean),
        query_ms_total: Some(query_ms_total),
        cold_build_ms: None, cold_write_ms: None, cold_read_ms: None, cold_query_ms: None,
        warm_load_ms: None, warm_query_ms: None,
        query_count: queries.len(),
        symbols: last_idx.stats.symbols,
        edges:   last_idx.stats.edges,
        states:  last_idx.stats.states,
    }
}

fn bench_cold(corpus_path: &str, index_json_path: &str, queries_path: &str, iters: usize) -> SplitResult {
    let corpus = fs::read_to_string(corpus_path).unwrap_or_else(|e| {
        eprintln!("read corpus: {e}"); process::exit(2);
    });
    let corpus_bytes = corpus.len() as u64;
    let queries = load_queries(queries_path);
    let json_path = Path::new(index_json_path);

    let mut cold_build = 0.0f64;
    let mut cold_write = 0.0f64;
    let mut cold_read  = 0.0f64;
    let mut cold_query = 0.0f64;
    let mut last_symbols = 0;
    let mut last_edges   = 0;
    let mut last_states  = 0;
    let mut json_bytes   = 0u64;

    for _ in 0..iters {
        let t = Instant::now();
        let idx = build_index_from_text(corpus_path, &corpus);
        cold_build += t.elapsed().as_secs_f64() * 1000.0;

        let t = Instant::now();
        write_index_json(&idx, json_path).unwrap_or_else(|e| {
            eprintln!("write index: {e}"); process::exit(2);
        });
        cold_write += t.elapsed().as_secs_f64() * 1000.0;
        json_bytes = fs::metadata(json_path).map(|m| m.len()).unwrap_or(0);

        let t = Instant::now();
        let idx2 = read_index_json(json_path).unwrap_or_else(|e| {
            eprintln!("read index: {e}"); process::exit(2);
        });
        cold_read += t.elapsed().as_secs_f64() * 1000.0;

        let t = Instant::now();
        run_queries_inprocess(&idx2, &queries);
        cold_query += t.elapsed().as_secs_f64() * 1000.0;

        last_symbols = idx2.stats.symbols;
        last_edges   = idx2.stats.edges;
        last_states  = idx2.stats.states;
    }

    SplitResult {
        mode: "cold".to_string(),
        corpus_bytes: Some(corpus_bytes),
        index_bytes_json: Some(json_bytes),
        index_bytes_bin: None,
        iters,
        build_ms_mean: None, query_ms_mean: None, query_ms_total: None,
        cold_build_ms: Some(cold_build / iters as f64),
        cold_write_ms: Some(cold_write / iters as f64),
        cold_read_ms:  Some(cold_read  / iters as f64),
        cold_query_ms: Some(cold_query / iters as f64),
        warm_load_ms: None, warm_query_ms: None,
        query_count: queries.len(),
        symbols: last_symbols, edges: last_edges, states: last_states,
    }
}

fn bench_warm(index_bin_path: &str, queries_path: &str, iters: usize) -> SplitResult {
    let queries = load_queries(queries_path);
    let bin_path = Path::new(index_bin_path);
    let bin_bytes = fs::metadata(bin_path).map(|m| m.len()).unwrap_or(0);

    let mut warm_load  = 0.0f64;
    let mut warm_query = 0.0f64;
    let mut last_symbols = 0;
    let mut last_edges   = 0;
    let mut last_states  = 0;

    for _ in 0..iters {
        let t = Instant::now();
        let idx = read_index_binary(bin_path).unwrap_or_else(|e| {
            eprintln!("load binary: {e}"); process::exit(2);
        });
        warm_load += t.elapsed().as_secs_f64() * 1000.0;

        let t = Instant::now();
        run_queries_inprocess(&idx, &queries);
        warm_query += t.elapsed().as_secs_f64() * 1000.0;

        last_symbols = idx.stats.symbols;
        last_edges   = idx.stats.edges;
        last_states  = idx.stats.states;
    }

    SplitResult {
        mode: "warm".to_string(),
        corpus_bytes: None,
        index_bytes_json: None,
        index_bytes_bin: Some(bin_bytes),
        iters,
        build_ms_mean: None, query_ms_mean: None, query_ms_total: None,
        cold_build_ms: None, cold_write_ms: None, cold_read_ms: None, cold_query_ms: None,
        warm_load_ms:  Some(warm_load  / iters as f64),
        warm_query_ms: Some(warm_query / iters as f64),
        query_count: queries.len(),
        symbols: last_symbols, edges: last_edges, states: last_states,
    }
}

fn usage() -> ! {
    eprintln!("nsq-bench-split core  <corpus.nsq> <queries.json> [iters=10]");
    eprintln!("nsq-bench-split cold  <corpus.nsq> <index.idx.json> <queries.json> [iters=5]");
    eprintln!("nsq-bench-split warm  <index.idx.bin> <queries.json> [iters=10]");
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { usage(); }

    let result = match args[1].as_str() {
        "core" => {
            if args.len() < 4 { usage(); }
            let iters = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(10);
            bench_core(&args[2], &args[3], iters)
        }
        "cold" => {
            if args.len() < 5 { usage(); }
            let iters = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(5);
            bench_cold(&args[2], &args[3], &args[4], iters)
        }
        "warm" => {
            if args.len() < 4 { usage(); }
            let iters = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(10);
            bench_warm(&args[2], &args[3], iters)
        }
        _ => usage(),
    };

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
RUST

echo "nsq-bench-split written"

# ══════════════════════════════════════════════════════════════════════════════
# 6. Run script for the complete split suite
# ══════════════════════════════════════════════════════════════════════════════
cat > "$HOME/bin/nsq-run-split-bench" << 'RUN_EOF'
#!/data/data/com.termux/files/usr/bin/bash
# nsq-run-split-bench — generates corpus, builds binary indexes, runs all three modes
set -euo pipefail

BRAXON_HOME="${BRAXON_HOME:-$HOME/Braxon}"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$BRAXON_HOME/artifacts/split_bench/$STAMP"
CORPUS="$OUT/corpus"
INDEXES="$OUT/indexes"
RESULTS="$OUT/results"

mkdir -p "$CORPUS" "$INDEXES" "$RESULTS"

PROFILES="chain fanout dense mixed duplicate_stress pathological membrane_dense"

# Generate corpus
python3 - <<PY
import pathlib, sys
root = pathlib.Path("$CORPUS")

defs = {
    "chain":            4000,
    "fanout":           4000,
    "dense":            250,
    "mixed":            5000,
    "duplicate_stress": 3000,
    "pathological":     8000,
    "membrane_dense":   6000,
}

queries_by_profile = {
    "chain":            ["find symbol node.0","find rel links","neighbors node.0","anchors min=1000 max=9999"],
    "fanout":           ["find symbol hub.root","find rel links","neighbors hub.root","anchors min=2000 max=6000"],
    "dense":            ["find symbol dense.0","find rel links","neighbors dense.0","anchors min=3000 max=60000"],
    "mixed":            ["find symbol actor.0","find rel owns","neighbors actor.0","states target=actor.0"],
    "duplicate_stress": ["find symbol dup.0","find rel links","neighbors dup.0","states target=dup.0"],
    "pathological":     ["find symbol hot.root","find rel fanout","neighbors hot.root","anchors min=100000 max=200000"],
    "membrane_dense":   ["find symbol msym.0","find rel links","neighbors msym.0","states target=msym.0"],
}

import json
for name, n in defs.items():
    p = root / f"{name}.nsq"
    with p.open("w") as f:
        f.write("# split bench corpus\n")
        if name == "chain":
            for i in range(n):
                f.write(f"triple node.{i} -> links -> node.{(i+1)%n} :layer 1 :plane 1 :anchor {1000+i} :weight 1 :flags 0\n")
                f.write(f"membrane node.{i} :state active :flux 1 :gate 1 :phase 1\n")
        elif name == "fanout":
            for i in range(n):
                f.write(f"triple hub.root -> links -> leaf.{i} :layer 1 :plane 1 :anchor {2000+i} :weight 1 :flags 0\n")
            f.write("membrane hub.root :state active :flux 1 :gate 1 :phase 1\n")
        elif name == "dense":
            limit = n
            for i in range(limit):
                for j in range(limit):
                    if i != j:
                        f.write(f"triple dense.{i} -> links -> dense.{j} :layer 1 :plane 1 :anchor {3000+i*limit+j} :weight 1 :flags 0\n")
        elif name == "mixed":
            for i in range(n):
                f.write(f"triple actor.{i} -> owns -> item.{i} :layer 1 :plane 1 :anchor {4000+i} :weight 2 :flags 1\n")
                f.write(f"triple actor.{i} -> visits -> place.{i%71} :layer 1 :plane 1 :anchor {8000+i} :weight 1 :flags 0\n")
                if i % 3 == 0:
                    f.write(f"membrane actor.{i} :state active :flux 2 :gate 1 :phase 1\n")
        elif name == "duplicate_stress":
            for i in range(n):
                line = f"triple dup.{i%97} -> links -> dup.{(i+1)%97} :layer 1 :plane 1 :anchor {9000+(i%97)} :weight 1 :flags 0\n"
                f.write(line); f.write(line)
            for i in range(97):
                s = f"membrane dup.{i} :state active :flux 1 :gate 1 :phase 1\n"
                f.write(s); f.write(s)
        elif name == "pathological":
            for i in range(n):
                f.write(f"triple hot.root -> fanout -> leaf.{i} :layer 1 :plane 1 :anchor {100000+i} :weight 1 :flags 0\n")
                f.write(f"triple leaf.{i} -> returns -> hot.root :layer 1 :plane 1 :anchor {200000+i} :weight 1 :flags 0\n")
                if i % 5 == 0:
                    f.write(f"membrane leaf.{i} :state active :flux 1 :gate 1 :phase 1\n")
        elif name == "membrane_dense":
            # High state density: every symbol has 3 states, sparse edges
            for i in range(n):
                f.write(f"triple msym.{i} -> links -> msym.{(i+1)%n} :layer 1 :plane 1 :anchor {5000+i} :weight 1 :flags 0\n")
                f.write(f"membrane msym.{i} :state active :flux 1 :gate 1 :phase 0\n")
                f.write(f"membrane msym.{i} :state pending :flux 0 :gate 0 :phase 1\n")
                f.write(f"membrane msym.{i} :state dormant :flux 0 :gate 1 :phase 2\n")

    qpath = root / f"{name}.queries.json"
    qpath.write_text(json.dumps(queries_by_profile[name]))
    print(f"wrote {name}.nsq + {name}.queries.json")
PY

BIN="$BRAXON_HOME/target/release"

echo "building..."
cd "$BRAXON_HOME"
cargo build -p nsq-index -p nsq-query -p nsq-bench-split --release 2>&1 | tail -5

echo ""
echo "=== CORE MODE (pure in-process, no disk I/O on query) ==="
for profile in $PROFILES; do
    out="$RESULTS/${profile}_core.json"
    "$BIN/nsq-bench-split" core "$CORPUS/${profile}.nsq" "$CORPUS/${profile}.queries.json" 10 \
        > "$out" 2>/dev/null
    python3 -c "
import json; d = json.load(open('$out'))
print(f\"  {d['mode']:<8} {d.get('symbols',0):>6} sym  {d.get('edges',0):>7} edges  \
build={d.get('build_ms_mean',0):.1f}ms  \
query_total={d.get('query_ms_total',0):.3f}ms  \
per_query={d.get('query_ms_mean',0)*1000:.1f}µs  profile=$profile\")
"
done

echo ""
echo "=== COLD MODE (disk round-trip: build→write→read→query) ==="
for profile in $PROFILES; do
    out="$RESULTS/${profile}_cold.json"
    "$BIN/nsq-bench-split" cold \
        "$CORPUS/${profile}.nsq" \
        "$INDEXES/${profile}.idx.json" \
        "$CORPUS/${profile}.queries.json" 3 \
        > "$out" 2>/dev/null
    python3 -c "
import json; d = json.load(open('$out'))
print(f\"  {d['mode']:<8} build={d.get('cold_build_ms',0):.1f}ms  \
write={d.get('cold_write_ms',0):.1f}ms  \
read={d.get('cold_read_ms',0):.1f}ms  \
query={d.get('cold_query_ms',0):.3f}ms  profile=$profile\")
"
done

echo ""
echo "=== BUILDING BINARY INDEXES ==="
for profile in $PROFILES; do
    python3 - <<PY
import sys; sys.path.insert(0, '')
import subprocess, json
# Use nsq-index binary to write .bin
# Actually call via a small inline tool
PY
    # Use nsq-index binary directly
    "$BIN/nsq-index" \
        "$CORPUS/${profile}.nsq" \
        "$INDEXES/${profile}.idx.json" \
        "$INDEXES/${profile}.idx.bin" \
        2>/dev/null || true
    if [ -f "$INDEXES/${profile}.idx.bin" ]; then
        bin_sz=$(stat -c %s "$INDEXES/${profile}.idx.bin" 2>/dev/null || stat -f %z "$INDEXES/${profile}.idx.bin")
        json_sz=$(stat -c %s "$INDEXES/${profile}.idx.json" 2>/dev/null || stat -f %z "$INDEXES/${profile}.idx.json")
        echo "  $profile: json=${json_sz}B bin=${bin_sz}B ratio=$(python3 -c "print(f'{$json_sz/$bin_sz:.2f}x')")"
    fi
done

echo ""
echo "=== WARM MODE (binary load + in-process query) ==="
for profile in $PROFILES; do
    if [ ! -f "$INDEXES/${profile}.idx.bin" ]; then
        echo "  $profile: no binary index, skipping"
        continue
    fi
    out="$RESULTS/${profile}_warm.json"
    "$BIN/nsq-bench-split" warm \
        "$INDEXES/${profile}.idx.bin" \
        "$CORPUS/${profile}.queries.json" 10 \
        > "$out" 2>/dev/null
    python3 -c "
import json; d = json.load(open('$out'))
print(f\"  {d['mode']:<8} load={d.get('warm_load_ms',0):.1f}ms  \
query={d.get('warm_query_ms',0):.3f}ms  profile=$profile\")
"
done

# Summary table
echo ""
echo "=== SUMMARY TABLE ==="
python3 - <<PY
import json, pathlib, os

results_dir = pathlib.Path("$RESULTS")
profiles = "$PROFILES".split()

print(f"{'Profile':<22} {'core_build':>12} {'core_q_µs':>11} {'cold_build':>11} {'cold_read':>10} {'cold_q_ms':>11} {'warm_load':>10} {'warm_q_ms':>10}")
print("-" * 110)
for p in profiles:
    row = {}
    for mode in ["core","cold","warm"]:
        f = results_dir / f"{p}_{mode}.json"
        if f.exists():
            row[mode] = json.loads(f.read_text())
    c = row.get("core",{})
    cl = row.get("cold",{})
    w = row.get("warm",{})
    print(f"{p:<22} "
          f"{c.get('build_ms_mean',0):>10.1f}ms "
          f"{c.get('query_ms_mean',0)*1000:>9.1f}µs "
          f"{cl.get('cold_build_ms',0):>9.1f}ms "
          f"{cl.get('cold_read_ms',0):>8.1f}ms "
          f"{cl.get('cold_query_ms',0):>9.3f}ms "
          f"{w.get('warm_load_ms',0):>8.1f}ms "
          f"{w.get('warm_query_ms',0):>8.3f}ms")
PY

echo ""
echo "saved: $OUT"
RUN_EOF

chmod +x "$HOME/bin/nsq-run-split-bench"

echo ""
echo "=== DONE ==="
echo "Files modified:"
echo "  crates/nsq-index/src/lib.rs    — binary frame (NSQIDX01), anchors_in_range() O(log n)"
echo "  crates/nsq-query/src/lib.rs    — updated anchors to use binary search"
echo "  crates/nsq-query/src/main.rs   — --batch and --batch-json modes"
echo "  crates/nsq-query/Cargo.toml    — [lib] declared"
echo "  crates/nsq-bench-split/        — new crate: core / cold / warm split benchmarks"
echo ""
echo "Build:"
echo "  cd ~/Braxon && cargo build -p nsq-index -p nsq-query -p nsq-bench-split --release"
echo ""
echo "Run:"
echo "  export PATH=\"\$HOME/bin:\$PATH\""
echo "  export BRAXON_HOME=\"\$HOME/Braxon\""
echo "  nsq-run-split-bench"

# ── nsq-index/src/main.rs — CLI wrapper to build and write indexes ─────────
mkdir -p crates/nsq-index/src
cat > crates/nsq-index/src/main.rs << 'RUST'
//! nsq-index — CLI: read corpus NSQ, write .idx.json and optionally .idx.bin
//!
//! Usage: nsq-index <corpus.nsq> <out.idx.json> [out.idx.bin]

use nsq_index::{build_index_from_text, write_index_binary, write_index_json};
use std::{env, fs, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: nsq-index <corpus.nsq> <out.idx.json> [out.idx.bin]");
        process::exit(2);
    }

    let text = fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("read {}: {e}", args[1]); process::exit(2);
    });

    let idx = build_index_from_text(&args[1], &text);

    write_index_json(&idx, Path::new(&args[2])).unwrap_or_else(|e| {
        eprintln!("write json: {e}"); process::exit(2);
    });

    if let Some(bin_path) = args.get(3) {
        let n = write_index_binary(&idx, Path::new(bin_path)).unwrap_or_else(|e| {
            eprintln!("write binary: {e}"); process::exit(2);
        });
        eprintln!("wrote {} bytes binary index", n);
    }

    eprintln!("symbols={} edges={} states={}",
        idx.stats.symbols, idx.stats.edges, idx.stats.states);
}
RUST

# Update nsq-index Cargo.toml to declare the binary
cat > crates/nsq-index/Cargo.toml << 'TOML'
[package]
name = "nsq-index"
version = "0.1.0"
edition = "2021"

[lib]
name = "nsq_index"
path = "src/lib.rs"

[[bin]]
name = "nsq-index"
path = "src/main.rs"

[dependencies]
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
TOML

echo "nsq-index main.rs + Cargo.toml written"
