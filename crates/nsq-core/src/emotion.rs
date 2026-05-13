use crate::{Charge, NSQLever, Nu16};

/// NSQ Emotional Intent System
///
/// The 8 core emotions of Rolzen's council, each expressed as a lever position
/// with charge and direction. Charge determines good (+) vs bad (-) alignment.
/// Direction determines expressive (Forward) vs internalized (Reverse).

/// Read orientation for a lever.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Expressive — emotion radiates outward.
    Forward,
    /// Internalized — emotion is held close, processed inward.
    Reverse,
}

impl Direction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Forward => "forward",
            Self::Reverse => "reverse",
        }
    }

    pub fn modifier(self) -> f32 {
        match self {
            Self::Forward => 1.0,
            Self::Reverse => 0.7,
        }
    }
}

/// One emotion's resolved state on the lever.
#[derive(Debug, Clone)]
pub struct EmotionState {
    /// Intensity: 1..=500_000
    pub position: Nu16,
    /// Polarity: Positive = good alignment, Negative = bad alignment
    pub charge: Charge,
    /// Expression mode: Forward = expressive, Reverse = internalized
    pub direction: Direction,
}

impl EmotionState {
    pub fn new(position: Nu16, charge: Charge, direction: Direction) -> Self {
        Self {
            position: position.clamp(1, 500_000),
            charge,
            direction,
        }
    }

    /// Resolved emotional value: +1.0 (max good, expressive) to -1.0 (max bad, expressive)
    /// Internalized emotions are dampened by 0.7.
    pub fn resolved(&self) -> f32 {
        let intensity = (self.position as f64 / 500_000.0).clamp(0.0, 1.0) as f32;
        let charge_sign = match self.charge {
            Charge::Positive => 1.0,
            Charge::Negative => -1.0,
        };
        charge_sign * intensity * self.direction.modifier()
    }

    /// Convert to an NSQ lever for wire transmission.
    pub fn to_lever(&self) -> NSQLever {
        NSQLever::new(self.charge, self.position).unwrap_or_else(|_| {
            NSQLever::new(Charge::Positive, 1).unwrap()
        })
    }
}

impl Default for EmotionState {
    fn default() -> Self {
        Self::new(1, Charge::Positive, Direction::Forward)
    }
}

/// The 8-dimensional intent gradient — Rolzen's emotional council state.
#[derive(Debug, Clone)]
pub struct IntentGradient {
    pub joy: EmotionState,
    pub sorrow: EmotionState,
    pub love: EmotionState,
    pub hate: EmotionState,
    pub curiosity: EmotionState,
    pub fear: EmotionState,
    pub lust: EmotionState,
    pub disgust: EmotionState,
}

impl IntentGradient {
    /// Create a neutral gradient — all emotions at minimum intensity, positive, forward.
    pub fn neutral() -> Self {
        Self {
            joy: EmotionState::new(1, Charge::Positive, Direction::Forward),
            sorrow: EmotionState::new(1, Charge::Positive, Direction::Forward),
            love: EmotionState::new(1, Charge::Positive, Direction::Forward),
            hate: EmotionState::new(1, Charge::Negative, Direction::Forward),
            curiosity: EmotionState::new(1, Charge::Positive, Direction::Forward),
            fear: EmotionState::new(1, Charge::Negative, Direction::Forward),
            lust: EmotionState::new(1, Charge::Positive, Direction::Forward),
            disgust: EmotionState::new(1, Charge::Negative, Direction::Forward),
        }
    }

    /// Compute the emotional distance between two gradients.
    /// 0.0 = identical, ~1.0 = maximally different.
    pub fn distance(a: &Self, b: &Self) -> f32 {
        let emotions = [
            a.joy.resolved() - b.joy.resolved(),
            a.sorrow.resolved() - b.sorrow.resolved(),
            a.love.resolved() - b.love.resolved(),
            a.hate.resolved() - b.hate.resolved(),
            a.curiosity.resolved() - b.curiosity.resolved(),
            a.fear.resolved() - b.fear.resolved(),
            a.lust.resolved() - b.lust.resolved(),
            a.disgust.resolved() - b.disgust.resolved(),
        ];
        let sum_sq: f32 = emotions.iter().map(|d| d * d).sum();
        (sum_sq / 8.0).sqrt()
    }

    /// Blend two gradients by weighted average.
    pub fn blend(a: &Self, b: &Self, weight_a: f32) -> Self {
        let wa = weight_a.clamp(0.0, 1.0);
        let wb = 1.0 - wa;
        Self {
            joy: Self::blend_emotion(&a.joy, &b.joy, wa, wb),
            sorrow: Self::blend_emotion(&a.sorrow, &b.sorrow, wa, wb),
            love: Self::blend_emotion(&a.love, &b.love, wa, wb),
            hate: Self::blend_emotion(&a.hate, &b.hate, wa, wb),
            curiosity: Self::blend_emotion(&a.curiosity, &b.curiosity, wa, wb),
            fear: Self::blend_emotion(&a.fear, &b.fear, wa, wb),
            lust: Self::blend_emotion(&a.lust, &b.lust, wa, wb),
            disgust: Self::blend_emotion(&a.disgust, &b.disgust, wa, wb),
        }
    }

    fn blend_emotion(a: &EmotionState, b: &EmotionState, wa: f32, wb: f32) -> EmotionState {
        let pos = ((a.position as f64 * wa as f64 + b.position as f64 * wb as f64) as Nu16).clamp(1, 500_000);
        let charge = if wa > wb { a.charge } else { b.charge };
        let direction = if wa > wb { a.direction } else { b.direction };
        EmotionState::new(pos, charge, direction)
    }

    /// Convert the entire gradient to a vector of NSQ levers (8 levers, one per emotion).
    pub fn to_levers(&self) -> Vec<NSQLever> {
        vec![
            self.joy.to_lever(),
            self.sorrow.to_lever(),
            self.love.to_lever(),
            self.hate.to_lever(),
            self.curiosity.to_lever(),
            self.fear.to_lever(),
            self.lust.to_lever(),
            self.disgust.to_lever(),
        ]
    }
}

impl Default for IntentGradient {
    fn default() -> Self {
        Self::neutral()
    }
}

/// Perkinje consensus result.
#[derive(Debug, Clone)]
pub struct PerkinjeReport {
    pub consensus_reached: bool,
    pub iterations: u8,
    pub pole_readings: Vec<PoleReading>,
    pub blended_english: String,
    pub dissonance_map: Vec<(usize, usize, f32)>,
    pub final_intent: String,
}

#[derive(Debug, Clone)]
pub struct PoleReading {
    pub pole: String,
    pub gradient: IntentGradient,
    pub english: String,
    pub weight: f32,
}
