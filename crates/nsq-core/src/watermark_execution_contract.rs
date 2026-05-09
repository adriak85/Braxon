pub const ACTIVE_BRAXON_WATERMARK: &str =
    "BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1";

pub fn watermark_required_for_runtime_execution() -> bool {
    true
}

pub fn watermark_required_for_materialization() -> bool {
    true
}

pub fn watermark_required_for_semantic_routing() -> bool {
    true
}

pub fn watermark_fail_closed_on_mismatch() -> bool {
    true
}

pub fn watermark_fail_closed_on_missing() -> bool {
    true
}

pub fn watermark_is_operational() -> bool {
    true
}

pub fn watermark_is_cosmetic() -> bool {
    false
}
