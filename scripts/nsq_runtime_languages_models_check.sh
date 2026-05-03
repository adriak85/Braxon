#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${BRAXON_ROOT:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/nsq_runtime_languages_models_check_${STAMP}"

mkdir -p "$OUT"
cd "$ROOT"

{
  echo "== NSQ runtime language/platform/model check =="
  date
  echo "root=$ROOT"
  echo

  echo "== language runtime audit =="
  python3 tools/nsq_finish/nsq_language_runtime_audit.py | tee "$OUT/language_runtime_audit.json"
  echo

  echo "== model install audit =="
  python3 tools/nsq_finish/nsq_model_install_audit.py | tee "$OUT/model_install_audit.json"
  echo

  echo "== Blake Null digest docs =="
  python3 tools/nsq_finish/nsq_blake_null.py docs/nsq/NSQ_RUNTIME_LANGUAGE_SURFACES.md --out "$OUT/blake_null_language_surfaces.json"
  cat "$OUT/blake_null_language_surfaces.json"
  echo

  echo "== model/state relevant files =="
  for f in \
    models/braxon/manifest.json \
    state/braxon/offline_model_registry.json \
    state/braxon/braxon_binding.json \
    state/braxon/braxon_nsq_pipeline.status \
    state/nsq/model_reconstruction_manifest.json \
    config/nsq/nsq_runtime_language_registry.json \
    config/nsq/nsq_runtime_platform_registry.json \
    config/nsq/nsq_model_install_targets.json
  do
    if [ -e "$f" ]; then
      echo "present $f"
    else
      echo "missing $f"
    fi
  done
  echo

  echo "== cargo check quick =="
  if command -v cargo >/dev/null 2>&1 && [ -f Cargo.toml ]; then
    cargo check --workspace > "$OUT/cargo_check.out" 2> "$OUT/cargo_check.err" || true
    tail -n 40 "$OUT/cargo_check.err" || true
  else
    echo "cargo unavailable or no Cargo.toml"
  fi

  echo
  echo "== git new files summary =="
  git status --short docs/nsq config/nsq tools/nsq_finish scripts/nsq_runtime_languages_models_check.sh state/nsq 2>/dev/null || true

} | tee "$OUT/summary.txt"

echo "report_dir=$OUT"
