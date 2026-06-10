#!/bin/sh
set -eu

if [ "$#" -lt 3 ] || [ "$#" -gt 6 ]; then
    echo "usage: $0 <evidence_dir> <run_label> <serial_cursor> [timeout_seconds] [settle_ms] [max_bytes]" >&2
    exit 2
fi

EVIDENCE_DIR="$1"
RUN_LABEL="$2"
SERIAL_CURSOR="$3"
TIMEOUT_SECONDS="${4:-75}"
SETTLE_MS="${5:-1000}"
MAX_BYTES="${6:-65536}"
HELPER="${TALOS_RUNTIME_READINESS_HELPER:-scripts/rpi5-observe-runtime-readiness.sh}"

case "$EVIDENCE_DIR" in
    '')
        echo "evidence_dir must be non-empty" >&2
        exit 2
        ;;
esac

case "$RUN_LABEL" in
    ''|*/*|*..*|*[!A-Za-z0-9_.-]*)
        echo "run_label must contain only A-Za-z0-9_.- and must not contain path components" >&2
        exit 2
        ;;
esac

case "$SERIAL_CURSOR" in
    ''|*[!0-9]*)
        echo "serial_cursor must be a non-empty numeric /serial cursor" >&2
        exit 2
        ;;
esac

mkdir -p "$EVIDENCE_DIR"

PRIMARY_JSON="${EVIDENCE_DIR}/${RUN_LABEL}-runtime-readiness-primary.json"
PRIMARY_SUMMARY="${EVIDENCE_DIR}/${RUN_LABEL}-runtime-readiness-primary-summary.json"
STATUS_FILE="${EVIDENCE_DIR}/${RUN_LABEL}-runtime-readiness-primary.status"
TMP_JSON="${EVIDENCE_DIR}/.${RUN_LABEL}-runtime-readiness-primary.$$.tmp"
TMP_SUMMARY="${EVIDENCE_DIR}/.${RUN_LABEL}-runtime-readiness-primary-summary.$$.tmp"
TMP_STATUS="${EVIDENCE_DIR}/.${RUN_LABEL}-runtime-readiness-primary-status.$$.tmp"

cleanup() {
    rm -f "$TMP_JSON" "$TMP_SUMMARY" "$TMP_STATUS"
}
trap cleanup EXIT

for path in "$PRIMARY_JSON" "$PRIMARY_SUMMARY" "$STATUS_FILE"; do
    if [ -e "$path" ]; then
        echo "refusing to overwrite retained runtime-readiness primary artifact: $path" >&2
        exit 3
    fi
done

set +e
"$HELPER" "$SERIAL_CURSOR" "$TIMEOUT_SECONDS" "$SETTLE_MS" "$MAX_BYTES" >"$TMP_JSON"
HELPER_RC=$?
set -e

jq empty "$TMP_JSON"

printf '%s\n' "$HELPER_RC" >"$TMP_STATUS"
jq \
    --arg run_label "$RUN_LABEL" \
    --arg primary_artifact "$PRIMARY_JSON" \
    --arg status_artifact "$STATUS_FILE" \
    --argjson helper_exit_code "$HELPER_RC" \
    '{
        run_label: $run_label,
        retained_primary_artifact: $primary_artifact,
        status_artifact: $status_artifact,
        helper_exit_code: $helper_exit_code,
        derived_from_retained_primary_artifact: true,
        overwrite_policy: "refuse-existing-primary-summary-or-status",
        followup_artifact_policy: "follow-up direct-read or endpoint checks must use separately named artifacts and must not replace this primary artifact",
        talos_runtime_readiness: .talos_runtime_readiness
    }' "$TMP_JSON" >"$TMP_SUMMARY"

for path in "$PRIMARY_JSON" "$PRIMARY_SUMMARY" "$STATUS_FILE"; do
    if [ -e "$path" ]; then
        echo "refusing to overwrite retained runtime-readiness primary artifact: $path" >&2
        exit 3
    fi
done

mv "$TMP_JSON" "$PRIMARY_JSON"
mv "$TMP_SUMMARY" "$PRIMARY_SUMMARY"
mv "$TMP_STATUS" "$STATUS_FILE"

printf '%s\n' "$PRIMARY_JSON"
printf '%s\n' "$PRIMARY_SUMMARY"
printf '%s\n' "$STATUS_FILE"

exit "$HELPER_RC"
