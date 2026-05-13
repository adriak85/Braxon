#!/usr/bin/env python3
import re
from pathlib import Path

BUS = Path("/data/data/com.termux/files/home/Braxon/crates/braxon-core/src/bus.rs")
bus_src = BUS.read_text()

# Replace .unwrap_or_else(|_| CoachingMode::Variant) with .unwrap_or(CoachingMode::Variant)
new_src = re.sub(
    r'nsq_citadel::load_coaching_mode\("([^"]+)"\)\s*\.unwrap_or_else\(\|_\| CoachingMode::(\w+)\)',
    r'nsq_citadel::load_coaching_mode("\1").unwrap_or(CoachingMode::\2)',
    bus_src
)

if new_src != bus_src:
    BUS.write_text(new_src)
    print("OK: patched bus.rs → .unwrap_or(CoachingMode::Variant)")
else:
    print("SKIP: no .unwrap_or_else pattern found to patch")

print("DONE. Run: cargo run --all-features --release")
