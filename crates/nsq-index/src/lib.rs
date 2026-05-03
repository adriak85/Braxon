//! DERIVED ARTIFACT ONLY\n//! This crate is not canonical NSQ truth.\n//! Integer lanes, packed transport, and benchmark/index layouts here are\n//! disposable derivatives regenerated from preserved canonical NSQ artifacts.\n\n//! NSQ derived index artifact only.
//! This crate is not canonical NSQ truth.
//! It may use integer lanes, adjacency packing, and transport-oriented layouts,
//! but those representations are disposable and must be regenerated from
//! canonical preserved artifacts.
//!
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

use nsq_core::NsqSurfaceValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Cursor, Read};
use std::path::Path;
use std::{collections::BTreeSet, fs};

// ── wire types ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub left: String,
    pub rel: String,
    pub right: String,
    pub layer: NsqSurfaceValue,
    pub plane: NsqSurfaceValue,
    pub anchor: NsqSurfaceValue,
    pub weight: NsqSurfaceValue,
    pub flags: NsqSurfaceValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub target: String,
    pub state: String,
    pub flux: NsqSurfaceValue,
    pub gate: NsqSurfaceValue,
    pub phase: NsqSurfaceValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexStats {
    pub input_lines: usize,
    pub normalized_lines: usize,
    pub comment_lines_stripped: usize,
    pub duplicate_lines_removed: usize,
    pub symbols: usize,
    pub macros: usize,
    pub edges: usize,
    pub states: usize,
    pub index_bytes: usize,
}

// ── main index type ────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactIndex {
    pub source_path: String,
    pub derived_symbol_transport_id: crate::derived_transport::DerivedSymbolTransportIdMap,
    pub id_to_symbol: Vec<String>,
    pub macro_to_edges: HashMap<String, Vec<u32>>,
    pub left_to_edges: HashMap<String, Vec<u32>>,
    pub right_to_edges: HashMap<String, Vec<u32>>,
    pub target_to_states: HashMap<String, Vec<u32>>,
    pub anchor_edges: Vec<(u32, u32)>, // (anchor, edge_id) sorted
    pub edges: Vec<Edge>,
    pub states: Vec<State>,
    pub stats: IndexStats,
}

// ── build ─────────────────────────────────────────────────────────────────

fn canonical_surface_value(s: &str) -> NsqSurfaceValue {
    NsqSurfaceValue::new(s).unwrap_or_else(|_| NsqSurfaceValue::zero())
}

fn surface_value_from_transport(value: impl ToString) -> NsqSurfaceValue {
    NsqSurfaceValue::new(value.to_string()).unwrap()
}

fn transport_u8(value: &NsqSurfaceValue) -> u8 {
    value.as_text().parse().unwrap_or(0)
}

fn transport_u16(value: &NsqSurfaceValue) -> u16 {
    value.as_text().parse().unwrap_or(0)
}

fn transport_u32(value: &NsqSurfaceValue) -> u32 {
    value.as_text().parse().unwrap_or(0)
}

fn transport_anchor_bound(raw: &str, default: u32) -> u32 {
    NsqSurfaceValue::new(raw)
        .ok()
        .map(|value| transport_u32(&value))
        .unwrap_or(default)
}

pub fn normalize_canonical_text(text: &str) -> (Vec<String>, usize, usize) {
    let mut stripped = 0usize;
    let mut seen = BTreeSet::<String>::new();
    let mut dups = 0usize;
    let mut out = Vec::<String>::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            stripped += 1;
            continue;
        }
        if line.starts_with("@dialect ")
            || line.starts_with("!dialect ")
            || line.starts_with("dialect ")
        {
            continue;
        }
        let norm = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if seen.insert(norm.clone()) {
            out.push(norm);
        } else {
            dups += 1;
        }
    }
    out.sort();
    (out, stripped, dups)
}

pub fn parse_edges_states(
    lines: &[String],
) -> (Vec<Edge>, Vec<State>, BTreeSet<String>, BTreeSet<String>) {
    let mut edges_map = std::collections::BTreeMap::<String, Edge>::new();
    let mut states_map = std::collections::BTreeMap::<String, State>::new();
    let mut symbols = BTreeSet::<String>::new();
    let mut macros = BTreeSet::<String>::new();

    for line in lines {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.is_empty() {
            continue;
        }
        match toks[0] {
            "triple" => {
                if toks.len() < 6 {
                    continue;
                }
                let left = toks[1].to_string();
                let rel = toks[3].to_string();
                let right = toks[5].to_string();
                symbols.insert(left.clone());
                symbols.insert(right.clone());
                macros.insert(rel.clone());
                let mut layer = NsqSurfaceValue::zero();
                let mut plane = NsqSurfaceValue::zero();
                let mut anchor = NsqSurfaceValue::zero();
                let mut weight = NsqSurfaceValue::zero();
                let mut flags = NsqSurfaceValue::zero();
                let mut i = 6usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":layer" => layer = canonical_surface_value(toks[i + 1]),
                        ":plane" => plane = canonical_surface_value(toks[i + 1]),
                        ":anchor" => anchor = canonical_surface_value(toks[i + 1]),
                        ":weight" => weight = canonical_surface_value(toks[i + 1]),
                        ":flags" => flags = canonical_surface_value(toks[i + 1]),
                        _ => {}
                    }
                    i += 2;
                }
                let e = Edge {
                    left,
                    rel,
                    right,
                    layer,
                    plane,
                    anchor,
                    weight,
                    flags,
                };
                let key = format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}",
                    e.left,
                    e.rel,
                    e.right,
                    e.layer.as_text(),
                    e.plane.as_text(),
                    e.anchor.as_text(),
                    e.weight.as_text(),
                    e.flags.as_text()
                );
                edges_map.insert(key, e);
            }
            "membrane" => {
                if toks.len() < 2 {
                    continue;
                }
                let target = toks[1].to_string();
                symbols.insert(target.clone());
                let mut state = "<?>".to_string();
                let mut flux = NsqSurfaceValue::zero();
                let mut gate = NsqSurfaceValue::zero();
                let mut phase = NsqSurfaceValue::zero();
                let mut i = 2usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":state" => state = toks[i + 1].to_string(),
                        ":flux" => flux = canonical_surface_value(toks[i + 1]),
                        ":gate" => gate = canonical_surface_value(toks[i + 1]),
                        ":phase" => phase = canonical_surface_value(toks[i + 1]),
                        _ => {}
                    }
                    i += 2;
                }
                let st = State {
                    target,
                    state,
                    flux,
                    gate,
                    phase,
                };
                let key = format!(
                    "{}|{}|{}|{}|{}",
                    st.target,
                    st.state,
                    st.flux.as_text(),
                    st.gate.as_text(),
                    st.phase.as_text()
                );
                states_map.insert(key, st);
            }
            _ => {}
        }
    }
    (
        edges_map.into_values().collect(),
        states_map.into_values().collect(),
        symbols,
        macros,
    )
}

pub fn build_index_from_text(source_path: &str, text: &str) -> ArtifactIndex {
    let input_lines = text.lines().count();
    let (normalized, comment_lines_stripped, duplicate_lines_removed) =
        normalize_canonical_text(text);
    let (mut edges, mut states, symbols, macros) = parse_edges_states(&normalized);

    edges.sort_by(|a, b| {
        (
            &a.left, &a.rel, &a.right, &a.layer, &a.plane, &a.anchor, &a.weight, &a.flags,
        )
            .cmp(&(
                &b.left, &b.rel, &b.right, &b.layer, &b.plane, &b.anchor, &b.weight, &b.flags,
            ))
    });
    states.sort_by(|a, b| {
        (&a.target, &a.state, &a.flux, &a.gate, &a.phase)
            .cmp(&(&b.target, &b.state, &b.flux, &b.gate, &b.phase))
    });

    let id_to_symbol: Vec<String> = symbols.iter().cloned().collect();
    let derived_symbol_transport_id: crate::derived_transport::DerivedSymbolTransportIdMap =
        id_to_symbol
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i as u32))
            .collect();

    let mut macro_to_edges = HashMap::<String, Vec<u32>>::new();
    let mut left_to_edges = HashMap::<String, Vec<u32>>::new();
    let mut right_to_edges = HashMap::<String, Vec<u32>>::new();
    let mut target_to_states = HashMap::<String, Vec<u32>>::new();
    let mut anchor_edges = Vec::<(u32, u32)>::new();

    for (i, e) in edges.iter().enumerate() {
        let id = i as u32;
        macro_to_edges.entry(e.rel.clone()).or_default().push(id);
        left_to_edges.entry(e.left.clone()).or_default().push(id);
        right_to_edges.entry(e.right.clone()).or_default().push(id);
        anchor_edges.push((transport_u32(&e.anchor), id));
    }
    anchor_edges.sort();

    for (i, s) in states.iter().enumerate() {
        target_to_states
            .entry(s.target.clone())
            .or_default()
            .push(i as u32);
    }

    let mut idx = ArtifactIndex {
        source_path: source_path.to_string(),
        derived_symbol_transport_id,
        id_to_symbol,
        macro_to_edges,
        left_to_edges,
        right_to_edges,
        target_to_states,
        anchor_edges,
        edges,
        states,
        stats: IndexStats {
            input_lines,
            normalized_lines: normalized.len(),
            comment_lines_stripped,
            duplicate_lines_removed,
            symbols: symbols.len(),
            macros: macros.len(),
            edges: 0,
            states: 0,
            index_bytes: 0,
        },
    };
    idx.stats.edges = idx.edges.len();
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

fn put_transport_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}
fn put_transport_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn get_transport_u16(cur: &mut Cursor<&[u8]>) -> io::Result<u16> {
    let mut b = [0u8; 2];
    cur.read_exact(&mut b)?;
    Ok(u16::from_le_bytes(b))
}
fn get_transport_u32(cur: &mut Cursor<&[u8]>) -> io::Result<u32> {
    let mut b = [0u8; 4];
    cur.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

pub fn write_index_binary(idx: &ArtifactIndex, path: &Path) -> Result<usize, String> {
    let mut buf = Vec::with_capacity(64 * 1024);

    buf.extend_from_slice(MAGIC);
    put_transport_u32(&mut buf, idx.id_to_symbol.len() as u32);
    put_transport_u32(&mut buf, idx.edges.len() as u32);
    put_transport_u32(&mut buf, idx.states.len() as u32);

    // Symbol table
    for sym in &idx.id_to_symbol {
        let b = sym.as_bytes();
        put_transport_u16(&mut buf, b.len() as u16);
        buf.extend_from_slice(b);
    }

    // Build rel symbol table (macros not in id_to_symbol necessarily)
    // We intern rel strings into id_to_symbol if present, else store inline
    // For binary format: edge left/right use sym_to_id; rel stored as u16 from a separate rel table
    // Simple approach: build rel_to_id from macro_to_edges keys
    let mut rel_order: Vec<String> = idx.macro_to_edges.keys().cloned().collect();
    rel_order.sort();
    let rel_to_id: HashMap<String, u16> = rel_order
        .iter()
        .enumerate()
        .map(|(i, s)| (s.clone(), i as u16))
        .collect();
    put_transport_u16(&mut buf, rel_order.len() as u16);
    for rel in &rel_order {
        let b = rel.as_bytes();
        put_transport_u16(&mut buf, b.len() as u16);
        buf.extend_from_slice(b);
    }

    // Edge table: 15 bytes per edge
    for e in &idx.edges {
        let left_id = idx
            .derived_symbol_transport_id
            .get(&e.left)
            .copied()
            .unwrap_or(0) as u16;
        let right_id = idx
            .derived_symbol_transport_id
            .get(&e.right)
            .copied()
            .unwrap_or(0) as u16;
        let rel_id = rel_to_id.get(&e.rel).copied().unwrap_or(0);
        put_transport_u16(&mut buf, left_id);
        put_transport_u16(&mut buf, rel_id);
        put_transport_u16(&mut buf, right_id);
        buf.push(transport_u8(&e.layer));
        buf.push(transport_u8(&e.plane));
        put_transport_u32(&mut buf, transport_u32(&e.anchor));
        put_transport_u16(&mut buf, transport_u16(&e.weight));
        buf.push(transport_u8(&e.flags));
    }

    // State table: 8 bytes per state
    for s in &idx.states {
        let target_id = idx
            .derived_symbol_transport_id
            .get(&s.target)
            .copied()
            .unwrap_or(0) as u16;
        let state_sym = idx
            .derived_symbol_transport_id
            .get(&s.state)
            .copied()
            .unwrap_or(u32::MAX) as u16;
        put_transport_u16(&mut buf, target_id);
        put_transport_u16(&mut buf, state_sym);
        put_transport_u16(&mut buf, transport_u16(&s.flux));
        buf.push(transport_u8(&s.gate));
        buf.push(transport_u8(&s.phase));
    }

    // Adjacency tables (left, right)
    // For binary, use sym id → [edge_id] directly
    let mut left_adj: Vec<(u32, Vec<u32>)> = idx
        .left_to_edges
        .iter()
        .filter_map(|(k, v)| {
            idx.derived_symbol_transport_id
                .get(k)
                .map(|&id| (id, v.clone()))
        })
        .collect();
    left_adj.sort_by_key(|(id, _)| *id);
    put_transport_u32(&mut buf, left_adj.len() as u32);
    let mut flat_left: Vec<u8> = Vec::new();
    let mut offset = 0u32;
    let la_hdr_start = buf.len();
    for _ in &left_adj {
        put_transport_u32(&mut buf, 0);
        put_transport_u32(&mut buf, 0);
        put_transport_u32(&mut buf, 0);
    }
    for (i, (sym_id, ids)) in left_adj.iter().enumerate() {
        let h = la_hdr_start + i * 12;
        buf[h..h + 4].copy_from_slice(&sym_id.to_le_bytes());
        buf[h + 4..h + 8].copy_from_slice(&offset.to_le_bytes());
        buf[h + 8..h + 12].copy_from_slice(&(ids.len() as u32).to_le_bytes());
        for &id in ids {
            flat_left.extend_from_slice(&id.to_le_bytes());
        }
        offset += ids.len() as u32;
    }
    buf.extend_from_slice(&flat_left);

    // Anchor index
    put_transport_u32(&mut buf, idx.anchor_edges.len() as u32);
    for &(anchor, edge_id) in &idx.anchor_edges {
        put_transport_u32(&mut buf, anchor);
        put_transport_u32(&mut buf, edge_id);
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
    if &magic != MAGIC {
        return Err("bad magic".to_string());
    }

    let sym_count = get_transport_u32(&mut cur).map_err(|e| e.to_string())? as usize;
    let edge_count = get_transport_u32(&mut cur).map_err(|e| e.to_string())? as usize;
    let state_count = get_transport_u32(&mut cur).map_err(|e| e.to_string())? as usize;

    // Symbol table
    let mut id_to_symbol = Vec::with_capacity(sym_count);
    for _ in 0..sym_count {
        let len = get_transport_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let mut b = vec![0u8; len];
        cur.read_exact(&mut b).map_err(|e| e.to_string())?;
        id_to_symbol.push(String::from_utf8_lossy(&b).to_string());
    }
    let derived_symbol_transport_id: crate::derived_transport::DerivedSymbolTransportIdMap =
        id_to_symbol
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i as u32))
            .collect();

    // Rel table
    let rel_count = get_transport_u16(&mut cur).map_err(|e| e.to_string())? as usize;
    let mut id_to_rel = Vec::with_capacity(rel_count);
    for _ in 0..rel_count {
        let len = get_transport_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let mut b = vec![0u8; len];
        cur.read_exact(&mut b).map_err(|e| e.to_string())?;
        id_to_rel.push(String::from_utf8_lossy(&b).to_string());
    }

    // Edge table
    let mut edges = Vec::with_capacity(edge_count);
    for _ in 0..edge_count {
        let left_id = get_transport_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let rel_id = get_transport_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let right_id = get_transport_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let layer = {
            let mut b = [0u8; 1];
            cur.read_exact(&mut b).map_err(|e| e.to_string())?;
            b[0]
        };
        let plane = {
            let mut b = [0u8; 1];
            cur.read_exact(&mut b).map_err(|e| e.to_string())?;
            b[0]
        };
        let anchor = get_transport_u32(&mut cur).map_err(|e| e.to_string())?;
        let weight = get_transport_u16(&mut cur).map_err(|e| e.to_string())?;
        let flags = {
            let mut b = [0u8; 1];
            cur.read_exact(&mut b).map_err(|e| e.to_string())?;
            b[0]
        };
        edges.push(Edge {
            left: id_to_symbol.get(left_id).cloned().unwrap_or_default(),
            rel: id_to_rel.get(rel_id).cloned().unwrap_or_default(),
            right: id_to_symbol.get(right_id).cloned().unwrap_or_default(),
            layer: surface_value_from_transport(layer),
            plane: surface_value_from_transport(plane),
            anchor: surface_value_from_transport(anchor),
            weight: surface_value_from_transport(weight),
            flags: surface_value_from_transport(flags),
        });
    }

    // State table
    let mut states = Vec::with_capacity(state_count);
    for _ in 0..state_count {
        let target_id = get_transport_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let state_id = get_transport_u16(&mut cur).map_err(|e| e.to_string())? as usize;
        let flux = get_transport_u16(&mut cur).map_err(|e| e.to_string())?;
        let gate = {
            let mut b = [0u8; 1];
            cur.read_exact(&mut b).map_err(|e| e.to_string())?;
            b[0]
        };
        let phase = {
            let mut b = [0u8; 1];
            cur.read_exact(&mut b).map_err(|e| e.to_string())?;
            b[0]
        };
        states.push(State {
            target: id_to_symbol.get(target_id).cloned().unwrap_or_default(),
            state: id_to_symbol
                .get(state_id)
                .cloned()
                .unwrap_or("unknown".to_string()),
            flux: surface_value_from_transport(flux),
            gate: surface_value_from_transport(gate),
            phase: surface_value_from_transport(phase),
        });
    }

    // Left adjacency
    let n_left = get_transport_u32(&mut cur).map_err(|e| e.to_string())? as usize;
    let mut left_hdrs = Vec::with_capacity(n_left);
    for _ in 0..n_left {
        let sym_id = get_transport_u32(&mut cur).map_err(|e| e.to_string())? as usize;
        let offset = get_transport_u32(&mut cur).map_err(|e| e.to_string())? as usize;
        let len = get_transport_u32(&mut cur).map_err(|e| e.to_string())? as usize;
        left_hdrs.push((sym_id, offset, len));
    }
    let total_left_ids: usize = left_hdrs.iter().map(|(_, _, l)| l).sum();
    let mut flat_left = vec![0u8; total_left_ids * 4];
    cur.read_exact(&mut flat_left).map_err(|e| e.to_string())?;
    let mut left_to_edges = HashMap::with_capacity(n_left);
    for (sym_id, offset, len) in left_hdrs {
        let sym = id_to_symbol.get(sym_id).cloned().unwrap_or_default();
        let ids: Vec<u32> = flat_left[offset * 4..(offset + len) * 4]
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        left_to_edges.insert(sym, ids);
    }

    // Anchor index
    let n_anchors = get_transport_u32(&mut cur).map_err(|e| e.to_string())? as usize;
    let mut anchor_edges = Vec::with_capacity(n_anchors);
    for _ in 0..n_anchors {
        let anchor = get_transport_u32(&mut cur).map_err(|e| e.to_string())?;
        let edge_id = get_transport_u32(&mut cur).map_err(|e| e.to_string())?;
        anchor_edges.push((anchor, edge_id));
    }

    // Rebuild right and macro adjacency from edges (not stored in binary to save space)
    let mut right_to_edges = HashMap::<String, Vec<u32>>::new();
    let mut macro_to_edges = HashMap::<String, Vec<u32>>::new();
    let mut target_to_states = HashMap::<String, Vec<u32>>::new();
    for (i, e) in edges.iter().enumerate() {
        right_to_edges
            .entry(e.right.clone())
            .or_default()
            .push(i as u32);
        macro_to_edges
            .entry(e.rel.clone())
            .or_default()
            .push(i as u32);
    }
    for (i, s) in states.iter().enumerate() {
        target_to_states
            .entry(s.target.clone())
            .or_default()
            .push(i as u32);
    }

    Ok(ArtifactIndex {
        source_path: path.display().to_string(),
        derived_symbol_transport_id,
        id_to_symbol,
        macro_to_edges,
        left_to_edges,
        right_to_edges,
        target_to_states,
        anchor_edges,
        edges,
        states,
        stats: IndexStats {
            input_lines: 0,
            normalized_lines: 0,
            comment_lines_stripped: 0,
            duplicate_lines_removed: 0,
            symbols: sym_count,
            macros: rel_count,
            edges: edge_count,
            states: state_count,
            index_bytes: raw.len(),
        },
    })
}

// ── shortest path (BFS) ────────────────────────────────────────────────────

pub fn shortest_path(
    idx: &ArtifactIndex,
    src: &str,
    dst: &str,
    depth: usize,
) -> Option<Vec<String>> {
    if src == dst {
        return Some(vec![src.to_string()]);
    }
    let mut q = std::collections::VecDeque::<(String, Vec<String>)>::new();
    let mut seen = std::collections::HashSet::<String>::new();
    q.push_back((src.to_string(), vec![src.to_string()]));
    seen.insert(src.to_string());
    while let Some((cur, path)) = q.pop_front() {
        if path.len() > depth + 1 {
            continue;
        }
        if let Some(edge_ids) = idx.left_to_edges.get(&cur) {
            for &edge_id in edge_ids {
                let next = idx.edges[edge_id as usize].right.clone();
                if seen.contains(&next) {
                    continue;
                }
                let mut np = path.clone();
                np.push(next.clone());
                if next == dst {
                    return Some(np);
                }
                seen.insert(next.clone());
                q.push_back((next, np));
            }
        }
    }
    None
}

// ── anchor binary search ───────────────────────────────────────────────────

pub fn anchors_in_range<'a>(idx: &'a ArtifactIndex, min: &str, max: &str) -> Vec<&'a Edge> {
    let min = transport_anchor_bound(min, 0);
    let max = transport_anchor_bound(max, u32::MAX);
    let lo = idx.anchor_edges.partition_point(|&(a, _)| a < min);
    let hi = idx.anchor_edges.partition_point(|&(a, _)| a <= max);
    idx.anchor_edges[lo..hi]
        .iter()
        .map(|&(_, id)| &idx.edges[id as usize])
        .collect()
}

#[allow(dead_code)]
pub mod derived_transport;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_queries_accept_canonical_text_bounds() {
        let idx = build_index_from_text(
            "memory://corpus.nsq",
            "triple left -> binds -> right :anchor 12\ntriple left -> binds -> next :anchor 44\n",
        );

        let rows = anchors_in_range(&idx, "10", "20");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].anchor.as_text(), "12");
    }

    #[test]
    fn anchor_queries_fall_back_cleanly_for_invalid_bounds() {
        let idx = build_index_from_text(
            "memory://corpus.nsq",
            "triple left -> binds -> right :anchor 12\ntriple left -> binds -> next :anchor 44\n",
        );

        let rows = anchors_in_range(&idx, "bad-min", "bad-max");
        assert_eq!(rows.len(), 2);
    }
}
