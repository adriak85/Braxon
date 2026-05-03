#!/data/data/com.termux/files/usr/bin/bash
SELF="${BASH_SOURCE[0]}"
DIR="$(cd "$(dirname "$SELF")" && pwd -P)"
ROOT="$(cd "$DIR/../../.." && pwd -P)"

export BRAXON_EMBEDDED_SERVER_BIN="$ROOT/tools/nsq/server/bin"
export BRAXON_EMBEDDED_SERVER_STATE="$ROOT/state/nsq/server"

case ":$PATH:" in
  *":$BRAXON_EMBEDDED_SERVER_BIN:"*) ;;
  *) export PATH="$BRAXON_EMBEDDED_SERVER_BIN:$PATH" ;;
esac

echo "Braxon embedded server surface active:"
echo "  bin:   $BRAXON_EMBEDDED_SERVER_BIN"
echo "  state: $BRAXON_EMBEDDED_SERVER_STATE"
