#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
CLASSIFIER="$SCRIPT_DIR/rpi5-known-good-readiness-v3-classify.sh"
RETENTION_GUARD="$SCRIPT_DIR/rpi5-runtime-readiness-retention-guard.sh"
if [ -z "${TMPDIR:-}" ]; then
    TMP_ROOT="/opt/strider/openclaw/current/workspace/tmp"
else
    TMP_ROOT="$TMPDIR"
fi

SOURCE_DIR="$REPO_ROOT/tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof"
SOURCE_PRIMARY="$SOURCE_DIR/known-good-runtime-readiness-v2-20260610T2332Z-runtime-readiness-primary.json"
SOURCE_TFTP="$SOURCE_DIR/tftp-delta-stable.json"
SOURCE_PRE_STATUS="$SOURCE_DIR/pre-status.json"
SOURCE_FINAL_STATUS="$SOURCE_DIR/final-status.json"

mkdir -p "$TMP_ROOT"
WORK_DIR="$(TMPDIR="$TMP_ROOT" mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

primary="$WORK_DIR/fixture-retained-v2-runtime-readiness-primary.json"
tftp="$WORK_DIR/fixture-tftp-delta-stable.json"
pre_status="$WORK_DIR/fixture-pre-status.json"
final_status="$WORK_DIR/fixture-final-status.json"

cp "$SOURCE_PRIMARY" "$primary"
cp "$SOURCE_TFTP" "$tftp"
cp "$SOURCE_PRE_STATUS" "$pre_status"
cp "$SOURCE_FINAL_STATUS" "$final_status"

"$CLASSIFIER" "$primary" "$tftp" "$pre_status" "$final_status" >"$WORK_DIR/pass.json"
jq -e '
  .talos_runtime_readiness_v3.valid_known_good_talos_readiness_v3 == true and
  .talos_runtime_readiness_v3.classification == "valid-known-good-talos-readiness-v3" and
  .talos_runtime_readiness_v3.has_required_success_marker == true and
  .talos_runtime_readiness_v3.has_kernel_main == false and
  (.retained_risks | index("TALOS: kernel_main absent from retained primary serial window; v3 records this as metadata rather than a mandatory readiness marker") != null)
' "$WORK_DIR/pass.json" >/dev/null

missing_marker_primary="$WORK_DIR/fixture-missing-marker-runtime-readiness-primary.json"
jq '
  .text = "fixture output without the production success line" |
  .talos_runtime_readiness.has_required_success_marker = false |
  .talos_runtime_readiness.valid_known_good_talos_readiness = false |
  .talos_runtime_readiness.classification = "fixture-missing-success-marker"
' "$primary" >"$missing_marker_primary"

missing_marker_rc=0
"$CLASSIFIER" "$missing_marker_primary" "$tftp" "$pre_status" "$final_status" >"$WORK_DIR/missing-marker.json" || missing_marker_rc=$?
if [ "$missing_marker_rc" -ne 1 ]; then
    echo "expected missing-marker fixture to exit 1, got $missing_marker_rc" >&2
    exit 1
fi
jq -e '
  .talos_runtime_readiness_v3.valid_known_good_talos_readiness_v3 == false and
  (.rejection_reasons | index("missing-production-success-marker") != null)
' "$WORK_DIR/missing-marker.json" >/dev/null

missing_join_tftp="$WORK_DIR/fixture-missing-join-tftp.json"
missing_join_final_status="$WORK_DIR/fixture-missing-join-final-status.json"
jq '.talos_tftp_stability.stable = false | .talos_tftp_stability.reason = "fixture-unstable"' "$tftp" >"$missing_join_tftp"
jq '.boot.tree_hash = "fixture-different-tree-hash"' "$final_status" >"$missing_join_final_status"

missing_join_rc=0
"$CLASSIFIER" "$primary" "$missing_join_tftp" "$pre_status" "$missing_join_final_status" >"$WORK_DIR/missing-join.json" || missing_join_rc=$?
if [ "$missing_join_rc" -ne 1 ]; then
    echo "expected missing identity/TFTP join fixture to exit 1, got $missing_join_rc" >&2
    exit 1
fi
jq -e '
  .talos_runtime_readiness_v3.valid_known_good_talos_readiness_v3 == false and
  (.rejection_reasons | index("missing-or-unstable-boot-identity-join") != null) and
  (.rejection_reasons | index("missing-stable-tftp-delta") != null)
' "$WORK_DIR/missing-join.json" >/dev/null

"$RETENTION_GUARD" >"$WORK_DIR/retention-guard.json"
jq -e '
  .classification == "runtime-readiness-primary-retention-overwrite-prevented" and
  .helper_invocations == 1 and
  .overwrite_attempt_exit_code == 3
' "$WORK_DIR/retention-guard.json" >/dev/null

jq -n \
  --slurpfile pass "$WORK_DIR/pass.json" \
  --slurpfile missing_marker "$WORK_DIR/missing-marker.json" \
  --slurpfile missing_join "$WORK_DIR/missing-join.json" \
  --slurpfile retention_guard "$WORK_DIR/retention-guard.json" \
  '{
    classification: "known-good-readiness-v3-fixtures-pass",
    cases: {
      retained_v2_primary_with_joined_identity_tftp: $pass[0].talos_runtime_readiness_v3.classification,
      missing_success_marker_rejection: $missing_marker[0].rejection_reasons,
      missing_identity_tftp_join_rejection: $missing_join[0].rejection_reasons,
      primary_artifact_overwrite_prevention: $retention_guard[0].classification
    }
  }'
