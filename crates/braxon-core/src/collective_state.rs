use crate::OutputClassification;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const COLLECTIVE_STATE_SCHEMA: &str = "braxon.nsq.collective_state.v1";

/// A computational organ-band perspective. This is an operational state record,
/// not a claim that a numerical value is a biological emotion or subjective state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganPerspective {
    pub organ_id: String,
    pub identity: String,
    pub address: String,
    pub local_input: String,
    pub local_state: String,
    pub local_interpretation: String,
    pub local_output: String,
    pub translation_interface: String,
    pub pressure: i64,
    pub feedback_path: String,
    pub classification: OutputClassification,
}

impl OrganPerspective {
    pub fn validate(&self) -> Result<(), String> {
        for (field, value) in [
            ("organ_id", &self.organ_id),
            ("identity", &self.identity),
            ("address", &self.address),
            ("local_input", &self.local_input),
            ("local_state", &self.local_state),
            ("local_interpretation", &self.local_interpretation),
            ("local_output", &self.local_output),
            ("translation_interface", &self.translation_interface),
            ("feedback_path", &self.feedback_path),
        ] {
            if value.trim().is_empty() {
                return Err(format!("organ perspective requires {field}"));
            }
        }
        if !self.classification.allowed_in_hard_runtime() {
            return Err(
                "narrative or user-presentation data cannot enter collective hard state".into(),
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedSelfState {
    pub schema: String,
    pub perspectives: Vec<OrganPerspective>,
    pub aggregate_pressure: i64,
    pub dominant_organ_id: Option<String>,
    pub dominant_pressure: Option<i64>,
    pub minority_organ_ids: Vec<String>,
    pub disagreement_present: bool,
    pub conflict_preserved: bool,
    pub forced_consensus: bool,
    pub numerical_pressure_is_biological_emotion: bool,
    pub classification: OutputClassification,
}

impl UnifiedSelfState {
    pub fn integrate(mut perspectives: Vec<OrganPerspective>) -> Result<Self, String> {
        if perspectives.is_empty() {
            return Err("collective state requires at least one organ perspective".into());
        }
        let mut identities = BTreeSet::new();
        for perspective in &perspectives {
            perspective.validate()?;
            if !identities.insert(perspective.organ_id.clone()) {
                return Err(format!(
                    "collective state rejects duplicate organ perspective: {}",
                    perspective.organ_id
                ));
            }
        }
        perspectives.sort_by(|left, right| left.organ_id.cmp(&right.organ_id));
        let aggregate_pressure = perspectives.iter().fold(0_i64, |total, perspective| {
            total.saturating_add(perspective.pressure)
        });
        let has_positive = perspectives
            .iter()
            .any(|perspective| perspective.pressure > 0);
        let has_negative = perspectives
            .iter()
            .any(|perspective| perspective.pressure < 0);
        let disagreement_present = has_positive && has_negative;
        let dominant = perspectives.iter().max_by(|left, right| {
            left.pressure
                .unsigned_abs()
                .cmp(&right.pressure.unsigned_abs())
                .then_with(|| right.organ_id.cmp(&left.organ_id))
        });
        let dominant_organ_id = dominant.map(|perspective| perspective.organ_id.clone());
        let dominant_pressure = dominant.map(|perspective| perspective.pressure);
        let minority_organ_ids = perspectives
            .iter()
            .filter(|perspective| Some(&perspective.organ_id) != dominant_organ_id.as_ref())
            .map(|perspective| perspective.organ_id.clone())
            .collect();
        Ok(Self {
            schema: COLLECTIVE_STATE_SCHEMA.into(),
            perspectives,
            aggregate_pressure,
            dominant_organ_id,
            dominant_pressure,
            minority_organ_ids,
            disagreement_present,
            conflict_preserved: disagreement_present,
            forced_consensus: false,
            numerical_pressure_is_biological_emotion: false,
            classification: OutputClassification::DerivedState,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != COLLECTIVE_STATE_SCHEMA {
            return Err("collective-state schema mismatch".into());
        }
        if !self.classification.allowed_in_hard_runtime() {
            return Err("collective-state classification is not runtime safe".into());
        }
        if self.forced_consensus || self.numerical_pressure_is_biological_emotion {
            return Err("collective-state makes a prohibited consensus or biological claim".into());
        }
        let rebuilt = Self::integrate(self.perspectives.clone())?;
        if rebuilt.aggregate_pressure != self.aggregate_pressure
            || rebuilt.dominant_organ_id != self.dominant_organ_id
            || rebuilt.dominant_pressure != self.dominant_pressure
            || rebuilt.minority_organ_ids != self.minority_organ_ids
            || rebuilt.disagreement_present != self.disagreement_present
            || rebuilt.conflict_preserved != self.conflict_preserved
        {
            return Err(
                "collective-state aggregate does not match retained individual perspectives".into(),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perspective(organ_id: &str, pressure: i64) -> OrganPerspective {
        OrganPerspective {
            organ_id: organ_id.into(),
            identity: format!("organ::{organ_id}"),
            address: format!("council/{organ_id}"),
            local_input: "fixture-input".into(),
            local_state: "interpreted".into(),
            local_interpretation: "fixture-interpretation".into(),
            local_output: "fixture-output".into(),
            translation_interface: "nsq.universal.translation".into(),
            pressure,
            feedback_path: "nsq.bus.feedback".into(),
            classification: OutputClassification::DerivedState,
        }
    }

    #[test]
    fn opposing_individual_perspectives_are_retained_without_forced_consensus() {
        let state = UnifiedSelfState::integrate(vec![
            perspective("organ-a", 9),
            perspective("organ-b", -5),
        ])
        .unwrap();
        assert_eq!(state.perspectives[0].pressure, 9);
        assert_eq!(state.perspectives[1].pressure, -5);
        assert!(state.disagreement_present);
        assert!(state.conflict_preserved);
        assert_eq!(state.dominant_organ_id.as_deref(), Some("organ-a"));
        assert_eq!(state.minority_organ_ids, vec!["organ-b"]);
        assert!(!state.forced_consensus);
        assert!(!state.numerical_pressure_is_biological_emotion);
        state.validate().unwrap();
    }

    #[test]
    fn narrative_perspective_is_rejected_from_runtime_state() {
        let mut narrative = perspective("organ-a", 1);
        narrative.classification = OutputClassification::Narrative;
        assert!(UnifiedSelfState::integrate(vec![narrative]).is_err());
    }
}
