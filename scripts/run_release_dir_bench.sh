#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
. "$ROOT/env/BRAXON_env.sh"

CORPUS="$BRAXON_HOME/benchmarks/repo_reality_vs_c/corpus/project"
OUT_BASE="$HOME/storage/shared/Download/nsq_release_bench_$(date +%Y%m%d_%H%M%S)"
TRACE_DIR="$OUT_BASE/strace"
mkdir -p "$OUT_BASE" "$TRACE_DIR"

cd "$BRAXON_HOME"
cargo build -q --release -p nsq-native-bench

BIN="$CARGO_TARGET_DIR/release/nsq-native-bench"

"$BIN" dir "$CORPUS" | tee "$OUT_BASE/nsq_release_dir_report.json"

strace -ff -tt -T \
  -o "$TRACE_DIR/nsq_native_bench_release_dir" \
  -e trace=execve,openat,statx,newfstatat,access,read,write,mmap,munmap,clone,fork,vfork,getdents64 \
  "$BIN" dir "$CORPUS" \
  | tee "$OUT_BASE/nsq_release_dir_report_straced.json"

grep -h 'execve(' "$TRACE_DIR"/nsq_native_bench_release_dir* > "$OUT_BASE/execve.txt" || true
grep -h 'openat(' "$TRACE_DIR"/nsq_native_bench_release_dir* | sed -E 's/.*"([^"]+)".*/\1/' | sort | uniq -c | sort -nr > "$OUT_BASE/opened_paths.txt" || true
grep -hE 'statx\(|newfstatat\(|access\(' "$TRACE_DIR"/nsq_native_bench_release_dir* | sed -E 's/.*"([^"]+)".*/\1/' | sort | uniq -c | sort -nr > "$OUT_BASE/stat_access_churn.txt" || true

echo "bench output: $OUT_BASE"
