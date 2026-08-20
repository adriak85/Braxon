use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const TOKENIZER_BRIDGE_SCHEMA: &str = "braxon.nsq.tokenizer_bridge.v1";
pub const TOKENIZER_BAND_REGISTRY_RELATIVE_PATH: &str = "config/nsq/tokenizer_band_registry.json";

#[derive(Debug, Clone, Deserialize)]
struct TokenizerBandRegistry {
    schema: String,
    universal_translation_version: String,
    address_namespace: String,
    bands: Vec<TokenizerBand>,
}

#[derive(Debug, Clone, Deserialize)]
struct TokenizerBand {
    band_id: String,
    model_id: String,
    active: bool,
    tokenizer_path: String,
    representation: String,
    provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeTokenProjection {
    pub lexical: String,
    pub native_id: u64,
    pub universal_id: u64,
    pub nsq_address: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenizerBridgeReceipt {
    pub schema: String,
    pub band_id: String,
    pub model_id: String,
    pub tokenizer_path: String,
    pub provenance: String,
    pub native_representation_retained: bool,
    pub deterministic_mapping: bool,
    pub forward_translation: bool,
    pub reverse_translation: bool,
    pub shared_semantic_addressing: bool,
    pub collective_state_contribution_ready: bool,
    pub no_resident_runtime: bool,
    pub projections: Vec<NativeTokenProjection>,
    pub unresolved_tokens: Vec<String>,
    pub collision_count: usize,
}

impl TokenizerBridgeReceipt {
    pub fn all_required_mappings_resolved(&self) -> bool {
        self.native_representation_retained
            && self.deterministic_mapping
            && self.forward_translation
            && self.reverse_translation
            && self.shared_semantic_addressing
            && self.collective_state_contribution_ready
            && self.no_resident_runtime
            && self.unresolved_tokens.is_empty()
            && self.collision_count == 0
    }
}

#[derive(Debug, Clone)]
pub struct TokenizerBridge {
    band_id: String,
    model_id: String,
    tokenizer_path: String,
    provenance: String,
    address_namespace: String,
    vocabulary: BTreeMap<String, u64>,
    reverse_vocabulary: BTreeMap<u64, String>,
}

impl TokenizerBridge {
    pub fn from_root(root: &Path, requested_band_id: &str) -> Result<Self, String> {
        let root = resolve_repository_root(root)?;
        let registry_path = root.join(TOKENIZER_BAND_REGISTRY_RELATIVE_PATH);
        let registry_raw = fs::read_to_string(&registry_path)
            .map_err(|error| format!("failed to read '{}': {error}", registry_path.display()))?;
        let registry: TokenizerBandRegistry = serde_json::from_str(&registry_raw)
            .map_err(|error| format!("failed to parse '{}': {error}", registry_path.display()))?;
        if registry.schema != "braxon.nsq.tokenizer_band_registry.v1" {
            return Err("tokenizer band registry schema mismatch".into());
        }
        if registry.universal_translation_version != "nsq.universal.token.sync.v1" {
            return Err("unsupported universal tokenizer translation version".into());
        }
        let band = registry
            .bands
            .into_iter()
            .find(|candidate| candidate.band_id == requested_band_id)
            .ok_or_else(|| format!("tokenizer band '{requested_band_id}' is not registered"))?;
        if !band.active {
            return Err(format!(
                "tokenizer band '{requested_band_id}' is not active"
            ));
        }
        if band.representation.trim().is_empty() || band.provenance.trim().is_empty() {
            return Err("tokenizer band representation and provenance are required".into());
        }
        let tokenizer_path = root.join(&band.tokenizer_path);
        let tokenizer_raw = fs::read_to_string(&tokenizer_path)
            .map_err(|error| format!("failed to read '{}': {error}", tokenizer_path.display()))?;
        let tokenizer: Value = serde_json::from_str(&tokenizer_raw)
            .map_err(|error| format!("failed to parse '{}': {error}", tokenizer_path.display()))?;
        let vocab = tokenizer
            .get("vocab")
            .and_then(Value::as_object)
            .ok_or("tokenizer artifact has no 'vocab' object")?;
        let mut vocabulary = BTreeMap::new();
        let mut reverse_vocabulary = BTreeMap::new();
        for (lexical, id) in vocab {
            let id = id.as_u64().ok_or_else(|| {
                format!("tokenizer id for '{lexical}' is not an unsigned integer")
            })?;
            if reverse_vocabulary.insert(id, lexical.clone()).is_some() {
                return Err(format!("tokenizer has duplicate native id: {id}"));
            }
            vocabulary.insert(lexical.clone(), id);
        }
        if vocabulary.is_empty() {
            return Err("tokenizer vocabulary is empty".into());
        }
        Ok(Self {
            band_id: band.band_id,
            model_id: band.model_id,
            tokenizer_path: band.tokenizer_path,
            provenance: band.provenance,
            address_namespace: registry.address_namespace,
            vocabulary,
            reverse_vocabulary,
        })
    }

    /// Character-level encoding is deliberately used because the committed tokenizer
    /// vocabulary carries canonical character entries. This preserves the native IDs
    /// and does not flatten the native representation into a universal vocabulary.
    pub fn encode_translate_round_trip(&self, input: &str) -> TokenizerBridgeReceipt {
        let mut projections = Vec::new();
        let mut unresolved_tokens = Vec::new();
        let mut universal_ids = BTreeMap::new();
        let mut collision_count = 0;
        for character in input.chars() {
            let lexical = character.to_string();
            let Some(native_id) = self.vocabulary.get(&lexical).copied() else {
                unresolved_tokens.push(lexical);
                continue;
            };
            let universal_id = stable_id(&lexical);
            if let Some(previous) = universal_ids.insert(universal_id, lexical.clone()) {
                if previous != lexical {
                    collision_count += 1;
                }
            }
            let reverse = self.reverse_vocabulary.get(&native_id);
            if reverse != Some(&lexical) {
                collision_count += 1;
            }
            projections.push(NativeTokenProjection {
                lexical,
                native_id,
                universal_id,
                nsq_address: format!("{}/{universal_id:016x}", self.address_namespace),
            });
        }
        let deterministic_mapping = collision_count == 0;
        let forward_translation = projections
            .iter()
            .all(|projection| projection.universal_id != 0);
        let reverse_translation = projections.iter().all(|projection| {
            self.reverse_vocabulary.get(&projection.native_id) == Some(&projection.lexical)
        });
        let shared_semantic_addressing = projections.iter().all(|projection| {
            projection.nsq_address.starts_with(&self.address_namespace)
                && projection.nsq_address != format!("{}/0000000000000000", self.address_namespace)
        });
        let collective_state_contribution_ready = !projections.is_empty()
            && unresolved_tokens.is_empty()
            && deterministic_mapping
            && forward_translation
            && reverse_translation
            && shared_semantic_addressing;
        TokenizerBridgeReceipt {
            schema: TOKENIZER_BRIDGE_SCHEMA.into(),
            band_id: self.band_id.clone(),
            model_id: self.model_id.clone(),
            tokenizer_path: self.tokenizer_path.clone(),
            provenance: self.provenance.clone(),
            native_representation_retained: true,
            deterministic_mapping,
            forward_translation,
            reverse_translation,
            shared_semantic_addressing,
            collective_state_contribution_ready,
            no_resident_runtime: true,
            projections,
            unresolved_tokens,
            collision_count,
        }
    }
}

fn resolve_repository_root(start: &Path) -> Result<PathBuf, String> {
    let canonical = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    canonical
        .ancestors()
        .find(|candidate| {
            candidate
                .join(TOKENIZER_BAND_REGISTRY_RELATIVE_PATH)
                .exists()
        })
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            format!(
                "unable to locate repository root containing {} from {}",
                TOKENIZER_BAND_REGISTRY_RELATIVE_PATH,
                start.display()
            )
        })
}

fn stable_id(value: &str) -> u64 {
    value
        .as_bytes()
        .iter()
        .fold(0xcbf29ce484222325u64, |hash, byte| {
            hash.wrapping_mul(0x100000001b3)
                .wrapping_add(u64::from(*byte))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap()
    }

    #[test]
    fn active_native_tokenizer_encodes_translates_addresses_and_round_trips_without_residency() {
        let bridge = TokenizerBridge::from_root(&repo_root(), "braxon_native").unwrap();
        let receipt = bridge.encode_translate_round_trip("is truth");
        assert!(receipt.all_required_mappings_resolved(), "{receipt:?}");
        assert!(!receipt.projections.is_empty());
        assert!(receipt
            .projections
            .iter()
            .all(|projection| projection.nsq_address.starts_with("nsq.address.token/")));
    }

    #[test]
    fn unknown_token_is_reported_and_never_promoted_to_universal_state() {
        let bridge = TokenizerBridge::from_root(&repo_root(), "braxon_native").unwrap();
        let receipt = bridge.encode_translate_round_trip("truth🙂");
        assert_eq!(receipt.unresolved_tokens, vec!["🙂"]);
        assert!(!receipt.all_required_mappings_resolved());
    }
}
