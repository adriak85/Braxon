#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${BRAXON_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
cd "$ROOT"

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  exit 0
fi

STAGED_FILES="$(git diff --cached --name-only --diff-filter=ACMR || true)"
if [ -z "$STAGED_FILES" ]; then
  exit 0
fi

TMP="${TMPDIR:-/tmp}/BRAXON_codex_quality_gate.$$"
FILTERED="${TMP}.files"
trap 'rm -f "$TMP" "$FILTERED"' EXIT

printf '%s\n' "$STAGED_FILES" | grep -Ev '^(tools/codex_quality_gate/|CODEX_AGENT_IMPLICIT_INSTRUCTIONS\.md$|AGENTS\.md$)' >"$FILTERED" || true
if [ ! -s "$FILTERED" ]; then
  exit 0
fi

git diff --cached --unified=0 -- $(cat "$FILTERED") >"$TMP" || exit 0

fail() {
  printf '%s\n' "BRAXON Codex quality gate failed: $1" >&2
  printf '%s\n' "NSQ is base 8; it is not u8, bytes, or a binary substrate." >&2
  printf '%s\n' "Court king is compositor authority; Qwen is theater/donor material, not substrate." >&2
  exit 1
}

added_lines() {
  grep -E '^\+[^+]' "$TMP" | sed 's/^+//'
}

if added_lines | grep -Eiq 'NSQ.*(is|as|=|means|becomes|underlying|substrate).*(u8|u16|u32|u64|u128|byte|bytes|binary bit|binary byte)'; then
  fail "new text/code makes NSQ a byte, binary, or host-width truth"
fi

if added_lines | grep -Ei '\b(u-?band|uband)\b' | grep -Eiv '(allowed *= *false|forbidden|not allowed|no u-?band|no uband)' >/dev/null; then
  fail "new text/code introduces forbidden uband framing"
fi

if added_lines | grep -Eiq '(flatten|collapse|reduce|boil down|lower).*(NSQ|Braxon|court|runtime).*(byte|bytes|binary|u8|u16|u32|u64|u128)'; then
  fail "new text/code flattens NSQ/Braxon/court runtime into binary or width-class semantics"
fi

if added_lines | grep -Eiq '(ultra dense|ultradense|stamp|macro|whole intent|single base-8 bit|tiny seed).*(not|isn.t|cannot|fake|synthetic stand-in)'; then
  fail "new text/code denies ultra-dense stamp/macro/tiny-seed reconstruction semantics"
fi

if added_lines | grep -Eiq '(citadel699|model|models).*(not|never|cannot|blocked).*(fully active|unified|mb-scale|mb scale|tiny seed|reconstruct)'; then
  fail "new text/code contradicts Citadel699 fully active unified MB-scale reconstruction target"
fi

if added_lines | grep -Eiq '(llama\.cpp|gguf as runtime authority|ggml as runtime authority)'; then
  fail "new text/code introduces forbidden foreign runtime authority"
fi

if added_lines | grep -Eiq '(python|qwen|language|runtime).*(hook|plugin|wrapper|sidecar).*(runtime|incorporation|completion|authority)'; then
  fail "new text/code treats hooks/plugins/wrappers as runtime incorporation"
fi

if added_lines | grep -Eiq '(remove|delete|quarantine|ignore|bypass).*(metadata hooks|hook matrices|hook matrix|hooks/hook_matrix|nsq/hooks/hook_matrix)'; then
  fail "new text/code removes or bypasses metadata hooks instead of preserving them as guidance"
fi

if added_lines | grep -Eiq '(rebuild|recreate|replace).*(Braxon|system|workspace|architecture).*(from scratch|without source|without pieces|without audit)'; then
  fail "new text/code permits rebuilding Braxon without its existing source pieces and audit path"
fi

if added_lines | grep -Eiq '(court king|king).*(is|=|as).*(not compositor|scheduler|plugin|wrapper|sidecar)'; then
  fail "new text/code drifts court king away from compositor authority"
fi

if added_lines | grep -Eiq '(qwen|quen).*(is|=|as).*(substrate|base machine|lowest base|runtime authority)'; then
  fail "new text/code makes Qwen substrate instead of theater/donor material"
fi

if added_lines | grep -Eiq '(stamp|macro).*(byte|bytes|binary payload|u8|u16|u32|u64|u128)'; then
  fail "new text/code drifts stamps/macros into byte or width-class carriers"
fi

exit 0
