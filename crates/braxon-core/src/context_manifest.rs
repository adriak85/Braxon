use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const BRAXON_CONTEXT_MANIFEST_RELATIVE_PATH: &str = "config/braxon_context_manifest.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticPointer {
    pub id: String,
    pub kind: String,
    pub path: String,
    pub required: bool,
    pub relationship: String,
    pub route: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeftOutContext {
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeftOutPolicy {
    pub must_call_out_omissions: bool,
    pub missing_required_pointer_action: String,
    pub missing_optional_pointer_action: String,
    pub citadel_surfaces_must_be_named: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WakeTriggers {
    pub enabled_by_env: String,
    pub changed_files_env: String,
    #[serde(default = "default_exit_changed_files_env")]
    pub exit_changed_files_env: String,
    pub default_mode: String,
    pub overhead_policy: String,
    pub surface_match_mode: String,
    pub suggest_linked_changes_for_each_changed_surface: bool,
    pub pipe_and_chain_identification: bool,
    #[serde(default = "default_mechanical_watermark")]
    pub mechanical_watermark: String,
    #[serde(default = "default_tag_namespace")]
    pub tag_namespace: String,
    #[serde(default = "default_chain_root")]
    pub chain_root: String,
    #[serde(default = "default_centralized_chain_db")]
    pub centralized_chain_db: String,
    #[serde(default = "default_non_runtime_adjustment_wake_action")]
    pub non_runtime_adjustment_wake_action: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BraxonContextManifest {
    pub schema: String,
    pub generated_at: String,
    pub identity: String,
    pub canonical_semantics: String,
    pub private_license: bool,
    pub offline_context_api: String,
    pub semantic_pointers: Vec<SemanticPointer>,
    pub known_left_out: Vec<LeftOutContext>,
    pub left_out_policy: LeftOutPolicy,
    pub wake_triggers: WakeTriggers,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissingContextPointer {
    pub id: String,
    pub kind: String,
    pub path: String,
    pub required: bool,
    pub relationship: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BraxonContextManifestStatus {
    pub manifest_path: String,
    pub loaded: bool,
    pub identity: String,
    pub canonical_semantics: String,
    pub private_license: bool,
    pub semantic_pointer_count: usize,
    pub missing_required: Vec<MissingContextPointer>,
    pub missing_optional: Vec<MissingContextPointer>,
    pub known_left_out: Vec<LeftOutContext>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAdjustmentLane {
    pub numeric_id: u64,
    pub numeric_id_base8: String,
    pub semantic_id: String,
    pub path: String,
    pub match_mode: String,
    pub relationship: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainRootRecord {
    pub numeric_id: u64,
    pub numeric_id_base8: String,
    pub semantic_id: String,
    pub pointer_id: String,
    pub kind: String,
    pub path: String,
    pub runtime_adjustable: bool,
    pub chain_root: String,
    pub wake_action: String,
    pub watermark: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextChainRootDb {
    pub schema: String,
    pub watermark: String,
    pub tag_namespace: String,
    pub chain_root: String,
    pub default_wake_action: String,
    pub runtime_adjustment_lanes: Vec<RuntimeAdjustmentLane>,
    pub chain_records: Vec<ChainRootRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WakeChainLink {
    pub numeric_id: u64,
    pub numeric_id_base8: String,
    pub semantic_id: String,
    pub surface: String,
    pub ordinal: usize,
    pub mechanical_tag: String,
    pub watermark: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedSurfaceTag {
    pub numeric_id: u64,
    pub numeric_id_base8: String,
    pub semantic_id: String,
    pub pointer_id: String,
    pub kind: String,
    pub path: String,
    pub relationship: String,
    pub mechanical_tag: String,
    pub watermark: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedChangeSuggestion {
    pub numeric_id: u64,
    pub numeric_id_base8: String,
    pub semantic_id: String,
    pub watermark: String,
    pub mechanical_tags: Vec<String>,
    pub changed_path: String,
    pub matched_pointer_id: String,
    pub matched_pointer_kind: String,
    pub relationship: String,
    pub pipe_chain: Vec<WakeChainLink>,
    pub pipe_chain_surfaces: Vec<String>,
    pub linked_paths: Vec<String>,
    pub linked_surfaces: Vec<LinkedSurfaceTag>,
    pub exit_wake_action: Option<WakeActionAnnouncement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WakeActionAnnouncement {
    pub numeric_id: u64,
    pub numeric_id_base8: String,
    pub semantic_id: String,
    pub action: String,
    pub source_db: String,
    pub chain_root: String,
    pub runtime_adjustable: bool,
    pub announcement: String,
    pub mechanical_tag: String,
    pub watermark: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WakeLinkedChangeReport {
    pub watermark: String,
    pub tag_namespace: String,
    pub centralized_chain_db: String,
    pub chain_root: String,
    pub wake_enabled: bool,
    pub env_trigger: String,
    pub changed_files_env: String,
    pub exit_changed_files_env: String,
    pub changed_path_count: usize,
    pub suggestions: Vec<LinkedChangeSuggestion>,
}

impl BraxonContextManifestStatus {
    pub fn all_required_context_present(&self) -> bool {
        self.loaded && self.missing_required.is_empty()
    }
}

pub fn braxon_context_manifest_path(root: &Path) -> PathBuf {
    root.join(BRAXON_CONTEXT_MANIFEST_RELATIVE_PATH)
}

pub fn load_braxon_context_manifest(root: &Path) -> Result<BraxonContextManifest, String> {
    let path = braxon_context_manifest_path(root);
    let raw = fs::read_to_string(&path).map_err(|err| {
        format!(
            "failed to read Braxon context manifest '{}': {err}",
            path.display()
        )
    })?;
    serde_json::from_str(&raw).map_err(|err| {
        format!(
            "failed to parse Braxon context manifest '{}': {err}",
            path.display()
        )
    })
}

pub fn load_braxon_chain_root_db(
    root: &Path,
    manifest: &BraxonContextManifest,
) -> Result<ContextChainRootDb, String> {
    let path = root.join(&manifest.wake_triggers.centralized_chain_db);
    let raw = fs::read_to_string(&path).map_err(|err| {
        format!(
            "failed to read Braxon chain root db '{}': {err}",
            path.display()
        )
    })?;
    serde_json::from_str(&raw).map_err(|err| {
        format!(
            "failed to parse Braxon chain root db '{}': {err}",
            path.display()
        )
    })
}

pub fn braxon_context_manifest_status(root: &Path) -> Result<BraxonContextManifestStatus, String> {
    let path = braxon_context_manifest_path(root);
    let manifest = load_braxon_context_manifest(root)?;
    let mut missing_required = Vec::new();
    let mut missing_optional = Vec::new();

    for pointer in &manifest.semantic_pointers {
        if root.join(&pointer.path).exists() {
            continue;
        }

        let missing = MissingContextPointer {
            id: pointer.id.clone(),
            kind: pointer.kind.clone(),
            path: pointer.path.clone(),
            required: pointer.required,
            relationship: pointer.relationship.clone(),
        };

        if pointer.required {
            missing_required.push(missing);
        } else {
            missing_optional.push(missing);
        }
    }

    Ok(BraxonContextManifestStatus {
        manifest_path: path.display().to_string(),
        loaded: true,
        identity: manifest.identity,
        canonical_semantics: manifest.canonical_semantics,
        private_license: manifest.private_license,
        semantic_pointer_count: manifest.semantic_pointers.len(),
        missing_required,
        missing_optional,
        known_left_out: manifest.known_left_out,
    })
}

pub fn braxon_wake_linked_change_report_from_env(
    root: &Path,
) -> Result<WakeLinkedChangeReport, String> {
    let manifest = load_braxon_context_manifest(root)?;
    let wake_enabled = std::env::var(&manifest.wake_triggers.enabled_by_env)
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);
    if !wake_enabled {
        return Ok(braxon_wake_linked_change_report(
            &manifest,
            false,
            Vec::new(),
        ));
    }

    let chain_db = load_braxon_chain_root_db(root, &manifest)?;
    let changed_raw = std::env::var(&manifest.wake_triggers.changed_files_env).unwrap_or_default();
    let exit_changed_raw =
        std::env::var(&manifest.wake_triggers.exit_changed_files_env).unwrap_or_default();
    let changed_paths = format!("{changed_raw}\n{exit_changed_raw}")
        .lines()
        .flat_map(|line| line.split(','))
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    Ok(braxon_wake_linked_change_report_with_db(
        &manifest,
        &chain_db,
        wake_enabled,
        changed_paths,
    ))
}

pub fn braxon_wake_linked_change_report(
    manifest: &BraxonContextManifest,
    wake_enabled: bool,
    changed_paths: Vec<String>,
) -> WakeLinkedChangeReport {
    let chain_db = derived_chain_root_db(manifest);
    braxon_wake_linked_change_report_with_db(manifest, &chain_db, wake_enabled, changed_paths)
}

pub fn braxon_wake_linked_change_report_with_db(
    manifest: &BraxonContextManifest,
    chain_db: &ContextChainRootDb,
    wake_enabled: bool,
    changed_paths: Vec<String>,
) -> WakeLinkedChangeReport {
    if !wake_enabled {
        return WakeLinkedChangeReport {
            watermark: manifest.wake_triggers.mechanical_watermark.clone(),
            tag_namespace: manifest.wake_triggers.tag_namespace.clone(),
            centralized_chain_db: manifest.wake_triggers.centralized_chain_db.clone(),
            chain_root: manifest.wake_triggers.chain_root.clone(),
            wake_enabled,
            env_trigger: manifest.wake_triggers.enabled_by_env.clone(),
            changed_files_env: manifest.wake_triggers.changed_files_env.clone(),
            exit_changed_files_env: manifest.wake_triggers.exit_changed_files_env.clone(),
            changed_path_count: 0,
            suggestions: Vec::new(),
        };
    }

    let mut suggestions = Vec::new();
    for changed_path in &changed_paths {
        if let Some(pointer) = manifest
            .semantic_pointers
            .iter()
            .find(|pointer| path_matches_surface(changed_path, &pointer.path))
        {
            let linked_surfaces = manifest
                .semantic_pointers
                .iter()
                .filter(|candidate| candidate.id != pointer.id)
                .filter(|candidate| shares_route_or_kind(pointer, candidate))
                .map(|candidate| linked_surface_tag(manifest, pointer, candidate))
                .collect::<Vec<_>>();
            let linked_paths = linked_surfaces
                .iter()
                .map(|surface| surface.path.clone())
                .collect::<Vec<_>>();
            let pipe_chain = chain_links(manifest, pointer);
            let semantic_id = semantic_change_id(
                &manifest.wake_triggers.tag_namespace,
                &pointer.id,
                changed_path,
            );
            let numeric_id = stable_numeric_id(&semantic_id);
            let mut mechanical_tags = pipe_chain
                .iter()
                .map(|link| link.mechanical_tag.clone())
                .collect::<Vec<_>>();
            mechanical_tags.push(mechanical_tag(
                manifest,
                "change",
                &semantic_id,
                numeric_id,
                changed_path,
            ));
            let exit_wake_action =
                non_runtime_wake_action(manifest, chain_db, changed_path, &semantic_id);

            suggestions.push(LinkedChangeSuggestion {
                numeric_id,
                numeric_id_base8: format!("{numeric_id:o}"),
                semantic_id,
                watermark: manifest.wake_triggers.mechanical_watermark.clone(),
                mechanical_tags,
                changed_path: changed_path.clone(),
                matched_pointer_id: pointer.id.clone(),
                matched_pointer_kind: pointer.kind.clone(),
                relationship: pointer.relationship.clone(),
                pipe_chain,
                pipe_chain_surfaces: pointer.route.clone(),
                linked_paths,
                linked_surfaces,
                exit_wake_action,
            });
        } else if let Some(lane) = runtime_lane_for_path(chain_db, changed_path) {
            let semantic_id = semantic_change_id(
                &manifest.wake_triggers.tag_namespace,
                &lane.semantic_id,
                changed_path,
            );
            let numeric_id = stable_numeric_id(&semantic_id);
            let pipe_chain = runtime_lane_chain_links(manifest, lane, changed_path);
            let mut mechanical_tags = pipe_chain
                .iter()
                .map(|link| link.mechanical_tag.clone())
                .collect::<Vec<_>>();
            mechanical_tags.push(mechanical_tag(
                manifest,
                "change",
                &semantic_id,
                numeric_id,
                changed_path,
            ));

            suggestions.push(LinkedChangeSuggestion {
                numeric_id,
                numeric_id_base8: format!("{numeric_id:o}"),
                semantic_id,
                watermark: manifest.wake_triggers.mechanical_watermark.clone(),
                mechanical_tags,
                changed_path: changed_path.clone(),
                matched_pointer_id: lane.semantic_id.clone(),
                matched_pointer_kind: "runtime_adjustment_lane".to_string(),
                relationship: lane.relationship.clone(),
                pipe_chain,
                pipe_chain_surfaces: vec!["runtime_adjustment_lane".to_string()],
                linked_paths: vec![lane.path.clone()],
                linked_surfaces: Vec::new(),
                exit_wake_action: None,
            });
        } else {
            let semantic_id = semantic_change_id(
                &manifest.wake_triggers.tag_namespace,
                "unmapped_surface",
                changed_path,
            );
            let numeric_id = stable_numeric_id(&semantic_id);
            let pipe_chain = unmapped_chain_links(manifest, changed_path);
            let mut mechanical_tags = pipe_chain
                .iter()
                .map(|link| link.mechanical_tag.clone())
                .collect::<Vec<_>>();
            mechanical_tags.push(mechanical_tag(
                manifest,
                "change",
                &semantic_id,
                numeric_id,
                changed_path,
            ));
            let exit_wake_action =
                non_runtime_wake_action(manifest, chain_db, changed_path, &semantic_id);

            suggestions.push(LinkedChangeSuggestion {
                numeric_id,
                numeric_id_base8: format!("{numeric_id:o}"),
                semantic_id,
                watermark: manifest.wake_triggers.mechanical_watermark.clone(),
                mechanical_tags,
                changed_path: changed_path.clone(),
                matched_pointer_id: "unmapped_surface".to_string(),
                matched_pointer_kind: "unknown".to_string(),
                relationship: "changed path is not covered by the Braxon context manifest"
                    .to_string(),
                pipe_chain,
                pipe_chain_surfaces: vec!["unmapped".to_string()],
                linked_paths: manifest
                    .known_left_out
                    .iter()
                    .map(|entry| entry.path.clone())
                    .collect(),
                linked_surfaces: Vec::new(),
                exit_wake_action,
            });
        }
    }

    WakeLinkedChangeReport {
        watermark: manifest.wake_triggers.mechanical_watermark.clone(),
        tag_namespace: manifest.wake_triggers.tag_namespace.clone(),
        centralized_chain_db: manifest.wake_triggers.centralized_chain_db.clone(),
        chain_root: manifest.wake_triggers.chain_root.clone(),
        wake_enabled,
        env_trigger: manifest.wake_triggers.enabled_by_env.clone(),
        changed_files_env: manifest.wake_triggers.changed_files_env.clone(),
        exit_changed_files_env: manifest.wake_triggers.exit_changed_files_env.clone(),
        changed_path_count: changed_paths.len(),
        suggestions,
    }
}

fn path_matches_surface(changed_path: &str, pointer_path: &str) -> bool {
    changed_path == pointer_path
        || changed_path.starts_with(&format!("{}/", pointer_path.trim_end_matches('/')))
}

fn shares_route_or_kind(left: &SemanticPointer, right: &SemanticPointer) -> bool {
    left.kind == right.kind
        || left
            .route
            .iter()
            .any(|surface| right.route.iter().any(|candidate| candidate == surface))
}

fn default_mechanical_watermark() -> String {
    "BRAXON_NSQ_BASE8_CONTEXT_WAKE_V1".to_string()
}

fn default_exit_changed_files_env() -> String {
    "BRAXON_EXIT_CHANGED_FILES".to_string()
}

fn default_tag_namespace() -> String {
    "braxon.context".to_string()
}

fn default_chain_root() -> String {
    "state/braxon/context_chain_root".to_string()
}

fn default_centralized_chain_db() -> String {
    "state/braxon/context_chain_root/chain_wake_registry.json".to_string()
}

fn default_non_runtime_adjustment_wake_action() -> String {
    "wake_announce_non_runtime_adjustment".to_string()
}

fn derived_chain_root_db(manifest: &BraxonContextManifest) -> ContextChainRootDb {
    ContextChainRootDb {
        schema: "braxon.context.chain_root.v1".to_string(),
        watermark: manifest.wake_triggers.mechanical_watermark.clone(),
        tag_namespace: manifest.wake_triggers.tag_namespace.clone(),
        chain_root: manifest.wake_triggers.chain_root.clone(),
        default_wake_action: manifest
            .wake_triggers
            .non_runtime_adjustment_wake_action
            .clone(),
        runtime_adjustment_lanes: Vec::new(),
        chain_records: manifest
            .semantic_pointers
            .iter()
            .enumerate()
            .map(|(index, pointer)| {
                let numeric_id = (index as u64) + 1;
                ChainRootRecord {
                    numeric_id,
                    numeric_id_base8: format!("{numeric_id:o}"),
                    semantic_id: format!(
                        "{}.chain_root.{}",
                        manifest.wake_triggers.tag_namespace, pointer.id
                    ),
                    pointer_id: pointer.id.clone(),
                    kind: pointer.kind.clone(),
                    path: pointer.path.clone(),
                    runtime_adjustable: false,
                    chain_root: format!("{}/{}", manifest.wake_triggers.chain_root, pointer.id),
                    wake_action: manifest
                        .wake_triggers
                        .non_runtime_adjustment_wake_action
                        .clone(),
                    watermark: manifest.wake_triggers.mechanical_watermark.clone(),
                }
            })
            .collect(),
    }
}

fn chain_links(manifest: &BraxonContextManifest, pointer: &SemanticPointer) -> Vec<WakeChainLink> {
    pointer
        .route
        .iter()
        .enumerate()
        .map(|(index, surface)| {
            let ordinal = index + 1;
            let semantic_id = format!(
                "{}.chain.{}.{}.{}",
                manifest.wake_triggers.tag_namespace,
                pointer.id,
                format!("{ordinal:03}"),
                normalize_id_part(surface)
            );
            let numeric_id = stable_numeric_id(&semantic_id);

            WakeChainLink {
                numeric_id,
                numeric_id_base8: format!("{numeric_id:o}"),
                semantic_id: semantic_id.clone(),
                surface: surface.clone(),
                ordinal,
                mechanical_tag: mechanical_tag(
                    manifest,
                    "chain",
                    &semantic_id,
                    numeric_id,
                    surface,
                ),
                watermark: manifest.wake_triggers.mechanical_watermark.clone(),
            }
        })
        .collect()
}

fn unmapped_chain_links(
    manifest: &BraxonContextManifest,
    changed_path: &str,
) -> Vec<WakeChainLink> {
    let semantic_id = format!(
        "{}.chain.unmapped_surface.001.unmapped",
        manifest.wake_triggers.tag_namespace
    );
    let numeric_id = stable_numeric_id(&format!("{semantic_id}:{changed_path}"));

    vec![WakeChainLink {
        numeric_id,
        numeric_id_base8: format!("{numeric_id:o}"),
        semantic_id: semantic_id.clone(),
        surface: "unmapped".to_string(),
        ordinal: 1,
        mechanical_tag: mechanical_tag(manifest, "chain", &semantic_id, numeric_id, changed_path),
        watermark: manifest.wake_triggers.mechanical_watermark.clone(),
    }]
}

fn non_runtime_wake_action(
    manifest: &BraxonContextManifest,
    chain_db: &ContextChainRootDb,
    changed_path: &str,
    parent_semantic_id: &str,
) -> Option<WakeActionAnnouncement> {
    if runtime_adjustable_path(chain_db, changed_path) {
        return None;
    }

    let semantic_id = format!(
        "{}.wake.exit.{}",
        manifest.wake_triggers.tag_namespace,
        normalize_id_part(parent_semantic_id)
    );
    let numeric_id = stable_numeric_id(&semantic_id);
    let action = chain_record_for_path(chain_db, changed_path)
        .map(|record| record.wake_action.clone())
        .unwrap_or_else(|| chain_db.default_wake_action.clone());
    let chain_root = chain_record_for_path(chain_db, changed_path)
        .map(|record| record.chain_root.clone())
        .unwrap_or_else(|| chain_db.chain_root.clone());
    let source_db = manifest.wake_triggers.centralized_chain_db.clone();
    let announcement = format!(
        "{action}: changed_path={changed_path}; source_db={source_db}; chain_root={chain_root}"
    );

    Some(WakeActionAnnouncement {
        numeric_id,
        numeric_id_base8: format!("{numeric_id:o}"),
        semantic_id: semantic_id.clone(),
        action,
        source_db,
        chain_root,
        runtime_adjustable: false,
        announcement,
        mechanical_tag: mechanical_tag(
            manifest,
            "exit_wake",
            &semantic_id,
            numeric_id,
            changed_path,
        ),
        watermark: manifest.wake_triggers.mechanical_watermark.clone(),
    })
}

fn runtime_adjustable_path(chain_db: &ContextChainRootDb, changed_path: &str) -> bool {
    chain_db
        .runtime_adjustment_lanes
        .iter()
        .any(|lane| path_matches_by_mode(changed_path, &lane.path, &lane.match_mode))
        || chain_record_for_path(chain_db, changed_path)
            .map(|record| record.runtime_adjustable)
            .unwrap_or(false)
}

fn runtime_lane_for_path<'a>(
    chain_db: &'a ContextChainRootDb,
    changed_path: &str,
) -> Option<&'a RuntimeAdjustmentLane> {
    chain_db
        .runtime_adjustment_lanes
        .iter()
        .find(|lane| path_matches_by_mode(changed_path, &lane.path, &lane.match_mode))
}

fn chain_record_for_path<'a>(
    chain_db: &'a ContextChainRootDb,
    changed_path: &str,
) -> Option<&'a ChainRootRecord> {
    chain_db
        .chain_records
        .iter()
        .find(|record| path_matches_surface(changed_path, &record.path))
}

fn path_matches_by_mode(changed_path: &str, target_path: &str, match_mode: &str) -> bool {
    match match_mode {
        "exact" => changed_path == target_path,
        "prefix" => path_matches_surface(changed_path, target_path),
        _ => path_matches_surface(changed_path, target_path),
    }
}

fn runtime_lane_chain_links(
    manifest: &BraxonContextManifest,
    lane: &RuntimeAdjustmentLane,
    changed_path: &str,
) -> Vec<WakeChainLink> {
    let semantic_id = format!(
        "{}.chain.{}.001.runtime_adjustment_lane",
        manifest.wake_triggers.tag_namespace,
        normalize_id_part(&lane.semantic_id)
    );
    let numeric_id = stable_numeric_id(&format!("{semantic_id}:{changed_path}"));

    vec![WakeChainLink {
        numeric_id,
        numeric_id_base8: format!("{numeric_id:o}"),
        semantic_id: semantic_id.clone(),
        surface: "runtime_adjustment_lane".to_string(),
        ordinal: 1,
        mechanical_tag: mechanical_tag(manifest, "chain", &semantic_id, numeric_id, changed_path),
        watermark: manifest.wake_triggers.mechanical_watermark.clone(),
    }]
}

fn linked_surface_tag(
    manifest: &BraxonContextManifest,
    source: &SemanticPointer,
    linked: &SemanticPointer,
) -> LinkedSurfaceTag {
    let semantic_id = format!(
        "{}.linked.{}.{}",
        manifest.wake_triggers.tag_namespace, source.id, linked.id
    );
    let numeric_id = stable_numeric_id(&semantic_id);

    LinkedSurfaceTag {
        numeric_id,
        numeric_id_base8: format!("{numeric_id:o}"),
        semantic_id: semantic_id.clone(),
        pointer_id: linked.id.clone(),
        kind: linked.kind.clone(),
        path: linked.path.clone(),
        relationship: linked.relationship.clone(),
        mechanical_tag: mechanical_tag(manifest, "linked", &semantic_id, numeric_id, &linked.path),
        watermark: manifest.wake_triggers.mechanical_watermark.clone(),
    }
}

fn semantic_change_id(tag_namespace: &str, pointer_id: &str, changed_path: &str) -> String {
    format!(
        "{tag_namespace}.change.{}.{}",
        normalize_id_part(pointer_id),
        normalize_id_part(changed_path)
    )
}

fn mechanical_tag(
    manifest: &BraxonContextManifest,
    tag_kind: &str,
    semantic_id: &str,
    numeric_id: u64,
    surface: &str,
) -> String {
    format!(
        "tag={};kind={tag_kind};numeric_id={numeric_id};numeric_id_base8={:o};semantic_id={semantic_id};surface={};watermark={}",
        manifest.wake_triggers.tag_namespace,
        numeric_id,
        normalize_id_part(surface),
        manifest.wake_triggers.mechanical_watermark
    )
}

fn stable_numeric_id(seed: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for ch in seed.chars() {
        hash ^= ch as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    if hash == 0 {
        1
    } else {
        hash
    }
}

fn normalize_id_part(raw: &str) -> String {
    let mut normalized = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
        } else if !normalized.ends_with('_') {
            normalized.push('_');
        }
    }
    normalized.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("braxon-core lives under crates/braxon-core")
            .to_path_buf()
    }

    #[test]
    fn context_manifest_calls_out_omissions_and_required_pointers() {
        let root = workspace_root();
        let status = braxon_context_manifest_status(&root).unwrap();

        assert_eq!(status.identity, "Braxon");
        assert_eq!(status.canonical_semantics, "base8_switch_topology");
        assert!(status.private_license);
        assert!(status.semantic_pointer_count >= 10);
        assert!(status
            .known_left_out
            .iter()
            .any(|entry| entry.path == "target"));
    }

    #[test]
    fn wake_report_is_empty_until_env_trigger_equivalent_is_enabled() {
        let root = workspace_root();
        let manifest = load_braxon_context_manifest(&root).unwrap();

        let asleep = braxon_wake_linked_change_report(
            &manifest,
            false,
            vec!["scripts/braxon_seating_verify.sh".to_string()],
        );
        assert!(asleep.suggestions.is_empty());

        let awake = braxon_wake_linked_change_report(
            &manifest,
            true,
            vec!["scripts/braxon_seating_verify.sh".to_string()],
        );
        assert_eq!(awake.changed_path_count, 1);
        assert_eq!(
            awake.suggestions[0].matched_pointer_id,
            "braxon_seating_verify"
        );
        assert!(awake.suggestions[0]
            .pipe_chain
            .iter()
            .any(|link| link.surface == "inspector"
                && link.semantic_id.contains("braxon_seating_verify")
                && !link.numeric_id_base8.is_empty()));
        assert!(!awake.suggestions[0].linked_paths.is_empty());
        assert_eq!(
            awake.suggestions[0]
                .exit_wake_action
                .as_ref()
                .unwrap()
                .action,
            "wake_announce_non_runtime_adjustment"
        );
    }

    #[test]
    fn loaded_chain_db_skips_exit_wake_for_runtime_adjustment_lane() {
        let root = workspace_root();
        let manifest = load_braxon_context_manifest(&root).unwrap();
        let chain_db = load_braxon_chain_root_db(&root, &manifest).unwrap();

        let awake = braxon_wake_linked_change_report_with_db(
            &manifest,
            &chain_db,
            true,
            vec!["state/braxon/runtime_sessions.json".to_string()],
        );

        assert_eq!(awake.changed_path_count, 1);
        assert_eq!(
            awake.suggestions[0].matched_pointer_kind,
            "runtime_adjustment_lane"
        );
        assert!(awake.suggestions[0]
            .pipe_chain
            .iter()
            .all(|link| !link.numeric_id_base8.is_empty()
                && link.semantic_id.contains("runtime_adjustment_lane")));
        assert!(awake.suggestions[0].exit_wake_action.is_none());
        assert_eq!(awake.watermark, "BRAXON_NSQ_BASE8_CONTEXT_WAKE_V1");
    }
}
