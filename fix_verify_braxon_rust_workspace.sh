#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
OUT="$TC/fix_verify_braxon_rust_workspace_$(date +%Y%m%d_%H%M%S).log"

{
  cd "$ROOT"
  source "$ROOT/braxon-rust-env"

  META="$TC/tmp/braxon_cargo_metadata.json"
  mkdir -p "$TC/tmp"

  echo "=== capture cargo metadata ==="
  "$ROOT/braxon-cargo" metadata --no-deps --format-version 1 > "$META"

  echo "=== exact package names ==="
  "$ROOT/braxon-python" -c '
import json
p="/data/data/com.termux/files/home/Braxon/state/full_android_language_toolchain/tmp/braxon_cargo_metadata.json"
m=json.load(open(p))
for pkg in m["packages"]:
    print(pkg["name"])
'

  echo "=== apply rustfmt ==="
  "$ROOT/braxon-cargo" fmt --all

  echo "=== test nsq-core ==="
  "$ROOT/braxon-cargo" test -p nsq-core -- --nocapture

  echo "=== test package names that actually exist ==="
  "$ROOT/braxon-python" -c '
import json, subprocess
root="/data/data/com.termux/files/home/Braxon"
meta="/data/data/com.termux/files/home/Braxon/state/full_android_language_toolchain/tmp/braxon_cargo_metadata.json"
m=json.load(open(meta))
wanted={"nsq-runtime","Braxon-core","Braxon-ingest","braxon-core","braxon-ingest"}
for pkg in [p["name"] for p in m["packages"] if p["name"] in wanted]:
    print("=== testing", pkg, "===")
    subprocess.run([root+"/braxon-cargo","test","-p",pkg,"--","--nocapture"], check=True)
'

  echo "=== lock refreshed Rust manifest ==="
  LOCKDIR="$TC/locks/braxon_rust_nightly_native"
  SYSROOT="$("$ROOT/braxon-rustc" --print sysroot)"
  mkdir -p "$LOCKDIR"

  {
    echo "BRAXON_RUST_NIGHTLY_NATIVE_LOCK=1"
    date
    echo "SYSROOT=$SYSROOT"
    "$ROOT/braxon-rustc" --version --verbose
    "$ROOT/braxon-cargo" --version --verbose
  } > "$LOCKDIR/LOCKED_RUST_NIGHTLY_NATIVE.txt"

  find "$SYSROOT" "$ROOT/braxon-rust-env" "$ROOT/braxon-rustc" "$ROOT/braxon-cargo" \
    -type f -print0 | sort -z | xargs -0 sha256sum > "$LOCKDIR/manifest.sha256"

  echo "DONE"
  echo "log: $OUT"
} 2>&1 | tee "$OUT"
