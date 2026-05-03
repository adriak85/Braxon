use nsq_core::NsqSurfaceValue;
use nsq_index::{ArtifactIndex, Edge, IndexStats, State};
use nsq_query::{edges_left, edges_rel, edges_right, find_rel, find_symbol, neighbors, states_target};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn z() -> NsqSurfaceValue { NsqSurfaceValue::zero() }

fn temp_dir(name: &str) -> PathBuf {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::temp_dir().join(format!("BRAXON_phase_one_{}_{}_{}", name, std::process::id(), stamp));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn sample_index() -> ArtifactIndex {
    let edges = vec![
        Edge { left: "alpha".into(), rel: "likes".into(), right: "beta".into(), layer: z(), plane: z(), anchor: z(), weight: z(), flags: z() },
        Edge { left: "alpha".into(), rel: "likes".into(), right: "gamma".into(), layer: z(), plane: z(), anchor: z(), weight: z(), flags: z() },
        Edge { left: "alpha".into(), rel: "knows".into(), right: "gamma".into(), layer: z(), plane: z(), anchor: z(), weight: z(), flags: z() },
    ];
    let states = vec![
        State { target: "beta".into(), state: "warm".into(), flux: z(), gate: z(), phase: z() }
    ];

    let mut derived = HashMap::new();
    derived.insert("alpha".into(), 1);
    derived.insert("beta".into(), 2);
    derived.insert("gamma".into(), 3);

    let mut macro_to_edges = HashMap::new();
    macro_to_edges.insert("likes".into(), vec![0, 1]);
    macro_to_edges.insert("knows".into(), vec![2]);

    let mut left_to_edges = HashMap::new();
    left_to_edges.insert("alpha".into(), vec![0, 1, 2]);

    let mut right_to_edges = HashMap::new();
    right_to_edges.insert("beta".into(), vec![0]);
    right_to_edges.insert("gamma".into(), vec![1, 2]);

    let mut target_to_states = HashMap::new();
    target_to_states.insert("beta".into(), vec![0]);

    ArtifactIndex {
        source_path: "fixture.nsq".into(),
        derived_symbol_transport_id: derived,
        id_to_symbol: vec!["alpha".into(), "beta".into(), "gamma".into()],
        macro_to_edges,
        left_to_edges,
        right_to_edges,
        target_to_states,
        anchor_edges: vec![],
        edges,
        states,
        stats: IndexStats {
            input_lines: 0,
            normalized_lines: 0,
            comment_lines_stripped: 0,
            duplicate_lines_removed: 0,
            symbols: 3,
            macros: 2,
            edges: 3,
            states: 1,
            index_bytes: 0,
        },
    }
}

#[test]
fn lib_queries_return_expected_matches() {
    let idx = sample_index();

    let symbol = find_symbol(&idx, "alpha");
    assert_eq!(symbol.matches["symbol"], "alpha");
    assert_eq!(symbol.matches["id"], 1);
    assert_eq!(symbol.matches["exists"], true);

    let missing = find_symbol(&idx, "missing");
    assert_eq!(missing.matches["exists"], false);

    let rel = find_rel(&idx, "likes");
    assert_eq!(rel.matches["count"], 2);

    let neigh = neighbors(&idx, "alpha");
    assert_eq!(neigh.matches["neighbors"], serde_json::json!(["beta", "gamma"]));

    assert_eq!(edges_left(&idx, "alpha").matches.as_array().unwrap().len(), 3);
    assert_eq!(edges_right(&idx, "gamma").matches.as_array().unwrap().len(), 2);
    assert_eq!(edges_rel(&idx, "likes").matches.as_array().unwrap().len(), 2);
    assert_eq!(states_target(&idx, "beta").matches.as_array().unwrap().len(), 1);
}

#[test]
fn binary_query_surface_handles_single_query() {
    let dir = temp_dir("nsq_query_bin");
    let idx = sample_index();
    let idx_path = dir.join("index.idx.json");
    fs::write(&idx_path, serde_json::to_vec_pretty(&idx).unwrap()).unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_nsq-query"))
        .arg(&idx_path)
        .arg("find symbol alpha")
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("\"symbol\": \"alpha\""));
    assert!(text.contains("\"exists\": true"));
}
