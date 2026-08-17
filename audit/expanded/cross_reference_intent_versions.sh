#!/usr/bin/env bash
set -u
BASE=/home/ubuntu/Braxon
OUT="$BASE/audit/expanded/intent_version_cross_reference.tsv"
DOC="$BASE/audit/expanded/intent_documentation_cross_reference.tsv"
DUP="$BASE/audit/expanded/duplicate_version_groups.tsv"
printf 'repo\tref\tterm\tpath\tline\tmatch\n' > "$OUT"
printf 'repo\tref\tdocumentation_path\tterm\tline\tmatch\n' > "$DOC"
printf 'repo\tworking_tree\tnormalized_name\tcount\tpaths\n' > "$DUP"
scan_ref() {
  local dir="$1" repo="$2" ref="$3" path term line text
  cd "$dir" || return
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    for term in 'intent' 'whisper' 'willow' 'stone' 'truth' 'canonical' 'deprecated' 'obsolete' 'rebuild'; do
      while IFS=: read -r line text; do
        [ -n "$line" ] && printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$repo" "$ref" "$term" "$path" "$line" "$text" >> "$OUT"
      done < <(git grep -I -n -i -e "$term" "$ref" -- "$path" 2>/dev/null || true)
    done
    lower=$(printf '%s' "$path" | tr '[:upper:]' '[:lower:]')
    if [[ "$lower" == *.md || "$lower" == *.txt || "$lower" == */docs/* || "$lower" == docs/* || "$lower" == *readme* ]]; then
      for term in 'intent' 'whisper' 'willow' 'stone' 'truth' 'canonical' 'deprecated' 'obsolete' 'rebuild'; do
        while IFS=: read -r line text; do
          [ -n "$line" ] && printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$repo" "$ref" "$path" "$term" "$line" "$text" >> "$DOC"
        done < <(git grep -I -n -i -e "$term" "$ref" -- "$path" 2>/dev/null || true)
      done
    fi
  done < <(git ls-tree -r --name-only "$ref" 2>/dev/null)
}
for dir in "$BASE" /home/ubuntu/related/*; do
  [ -d "$dir/.git" ] || continue
  repo=$(basename "$dir")
  refs=$(git -C "$dir" for-each-ref --format='%(refname:short)' refs/remotes refs/heads)
  while IFS= read -r ref; do [ -n "$ref" ] && scan_ref "$dir" "$repo" "$ref"; done <<< "$refs"
  cd "$dir"
  find . -xdev -type f -printf '%p\n' | awk -F/ '{name=$NF; lower=tolower(name); gsub(/before_[^ ]+/,"",lower); gsub(/backup[^ ]*/,"",lower); print lower"\t"$0}' | sort | awk -F '\t' '{paths[$1]=paths[$1]" | "$2; count[$1]++} END{for(n in count) if(count[n]>1) print "'"$repo"'\tworking_tree\t"n"\t"count[n]"\t"paths[n]}' >> "$DUP"
done
sort -o "$OUT" -t $'\t' -k1,1 -k2,2 -k3,3 -k4,4 "$OUT"
sort -o "$DOC" -t $'\t' -k1,1 -k2,2 -k3,3 -k4,4 "$DOC"
sort -o "$DUP" -t $'\t' -k1,1 -k3,3 "$DUP"
