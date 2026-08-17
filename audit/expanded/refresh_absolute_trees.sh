#!/usr/bin/env bash
set -euo pipefail
BASE=/home/ubuntu/Braxon
OUT="$BASE/audit/expanded/absolute_tree_refresh.tsv"
mkdir -p "$BASE/audit/expanded"
printf 'repo\tremote\tcommit\tbranch\tpath_type\tbytes\tmode\tpath\n' > "$OUT"
refresh_repo() {
  local dir="$1" repo remote
  repo=$(basename "$dir")
  cd "$dir"
  git fetch --all --prune --tags
  remote=$(git config --get remote.origin.url || true)
  while IFS=$'\t' read -r mode type object path; do
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$repo" "$remote" "$(git rev-parse HEAD)" "$(git branch --show-current)" "git:$type" "-" "$mode" "$path" >> "$OUT"
  done < <(git ls-tree -r -t --full-tree HEAD | awk '{print $1"\t"$2"\t"$3"\t"substr($0,index($0,$4))}')
  while IFS= read -r -d '' path; do
    if [ -L "$path" ]; then typ=symlink; elif [ -d "$path" ]; then typ=directory; elif [ -f "$path" ]; then typ=file; else typ=special; fi
    bytes=$(stat -c '%s' "$path" 2>/dev/null || printf '%s' '-')
    mode=$(stat -c '%a' "$path" 2>/dev/null || printf '%s' '-')
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$repo" "$remote" "$(git rev-parse HEAD)" "$(git branch --show-current)" "fs:$typ" "$bytes" "$mode" "${path#./}" >> "$OUT"
  done < <(find . -xdev -print0)
  git for-each-ref --format='%(refname:short)\t%(objectname)\t%(authordate:iso8601)' refs/remotes refs/heads > "$BASE/audit/expanded/${repo}_refs.tsv"
}
refresh_repo "$BASE"
for dir in /home/ubuntu/related/*; do [ -d "$dir/.git" ] && refresh_repo "$dir"; done
sort -o "$OUT" -t $'\t' -k1,1 -k8,8 "$OUT"
