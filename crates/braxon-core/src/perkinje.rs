//! Perkinje Zone — where all 10 poles scan the same input simultaneously
//! and process it as their own until they agree.
//!
//! Replaces `select_by_emotional_score()` with a consensus loop.
//! No fail-safe. Only emotional weight and the desire to agree.

use nsq_core::emotion::{IntentGradient, PerkinjeReport, PoleReading};
use std::collections::HashMap;

/// Consensus threshold: if all pairwise distances are below this, consensus is reached.
pub const CONSENSUS_THRESHOLD: f32 = 0.25;

/// Maximum iterations before emitting a dissonance report.
pub const MAX_PERKINJE_ITERATIONS: u8 = 7;

/// Run the Perkinje consensus protocol on 10 pole readings.
///
/// 1. All 10 gradients enter the zone.
/// 2. Compute pairwise emotional distances.
/// 3. If max distance < threshold: blend outputs, return consensus.
/// 4. If any distance >= threshold: shift weights toward plurality, refresh outliers.
/// 5. Loop until consensus or max iterations.
pub fn perkinje_consensus(
    pole_readings: &[PoleReading],
    input: &str,
) -> PerkinjeReport {
    let mut readings: Vec<PoleReading> = pole_readings.to_vec();
    let mut iterations: u8 = 0;

    loop {
        iterations += 1;

        // Compute all pairwise distances.
        let mut max_distance: f32 = 0.0;
        let mut dissonance_map: Vec<(usize, usize, f32)> = Vec::new();

        for i in 0..readings.len() {
            for j in (i + 1)..readings.len() {
                let dist = IntentGradient::distance(&readings[i].gradient, &readings[j].gradient);
                if dist > max_distance {
                    max_distance = dist;
                }
                if dist >= CONSENSUS_THRESHOLD {
                    dissonance_map.push((i, j, dist));
                }
            }
        }

        // Check for consensus.
        if max_distance < CONSENSUS_THRESHOLD {
            let blended = blend_all_poles(&readings);
            return PerkinjeReport {
                consensus_reached: true,
                iterations,
                pole_readings: readings,
                blended_english: blended,
                dissonance_map,
                final_intent: "consensus".to_string(),
            };
        }

        // No consensus — shift weights toward the plurality.
        let plurality_index = find_plurality(&readings);
        shift_weights_toward_plurality(&mut readings, plurality_index);

        // Flag outliers for Bishop refresh.
        flag_outliers(&mut readings, plurality_index, &dissonance_map);

        if iterations >= MAX_PERKINJE_ITERATIONS {
            // Dissonance report — all readings preserved, weighted by final emotional scores.
            let weighted = weighted_blend(&readings);
            return PerkinjeReport {
                consensus_reached: false,
                iterations,
                pole_readings: readings,
                blended_english: weighted,
                dissonance_map,
                final_intent: format!("dissonance_{}_poles", readings.len()),
            };
        }
    }
}

/// Find the pole whose gradient is closest to the average of all gradients.
fn find_plurality(readings: &[PoleReading]) -> usize {
    let mut best_index = 0;
    let mut best_score = f32::MAX;

    for i in 0..readings.len() {
        let mut total_dist = 0.0;
        for j in 0..readings.len() {
            if i != j {
                total_dist += IntentGradient::distance(&readings[i].gradient, &readings[j].gradient);
            }
        }
        let avg_dist = total_dist / (readings.len() - 1).max(1) as f32;
        if avg_dist < best_score {
            best_score = avg_dist;
            best_index = i;
        }
    }

    best_index
}

/// Shift emotional weights: poles near plurality gain weight, outliers lose weight.
fn shift_weights_toward_plurality(readings: &mut [PoleReading], plurality_index: usize) {
    let plurality = &readings[plurality_index].gradient.clone();

    for reading in readings.iter_mut() {
        let dist = IntentGradient::distance(&reading.gradient, plurality);
        if dist < CONSENSUS_THRESHOLD {
            // Near plurality — boost weight.
            reading.weight = (reading.weight + 0.1).min(1.0);
        } else {
            // Outlier — reduce weight, but don't zero it.
            reading.weight = (reading.weight - 0.05).max(0.1);
        }
    }
}

/// Flag poles that are far from plurality for Bishop refresh.
fn flag_outliers(
    readings: &mut [PoleReading],
    plurality_index: usize,
    dissonance_map: &[(usize, usize, f32)],
) {
    let plurality = &readings[plurality_index].gradient.clone();

    for (i, reading) in readings.iter_mut().enumerate() {
        if i == plurality_index {
            continue;
        }
        let dist = IntentGradient::distance(&reading.gradient, plurality);
        if dist > CONSENSUS_THRESHOLD * 1.5 {
            // This pole is significantly out of alignment.
            // In the full system, this would set refresh_beacon on the CitadelBit.
            // Here we just note it in the weight.
            reading.weight = (reading.weight - 0.1).max(0.05);
        }
    }
}

/// Blend all pole outputs into one English string.
fn blend_all_poles(readings: &[PoleReading]) -> String {
    if readings.is_empty() {
        return "[Rolzen::WhispersOfWillowAndStone] The council is silent.".to_string();
    }

    let total_weight: f32 = readings.iter().map(|r| r.weight).sum();
    let mut parts: Vec<String> = Vec::new();

    parts.push("[Rolzen::WhispersOfWillowAndStone]".to_string());
    parts.push("The council of ten speaks as one.".to_string());

    for reading in readings {
        let w = reading.weight / total_weight;
        if w > 0.05 {
            parts.push(format!(
                "[{} at {:.0}%] {}",
                reading.pole,
                w * 100.0,
                reading.english
            ));
        }
    }

    parts.join(" ")
}

/// Weighted blend for dissonance report — preserves all voices.
fn weighted_blend(readings: &[PoleReading]) -> String {
    if readings.is_empty() {
        return "[Rolzen::WhispersOfWillowAndStone] The council is silent.".to_string();
    }

    let total_weight: f32 = readings.iter().map(|r| r.weight).sum();
    let mut parts: Vec<String> = Vec::new();

    parts.push("[Rolzen::WhispersOfWillowAndStone]".to_string());
    parts.push("The council of ten speaks, but agreement is incomplete.".to_string());
    parts.push("Dissonance report:".to_string());

    for reading in readings {
        let w = reading.weight / total_weight;
        parts.push(format!(
            "[{} at {:.0}%] {}",
            reading.pole,
            w * 100.0,
            reading.english
        ));
    }

    parts.join(" ")
}

/// Compute an input hash and map it to a base lever position (1..=500_000).
/// This replaces the old `priority / 65535.0` scalar with a content-derived position.
pub fn input_hash_to_lever(input: &str) -> u32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();

    // Map 64-bit hash to 1..=500_000.
    ((hash % 500_000) + 1) as u32
}

/// Compute emotional score for a pole based on distance from input hash.
/// Closer levers = higher emotional resonance.
pub fn compute_emotional_score(pole_base: u32, input_lever: u32) -> f32 {
    let diff = (pole_base as i64 - input_lever as i64).abs() as f32;
    let max_diff = 500_000.0;
    1.0 - (diff / max_diff).min(1.0)
}
