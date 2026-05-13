#!/usr/bin/env python3
"""
Fix nsq-citadel crate structure and CoachingMode fallback.
"""
import re, shutil, sys
from pathlib import Path

ROOT = Path("/data/data/com.termux/files/home/Braxon")
CITADEL = ROOT / "crates/nsq-citadel"
SRC = CITADEL / "src"
CARGO = CITADEL / "Cargo.toml"
BUS = ROOT / "crates/braxon-core/src/bus.rs"

# ── 1. Ensure src/ exists ──────────────────────────────────────────
SRC.mkdir(parents=True, exist_ok=True)

# ── 2. Move root .rs files into src/ (skip if already there) ───────
root_rs_files = [
    "lib.rs",
    "bit.rs",
    "capital.rs",
    "materialization.rs",
    "wire.rs",
]

for name in root_rs_files:
    src = CITADEL / name
    dst = SRC / name
    if src.exists() and not dst.exists():
        shutil.move(str(src), str(dst))
        print(f"OK: moved {name} → src/{name}")
    elif dst.exists():
        print(f"SKIP: src/{name} already exists")
    else:
        print(f"SKIP: {name} not found at root")

# ── 3. Fix Cargo.toml to point to src/lib.rs ───────────────────────
cargo_txt = CARGO.read_text()
if 'path = "lib.rs"' in cargo_txt:
    cargo_txt = cargo_txt.replace('path = "lib.rs"', 'path = "src/lib.rs"')
    CARGO.write_text(cargo_txt)
    print("OK: fixed Cargo.toml → path = src/lib.rs")
elif 'path = "src/lib.rs"' in cargo_txt:
    print("SKIP: Cargo.toml already points to src/lib.rs")
else:
    print("WARN: Cargo.toml lib path is unexpected; manual check needed")

# ── 4. Find first CoachingMode variant from src/coaching.rs ────────
coaching_path = SRC / "coaching.rs"
if not coaching_path.exists():
    print("ERROR: src/coaching.rs not found")
    sys.exit(1)

coaching_src = coaching_path.read_text()
m = re.search(r'pub enum CoachingMode \{([^}]+)\}', coaching_src, re.DOTALL)
if not m:
    print("ERROR: cannot parse CoachingMode enum")
    sys.exit(1)

body = m.group(1)
first_variant = None
for line in body.splitlines():
    line = line.strip()
    if not line or line.startswith('//'):
        continue
    vm = re.match(r'([A-Z][A-Za-z0-9]*)', line)
    if vm:
        first_variant = vm.group(1)
        break

if not first_variant:
    print("ERROR: no CoachingMode variant found")
    sys.exit(1)

print(f"OK: CoachingMode first variant = {first_variant}")

# ── 5. Patch bus.rs fallback ───────────────────────────────────────
bus_src = BUS.read_text()
old = "CoachingMode::default()"
new = f"CoachingMode::{first_variant}"
if old in bus_src:
    bus_src = bus_src.replace(old, new)
    BUS.write_text(bus_src)
    print(f"OK: patched bus.rs fallback → {new}")
elif new in bus_src:
    print(f"SKIP: bus.rs already uses {new}")
else:
    print("WARN: bus.rs fallback pattern not found; manual check needed")

print("\nDONE. Run: cargo run --all-features --release")
