#!/usr/bin/env bash
set -euo pipefail
ROOT=/home/ubuntu/Braxon
OUT="$ROOT/audit/expanded/function_inventory.tsv"
printf 'repo\tbranch_or_head\tlanguage\tsource_files\tfunction_declarations\ttest_markers\tfiles_with_functions\tfiles_with_test_markers\tfunction_files_without_local_test_marker\n' > "$OUT"
scan_repo() {
  local name="$1" dir="$2" ref="$3"
  cd "$dir"
  local files funcs tests funfiles testfiles untested lang
  for lang in rust python javascript typescript go; do
    case "$lang" in
      rust) files=$(git ls-tree -r --name-only "$ref" | grep -E '\.rs$' | grep -Ev '(^|/)(target|release|node_modules)/' || true); funcs=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc '^\s*(pub\s+)?(async\s+)?fn\s+[A-Za-z0-9_]+|^\s*fn\s+[A-Za-z0-9_]+' || true); tests=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc '#\[(test|cfg\(test\)|tokio::test)' || true);;
      python) files=$(git ls-tree -r --name-only "$ref" | grep -E '\.py$' | grep -Ev '(^|/)(target|release|node_modules|__pycache__)/' || true); funcs=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc '^\s*(async\s+)?def\s+[A-Za-z0-9_]+' || true); tests=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc '(^|[^A-Za-z])def test_|pytest|unittest|assert ' || true);;
      javascript) files=$(git ls-tree -r --name-only "$ref" | grep -E '\.js$' | grep -Ev '(^|/)(target|release|node_modules)/' || true); funcs=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc '^\s*(export\s+)?(async\s+)?function\s+|=>\s*\{' || true); tests=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc 'describe\(|it\(|test\(' || true);;
      typescript) files=$(git ls-tree -r --name-only "$ref" | grep -E '\.tsx?$' | grep -Ev '(^|/)(target|release|node_modules)/' || true); funcs=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc '^\s*(export\s+)?(async\s+)?function\s+|=>\s*\{' || true); tests=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc 'describe\(|it\(|test\(' || true);;
      go) files=$(git ls-tree -r --name-only "$ref" | grep -E '\.go$' | grep -Ev '(^|/)(target|release|node_modules)/' || true); funcs=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc '^\s*func\s+' || true); tests=$(printf '%s\n' "$files" | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null; done | grep -Ehc 'func Test|func Benchmark' || true);;
    esac
    funfiles=$(printf '%s\n' "$files" | sed '/^$/d' | wc -l)
    testfiles=$(printf '%s\n' "$files" | sed '/^$/d' | while read -r f; do [ -n "$f" ] && git show "$ref:$f" 2>/dev/null | grep -Eq '#\[(test|cfg\(test\)|tokio::test)|(^|[^A-Za-z])def test_|describe\(|func Test' && echo x; done | wc -l)
    untested=$(printf '%s\n' "$files" | sed '/^$/d' | while read -r f; do [ -n "$f" ] && content=$(git show "$ref:$f" 2>/dev/null) && printf '%s' "$content" | grep -Eq 'fn |def |function |func ' && ! printf '%s' "$content" | grep -Eq '#\[(test|cfg\(test\)|tokio::test)|(^|[^A-Za-z])def test_|describe\(|func Test' && echo x; done | wc -l)
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$name" "$ref" "$lang" "$(printf '%s\n' "$files" | sed '/^$/d' | wc -l)" "$funcs" "$tests" "$funfiles" "$testfiles" "$untested" >> "$OUT"
  done
}
scan_repo Braxon "$ROOT" reconstruction
for dir in /home/ubuntu/related/*; do
  [ -d "$dir/.git" ] || continue
  head=$(cd "$dir" && git symbolic-ref --quiet --short refs/remotes/origin/HEAD 2>/dev/null || echo origin/main)
  scan_repo "$(basename "$dir")" "$dir" "$head"
done
sort -o "$OUT" -t $'\t' -k1,1 -k3,3 "$OUT"
cat "$OUT"
