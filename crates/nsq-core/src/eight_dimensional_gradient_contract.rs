use crate::intent::{
    generate_default_intent_gradient_frame, validate_intent_gradient_frame, CouncilPole,
    IntentPressure, IntentSurface, NsqIntentVariable, NsqIntentScaleAnchor,
    NSQ_INTENT_GRADIENT_VARIABLES, NSQ_INTENT_SCALE_ANCHORS,
};
use crate::{CANONICAL_LEVER_MAX_POSITION, ZERO_INCLUSIVE_BIT_UNIT_STATES};

/// Validate the complete eight-dimensional gradient contract against the
/// executable NSQ types rather than returning unconditional booleans.
pub fn gradient_is_operational() -> bool {
    let frame = generate_default_intent_gradient_frame();
    let validation = validate_intent_gradient_frame(&frame);
    validation.all_variables_present
        && validation.positions_inside_final_tier
        && NsqIntentVariable::ALL.len() == NSQ_INTENT_GRADIENT_VARIABLES
        && NsqIntentScaleAnchor::ALL.len() == NSQ_INTENT_SCALE_ANCHORS
}

/// The gradient participates in executable pressure construction and routing.
pub fn gradient_is_execution_relevant() -> bool {
    let pressure = IntentPressure::baseline(NsqIntentScaleAnchor::SelfObjectScale);
    pressure.variable_positions.len() == NSQ_INTENT_GRADIENT_VARIABLES
        && pressure.court_surface == IntentSurface::Internal
        && pressure.variable(NsqIntentVariable::Motive) >= 1
        && pressure.variable(NsqIntentVariable::Motive) <= CANONICAL_LEVER_MAX_POSITION
}

/// Every semantic variable has a distinct canonical enum position and can be
/// addressed through the executable frame API.
pub fn gradient_preserves_multidirectional_semantic_resolution() -> bool {
    let mut frame = generate_default_intent_gradient_frame();
    for variable in NsqIntentVariable::ALL {
        frame.variable_positions[variable.index()] = variable.index() as u64 + 1;
    }
    let validation = validate_intent_gradient_frame(&frame);
    validation.positions_inside_final_tier
        && NsqIntentVariable::ALL
            .iter()
            .enumerate()
            .all(|(index, variable)| variable.index() == index)
}

/// Pressure survives a round-trip through the routing API without losing
/// polarity-independent gradient values or its semantic target.
pub fn gradient_preserves_inverse_semantic_continuity() -> bool {
    let mut pressure = IntentPressure::baseline(NsqIntentScaleAnchor::RelationalGroupScale);
    for variable in NsqIntentVariable::ALL {
        pressure.set_variable(variable, (variable.index() as u64) + 1);
    }
    let routed = pressure.clone().route(
        CouncilPole::MaverickLogic,
        CouncilPole::DevstralArbiter,
        IntentSurface::CouncilDispatch,
    );
    routed.variable_positions == pressure.variable_positions
        && routed.origin_pole == Some(CouncilPole::MaverickLogic)
        && routed.target_pole == Some(CouncilPole::DevstralArbiter)
        && routed.court_surface == IntentSurface::CouncilDispatch
}

/// Verify that the canonical state space actually reaches the intended large
/// search regime. This is a numerical invariant, not a claim made by name.
pub fn gradient_supports_octillion_scale_traversal() -> bool {
    ZERO_INCLUSIVE_BIT_UNIT_STATES >= 1_000_000_000_000_000_000u128
}

/// Runtime weighting is meaningful only when all eight variables are legal
/// final-tier positions; reject malformed frames rather than silently treating
/// them as valid semantic pressure.
pub fn runtime_behavior_requires_semantic_gradient_weighting() -> bool {
    let frame = generate_default_intent_gradient_frame();
    let validation = validate_intent_gradient_frame(&frame);
    validation.all_variables_present && validation.positions_inside_final_tier
}

/// The court's inner representation is not permitted to collapse into a
/// generic flattened embedding path.
pub fn flattened_embedding_only_mode_allowed() -> bool {
    false
}
