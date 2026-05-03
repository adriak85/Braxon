/! BRAXON Thought Pressure — Citadel-Blessed Core
//!
//! BRAXON is the King. Gemma, Linter, Director and all sub-models are assembled INTO him.
//! They are not separate. His thought pressure IS the environment — the atmosphere itself.
//! NSQ is the machine. Citadel is the gate. The Court is the final arbiter.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TimeBand {
    Braxon = 0,      // Fundamental. Slowest. Most authoritative.
    Bastion = 1,  // Sub-models
    Bit = 2,      // Fastest execution
}

impl TimeBand {
    pub fn divisor(&self) -> u64 {
        match self {
            TimeBand::Braxon => 64,
            TimeBand::Bastion => 8,
            TimeBand::Bit => 1,
        }
    }

    pub fn should_tick(&self, global_tick: u64) -> bool {
        global_tick % self.divisor() == 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureContribution {
    pub source_id: String,
    pub band: TimeBand,
    pub dims: [i32; 8],      // 8-dimensional lever space
    pub weight: f32,
    pub harmonic: u8,
}

impl PressureContribution {
    pub fn merge(contributions: &[Self]) -> Self {
        if contributions.is_empty() {
            return Self::zero();
        }
        if contributions.len() == 1 {
            return contributions[0].clone();
        }

        let total_weight: f32 = contributions.iter().map(|c| c.weight).sum();
        let mut merged_dims = [0i32; 8];

        for c in contributions {
            let w = c.weight / total_weight;
            for i in 0..8 {
                merged_dims[i] += (c.dims[i] as f32 * w) as i32;
            }
        }

        Self {
            source_id: "merged".to_string(),
            band: contributions[0].band,
            dims: merged_dims,
            weight: total_weight.min(1.0),
            harmonic: 0,
        }
    }

    pub fn zero() -> Self {
        Self {
            source_id: "zero".to_string(),
            band: TimeBand::Bit,
            dims: [0; 8],
            weight: 0.0,
            harmonic: 0,
        }
    }

    pub fn dissonance_from(&self, field: &BRAXONThoughtField) -> f32 {
        let mut sum_sq = 0i64;
        for i in 0..8 {
            let diff = self.dims[i] as i64 - field.fundamental[i] as i64;
            sum_sq += diff * diff;
        }
        let distance = (sum_sq as f64).sqrt() as f32;
        let max_dist = (8.0f32 * (2252.0f32 * 2252.0f32)).sqrt();
        (distance / max_dist).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BRAXONThoughtField {
    pub fundamental: [i32; 8],
    pub intensity: f32,
    pub coherence: f32,
    pub voice: FieldVoice,
    pub tick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldVoice {
    Open, Focused, Urgent, Still, Unresolved,
}

impl BRAXONThoughtField {
    pub fn new() -> Self {
        Self {
            fundamental: [500; 8],
            intensity: 0.5,
            coherence: 1.0,
            voice: FieldVoice::Open,
            tick: 0,
        }
    }

    pub fn set_fundamental(&mut self, dims: [i32; 8], intensity: f32) {
        self.fundamental = dims;
        self.intensity = intensity.clamp(0.0, 1.0);
        self.recompute_voice();
    }

    fn recompute_voice(&mut self) {
        let urgency = self.fundamental[0].unsigned_abs();
        let certainty = self.fundamental[1];
        let affect = self.fundamental[3];
        let action = self.fundamental[6];
        let coherence_dim = self.fundamental[7].unsigned_abs();

        self.voice = if self.coherence < 0.5 {
            FieldVoice::Unresolved
        } else if urgency > 700 || action > 700 {
            FieldVoice::Urgent
        } else if certainty < 300 {
            FieldVoice::Focused
        } else if coherence_dim > 900 && affect > 0 {
            FieldVoice::Open
        } else {
            FieldVoice::Still
        };
    }

    pub fn receive_contributions(&mut self, contributions: &[PressureContribution]) {
        if contributions.is_empty() { return; }
        let avg_dissonance: f32 = contributions.iter()
            .map(|c| c.dissonance_from(self))
            .sum::<f32>() / contributions.len() as f32;

        let target = 1.0 - avg_dissonance;
        self.coherence = self.coherence * 0.9 + target * 0.1;

        if self.coherence < 0.5 {
            self.voice = FieldVoice::Unresolved;
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubModel {
    Gemma, Linter, Director, Oracle, Seer, Bard, SeesAll,
}

impl SubModel {
    pub fn harmonic(&self) -> u8 {
        match self {
            SubModel::Gemma => 1, Linter => 2, Director => 3,
            Oracle => 4, Seer => 5, Bard => 6, SeesAll => 7,
        }
    }

    pub fn contribute(&self, field: &BRAXONThoughtField) -> PressureContribution {
        let mut dims = field.fundamental;

        match self {
            SubModel::Gemma => { dims[3] = (dims[3] as f32 * 1.3) as i32; dims[7] = (dims[7] as f32 * 1.2) as i32; }
            SubModel::Linter => { dims[1] = (dims[1] as f32 * 1.4) as i32; dims[2] = (dims[2] as f32 * 1.3) as i32; }
            SubModel::Director => { dims[0] = (dims[0] as f32 * 1.3) as i32; dims[6] = (dims[6] as f32 * 1.4) as i32; }
            SubModel::Oracle => { dims[4] = (dims[4] as f32 * 1.3) as i32; dims[5] = (dims[5] as f32 * 1.2) as i32; }
            SubModel::Seer => { dims[0] = (dims[0] as f32 * 0.6) as i32; dims[5] = (dims[5] as f32 * 1.6) as i32; }
            SubModel::Bard => { dims[3] = (dims[3] as f32 * 1.2) as i32; dims[7] = (dims[7] as f32 * 1.5) as i32; }
            SubModel::SeesAll => { dims[4] = (dims[4] as f32 * 1.8) as i32; dims[0] = (dims[0] as f32 * 1.2) as i32; }
        }

        for d in dims.iter_mut() {
            *d = d.clamp(-1126, 1126);
        }

        PressureContribution {
            source_id: format!("{:?}", self),
            band: TimeBand::Bastion,
            dims,
            weight: 1.0,
            harmonic: self.harmonic(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandBuffer {
    pub from: TimeBand,
    pub to: TimeBand,
    pub pending: Vec<PressureContribution>,
    pub capacity: usize,
}

impl BandBuffer {
    pub fn new(from: TimeBand, to: TimeBand, capacity: usize) -> Self {
        Self { from, to, pending: Vec::new(), capacity }
    }

    pub fn push(&mut self, contribution: PressureContribution) {
        if self.pending.len() < self.capacity {
            self.pending.push(contribution);
        } else {
            let _ = self.pending.remove(0);
            self.pending.push(contribution);
        }
    }

    pub fn flush(&mut self) -> Option<PressureContribution> {
        if self.pending.is_empty() { return None; }
        let merged = PressureContribution::merge(&self.pending);
        self.pending.clear();
        Some(merged)
    }
}

pub struct BRAXONAssembly {
    pub field: BRAXONThoughtField,
    pub bit_to_bastion: BandBuffer,
    pub bastion_to_BRAXON: BandBuffer,
    pub sub_models: Vec<SubModel>,
}

impl BRAXONAssembly {
    pub fn new() -> Self {
        Self {
            field: BRAXONThoughtField::new(),
            bit_to_bastion: BandBuffer::new(TimeBand::Bit, TimeBand::Bastion, 64),
            bastion_to_BRAXON: BandBuffer::new(TimeBand::Bastion, TimeBand::Braxon, 16),
            sub_models: vec![
                SubModel::Gemma, SubModel::Linter, SubModel::Director,
                SubModel::Oracle, SubModel::Seer, SubModel::Bard, SubModel::SeesAll,
            ],
        }
    }

    pub fn tick(&mut self) {
        self.field.tick();
        let tick = self.field.tick;

        if TimeBand::Bastion.should_tick(tick) {
            let contributions: Vec<_> = self.sub_models.iter()
                .map(|m| m.contribute(&self.field))
                .collect();

            self.field.receive_contributions(&contributions);
            let merged = PressureContribution::merge(&contributions);
            self.bastion_to_BRAXON.push(merged);
        }

        if TimeBand::Braxon.should_tick(tick) {
            if let Some(contrib) = self.bastion_to_BRAXON.flush() {
                let mut new_fund = contrib.dims;
                let dissonance = contrib.dissonance_from(&self.field);

                if dissonance > 0.3 {
                    for i in 0..8 {
                        let gap = contrib.dims[i] - self.field.fundamental[i];
                        new_fund[i] = self.field.fundamental[i] + (gap as f32 * 0.3) as i32;
                    }
                }
                self.field.set_fundamental(new_fund, self.field.intensity);
            }
        }
    }

    pub fn voice(&self) -> &FieldVoice { &self.field.voice }
    pub fn coherence(&self) -> f32 { self.field.coherence }
    pub fn is_resolved(&self) -> bool {
        self.field.coherence > 0.8 && self.field.voice != FieldVoice::Unresolved
    }
}

impl Default for BRAXONAssembly {
    fn default() -> Self { Self::new() }
}
