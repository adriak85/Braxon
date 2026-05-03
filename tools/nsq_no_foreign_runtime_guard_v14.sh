#!/data/data/com.termux/files/usr/bin/bash
set -eu
cd "$HOME/Braxon" || exit 1
name="${1:-}"
reg="state/nsq/stamps/foreign_tool_absorption/latest/foreign_tool_substrate_stamp_registry.jsonl"
[ -n "$name" ] || { echo "DENY missing tool name"; exit 91; }
[ -f "$reg" ] || { echo "DENY missing NSQ absorption registry"; exit 91; }
line="$(grep -F "\"name\":\"$name\"" "$reg" | tail -n 1 || true)"
[ -n "$line" ] || { echo "DENY not absorbed into NSQ substrate: $name"; exit 91; }
echo "$line" | grep -F '"runtime_authority":"nsq_substrate"' >/dev/null || { echo "DENY no NSQ substrate authority: $name"; exit 91; }
echo "$line" | grep -F '"foreign_runtime_allowed":false' >/dev/null || { echo "DENY foreign runtime still allowed: $name"; exit 91; }
echo "ALLOW NSQ-STAMPED ONLY: $name"
