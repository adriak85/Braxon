use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const NATIVE_MODEL_SUBSTRATE_SCHEMA: &str = "braxon.nsq.native_model_substrate.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NativeDType {
    I64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeArtifactManifest {
    pub record_id: String,
    pub artifact_hash: String,
    pub architecture: String,
    pub tokenizer_hash: String,
    pub dtype: NativeDType,
    pub tensor_count: u64,
    pub max_context: u64,
    pub provenance: String,
    pub license_ref: String,
}

impl NativeArtifactManifest {
    pub fn validate(&self) -> Result<(), String> {
        if self.record_id.trim().is_empty() || self.artifact_hash.trim().is_empty() {
            return Err("artifact identity and hash are required".into());
        }
        if self.architecture.trim().is_empty() || self.tokenizer_hash.trim().is_empty() {
            return Err("architecture and tokenizer hash are required".into());
        }
        if self.tensor_count == 0 || self.max_context == 0 {
            return Err("artifact tensor count and context must be nonzero".into());
        }
        if self.provenance.trim().is_empty() || self.license_ref.trim().is_empty() {
            return Err("artifact provenance and license are required".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeTensorBlock {
    pub record_id: String,
    pub shape: Vec<usize>,
    pub dtype: NativeDType,
    pub values: Vec<i64>,
    pub generation: u64,
    pub provenance: String,
}

impl NativeTensorBlock {
    pub fn new(
        record_id: impl Into<String>,
        shape: Vec<usize>,
        values: Vec<i64>,
        provenance: impl Into<String>,
    ) -> Result<Self, String> {
        let expected = shape
            .iter()
            .try_fold(1usize, |acc, dim| acc.checked_mul(*dim))
            .ok_or("tensor shape overflow")?;
        if expected != values.len() {
            return Err("tensor value count does not match shape".into());
        }
        let record_id = record_id.into();
        if record_id.trim().is_empty() {
            return Err("tensor record_id is required".into());
        }
        Ok(Self {
            record_id,
            shape,
            dtype: NativeDType::I64,
            values,
            generation: 0,
            provenance: provenance.into(),
        })
    }
    pub fn bytes(&self) -> u64 {
        (self.values.len() as u64).saturating_mul(8)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeTokenizer {
    pub record_id: String,
    pub vocabulary: BTreeMap<String, u32>,
    pub unknown_token: u32,
    pub provenance: String,
}

impl NativeTokenizer {
    pub fn validate(&self) -> Result<(), String> {
        if self.record_id.trim().is_empty() || self.provenance.trim().is_empty() {
            return Err("tokenizer identity and provenance are required".into());
        }
        if self.vocabulary.is_empty() {
            return Err("tokenizer vocabulary cannot be empty".into());
        }
        Ok(())
    }
    pub fn encode(&self, input: &str) -> Result<Vec<u32>, String> {
        self.validate()?;
        Ok(input
            .split_whitespace()
            .map(|token| {
                self.vocabulary
                    .get(token)
                    .copied()
                    .unwrap_or(self.unknown_token)
            })
            .collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KvEntry {
    pub record_id: String,
    pub position: u64,
    pub value: i64,
    pub bytes: u64,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeKvCache {
    pub record_id: String,
    pub capacity_bytes: u64,
    pub active_bytes: u64,
    pub generation: u64,
    pub entries: BTreeMap<String, KvEntry>,
}

impl NativeKvCache {
    pub fn new(record_id: impl Into<String>, capacity_bytes: u64) -> Result<Self, String> {
        if capacity_bytes == 0 {
            return Err("KV cache capacity must be nonzero".into());
        }
        Ok(Self {
            record_id: record_id.into(),
            capacity_bytes,
            active_bytes: 0,
            generation: 0,
            entries: BTreeMap::new(),
        })
    }
    pub fn append(
        &mut self,
        entry_id: impl Into<String>,
        position: u64,
        value: i64,
    ) -> Result<(), String> {
        let entry_id = entry_id.into();
        if entry_id.trim().is_empty() {
            return Err("KV entry record_id is required".into());
        }
        let bytes = 8;
        if self.active_bytes.saturating_add(bytes) > self.capacity_bytes {
            return Err("KV cache pressure requires release before activation".into());
        }
        self.generation = self.generation.saturating_add(1);
        self.entries.insert(
            entry_id.clone(),
            KvEntry {
                record_id: entry_id,
                position,
                value,
                bytes,
                generation: self.generation,
            },
        );
        self.active_bytes = self.active_bytes.saturating_add(bytes);
        Ok(())
    }
    pub fn release(&mut self, entry_id: &str) -> Result<(), String> {
        let entry = self.entries.remove(entry_id).ok_or("KV entry not found")?;
        self.active_bytes = self.active_bytes.saturating_sub(entry.bytes);
        self.generation = self.generation.saturating_add(1);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeLinearModel {
    pub manifest: NativeArtifactManifest,
    pub parameters: NativeTensorBlock,
    pub tokenizer: NativeTokenizer,
    pub kv_cache: NativeKvCache,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeInferenceReceipt {
    pub record_id: String,
    pub token_count: u64,
    pub output: i64,
    pub active_parameter_bytes: u64,
    pub active_kv_bytes: u64,
    pub generation: u64,
    pub deterministic_hash: String,
}

impl NativeLinearModel {
    pub fn fixture() -> Result<Self, String> {
        let manifest = NativeArtifactManifest {
            record_id: "fixture-model".into(),
            artifact_hash: "fixture-model-hash".into(),
            architecture: "native-linear".into(),
            tokenizer_hash: "fixture-tokenizer-hash".into(),
            dtype: NativeDType::I64,
            tensor_count: 1,
            max_context: 64,
            provenance: "native-fixture".into(),
            license_ref: "test-fixture".into(),
        };
        manifest.validate()?;
        let tokenizer = NativeTokenizer {
            record_id: "fixture-tokenizer".into(),
            vocabulary: [("one".into(), 1), ("two".into(), 2), ("three".into(), 3)]
                .into_iter()
                .collect(),
            unknown_token: 0,
            provenance: "native-fixture".into(),
        };
        let parameters =
            NativeTensorBlock::new("fixture-weights", vec![3], vec![2, 3, 5], "native-fixture")?;
        Ok(Self {
            manifest,
            parameters,
            tokenizer,
            kv_cache: NativeKvCache::new("fixture-kv", 64)?,
            generation: 0,
        })
    }
    pub fn infer(
        &mut self,
        record_id: impl Into<String>,
        input: &str,
    ) -> Result<NativeInferenceReceipt, String> {
        let record_id = record_id.into();
        let tokens = self.tokenizer.encode(input)?;
        let mut output = 0i64;
        for (position, token) in tokens.iter().enumerate() {
            let index = (*token as usize) % self.parameters.values.len();
            output = output.saturating_add(self.parameters.values[index]);
            self.kv_cache.append(
                format!("{record_id}:kv:{position}"),
                position as u64,
                i64::from(*token),
            )?;
        }
        self.generation = self.generation.saturating_add(1);
        Ok(NativeInferenceReceipt {
            record_id,
            token_count: tokens.len() as u64,
            output,
            active_parameter_bytes: self.parameters.bytes(),
            active_kv_bytes: self.kv_cache.active_bytes,
            generation: self.generation,
            deterministic_hash: stable_hash(&format!(
                "{}:{}:{}",
                self.manifest.artifact_hash, output, self.generation
            )),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NativeTrainingPath {
    Dense,
    Reactive,
    Predictive,
    PersistentPredictive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeTrainingSample {
    pub input: Vec<i64>,
    pub target: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeTrainingReceipt {
    pub path: NativeTrainingPath,
    pub steps: u64,
    pub final_loss: i64,
    pub final_parameters: Vec<i64>,
    pub parameters_examined: u64,
    pub parameters_updated: u64,
    pub prediction_hits: u64,
    pub prediction_misses: u64,
    pub correction_events: u64,
    pub checkpoint_generation: u64,
    pub deterministic_hash: String,
}

pub fn run_native_training(
    path: NativeTrainingPath,
    samples: &[NativeTrainingSample],
    prediction_accuracy_percent: u64,
) -> Result<NativeTrainingReceipt, String> {
    if samples.is_empty() || prediction_accuracy_percent > 100 {
        return Err("training samples must be nonempty and accuracy must be 0..=100".into());
    }
    let mut parameters = vec![1i64; samples[0].input.len()];
    let mut prediction_hits = 0u64;
    let mut prediction_misses = 0u64;
    let mut corrections = 0u64;
    let mut loss = 0i64;
    for (step, sample) in samples.iter().enumerate() {
        if sample.input.len() != parameters.len() {
            return Err("training sample width mismatch".into());
        }
        let prediction = sample
            .input
            .iter()
            .zip(parameters.iter())
            .map(|(x, p)| x.saturating_mul(*p))
            .sum::<i64>();
        let error = sample.target.saturating_sub(prediction);
        loss = error.saturating_mul(error);
        let gradient: Vec<i64> = sample
            .input
            .iter()
            .map(|x| x.saturating_mul(error))
            .collect();
        let predicted_gradient = if matches!(
            path,
            NativeTrainingPath::Predictive | NativeTrainingPath::PersistentPredictive
        ) {
            if ((step as u64 * 37) % 100) < prediction_accuracy_percent {
                gradient.clone()
            } else {
                gradient
                    .iter()
                    .map(|value| value.saturating_add(1))
                    .collect()
            }
        } else {
            gradient.clone()
        };
        if matches!(
            path,
            NativeTrainingPath::Predictive | NativeTrainingPath::PersistentPredictive
        ) {
            for (predicted, actual) in predicted_gradient.iter().zip(gradient.iter()) {
                if predicted == actual {
                    prediction_hits += 1;
                } else {
                    prediction_misses += 1;
                    corrections += 1;
                }
            }
        }
        for (parameter, gradient_value) in parameters.iter_mut().zip(gradient.iter()) {
            *parameter = parameter.saturating_add(*gradient_value / 10);
        }
    }
    let checkpoint_generation = samples.len() as u64;
    Ok(NativeTrainingReceipt {
        path,
        steps: samples.len() as u64,
        final_loss: loss,
        final_parameters: parameters.clone(),
        parameters_examined: (samples.len() * parameters.len()) as u64,
        parameters_updated: (samples.len() * parameters.len()) as u64,
        prediction_hits,
        prediction_misses,
        correction_events: corrections,
        checkpoint_generation,
        deterministic_hash: stable_hash(&format!("{:?}:{}", parameters, loss)),
    })
}

pub fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn artifact_tokenizer_tensor_and_kv_contracts_fail_closed() {
        let model = NativeLinearModel::fixture().unwrap();
        assert_eq!(
            model.tokenizer.encode("one two unknown").unwrap(),
            vec![1, 2, 0]
        );
        assert_eq!(model.parameters.bytes(), 24);
        let mut kv = NativeKvCache::new("kv", 16).unwrap();
        kv.append("a", 0, 1).unwrap();
        kv.append("b", 1, 2).unwrap();
        assert!(kv.append("c", 2, 3).is_err());
        kv.release("a").unwrap();
        kv.append("c", 2, 3).unwrap();
    }
    #[test]
    fn native_inference_is_deterministic_and_bounded() {
        let mut model = NativeLinearModel::fixture().unwrap();
        let receipt = model.infer("inference-1", "one two").unwrap();
        assert_eq!(receipt.output, 8);
        assert_eq!(receipt.active_parameter_bytes, 24);
        assert_eq!(receipt.active_kv_bytes, 16);
        assert_eq!(
            receipt.deterministic_hash,
            stable_hash(&format!("{}:{}:{}", "fixture-model-hash", 8, 1)),
        );
    }
    #[test]
    fn native_training_paths_are_equivalent_and_prediction_is_non_authoritative() {
        let samples = vec![
            NativeTrainingSample {
                input: vec![1, 2],
                target: 5,
            },
            NativeTrainingSample {
                input: vec![2, 1],
                target: 4,
            },
        ];
        let dense = run_native_training(NativeTrainingPath::Dense, &samples, 0).unwrap();
        let reactive = run_native_training(NativeTrainingPath::Reactive, &samples, 0).unwrap();
        let predictive = run_native_training(NativeTrainingPath::Predictive, &samples, 0).unwrap();
        assert_eq!(dense.final_parameters, reactive.final_parameters);
        assert_eq!(reactive.final_parameters, predictive.final_parameters);
        assert!(predictive.prediction_misses > 0);
        assert!(predictive.correction_events > 0);
        assert_eq!(dense.deterministic_hash, reactive.deterministic_hash);
        assert_eq!(reactive.deterministic_hash, predictive.deterministic_hash);
    }
}
