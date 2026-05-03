#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
from typing import Dict, List, Tuple


def factor(n: int) -> Dict[int, int]:
    if n < 1:
        raise ValueError("factor input must be >= 1")
    out: Dict[int, int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            out[d] = out.get(d, 0) + 1
            n //= d
        d += 1 if d == 2 else 2
    if n > 1:
        out[n] = out.get(n, 0) + 1
    return out


def prime_path_object(n: int) -> dict:
    exponents = factor(n)
    paths = []
    vectors = []
    exponent_paths = {}

    for p, e in sorted(exponents.items()):
        exponent_paths[str(p)] = factor(e) if e > 1 else {}
        for j in range(1, e + 1):
            path_id = f"pi_{p}_{j}"
            vector_id = f"v_{p}_{j}"
            paths.append({
                "prime": p,
                "path_index": j,
                "path_id": path_id,
                "introduced_vector": vector_id,
            })
            vectors.append({
                "prime": p,
                "path_index": j,
                "vector_id": vector_id,
                "introduced_by": path_id,
            })

    return {
        "value": n,
        "prime_exponent_map": exponents,
        "exponent_factor_geometry": exponent_paths,
        "path_count": len(paths),
        "introduced_vector_count": len(vectors),
        "paths": paths,
        "introduced_vectors": vectors,
        "law_check": {
            "path_count_equals_vector_count": len(paths) == len(vectors),
            "path_count_equals_sum_of_exponents": len(paths) == sum(exponents.values()),
        },
    }


def group_counts(width: int, groups: int | None, orbit_payload: int | None) -> dict:
    if groups is None and orbit_payload is None:
        groups = 17
    if width <= 0:
        raise ValueError("width must be > 0")

    result = {"width": width}

    if groups is not None:
        orbit = groups * width
        leader = groups
        total = orbit + leader
        result["groups_given"] = groups
        result["orbit_payload_from_groups"] = orbit
        result["leader_bits_from_groups"] = leader
        result["total_governed_bits_from_groups"] = total

    if orbit_payload is not None:
        groups_needed = math.ceil(orbit_payload / width)
        leader_needed = groups_needed
        logical_total = orbit_payload + leader_needed
        padded_orbit = groups_needed * width
        padded_total = padded_orbit + leader_needed
        result["orbit_payload_given"] = orbit_payload
        result["groups_needed_for_payload"] = groups_needed
        result["leader_bits_needed_for_payload"] = leader_needed
        result["logical_total_for_payload"] = logical_total
        result["padded_orbit_slots_for_payload"] = padded_orbit
        result["padded_total_slots_for_payload"] = padded_total
        result["remainder"] = orbit_payload % width

    return result


def beyond_boundary(candidates: Path, boundary: int) -> dict:
    vals: List[int] = []
    if candidates.exists():
        for line in candidates.read_text(errors="ignore").splitlines():
            s = line.strip().split("#", 1)[0].strip()
            if not s:
                continue
            try:
                vals.append(int(s))
            except ValueError:
                pass

    beyond = [v for v in vals if v > boundary]
    return {
        "candidate_file": str(candidates),
        "boundary": boundary,
        "total_candidates": len(vals),
        "beyond_boundary_path_candidates": len(beyond),
        "min_beyond": min(beyond) if beyond else None,
        "max_beyond": max(beyond) if beyond else None,
        "note": "This counts path candidates only. Exponent-prime and Mersenne-prime proof require separate primality proof stages.",
    }


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def sparse_sample_digest(path: Path, stride: int = 699) -> dict:
    size = path.stat().st_size
    h = hashlib.sha256()
    count = 0
    with path.open("rb") as f:
        pos = 0
        while pos < size:
            f.seek(pos)
            b = f.read(1)
            if not b:
                break
            h.update(pos.to_bytes(8, "little", signed=False))
            h.update(b)
            count += 1
            pos += stride
    return {
        "file": str(path),
        "byte_count": size,
        "stride": stride,
        "sample_count": count,
        "sample_sha256": h.hexdigest(),
        "warning": "Sparse sample digest is only a mismatch detector/prefilter. Exact identity requires full byte count plus full-file cryptographic hashes after reconstruction.",
    }


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--number", type=int, default=12)
    ap.add_argument("--width", type=int, default=33)
    ap.add_argument("--groups", type=int, default=17)
    ap.add_argument("--orbit-payload", type=int, default=699)
    ap.add_argument("--boundary", type=int, default=int(os.environ.get("NSQ_RECORD_EXPONENT", "136279841")))
    ap.add_argument("--candidates", default="state/nsq/prime_path_candidates.txt")
    ap.add_argument("--hash-file", default="")
    ap.add_argument("--out", default="")
    args = ap.parse_args()

    report = {
        "prime_path_object": prime_path_object(args.number),
        "group_counts": group_counts(args.width, args.groups, args.orbit_payload),
        "beyond_boundary": beyond_boundary(Path(args.candidates), args.boundary),
    }

    if args.hash_file:
        p = Path(args.hash_file)
        if p.exists() and p.is_file():
            report["hash_file"] = {
                "file": str(p),
                "byte_count": p.stat().st_size,
                "sha256": sha256_file(p),
                "sparse_699": sparse_sample_digest(p, 699),
            }
        else:
            report["hash_file"] = {"file": str(p), "error": "not found"}

    text = json.dumps(report, indent=2, sort_keys=True)
    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(text)
    print(text)


if __name__ == "__main__":
    main()
