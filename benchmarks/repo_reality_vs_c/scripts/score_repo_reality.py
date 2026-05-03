from pathlib import Path
import json
import statistics
import hashlib

root = Path.home() / "Braxon" / "benchmarks" / "repo_reality_vs_c"
truth = json.loads((root / "specs" / "ground_truth.json").read_text())

def as_set(xs):
    return set(xs or [])

truth_symbols = set(truth["symbols"])
truth_entry = set(truth["entrypoints"])
truth_find = set(truth["findings"])

truth_imports = set((x["from"], x["to"]) for x in truth["imports"])
truth_calls = set((x["from"], x["to"]) for x in truth["calls"])

def normalize_imports(raw):
    out = set()
    for x in raw or []:
        s = str(x)
        name = s.rsplit("/", 1)[-1].replace(".h", "")
        out.add(name)
    return out

def normalize_calls(raw):
    out = set()
    for x in raw or []:
        s = str(x)
        if "_handle" in s and "_validate" in s:
            left = s.split("_handle",1)[0].split()[-1] + "_handle"
            right = s.split("_validate",1)[0].split()[-1] + "_validate"
            out.add((left, right))
        for suffix in ("_flush(", "_handle(", "_validate("):
            if suffix in s:
                pass
    return out

def prf(pred, truth):
    if not pred and not truth:
        return {"p": 1.0, "r": 1.0, "f1": 1.0, "tp": 0, "fp": 0, "fn": 0}
    tp = len(pred & truth)
    fp = len(pred - truth)
    fn = len(truth - pred)
    p = tp / (tp + fp) if (tp + fp) else 0.0
    r = tp / (tp + fn) if (tp + fn) else 0.0
    f1 = 2*p*r/(p+r) if (p+r) else 0.0
    return {"p": p, "r": r, "f1": f1, "tp": tp, "fp": fp, "fn": fn}

def load_reports(system_dir):
    out = []
    for p in sorted(system_dir.glob("*.json")):
        try:
            out.append(json.loads(p.read_text()))
        except Exception:
            pass
    return out

def score_one(rep):
    symbols = as_set(rep.get("symbols"))
    entry = as_set(rep.get("entrypoints"))
    findings = as_set(rep.get("findings"))

    imports_raw = normalize_imports(rep.get("imports"))
    calls_raw = normalize_calls(rep.get("calls"))

    sym = prf(symbols, truth_symbols)
    ent = prf(entry, truth_entry)
    fin = prf(findings, truth_find)

    # imports/calls are harder because representations differ; still score weakly
    imp_recall = len(imports_raw & {b for _, b in truth_imports}) / len({b for _, b in truth_imports}) if truth_imports else 1.0
    call_presence = 1.0 if len(rep.get("calls") or []) > 0 else 0.0

    briefing = rep.get("briefing", {})
    elapsed_ms = rep.get("elapsed_ms")
    sec = elapsed_ms / 1000.0 if elapsed_ms else None
    readable_bytes = briefing.get("readable_output_bytes", 0)
    readable_lines = briefing.get("readable_output_lines", 0)
    structural_units = len(symbols) + len(rep.get("calls") or []) + len(rep.get("imports") or []) + len(entry) + len(findings)

    return {
        "system": rep.get("system"),
        "elapsed_ms": elapsed_ms,
        "symbol_f1": sym["f1"],
        "entry_f1": ent["f1"],
        "finding_f1": fin["f1"],
        "import_recall_weak": imp_recall,
        "call_presence": call_presence,
        "obligation_recall": statistics.mean([sym["r"], ent["r"], fin["r"], imp_recall, call_presence]),
        "obligation_precision": statistics.mean([sym["p"], ent["p"], fin["p"]]),
        "readable_information_per_second": (readable_bytes / sec) if sec else None,
        "readable_lines_per_second": (readable_lines / sec) if sec else None,
        "structural_units_per_second": (structural_units / sec) if sec else None,
        "deterministic_repeat_match": bool(rep.get("replay_hash")),
        "null_output": readable_bytes == 0,
        "raw_report": rep
    }

def reduce(system_name, reports):
    rows = [score_one(r) for r in reports]
    if not rows:
        return {"system": system_name, "rows": [], "summary": None}
    summary = {
        "system": system_name,
        "runs": len(rows),
        "obligation_recall_median": statistics.median(x["obligation_recall"] for x in rows),
        "obligation_precision_median": statistics.median(x["obligation_precision"] for x in rows),
        "readable_information_per_second_median": statistics.median(x["readable_information_per_second"] for x in rows if x["readable_information_per_second"] is not None),
        "readable_lines_per_second_median": statistics.median(x["readable_lines_per_second"] for x in rows if x["readable_lines_per_second"] is not None),
        "structural_units_per_second_median": statistics.median(x["structural_units_per_second"] for x in rows if x["structural_units_per_second"] is not None),
        "deterministic_rate": sum(1 for x in rows if x["deterministic_repeat_match"]) / len(rows),
        "null_output_rate": sum(1 for x in rows if x["null_output"]) / len(rows)
    }
    return {"system": system_name, "rows": rows, "summary": summary}

c_reports = load_reports(root / "results" / "c")
nsq_reports = load_reports(root / "results" / "nsq")

result = {
    "version": 1,
    "c": reduce("c", c_reports),
    "nsq": reduce("nsq", nsq_reports)
}
(root / "results" / "repo_reality_score.json").write_text(json.dumps(result, indent=2))
print(json.dumps(result, indent=2))
