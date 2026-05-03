#!/data/data/com.termux/files/usr/bin/bash
SELF="${BASH_SOURCE[0]}"
DIR="$(cd "$(dirname "$SELF")" && pwd -P)"
ROOT="$(cd "$DIR/../../.." && pwd -P)"

export BRAXON_ROOT="$ROOT"
export BRAXON_CPAN_SURFACE_BIN="$ROOT/tools/nsq/cpan/bin"
export BRAXON_CPAN_SURFACE_STATE="$ROOT/state/nsq/cpan"
export BRAXON_CPAN_STAMP_STATE="$ROOT/state/nsq/stamps"

case ":$PATH:" in
  *":$BRAXON_CPAN_SURFACE_BIN:"*) ;;
  *) export PATH="$BRAXON_CPAN_SURFACE_BIN:$PATH" ;;
esac

echo "Braxon CPAN surface active:"
echo "  bin: $BRAXON_CPAN_SURFACE_BIN"
