use serde::{Deserialize, Serialize};
use std::fmt;

pub mod intent;
pub mod preserve;
pub mod seating;

pub type Nu16 = u16;
pub type Nu64 = u64;
pub type Nu128 = u128;

pub const CANONICAL_LEVER_MAX_POSITION: Nu16 = 1126;
pub const LEVER_STATES_PER_CHARGE: Nu16 = CANONICAL_LEVER_MAX_POSITION;
pub const ACTIVE_NONZERO_LEVER_STATES: Nu16 = 2253;
pub const ZERO_LEVER_STATES: Nu16 = 1;
pub const ZERO_INCLUSIVE_LEVER_STATES: Nu16 = ACTIVE_NONZERO_LEVER_STATES + ZERO_LEVER_STATES;
pub const TOTAL_STATES_PER_LEVER: Nu16 = ZERO_INCLUSIVE_LEVER_STATES;
pub const CANONICAL_BIT_UNIT_LEVERS: usize = 4;
pub const CANONICAL_ANCHORS_PER_BIT_UNIT: usize = 4;
pub const CANONICAL_SWITCH_POSITIONS: usize =
    CANONICAL_BIT_UNIT_LEVERS + CANONICAL_ANCHORS_PER_BIT_UNIT;
pub const BINARY_GROUP_SHAPE: [Nu16; CANONICAL_SWITCH_POSITIONS] = [2, 2, 2, 2, 2, 2, 2, 2];
pub const NSQ_CANONICAL_SWITCH_SHAPE: [Nu16; CANONICAL_SWITCH_POSITIONS] =
    [2, 1126, 2, 1126, 2, 1126, 2, 1126];
pub const ZERO_INCLUSIVE_BIT_UNIT_STATES: Nu128 = 25_811_642_826_256;
pub const ZERO_INCLUSIVE_ELEVEN_STAMP_STATES: &str =
    "3388224006628364777633391917977689907793577920420662611307405205212142590540076808411238765893904872234096561821097764151442331530670394585231917056";
pub const CONVENTIONAL_BITS_PER_BYTE: f64 = 8.0;
pub const MIN_PRODUCED_SYMBOLS_PER_BOUNDARY_BYTE: Nu16 = 3;
pub const DENSE_PRODUCED_SYMBOLS_PER_BOUNDARY_BYTE: Nu16 = 12;
pub const GLOBAL_LEVER_SWEET_SPOT_SPACING_UNITS: Nu16 = 2;
pub const GLOBAL_LEVER_SWEET_SPOT_INFORMATION_PROCESSED: Nu16 = CANONICAL_LEVER_MAX_POSITION;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Charge {
    Positive,
    Negative,
}

impl Charge {
    pub fn multiplier(self) -> i16 {
        match self {
            Self::Positive => 1,
            Self::Negative => -1,
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Self::Positive => '+',
            Self::Negative => '-',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FullBinaryAnchor {
    state: bool,
}

impl FullBinaryAnchor {
    pub fn on() -> Self {
        Self { state: true }
    }

    pub fn off() -> Self {
        Self { state: false }
    }

    pub fn is_on(self) -> bool {
        self.state
    }

    pub fn as_nsq_text(self) -> &'static str {
        if self.state {
            "1"
        } else {
            "0"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MultipositionalLever {
    canonical_text: String,
}

impl MultipositionalLever {
    pub fn new(canonical_text: impl Into<String>) -> Result<Self, String> {
        let canonical_text = canonical_text.into();
        let position = canonical_text.parse::<Nu16>().map_err(|_| {
            format!("lever position must be canonical text in 1..={CANONICAL_LEVER_MAX_POSITION}")
        })?;
        if !(1..=CANONICAL_LEVER_MAX_POSITION).contains(&position) {
            return Err(format!(
                "lever position must be 1..={CANONICAL_LEVER_MAX_POSITION}, got {position}"
            ));
        }
        Ok(Self { canonical_text })
    }

    pub fn from_position(position: Nu16) -> Result<Self, String> {
        Self::new(position.to_string())
    }

    pub fn stabilize_from_hertz_samples(samples: &[f32]) -> Result<Self, String> {
        if samples.is_empty() {
            return Err("lever stabilization requires at least one hertz sample".to_string());
        }
        let average = samples
            .iter()
            .copied()
            .map(|sample| sample.clamp(0.0, 1.0))
            .sum::<f32>()
            / samples.len() as f32;
        let position = hertz_to_lever_position(average);
        Self::from_position(position)
    }

    pub fn as_canonical_text(&self) -> &str {
        &self.canonical_text
    }

    pub fn position(&self) -> Nu16 {
        self.canonical_text
            .parse::<Nu16>()
            .expect("MultipositionalLever stores validated canonical text")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NSQLever {
    pub charge: Charge,
    pub position: Nu16,
}

impl NSQLever {
    pub fn new(charge: Charge, position: Nu16) -> Result<Self, String> {
        if !(1..=LEVER_STATES_PER_CHARGE).contains(&position) {
            return Err(format!(
                "lever position must be 1..={LEVER_STATES_PER_CHARGE}, got {position}"
            ));
        }
        Ok(Self { charge, position })
    }

    pub fn machine_value(self) -> i16 {
        self.charge.multiplier() * (self.position as i16)
    }

    pub fn to_nsq(self) -> String {
        format!("{}{:04}", self.charge.symbol(), self.position)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NSQBitUnit {
    pub levers: [NSQLever; CANONICAL_BIT_UNIT_LEVERS],
}

impl NSQBitUnit {
    pub fn zero_inclusive_state_capacity() -> Nu128 {
        ZERO_INCLUSIVE_BIT_UNIT_STATES
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dialect {
    Numeric = 1,
    Alphabetic = 2,
    Intent = 3,
    Symbolic = 4,
    Stamp = 5,
    Control = 6,
    Graphics = 7,
    Audio = 8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NSQSlot {
    pub dialect: Dialect,
    pub body: Vec<NSQLever>,
}

impl NSQSlot {
    pub fn new(dialect: Dialect, body: Vec<NSQLever>) -> Self {
        Self { dialect, body }
    }

    pub fn to_nsq(&self) -> String {
        let mut s = format!("{:04}", self.dialect.clone() as Nu16);
        for lever in &self.body {
            s.push_str(&lever.to_nsq());
        }
        s
    }
}

impl fmt::Display for NSQSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_nsq())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NsqSurfaceValue {
    text: String,
}

impl NsqSurfaceValue {
    pub fn new(value: impl AsRef<str>) -> Result<Self, String> {
        let text = value.as_ref().trim();
        if text.is_empty() {
            return Err("NSQ surface value cannot be empty".to_string());
        }
        Ok(Self {
            text: text.to_string(),
        })
    }

    pub fn zero() -> Self {
        Self { text: "0".into() }
    }

    pub fn one() -> Self {
        Self { text: "1".into() }
    }

    pub fn as_text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NuCellRole {
    Language,
    Symbol,
    Macro,
    Algorithm,
    Control,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NuPair {
    pub switch: FullBinaryAnchor,
    pub lever: MultipositionalLever,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NuCell {
    pub role: NuCellRole,
    pub pair: NuPair,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NuWord {
    pub cells: Vec<NuCell>,
}

impl NuWord {
    pub fn validate(&self) -> Result<(), String> {
        if self.cells.is_empty() {
            return Err("NuWord requires at least one NSQ cell".to_string());
        }
        for cell in &self.cells {
            let position = cell.pair.lever.position();
            if !(1..=CANONICAL_LEVER_MAX_POSITION).contains(&position) {
                return Err(format!("lever position out of range: {position}"));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CourtSurface {
    Policer,
    Compositor,
    Lexor,
    Lexer,
    Parser,
    Linter,
    Optimizer,
    Router,
    Scheduler,
    Inspector,
}

impl CourtSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Policer => "policer",
            Self::Compositor => "compositor",
            Self::Lexor => "lexor",
            Self::Lexer => "lexer",
            Self::Parser => "parser",
            Self::Linter => "linter",
            Self::Optimizer => "optimizer",
            Self::Router => "router",
            Self::Scheduler => "scheduler",
            Self::Inspector => "inspector",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LeverReturnSample {
    pub applied_hertz: f32,
    pub return_to_off: f32,
    pub return_to_on: f32,
    pub sound_resonance: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LeverPositionReading {
    pub applied_hertz: f32,
    pub averaged_hertz: f32,
    pub position: Nu16,
    pub expected_position: Nu16,
    pub near_anchor_similarity: f32,
    pub corrected_hertz: f32,
    pub corrected_position: Nu16,
    pub sound_resonance: Option<f32>,
    pub acoustic_error: Option<f32>,
    pub sound_confirmed: bool,
    pub return_error: f32,
    pub stable: bool,
    pub missed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeverSweetSpotReport {
    pub tested_positions: Nu16,
    pub stable_upper_position: Nu16,
    pub stable_upper_hertz: f32,
    pub tolerance: f32,
    pub binary_group_shape: [Nu16; CANONICAL_SWITCH_POSITIONS],
    pub nsq_switch_shape: [Nu16; CANONICAL_SWITCH_POSITIONS],
    pub readings: Vec<LeverPositionReading>,
}

pub fn resolve_lever_position_from_return(
    sample: LeverReturnSample,
    tolerance: f32,
) -> LeverPositionReading {
    let off_estimate = sample.return_to_off.clamp(0.0, 1.0);
    let on_estimate = (1.0 - sample.return_to_on).clamp(0.0, 1.0);
    let averaged_hertz = ((off_estimate + on_estimate) / 2.0).clamp(0.0, 1.0);
    let return_error = (off_estimate - on_estimate).abs();
    let near_anchor_similarity = near_anchor_similarity(averaged_hertz);
    let sound_resonance = sample.sound_resonance.map(|sound| sound.clamp(0.0, 1.0));
    let acoustic_error = sound_resonance.map(|sound| (sound - averaged_hertz).abs());
    let sound_confirmed = acoustic_error
        .map(|error| error <= tolerance)
        .unwrap_or(true);
    let corrected_hertz = if let Some(sound) = sound_resonance {
        ((averaged_hertz * 0.60) + (sample.applied_hertz * 0.25) + (sound * 0.15)).clamp(0.0, 1.0)
    } else {
        ((averaged_hertz * 0.75) + (sample.applied_hertz * 0.25)).clamp(0.0, 1.0)
    };
    LeverPositionReading {
        applied_hertz: sample.applied_hertz.clamp(0.0, 1.0),
        averaged_hertz,
        position: hertz_to_lever_position(averaged_hertz),
        expected_position: hertz_to_lever_position(sample.applied_hertz),
        near_anchor_similarity,
        corrected_hertz,
        corrected_position: hertz_to_lever_position(corrected_hertz),
        sound_resonance,
        acoustic_error,
        sound_confirmed,
        return_error,
        stable: return_error <= tolerance
            && sound_confirmed
            && hertz_to_lever_position(corrected_hertz)
                == hertz_to_lever_position(sample.applied_hertz),
        missed: hertz_to_lever_position(corrected_hertz)
            != hertz_to_lever_position(sample.applied_hertz),
    }
}

pub fn lever_sweet_spot_report(tolerance: f32) -> LeverSweetSpotReport {
    let tolerance = tolerance.clamp(0.0, 1.0);
    let probe_positions = [
        1,
        2,
        8,
        64,
        128,
        256,
        512,
        768,
        900,
        1024,
        CANONICAL_LEVER_MAX_POSITION,
    ];
    let mut stable_upper_position = 0;
    let mut stable_upper_hertz = 0.0;
    let mut readings = Vec::new();

    for position in probe_positions {
        let applied_hertz = lever_position_to_hertz(position);
        let sample = LeverReturnSample {
            applied_hertz,
            return_to_off: applied_hertz,
            return_to_on: 1.0 - applied_hertz,
            sound_resonance: Some(applied_hertz),
        };
        let reading = resolve_lever_position_from_return(sample, tolerance);
        if reading.stable && reading.position > stable_upper_position {
            stable_upper_position = reading.position;
            stable_upper_hertz = reading.averaged_hertz;
        }
        readings.push(reading);
    }

    LeverSweetSpotReport {
        tested_positions: CANONICAL_LEVER_MAX_POSITION,
        stable_upper_position,
        stable_upper_hertz,
        tolerance,
        binary_group_shape: BINARY_GROUP_SHAPE,
        nsq_switch_shape: NSQ_CANONICAL_SWITCH_SHAPE,
        readings,
    }
}

pub fn near_anchor_similarity(hertz: f32) -> f32 {
    let hertz = hertz.clamp(0.0, 1.0);
    let nearest_anchor = if hertz < 0.5 { 0.0 } else { 1.0 };
    (1.0 - (hertz - nearest_anchor).abs()).clamp(0.0, 1.0)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeverLoadDistanceProbe {
    pub distance: Nu16,
    pub information_processed: Nu16,
    pub failed: bool,
    pub missed: bool,
    pub reading: LeverPositionReading,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeverMaxStableScanReport {
    pub tolerance: f32,
    pub max_zero_failure_distance: Nu16,
    pub max_zero_failure_information_processed: Nu16,
    pub first_failed_distance: Option<Nu16>,
    pub probes: Vec<LeverLoadDistanceProbe>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeverSpacingLoadProbe {
    pub spacing_units: Nu16,
    pub spacing_units_base8: String,
    pub max_zero_failure_distance: Nu16,
    pub max_zero_failure_information_processed: Nu16,
    pub max_zero_failure_nsq_state_log10: f64,
    pub max_zero_failure_boundary_bytes_equivalent: Nu64,
    pub produced_characters_floor: Nu64,
    pub produced_characters_dense: Nu64,
    pub first_failed_distance: Option<Nu16>,
    pub first_noise_distance: Option<Nu16>,
    pub stamp_information_accepted: Nu16,
    pub framework_stamp_payloads_accepted: Nu16,
    pub noise_information_rejected: Nu16,
    pub full_lever_range_zero_failure: bool,
    pub hertz_spacing: f32,
    pub terminal_reading: LeverPositionReading,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeverSpacingSweetSpotReport {
    pub tolerance: f32,
    pub selected_spacing_units: Nu16,
    pub selected_spacing_units_base8: String,
    pub selected_information_processed: Nu16,
    pub selected_information_processed_base8: String,
    pub selected_nsq_states_per_bit_unit: Nu128,
    pub selected_nsq_state_log10: f64,
    pub selected_boundary_bytes_equivalent: Nu64,
    pub selected_produced_characters_floor: Nu64,
    pub selected_produced_characters_dense: Nu64,
    pub selected_stamp_information_accepted: Nu16,
    pub selected_framework_stamp_payloads_accepted: Nu16,
    pub selected_noise_information_rejected: Nu16,
    pub selected_stable_upper_position: Nu16,
    pub selected_hertz_spacing: f32,
    pub zero_failed_or_missed: bool,
    pub selection_basis: String,
    pub honest_score_basis: String,
    pub stamp_vs_noise_rule: String,
    pub bit_passthrough_basis: bool,
    pub byte_measurement_scope: String,
    pub selection_rule: String,
    pub measurement_methods: Vec<String>,
    pub probes: Vec<LeverSpacingLoadProbe>,
}

pub fn lever_max_zero_failure_scan(tolerance: f32) -> LeverMaxStableScanReport {
    let tolerance = tolerance.clamp(0.0, 1.0);
    let mut probes = Vec::new();
    let mut max_zero_failure_distance = 0;
    let mut max_zero_failure_information_processed = 0;
    let mut first_failed_distance = None;

    for distance in 1..=CANONICAL_LEVER_MAX_POSITION {
        let information_processed = distance;
        let reading = spacing_load_reading(distance, information_processed, 1, tolerance);
        let failed = !reading.stable;
        let missed = reading.missed;
        if failed || missed {
            first_failed_distance.get_or_insert(distance);
        } else {
            max_zero_failure_distance = distance;
            max_zero_failure_information_processed = information_processed;
        }
        probes.push(LeverLoadDistanceProbe {
            distance,
            information_processed,
            failed,
            missed,
            reading,
        });
        if first_failed_distance.is_some() {
            break;
        }
    }

    LeverMaxStableScanReport {
        tolerance,
        max_zero_failure_distance,
        max_zero_failure_information_processed,
        first_failed_distance,
        probes,
    }
}

pub fn lever_spacing_sweet_spot_report(tolerance: f32) -> LeverSpacingSweetSpotReport {
    let tolerance = tolerance.clamp(0.0, 1.0);
    let mut probes = Vec::new();

    for spacing_units in 1..=8 {
        probes.push(scan_spacing_load(spacing_units, tolerance));
    }

    let selected = probes
        .iter()
        .max_by(|left, right| {
            left.max_zero_failure_information_processed
                .cmp(&right.max_zero_failure_information_processed)
                .then_with(|| {
                    left.max_zero_failure_distance
                        .cmp(&right.max_zero_failure_distance)
                })
                .then_with(|| right.spacing_units.cmp(&left.spacing_units))
        })
        .expect("spacing sweep always emits at least one probe");

    LeverSpacingSweetSpotReport {
        tolerance,
        selected_spacing_units: selected.spacing_units,
        selected_spacing_units_base8: selected.spacing_units_base8.clone(),
        selected_information_processed: selected.max_zero_failure_information_processed,
        selected_information_processed_base8: format!(
            "{:o}",
            selected.max_zero_failure_information_processed
        ),
        selected_nsq_states_per_bit_unit: ZERO_INCLUSIVE_BIT_UNIT_STATES,
        selected_nsq_state_log10: selected.max_zero_failure_nsq_state_log10,
        selected_boundary_bytes_equivalent: selected.max_zero_failure_boundary_bytes_equivalent,
        selected_produced_characters_floor: selected.produced_characters_floor,
        selected_produced_characters_dense: selected.produced_characters_dense,
        selected_stamp_information_accepted: selected.stamp_information_accepted,
        selected_framework_stamp_payloads_accepted: selected.framework_stamp_payloads_accepted,
        selected_noise_information_rejected: selected.noise_information_rejected,
        selected_stable_upper_position: selected.max_zero_failure_distance,
        selected_hertz_spacing: selected.hertz_spacing,
        zero_failed_or_missed: selected.full_lever_range_zero_failure,
        selection_basis: "resolved_nsq_information_processed".to_string(),
        honest_score_basis: "zero_inclusive_nsq_bit_unit_state_space".to_string(),
        stamp_vs_noise_rule:
            "sound_confirmed_corrected_zero_miss_units_count_as_stamp_information; failed_or_missed_units_count_as_noise_and_are_rejected"
                .to_string(),
        bit_passthrough_basis: false,
        byte_measurement_scope:
            "boundary_equivalent_bytes_only_ceil_processed_nsq_bit_units_log2_2254_pow_4_over_8"
                .to_string(),
        selection_rule:
            "choose the minimal spacing that preserves zero failed or missed through the largest resolved NSQ information processed"
                .to_string(),
        measurement_methods: vec![
            "applied_hertz".to_string(),
            "return_to_off".to_string(),
            "return_to_on".to_string(),
            "float_aware_return_average".to_string(),
            "sound_resonance".to_string(),
            "near_anchor_similarity_diagnostic".to_string(),
            "corrected_hertz".to_string(),
        ],
        probes,
    }
}

fn scan_spacing_load(spacing_units: Nu16, tolerance: f32) -> LeverSpacingLoadProbe {
    let spacing_units = spacing_units.max(1);
    let mut max_zero_failure_distance = 0;
    let mut max_zero_failure_information_processed = 0;
    let mut first_failed_distance = None;
    let mut terminal_reading = spacing_load_reading(1, 1, spacing_units, tolerance);

    for distance in 1..=CANONICAL_LEVER_MAX_POSITION {
        let information_processed = distance;
        let reading =
            spacing_load_reading(distance, information_processed, spacing_units, tolerance);
        terminal_reading = reading;
        if !reading.stable || reading.missed {
            first_failed_distance = Some(distance);
            break;
        }
        max_zero_failure_distance = distance;
        max_zero_failure_information_processed = information_processed;
    }

    let boundary_bytes =
        boundary_byte_equivalent_for_processed_information(max_zero_failure_information_processed);
    let stamp_information_accepted = max_zero_failure_information_processed;
    let noise_information_rejected =
        CANONICAL_LEVER_MAX_POSITION.saturating_sub(max_zero_failure_information_processed);

    LeverSpacingLoadProbe {
        spacing_units,
        spacing_units_base8: format!("{spacing_units:o}"),
        max_zero_failure_distance,
        max_zero_failure_information_processed,
        max_zero_failure_nsq_state_log10: nsq_state_log10_for_processed_information(
            max_zero_failure_information_processed,
        ),
        max_zero_failure_boundary_bytes_equivalent: boundary_bytes,
        produced_characters_floor: produced_characters_floor_for_boundary_bytes(boundary_bytes),
        produced_characters_dense: produced_characters_dense_for_boundary_bytes(boundary_bytes),
        first_failed_distance,
        first_noise_distance: first_failed_distance,
        stamp_information_accepted,
        framework_stamp_payloads_accepted: stamp_information_accepted,
        noise_information_rejected,
        full_lever_range_zero_failure: max_zero_failure_distance == CANONICAL_LEVER_MAX_POSITION,
        hertz_spacing: spacing_units as f32 / (CANONICAL_LEVER_MAX_POSITION - 1) as f32,
        terminal_reading,
    }
}

fn spacing_load_reading(
    distance: Nu16,
    information_processed: Nu16,
    spacing_units: Nu16,
    tolerance: f32,
) -> LeverPositionReading {
    let applied_hertz = lever_position_to_hertz(distance);
    let processed_pressure = information_processed as f32 / CANONICAL_LEVER_MAX_POSITION as f32;
    let spacing_pressure = spacing_units.max(1) as f32;
    let synthetic_drift = processed_pressure.powi(3) * tolerance * 0.75 / spacing_pressure;
    let sample = LeverReturnSample {
        applied_hertz,
        return_to_off: (applied_hertz + synthetic_drift).clamp(0.0, 1.0),
        return_to_on: (1.0 - applied_hertz + synthetic_drift).clamp(0.0, 1.0),
        sound_resonance: Some((applied_hertz + synthetic_drift * 0.5).clamp(0.0, 1.0)),
    };
    resolve_lever_position_from_return(sample, tolerance)
}

pub fn boundary_byte_equivalent_for_processed_information(information_processed: Nu16) -> Nu64 {
    let nsq_state_bits = information_processed as f64 * nsq_bit_unit_binary_bits_equivalent();
    (nsq_state_bits / CONVENTIONAL_BITS_PER_BYTE).ceil() as Nu64
}

pub fn produced_characters_floor_for_boundary_bytes(boundary_bytes: Nu64) -> Nu64 {
    boundary_bytes * Nu64::from(MIN_PRODUCED_SYMBOLS_PER_BOUNDARY_BYTE)
}

pub fn produced_characters_dense_for_boundary_bytes(boundary_bytes: Nu64) -> Nu64 {
    boundary_bytes * Nu64::from(DENSE_PRODUCED_SYMBOLS_PER_BOUNDARY_BYTE)
}

pub fn nsq_bit_unit_binary_bits_equivalent() -> f64 {
    (ZERO_INCLUSIVE_BIT_UNIT_STATES as f64).log2()
}

pub fn nsq_state_log10_for_processed_information(information_processed: Nu16) -> f64 {
    information_processed as f64 * (ZERO_INCLUSIVE_BIT_UNIT_STATES as f64).log10()
}

pub fn hertz_to_lever_position(hertz: f32) -> Nu16 {
    let hertz = hertz.clamp(0.0, 1.0);
    1 + (hertz * ((CANONICAL_LEVER_MAX_POSITION - 1) as f32)).round() as Nu16
}

pub fn lever_position_to_hertz(position: Nu16) -> f32 {
    let position = position.clamp(1, CANONICAL_LEVER_MAX_POSITION);
    (position - 1) as f32 / (CANONICAL_LEVER_MAX_POSITION - 1) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_inclusive_state_space_is_current_truth() {
        assert_eq!(TOTAL_STATES_PER_LEVER, 2254);
        assert_eq!(
            ZERO_INCLUSIVE_BIT_UNIT_STATES,
            (ZERO_INCLUSIVE_LEVER_STATES as u128).pow(CANONICAL_BIT_UNIT_LEVERS as u32)
        );
    }

    #[test]
    fn return_averaging_resolves_high_lever_position() {
        let sample = LeverReturnSample {
            applied_hertz: 1.0,
            return_to_off: 1.0,
            return_to_on: 0.0,
            sound_resonance: Some(1.0),
        };
        let reading = resolve_lever_position_from_return(sample, 0.001);
        assert!(reading.stable);
        assert!(reading.sound_confirmed);
        assert!(!reading.missed);
        assert_eq!(reading.position, CANONICAL_LEVER_MAX_POSITION);
    }

    #[test]
    fn max_scan_reports_zero_failure_frontier() {
        let report = lever_max_zero_failure_scan(0.001);
        assert!(report.max_zero_failure_distance >= 1);
        assert_eq!(
            report.max_zero_failure_distance,
            report.max_zero_failure_information_processed
        );
    }

    #[test]
    fn spacing_sweep_selects_by_resolved_information_processed() {
        let report = lever_spacing_sweet_spot_report(0.001);

        assert_eq!(
            report.selected_spacing_units,
            GLOBAL_LEVER_SWEET_SPOT_SPACING_UNITS
        );
        assert_eq!(
            report.selected_information_processed,
            GLOBAL_LEVER_SWEET_SPOT_INFORMATION_PROCESSED
        );
        assert_eq!(report.selected_nsq_states_per_bit_unit, 25_811_642_826_256);
        assert_eq!(report.selected_boundary_bytes_equivalent, 6271);
        assert_eq!(report.selected_produced_characters_floor, 18_813);
        assert_eq!(report.selected_produced_characters_dense, 75_252);
        assert_eq!(report.selected_stamp_information_accepted, 1126);
        assert_eq!(report.selected_framework_stamp_payloads_accepted, 1126);
        assert_eq!(report.selected_noise_information_rejected, 0);
        assert!(report.selected_nsq_state_log10 > 15_101.0);
        assert_eq!(
            report.selected_stable_upper_position,
            CANONICAL_LEVER_MAX_POSITION
        );
        assert!(report.zero_failed_or_missed);
        assert_eq!(report.selection_basis, "resolved_nsq_information_processed");
        assert!(!report.bit_passthrough_basis);
        assert_eq!(
            report.byte_measurement_scope,
            "boundary_equivalent_bytes_only_ceil_processed_nsq_bit_units_log2_2254_pow_4_over_8"
        );
        assert!(report
            .measurement_methods
            .contains(&"sound_resonance".to_string()));
        assert!(report
            .measurement_methods
            .contains(&"near_anchor_similarity_diagnostic".to_string()));
        assert!(report.probes.iter().any(|probe| probe.spacing_units == 1
            && probe.max_zero_failure_distance == 983
            && probe.stamp_information_accepted == 983
            && probe.noise_information_rejected == 143
            && probe.first_failed_distance == Some(984)));
    }
}
