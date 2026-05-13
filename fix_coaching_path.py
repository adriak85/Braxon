#!/usr/bin/env python3
import re
from pathlib import Path

BUS = Path("/data/data/com.termux/files/home/Braxon/crates/braxon-core/src/bus.rs")
bus_src = BUS.read_text()

# Replace the broken load_coaching_mode_or_default function
old_pattern = r'fn load_coaching_mode_or_default\(\) -> CoachingMode \{[^}]+nsq_citadel::load_coaching_mode\("[^"]+"\)[^}]+unwrap_or\(CoachingMode::\w+\)[^}]*\}'

new_code = """fn load_coaching_mode_or_default() -> CoachingMode {
    nsq_citadel::load_coaching_mode(std::path::Path::new("config/nsq/coaching.json"))
}"""

new_src = re.sub(old_pattern, new_code, bus_src, flags=re.DOTALL)

if new_src != bus_src:
    BUS.write_text(new_src)
    print("OK: patched bus.rs → load_coaching_mode(&Path) without unwrap")
else:
    print("SKIP: pattern not found; checking for alternate forms...")
    # Try simpler replacement
    bus_src = bus_src.replace(
        'nsq_citadel::load_coaching_mode("config/nsq/coaching.json")\n        .unwrap_or(CoachingMode::',
        'nsq_citadel::load_coaching_mode(std::path::Path::new("config/nsq/coaching.json"))\n        // '
    )
    # Remove the variant line and closing brace of the old function
    lines = bus_src.splitlines()
    out_lines = []
    skip_until_brace = False
    for line in lines:
        if 'nsq_citadel::load_coaching_mode(std::path::Path::new' in line:
            out_lines.append('    nsq_citadel::load_coaching_mode(std::path::Path::new("config/nsq/coaching.json"))')
            out_lines.append('}')
            skip_until_brace = True
            continue
        if skip_until_brace:
            if line.strip() == '}':
                skip_until_brace = False
            continue
        out_lines.append(line)
    bus_src = '\n'.join(out_lines)
    BUS.write_text(bus_src)
    print("OK: patched bus.rs via line replacement")

print("DONE. Run: cargo run --all-features --release")
