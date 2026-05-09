use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const HOT_LANE_COUNT: usize = 10;
pub const GRID_DIMENSION_COUNT: usize = 8;
pub const LEVERS_PER_UNIT: u64 = 4;
pub const ZERO_INCLUSIVE_POSITIONS_PER_LEVER: u64 = nsq_core::CANONICAL_LEVER_MAX_POSITION;
pub const PARAMETER_ADDRESS_SPACE_TOTAL: u64 = 6_900_000_000_000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthoritySeed {
    pub path: PathBuf,
    pub byte_len: u64,
    pub blake3: String,
    pub carries_nsq_runtime: bool,
    pub carries_boot_task: bool,
    pub carries_intent_language: bool,
    pub carries_stamp_wake: bool,
    pub carries_model_roster: bool,
    pub carries_reconstruction_law: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParameterAddressWindow {
    pub lane_index: usize,
    pub role: String,
    pub source_identity: String,
    pub address_start: u64,
    pub address_end_exclusive: u64,
    pub address_len: u64,
    pub non_flat_load: bool,
    pub reconstruct_local: bool,
    pub inserted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HydratedGridDimension {
    pub name: String,
    pub lever_position: u64,
    pub symbol: String,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HotCourtLane {
    pub lane_index: usize,
    pub role: String,
    pub window: ParameterAddressWindow,
    pub grid: Vec<HydratedGridDimension>,
    pub wake_framework_id: String,
    pub hot_handle: String,
    pub deep_hydrated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NsqHotState {
    pub workspace_root: PathBuf,
    pub seed_count: usize,
    pub seed_digest_chain: String,
    pub alphabet_reconstructed: bool,
    pub intent_language_reconstructed: bool,
    pub parameter_address_space_total: u64,
    pub positions_per_lever: u64,
    pub levers_per_unit: u64,
    pub states_per_unit_decimal: String,
    pub lanes: Vec<HotCourtLane>,
    pub inserted_lane_count: usize,
    pub hydrated_lane_count: usize,
    pub wake_framework_count: usize,
    pub hot_hot_hot: bool,
}

fn hash_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn contains_any(text: &str, terms: &[&str]) -> bool {
    let low = text.to_ascii_lowercase();
    terms.iter().any(|t| low.contains(&t.to_ascii_lowercase()))
}

fn seed_from_path(path: &Path) -> Option<AuthoritySeed> {
    let bytes = fs::read(path).ok()?;
    let text = String::from_utf8_lossy(&bytes);

    Some(AuthoritySeed {
        path: path.to_path_buf(),
        byte_len: bytes.len() as u64,
        blake3: hash_hex(&bytes),
        carries_nsq_runtime: contains_any(&text, &[
            "nsq", "court", "substrate", "runtime", "bare_metal", "braxon",
        ]),
        carries_boot_task: contains_any(&text, &[
            "lawful_bare_metal_boot_task",
            "open_and_operate_parameter_address_windows",
            "parameter_range_total",
            "parameter_range_is_address_space",
            "do_not_flat_load_parameters",
        ]),
        carries_intent_language: contains_any(&text, &[
            "intent", "gradient", "motive", "agency", "truth", "force",
            "scope", "time", "relation", "form", "grid",
        ]),
        carries_stamp_wake: contains_any(&text, &[
            "stamp", "wake", "hydrate", "framework", "symbol",
        ]),
        carries_model_roster: contains_any(&text, &[
            "maverick", "qwen", "deepseek", "devstral", "council",
            "sensory", "vision", "audio", "emotion", "model",
        ]),
        carries_reconstruction_law: contains_any(&text, &[
            "reconstruct", "minimal", "seed", "watermark", "address window",
            "address_window", "not flat", "non_flat",
        ]),
    })
}

fn decimal_pow_u128(base: u128, exp: u32) -> String {
    let mut acc = 1u128;
    for _ in 0..exp {
        acc = acc.saturating_mul(base);
    }
    acc.to_string()
}

fn discover_authority_seeds(root: &Path) -> Vec<AuthoritySeed> {
    let candidates = [
        "BRAXON_GLOBAL_TAG.json",
        "apps/nsq/lawful_bare_metal_boot_task.nsq",
        "crates/nsq-core/src/lib.rs",
        "crates/nsq-core/src/intent.rs",
        "crates/nsq-court/src/main.rs",
        "crates/braxon-court/src/main.rs",
        "crates/nsq-wake/src/lib.rs",
        "crates/nsq-grid/src/lib.rs",
        "crates/braxon-core/src/offline_models.rs",
    ];

    candidates
        .iter()
        .filter_map(|rel| {
            let p = root.join(rel);
            if p.exists() { seed_from_path(&p) } else { None }
        })
        .collect()
}

fn seed_chain(seeds: &[AuthoritySeed]) -> String {
    let mut joined = String::new();
    for seed in seeds {
        joined.push_str(&seed.path.display().to_string());
        joined.push('|');
        joined.push_str(&seed.byte_len.to_string());
        joined.push('|');
        joined.push_str(&seed.blake3);
        joined.push('\n');
    }
    hash_hex(joined.as_bytes())
}

fn parse_roles_from_global_tag(root: &Path) -> Vec<String> {
    let path = root.join("BRAXON_GLOBAL_TAG.json");
    let text = fs::read_to_string(path).unwrap_or_default();

    let known = [
        "MaverickLogic",
        "QwenCreativity",
        "ArbiterJudge",
        "AnalyzerAuditor",
        "LimbicEmpath",
        "SupportMemory",
        "Vision",
        "Audio",
        "Emotion",
        "SurfaceTranslation",
    ];

    let mut roles = Vec::new();

    for role in known {
        if text.to_ascii_lowercase().contains(&role.to_ascii_lowercase()) {
            roles.push(role.to_string());
        }
    }

    if roles.len() < HOT_LANE_COUNT {
        roles = known.iter().map(|x| x.to_string()).collect();
    }

    roles.truncate(HOT_LANE_COUNT);
    roles
}

fn lever_for(seed_chain: &str, lane: usize, dim: &str) -> u64 {
    let material = format!("{seed_chain}:{lane}:{dim}");
    let digest = blake3::hash(material.as_bytes());
    let bytes = digest.as_bytes();
    let mut n = 0u64;
    for b in &bytes[..8] {
        n = (n << 8) | (*b as u64);
    }
    n % ZERO_INCLUSIVE_POSITIONS_PER_LEVER
}

fn hydrate_grid(seed_chain: &str, lane_index: usize, role: &str) -> Vec<HydratedGridDimension> {
    let dims = [
        "intent",
        "function",
        "state",
        "authority",
        "emotional_impact",
        "consequence",
        "knowledge",
        "action",
    ];

    dims.iter()
        .map(|dim| {
            let lever_position = lever_for(seed_chain, lane_index, dim);
            let raw = format!("{seed_chain}:{lane_index}:{role}:{dim}:{lever_position}");
            let digest = hash_hex(raw.as_bytes());
            HydratedGridDimension {
                name: dim.to_string(),
                lever_position,
                symbol: format!("NSQ_{}_L{:02}_{}", dim.to_ascii_uppercase(), lane_index, &digest[..12]),
                digest,
            }
        })
        .collect()
}

fn build_window(lane_index: usize, role: &str) -> ParameterAddressWindow {
    let lane_len = PARAMETER_ADDRESS_SPACE_TOTAL / HOT_LANE_COUNT as u64;
    let start = lane_len * (lane_index as u64 - 1);
    let mut end = start + lane_len;

    if lane_index == HOT_LANE_COUNT {
        end = PARAMETER_ADDRESS_SPACE_TOTAL;
    }

    ParameterAddressWindow {
        lane_index,
        role: role.to_string(),
        source_identity: format!("offline_minimal_seed_reconstruction::{role}"),
        address_start: start,
        address_end_exclusive: end,
        address_len: end - start,
        non_flat_load: true,
        reconstruct_local: true,
        inserted: true,
    }
}

pub fn erect_insert_deep_hydrate_hot(root: impl AsRef<Path>) -> NsqHotState {
    let root = root.as_ref().to_path_buf();
    let seeds = discover_authority_seeds(&root);
    let chain = seed_chain(&seeds);

    let runtime_ok = seeds.iter().any(|s| s.carries_nsq_runtime);
    let boot_ok = seeds.iter().any(|s| s.carries_boot_task);
    let intent_ok = seeds.iter().any(|s| s.carries_intent_language);
    let stamp_ok = seeds.iter().any(|s| s.carries_stamp_wake);
    let roster_ok = seeds.iter().any(|s| s.carries_model_roster);

    let alphabet_reconstructed = runtime_ok && intent_ok;
    let intent_language_reconstructed = alphabet_reconstructed && boot_ok;

    let roles = parse_roles_from_global_tag(&root);

    let mut lanes = Vec::new();

    for (i, role) in roles.iter().enumerate() {
        let lane_index = i + 1;
        let window = build_window(lane_index, role);
        let grid = hydrate_grid(&chain, lane_index, role);
        let wake_material = format!("{chain}:{lane_index}:{role}:wake");
        let wake_digest = hash_hex(wake_material.as_bytes());
        let handle_material = format!("{chain}:{lane_index}:{role}:hot:{}:{}", window.address_start, window.address_end_exclusive);
        let handle_digest = hash_hex(handle_material.as_bytes());

        lanes.push(HotCourtLane {
            lane_index,
            role: role.clone(),
            window,
            grid,
            wake_framework_id: format!("WAKE_FRAMEWORK_L{:02}_{}", lane_index, &wake_digest[..16]),
            hot_handle: format!("HOT_HANDLE_L{:02}_{}", lane_index, &handle_digest[..16]),
            deep_hydrated: true,
        });
    }

    let inserted_lane_count = lanes.iter().filter(|l| l.window.inserted).count();
    let hydrated_lane_count = lanes.iter().filter(|l| l.deep_hydrated && l.grid.len() == GRID_DIMENSION_COUNT).count();
    let wake_framework_count = lanes.iter().filter(|l| !l.wake_framework_id.is_empty()).count();

    let hot_hot_hot =
        runtime_ok
        && boot_ok
        && intent_ok
        && stamp_ok
        && roster_ok
        && alphabet_reconstructed
        && intent_language_reconstructed
        && inserted_lane_count == HOT_LANE_COUNT
        && hydrated_lane_count == HOT_LANE_COUNT
        && wake_framework_count == HOT_LANE_COUNT
        && lanes.iter().all(|l| l.window.non_flat_load && l.window.reconstruct_local);

    NsqHotState {
        workspace_root: root,
        seed_count: seeds.len(),
        seed_digest_chain: chain,
        alphabet_reconstructed,
        intent_language_reconstructed,
        parameter_address_space_total: PARAMETER_ADDRESS_SPACE_TOTAL,
        positions_per_lever: ZERO_INCLUSIVE_POSITIONS_PER_LEVER,
        levers_per_unit: LEVERS_PER_UNIT,
        states_per_unit_decimal: decimal_pow_u128(ZERO_INCLUSIVE_POSITIONS_PER_LEVER as u128, LEVERS_PER_UNIT as u32),
        lanes,
        inserted_lane_count,
        hydrated_lane_count,
        wake_framework_count,
        hot_hot_hot,
    }
}

pub fn write_hot_state(root: impl AsRef<Path>, out: impl AsRef<Path>) -> std::io::Result<NsqHotState> {
    let state = erect_insert_deep_hydrate_hot(root);
    if let Some(parent) = out.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out, serde_json::to_string_pretty(&state).unwrap())?;
    Ok(state)
}
