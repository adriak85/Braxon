#!/usr/bin/env bash
set -u
ROOT=/home/ubuntu/Braxon
OUT="$ROOT/audit/expanded/function_inventory.tsv"
printf 'repo\tbranch_or_head\tlanguage\tsource_files\tfunction_declarations\ttest_markers\tfunction_bearing_files\ttest_bearing_files\n' > "$OUT"
count_repo() {
  local name="$1" dir="$2" ref="$3" ext="$4" fnre="$5" testre="$6"
  cd "$dir" || return
  local files content
  files=$(git ls-tree -r --name-only "$ref" 2>/dev/null | grep -E "$ext" | grep -Ev '(^|/)(target|release|node_modules|__pycache__)/' || true)
  local source_files; source_files=$(printf '%s\n' "$files" | sed '/^$/d' | wc -l)
  content=$(printf '%s\n' "$files" | sed '/^$/d' | while IFS= read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done || true)
  local funcs tests
  funcs=$(printf '%s\n' "$content" | grep -E "$fnre" | wc -l)
  tests=$(printf '%s\n' "$content" | grep -E "$testre" | wc -l)
  local function_files test_files
  function_files=$(printf '%s\n' "$files" | sed '/^$/d' | while IFS= read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null | grep -Eq "$fnre" && echo x; done | wc -l)
  test_files=$(printf '%s\n' "$files" | sed '/^$/d' | while IFS= read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null | grep -Eq "$testre" && echo x; done | wc -l)
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$name" "$ref" "$7" "$source_files" "$funcs" "$tests" "$function_files" "$test_files" >> "$OUT"
}
scan_repo() {
  local name="$1" dir="$2" ref="$3"
  count_repo "$name" "$dir" "$ref" '\.rs$' '^\s*(pub\s+)?(async\s+)?fn\s+[A-Za-z0-9_]+' '#\[(test|cfg\(test\)|tokio::test)' rust
  count_repo "$name" "$dir" "$ref" '\.py$' '^\s*(async\s+)?def\s+[A-Za-z0-9_]+' '(^|[^A-Za-z])def test_|pytest|unittest' python
  count_repo "$name" "$dir" "$ref" '\.(js|jsx)$' '^\s*(export\s+)?(async\s+)?function\s+|=>\s*\{' 'describe\(|it\(|test\(' javascript
  count_repo "$name" "$dir" "$ref" '\.(ts|tsx)$' '^\s*(export\s+)?(async\s+)?function\s+|=>\s*\{' 'describe\(|it\(|test\(' typescript
  count_repo "$name" "$dir" "$ref" '\.go$' '^\s*func\s+' 'func Test|func Benchmark' go
}
scan_repo Braxon "$ROOT" reconstruction
for dir in /home/ubuntu/related/*; do
  [ -d "$dir/.git" ] || continue
  head=$(cd "$dir" && git symbolic-ref --quiet --short refs/remotes/origin/HEAD 2>/dev/null || echo origin/main)
  scan_repo "$(basename "$dir")" "$dir" "$head"
done
sort -o "$OUT" -t $'\t' -k1,1 -k3,3 "$OUT"
cat "$OUT"
