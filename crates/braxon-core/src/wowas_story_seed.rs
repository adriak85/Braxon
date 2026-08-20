use crate::content_surface::{NarrativeRecord, NARRATIVE_SCHEMA};
#[cfg(test)]
use nsq_citadel::CoachingMode;
use nsq_citadel::{CitadelInventory, CitadelMaterialization, CitadelNativeRuntime, IntentSeed};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub const WOWAS_STORY_SEED_SCHEMA: &str = "braxon.wowas.story_seed.v2";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WowasStorySeedManifest {
    pub schema: String,
    pub series: String,
    pub book_spine_path: String,
    pub scene_index_path: String,
    pub character_timeline_path: String,
    pub world_map_path: String,
    pub entry_scene_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WowasStorySeed {
    pub schema: String,
    pub series: String,
    pub record_id: String,
    pub book_num: u32,
    pub book_code: String,
    pub book_title: String,
    pub scene_title: String,
    pub scene_description: String,
    pub character_ids: Vec<String>,
    pub source_trace: String,
    pub semantic_intent: String,
    pub universal_intent: String,
    pub book_spine_path: String,
    pub scene_index_path: String,
    pub character_timeline_path: String,
    pub world_map_path: String,
}

impl WowasStorySeedManifest {
    pub fn from_json(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw)
            .map_err(|error| format!("invalid WOWAS story-seed manifest: {error}"))
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != WOWAS_STORY_SEED_SCHEMA {
            return Err("WOWAS story-seed schema mismatch".into());
        }
        for (name, value) in [
            ("series", &self.series),
            ("book_spine_path", &self.book_spine_path),
            ("scene_index_path", &self.scene_index_path),
            ("character_timeline_path", &self.character_timeline_path),
            ("world_map_path", &self.world_map_path),
            ("entry_scene_id", &self.entry_scene_id),
        ] {
            if value.trim().is_empty() {
                return Err(format!("story-seed manifest field {name} is required"));
            }
        }
        Ok(())
    }

    pub fn load_from_root(&self, root: &Path) -> Result<WowasStorySeed, String> {
        self.validate()?;
        let book_spine = read_required(root, &self.book_spine_path)?;
        let scene_index = read_required(root, &self.scene_index_path)?;
        let timeline = read_required(root, &self.character_timeline_path)?;
        let world_map = read_required(root, &self.world_map_path)?;
        WowasStorySeed::from_sources(
            &self.series,
            &self.entry_scene_id,
            &self.book_spine_path,
            &self.scene_index_path,
            &self.character_timeline_path,
            &self.world_map_path,
            &book_spine,
            &scene_index,
            &timeline,
            &world_map,
        )
    }
}

impl WowasStorySeed {
    pub fn materialize_into_nsq(
        &self,
        runtime: &mut CitadelNativeRuntime,
    ) -> Result<(CitadelInventory, CitadelMaterialization), String> {
        let manifest = format!(
            "{{\"schema\":\"braxon.council.full_artifact_seed.v1\",\"lanes\":[{}]}}",
            ["maverick_logic", "qwen_creativity", "arbiter_judge", "analyzer_auditor", "limbic_empath", "support_memory", "image_cortex", "video_cortex", "voice_body", "world_body_3d"]
                .into_iter()
                .map(|lane| format!("{{\"lane\":\"{}\",\"model_id\":\"wowas-story\",\"source_repo\":\"{}\",\"revision\":\"{}\",\"artifact_family\":\"narrative\",\"bus_dialect\":\"intent\",\"semantic_projection\":\"{}\"}}", lane, self.record_id, self.universal_intent, self.universal_intent))
                .collect::<Vec<_>>()
                .join(",")
        );
        runtime
            .materialize_manifest(&manifest)
            .map_err(|error| error.to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_sources(
        series: &str,
        scene_id: &str,
        book_spine_path: &str,
        scene_index_path: &str,
        character_timeline_path: &str,
        world_map_path: &str,
        book_spine: &str,
        scene_index: &str,
        character_timeline: &str,
        world_map: &str,
    ) -> Result<Self, String> {
        if series.trim().is_empty() || scene_id.trim().is_empty() {
            return Err("story seed requires series and scene id".into());
        }
        let spine = parse_tsv(book_spine, "book_spine")?;
        let scenes = parse_tsv(scene_index, "scene_index")?;
        let timeline = parse_tsv(character_timeline, "character_timeline")?;
        if world_map.trim().is_empty() {
            return Err("world map source is empty".into());
        }
        let scene = scenes
            .iter()
            .find(|row| row.get("scene_id").map(String::as_str) == Some(scene_id))
            .ok_or_else(|| format!("scene {scene_id} is absent from canonical scene index"))?;
        let book_num = parse_u32(scene, "book_num")?;
        let book = spine
            .iter()
            .find(|row| parse_u32(row, "book_num").ok() == Some(book_num))
            .ok_or_else(|| format!("book {book_num} is absent from canonical book spine"))?;
        let character_ids = split_refs(
            scene
                .get("inferred_character_ids")
                .map(String::as_str)
                .unwrap_or_default(),
        );
        if character_ids.is_empty() {
            return Err(format!("scene {scene_id} has no canonical character links"));
        }
        if !timeline.iter().any(|row| {
            character_ids
                .iter()
                .any(|id| row.get("character_code") == Some(id))
        }) {
            return Err(format!(
                "scene {scene_id} has no character timeline linkage"
            ));
        }
        let book_title = required(book, "active_title")?;
        let scene_title = required(scene, "clean_title")?;
        let scene_description = required(scene, "brief_scene_description")?;
        let source_trace = required(scene, "source_trace")?;
        let semantic_intent = format!(
            "series={series};book={book_num};book_title={book_title};scene={scene_id};scene_title={scene_title};characters={};description={scene_description};source_trace={source_trace}",
            character_ids.join(",")
        );
        let universal_intent = format!("nsq.intent.v1::{}", stable_hash(&semantic_intent));
        Ok(Self {
            schema: WOWAS_STORY_SEED_SCHEMA.into(),
            series: series.into(),
            record_id: scene_id.into(),
            book_num,
            book_code: required(book, "book_code")?.into(),
            book_title: book_title.into(),
            scene_title: scene_title.into(),
            scene_description: scene_description.into(),
            character_ids,
            source_trace: source_trace.into(),
            semantic_intent,
            universal_intent,
            book_spine_path: book_spine_path.into(),
            scene_index_path: scene_index_path.into(),
            character_timeline_path: character_timeline_path.into(),
            world_map_path: world_map_path.into(),
        })
    }

    pub fn narrative_record(&self) -> NarrativeRecord {
        NarrativeRecord {
            schema: NARRATIVE_SCHEMA.into(),
            record_id: self.record_id.clone(),
            title: self.scene_title.clone(),
            text: self.scene_description.clone(),
            source: "wowas_narrative".into(),
            version: self.schema.clone(),
        }
    }

    pub fn to_intent_seed(&self) -> IntentSeed {
        IntentSeed::new(&self.record_id, &self.semantic_intent)
    }
}

fn read_required(root: &Path, relative: &str) -> Result<String, String> {
    let path = root.join(relative);
    fs::read_to_string(&path)
        .map_err(|error| format!("cannot read story source {}: {error}", path.display()))
}

fn parse_tsv(content: &str, label: &str) -> Result<Vec<BTreeMap<String, String>>, String> {
    let mut lines = content.lines();
    let headers = lines
        .next()
        .ok_or_else(|| format!("{label} source has no header"))?
        .split('\t')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if headers.iter().any(String::is_empty) {
        return Err(format!("{label} source has an empty header"));
    }
    let mut rows = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields = line.split('\t').map(str::to_owned).collect::<Vec<_>>();
        if fields.len() != headers.len() {
            return Err(format!(
                "{label} row has {} fields, expected {}",
                fields.len(),
                headers.len()
            ));
        }
        rows.push(headers.iter().cloned().zip(fields).collect());
    }
    if rows.is_empty() {
        return Err(format!("{label} source has no records"));
    }
    Ok(rows)
}

fn required<'a>(row: &'a BTreeMap<String, String>, field: &str) -> Result<&'a str, String> {
    row.get(field)
        .map(String::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("canonical story field {field} is empty"))
}

fn parse_u32(row: &BTreeMap<String, String>, field: &str) -> Result<u32, String> {
    required(row, field)?
        .parse::<u32>()
        .map_err(|error| format!("invalid {field}: {error}"))
}

fn split_refs(value: &str) -> Vec<String> {
    value
        .split('|')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect()
}

fn stable_hash(value: &str) -> String {
    let mut hash = 14695981039346656037u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1099511628211);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPINE: &str = "book_num\tbook_code\tactive_title\n1\tB01\tChoices Make World\n";
    const SCENES: &str = "scene_id\tbook_num\tclean_title\tbrief_scene_description\tinferred_character_ids\tsource_trace\nB01_C001\t1\tThe Dream\tPip sees the dream\twowas::pip_indalwin_willowjayce|wowas::mack_willow_father\tDIRECT_SOURCE:B01_C001\n";
    const TIMELINE: &str = "character_code\tcanonical_name\nwowas::pip_indalwin_willowjayce\tPip\n";

    #[test]
    fn story_seed_links_real_scene_book_characters_and_intent() {
        let seed = WowasStorySeed::from_sources(
            "Whispers of Willow and Stone",
            "B01_C001",
            "book_spine.tsv",
            "scene_index.tsv",
            "timeline.tsv",
            "world.json",
            SPINE,
            SCENES,
            TIMELINE,
            "{\"schema_version\":1}",
        )
        .unwrap();
        assert_eq!(seed.book_code, "B01");
        assert_eq!(seed.character_ids.len(), 2);
        assert!(seed.semantic_intent.contains("Pip sees the dream"));
        assert_eq!(seed.narrative_record().validate(), Ok(()));
        assert_eq!(seed.to_intent_seed().identity, "B01_C001");
    }

    #[test]
    fn committed_story_seed_manifest_reaches_canonical_scene() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let raw = include_str!("../../../config/wowas/story_seed_manifest.json");
        let manifest = WowasStorySeedManifest::from_json(raw).unwrap();
        let story = manifest.load_from_root(&root).unwrap();
        assert_eq!(story.series, "Whispers of Willow and Stone");
        assert_eq!(story.record_id, "B01_C001");
        assert!(!story.source_trace.is_empty());
    }

    #[test]
    fn real_canonical_scene_reaches_nsq_materialization() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let manifest = WowasStorySeedManifest {
            schema: WOWAS_STORY_SEED_SCHEMA.into(),
            series: "Whispers of Willow and Stone".into(),
            book_spine_path: "crates/wowas-final-edition-v10/canon/active/book_spine_33.tsv".into(),
            scene_index_path: "crates/wowas-final-edition-v10/canon/wowas_clean_scene_index_v2.tsv".into(),
            character_timeline_path: "crates/wowas-final-edition-v10/canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv".into(),
            world_map_path: "crates/wowas-final-edition-v10/canon/canonical_story_tree/world/wowas_world_zone_map.json".into(),
            entry_scene_id: "B01_C001".into(),
        };
        let story = manifest.load_from_root(&root).unwrap();
        let narrative = story.narrative_record();
        narrative.validate().unwrap();
        let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
        let (inventory, materialization) = story.materialize_into_nsq(&mut runtime).unwrap();
        assert_eq!(story.record_id, "B01_C001");
        assert_eq!(inventory.entries.len(), 10);
        assert_eq!(materialization.bodies.len(), 10);
        assert!(story.semantic_intent.contains("B01_C001"));
    }

    #[test]
    fn story_seed_rejects_unlinked_scene() {
        let scenes = SCENES.replace("B01_C001", "B01_C999");
        assert!(WowasStorySeed::from_sources(
            "Whispers of Willow and Stone",
            "B01_C001",
            "b",
            "s",
            "t",
            "w",
            SPINE,
            &scenes,
            TIMELINE,
            "world"
        )
        .is_err());
    }
}
