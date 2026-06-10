#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
RETAIN_HELPER="${SCRIPT_DIR}/rpi5-retain-runtime-readiness-primary.sh"
TMP_ROOT="${TMPDIR:-/opt/strider/openclaw/current/workspace/tmp}"
mkdir -p "$TMP_ROOT"
WORK_DIR="$(TMPDIR="$TMP_ROOT" mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

HELPER="${WORK_DIR}/fixture-helper.sh"
STATE_FILE="${WORK_DIR}/helper-count"
EVIDENCE_DIR="${WORK_DIR}/evidence"
RUN_LABEL="retention-guard-run"

printf '0\n' >"$STATE_FILE"
cat >"$HELPER" <<'EOF'
#!/bin/sh
set -eu
state_file="${TALOS_FIXTURE_STATE_FILE:?}"
count="$(cat "$state_file")"
next=$((count + 1))
printf '%s\n' "$next" >"$state_file"
jq -n --arg text "fixture helper call ${next}" --argjson cursor_start "$1" --argjson next "$next" '{
  cursor_start: $cursor_start,
  cursor_end: ($cursor_start + $next),
  bytes: ($next * 10),
  text: $text,
  talos_runtime_readiness: {
    observe_contract: "fixture-primary-retention",
    capture_mode: "fixture",
    valid_known_good_talos_readiness: false,
    classification: ("fixture-call-" + ($next|tostring))
  }
}'
exit 1
EOF
chmod +x "$HELPER"

first_rc=0
TALOS_RUNTIME_READINESS_HELPER="$HELPER" \
TALOS_FIXTURE_STATE_FILE="$STATE_FILE" \
    "$RETAIN_HELPER" "$EVIDENCE_DIR" "$RUN_LABEL" 4194304 1 1 64 >"$WORK_DIR/first.stdout" || first_rc=$?

if [ "$first_rc" -ne 1 ]; then
    echo "expected first helper-retention run to return helper exit code 1, got $first_rc" >&2
    exit 1
fi

primary="${EVIDENCE_DIR}/${RUN_LABEL}-runtime-readiness-primary.json"
summary="${EVIDENCE_DIR}/${RUN_LABEL}-runtime-readiness-primary-summary.json"
status="${EVIDENCE_DIR}/${RUN_LABEL}-runtime-readiness-primary.status"

jq -e '.talos_runtime_readiness.classification == "fixture-call-1"' "$primary" >/dev/null
jq -e '.derived_from_retained_primary_artifact == true and .talos_runtime_readiness.classification == "fixture-call-1"' "$summary" >/dev/null
grep -qx '1' "$status"

second_rc=0
TALOS_RUNTIME_READINESS_HELPER="$HELPER" \
TALOS_FIXTURE_STATE_FILE="$STATE_FILE" \
    "$RETAIN_HELPER" "$EVIDENCE_DIR" "$RUN_LABEL" 4194304 1 1 64 >"$WORK_DIR/second.stdout" 2>"$WORK_DIR/second.stderr" || second_rc=$?

if [ "$second_rc" -ne 3 ]; then
    echo "expected overwrite attempt to return 3, got $second_rc" >&2
    exit 1
fi

jq -e '.talos_runtime_readiness.classification == "fixture-call-1"' "$primary" >/dev/null
jq -e '.talos_runtime_readiness.classification == "fixture-call-1"' "$summary" >/dev/null
grep -qx '1' "$STATE_FILE"
grep -q 'refusing to overwrite retained runtime-readiness primary artifact' "$WORK_DIR/second.stderr"

jq -n \
    --arg primary "$primary" \
    --arg summary "$summary" \
    --arg status "$status" \
    --arg first_stdout "$WORK_DIR/first.stdout" \
    --arg second_stderr "$WORK_DIR/second.stderr" \
    '{
        classification: "runtime-readiness-primary-retention-overwrite-prevented",
        retained_primary_artifact: $primary,
        retained_summary_artifact: $summary,
        retained_status_artifact: $status,
        first_stdout: $first_stdout,
        second_stderr: $second_stderr,
        helper_invocations: 1,
        overwrite_attempt_exit_code: 3
    }'
