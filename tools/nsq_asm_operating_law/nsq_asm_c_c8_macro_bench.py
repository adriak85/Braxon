#!/usr/bin/env python3
import argparse
import hashlib
import json
import os
import re
import time
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

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
    "state/nsq/metadata_law/impact/",
    "state/nsq/metadata_law/snapshots/",
    "state/nsq/stamps/libraries/",
    "state/nsq/stamps/registry/",
    "state/nsq/stamps/indices/",
    "state/nsq/asm_c_c8_macro_benchmark/",
)

NSQ_PRIORITY_PREFIXES = (
    "apps/nsq/",
    "config/nsq/",
    "docs/nsq/",
    "specs/nsq/",
    "crates/nsq-",
    "tools/nsq_",
    "state/nsq/",
    "specs/Braxon/",
    "docs/Braxon/",
    "state/braxon/",
)

SURFACES = ("asm", "c", "c8_asm", "nsq")

SEED_MACROS = {
    "SOURCE_HASH": ["sha256", "source hash", "source_sha256"],
    "LINEAGE_STAMP": ["lineage", "source lineage", "lineage stamp"],
    "AUTHORITY_CHECK": ["authority", "authority check", "source truth"],
    "ASM_RECODE_OP": ["asm operating", "asm recode", "asm_operating"],
    "C8_PACK": ["c8", "base 8", "lever"],
    "MORAL_GUARD": ["moral invariant", "guard", "protected moral"],
    "METADATA_IMPACT": ["metadata impact", "impact report", "alteration creates impact"],
    "REVERSE_DEPENDENCY": ["reverse dependency", "is_required_by"],
    "ROUND_TRIP": ["round trip", "round_trip"],
    "BINARY_BOUNDARY": ["binary boundary", "binary translation", "downstream output"],
    "NSQ_PASS": ["nsq pass", "NSQ_PASS", "nsq authority"],
    "GENERATED_NOT_SOURCE": ["generated reports are not source", "generated output not source", "not source authority"],
    "READ_EVERYTHING": ["read everything", "read_everything"],
    "CHECK_SURROUNDINGS": ["check surroundings", "context"],
    "IDENTIFY_ALL_PERCEIVABLE": ["identify all aspects", "all perceivable"],
    "UNDERSTAND_ACTION": ["understand action", "viewpoints"],
    "BEST_DECISION": ["best decision", "make_best_decision"],
    "BEST_BEING": ["best of being", "very best of being"],
}

WORD_RE = re.compile(r"[A-Za-z0-9_]+")

def now():
    return datetime.now(timezone.utc).isoformat()

def h_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()

def h_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8", errors="replace")).hexdigest()

def rel_to_root(root: Path, path: Path) -> str:
    return str(path.relative_to(root)).replace("\\", "/")

def skip_generated(rel: str) -> bool:
    return any(rel.startswith(prefix) for prefix in GENERATED_SKIP_PREFIXES)

def in_priority_scope(rel: str) -> bool:
    return any(rel.startswith(prefix) for prefix in NSQ_PRIORITY_PREFIXES)

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
    return b"\x00" not in chunk

def iter_files(root: Path, max_bytes: int):
    priority = []
    support = []

    for dirpath, dirnames, filenames in os.walk(root):
        dpath = Path(dirpath)
        rel_dir = rel_to_root(root, dpath) if dpath != root else ""
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        if rel_dir and skip_generated(rel_dir + "/"):
            dirnames[:] = []
            continue

        for fn in filenames:
            path = dpath / fn
            rel = rel_to_root(root, path)
            if skip_generated(rel):
                continue
            if not is_text_file(path, max_bytes):
                continue
            if in_priority_scope(rel):
                priority.append((rel, path))
            else:
                support.append((rel, path))

    return priority + support

def load_records(root: Path, limit: int, max_bytes: int):
    records = []
    for rel, path in iter_files(root, max_bytes):
        try:
            data = path.read_bytes()
            text = data.decode("utf-8", errors="replace")
        except OSError:
            continue

        source_hash = h_bytes(data)
        for line_no, line in enumerate(text.splitlines(), 1):
            stripped = line.strip()
            if not stripped:
                continue
            records.append({
                "path": rel,
                "line": line_no,
                "text": stripped[:800],
                "source_hash": source_hash,
                "surface": path.suffix.lower().lstrip(".") or "none",
            })
            if len(records) >= limit:
                return records
    return records

def normalize_words(text: str):
    return [w.lower() for w in WORD_RE.findall(text) if len(w) >= 3]

def ngrams(words, min_n=2, max_n=5):
    for n in range(min_n, max_n + 1):
        if len(words) < n:
            continue
        for i in range(0, len(words) - n + 1):
            yield " ".join(words[i:i+n])

def seed_macro_hits(text: str):
    low = text.lower()
    hits = []
    for name, terms in SEED_MACROS.items():
        for term in terms:
            if term.lower() in low:
                hits.append(name)
                break
    return hits

def discover_candidates(records, max_candidates=300):
    counts = Counter()
    examples = {}

    for rec in records:
        text = rec["text"]

        for name in seed_macro_hits(text):
            counts[name] += 1
            examples.setdefault(name, {
                "kind": "seed",
                "example_path": rec["path"],
                "example_line": rec["line"],
                "example_text": text[:200],
            })

        words = normalize_words(text)
        for gram in ngrams(words):
            if len(gram) < 8:
                continue
            key = "PHRASE_" + h_text(gram)[:12]
            counts[key] += 1
            examples.setdefault(key, {
                "kind": "phrase",
                "phrase": gram,
                "example_path": rec["path"],
                "example_line": rec["line"],
                "example_text": text[:200],
            })

    promoted = []
    for key, count in counts.most_common(max_candidates):
        if count > 3:
            row = examples.get(key, {})
            row.update({
                "macro": key,
                "use_count": count,
                "digest": h_text(key + ":" + str(row))[:16],
            })
            promoted.append(row)

    return promoted, counts

def compile_macro_table(seed_macros, promoted):
    table = {}

    for name, terms in seed_macros.items():
        table[name] = {
            "macro": name,
            "kind": "seed",
            "terms": terms,
            "digest": h_text(name)[:16],
            "use_count": 0,
        }

    for row in promoted:
        table[row["macro"]] = row

    return table

def record_macro_hits(rec, macro_table):
    text = rec["text"].lower()
    words = normalize_words(rec["text"])
    grams = set(ngrams(words))

    hits = []

    for name, row in macro_table.items():
        if row.get("kind") == "seed":
            if any(term.lower() in text for term in row.get("terms", [])):
                hits.append(name)
        elif row.get("kind") == "phrase":
            phrase = row.get("phrase")
            if phrase and phrase in grams:
                hits.append(name)

    return hits

def mix64(x):
    x &= 0xFFFFFFFFFFFFFFFF
    x ^= (x >> 33)
    x = (x * 0xff51afd7ed558ccd) & 0xFFFFFFFFFFFFFFFF
    x ^= (x >> 33)
    x = (x * 0xc4ceb9fe1a85ec53) & 0xFFFFFFFFFFFFFFFF
    x ^= (x >> 33)
    return x & 0xFFFFFFFFFFFFFFFF

def run_surface(surface, records, macro_table, round_no, timeout_seconds):
    tracking_enabled = round_no in (1, 2)
    tracer_collection_enabled = round_no in (1, 2)
    production_like = round_no == 3

    started = time.perf_counter()
    deadline = started + timeout_seconds

    macro_use = Counter() if tracking_enabled else None
    digest_mix = 0
    work_units = 0
    macro_hits_total = 0 if tracking_enabled else None
    lineage_checks = 0
    guard_checks = 0
    recode_checks = 0
    processed = 0
    timed_out = False

    for idx, rec in enumerate(records):
        if idx % 256 == 0 and time.perf_counter() >= deadline:
            timed_out = True
            break

        base = int(h_text(f"{surface}|{round_no}|{rec['path']}|{rec['line']}|{rec['text']}")[:16], 16)

        hits = record_macro_hits(rec, macro_table)

        if tracking_enabled:
            macro_hits_total += len(hits)
            for hit in hits:
                macro_use[hit] += 1

        byte_len = len(rec["text"].encode("utf-8", errors="replace"))
        word_count = max(1, len(normalize_words(rec["text"])))

        if surface == "c":
            local_work = byte_len + word_count * 7 + 24
            lineage_bonus = 0
            guard_bonus = 0
            recode_bonus = 0

        elif surface == "asm":
            macro_discount = min(byte_len // 2, len(hits) * 17)
            local_work = max(24, byte_len + word_count * 5 - macro_discount + 16)
            lineage_bonus = len(hits)
            guard_bonus = 0
            recode_bonus = 1

        elif surface == "c8_asm":
            c8_pack = max(1, (byte_len + 7) // 8)
            macro_discount = min(byte_len, len(hits) * 23)
            local_work = max(16, c8_pack * 5 + word_count * 4 - macro_discount + 32)
            lineage_bonus = len(hits)
            guard_bonus = len(hits) // 2
            recode_bonus = 1

        elif surface == "nsq":
            macro_discount = min(byte_len // 2, len(hits) * 19)
            local_work = max(32, byte_len + word_count * 6 - macro_discount + 48)
            lineage_bonus = 1 + len(hits)
            guard_bonus = 1
            recode_bonus = 1

        else:
            local_work = byte_len + word_count
            lineage_bonus = 0
            guard_bonus = 0
            recode_bonus = 0

        lineage_checks += lineage_bonus
        guard_checks += guard_bonus
        recode_checks += recode_bonus
        work_units += local_work + lineage_bonus * 3 + guard_bonus * 5 + recode_bonus * 7
        digest_mix ^= mix64(base ^ (len(hits) << 32) ^ idx)
        processed += 1

    elapsed = time.perf_counter() - started

    row = {
        "surface": surface,
        "round": round_no,
        "role": "production_like_scored_run" if production_like else "instrumented_discovery",
        "records_available": len(records),
        "records_processed": processed,
        "timed_out": timed_out,
        "timeout_seconds": timeout_seconds,
        "elapsed_seconds": round(elapsed, 6),
        "records_per_second": round(processed / elapsed, 3) if elapsed else None,
        "work_units": int(work_units),
        "work_units_per_second": round(work_units / elapsed, 3) if elapsed else None,
        "lineage_checks": lineage_checks,
        "guard_checks": guard_checks,
        "recode_checks": recode_checks,
        "digest_mix": f"{digest_mix:016x}",
        "tracking_enabled": tracking_enabled,
        "tracer_collection_enabled": tracer_collection_enabled,
        "production_like": production_like,
    }

    if tracking_enabled:
        row["macro_hits_total"] = macro_hits_total
        row["unique_macros_used"] = len(macro_use)
        row["macro_use_top"] = macro_use.most_common(40)
    else:
        row["macro_hits_total"] = None
        row["unique_macros_used"] = None
        row["macro_use_top"] = []
        row["production_note"] = "Macro tracking and tracer collection disabled for scored production-like runtime."

    return row

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=os.environ.get("BRAXON_ROOT", str(Path.home() / "Braxon")))
    ap.add_argument("--out", required=True)
    ap.add_argument("--records", type=int, default=int(os.environ.get("NSQ_ASM_C8_RECORDS", "65536")))
    ap.add_argument("--max-bytes", type=int, default=int(os.environ.get("NSQ_ASM_C8_MAX_FILE_BYTES", "1048576")))
    ap.add_argument("--timeout-seconds", type=float, default=float(os.environ.get("NSQ_SURFACE_TIMEOUT_SECONDS", "10")))
    args = ap.parse_args()

    root = Path(args.root).resolve()
    out = Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=True)

    records = load_records(root, args.records, args.max_bytes)

    promoted = []
    rounds = []
    macro_history = []

    for round_no in (1, 2, 3):
        macro_table = compile_macro_table(SEED_MACROS, promoted)
        surface_results = []

        for surface in SURFACES:
            surface_results.append(
                run_surface(
                    surface=surface,
                    records=records,
                    macro_table=macro_table,
                    round_no=round_no,
                    timeout_seconds=args.timeout_seconds,
                )
            )

        round_summary = {
            "round": round_no,
            "role": "production_like_scored_run" if round_no == 3 else "instrumented_macro_discovery",
            "scored": round_no == 3,
            "tracking_enabled": round_no in (1, 2),
            "tracer_collection_enabled": round_no in (1, 2),
            "production_like": round_no == 3,
            "timeout_seconds_per_surface": args.timeout_seconds,
            "macro_table_size": len(macro_table),
            "surface_results": surface_results,
        }
        rounds.append(round_summary)

        if round_no in (1, 2):
            discovered, counts = discover_candidates(records)
            existing = {row["macro"] for row in promoted}
            added = [row for row in discovered if row["macro"] not in existing]
            promoted.extend(added)
            macro_history.append({
                "round": round_no,
                "tracking_enabled": True,
                "tracer_collection_enabled": True,
                "discovered_count": len(discovered),
                "new_promoted_count": len(added),
                "promoted_total_after_round": len(promoted),
                "top_new": added[:40],
            })

    scored = rounds[-1]["surface_results"]
    ranked_by_work_rate = sorted(
        scored,
        key=lambda r: (r["work_units_per_second"] or 0),
        reverse=True
    )

    report = {
        "schema": "nsq.asm_c_c8_macro_benchmark.report.v2",
        "generated_at": now(),
        "kiwi_run": True,
        "simulation_only": True,
        "BRAXON_live_claim": False,
        "rounds": 3,
        "scored_round": 3,
        "records_loaded": len(records),
        "surfaces": list(SURFACES),
        "timeout_seconds_per_surface": args.timeout_seconds,
        "macro_promotion_rule": "promote if used more than 3 times during discovery rounds",
        "seed_macro_count": len(SEED_MACROS),
        "promoted_macro_count": len(promoted),
        "round_1_tracking_enabled": True,
        "round_2_tracking_enabled": True,
        "round_3_tracking_enabled": False,
        "round_3_tracer_collection_enabled": False,
        "round_3_production_like": True,
        "macro_history": macro_history,
        "round_results": rounds,
        "scored_ranked_by_work_units_per_second": ranked_by_work_rate,
        "best_scored_surface_by_work_rate": ranked_by_work_rate[0]["surface"] if ranked_by_work_rate else None,
        "honesty": [
            "This is a deterministic local benchmark harness.",
            "It compares internal work units and local elapsed runtime.",
            "It does not claim Braxon is live or complete.",
            "ASM is treated as NSQ operating form, not NSQ source identity.",
            "Binary remains downstream Braxon translation output.",
            "Round 3 disables macro tracking and tracer collection to approximate production runtime."
        ],
    }

    json_path = out / "asm_c_c8_nsq_macro_benchmark_report.json"
    txt_path = out / "asm_c_c8_nsq_macro_benchmark_report.txt"

    json_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    with txt_path.open("w", encoding="utf-8") as f:
        f.write("== ASM vs C vs C8+ASM vs NSQ macro benchmark ==\n")
        f.write(f"kiwi_run={report['kiwi_run']}\n")
        f.write(f"simulation_only={report['simulation_only']}\n")
        f.write(f"BRAXON_live_claim={report['BRAXON_live_claim']}\n")
        f.write(f"records_loaded={report['records_loaded']}\n")
        f.write(f"rounds={report['rounds']}\n")
        f.write(f"scored_round={report['scored_round']}\n")
        f.write(f"timeout_seconds_per_surface={report['timeout_seconds_per_surface']}\n")
        f.write(f"round_1_tracking_enabled={report['round_1_tracking_enabled']}\n")
        f.write(f"round_2_tracking_enabled={report['round_2_tracking_enabled']}\n")
        f.write(f"round_3_tracking_enabled={report['round_3_tracking_enabled']}\n")
        f.write(f"round_3_tracer_collection_enabled={report['round_3_tracer_collection_enabled']}\n")
        f.write(f"round_3_production_like={report['round_3_production_like']}\n")
        f.write(f"seed_macro_count={report['seed_macro_count']}\n")
        f.write(f"promoted_macro_count={report['promoted_macro_count']}\n")
        f.write(f"best_scored_surface_by_work_rate={report['best_scored_surface_by_work_rate']}\n\n")

        f.write("== macro discovery rounds only ==\n")
        for row in macro_history:
            f.write(
                f"round={row['round']} tracking={row['tracking_enabled']} "
                f"tracers={row['tracer_collection_enabled']} "
                f"discovered={row['discovered_count']} "
                f"new_promoted={row['new_promoted_count']} "
                f"promoted_total={row['promoted_total_after_round']}\n"
            )

        f.write("\n== scored production-like round results ==\n")
        for row in ranked_by_work_rate:
            f.write(
                f"{row['surface']}: processed={row['records_processed']}/{row['records_available']} "
                f"timed_out={row['timed_out']} "
                f"elapsed={row['elapsed_seconds']} "
                f"records_per_second={row['records_per_second']} "
                f"work_units={row['work_units']} "
                f"work_units_per_second={row['work_units_per_second']} "
                f"tracking={row['tracking_enabled']} "
                f"tracers={row['tracer_collection_enabled']} "
                f"production_like={row['production_like']} "
                f"digest={row['digest_mix']}\n"
            )

        f.write("\n== top promoted macros from discovery rounds ==\n")
        for row in promoted[:80]:
            f.write(json.dumps(row, sort_keys=True) + "\n")

    print(json.dumps({
        "ok": True,
        "json": str(json_path),
        "txt": str(txt_path),
        "records_loaded": len(records),
        "timeout_seconds_per_surface": args.timeout_seconds,
        "seed_macro_count": len(SEED_MACROS),
        "promoted_macro_count": len(promoted),
        "round_3_tracking_enabled": False,
        "round_3_tracer_collection_enabled": False,
        "round_3_production_like": True,
        "best_scored_surface_by_work_rate": report["best_scored_surface_by_work_rate"],
    }, indent=2, sort_keys=True))

if __name__ == "__main__":
    main()
