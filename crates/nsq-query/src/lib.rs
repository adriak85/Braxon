use nsq_index::{ArtifactIndex, Edge, State};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub command: String,
    pub matches: serde_json::Value,
}

pub fn find_symbol(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let id = idx.derived_symbol_transport_id.get(name).copied();
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
    let mut out: Vec<&str> = idx
        .left_to_edges
        .get(name)
        .into_iter()
        .flatten()
        .map(|&id| idx.edges[id as usize].right.as_str())
        .collect();
    out.sort_unstable();
    out.dedup();
    QueryResult {
        command: format!("neighbors {}", name),
        matches: serde_json::json!({ "neighbors": out }),
    }
}

pub fn edges_left(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let rows: Vec<&Edge> = idx
        .left_to_edges
        .get(name)
        .into_iter()
        .flatten()
        .map(|&id| &idx.edges[id as usize])
        .collect();
    QueryResult {
        command: format!("edges left={}", name),
        matches: serde_json::to_value(rows).unwrap(),
    }
}

pub fn edges_right(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let rows: Vec<&Edge> = idx
        .right_to_edges
        .get(name)
        .into_iter()
        .flatten()
        .map(|&id| &idx.edges[id as usize])
        .collect();
    QueryResult {
        command: format!("edges right={}", name),
        matches: serde_json::to_value(rows).unwrap(),
    }
}

pub fn edges_rel(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let rows: Vec<&Edge> = idx
        .macro_to_edges
        .get(name)
        .into_iter()
        .flatten()
        .map(|&id| &idx.edges[id as usize])
        .collect();
    QueryResult {
        command: format!("edges rel={}", name),
        matches: serde_json::to_value(rows).unwrap(),
    }
}

pub fn states_target(idx: &ArtifactIndex, name: &str) -> QueryResult {
    let rows: Vec<&State> = idx
        .target_to_states
        .get(name)
        .into_iter()
        .flatten()
        .map(|&id| &idx.states[id as usize])
        .collect();
    QueryResult {
        command: format!("states target={}", name),
        matches: serde_json::to_value(rows).unwrap(),
    }
}
