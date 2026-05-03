#!/data/data/com.termux/files/usr/bin/bash
set -e
exec /data/data/com.termux/files/usr/bin/gpg-authcode-sign.sh "$@"
