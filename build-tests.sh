#!/bin/bash
# Braxon CI/CD: Build Missing Test Infrastructure First

echo "🔨 BUILDING MISSING TEST INFRASTRUCTURE..."
echo "═══════════════════════════════════════════════"

# 1. Fix Clippy errors blocking compilation FIRST
cargo fix --workspace --all-features --broken-code || true
cargo clippy --fix --workspace --all-features --allow-dirty --allow-staged || true

# 2. Generate missing test stubs for 5 failures
mkdir -p crates/{braxon-core,braxon-ingest,nsq-runtime}/tests/failing

cat > crates/braxon-core/tests/nu128_oversight.rs << 'EOF'
#[test]
fn nu128_install_oversight_status_keeps_pipeline_safe_while_whole_model_is_blocked() {
    let status = Nu128Status::default();
    assert!(status.direct_source_path_ready, "Pipeline safety requires direct path");
}
