#!/usr/bin/env python3
import json
import pathlib
import sys

out_dir = pathlib.Path(sys.argv[1]).resolve()
out_dir.mkdir(parents=True, exist_ok=True)

profiles = {
    "tiny": 100,
    "small": 1000,
    "medium": 5000,
    "large": 20000,
    "pathological": 20000,
}

manifest = {}

for name, n in profiles.items():
    p = out_dir / f"{name}.nsq"
    with p.open("w", encoding="utf-8") as f:
        f.write("# generated scale corpus\n")
        for i in range(n):
            f.write(f"triple node.{i} -> links -> node.{(i+1)%max(n,1)} :layer 1 :plane 1 :anchor {1000+i} :weight 1 :flags 0\n")
            f.write(f"membrane node.{i} :state active :flux 1 :gate 1 :phase 1\n")
        if name == "pathological":
            for i in range(n):
                f.write(f"triple hot.root -> fanout -> leaf.{i} :layer 1 :plane 1 :anchor {500000+i} :weight 1 :flags 0\n")
    manifest[name] = {"path": str(p), "records": n}

(out_dir / "scale_manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
print(out_dir)
