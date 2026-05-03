use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const COUNCIL_MODEL_COUNT: usize = 6;
pub const INTENT_GRADIENT_ANCHOR_CHOICES: u16 = 2;
pub const INTENT_GRADIENT_SEMANTIC_LANGUAGE: &str = "nsq.intent_gradient.semantic_language.v1";
pub const SEMANTIC_INTENT_ID_GOAL: &str = "every_possible_semantic_intent_id";
pub const BINARY_BYTE_SWITCHES: u16 = 8;
pub const STANDARD_BINARY_BYTE_STATES: u16 = 256;
pub const NSQ_BYTE_SCALE_GRADIENT_STATES: &str = "666240905390212827402977536";
pub const NSQ_BYTE_SCALE_MARGIN_FLOOR: &str = "greater_than_90_septillion_states";
pub const SENSORY_GENERATION_BODY_COUNT: usize = 4;
pub const GRAPHICS_FOOTPRINT_ALLOWANCE_UNITS: u16 = 50;
pub const NEXT_SENSORY_FOCUS: &str = "nsq_to_indextts2_emotional_frequency_mapping";
pub const INDEXTTS2_EMOTIONAL_CHANNEL_COUNT: usize = 7;
pub const NSQ_ZERO_INCLUSIVE_LEVER_STATES: u16 = 2254;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CouncilRole {
    MaverickLogic,
    QwenCreativity,
    ArbiterJudge,
    AnalyzerAuditor,
    LimbicEmpath,
    SupportMemory,
}

impl CouncilRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MaverickLogic => "maverick_logic",
            Self::QwenCreativity => "qwen_creativity",
            Self::ArbiterJudge => "arbiter_judge",
            Self::AnalyzerAuditor => "analyzer_auditor",
            Self::LimbicEmpath => "limbic_empath",
            Self::SupportMemory => "support_memory",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrainRegion {
    PrefrontalCortex,
    DefaultModeNetwork,
    AnteriorCingulateCortex,
    InsularSalienceNetwork,
    LimbicSystem,
    HippocampalFormation,
}

impl BrainRegion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PrefrontalCortex => "prefrontal_cortex",
            Self::DefaultModeNetwork => "default_mode_network",
            Self::AnteriorCingulateCortex => "anterior_cingulate_cortex",
            Self::InsularSalienceNetwork => "insular_salience_network",
            Self::LimbicSystem => "limbic_system",
            Self::HippocampalFormation => "hippocampal_formation",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilMember {
    pub id: String,
    pub role: CouncilRole,
    pub brain_region: BrainRegion,
    pub model_source: String,
    pub cognitive_pole: String,
    pub intent_gradient_bias: String,
    pub balanced_pressure_share: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentGradientSemanticLanguage {
    pub language_id: String,
    pub canonical_semantics: String,
    pub anchor_choices: u16,
    pub anchor_meaning: String,
    pub semantic_intent_id_goal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedThoughtPressure {
    pub language: IntentGradientSemanticLanguage,
    pub region_count: usize,
    pub balanced_share_per_region: f32,
    pub balance_mode: String,
    pub scale_adaptation_mode: String,
    pub binary_correction: String,
    pub nsq_byte_scale_gradient_states: String,
    pub nsq_byte_scale_margin_floor: String,
    pub all_regions_present: bool,
    pub all_regions_unique: bool,
    pub unified_pressure_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryGenerationBody {
    pub numeric_id: u16,
    pub numeric_id_base8: String,
    pub semantic_id: String,
    pub body: String,
    pub model: String,
    pub role: String,
    pub integration: String,
    pub constraint: String,
    pub nsq_route: Vec<String>,
    pub graphics_footprint_units: u16,
    pub todo: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryGenerationRoster {
    pub canonical_semantics: String,
    pub extends_council_of_six: bool,
    pub replaces_council_of_six: bool,
    pub graphics_footprint_allowance: u16,
    pub graphics_footprint_used: u16,
    pub footprint_within_allowance: bool,
    pub next_focus: String,
    pub bodies: Vec<SensoryGenerationBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalFrequencyChannel {
    pub numeric_id: u16,
    pub numeric_id_base8: String,
    pub semantic_id: String,
    pub intent_axis: String,
    pub acoustic_target: String,
    pub anchor_pair: String,
    pub lever_states: u16,
    pub default_position: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexTts2EmotionalFrequencyMap {
    pub canonical_semantics: String,
    pub model: String,
    pub voice_body_semantic_id: String,
    pub zero_shot_synthesis: bool,
    pub zero_inclusive_lever_states: u16,
    pub carrier_rule: String,
    pub channels: Vec<EmotionalFrequencyChannel>,
    pub next_runtime_todo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilDecision {
    pub topic: String,
    pub consensus: String,
    pub confidence: f32,
    pub member_votes: BTreeMap<String, String>,
    pub thought_pressure: UnifiedThoughtPressure,
}

pub struct CouncilOfSix {
    pub members: Vec<CouncilMember>,
}

impl CouncilOfSix {
    pub fn new() -> Self {
        Self {
            members: vec![
                CouncilMember {
                    id: "maverick".to_string(),
                    role: CouncilRole::MaverickLogic,
                    brain_region: BrainRegion::PrefrontalCortex,
                    model_source: "deepseek-v3-671b".to_string(),
                    cognitive_pole: "executive".to_string(),
                    intent_gradient_bias: "logic_action_selection".to_string(),
                    balanced_pressure_share: balanced_pressure_share(),
                },
                CouncilMember {
                    id: "qwen".to_string(),
                    role: CouncilRole::QwenCreativity,
                    brain_region: BrainRegion::DefaultModeNetwork,
                    model_source: "qwen3-235b-a22b".to_string(),
                    cognitive_pole: "dreamer".to_string(),
                    intent_gradient_bias: "creative_semantic_expansion".to_string(),
                    balanced_pressure_share: balanced_pressure_share(),
                },
                CouncilMember {
                    id: "arbiter".to_string(),
                    role: CouncilRole::ArbiterJudge,
                    brain_region: BrainRegion::AnteriorCingulateCortex,
                    model_source: "qwen2.5-72b".to_string(),
                    cognitive_pole: "judge".to_string(),
                    intent_gradient_bias: "conflict_resolution".to_string(),
                    balanced_pressure_share: balanced_pressure_share(),
                },
                CouncilMember {
                    id: "analyzer".to_string(),
                    role: CouncilRole::AnalyzerAuditor,
                    brain_region: BrainRegion::InsularSalienceNetwork,
                    model_source: "deepseek-v3-671b-analyzer".to_string(),
                    cognitive_pole: "auditor".to_string(),
                    intent_gradient_bias: "salience_audit_and_proof".to_string(),
                    balanced_pressure_share: balanced_pressure_share(),
                },
                CouncilMember {
                    id: "limbic".to_string(),
                    role: CouncilRole::LimbicEmpath,
                    brain_region: BrainRegion::LimbicSystem,
                    model_source: "llama3.3-70b".to_string(),
                    cognitive_pole: "empath".to_string(),
                    intent_gradient_bias: "affective_weight_and_subtext".to_string(),
                    balanced_pressure_share: balanced_pressure_share(),
                },
                CouncilMember {
                    id: "support".to_string(),
                    role: CouncilRole::SupportMemory,
                    brain_region: BrainRegion::HippocampalFormation,
                    model_source: "gemma3-27b".to_string(),
                    cognitive_pole: "memory".to_string(),
                    intent_gradient_bias: "continuity_context_recall".to_string(),
                    balanced_pressure_share: balanced_pressure_share(),
                },
            ],
        }
    }

    pub fn unified_thought_pressure(&self) -> UnifiedThoughtPressure {
        let region_count = self.members.len();
        let unique_regions = self
            .members
            .iter()
            .map(|member| member.brain_region)
            .collect::<BTreeSet<_>>();
        UnifiedThoughtPressure {
            language: IntentGradientSemanticLanguage {
                language_id: INTENT_GRADIENT_SEMANTIC_LANGUAGE.to_string(),
                canonical_semantics: "base8_switch_topology".to_string(),
                anchor_choices: INTENT_GRADIENT_ANCHOR_CHOICES,
                anchor_meaning: "two full anchor choices inside NSQ switch topology, never byte-native width".to_string(),
                semantic_intent_id_goal: SEMANTIC_INTENT_ID_GOAL.to_string(),
            },
            region_count,
            balanced_share_per_region: balanced_pressure_share(),
            balance_mode: "six_region_equal_pressure_round_table".to_string(),
            scale_adaptation_mode: "language_selected_highest_stable_margin".to_string(),
            binary_correction: format!(
                "binary_bit_is_one_switch_binary_byte_is_{BINARY_BYTE_SWITCHES}_switches_{STANDARD_BINARY_BYTE_STATES}_states"
            ),
            nsq_byte_scale_gradient_states: NSQ_BYTE_SCALE_GRADIENT_STATES.to_string(),
            nsq_byte_scale_margin_floor: NSQ_BYTE_SCALE_MARGIN_FLOOR.to_string(),
            all_regions_present: region_count == COUNCIL_MODEL_COUNT,
            all_regions_unique: unique_regions.len() == COUNCIL_MODEL_COUNT,
            unified_pressure_ready: region_count == COUNCIL_MODEL_COUNT
                && unique_regions.len() == COUNCIL_MODEL_COUNT,
        }
    }

    pub fn brain_region_map(&self) -> BTreeMap<String, String> {
        self.members
            .iter()
            .map(|member| {
                (
                    member.model_source.clone(),
                    member.brain_region.as_str().to_string(),
                )
            })
            .collect()
    }

    pub fn sensory_generation_roster(&self) -> SensoryGenerationRoster {
        let bodies = vec![
            SensoryGenerationBody {
                numeric_id: 1,
                numeric_id_base8: "1".to_string(),
                semantic_id: "braxon.sensory_generation.image_cortex".to_string(),
                body: "image_cortex".to_string(),
                model: "FLUX.1-dev".to_string(),
                role: "highest_level_artistic_images_textures_and_aura_designs".to_string(),
                integration: "main_mind_visual_intent_to_nsq_image_route".to_string(),
                constraint: "native_nsq_image_route_boundary_output_only".to_string(),
                nsq_route: vec![
                    "policer".to_string(),
                    "router".to_string(),
                    "nsq_image_route".to_string(),
                    "compositor".to_string(),
                    "inspector".to_string(),
                ],
                graphics_footprint_units: 12,
                todo: vec![
                    "define_visual_intent_lever_anchors".to_string(),
                    "bind_image_output_inspection_to_chain_wake_registry".to_string(),
                ],
            },
            SensoryGenerationBody {
                numeric_id: 2,
                numeric_id_base8: "2".to_string(),
                semantic_id: "braxon.sensory_generation.video_cortex".to_string(),
                body: "video_cortex".to_string(),
                model: "Wan2.1-T2V-14B".to_string(),
                role: "physically_possible_motion_momentum_and_fluid_dynamics".to_string(),
                integration: "physical_consequences_of_citadel_intent".to_string(),
                constraint: "pure_visual_renderer_no_independent_decision_authority".to_string(),
                nsq_route: vec![
                    "policer".to_string(),
                    "router".to_string(),
                    "nsq_video_route".to_string(),
                    "scheduler".to_string(),
                    "inspector".to_string(),
                ],
                graphics_footprint_units: 14,
                todo: vec![
                    "add_motion_physics_check_gates".to_string(),
                    "verify_citadel_intent_physical_plausibility".to_string(),
                ],
            },
            SensoryGenerationBody {
                numeric_id: 3,
                numeric_id_base8: "3".to_string(),
                semantic_id: "braxon.sensory_generation.voice_body".to_string(),
                body: "voice_body".to_string(),
                model: "IndexTTS2".to_string(),
                role: "physical_left_brain_voice_with_emotional_variability".to_string(),
                integration: "semantic_intent_pressure_to_emotional_acoustic_frequency".to_string(),
                constraint: "anchored_to_2254_state_nsq_levers_for_zero_shot_synthesis".to_string(),
                nsq_route: vec![
                    "policer".to_string(),
                    "router".to_string(),
                    "nsq_voice_route".to_string(),
                    "scheduler".to_string(),
                    "inspector".to_string(),
                ],
                graphics_footprint_units: 8,
                todo: vec![
                    "focus_next_on_nsq_to_indextts2_emotional_frequency_mapping".to_string(),
                    "map_valence_arousal_breath_tremor_timbre_cadence_and_emphasis".to_string(),
                ],
            },
            SensoryGenerationBody {
                numeric_id: 4,
                numeric_id_base8: "4".to_string(),
                semantic_id: "braxon.sensory_generation.world_body_3d".to_string(),
                body: "world_body_3d".to_string(),
                model: "Hunyuan3D-2.1".to_string(),
                role: "two_d_imagination_to_three_d_mesh_spatial_worldbuilding".to_string(),
                integration: "final_spatial_resolution_for_citadel_environment".to_string(),
                constraint: "geometry_and_physics_body_canonical_intent_remains_nsq".to_string(),
                nsq_route: vec![
                    "policer".to_string(),
                    "router".to_string(),
                    "nsq_world_route".to_string(),
                    "optimizer".to_string(),
                    "inspector".to_string(),
                ],
                graphics_footprint_units: 16,
                todo: vec![
                    "define_two_d_to_mesh_semantic_bridge".to_string(),
                    "add_spatial_consistency_checks".to_string(),
                ],
            },
        ];
        let graphics_footprint_used = bodies
            .iter()
            .map(|body| body.graphics_footprint_units)
            .sum();

        SensoryGenerationRoster {
            canonical_semantics: "base8_switch_topology".to_string(),
            extends_council_of_six: true,
            replaces_council_of_six: false,
            graphics_footprint_allowance: GRAPHICS_FOOTPRINT_ALLOWANCE_UNITS,
            graphics_footprint_used,
            footprint_within_allowance: graphics_footprint_used
                <= GRAPHICS_FOOTPRINT_ALLOWANCE_UNITS,
            next_focus: NEXT_SENSORY_FOCUS.to_string(),
            bodies,
        }
    }

    pub fn index_tts2_emotional_frequency_map(&self) -> IndexTts2EmotionalFrequencyMap {
        let channels = vec![
            EmotionalFrequencyChannel {
                numeric_id: 1,
                numeric_id_base8: "1".to_string(),
                semantic_id: "braxon.voice.emotion.valence".to_string(),
                intent_axis: "valence".to_string(),
                acoustic_target: "warmth_vs_coldness".to_string(),
                anchor_pair: "positive_negative_affect".to_string(),
                lever_states: NSQ_ZERO_INCLUSIVE_LEVER_STATES,
                default_position: 563,
            },
            EmotionalFrequencyChannel {
                numeric_id: 2,
                numeric_id_base8: "2".to_string(),
                semantic_id: "braxon.voice.emotion.arousal".to_string(),
                intent_axis: "arousal".to_string(),
                acoustic_target: "energy_and_activation".to_string(),
                anchor_pair: "calm_active".to_string(),
                lever_states: NSQ_ZERO_INCLUSIVE_LEVER_STATES,
                default_position: 563,
            },
            EmotionalFrequencyChannel {
                numeric_id: 3,
                numeric_id_base8: "3".to_string(),
                semantic_id: "braxon.voice.emotion.breath".to_string(),
                intent_axis: "breath".to_string(),
                acoustic_target: "breath_pressure_and_phrase_air".to_string(),
                anchor_pair: "closed_open_breath".to_string(),
                lever_states: NSQ_ZERO_INCLUSIVE_LEVER_STATES,
                default_position: 563,
            },
            EmotionalFrequencyChannel {
                numeric_id: 4,
                numeric_id_base8: "4".to_string(),
                semantic_id: "braxon.voice.emotion.tremor".to_string(),
                intent_axis: "tremor".to_string(),
                acoustic_target: "micro_shake_and_vulnerability".to_string(),
                anchor_pair: "steady_tremor".to_string(),
                lever_states: NSQ_ZERO_INCLUSIVE_LEVER_STATES,
                default_position: 1,
            },
            EmotionalFrequencyChannel {
                numeric_id: 5,
                numeric_id_base8: "5".to_string(),
                semantic_id: "braxon.voice.emotion.timbre".to_string(),
                intent_axis: "timbre".to_string(),
                acoustic_target: "color_grain_and_resonance".to_string(),
                anchor_pair: "bright_dark".to_string(),
                lever_states: NSQ_ZERO_INCLUSIVE_LEVER_STATES,
                default_position: 563,
            },
            EmotionalFrequencyChannel {
                numeric_id: 6,
                numeric_id_base8: "6".to_string(),
                semantic_id: "braxon.voice.emotion.cadence".to_string(),
                intent_axis: "cadence".to_string(),
                acoustic_target: "pace_pause_and_phrase_shape".to_string(),
                anchor_pair: "short_long_phrase".to_string(),
                lever_states: NSQ_ZERO_INCLUSIVE_LEVER_STATES,
                default_position: 563,
            },
            EmotionalFrequencyChannel {
                numeric_id: 7,
                numeric_id_base8: "7".to_string(),
                semantic_id: "braxon.voice.emotion.emphasis".to_string(),
                intent_axis: "emphasis".to_string(),
                acoustic_target: "stress_focus_and_salience".to_string(),
                anchor_pair: "soft_hard_stress".to_string(),
                lever_states: NSQ_ZERO_INCLUSIVE_LEVER_STATES,
                default_position: 563,
            },
        ];

        IndexTts2EmotionalFrequencyMap {
            canonical_semantics: "base8_switch_topology".to_string(),
            model: "IndexTTS2".to_string(),
            voice_body_semantic_id: "braxon.sensory_generation.voice_body".to_string(),
            zero_shot_synthesis: true,
            zero_inclusive_lever_states: NSQ_ZERO_INCLUSIVE_LEVER_STATES,
            carrier_rule:
                "each acoustic channel is resolved through NSQ anchor plus lever state; host audio parameters are boundary projection only"
                    .to_string(),
            channels,
            next_runtime_todo:
                "implement_nsq_voice_route_adapter_for_indextts2_boundary_controls".to_string(),
        }
    }

    pub fn run_consensus(&self, topic: &str) -> CouncilDecision {
        let member_votes = self
            .members
            .iter()
            .map(|member| {
                (
                    member.brain_region.as_str().to_string(),
                    format!(
                        "{}:{}:balanced:{:.6}",
                        member.role.as_str(),
                        member.intent_gradient_bias,
                        member.balanced_pressure_share
                    ),
                )
            })
            .collect();
        CouncilDecision {
            topic: topic.to_string(),
            consensus: "NSQ substrate expansion remains switch-faithful and Braxon-aligned."
                .to_string(),
            confidence: 0.98,
            member_votes,
            thought_pressure: self.unified_thought_pressure(),
        }
    }
}

impl Default for CouncilOfSix {
    fn default() -> Self {
        Self::new()
    }
}

fn balanced_pressure_share() -> f32 {
    1.0 / COUNCIL_MODEL_COUNT as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_six_models_are_internal_brain_regions() {
        let council = CouncilOfSix::new();
        let pressure = council.unified_thought_pressure();

        assert_eq!(council.members.len(), COUNCIL_MODEL_COUNT);
        assert_eq!(council.brain_region_map().len(), COUNCIL_MODEL_COUNT);
        assert!(pressure.all_regions_present);
        assert!(pressure.all_regions_unique);
        assert!(pressure.unified_pressure_ready);
    }

    #[test]
    fn unified_pressure_uses_intent_gradient_semantic_language() {
        let council = CouncilOfSix::new();
        let decision = council.run_consensus("semantic intent id coverage");

        assert_eq!(decision.member_votes.len(), COUNCIL_MODEL_COUNT);
        assert_eq!(
            decision.thought_pressure.language.language_id,
            INTENT_GRADIENT_SEMANTIC_LANGUAGE
        );
        assert_eq!(
            decision.thought_pressure.language.anchor_choices,
            INTENT_GRADIENT_ANCHOR_CHOICES
        );
        assert_eq!(
            decision.thought_pressure.language.semantic_intent_id_goal,
            SEMANTIC_INTENT_ID_GOAL
        );
        assert!(decision
            .thought_pressure
            .language
            .anchor_meaning
            .contains("never byte-native"));
        assert_eq!(
            decision.thought_pressure.nsq_byte_scale_gradient_states,
            NSQ_BYTE_SCALE_GRADIENT_STATES
        );
        assert_eq!(
            decision.thought_pressure.scale_adaptation_mode,
            "language_selected_highest_stable_margin"
        );
    }

    #[test]
    fn sensory_generation_roster_adds_four_bodies_with_50_unit_footprint() {
        let council = CouncilOfSix::new();
        let roster = council.sensory_generation_roster();
        let models = roster
            .bodies
            .iter()
            .map(|body| body.model.as_str())
            .collect::<BTreeSet<_>>();

        assert_eq!(council.members.len(), COUNCIL_MODEL_COUNT);
        assert_eq!(roster.bodies.len(), SENSORY_GENERATION_BODY_COUNT);
        assert_eq!(roster.graphics_footprint_allowance, 50);
        assert_eq!(roster.graphics_footprint_used, 50);
        assert!(roster.footprint_within_allowance);
        assert!(roster.extends_council_of_six);
        assert!(!roster.replaces_council_of_six);
        assert!(models.contains("FLUX.1-dev"));
        assert!(models.contains("Wan2.1-T2V-14B"));
        assert!(models.contains("IndexTTS2"));
        assert!(models.contains("Hunyuan3D-2.1"));
        assert_eq!(roster.next_focus, NEXT_SENSORY_FOCUS);
    }

    #[test]
    fn index_tts2_emotional_frequency_map_uses_seven_nsq_lever_channels() {
        let council = CouncilOfSix::new();
        let map = council.index_tts2_emotional_frequency_map();
        let axes = map
            .channels
            .iter()
            .map(|channel| channel.intent_axis.as_str())
            .collect::<BTreeSet<_>>();

        assert_eq!(map.model, "IndexTTS2");
        assert_eq!(map.channels.len(), INDEXTTS2_EMOTIONAL_CHANNEL_COUNT);
        assert_eq!(map.zero_inclusive_lever_states, 2254);
        assert!(map.zero_shot_synthesis);
        assert!(map.carrier_rule.contains("boundary projection only"));
        assert!(map.channels.iter().all(|channel| channel.lever_states
            == NSQ_ZERO_INCLUSIVE_LEVER_STATES
            && !channel.semantic_id.is_empty()
            && !channel.numeric_id_base8.is_empty()));
        assert!(axes.contains("valence"));
        assert!(axes.contains("arousal"));
        assert!(axes.contains("breath"));
        assert!(axes.contains("tremor"));
        assert!(axes.contains("timbre"));
        assert!(axes.contains("cadence"));
        assert!(axes.contains("emphasis"));
    }
}
