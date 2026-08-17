//! Seed-first reconstruction: intent is canonical, residency is materialization.
use nsq_core::{Charge, Dialect, NSQLever, NSQSlot, CANONICAL_LEVER_MAX_POSITION};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentSeed {
    pub identity: String,
    pub intent: String,
    pub coordinates: Vec<u64>,
    pub sections: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterializedState {
    pub logical_complete: bool,
    pub resident_begin: usize,
    pub resident_end: usize,
    pub slots: Vec<NSQSlot>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalToken {
    pub canonical: String,
    pub sections: Vec<String>,
    pub universal: String,
}

impl IntentSeed {
    pub fn new(identity: &str, intent: &str) -> Self {
        let coordinates = coordinate_intent(intent);
        Self {
            identity: identity.into(),
            intent: intent.into(),
            coordinates,
            sections: vec![
                "language".into(),
                "symbol".into(),
                "intent".into(),
                "world".into(),
            ],
        }
    }
    pub fn materialize(&self, start: usize, count: usize) -> MaterializedState {
        let slots = self
            .coordinates
            .iter()
            .map(|p| {
                NSQSlot::new(
                    Dialect::Intent,
                    vec![NSQLever::new(Charge::Positive, *p).unwrap()],
                )
            })
            .collect::<Vec<_>>();
        let end = (start + count).min(slots.len());
        MaterializedState {
            logical_complete: true,
            resident_begin: start.min(slots.len()),
            resident_end: end,
            slots: slots
                .into_iter()
                .skip(start.min(end))
                .take(end.saturating_sub(start))
                .collect(),
        }
    }
}
pub fn coordinate_intent(text: &str) -> Vec<u64> {
    text.split_whitespace()
        .map(|w| {
            let h = w
                .bytes()
                .fold(2166136261u64, |a, b| a.wrapping_mul(16777619) ^ b as u64);
            1 + (h % CANONICAL_LEVER_MAX_POSITION)
        })
        .collect()
}
pub fn synchronize(canonical: &str, sections: &[&str]) -> UniversalToken {
    UniversalToken {
        canonical: canonical.into(),
        sections: sections.iter().map(|s| (*s).into()).collect(),
        universal: format!("nsq.intent.v1::{canonical}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn seed_is_logically_complete_with_partial_residency() {
        let s = IntentSeed::new("test", "willow stone remembers");
        let m = s.materialize(0, 1);
        assert!(m.logical_complete);
        assert_eq!(m.slots.len(), 1);
        assert!(s.coordinates.len() > 1);
    }
    #[test]
    fn synchronization_has_one_universal_identity() {
        let t = synchronize("remember", &["language", "image", "world"]);
        assert_eq!(t.universal, "nsq.intent.v1::remember");
        assert_eq!(t.sections.len(), 3);
    }
    #[test]
    fn coordinates_are_deterministic() {
        assert_eq!(
            coordinate_intent("stone willow"),
            coordinate_intent("stone willow")
        );
    }
}
