#!/usr/bin/env python3
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1]).resolve()
root.mkdir(parents=True, exist_ok=True)

families = {
    "chain": 4000,
    "fanout": 4000,
    "dense": 2000,
    "mixed": 5000,
    "duplicate_stress": 3000,
    "pathological": 8000,
    "membrane_dense": 12000,
}

tasks = []

for name, n in families.items():
    p = root / f"{name}.nsq"
    with p.open("w", encoding="utf-8") as f:
        f.write("# neutral task-equivalent corpus\n")
        if name == "chain":
            for i in range(n):
                f.write(f"triple node.{i} -> links -> node.{(i+1)%n} :layer 1 :plane 1 :anchor {1000+i} :weight 1 :flags 0\n")
                f.write(f"membrane node.{i} :state active :flux 1 :gate 1 :phase 1\n")
        elif name == "fanout":
            for i in range(n):
                f.write(f"triple hub.root -> links -> leaf.{i} :layer 1 :plane 1 :anchor {2000+i} :weight 1 :flags 0\n")
            f.write("membrane hub.root :state active :flux 1 :gate 1 :phase 1\n")
        elif name == "dense":
            limit = min(250, n)
            for i in range(limit):
                for j in range(limit):
                    if i != j:
                        f.write(f"triple dense.{i} -> links -> dense.{j} :layer 1 :plane 1 :anchor {3000 + i*limit + j} :weight 1 :flags 0\n")
        elif name == "mixed":
            for i in range(n):
                f.write(f"triple actor.{i} -> owns -> item.{i} :layer 1 :plane 1 :anchor {4000+i} :weight 2 :flags 1\n")
                f.write(f"triple actor.{i} -> visits -> place.{i%71} :layer 1 :plane 1 :anchor {8000+i} :weight 1 :flags 0\n")
                if i % 3 == 0:
                    f.write(f"membrane actor.{i} :state active :flux 2 :gate 1 :phase 1\n")
        elif name == "duplicate_stress":
            for i in range(n):
                line = f"triple dup.{i%97} -> links -> dup.{(i+1)%97} :layer 1 :plane 1 :anchor {9000 + (i%97)} :weight 1 :flags 0\n"
                f.write(line)
                f.write(line)
            for i in range(97):
                f.write(f"membrane dup.{i} :state active :flux 1 :gate 1 :phase 1\n")
                f.write(f"membrane dup.{i} :state active :flux 1 :gate 1 :phase 1\n")
        elif name == "pathological":
            for i in range(n):
                f.write(f"triple hot.root -> fanout -> leaf.{i} :layer 1 :plane 1 :anchor {100000+i} :weight 1 :flags 0\n")
                f.write(f"triple leaf.{i} -> returns -> hot.root :layer 1 :plane 1 :anchor {200000+i} :weight 1 :flags 0\n")
                if i % 5 == 0:
                    f.write(f"membrane leaf.{i} :state active :flux 1 :gate 1 :phase 1\n")
        elif name == "membrane_dense":
            for i in range(n):
                f.write(f"membrane state.{i} :state active :flux {1 + (i % 7)} :gate 1 :phase {(i % 5) + 1}\n")
                if i < n - 1:
                    f.write(f"triple state.{i} -> links -> state.{i+1} :layer 1 :plane 1 :anchor {300000+i} :weight 1 :flags 0\n")

    tasks.append({
        "name": name,
        "corpus_path": str(p),
        "queries": [
            "find symbol node.0",
            "find rel links",
            "neighbors node.0",
            "anchors min=1000 max=999999",
            "states target=node.0",
        ],
    })

(root / "neutral_task_spec.json").write_text(json.dumps(tasks, indent=2) + "\n", encoding="utf-8")
print(root)
