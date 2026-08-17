#!/usr/bin/env bash
set -u
BASE=/home/ubuntu/Braxon
OUT="$BASE/audit/expanded/reassessed_prior_blockers_rust196.log"
RUSTUP_TOOLCHAIN=1.96.0
RUSTUP=/home/ubuntu/.cargo/bin/rustup
RUSTC=/home/ubuntu/.cargo/bin/rustc
CARGO=/home/ubuntu/.cargo/bin/cargo
export RUSTUP_TOOLCHAIN
printf 'rustc=%s\ncargo=%s\n' "$($RUSTUP run 1.96.0 rustc --version)" "$($RUSTUP run 1.96.0 cargo --version)" > "$OUT"
for repo in /home/ubuntu/related/0 /home/ubuntu/related/DAX-FULL /home/ubuntu/related/Dax /home/ubuntu/related/Dax-Autonomous-System; do
  [ -d "$repo/.git" ] || continue
  {
    printf '\n=== %s ===\n' "$(basename "$repo")"
    git -C "$repo" branch --show-current
    git -C "$repo" rev-parse HEAD
    printf '%s\n' '-- manifests --'
    find "$repo" -xdev -type f \( -name Cargo.toml -o -name pyproject.toml -o -name setup.cfg -o -name requirements*.txt -o -name '*.md' -o -name 'README*' \) -print | sort | sed -n '1,160p'
    printf '%s\n' '-- cargo check --workspace --all-targets --locked --offline --'
    (cd "$repo" && RUSTUP_TOOLCHAIN=1.96.0 RUSTC="$RUSTC" RUSTDOC=/home/ubuntu/.cargo/bin/rustdoc "$CARGO" check --workspace --all-targets --locked --offline) 2>&1 || true
    printf '%s\n' '-- cargo check --workspace --all-targets --locked --online --'
    (cd "$repo" && RUSTUP_TOOLCHAIN=1.96.0 RUSTC="$RUSTC" RUSTDOC=/home/ubuntu/.cargo/bin/rustdoc "$CARGO" check --workspace --all-targets --locked) 2>&1 || true
    printf '%s\n' '-- python compile --'
    (cd "$repo" && python3 -m compileall -q -f .) 2>&1 || true
    printf '%s\n' '-- targeted invalid/unsupported markers --'
    git -C "$repo" grep -n -I -i -E 'unsupported|invalid|deprecated|obsolete|whisper|willow|stone|intent|rebuild|adapter|vulkan|wgpu|target.?field' -- ':!target/**' 2>/dev/null | sed -n '1,1000p' || true
  } >> "$OUT"
done
