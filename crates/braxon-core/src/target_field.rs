use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const TARGET_FIELD_PATH: &str = "state/braxon/target_field.json";
pub const TARGET_FIELD_SCHEMA: &str = "braxon.target_field.v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetField {
    pub schema: String,
    pub authority: String,
    pub canonical_semantics: String,
    pub coordinate_space: String,
    pub coordinates: [f64; 8],
    pub target_size_class: String,
    pub required_model_count: u64,
    pub brain_model_count: u64,
    pub sensory_body_count: u64,
    pub source_manifest: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetFieldActuation {
    pub resource_pressure: f64,
    pub information_pressure: f64,
    pub load_shed_fraction: f64,
    pub cache_flush_requested: bool,
    pub state_reconstruction_requested: bool,
    pub coordinate: [f64; 8],
}

impl Default for TargetField {
    fn default() -> Self {
        Self {
            schema: TARGET_FIELD_SCHEMA.to_string(),
            authority: "NSQ_COURT".to_string(),
            canonical_semantics: "base8_switch_topology".to_string(),
            coordinate_space: "eight_dimensional_intent_gradient".to_string(),
            coordinates: [0.0; 8],
            target_size_class: "mb_scale".to_string(),
            required_model_count: 10,
            brain_model_count: 6,
            sensory_body_count: 4,
            source_manifest: "config/nsq/braxon_council_ten_stack.json".to_string(),
            status: "initialized_from_validated_defaults".to_string(),
        }
    }
}

impl TargetField {
    pub fn path(root: &Path) -> PathBuf {
        root.join(TARGET_FIELD_PATH)
    }

    pub fn from_repository(root: &Path) -> Self {
        let mut field = Self::default();
        if let Ok(raw) = fs::read_to_string(root.join(&field.source_manifest)) {
            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&raw) {
                field.required_model_count = config
                    .get("required_model_count")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(field.required_model_count);
                field.brain_model_count = config
                    .get("brain_model_count")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(field.brain_model_count);
                field.sensory_body_count = config
                    .get("sensory_body_count")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(field.sensory_body_count);
                field.target_size_class = config
                    .get("target_size_class")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or(&field.target_size_class)
                    .to_string();
            }
        }
        field.status = "derived_from_validated_council_ten_manifest".to_string();
        field
    }

    pub fn load_or_initialize(root: &Path) -> Result<Self, String> {
        let path = Self::path(root);
        if let Ok(raw) = fs::read_to_string(&path) {
            let field = serde_json::from_str::<Self>(&raw).map_err(|err| err.to_string())?;
            field.validate()?;
            return Ok(field);
        }
        let field = Self::from_repository(root);
        field.persist(root)?;
        Ok(field)
    }

    pub fn persist(&self, root: &Path) -> Result<(), String> {
        self.validate()?;
        let path = Self::path(root);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        }
        let raw = serde_json::to_string_pretty(self).map_err(|err| err.to_string())?;
        fs::write(path, format!("{raw}\n")).map_err(|err| err.to_string())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != TARGET_FIELD_SCHEMA {
            return Err("target field schema mismatch".to_string());
        }
        if self.authority != "NSQ_COURT" {
            return Err("target field authority mismatch".to_string());
        }
        if self.coordinates.iter().any(|value| !value.is_finite()) {
            return Err("target field contains a non-finite coordinate".to_string());
        }
        if self.required_model_count != self.brain_model_count + self.sensory_body_count {
            return Err("target field model counts do not reconcile".to_string());
        }
        Ok(())
    }

    pub fn actuation(&self, gradient: [f64; 8]) -> Result<TargetFieldActuation, String> {
        self.validate()?;
        let distance = gradient
            .iter()
            .zip(self.coordinates.iter())
            .map(|(value, target)| (value - target).abs())
            .sum::<f64>()
            / self.coordinates.len() as f64;
        let resource_pressure = gradient[0].clamp(0.0, 1.0);
        let information_pressure = gradient[1].clamp(0.0, 1.0);
        let load_shed_fraction =
            (resource_pressure.max(information_pressure) * distance).clamp(0.0, 1.0);
        Ok(TargetFieldActuation {
            resource_pressure,
            information_pressure,
            load_shed_fraction,
            cache_flush_requested: information_pressure >= 0.75,
            state_reconstruction_requested: distance >= 0.5,
            coordinate: gradient,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn target_field_is_reconciled_and_deterministic() {
        let field = TargetField::default();
        field.validate().expect("default target field validates");
        let actuation = field.actuation([1.0; 8]).unwrap();
        assert!(actuation.cache_flush_requested);
        assert!(actuation.state_reconstruction_requested);
    }

    #[test]
    fn target_field_persists_and_reloads() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("braxon-target-field-{suffix}"));
        let field = TargetField::default();
        field.persist(&root).unwrap();
        assert_eq!(TargetField::load_or_initialize(&root).unwrap(), field);
        let _ = fs::remove_dir_all(root);
    }
}
