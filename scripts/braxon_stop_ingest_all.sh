#!/data/data/com.termux/files/usr/bin/bash
echo "disabled_for_safety=/data/data/com.termux/files/home/Braxon/scripts/braxon_stop_ingest_all.sh"
echo "backup=/data/data/com.termux/files/home/storage/shared/Download/nsq_repair_bundle_20260414_173628/quarantined_wrappers/braxon_stop_ingest_all.sh.disabled.20260414_173628"
echo "reason=wrapper was canceling/killing live ingress or relaunching unsafe install flow"
exit 1
