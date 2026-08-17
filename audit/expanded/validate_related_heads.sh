#!/usr/bin/env bash
set +e
OUT=/home/ubuntu/Braxon/audit/expanded/related_heads_validation.log
: > "$OUT"
run_repo() {
  local dir="$1" name; name=$(basename "$dir")
  echo "=== $name ===" | tee -a "$OUT"
  (cd "$dir" && echo "remote=$(git config --get remote.origin.url)" && echo "head=$(git rev-parse HEAD)" && echo "status=$(git status --porcelain | wc -l)" ) | tee -a "$OUT"
  if [ -f "$dir/Cargo.toml" ]; then
    (cd "$dir" && timeout 180 /usr/lib/rust-1.85/bin/cargo test --workspace --no-fail-fast) >>"$OUT" 2>&1
    echo "cargo_test_exit=$?" | tee -a "$OUT"
  else echo 'cargo_test=SKIPPED(no Cargo.toml)' | tee -a "$OUT"; fi
  if find "$dir" -path '*/target' -prune -o -name '*.py' -print -quit | grep -q .; then
    (cd "$dir" && timeout 120 python3 -m compileall -q -f .) >>"$OUT" 2>&1
    echo "python_compileall_exit=$?" | tee -a "$OUT"
  else echo 'python_compileall=SKIPPED(no Python files)' | tee -a "$OUT"; fi
  if [ -f "$dir/go.mod" ]; then
    (cd "$dir" && timeout 120 go test ./...) >>"$OUT" 2>&1
    echo "go_test_exit=$?" | tee -a "$OUT"
  else echo 'go_test=SKIPPED(no go.mod)' | tee -a "$OUT"; fi
}
for dir in /home/ubuntu/related/*; do [ -d "$dir/.git" ] && run_repo "$dir"; done
