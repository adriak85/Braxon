from pathlib import Path
import json
import random
import hashlib

root = Path.home() / "Braxon" / "benchmarks" / "repo_reality_vs_c"
corpus = root / "corpus" / "project"
if corpus.exists():
    for p in sorted(corpus.rglob("*"), reverse=True):
        if p.is_file():
            p.unlink()
        elif p.is_dir():
            try:
                p.rmdir()
            except OSError:
                pass
corpus.mkdir(parents=True, exist_ok=True)

rng = random.Random(260426)

modules = [
    "auth", "billing", "orders", "inventory", "shipping", "audit",
    "notify", "reporting", "admin", "session", "fraud", "search"
]

symbols = []
imports = []
calls = []
entrypoints = []
findings = []

def add_file(rel, text):
    p = corpus / rel
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(text)

for idx, mod in enumerate(modules):
    hdr = []
    src = []

    hdr.append(f"#ifndef {mod.upper()}_H")
    hdr.append(f"#define {mod.upper()}_H")
    hdr.append("")
    hdr.append(f"// module: {mod}")
    hdr.append(f"typedef struct {mod}_ctx {{ int state; int flags; }} {mod}_ctx;")
    hdr.append(f"int {mod}_init({mod}_ctx* ctx);")
    hdr.append(f"int {mod}_handle({mod}_ctx* ctx, int request_id);")
    hdr.append(f"int {mod}_flush({mod}_ctx* ctx);")
    hdr.append("#endif")
    add_file(f"include/{mod}.h", "\n".join(hdr) + "\n")

    symbols += [f"{mod}_ctx", f"{mod}_init", f"{mod}_handle", f"{mod}_flush"]

    src.append(f'#include "{mod}.h"')
    if idx > 0:
        dep = modules[idx - 1]
        src.append(f'#include "{dep}.h"')
        imports.append((mod, dep))
    if idx > 1 and idx % 2 == 0:
        dep2 = modules[idx - 2]
        src.append(f'#include "{dep2}.h"')
        imports.append((mod, dep2))
    src.append("")
    src.append(f"static int {mod}_validate({mod}_ctx* ctx, int request_id) {{")
    src.append("    int score = request_id + ctx->state;")
    src.append("    if ((score % 7) == 0) { score += 13; }")
    src.append("    for (int i = 0; i < 3; i++) { score += i; }")
    src.append("    return score;")
    src.append("}")
    symbols.append(f"{mod}_validate")

    src.append("")
    src.append(f"int {mod}_init({mod}_ctx* ctx) {{")
    src.append("    ctx->state = 1;")
    src.append("    ctx->flags = 0;")
    src.append("    return 0;")
    src.append("}")

    src.append("")
    src.append(f"int {mod}_handle({mod}_ctx* ctx, int request_id) {{")
    src.append(f"    int local = {mod}_validate(ctx, request_id);")
    calls.append((f"{mod}_handle", f"{mod}_validate"))
    if idx > 0:
        dep = modules[idx - 1]
        src.append(f"    local += {dep}_flush(({dep}_ctx*)ctx);")
        calls.append((f"{mod}_handle", f"{dep}_flush"))
    if idx % 3 == 0:
        src.append("    // suspicious: fallback bypass path")
        findings.append(f"{mod}:fallback_bypass_comment")
    src.append("    if (local > 5000) { return -1; }")
    src.append("    return local;")
    src.append("}")

    src.append("")
    src.append(f"int {mod}_flush({mod}_ctx* ctx) {{")
    src.append("    return ctx->state;")
    src.append("}")

    add_file(f"src/{mod}.c", "\n".join(src) + "\n")

    if mod in ("auth", "orders", "admin", "reporting"):
        entrypoints.append(f"{mod}_handle")

cfg = {
    "service_name": "local_stack",
    "primary_entrypoints": entrypoints,
    "feature_flags": {
        "allow_shadow_admin": True,
        "strict_audit": False,
        "replay_checkpointing": True
    }
}
add_file("config/service.json", json.dumps(cfg, indent=2) + "\n")

docs = """# Incident Brief
A field deployment is partially degraded.

Known concerns:
- admin fallback comments remain in-tree
- audit strictness is disabled
- replay checkpointing is enabled
- corruption has touched some migration and patch surfaces
"""
add_file("docs/incident_brief.md", docs)

sql = []
sql.append("CREATE TABLE sessions (id INTEGER PRIMARY KEY, user_id INTEGER, token TEXT);")
sql.append("CREATE TABLE orders (id INTEGER PRIMARY KEY, session_id INTEGER, total_cents INTEGER);")
sql.append("CREATE INDEX idx_orders_session ON orders(session_id);")
sql.append("-- suspicious: nullable admin override token")
findings.append("sql:nullable_admin_override_token_comment")
add_file("migrations/001_core.sql", "\n".join(sql) + "\n")

log = []
for i in range(30):
    sev = "WARN" if i % 7 == 0 else "INFO"
    line = f"{sev} module={modules[i % len(modules)]} event=req_{i} status={(i * 17) % 5}"
    log.append(line)
log.append("WARN module=admin event=fallback_bypass_detected status=1")
findings.append("log:fallback_bypass_detected")
add_file("logs/runtime.log", "\n".join(log) + "\n")

patch = []
patch.append("--- a/src/admin.c")
patch.append("+++ b/src/admin.c")
patch.append("@@")
patch.append("-    // TODO remove bypass")
patch.append("+    // TODO remove bypass before release")
patch.append("+    local += auth_flush((auth_ctx*)ctx);")
calls.append(("admin_handle", "auth_flush"))
add_file("patches/hotfix.diff", "\n".join(patch) + "\n")

# corruption slices
corrupt = corpus / "corrupt"
corrupt.mkdir(exist_ok=True)
bad = """fn ??? broken @@
#include "auth.h"
int broken_path(auth_ctx* ctx) {
    if (ctx->state > 0) {
        return auth_handle(ctx, 99);
%%CORRUPT%%
"""
add_file("corrupt/bad_slice_01.code", bad)
findings.append("corrupt:bad_slice_01")

truth = {
    "version": 1,
    "symbols": sorted(set(symbols)),
    "imports": sorted([{"from": a, "to": b} for a, b in imports], key=lambda x: (x["from"], x["to"])),
    "calls": sorted([{"from": a, "to": b} for a, b in calls], key=lambda x: (x["from"], x["to"])),
    "entrypoints": sorted(set(entrypoints)),
    "findings": sorted(set(findings)),
}
truth_text = json.dumps(truth, sort_keys=True).encode("utf-8")
truth["truth_hash"] = hashlib.sha256(truth_text).hexdigest()
(root / "specs" / "ground_truth.json").write_text(json.dumps(truth, indent=2))

print(json.dumps({
    "version": 1,
    "corpus_root": str(corpus),
    "truth_path": str(root / "specs" / "ground_truth.json"),
    "files": len(list(corpus.rglob("*")))
}, indent=2))
