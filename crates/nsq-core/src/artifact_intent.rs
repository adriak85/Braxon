use crate::dynamic_parameter::stable_hash;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub const ARTIFACT_INTENT_SCHEMA: &str = "nsq.artifact_intent.v1";
pub const COUNCIL_SYNC_SCHEMA: &str = "nsq.council_sync.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum CouncilSurface {
    MaverickLogic,
    QwenCreativity,
    ArbiterJudge,
    AnalyzerAuditor,
    LimbicEmpath,
    SupportMemory,
    ImageCortex,
    VideoCortex,
    VoiceBody,
    WorldBody3d,
}

impl CouncilSurface {
    pub const ALL: [Self; 10] = [
        Self::MaverickLogic,
        Self::QwenCreativity,
        Self::ArbiterJudge,
        Self::AnalyzerAuditor,
        Self::LimbicEmpath,
        Self::SupportMemory,
        Self::ImageCortex,
        Self::VideoCortex,
        Self::VoiceBody,
        Self::WorldBody3d,
    ];

    pub fn dialect(self) -> &'static str {
        match self {
            Self::MaverickLogic => "logic",
            Self::QwenCreativity => "narrative_creativity",
            Self::ArbiterJudge => "judgment",
            Self::AnalyzerAuditor => "audit",
            Self::LimbicEmpath => "affect",
            Self::SupportMemory => "memory",
            Self::ImageCortex => "visual",
            Self::VideoCortex => "temporal_visual",
            Self::VoiceBody => "acoustic",
            Self::WorldBody3d => "spatial",
        }
    }

    pub fn model_or_body(self) -> &'static str {
        match self {
            Self::MaverickLogic => "deepseek-v3-671b",
            Self::QwenCreativity => "qwen3-235b-a22b",
            Self::ArbiterJudge => "qwen2.5-72b",
            Self::AnalyzerAuditor => "deepseek-v3-671b-analyzer",
            Self::LimbicEmpath => "llama3.3-70b",
            Self::SupportMemory => "gemma3-27b",
            Self::ImageCortex => "FLUX.1-dev",
            Self::VideoCortex => "Wan2.1-T2V-14B",
            Self::VoiceBody => "IndexTTS2",
            Self::WorldBody3d => "Hunyuan3D-2.1",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticGradient {
    pub values: BTreeMap<String, f32>,
    pub universal_hash: String,
}

impl SemanticGradient {
    pub fn from_values(values: BTreeMap<String, f32>) -> Result<Self, String> {
        if values.is_empty() {
            return Err("semantic gradient cannot be empty".into());
        }
        if values
            .values()
            .any(|value| !value.is_finite() || !(-1.0..=1.0).contains(value))
        {
            return Err("semantic gradient values must be finite and within -1..=1".into());
        }
        let canonical = values
            .iter()
            .map(|(key, value)| format!("{key}={value:.6}"))
            .collect::<Vec<_>>()
            .join("|");
        Ok(Self {
            universal_hash: stable_hash(&canonical),
            values,
        })
    }

    pub fn aligned(&self, other: &Self) -> bool {
        self.universal_hash == other.universal_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactProvenance {
    pub record_id: String,
    pub source_path: String,
    pub source_format: String,
    pub source_hash: String,
    pub extractor_version: String,
    pub authority: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactIntentRecord {
    pub record_id: String,
    pub artifact_kind: String,
    pub architecture: Option<String>,
    pub metadata: BTreeMap<String, String>,
    pub tensor_count: u64,
    pub tensor_names: Vec<String>,
    pub quantization: Option<String>,
    pub tokenizer_intent: BTreeMap<String, String>,
    pub provenance: ArtifactProvenance,
    pub universal_intent_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialectProjection {
    pub surface: CouncilSurface,
    pub dialect: String,
    pub record_id: String,
    pub local_intent: BTreeMap<String, String>,
    pub universal_intent_hash: String,
    pub gradient_hash: String,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CouncilAlignment {
    pub schema: String,
    pub record_id: String,
    pub universal_intent_hash: String,
    pub gradient: SemanticGradient,
    pub projections: Vec<DialectProjection>,
    pub synchronized_surfaces: BTreeSet<CouncilSurface>,
    pub drifted_surfaces: BTreeSet<CouncilSurface>,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDelta {
    pub record_id: String,
    pub previous_hash: Option<String>,
    pub next_hash: String,
    pub changed_fields: Vec<String>,
    pub activated_surfaces: Vec<CouncilSurface>,
    pub deactivated_surfaces: Vec<CouncilSurface>,
    pub generation: u64,
}

impl ArtifactIntentRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.record_id.is_empty() || self.provenance.record_id != self.record_id {
            return Err("record_id is authoritative and must match provenance".into());
        }
        if self.provenance.source_path.is_empty() || self.provenance.source_hash.is_empty() {
            return Err("artifact provenance is incomplete".into());
        }
        if self.tensor_count != self.tensor_names.len() as u64 {
            return Err("tensor count does not match tensor names".into());
        }
        if self.tensor_names.iter().any(|name| name.is_empty()) {
            return Err("tensor names cannot be empty".into());
        }
        Ok(())
    }

    pub fn canonical_intent(&self) -> String {
        let metadata = self.metadata.iter().map(|(k, v)| format!("m:{k}={v}"));
        let tensors = self.tensor_names.iter().map(|name| format!("t:{name}"));
        metadata.chain(tensors).collect::<Vec<_>>().join("|")
    }
}

impl CouncilAlignment {
    pub fn new(
        artifact: &ArtifactIntentRecord,
        gradient: SemanticGradient,
        generation: u64,
    ) -> Result<Self, String> {
        artifact.validate()?;
        let universal_intent_hash = artifact.universal_intent_hash.clone();
        let projections = CouncilSurface::ALL
            .into_iter()
            .map(|surface| DialectProjection {
                surface,
                dialect: surface.dialect().into(),
                record_id: artifact.record_id.clone(),
                local_intent: BTreeMap::from([
                    ("artifact_kind".into(), artifact.artifact_kind.clone()),
                    (
                        "architecture".into(),
                        artifact.architecture.clone().unwrap_or_default(),
                    ),
                    ("model_or_body".into(), surface.model_or_body().into()),
                    ("native_dialect".into(), surface.dialect().into()),
                ]),
                universal_intent_hash: universal_intent_hash.clone(),
                gradient_hash: gradient.universal_hash.clone(),
                generation,
            })
            .collect::<Vec<_>>();
        Ok(Self {
            schema: COUNCIL_SYNC_SCHEMA.into(),
            record_id: artifact.record_id.clone(),
            universal_intent_hash,
            gradient,
            projections,
            synchronized_surfaces: CouncilSurface::ALL.into_iter().collect(),
            drifted_surfaces: BTreeSet::new(),
            generation,
        })
    }

    pub fn reconcile(
        &mut self,
        observed_hashes: &BTreeMap<CouncilSurface, String>,
    ) -> ArtifactDelta {
        let mut changed_fields = Vec::new();
        let mut activated = Vec::new();
        let mut deactivated = Vec::new();
        self.drifted_surfaces.clear();
        for projection in &mut self.projections {
            match observed_hashes.get(&projection.surface) {
                Some(hash) if hash == &self.universal_intent_hash => {
                    self.synchronized_surfaces.insert(projection.surface);
                }
                Some(hash) => {
                    self.drifted_surfaces.insert(projection.surface);
                    self.synchronized_surfaces.remove(&projection.surface);
                    projection.generation = projection.generation.saturating_add(1);
                    projection.universal_intent_hash = hash.clone();
                    changed_fields.push(format!("surface:{:?}", projection.surface));
                    activated.push(projection.surface);
                }
                None => {
                    self.synchronized_surfaces.remove(&projection.surface);
                    deactivated.push(projection.surface);
                }
            }
        }
        self.generation = self.generation.saturating_add(1);
        ArtifactDelta {
            record_id: self.record_id.clone(),
            previous_hash: Some(self.universal_intent_hash.clone()),
            next_hash: stable_hash(&format!(
                "{}:{}",
                self.universal_intent_hash, self.generation
            )),
            changed_fields,
            activated_surfaces: activated,
            deactivated_surfaces: deactivated,
            generation: self.generation,
        }
    }
}

pub fn artifact_record_id(path: &Path, source_hash: &str) -> String {
    stable_hash(&format!("{}:{}", path.display(), source_hash))
}

/// Extract the semantic loading intent from a GGUF artifact. This reads only the
/// header, metadata, and tensor directory; tensor payload bytes remain on the
/// repository-addressed semantic link and are never copied into the NSQ record.
pub fn extract_gguf_intent(
    bytes: &[u8],
    path: &Path,
    source_hash: &str,
) -> Result<ArtifactIntentRecord, String> {
    let mut cursor = Cursor::new(bytes);
    if cursor.take_bytes(4)? != b"GGUF" {
        return Err("GGUF magic is missing".into());
    }
    let version = cursor.u32()?;
    if !(1..=3).contains(&version) {
        return Err(format!("unsupported GGUF version {version}"));
    }
    let tensor_count = cursor.u64()?;
    let metadata_count = cursor.u64()?;
    let mut metadata = BTreeMap::new();
    let mut architecture = None;
    let mut tokenizer_intent = BTreeMap::new();
    for _ in 0..metadata_count {
        let key = cursor.gguf_string()?;
        let value = cursor.gguf_value()?;
        if key == "general.architecture" {
            architecture = Some(value.clone());
        }
        if key.starts_with("tokenizer.") {
            tokenizer_intent.insert(key.clone(), value.clone());
        }
        metadata.insert(key, value);
    }
    let mut tensor_names = Vec::new();
    let mut tensor_types = BTreeSet::new();
    for _ in 0..tensor_count {
        tensor_names.push(cursor.gguf_string()?);
        let dimensions = cursor.u32()?;
        for _ in 0..dimensions {
            let _ = cursor.u64()?;
        }
        let tensor_type = cursor.u32()?;
        tensor_types.insert(format!("ggml_type_{tensor_type}"));
        let _ = cursor.u64()?;
    }
    let artifact_kind = metadata
        .get("general.type")
        .cloned()
        .unwrap_or_else(|| "model".into());
    let quantization = if tensor_types.is_empty() {
        None
    } else {
        Some(tensor_types.into_iter().collect::<Vec<_>>().join(","))
    };
    let record_id = artifact_record_id(path, source_hash);
    let provenance = ArtifactProvenance {
        record_id: record_id.clone(),
        source_path: path.display().to_string(),
        source_format: "gguf".into(),
        source_hash: source_hash.into(),
        extractor_version: format!("{ARTIFACT_INTENT_SCHEMA}:gguf-v{version}"),
        authority: "NSQ".into(),
    };
    let canonical = format!(
        "format=gguf|version={version}|architecture={}|tensor_count={tensor_count}|metadata={}|tensors={}",
        architecture.clone().unwrap_or_default(),
        metadata.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(","),
        tensor_names.join(",")
    );
    let record = ArtifactIntentRecord {
        record_id,
        artifact_kind,
        architecture,
        metadata,
        tensor_count,
        tensor_names,
        quantization,
        tokenizer_intent,
        provenance,
        universal_intent_hash: stable_hash(&canonical),
    };
    record.validate()?;
    Ok(record)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take_bytes(&mut self, length: usize) -> Result<&'a [u8], String> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or("GGUF offset overflow")?;
        if end > self.bytes.len() {
            return Err("GGUF input ends before a declared field is fully available".into());
        }
        let result = &self.bytes[self.offset..end];
        self.offset = end;
        Ok(result)
    }

    fn u32(&mut self) -> Result<u32, String> {
        let mut value = [0u8; 4];
        value.copy_from_slice(self.take_bytes(4)?);
        Ok(u32::from_le_bytes(value))
    }

    fn u64(&mut self) -> Result<u64, String> {
        let mut value = [0u8; 8];
        value.copy_from_slice(self.take_bytes(8)?);
        Ok(u64::from_le_bytes(value))
    }

    fn gguf_string(&mut self) -> Result<String, String> {
        let length = self.u64()?;
        let length = usize::try_from(length).map_err(|_| "GGUF string length exceeds host size")?;
        let bytes = self.take_bytes(length)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| "GGUF string is not valid UTF-8".into())
    }

    fn gguf_value(&mut self) -> Result<String, String> {
        let value_type = self.u32()?;
        match value_type {
            0 => Ok(self.take_bytes(1)?[0].to_string()),
            1 => Ok(self.take_bytes(1)?[0].to_string()),
            2 => Ok(self.take_bytes(1)?[0].to_string()),
            3 => Ok(self.take_bytes(1)?[0].to_string()),
            4 => Ok(i16::from_le_bytes(self.take_bytes(2)?.try_into().unwrap()).to_string()),
            5 => Ok(u16::from_le_bytes(self.take_bytes(2)?.try_into().unwrap()).to_string()),
            6 => Ok(i32::from_le_bytes(self.take_bytes(4)?.try_into().unwrap()).to_string()),
            7 => Ok(u32::from_le_bytes(self.take_bytes(4)?.try_into().unwrap()).to_string()),
            8 => self.gguf_string(),
            9 => {
                let element_type = self.u32()?;
                let count = usize::try_from(self.u64()?).map_err(|_| "GGUF array is too large")?;
                let mut values = Vec::with_capacity(count.min(64));
                for index in 0..count {
                    let value = self.gguf_array_value(element_type)?;
                    if index < 64 {
                        values.push(value);
                    }
                }
                Ok(format!("[{}]", values.join(",")))
            }
            10 => Ok(i64::from_le_bytes(self.take_bytes(8)?.try_into().unwrap()).to_string()),
            11 => Ok(u64::from_le_bytes(self.take_bytes(8)?.try_into().unwrap()).to_string()),
            12 => Ok(format!(
                "{}",
                f32::from_le_bytes(self.take_bytes(4)?.try_into().unwrap())
            )),
            13 => Ok(format!(
                "{}",
                f64::from_le_bytes(self.take_bytes(8)?.try_into().unwrap())
            )),
            14 => Ok(self.take_bytes(1)?[0].to_string()),
            _ => Err(format!("unsupported GGUF metadata value type {value_type}")),
        }
    }

    fn gguf_array_value(&mut self, value_type: u32) -> Result<String, String> {
        match value_type {
            8 => self.gguf_string(),
            0 => Ok(self.take_bytes(1)?[0].to_string()),
            1 => Ok(self.take_bytes(1)?[0].to_string()),
            2 => Ok(self.take_bytes(1)?[0].to_string()),
            3 => Ok(self.take_bytes(1)?[0].to_string()),
            4 => Ok(i16::from_le_bytes(self.take_bytes(2)?.try_into().unwrap()).to_string()),
            5 => Ok(u16::from_le_bytes(self.take_bytes(2)?.try_into().unwrap()).to_string()),
            6 => Ok(i32::from_le_bytes(self.take_bytes(4)?.try_into().unwrap()).to_string()),
            7 => Ok(u32::from_le_bytes(self.take_bytes(4)?.try_into().unwrap()).to_string()),
            10 => Ok(i64::from_le_bytes(self.take_bytes(8)?.try_into().unwrap()).to_string()),
            11 => Ok(u64::from_le_bytes(self.take_bytes(8)?.try_into().unwrap()).to_string()),
            12 => Ok(format!(
                "{}",
                f32::from_le_bytes(self.take_bytes(4)?.try_into().unwrap())
            )),
            13 => Ok(format!(
                "{}",
                f64::from_le_bytes(self.take_bytes(8)?.try_into().unwrap())
            )),
            14 => Ok(self.take_bytes(1)?[0].to_string()),
            _ => Err(format!("unsupported GGUF array value type {value_type}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn artifact() -> ArtifactIntentRecord {
        let provenance = ArtifactProvenance {
            record_id: "artifact-1".into(),
            source_path: "model.gguf".into(),
            source_format: "gguf".into(),
            source_hash: "source-hash".into(),
            extractor_version: "test".into(),
            authority: "NSQ".into(),
        };
        ArtifactIntentRecord {
            record_id: "artifact-1".into(),
            artifact_kind: "model".into(),
            architecture: Some("llama".into()),
            metadata: BTreeMap::from([("general.architecture".into(), "llama".into())]),
            tensor_count: 1,
            tensor_names: vec!["token_embd.weight".into()],
            quantization: Some("Q4_K_M".into()),
            tokenizer_intent: BTreeMap::new(),
            provenance,
            universal_intent_hash: "universal".into(),
        }
    }

    #[test]
    fn council_has_ten_native_dialects_over_one_universal_intent() {
        let gradient = SemanticGradient::from_values(BTreeMap::from([
            ("logic".into(), 0.2),
            ("affect".into(), 0.4),
            ("spatial".into(), -0.1),
        ]))
        .unwrap();
        let alignment = CouncilAlignment::new(&artifact(), gradient, 0).unwrap();
        assert_eq!(alignment.projections.len(), 10);
        assert_eq!(alignment.synchronized_surfaces.len(), 10);
        assert!(alignment.drifted_surfaces.is_empty());
        assert!(alignment
            .projections
            .iter()
            .all(|p| p.universal_intent_hash == "universal"));
    }

    #[test]
    fn delta_reconciliation_marks_drift_and_missing_surfaces() {
        let gradient =
            SemanticGradient::from_values(BTreeMap::from([("logic".into(), 0.5)])).unwrap();
        let mut alignment = CouncilAlignment::new(&artifact(), gradient, 0).unwrap();
        let observed = BTreeMap::from([
            (CouncilSurface::MaverickLogic, "changed".into()),
            (CouncilSurface::QwenCreativity, "universal".into()),
        ]);
        let delta = alignment.reconcile(&observed);
        assert!(alignment
            .drifted_surfaces
            .contains(&CouncilSurface::MaverickLogic));
        assert_eq!(alignment.synchronized_surfaces.len(), 1);
        assert_eq!(
            delta.activated_surfaces,
            vec![CouncilSurface::MaverickLogic]
        );
        assert_eq!(delta.deactivated_surfaces.len(), 8);
        assert_eq!(delta.generation, 1);
    }

    #[test]
    fn record_id_is_authoritative() {
        let mut value = artifact();
        value.provenance.record_id = "wrong".into();
        assert!(value.validate().is_err());
    }
}
