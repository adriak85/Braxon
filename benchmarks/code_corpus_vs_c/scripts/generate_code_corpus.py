from pathlib import Path
import json
import random

root = Path.home() / "Braxon" / "benchmarks" / "code_corpus_vs_c"
matrix = json.loads((root / "specs" / "code_corpus_matrix.json").read_text())
corpus_root = root / "corpus"

rng = random.Random(260426)

def emit_file_text(tier, file_idx, file_count, fn_count, depth, fanout, corruption):
    lines = []
    lines.append(f"// tier={tier} file={file_idx}")
    lines.append(f"module mod_{tier}_{file_idx};")
    lines.append("")
    if file_idx > 0:
        for j in range(min(3, file_idx)):
            lines.append(f"use mod_{tier}_{j}::fn_{j}_0;")
    lines.append("")
    for fn_idx in range(fn_count):
        fn_name = f"fn_{file_idx}_{fn_idx}"
        lines.append(f"/// doc: function {fn_name} in tier {tier}")
        lines.append(f"fn {fn_name}(arg0: i32, arg1: i32) -> i32 {{")
        lines.append("    let base = arg0 + arg1;")
        for d in range(depth):
            lines.append(f"    let level_{d} = base + {d};")
            lines.append(f"    if level_{d} % 2 == 0 {{")
            lines.append(f"        let shadow = level_{d} * {d+2};")
            lines.append("    } else {")
            lines.append(f"        let shadow = level_{d} - {d+3};")
            lines.append("    }")
        for k in range(fanout):
            target_file = (file_idx + k) % max(file_count, 1)
            target_fn = k % max(fn_count, 1)
            lines.append(f"    let call_{k} = fn_{target_file}_{target_fn}(base, {k});")
        lines.append("    let mut acc = base;")
        lines.append("    for i in 0..4 {")
        lines.append("        acc = acc + i;")
        lines.append("    }")
        lines.append("    return acc;")
        lines.append("}")
        lines.append("")
    text = "\n".join(lines) + "\n"

    if corruption > 0:
        chars = list(text)
        flips = int(len(chars) * corruption)
        for _ in range(flips):
            i = rng.randrange(len(chars))
            chars[i] = rng.choice(["@", "#", "$", "%", "^", "&"])
        text = "".join(chars)

    return text

for tier, cfg in matrix["tiers"].items():
    tier_dir = corpus_root / tier
    tier_dir.mkdir(parents=True, exist_ok=True)
    for old in tier_dir.glob("*.code"):
        old.unlink()

    for file_idx in range(cfg["files"]):
        text = emit_file_text(
            tier=tier,
            file_idx=file_idx,
            file_count=cfg["files"],
            fn_count=cfg["functions_per_file"],
            depth=cfg["depth"],
            fanout=cfg["fanout"],
            corruption=cfg["corruption"]
        )
        (tier_dir / f"file_{file_idx:03d}.code").write_text(text)

print(json.dumps({
    "version": 1,
    "corpus_root": str(corpus_root),
    "tiers": list(matrix["tiers"].keys())
}, indent=2))
