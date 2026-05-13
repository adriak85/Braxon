#!/usr/bin/env python3
import re
from pathlib import Path

ROOT = Path("/data/data/com.termux/files/home/Braxon")

# ── FIX 1: Point Cargo.toml to the actual lib.rs location ──────────
cargo = ROOT / "crates/nsq-citadel/Cargo.toml"
cargo_src = cargo.read_text()

if 'path = "src/lib.rs"' in cargo_src:
    cargo_src = cargo_src.replace('path = "src/lib.rs"', 'path = "lib.rs"')
    cargo.write_text(cargo_src)
    print("OK: fixed nsq-citadel/Cargo.toml → path = lib.rs")
else:
    print("SKIP: Cargo.toml already patched or different path")

# ── FIX 2: Find the first CoachingMode variant and patch bus.rs ───
# Try src/coaching.rs first, then coaching.rs at root
coaching_paths = [
    ROOT / "crates/nsq-citadel/src/coaching.rs",
    ROOT / "crates/nsq-citadel/coaching.rs",
]

coaching_file = None
for p in coaching_paths:
    if p.exists():
        coaching_file = p
        break

if coaching_file is None:
    print("ERROR: cannot find coaching.rs")
    sys.exit(1)

coaching_src = coaching_file.read_text()
m = re.search(r'pub enum CoachingMode \{([^}]+)\}', coaching_src, re.DOTALL)

if not m:
    print("ERROR: cannot parse CoachingMode enum")
    sys.exit(1)

body = m.group(1)
# Find first variant (ignore comments and whitespace)
first_variant = None
for line in body.splitlines():
    line = line.strip()
    if not line or line.startswith('//'):
        continue
    # Match CamelCase identifier optionally followed by data
    vm = re.match(r'([A-Z][A-Za-z0-9]*)', line)
    if vm:
        first_variant = vm.group(1)
        break

if not first_variant:
    print("ERROR: no CoachingMode variant found")
    sys.exit(1)

print(f"OK: CoachingMode first variant = {first_variant}")

# Patch bus.rs to use this variant instead of Default
bus = ROOT / "crates/braxon-core/src/bus.rs"
bus_src = bus.read_text()

old_fallback = "CoachingMode::default()"
new_fallback = f"CoachingMode::{first_variant}"

if old_fallback in bus_src:
    bus_src = bus_src.replace(old_fallback, new_fallback)
    bus.write_text(bus_src)
    print(f"OK: patched bus.rs fallback → {new_fallback}")
else:
    print("SKIP: bus.rs already patched or uses different fallback")

print("DONE. Run: cargo run --all-features --release")
