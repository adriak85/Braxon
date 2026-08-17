#!/usr/bin/env bash
set -euo pipefail
ROOT=/home/ubuntu/Braxon
OUT="$ROOT/audit/expanded/branch_candidate_metrics.tsv"
mkdir -p "$(dirname "$OUT")"
printf 'repo\tbranch\tcommit\tfiles\tchanged_vs_baseline\tadded\tdeleted\tmodified\tcode_paths\ttest_paths\tmanifest_paths\tbenchmark_paths\tlikely_candidate\n' > "$OUT"
metric_repo() {
  local repo="$1" baseline_ref="$2" repo_name
  repo_name=$(basename "$repo")
  cd "$repo"
  local base_tree="$baseline_ref"
  for ref in $(git for-each-ref --format='%(refname)' refs/remotes/origin | sort); do
    local branch=${ref#refs/remotes/origin/}
    [ "$branch" = "origin" ] && continue
    local commit files changed added deleted modified code tests manifests benchmarks score
    commit=$(git rev-parse "$ref")
    files=$(git ls-tree -r --name-only "$ref" | wc -l)
    changed=$(git diff --no-renames --name-only "$base_tree" "$ref" | wc -l)
    added=$(git diff --no-renames --name-only --diff-filter=A "$base_tree" "$ref" | wc -l)
    deleted=$(git diff --no-renames --name-only --diff-filter=D "$base_tree" "$ref" | wc -l)
    modified=$(git diff --no-renames --name-only --diff-filter=M "$base_tree" "$ref" | wc -l)
    code=$(git diff --no-renames --name-only "$base_tree" "$ref" | grep -E '\.(rs|c|cc|cpp|h|hpp|py|js|ts|tsx|go|java|kt|swift|sh|bash|zig|wasm|asm|s|S)$' | wc -l || true)
    tests=$(git diff --no-renames --name-only "$base_tree" "$ref" | grep -Ei '(^|/)(test|tests|bench|benchmark|ci|workflow)(/|$)|(^|/)[^/]*(test|bench)[^/]*\.' | wc -l || true)
    manifests=$(git diff --no-renames --name-only "$base_tree" "$ref" | grep -Ei '(^|/)(Cargo\.toml|Cargo\.lock|package\.json|pyproject\.toml|requirements[^/]*|Makefile|CMakeLists\.txt|Dockerfile|.*\.ya?ml|.*\.json)$' | wc -l || true)
    benchmarks=$(git diff --no-renames --name-only "$base_tree" "$ref" | grep -Ei '(bench|perf|benchmark|profile|stress|fuzz)' | wc -l || true)
    score=$((code + tests + benchmarks + manifests - deleted))
    if [ "$score" -ge 3 ] && [ "$code" -gt 0 ] && [ "$deleted" -lt 1000 ]; then likely=yes; else likely=no; fi
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$repo_name" "$branch" "${commit:0:12}" "$files" "$changed" "$added" "$deleted" "$modified" "$code" "$tests" "$manifests" "$benchmarks" "$likely" >> "$OUT"
  done
}
metric_repo "$ROOT" origin/main
for repo in /home/ubuntu/related/*; do
  [ -d "$repo/.git" ] || continue
  default=$(cd "$repo" && git symbolic-ref --quiet --short refs/remotes/origin/HEAD 2>/dev/null || true)
  [ -z "$default" ] && default="origin/$(cd "$repo" && git symbolic-ref --short HEAD | sed 's#^origin/##')"
  metric_repo "$repo" "$default"
done
sort -o "$OUT" -t $'\t' -k1,1 -k13,13r -k9,9nr "$OUT"
printf 'metrics=%s\n' "$OUT"
printf 'rows=%s\n' "$(($(wc -l < "$OUT")-1))"
printf 'likely_candidates=%s\n' "$(grep -c $'\tyes$' "$OUT" || true)"
