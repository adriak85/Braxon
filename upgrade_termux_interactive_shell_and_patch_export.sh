#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
OUT="$ROOT/state/full_android_language_toolchain/upgrade_termux_shell_$(date +%Y%m%d_%H%M%S).log"

{
  echo "=== install interactive shell helpers ==="
  pkg install -y zsh fish nano micro vim fzf ripgrep fd bat jq less termux-tools

  echo
  echo "=== safe patch export cargo inventory block ==="
  python3 - <<'PY'
from pathlib import Path
import re

p = Path.home() / "Braxon/export_full_braxon_system_state.sh"
s = p.read_text()

pattern = re.compile(
    r'"\$ROOT/braxon-cargo"\s+metadata\s+--no-deps\s+--format-version\s+1\s+\\\n'
    r'\s*\|\s*"\$ROOT/braxon-python"\s+-\s+<<[\'"]?PY[\'"]?\s+\\\n'
    r'\s*>\s*"\$OUTDIR/cargo/package_inventory\.txt"\n'
    r'(?P<body>.*?)\nPY',
    re.DOTALL,
)

replacement = '''CARGO_META_NO_DEPS="$OUTDIR/cargo/cargo_metadata_no_deps_for_inventory.json"
"$ROOT/braxon-cargo" metadata --no-deps --format-version 1 > "$CARGO_META_NO_DEPS" 2> "$OUTDIR/cargo/cargo_metadata_no_deps_for_inventory.stderr" || true
"$ROOT/braxon-python" -c 'import json,sys,pathlib; p=pathlib.Path(sys.argv[1]); data=p.read_text().strip(); print("\\n".join(pkg["name"] for pkg in json.loads(data)["packages"]) if data else "cargo_metadata_empty")' "$CARGO_META_NO_DEPS" > "$OUTDIR/cargo/package_inventory.txt"'''

s2, n = pattern.subn(replacement, s, count=1)

if n == 0:
    # Fallback: replace the narrower stdin-json block if the script was generated in a different shape.
    pattern2 = re.compile(
        r'"\$ROOT/braxon-cargo"\s+metadata\s+--no-deps\s+--format-version\s+1\s+\\\n'
        r'\s*\|\s*"\$ROOT/braxon-python"\s+-c\s+.*?json\.load\(sys\.stdin\).*?\n'
        r'\s*>\s*"\$OUTDIR/cargo/package_inventory\.txt"',
        re.DOTALL,
    )
    s2, n = pattern2.subn(replacement, s, count=1)

if n == 0:
    raise SystemExit("cargo inventory block still not found; inspect with: grep -n -A12 -B4 'cargo packages' ~/Braxon/export_full_braxon_system_state.sh")

p.write_text(s2)
print("patched cargo inventory blocks:", n)
PY

  echo
  echo "=== write responsive bash helpers ==="
  cat >> "$HOME/.bashrc" <<'EOF'

# Braxon interactive helpers
export EDITOR=micro
export LESS='-R'
export FZF_DEFAULT_COMMAND='fd --type f --hidden --follow --exclude .git 2>/dev/null'
alias ll='ls -lah --color=auto'
alias gs='git status --short'
alias br='cd ~/Braxon'
alias fast='~/Braxon/fastest_status'
alias bx-report='~/Braxon/export_full_braxon_system_state.sh'
alias rgc='rg --line-number --hidden --glob "!.git"'
set -o vi 2>/dev/null || true
EOF

  echo
  echo "=== write zsh config if available ==="
  if command -v zsh >/dev/null 2>&1; then
    cat > "$HOME/.zshrc" <<'EOF'
export EDITOR=micro
export LESS='-R'
export FZF_DEFAULT_COMMAND='fd --type f --hidden --follow --exclude .git 2>/dev/null'

autoload -Uz compinit
compinit

bindkey -v

alias ll='ls -lah --color=auto'
alias gs='git status --short'
alias br='cd ~/Braxon'
alias fast='~/Braxon/fastest_status'
alias bx-report='~/Braxon/export_full_braxon_system_state.sh'
alias rgc='rg --line-number --hidden --glob "!.git"'

setopt auto_cd
setopt correct
setopt hist_ignore_dups
setopt share_history
HISTFILE=~/.zsh_history
HISTSIZE=50000
SAVEHIST=50000
EOF
  fi

  echo
  echo "=== syntax check export script ==="
  bash -n "$ROOT/export_full_braxon_system_state.sh"

  echo
  echo "=== cargo block now ==="
  grep -n -A10 -B4 'CARGO_META_NO_DEPS\|cargo packages' "$ROOT/export_full_braxon_system_state.sh" || true

  echo
  echo "DONE"
  echo "log: $OUT"
} 2>&1 | tee "$OUT"
