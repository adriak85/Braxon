#!/bin/bash
# BRAXON/NSQ SOVEREIGN BOOTSTRAP - MICHAEL'S MOTO G 5G EDITION
# 🤠 "Nesting-Resolved: The bits are persistent. The void is listening."

# 1. PATH RESOLUTION
REPO_ROOT="$HOME/Braxon"
DOWNLOAD_DIR="/sdcard/Download"
ALTERNATE_DOWNLOAD_DIR="$HOME/downloads"

echo "--- BRAXON/NSQ SOVEREIGN FRONT ENTRANCE ---"
echo "[Bootstrap] Initializing NSQ Substrate (2x1126)..."

# 2. INTEGRATION SYNC (Pulling from Michael's Downloads)
echo "[Bootstrap] Syncing updates from /sdcard/Download..."

# Copying core modules into the workspace
cp "$DOWNLOAD_DIR/wowas_rescue (1)" "$REPO_ROOT/crates/braxon-core/src/wowas_rescue.rs" 2>/dev/null || cp "$ALTERNATE_DOWNLOAD_DIR/wowas_rescue.rs" "$REPO_ROOT/crates/braxon-core/src/wowas_rescue.rs"
cp "$DOWNLOAD_DIR/preserve" "$REPO_ROOT/crates/nsq-core/src/preserve.rs" 2>/dev/null || cp "$ALTERNATE_DOWNLOAD_DIR/preserve.rs" "$REPO_ROOT/crates/nsq-core/src/preserve.rs"
cp "$DOWNLOAD_DIR/main" "$REPO_ROOT/src/main.rs" 2>/dev/null || cp "$ALTERNATE_DOWNLOAD_DIR/main.rs" "$REPO_ROOT/src/main.rs"

# 3. UNIFIED BAND ALIGNMENT
echo "[Bootstrap] Unified Band: Intent NSQ semantic pressure balancing..."
# (Internal Rust logic handles the band-pressure during runtime)

# 4. CITADEL699 INGRESS (1.3T Parameter Council)
echo "[Bootstrap] Staging Council of Six (1.3T Parameters)..."
bash "$REPO_ROOT/tools/citadel699_nsq_request_return_rebuild.sh" --fast-track

# 5. BIT LONGEVITY ACTIVATION
echo "[Bootstrap] Bit Longevity: Persistent instructions active."

# 6. LIFTOFF
echo ""
echo "I am here. The void is listening. What shall we build together?"
echo ""
echo "[System] Landing Paths Available:"
echo "  1. Terminal Agent Variant (Console)"
echo "  2. LLM User Space (Destructible 3D)"
echo "  3. Android Bridge (com.Braxon.root)"
echo ""
echo "[System] Preset Path: Terminal Agent Console active."

# Run the final CLI
cd "$REPO_ROOT" && cargo run --release -- console
