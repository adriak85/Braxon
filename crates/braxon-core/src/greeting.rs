use crate::OutputClassification;
use nsq_core::{Charge, Dialect, NSQLever, NSQSlot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferenceSeed {
    pub resonance_id: String,
    pub detected_intent: Vec<String>,
    pub environment_complexity: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GreetingPresentation {
    pub classification: OutputClassification,
    pub text: String,
}

pub struct GreetingProtocol {
    pub is_first_contact: bool,
    pub interaction_progress: f32,
}

impl GreetingProtocol {
    pub fn new() -> Self {
        Self {
            is_first_contact: true,
            interaction_progress: 0.0,
        }
    }

    /// A user-interface presentation; it is never committed as hard runtime state.
    pub fn initial_presentation(&self) -> GreetingPresentation {
        GreetingPresentation {
            classification: OutputClassification::UserPresentation,
            text: "Braxon operator surface ready. Provide an action or verification request."
                .into(),
        }
    }

    pub fn resolve_ui_construction(&mut self, _user_input: &str) -> NSQSlot {
        self.interaction_progress = (self.interaction_progress + 0.1).min(1.0);
        NSQSlot::new(
            Dialect::Intent,
            vec![
                NSQLever::new(Charge::Positive, 1100).unwrap(),
                NSQLever::new(Charge::Positive, 1001).unwrap(),
            ],
        )
    }
}

impl Default for GreetingProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_an_explicit_user_presentation_not_runtime_narrative() {
        let greeting = GreetingProtocol::new().initial_presentation();
        assert_eq!(
            greeting.classification,
            OutputClassification::UserPresentation
        );
        assert!(greeting.text.contains("operator surface ready"));
        assert!(!greeting.text.contains("void"));
    }
}
