#!/data/data/com.termux/files/usr/bin/sh
set -eu

cd "$HOME/Braxon"

echo "== substrate scan =="

bad=0

scan() {
  pattern="$1"
  label="$2"
  if rg -n --hidden --glob '!target' --glob '!.git' "$pattern" \
      crates/nsq-* 2>/dev/null; then
    echo "SUBSTRATE WARNING: $label"
    bad=1
  fi
}

scan '\bsymbol_to_id\b' 'legacy symbol_to_id naming still present'
scan '\bmacro_to_id\b' 'legacy macro_to_id naming still present'
scan '\bsymbol_id_class\b' 'legacy symbol_id_class naming still present'
scan '\bmacro_id_class\b' 'legacy macro_id_class naming still present'
scan '\[symbol:u16\]|\[macro:u16\]|\[anchor:u32\]' 'legacy canonical lane examples still present'

if [ "$bad" -eq 0 ]; then
  echo "substrate scan clean"
else
  echo "substrate scan found drift"
  exit 1
fi
