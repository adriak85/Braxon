use crate::offline_models::load_or_initialize_model_registry;
use crate::wowas_realization::{WowasRealization, WowasRealizedPacket};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const WOWAS_GENERATION_SCHEMA: &str = "braxon.wowas.generation.v1";
pub const WOWAS_ACCEPTED_PROSE_RELATIVE_PATH: &str = "state/wowas/accepted_prose";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WowasGenerationReadiness {
    Ready { seated_poles: usize },
    Blocked { reasons: Vec<String> },
}

impl WowasGenerationReadiness {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WowasGenerationRequest {
    pub schema: String,
    pub request_id: String,
    pub packet_id: String,
    pub book_num: u32,
    pub book_code: String,
    pub scene_title: String,
    pub ordered_intent: String,
    pub source_character_id: String,
    pub source_character_name: String,
    pub source_role: String,
    pub source_region: String,
    pub source_anchor: String,
    pub encounter_id: String,
    pub event_id: String,
    pub prose_gate: String,
    pub system_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WowasCandidateProse {
    pub schema: String,
    pub packet_id: String,
    pub model_id: String,
    pub model_revision: String,
    pub source_trace: String,
    pub text: String,
    pub candidate_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WowasAcceptedProse {
    pub schema: String,
    pub packet_id: String,
    pub candidate_hash: String,
    pub text: String,
    pub model_id: String,
    pub model_revision: String,
    pub source_trace: String,
    pub promoted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WowasGenerationRun {
    pub schema: String,
    pub readiness: WowasGenerationReadiness,
    pub request_count: usize,
    pub requests: Vec<WowasGenerationRequest>,
    pub request_hash: String,
}

pub fn assess_generation_readiness(root: &Path) -> WowasGenerationReadiness {
    let registry = load_or_initialize_model_registry(root);
    let seating = registry.build_court_seating();
    let mut reasons = Vec::new();
    if !seating.council_ready() {
        reasons.push(format!(
            "Council seating is not complete: {}/10 poles operational",
            seating.operational_count()
        ));
    }
    for asset in &registry.assets {
        if asset.pipeline_stage.ready_for_seating() && !asset.pole_seated {
            reasons.push(format!(
                "pole {} has NSQ-ready asset but is not seated",
                asset.target_pole
            ));
        }
        if asset.pole_seated && !asset.source_ingest_path.exists() {
            reasons.push(format!(
                "seated pole {} source weights are absent",
                asset.target_pole
            ));
        }
    }
    if reasons.is_empty() {
        WowasGenerationReadiness::Ready {
            seated_poles: seating.operational_count(),
        }
    } else {
        WowasGenerationReadiness::Blocked { reasons }
    }
}

pub fn prepare_generation_run(
    realization: &WowasRealization,
    root: &Path,
) -> Result<WowasGenerationRun, String> {
    realization.validate()?;
    let requests = realization
        .packets
        .iter()
        .map(request_for_packet)
        .collect::<Vec<_>>();
    let request_bytes = serde_json::to_vec(&requests)
        .map_err(|e| format!("cannot serialize generation requests: {e}"))?;
    Ok(WowasGenerationRun {
        schema: WOWAS_GENERATION_SCHEMA.into(),
        readiness: assess_generation_readiness(root),
        request_count: requests.len(),
        requests,
        request_hash: stable_hash_bytes(&request_bytes),
    })
}

pub fn validate_candidate(
    realization: &WowasRealization,
    candidate: &WowasCandidateProse,
) -> Result<(), String> {
    if candidate.schema != WOWAS_GENERATION_SCHEMA {
        return Err("candidate schema mismatch".into());
    }
    let packet = realization
        .packets
        .iter()
        .find(|p| p.packet_id == candidate.packet_id)
        .ok_or_else(|| {
            format!(
                "candidate references unknown packet {}",
                candidate.packet_id
            )
        })?;
    if candidate.text.trim().is_empty() {
        return Err("candidate prose is empty".into());
    }
    if candidate.model_id.trim().is_empty() || candidate.model_revision.trim().is_empty() {
        return Err("candidate model identity is incomplete".into());
    }
    if candidate.source_trace.trim().is_empty() {
        return Err("candidate source trace is required".into());
    }
    if candidate.text.contains("Boojay")
        || candidate.text.contains("Riledge")
        || candidate.text.contains("Chrono decay")
    {
        return Err("candidate contains blocked legacy or drift material".into());
    }
    if candidate.candidate_hash != stable_hash(candidate.text.as_bytes()) {
        return Err("candidate hash mismatch".into());
    }
    if packet.prose_gate != "canonical_scene_existing"
        && packet.prose_gate != "requires_prose_realization"
    {
        return Err(format!(
            "packet {} has invalid prose gate",
            packet.packet_id
        ));
    }
    Ok(())
}

pub fn persist_accepted_candidate(
    root: &Path,
    realization: &WowasRealization,
    candidate: &WowasCandidateProse,
) -> Result<PathBuf, String> {
    validate_candidate(realization, candidate)?;
    let body = WowasAcceptedProse {
        schema: WOWAS_GENERATION_SCHEMA.into(),
        packet_id: candidate.packet_id.clone(),
        candidate_hash: candidate.candidate_hash.clone(),
        text: candidate.text.clone(),
        model_id: candidate.model_id.clone(),
        model_revision: candidate.model_revision.clone(),
        source_trace: candidate.source_trace.clone(),
        promoted: true,
    };
    let dir = root.join(WOWAS_ACCEPTED_PROSE_RELATIVE_PATH);
    fs::create_dir_all(&dir).map_err(|e| format!("cannot create accepted prose directory: {e}"))?;
    let path = dir.join(format!("{}.json", candidate.packet_id));
    let raw = serde_json::to_string_pretty(&body)
        .map_err(|e| format!("cannot serialize accepted prose: {e}"))?;
    fs::write(&path, raw).map_err(|e| format!("cannot persist accepted prose: {e}"))?;
    Ok(path)
}

fn request_for_packet(packet: &WowasRealizedPacket) -> WowasGenerationRequest {
    WowasGenerationRequest { schema: WOWAS_GENERATION_SCHEMA.into(), request_id: format!("request:{}", stable_hash(packet.packet_id.as_bytes())), packet_id: packet.packet_id.clone(), book_num: packet.book_num, book_code: packet.book_code.clone(), scene_title: packet.title.clone(), ordered_intent: packet.core_intent.clone(), source_character_id: packet.source_character_id.clone(), source_character_name: packet.source_character_name.clone(), source_role: packet.source_role.clone(), source_region: packet.source_region.clone(), source_anchor: packet.source_anchor.clone(), encounter_id: packet.encounter_id.clone(), event_id: packet.event_id.clone(), prose_gate: packet.prose_gate.clone(), system_policy: "Use only supplied canonical packet and validated lattice; never invent deprecated canon; return candidate only; acceptance is separate.".into() }
}
fn stable_hash(value: &[u8]) -> String {
    stable_hash_bytes(value)
}
fn stable_hash_bytes(value: &[u8]) -> String {
    let mut hash = 14695981039346656037u64;
    for byte in value {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1099511628211);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wowas_realization::WowasRealization;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn realization() -> WowasRealization {
        WowasRealization::from_ordered_manifest(include_str!(
            "../../../config/wowas/ordered_stretched_spine_manifest.json"
        ))
        .unwrap()
    }
    #[test]
    fn prepares_all_ordered_packets_without_model_shortcut() {
        let r = realization();
        let run = prepare_generation_run(&r, Path::new("/definitely/missing/model/root")).unwrap();
        assert_eq!(run.request_count, 535);
        assert!(!run.readiness.is_ready());
        assert!(matches!(
            run.readiness,
            WowasGenerationReadiness::Blocked { .. }
        ));
        assert_eq!(run.requests[0].packet_id, "B01_C001");
    }
    #[test]
    fn candidate_validation_rejects_bad_hash_and_blocked_material() {
        let r = realization();
        let mut c = WowasCandidateProse {
            schema: WOWAS_GENERATION_SCHEMA.into(),
            packet_id: "B01_C001".into(),
            model_id: "real-braxon".into(),
            model_revision: "r1".into(),
            source_trace: "model://real-braxon/r1".into(),
            text: "Boojay returns.".into(),
            candidate_hash: stable_hash(b"Boojay returns."),
        };
        assert!(validate_candidate(&r, &c).is_err());
        c.text = "A valid candidate.".into();
        c.candidate_hash = "wrong".into();
        assert!(validate_candidate(&r, &c).is_err());
    }
    #[test]
    fn accepted_candidate_persists_only_after_validation() {
        let r = realization();
        let text = "The dream holds its blue light.";
        let c = WowasCandidateProse {
            schema: WOWAS_GENERATION_SCHEMA.into(),
            packet_id: "B01_C001".into(),
            model_id: "real-braxon".into(),
            model_revision: "r1".into(),
            source_trace: "model://real-braxon/r1".into(),
            text: text.into(),
            candidate_hash: stable_hash(text.as_bytes()),
        };
        let root = std::env::temp_dir().join(format!(
            "wowas-generation-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let path = persist_accepted_candidate(&root, &r, &c).unwrap();
        assert!(path.exists());
        let _ = fs::remove_dir_all(root);
    }
}
