#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"

stamp="$(date -u +%Y%m%d_%H%M%S)"
iso="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

out_dir="state/nsq/offline_audit"
proof_dir="state/nsq/proofs"
mkdir -p "$out_dir" "$proof_dir"

hits_tsv="$out_dir/offline_dependency_hits_${stamp}.tsv"
latest_hits="$out_dir/offline_dependency_hits_latest.tsv"
summary="$out_dir/offline_dependency_summary_${stamp}.txt"
latest_summary="$out_dir/offline_dependency_summary_latest.txt"
proof="$proof_dir/offline_runtime_dependency_audit_${stamp}.json"
latest_proof="$proof_dir/offline_runtime_dependency_audit_latest.json"

allowed_paths_regex='^(\.git/|target/|\.cargo/|state/nsq/model_ingress/|state/nsq/stamps/|state/nsq/cpan/|docs/|specs/|config/nsq/model_transfer_nsq_only_policy\.json$|tools/nsq/audit_offline_runtime_dependency\.sh$)'
scan_regex='(https?://|git@|git clone|git fetch|git pull|git-lfs|git lfs|huggingface|hf_hub|from_pretrained|snapshot_download|curl[[:space:]]|wget[[:space:]]|pip install|python -m pip|npm install|pnpm install|yarn add|cargo install|cargo update|cpanm|cpan[[:space:]]|ssh[[:space:]].*@|rsync[[:space:]].*:|socket|TcpStream|UdpSocket|reqwest|ureq|hyper::|axum::Server|tokio::net|std::net)'

printf 'severity\tpath\tline\tmatch\tclassification\n' > "$hits_tsv"

while IFS=: read -r path line text; do
  [ -n "${path:-}" ] || continue
  classification="runtime_review_required"
  severity="WARN"
  clean_path="${path#./}"

  if printf '%s' "$clean_path" | grep -Eq "$allowed_paths_regex"; then
    classification="allowed_manifest_or_provenance_or_audit_surface"
    severity="INFO"
  fi

  case "$text" in
    *"normal_command\": \"request\""*|*"forbidden_normal_command\": \"fetch\""*|*"runtime_must_not_require"*|*"source_edge"*|*"Offline is not an optional feature"*)
      classification="offline_boundary_policy"
      severity="INFO"
      ;;
  esac

  safe_text="$(printf '%s' "$text" | tr '\t' ' ' | sed 's/[[:space:]][[:space:]]*/ /g' | cut -c 1-240)"
  printf '%s\t%s\t%s\t%s\t%s\n' "$severity" "$clean_path" "$line" "$safe_text" "$classification" >> "$hits_tsv"
done < <(
  grep -RInE "$scan_regex" . \
    --exclude-dir=.git \
    --exclude-dir=target \
    --exclude-dir=.cargo \
    --exclude-dir=node_modules \
    --exclude-dir=.venv \
    --exclude-dir=venv \
    2>/dev/null || true
)

cp "$hits_tsv" "$latest_hits"

warn_count="$(awk -F '\t' 'NR>1 && $1=="WARN" {c++} END{print c+0}' "$hits_tsv")"
info_count="$(awk -F '\t' 'NR>1 && $1=="INFO" {c++} END{print c+0}' "$hits_tsv")"
total_count="$(awk 'NR>1 {c++} END{print c+0}' "$hits_tsv")"

state="offline_clean_or_policy_only"
if [ "$warn_count" -gt 0 ]; then
  state="offline_dependency_review_required"
fi

cat > "$summary" <<EOF
schema=Braxon.nsq.offline_runtime_dependency_audit.v1
authority=NSQ_COURT
generated_at=$iso
state=$state
total_hits=$total_count
info_hits=$info_count
warn_hits=$warn_count
hits_tsv=$hits_tsv
rule=normal runtime must work completely offline after first source-edge translation/materialization
mission=offline access for users with scarce or no data, including people depending on limited government phones
EOF
cp "$summary" "$latest_summary"

cat > "$proof" <<EOF
{
  "schema": "Braxon.nsq.offline_runtime_dependency_audit.v1",
  "authority": "NSQ_COURT",
  "generated_at": "$iso",
  "state": "$state",
  "normal_runtime_must_work_completely_offline": true,
  "source_edge_first_translation_may_contact_source": true,
  "post_materialization_online_dependency_allowed": false,
  "scarce_data_user_mission": true,
  "total_hits": $total_count,
  "info_hits": $info_count,
  "warn_hits": $warn_count,
  "hits_tsv": "$hits_tsv",
  "latest_hits_tsv": "$latest_hits",
  "summary": "$summary",
  "latest_summary": "$latest_summary",
  "review_rule": "WARN hits must be removed, moved behind source-edge first-translation gating, or explicitly classified as non-runtime provenance."
}
EOF
cp "$proof" "$latest_proof"

echo "=== offline runtime dependency audit ==="
cat "$summary"
echo
if [ "$warn_count" -gt 0 ]; then
  echo "=== WARN hits ==="
  awk -F '\t' 'NR>1 && $1=="WARN" {print $2 ":" $3 " :: " $4 " :: " $5}' "$hits_tsv" | sed -n '1,120p'
  exit 1
fi

echo "offline dependency audit clean"
