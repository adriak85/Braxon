#!/usr/bin/env bash
set -euo pipefail
BASE=/home/ubuntu/Braxon
OUT="$BASE/audit/expanded/absolute_tree_refresh.tsv"
mkdir -p "$BASE/audit/expanded"
printf 'repo\tremote\tcommit\tbranch\trecord_type\tmode\tbytes\tpath\n' > "$OUT"
refresh_repo() {
  local dir="$1" repo remote commit branch
  repo=$(basename "$dir")
  cd "$dir"
  git fetch --all --prune --tags
  remote=$(git config --get remote.origin.url || true)
  commit=$(git rev-parse HEAD)
  branch=$(git branch --show-current)
  git ls-tree -r -t --full-tree HEAD | awk -v repo="$repo" -v remote="$remote" -v commit="$commit" -v branch="$branch" 'BEGIN{OFS="\t"}{path=substr($0,index($0,$4)); print repo,remote,commit,branch,"git:"$2,$1,"-",path}' >> "$OUT"
  find . -xdev -printf '%y\t%m\t%s\t%p\n' | awk -v repo="$repo" -v remote="$remote" -v commit="$commit" -v branch="$branch" 'BEGIN{OFS="\t"}{print repo,remote,commit,branch,"fs:"$1,$2,$3,$4}' >> "$OUT"
  git for-each-ref --format='%(refname:short)\t%(objectname)\t%(authordate:iso8601)' refs/remotes refs/heads > "$BASE/audit/expanded/${repo}_refs.tsv"
}
refresh_repo "$BASE"
for dir in /home/ubuntu/related/*; do [ -d "$dir/.git" ] && refresh_repo "$dir"; done
sort -o "$OUT" -t $'\t' -k1,1 -k8,8 "$OUT"
