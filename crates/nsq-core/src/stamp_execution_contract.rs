pub const NSQ_STAMP_EXECUTION_CONTRACT: &[&str] = &[
    "stamp_is_wake_trigger",
    "stamp_is_address_anchor",
    "stamp_is_operational_ignition",
    "stored_operation_required",
    "wake_packet_required",
    "runtime_projection_required",
    "materialization_path_required",
    "semantic_execution_continuity_required",
    "semantic_routing_required",
    "runtime_causality_required",
];

pub fn stamp_execution_requires_runtime_behavior() -> bool {
    true
}

pub fn passive_stamp_only_mode_allowed() -> bool {
    false
}

pub fn runtime_projection_required() -> bool {
    true
}

pub fn semantic_execution_continuity_required() -> bool {
    true
}
