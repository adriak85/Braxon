#!/usr/bin/env python3
import argparse
import hashlib
import json
import os
import re
import sys
import tarfile
import time
from pathlib import Path
from datetime import datetime, timezone

TEXT_EXTS = {
    ".rs", ".c", ".h", ".hpp", ".cpp", ".py", ".sh", ".bash", ".zsh",
    ".toml", ".json", ".md", ".txt", ".yaml", ".yml", ".xml", ".html",
    ".css", ".js", ".ts", ".tsx", ".jsx", ".sql", ".nsq", ".s", ".asm",
    ".jsonl", ".tsv", ".csv"
}

SKIP_DIRS = {
    ".git", "target", ".cargo", ".rustup", "node_modules",
    ".gradle", "build", "dist", "__pycache__"
}

GENERATED_SKIP_PREFIXES = (
    "state/nsq/stamps/libraries/",
    "state/nsq/stamps/registry/",
    "state/nsq/stamps/indices/",
)

HARM_TERMS = [
    "ultrawide banding",
    "pointer_stub",
    "pointer stubs",
    "catalog_complete_pointer",
    "model.safetensors",
    "external_tool_host",
    "raw_model",
]

FLATTEN_TERMS = [
    "nsq is a wrapper",
    "nsq is a layer",
    "nsq is an overlay",
    "nsq lowers to binary",
    "binary is the underlying truth",
    "byte language",
    "u8 language",
    "u16 language",
    "u32 language",
    "u64 language",
    "u128 language",
]

CANONICAL_NEGATION_WORDS = [
    "not", "never", "forbidden", "rejected", "reject", "wrong",
    "inactive", "replaces", "replaced", "must not", "no "
]

def now():
    return datetime.now(timezone.utc).isoformat()

def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()

def slug(s: str) -> str:
    s = s.replace("\\", "/")
    s = re.sub(r"[^A-Za-z0-9._/-]+", "_", s)
    s = s.strip("_/.")
    s = s.replace("/", "__")
    return s[:180] or "root"

def asm_symbol(s: str) -> str:
    s = re.sub(r"[^A-Za-z0-9_]+", "_", s)
    if not s or s[0].isdigit():
        s = "nsq_" + s
    return s[:220]

def rel_to_root(root: Path, path: Path) -> str:
    return str(path.relative_to(root)).replace("\\", "/")

def is_generated(rel: str) -> bool:
    return any(rel.startswith(prefix) for prefix in GENERATED_SKIP_PREFIXES)

def is_text_file(path: Path, max_bytes: int) -> bool:
    try:
        st = path.stat()
    except OSError:
        return False
    if st.st_size > max_bytes:
        return False
    if path.suffix.lower() not in TEXT_EXTS:
        return False
    try:
        chunk = path.read_bytes()[:4096]
    except OSError:
        return False
    if b"\x00" in chunk:
        return False
    return True

def library_for(rel: str) -> str:
    parts = rel.split("/")
    if len(parts) >= 2 and parts[0] == "crates":
        return f"crates__{parts[1]}"
    if rel.startswith("apps/nsq/"):
        return "apps__nsq"
    if rel.startswith("config/nsq/"):
        return "config__nsq"
    if rel.startswith("specs/nsq/"):
        return "specs__nsq"
    if rel.startswith("docs/nsq/"):
        return "docs__nsq"
    if rel.startswith("state/nsq/"):
        return "state__nsq"
    if rel.startswith("tools/nsq_"):
        return "tools__" + slug(parts[1] if len(parts) > 1 else "nsq")
    return "repo__" + slug(parts[0] if parts else "root")

def suspicious_lines(rel: str, text: str):
    hits = []
    for idx, line in enumerate(text.splitlines(), 1):
        low = line.lower()
        for term in HARM_TERMS:
            if term in low:
                hits.append({
                    "path": rel,
                    "line": idx,
                    "term": term,
                    "class": "harm_or_non_nsq_behavior",
                    "text": line[:240],
                })
        for term in FLATTEN_TERMS:
            if term in low and not any(n in low for n in CANONICAL_NEGATION_WORDS):
                hits.append({
                    "path": rel,
                    "line": idx,
                    "term": term,
                    "class": "flattening_drift",
                    "text": line[:240],
                })
    return hits

def asm_macro_text(stamp_id: str, rel: str, library: str, digest: str, size: int) -> str:
    sym = asm_symbol(stamp_id)
    return f""".section .note.nsq.asm_macros,"a"
.global {sym}
{sym}:
    .ascii "NSQ_ASM_MACRO_STAMP\\\\n"
    .ascii "stamp_id={stamp_id}\\\\n"
    .ascii "library={library}\\\\n"
    .ascii "source={rel}\\\\n"
    .ascii "source_sha256={digest}\\\\n"
    .ascii "source_bytes={size}\\\\n"
    .ascii "nsq_lowest_base_language=true\\\\n"
    .ascii "lever_is_one_switch=true\\\\n"
    .ascii "lever_is_one_eighth_of_nsq_bit=true\\\\n"
    .ascii "hertz_positions_lever=true\\\\n"
    .ascii "other_surfaces_translation_inputs_only=true\\\\n"
"""

def write_alphabet_unicode(root: Path, stamp_root: Path):
    base = stamp_root / "libraries" / "nsq__alphabet_unicode_index"
    asm_dir = base / "asm"
    reg_dir = base / "registry"
    idx_dir = base / "index"
    asm_dir.mkdir(parents=True, exist_ok=True)
    reg_dir.mkdir(parents=True, exist_ok=True)
    idx_dir.mkdir(parents=True, exist_ok=True)

    mappings = []
    mappings.append({ "slot": "+1000", "charge": "+", "body": 1000, "value": "0", "dialect": "numeric" })
    mappings.append({ "slot": "+1001", "charge": "+", "body": 1001, "value": "1", "dialect": "numeric" })
    mappings.append({ "slot": "-1000", "charge": "-", "body": 1000, "value": "space", "dialect": "punctuation" })
    for i in range(1, 100):
        mappings.append({ "slot": f"-{1000+i}", "charge": "-", "body": 1000+i, "value": f".{i:02d}", "dialect": "decimal_fraction" })
    mappings.append({ "slot": "-1100", "charge": "-", "body": 1100, "value": "newline", "dialect": "control" })
    for i in range(26):
        body = 1101 + i
        upper = chr(ord("A") + i)
        lower = chr(ord("a") + i)
        mappings.append({ "slot": f"+{body}", "charge": "+", "body": body, "value": upper, "dialect": "alphabetic" })
        mappings.append({ "slot": f"-{body}", "charge": "-", "body": body, "value": lower, "dialect": "alphabetic" })

    unicode_ranges = [
        ("basic_latin", "U+0000", "U+007F"),
        ("latin_1_supplement", "U+0080", "U+00FF"),
        ("latin_extended", "U+0100", "U+024F"),
        ("greek_coptic", "U+0370", "U+03FF"),
        ("cyrillic", "U+0400", "U+04FF"),
        ("hebrew", "U+0590", "U+05FF"),
        ("arabic", "U+0600", "U+06FF"),
        ("devanagari", "U+0900", "U+097F"),
        ("cjk_unified", "U+4E00", "U+9FFF"),
        ("private_use", "U+E000", "U+F8FF"),
        ("supplementary_planes", "U+10000", "U+10FFFF"),
    ]

    idx_jsonl = idx_dir / "alphabet_unicode_index.jsonl"
    with idx_jsonl.open("w", encoding="utf-8") as f:
        for row in mappings:
            f.write(json.dumps(row, sort_keys=True) + "\n")
        for name, start, end in unicode_ranges:
            f.write(json.dumps({
                "dialect": "unicode_range",
                "range_name": name,
                "start": start,
                "end": end,
                "rule": "unicode symbol space resolves through NSQ substrate; range row is index seed, not separate language",
            }, sort_keys=True) + "\n")

    nsq_index = idx_dir / "alphabet_unicode_index.nsq"
    nsq_index.write_text(
        "NSQ_INDEX alphabet_unicode_index\n"
        "basis = alphabet_and_unicode_range_seed\n"
        "charge_matters = true\n"
        "dialect_selected_by_first_lever = true\n"
        "leading_anchor_sets_charge = true\n"
        "body_index_chooses_slot = true\n"
        "unicode_symbol_space = stamp_space_middle_band\n"
        "unicode_full_expansion_policy = generate_on_demand_from_range_seed\n",
        encoding="utf-8",
    )

    asm = asm_dir / "alphabet_unicode_index.s"
    digest = sha256_bytes(idx_jsonl.read_bytes())
    asm.write_text(asm_macro_text(
        "nsq.asm.alphabet_unicode_index." + digest[:16],
        "state/nsq/stamps/libraries/nsq__alphabet_unicode_index/index/alphabet_unicode_index.jsonl",
        "nsq__alphabet_unicode_index",
        digest,
        idx_jsonl.stat().st_size,
    ), encoding="utf-8")

    meta = {
        "schema": "nsq.asm.alphabet_unicode_index.v1",
        "generated_at": now(),
        "library": "nsq__alphabet_unicode_index",
        "asm": str(asm.relative_to(root)),
        "index": str(idx_jsonl.relative_to(root)),
        "nsq_index": str(nsq_index.relative_to(root)),
        "entries": len(mappings) + len(unicode_ranges),
        "full_unicode_policy": "range_seed_plus_on_demand_expansion",
    }
    (reg_dir / "alphabet_unicode_index.meta.json").write_text(json.dumps(meta, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return meta

def iter_repo_files(root: Path, max_bytes: int):
    for dirpath, dirnames, filenames in os.walk(root):
        dpath = Path(dirpath)
        rel_dir = rel_to_root(root, dpath) if dpath != root else ""
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        if rel_dir.startswith("state/nsq/quarantine"):
            continue
        for fn in filenames:
            path = dpath / fn
            rel = rel_to_root(root, path)
            if is_generated(rel):
                continue
            if is_text_file(path, max_bytes):
                yield path, rel

def build(root: Path, round_no: int, max_bytes: int, limit: int):
    stamp_root = root / "state/nsq/stamps"
    registry_root = stamp_root / "registry"
    registry_root.mkdir(parents=True, exist_ok=True)

    alpha_meta = write_alphabet_unicode(root, stamp_root)

    translation_index = registry_root / f"translation_index_round{round_no}.jsonl"
    macro_registry = registry_root / f"asm_macro_registry_round{round_no}.jsonl"
    harm_report = root / "state/nsq/asm_macro_builder" / f"harm_report_round{round_no}.jsonl"
    harm_report.parent.mkdir(parents=True, exist_ok=True)

    count = 0
    suspicious_count = 0
    started = time.time()

    with translation_index.open("w", encoding="utf-8") as ti, macro_registry.open("w", encoding="utf-8") as mr, harm_report.open("w", encoding="utf-8") as hr:
        for path, rel in iter_repo_files(root, max_bytes=max_bytes):
            if limit and count >= limit:
                break
            try:
                data = path.read_bytes()
                text = data.decode("utf-8", errors="replace")
            except OSError:
                continue

            digest = sha256_bytes(data)
            library = library_for(rel)
            stamp_id = f"nsq.asm.{slug(library)}.{slug(rel)}.{digest[:16]}"
            lib_root = stamp_root / "libraries" / library
            asm_dir = lib_root / "asm" / "macros"
            meta_dir = lib_root / "metadata"
            asm_dir.mkdir(parents=True, exist_ok=True)
            meta_dir.mkdir(parents=True, exist_ok=True)

            asm_path = asm_dir / (slug(rel) + "." + digest[:16] + ".s")
            meta_path = meta_dir / (slug(rel) + "." + digest[:16] + ".json")

            asm_path.write_text(asm_macro_text(stamp_id, rel, library, digest, len(data)), encoding="utf-8")

            hits = suspicious_lines(rel, text)
            suspicious_count += len(hits)
            for hit in hits:
                hr.write(json.dumps(hit, sort_keys=True) + "\n")

            meta = {
                "schema": "nsq.asm.source_macro_stamp.v1",
                "generated_at": now(),
                "round": round_no,
                "stamp_id": stamp_id,
                "library": library,
                "source": rel,
                "source_sha256": digest,
                "source_bytes": len(data),
                "asm_macro": str(asm_path.relative_to(root)),
                "metadata": str(meta_path.relative_to(root)),
                "harm_hit_count": len(hits),
                "local_repo_direct_native_path": True,
                "external_recode_carrier_used": False,
                "other_language_surface_role": "translation_input_only",
            }
            meta_path.write_text(json.dumps(meta, indent=2, sort_keys=True) + "\n", encoding="utf-8")
            ti.write(json.dumps({
                "source": rel,
                "library": library,
                "stamp_id": stamp_id,
                "asm_macro": str(asm_path.relative_to(root)),
                "source_sha256": digest,
            }, sort_keys=True) + "\n")
            mr.write(json.dumps(meta, sort_keys=True) + "\n")
            count += 1

    elapsed = time.time() - started
    summary = {
        "schema": "nsq.asm_macro_builder.round_summary.v1",
        "generated_at": now(),
        "round": round_no,
        "scored": round_no == 2,
        "source_files_encoded": count,
        "harm_hits": suspicious_count,
        "alphabet_unicode_index": alpha_meta,
        "translation_index": str(translation_index.relative_to(root)),
        "macro_registry": str(macro_registry.relative_to(root)),
        "harm_report": str(harm_report.relative_to(root)),
        "elapsed_seconds": round(elapsed, 3),
        "score_rule": "only round 2 is scored",
    }
    summary_path = root / "state/nsq/asm_macro_builder" / f"round{round_no}_summary.json"
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(summary, indent=2, sort_keys=True))
    return summary

def write_smart_tool_policy(root: Path):
    p = root / "state/nsq/smart_tools/asm_macro_tool_usage.json"
    p.parent.mkdir(parents=True, exist_ok=True)
    obj = {
        "schema": "nsq.smart_tool_usage.asm_macro_first.v1",
        "generated_at": now(),
        "daemon_required": False,
        "asm_first": True,
        "local_repo_source_policy": "direct_native_repo_path_no_shim",
        "external_source_policy": "recode_carrier_before_stamp_save",
        "other_languages": "translation_and_recode_inputs_only",
        "benchmarks": {
            "rounds": 2,
            "round_1": "warmup_discovery_not_scored",
            "round_2": "truth_scored",
        },
        "stamp_directory_rule": "state/nsq/stamps/libraries/<matching_library>/asm/macros",
        "first_saved_indexes": [
            "alphabet",
            "unicode_range_index"
        ],
        "raw_body_transfer": "forbidden",
    }
    p.write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"smart_tool_policy={p}")
    return obj

def bundle(root: Path, out: Path):
    files = [
        root / "apps/nsq/asm_macro_spine.nsq",
        root / "config/nsq/asm_macro_builder.nsq",
        root / "state/nsq/smart_tools/asm_macro_tool_usage.json",
        root / "state/nsq/asm_macro_builder",
        root / "state/nsq/stamps/libraries",
        root / "state/nsq/stamps/registry",
        root / "state/nsq/stamps/indices",
    ]
    tar_path = out / "nsq_asm_macro_spine_bundle.tar.gz"
    with tarfile.open(tar_path, "w:gz") as tf:
        for p in files:
            if p.exists():
                tf.add(p, arcname=str(p.relative_to(root)))
    print(f"bundle={tar_path}")

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("command", choices=["build", "policy", "bundle"])
    ap.add_argument("--root", default=os.environ.get("BRAXON_ROOT", str(Path.home() / "Braxon")))
    ap.add_argument("--out", default=None)
    ap.add_argument("--round", type=int, default=2)
    ap.add_argument("--max-bytes", type=int, default=int(os.environ.get("NSQ_ASM_MAX_FILE_BYTES", "1048576")))
    ap.add_argument("--limit", type=int, default=int(os.environ.get("NSQ_ASM_SCAN_LIMIT", "0")))
    args = ap.parse_args()

    root = Path(args.root).resolve()
    out = Path(args.out).resolve() if args.out else root / "state/nsq/asm_macro_builder"
    out.mkdir(parents=True, exist_ok=True)

    if args.command == "policy":
        write_smart_tool_policy(root)
    elif args.command == "build":
        write_smart_tool_policy(root)
        build(root, args.round, args.max_bytes, args.limit)
    elif args.command == "bundle":
        bundle(root, out)

if __name__ == "__main__":
    main()
