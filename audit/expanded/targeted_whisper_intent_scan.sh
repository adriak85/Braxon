#!/usr/bin/env bash
set -u
BASE=/home/ubuntu/Braxon
OUT="$BASE/audit/expanded/targeted_whisper_intent_matches.tsv"
DOC="$BASE/audit/expanded/targeted_whisper_intent_docs.tsv"
printf 'repo\tref\tpath\tline\tmatch\n' > "$OUT"
printf 'repo\tref\tpath\tline\tmatch\n' > "$DOC"
for dir in "$BASE" /home/ubuntu/related/*; do
  [ -d "$dir/.git" ] || continue
  repo=$(basename "$dir")
  refs=$(git -C "$dir" for-each-ref --format='%(refname:short)' refs/remotes refs/heads)
  while IFS= read -r ref; do
    [ -n "$ref" ] || continue
    cd "$dir"
    git grep -I -n -i -E 'whisper|willow|stone' "$ref" 2>/dev/null | awk -v repo="$repo" -v ref="$ref" -F: 'BEGIN{OFS="\t"}{path=$1;line=$2;sub(/^[^:]*:[^:]*:/,"",$0);print repo,ref,path,line,$0}' >> "$OUT" || true
    git grep -I -n -i -E 'intent|gradient|rebuild|materializ|seed|canonical|deprecated|obsolete' "$ref" -- '*.rs' '*.py' '*.js' '*.jsx' '*.ts' '*.tsx' '*.go' '*.sh' '*.bash' '*.toml' '*.yaml' '*.yml' '*.json' 2>/dev/null | awk -v repo="$repo" -v ref="$ref" -F: 'BEGIN{OFS="\t"}{path=$1;line=$2;sub(/^[^:]*:[^:]*:/,"",$0);print repo,ref,path,line,$0}' >> "$OUT" || true
    git grep -I -n -i -E 'whisper|willow|stone|intent|gradient|rebuild|materializ|seed|canonical|deprecated|obsolete' "$ref" -- '*.md' '*.txt' 'docs/**' 'README*' 2>/dev/null | awk -v repo="$repo" -v ref="$ref" -F: 'BEGIN{OFS="\t"}{path=$1;line=$2;sub(/^[^:]*:[^:]*:/,"",$0);print repo,ref,path,line,$0}' >> "$DOC" || true
  done <<< "$refs"
done
sort -o "$OUT" -t $'\t' -k1,1 -k2,2 -k3,3 "$OUT"
sort -o "$DOC" -t $'\t' -k1,1 -k2,2 -k3,3 "$DOC"
