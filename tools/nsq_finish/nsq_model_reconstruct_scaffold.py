#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def sample_1_of_699(path: Path) -> dict:
    h = hashlib.sha256()
    n = 0
    size = path.stat().st_size
    with path.open("rb") as f:
        for pos in range(0, size, 699):
            f.seek(pos)
            b = f.read(1)
            if not b:
                break
            h.update(pos.to_bytes(8, "little", signed=False))
            h.update(b)
            n += 1
    return {
        "byte_count": size,
        "sample_stride": 699,
        "sample_count": n,
        "sample_sha256": h.hexdigest(),
    }


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--manifest", default="state/nsq/model_reconstruction_manifest.json")
    ap.add_argument("--root", default=".")
    ap.add_argument("--write-template", action="store_true")
    args = ap.parse_args()

    root = Path(args.root).resolve()
    manifest = root / args.manifest

    if args.write_template or not manifest.exists():
        manifest.parent.mkdir(parents=True, exist_ok=True)
        template = {
            "canon": "Exact identity is full byte_count plus full sha256 plus full blake3 where available. Sparse one-of-699 sampling is a route/sanity prefilter, not a proof by itself.",
            "files": [
                {
                    "path": "assets/braxon_core/model_config/config.json",
                    "byte_count": None,
                    "sha256": None,
                    "blake3": None,
                    "source": "git/source target only",
                    "role": "small config target"
                }
            ]
        }
        manifest.write_text(json.dumps(template, indent=2))
        print(json.dumps({"written_template": str(manifest)}, indent=2))
        return

    data = json.loads(manifest.read_text())
    results = []
    for item in data.get("files", []):
        rel = item.get("path")
        if not rel:
            continue
        p = root / rel
        r = {"path": rel, "exists": p.exists()}
        if p.exists() and p.is_file():
            size = p.stat().st_size
            full_sha = sha256_file(p)
            r["byte_count"] = size
            r["sha256"] = full_sha
            r["sparse_1_of_699"] = sample_1_of_699(p)

            expected_size = item.get("byte_count")
            expected_sha = item.get("sha256")
            r["byte_count_match"] = (expected_size is None or expected_size == size)
            r["sha256_match"] = (expected_sha is None or expected_sha == full_sha)
            r["exact_sha256_identity"] = bool(expected_size == size and expected_sha == full_sha)
        results.append(r)

    print(json.dumps({
        "manifest": str(manifest),
        "root": str(root),
        "results": results,
        "canon_warning": "A cryptographic hash verifies exact identity only after the candidate bytes exist. It does not reconstruct arbitrary unknown model bytes from sparse samples unless there is a deterministic generator, parity/erasure coding, or complete source manifest."
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
