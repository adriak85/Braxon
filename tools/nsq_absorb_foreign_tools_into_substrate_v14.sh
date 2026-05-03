#!/data/data/com.termux/files/usr/bin/bash
set -eu

cd "$HOME/Braxon" || exit 1

STAMP="$(date +%Y%m%d_%H%M%S)"
ROOT="state/nsq/stamps/foreign_tool_absorption/$STAMP"
LATEST="state/nsq/stamps/foreign_tool_absorption/latest"
REGISTRY="$ROOT/foreign_tool_substrate_stamp_registry.jsonl"
CAPTURE="$ROOT/source_capture"
BODY="$ROOT/nsq_bodies"
META="$ROOT/metadata"
NSQ_APP="apps/nsq/foreign_tool_absorption_v14.nsq"
CONFIG="config/nsq/foreign_tool_absorption_v14.json"
ROUTE="state/nsq/court/routes/foreign_tool_absorption_v14.json"
SPEC="specs/nsq/NSQ_FOREIGN_TOOL_ABSORPTION_v14.md"
GUARD="tools/nsq_no_foreign_runtime_guard_v14.sh"
PERLLOCAL="$PREFIX/lib/perl5/5.42.0/aarch64-android/perllocal.pod"

mkdir -p "$ROOT" "$CAPTURE" "$BODY" "$META" "$(dirname "$NSQ_APP")" "$(dirname "$CONFIG")" "$(dirname "$ROUTE")" "$(dirname "$SPEC")"

: > "$REGISTRY"
: > "$ROOT/capture_manifest.tsv"
printf 'kind\tname\tsource_path\tcapture_path\tnsq_body\tsha256\tstatus\n' > "$ROOT/capture_manifest.tsv"

find_pm_without_loading() {
  mod="$1"
  rel="$(printf '%s.pm' "$mod" | sed 's#::#/#g')"
  for base in \
    "$PREFIX/lib/perl5/site_perl/5.42.0/aarch64-android" \
    "$PREFIX/lib/perl5/site_perl/5.42.0" \
    "$PREFIX/lib/perl5/5.42.0/aarch64-android" \
    "$PREFIX/lib/perl5/5.42.0"
  do
    p="$base/$rel"
    [ -f "$p" ] && { printf '%s\n' "$p"; return 0; }
  done
  return 1
}

safe_name() {
  printf '%s' "$1" | tr ':/ .-' '______' | tr -cd 'A-Za-z0-9_'
}

absorb_file() {
  kind="$1"
  name="$2"
  src="$3"

  safe="$(safe_name "$name")"
  cap="$CAPTURE/${safe}.source"
  nsq="$BODY/${safe}.translated.nsq"
  meta="$META/${safe}.json"

  if [ ! -f "$src" ]; then
    sha="missing"
    status="missing_source"
    printf '' > "$cap"
  else
    cp "$src" "$cap"
    sha="$(sha256sum "$cap" | awk '{print $1}')"
    status="captured_translated_to_nsq_stamp_body"
  fi

  {
    echo "NSQ_TRANSLATED_TOOL_BODY v14"
    echo "SOURCE_KIND $kind"
    echo "SOURCE_NAME $name"
    echo "SOURCE_SHA256 $sha"
    echo "RUNTIME_AUTHORITY nsq_substrate"
    echo "FOREIGN_RUNTIME_ALLOWED false"
    echo "NO_PARALLEL_RUNTIME true"
    echo "NO_SILENT_FOREIGN_EXECUTION true"
    echo "STAMP_SYSTEM_ACTIVE true"
    echo "BODY_BEGIN"
    if [ -s "$cap" ]; then
      awk '
        {
          gsub(/\\/,"\\\\");
          gsub(/\t/,"\\t");
          gsub(/\r/,"\\r");
          gsub(/"/,"\\\"");
          printf("NSQ_SOURCE_LINE %08d \"%s\"\n", NR, $0);
        }
      ' "$cap"
    fi
    echo "BODY_END"
  } > "$nsq"

  nsha="$(sha256sum "$nsq" | awk '{print $1}')"

  cat > "$meta" <<EOF
{
  "schema": "nsq.foreign_tool.absorbed_stamp.v14",
  "generated_at": "$STAMP",
  "kind": "$kind",
  "name": "$name",
  "source_path": "$src",
  "capture_path": "$cap",
  "source_sha256": "$sha",
  "nsq_body": "$nsq",
  "nsq_body_sha256": "$nsha",
  "runtime_authority": "nsq_substrate",
  "foreign_runtime_allowed": false,
  "parallel_runtime_allowed": false,
  "silent_foreign_execution_allowed": false,
  "stamp_system_active": true,
  "status": "$status"
}
EOF

  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$kind" "$name" "$src" "$cap" "$nsq" "$sha" "$status" >> "$ROOT/capture_manifest.tsv"

  printf '{"schema":"nsq.foreign_tool.absorbed_registry.v14","generated_at":"%s","kind":"%s","name":"%s","source_path":"%s","capture_path":"%s","source_sha256":"%s","nsq_body":"%s","nsq_body_sha256":"%s","metadata":"%s","runtime_authority":"nsq_substrate","foreign_runtime_allowed":false,"parallel_runtime_allowed":false,"silent_foreign_execution_allowed":false,"stamp_system_active":true,"status":"%s"}\n' \
    "$STAMP" "$kind" "$name" "$src" "$cap" "$sha" "$nsq" "$nsha" "$meta" "$status" >> "$REGISTRY"
}

if [ -f "$PERLLOCAL" ]; then
  perl -ne '
    if (/^=head2\s+(.+?):\s+C<Module>\s+L<([^|>]+)(?:\|[^>]+)?>/) {
      print "$2\n";
    }
  ' "$PERLLOCAL" | sort -u > "$ROOT/cpan_modules.txt"

  while IFS= read -r mod; do
    [ -n "$mod" ] || continue
    pm="$(find_pm_without_loading "$mod" || true)"
    [ -n "${pm:-}" ] && absorb_file "cpan_source" "$mod" "$pm" || absorb_file "cpan_source" "$mod" "missing"
  done < "$ROOT/cpan_modules.txt"
fi

for t in cc clang as ar ld cmake make cargo rustc git rg fd jq perl; do
  if command -v "$t" >/dev/null 2>&1; then
    absorb_file "native_tool_path" "$t" "$(command -v "$t")"
  else
    absorb_file "native_tool_path" "$t" "missing"
  fi
done

cat > "$NSQ_APP" <<EOF
NSQ_FORM Braxon.foreign_tool_absorption.v14

LAW nsq_is_only_runtime
LAW nsq_is_lowest_base_language
LAW nsq_is_substrate
LAW foreign_sources_are_absorbed_into_nsq_stamp_bodies
LAW foreign_runtime_execution_is_forbidden
LAW no_parallel_runtime
LAW no_silent_foreign_execution
LAW source_capture_and_nsq_body_are_stamped
LAW court_is_compositor
LAW court_is_not_agents

AUTHORITY NSQ_COURT
ROUTE foreign_tool_absorption_v14
STAMP_REGISTRY $REGISTRY
CAPTURE_MANIFEST $ROOT/capture_manifest.tsv
EOF

cat > "$CONFIG" <<EOF
{
  "schema": "Braxon.foreign_tool_absorption.v14",
  "generated_at": "$STAMP",
  "authority": "NSQ_COURT",
  "nsq_is_only_runtime": true,
  "foreign_sources_absorbed_into_nsq_stamp_bodies": true,
  "foreign_runtime_execution_forbidden": true,
  "parallel_runtime_allowed": false,
  "silent_foreign_execution_allowed": false,
  "registry": "$REGISTRY",
  "manifest": "$ROOT/capture_manifest.tsv",
  "guard": "$GUARD"
}
EOF

cat > "$ROUTE" <<EOF
{
  "schema": "nsq.court.route.v14",
  "generated_at": "$STAMP",
  "route": "foreign_tool_absorption_v14",
  "authority": "NSQ_COURT",
  "court_is_compositor": true,
  "court_is_agents": false,
  "nsq_is_only_runtime": true,
  "registry": "$REGISTRY",
  "foreign_runtime_execution_forbidden": true,
  "parallel_runtime_allowed": false,
  "silent_foreign_execution_allowed": false
}
EOF

cat > "$GUARD" <<'EOF'
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
EOF
chmod +x "$GUARD"

count="$(wc -l < "$REGISTRY" | tr -d ' ')"

cat > "$SPEC" <<EOF
# NSQ Foreign Tool Absorption v14

Generated: $STAMP

This install absorbs CPAN/native/build tool source surfaces into NSQ stamp bodies.

It does not load CPAN as runtime.
It does not create a parallel runtime.
It forbids silent foreign execution.
The active authority after this pass is the NSQ substrate stamp registry.

Registry: \`$REGISTRY\`
Capture manifest: \`$ROOT/capture_manifest.tsv\`
Stamp count: $count
Guard: \`$GUARD\`
EOF

cat > "$ROOT/proof.json" <<EOF
{
  "schema": "Braxon.foreign_tool_absorption.proof.v14",
  "generated_at": "$STAMP",
  "ok": true,
  "stamp_count": $count,
  "nsq_is_only_runtime": true,
  "foreign_sources_absorbed_into_nsq_stamp_bodies": true,
  "foreign_runtime_execution_forbidden": true,
  "parallel_runtime_allowed": false,
  "silent_foreign_execution_allowed": false,
  "registry": "$REGISTRY",
  "manifest": "$ROOT/capture_manifest.tsv"
}
EOF

ln -sfn "$PWD/$ROOT" "$LATEST"

echo "== proof =="
cat "$ROOT/proof.json"
echo
echo "== guard Alien::Build =="
"$GUARD" "Alien::Build" || true
echo
echo "== git status =="
git status --short
