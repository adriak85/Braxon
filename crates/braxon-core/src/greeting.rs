use nsq_core::{Charge, Dialect, NSQLever, NSQSlot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferenceSeed {
    pub resonance_id: String,
    pub detected_intent: Vec<String>,
    pub environment_complexity: f32,
}

pub struct GreetingProtocol {
    pub is_first_contact: bool,
    pub current_void_state: f32,
}

impl GreetingProtocol {
    pub fn new() -> Self {
        Self {
            is_first_contact: true,
            current_void_state: 0.0,
        }
    }

    pub fn generate_initial_greeting(&self) -> String {
        "I am here. The void is listening. What shall we build together?".to_string()
    }

    pub fn resolve_ui_construction(&mut self, _user_input: &str) -> NSQSlot {
        self.current_void_state = (self.current_void_state + 0.1).min(1.0);
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
