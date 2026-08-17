#!/usr/bin/env bash
set -u
BASE=/home/ubuntu/Braxon
OUT="$BASE/audit/expanded/intent_version_cross_reference.tsv"
DOC="$BASE/audit/expanded/intent_documentation_cross_reference.tsv"
DUP="$BASE/audit/expanded/duplicate_version_groups.tsv"
printf 'repo\tref\tpath\tline\tmatch\n' > "$OUT"
printf 'repo\tref\tpath\tline\tmatch\n' > "$DOC"
printf 'repo\tworking_tree\tnormalized_name\tcount\tpaths\n' > "$DUP"
pattern='intent|whisper|willow|stone|truth|canonical|deprecated|obsolete|rebuild'
for dir in "$BASE" /home/ubuntu/related/*; do
  [ -d "$dir/.git" ] || continue
  repo=$(basename "$dir")
  refs=$(git -C "$dir" for-each-ref --format='%(refname:short)' refs/remotes refs/heads)
  while IFS= read -r ref; do
    [ -n "$ref" ] || continue
    cd "$dir"
    git grep -I -n -i -E "$pattern" "$ref" 2>/dev/null | awk -v repo="$repo" -v ref="$ref" -F: 'BEGIN{OFS="\t"}{path=$1;line=$2;sub(/^[^:]*:[^:]*:/,"",$0);print repo,ref,path,line,$0}' >> "$OUT" || true
    git grep -I -n -i -E "$pattern" "$ref" -- '*.md' '*.txt' 'docs/**' 'README*' 2>/dev/null | awk -v repo="$repo" -v ref="$ref" -F: 'BEGIN{OFS="\t"}{path=$1;line=$2;sub(/^[^:]*:[^:]*:/,"",$0);print repo,ref,path,line,$0}' >> "$DOC" || true
  done <<< "$refs"
  cd "$dir"
  find . -xdev -type f -printf '%f\t%p\n' | awk -F '\t' '{n=tolower($1); gsub(/before_[^ ]+/,"",n); gsub(/backup[^ ]*/,"",n); print n"\t"$2}' | sort | awk -F '\t' -v repo="$repo" '{paths[$1]=paths[$1]" | "$2;count[$1]++} END{for(n in count)if(count[n]>1)print repo"\tworking_tree\t"n"\t"count[n]"\t"paths[n]}' >> "$DUP"
done
sort -o "$OUT" -t $'\t' -k1,1 -k2,2 -k3,3 "$OUT"
sort -o "$DOC" -t $'\t' -k1,1 -k2,2 -k3,3 "$DOC"
sort -o "$DUP" -t $'\t' -k1,1 -k3,3 "$DUP"
