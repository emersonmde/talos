#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-capture-invariant-proof-bundle.sh [--dry-run]
       --evidence-dir DIR --restore-snapshot NAME --label LABEL
       [--expected-tree-hash HASH]
       [--expected-kernel NAME]
       [--expected-fetch PATH]
       [--expected-fetch-bytes BYTES]
       [--serial-marker TEXT]
       [--serial-drain-attempts COUNT]
       [--serial-drain-read-timeout SECONDS]
       [--serial-drain-settle-ms MS]
       [--serial-drain-max-bytes BYTES]
       [--serial-timeout SECONDS]
       [--settle-ms MS]
       [--max-bytes BYTES]
       [--tftp-timeout SECONDS]
       [--stable-samples COUNT]

Captures one Pi 5 proof bundle after the caller has staged the intended boot
tree. The script records boot identity, fresh serial/TFTP cursors, bounded
serial output, stable same-cursor TFTP delta before restore, final pre-restore
identity, restore evidence, and an annotated summary.
EOF
}

API_BASE="${TALOS_LAB_API:-http://talos-lab-api:8080}"
DRY_RUN=false
EVIDENCE_DIR=
RESTORE_SNAPSHOT=
LABEL=
EXPECTED_TREE_HASH=
EXPECTED_KERNEL="kernel_2712.img"
EXPECTED_FETCH="da591740/kernel_2712.img"
EXPECTED_FETCH_BYTES=
SERIAL_MARKER="rpi5-rp1-post-handoff-marker-reset"
SERIAL_DRAIN_ATTEMPTS=16
SERIAL_DRAIN_READ_TIMEOUT=1
SERIAL_DRAIN_SETTLE_MS=100
SERIAL_DRAIN_MAX_BYTES=65536
SERIAL_TIMEOUT=90
SETTLE_MS=1000
MAX_BYTES=65536
TFTP_TIMEOUT=90
STABLE_SAMPLES=3
MARKER_FAMILY="${TALOS_SERIAL_MARKER_FAMILY:-}"

while [ "$#" -gt 0 ]; do
    case "$1" in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --evidence-dir)
            EVIDENCE_DIR="${2:-}"
            shift 2
            ;;
        --restore-snapshot)
            RESTORE_SNAPSHOT="${2:-}"
            shift 2
            ;;
        --label)
            LABEL="${2:-}"
            shift 2
            ;;
        --expected-tree-hash)
            EXPECTED_TREE_HASH="${2:-}"
            shift 2
            ;;
        --expected-kernel)
            EXPECTED_KERNEL="${2:-}"
            shift 2
            ;;
        --expected-fetch)
            EXPECTED_FETCH="${2:-}"
            shift 2
            ;;
        --expected-fetch-bytes)
            EXPECTED_FETCH_BYTES="${2:-}"
            shift 2
            ;;
        --serial-marker)
            SERIAL_MARKER="${2:-}"
            shift 2
            ;;
        --serial-drain-attempts)
            SERIAL_DRAIN_ATTEMPTS="${2:-}"
            shift 2
            ;;
        --serial-drain-read-timeout)
            SERIAL_DRAIN_READ_TIMEOUT="${2:-}"
            shift 2
            ;;
        --serial-drain-settle-ms)
            SERIAL_DRAIN_SETTLE_MS="${2:-}"
            shift 2
            ;;
        --serial-drain-max-bytes)
            SERIAL_DRAIN_MAX_BYTES="${2:-}"
            shift 2
            ;;
        --serial-timeout)
            SERIAL_TIMEOUT="${2:-}"
            shift 2
            ;;
        --settle-ms)
            SETTLE_MS="${2:-}"
            shift 2
            ;;
        --max-bytes)
            MAX_BYTES="${2:-}"
            shift 2
            ;;
        --tftp-timeout)
            TFTP_TIMEOUT="${2:-}"
            shift 2
            ;;
        --stable-samples)
            STABLE_SAMPLES="${2:-}"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "unknown argument: $1" >&2
            usage
            exit 2
            ;;
    esac
done

if [ -z "$EVIDENCE_DIR" ] || [ -z "$RESTORE_SNAPSHOT" ] || [ -z "$LABEL" ]; then
    usage
    exit 2
fi

validate_positive_uint() {
    name="$1"
    value="$2"
    case "$value" in
        ''|*[!0-9]*)
            echo "$name must be a positive integer" >&2
            exit 2
            ;;
        0)
            echo "$name must be a positive integer" >&2
            exit 2
            ;;
    esac
}

validate_positive_uint --serial-drain-attempts "$SERIAL_DRAIN_ATTEMPTS"
validate_positive_uint --serial-drain-read-timeout "$SERIAL_DRAIN_READ_TIMEOUT"
validate_positive_uint --serial-drain-settle-ms "$SERIAL_DRAIN_SETTLE_MS"
validate_positive_uint --serial-drain-max-bytes "$SERIAL_DRAIN_MAX_BYTES"
validate_positive_uint --serial-timeout "$SERIAL_TIMEOUT"
validate_positive_uint --settle-ms "$SETTLE_MS"
validate_positive_uint --max-bytes "$MAX_BYTES"
validate_positive_uint --tftp-timeout "$TFTP_TIMEOUT"
validate_positive_uint --stable-samples "$STABLE_SAMPLES"

if [ "$DRY_RUN" = true ]; then
    jq -n \
        --arg evidence_dir "$EVIDENCE_DIR" \
        --arg restore_snapshot "$RESTORE_SNAPSHOT" \
        --arg proof_label "$LABEL" \
        --arg expected_tree_hash "$EXPECTED_TREE_HASH" \
        --arg expected_kernel "$EXPECTED_KERNEL" \
        --arg expected_fetch "$EXPECTED_FETCH" \
        --arg expected_fetch_bytes "$EXPECTED_FETCH_BYTES" \
        --arg serial_marker "$SERIAL_MARKER" \
        --arg marker_family "$MARKER_FAMILY" \
        --argjson serial_drain_attempts "$SERIAL_DRAIN_ATTEMPTS" \
        --argjson serial_drain_read_timeout "$SERIAL_DRAIN_READ_TIMEOUT" \
        --argjson serial_drain_settle_ms "$SERIAL_DRAIN_SETTLE_MS" \
        --argjson serial_drain_max_bytes "$SERIAL_DRAIN_MAX_BYTES" \
        --argjson serial_timeout "$SERIAL_TIMEOUT" \
        --argjson settle_ms "$SETTLE_MS" \
        --argjson max_bytes "$MAX_BYTES" \
        --argjson tftp_timeout "$TFTP_TIMEOUT" \
        --argjson stable_samples "$STABLE_SAMPLES" \
        '{dry_run: true,
          hardware_actions: [],
          would_write: {
              evidence_dir: $evidence_dir,
              pre_root_endpoint: "pre-root-endpoint.json",
              pre_root_endpoint_body: "pre-root-endpoint-body.txt",
              pre_root: "pre-root.json",
              pre_status: "pre-status.json",
              pre_boot_files: "pre-boot-files.json",
              pre_snapshots: "pre-snapshots.json",
              pre_power_serial_peek: "pre-power-serial-peek.json",
              serial_drain_before_power: "serial-drain-before-power.json",
              serial_read_empty_before_power: "serial-read-empty-before-power.json",
              tftp_cursor_before_power: "tftp-cursor-before-power.json",
              power_cycle: "power-cycle.json",
              serial_observe_window: "serial-observe-window.json",
              tftp_delta_stable_pre_restore: "tftp-delta-stable-pre-restore.json",
              capture_window_order: "capture-window-order.json",
              final_pre_restore_root_endpoint: "final-pre-restore-root-endpoint.json",
              final_pre_restore_root_endpoint_body: "final-pre-restore-root-endpoint-body.txt",
              final_pre_restore_root: "final-pre-restore-root.json",
              final_pre_restore_status: "final-pre-restore-status.json",
              final_pre_restore_boot_files: "final-pre-restore-boot-files.json",
              restore_snapshot: "restore-snapshot.json",
              post_restore_root_endpoint: "post-restore-root-endpoint.json",
              post_restore_root_endpoint_body: "post-restore-root-endpoint-body.txt",
              post_restore_root: "post-restore-root.json",
              post_restore_status: "post-restore-status.json",
              post_restore_boot_files: "post-restore-boot-files.json",
              capture_window_order_completion: "capture-window-order.json completed_at/helper_run_completed",
              summary: "capture-invariant-summary.json"
          },
          contract: {
              capture_chain_contract_version: "pi5-capture-chain-v4",
              "label": $proof_label,
              restore_snapshot: $restore_snapshot,
              expected_tree_hash: $expected_tree_hash,
              expected_kernel: $expected_kernel,
              expected_fetch: $expected_fetch,
              expected_fetch_bytes: $expected_fetch_bytes,
              proof_run_identity_contract: {
                  version: "pi5-capture-transaction-v2",
                  shared_run_label: $proof_label,
                  required_fields: [
                      "selected_tree_hash",
                      "selected_tree_identity_source",
                      "effective_kernel",
                      "expected_fetch_path",
                      "expected_fetch_byte_count",
                      "pre_power_serial_peek_cursor",
                      "pre_power_serial_retained_nonce_absence",
                      "pre_power_serial_drain_empty",
                      "serial_cursor_nonce_freshness",
                      "tftp_cursor_and_stable_delta",
                      "final_pre_restore_identity",
                      "restore_identity"
                  ],
                  endpoint_fallback_rule: "GET / is optional endpoint-semantics evidence; /boot/files is the authoritative selected-tree identity source when GET / is unavailable",
                  rejection_rule: "missing or mismatched candidate identity, serial-drain, serial-window, TFTP, or final identity fields prevent decisive hardware classification"
              },
              serial_marker: $serial_marker,
              pre_power_serial_drain_contract: {
                  discriminator: "empty-read-or-bounded-drain-exhausted",
                  attempts: $serial_drain_attempts,
                  read_timeout_seconds: $serial_drain_read_timeout,
                  settle_ms: $serial_drain_settle_ms,
                  max_bytes_per_read: $serial_drain_max_bytes,
                  acceptance_rule: "empty pre-power /serial/read is strong positive evidence; non-empty drain requires cursor-nonce-post-power-freshness-v1"
              },
              serial_freshness_contract: "cursor-nonce-post-power-freshness-v1",
              serial_observe_contract: "serial-window-helper-auto-observe-or-direct-read",
              saturated_cursor_fallback: "direct-/serial/read when the saved cursor is at TALOS_SERIAL_CURSOR_SATURATION_LIMIT",
              tftp_contract: "stable-same-cursor-delta-before-restore",
              serial_marker_family: (if $marker_family == "" then [$serial_marker]
                  else ($marker_family | split("|") | map(select(. != ""))) end),
              serial_timeout_seconds: $serial_timeout,
              settle_ms: $settle_ms,
              max_bytes: $max_bytes,
              tftp_timeout_seconds: $tftp_timeout,
              stable_samples: $stable_samples
          }}'
    exit 0
fi

mkdir -p "$EVIDENCE_DIR"

CAPTURE_WINDOW_ORDER="$EVIDENCE_DIR/capture-window-order.json"

jq -n \
    --arg run_label "$LABEL" \
    --arg restore_snapshot "$RESTORE_SNAPSHOT" \
    '{
        contract_version: "pi5-candidate-capture-window-v5",
        helper: "scripts/rpi5-capture-invariant-proof-bundle.sh",
        run_label: $run_label,
        restore_snapshot: $restore_snapshot,
        helper_run_started_at: (now | todate),
        helper_run_completed: false,
        completed_at: null,
        rule: "final-pre-restore identity and stable TFTP delta must be captured by this helper before restore; post-restore/control identity must never satisfy candidate pre-restore evidence; candidate-capture-ready also requires helper_run_completed=true from this foreground helper",
        events: []
    }' > "$CAPTURE_WINDOW_ORDER"

append_capture_window_event() {
    stage="$1"
    shift
    now="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    files_json="$(printf '%s\n' "$@" | jq -R -s 'split("\n")[:-1]')"
    order_tmp="${CAPTURE_WINDOW_ORDER}.tmp"
    seq="$(jq '(.events | length) + 1' "$CAPTURE_WINDOW_ORDER")"
    jq \
        --arg stage "$stage" \
        --arg captured_at "$now" \
        --argjson sequence "$seq" \
        --argjson evidence_files "$files_json" \
        '.events += [{
            sequence: $sequence,
            stage: $stage,
            captured_at: $captured_at,
            evidence_files: $evidence_files
        }]' "$CAPTURE_WINDOW_ORDER" > "$order_tmp"
    mv "$order_tmp" "$CAPTURE_WINDOW_ORDER"
}

mark_capture_window_completed() {
    now="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    order_tmp="${CAPTURE_WINDOW_ORDER}.tmp"
    jq \
        --arg completed_at "$now" \
        '.helper_run_completed = true
         | .completed_at = $completed_at
         | .completion_event_count = (.events | length)' \
        "$CAPTURE_WINDOW_ORDER" > "$order_tmp"
    mv "$order_tmp" "$CAPTURE_WINDOW_ORDER"
}

capture_root_endpoint() {
    endpoint_file="$1"
    meta_file="$2"
    body_tmp="$(mktemp)"
    code_tmp="$(mktemp)"
    if curl -sS -o "$body_tmp" -w '%{http_code}' "${API_BASE}/" > "$code_tmp"; then
        curl_exit=0
    else
        curl_exit=$?
    fi
    http_code="$(cat "$code_tmp")"
    cp "$body_tmp" "$endpoint_file"
    jq -n \
        --arg endpoint "GET /" \
        --arg http_code "$http_code" \
        --argjson curl_exit "$curl_exit" \
        --rawfile body "$body_tmp" \
        '{
            endpoint: $endpoint,
            curl_exit: $curl_exit,
            http_code: (if ($http_code | test("^[0-9]+$")) then ($http_code | tonumber) else null end),
            body_bytes: ($body | length),
            usable_for_selected_tree_identity:
                ($curl_exit == 0
                 and ($http_code | test("^2[0-9][0-9]$"))
                 and (($body | fromjson? | .boot.tree_hash? // "") != "")),
            selected_tree_identity_source: "/boot/files",
            fallback_used:
                (($curl_exit != 0)
                 or (($http_code | test("^2[0-9][0-9]$")) | not)
                 or (($body | fromjson? | .boot.tree_hash? // "") == "")),
            fallback_reason:
                (if $curl_exit != 0 then "root-endpoint-curl-failed"
                 elif (($http_code | test("^2[0-9][0-9]$")) | not) then "root-endpoint-http-non-2xx"
                 elif (($body | fromjson? | .boot.tree_hash? // "") == "") then "root-endpoint-missing-boot-tree"
                 else null end)
        }' > "$meta_file"
    rm -f "$body_tmp" "$code_tmp"
}

capture_root_endpoint "$EVIDENCE_DIR/pre-root-endpoint-body.txt" "$EVIDENCE_DIR/pre-root-endpoint.json"
curl -fsS "${API_BASE}/boot/files" > "$EVIDENCE_DIR/pre-root.json"
curl -fsS "${API_BASE}/status" > "$EVIDENCE_DIR/pre-status.json"
curl -fsS "${API_BASE}/boot/files" > "$EVIDENCE_DIR/pre-boot-files.json"
curl -fsS "${API_BASE}/boot/snapshots" > "$EVIDENCE_DIR/pre-snapshots.json"

jq -n \
    --arg expected_tree_hash "$EXPECTED_TREE_HASH" \
    --arg expected_kernel "$EXPECTED_KERNEL" \
    --arg expected_fetch "$EXPECTED_FETCH" \
    --arg expected_fetch_bytes "$EXPECTED_FETCH_BYTES" \
    --arg marker_family "$MARKER_FAMILY" \
    --slurpfile status "$EVIDENCE_DIR/pre-status.json" \
    --slurpfile files "$EVIDENCE_DIR/pre-boot-files.json" \
    --slurpfile root_endpoint "$EVIDENCE_DIR/pre-root-endpoint.json" \
    '(($expected_fetch_bytes | tonumber?) // null) as $expected_bytes
     | ($status[0].boot.tree_hash // null) as $tree_hash
     | ($status[0].boot.effective_kernel // null) as $effective_kernel
     | (($files[0].boot.files // []) | map(select(.name == $expected_fetch)) | first) as $fetch
     | {
         expected_tree_hash: $expected_tree_hash,
         observed_tree_hash: $tree_hash,
         expected_kernel: $expected_kernel,
         observed_effective_kernel: $effective_kernel,
         expected_fetch: $expected_fetch,
         expected_fetch_bytes: $expected_bytes,
         observed_fetch_bytes: ($fetch.bytes // null),
         selected_tree_identity_source: "/boot/files",
         root_endpoint: $root_endpoint[0],
         tree_matches: ($expected_tree_hash == "" or $tree_hash == $expected_tree_hash),
         effective_kernel_matches: ($effective_kernel == $expected_kernel),
         expected_fetch_present: ($fetch != null),
         expected_fetch_bytes_match: ($expected_bytes == null or ($fetch.bytes // null) == $expected_bytes),
         staging_publication_mismatch:
             (($expected_tree_hash != "" and $tree_hash != $expected_tree_hash)
              or ($effective_kernel != $expected_kernel)
              or ($fetch == null)
              or ($expected_bytes != null and ($fetch.bytes // null) != $expected_bytes))
       }' > "$EVIDENCE_DIR/preflight-identity.json"
append_capture_window_event preflight_identity \
    pre-root-endpoint.json pre-root.json pre-status.json pre-boot-files.json \
    pre-snapshots.json preflight-identity.json

if jq -e '.staging_publication_mismatch' "$EVIDENCE_DIR/preflight-identity.json" >/dev/null; then
    jq -n \
        --arg proof_label "$LABEL" \
        --slurpfile preflight "$EVIDENCE_DIR/preflight-identity.json" \
        '{"label": $proof_label,
          classification: "staging-publication-mismatch",
          reason: "pre-power-cycle boot identity did not match the expected staged tree",
          preflight_identity: $preflight[0]}' > "$EVIDENCE_DIR/capture-invariant-summary.json"
    exit 1
fi

curl -fsS "${API_BASE}/serial/peek?max_bytes=${SERIAL_DRAIN_MAX_BYTES}&drain=true" \
    > "$EVIDENCE_DIR/pre-power-serial-peek.json"

serial_drain_tmp="$(mktemp)"
serial_drain_last="$(mktemp)"
trap 'rm -f "$serial_drain_tmp" "$serial_drain_last"' EXIT
drain_attempts=0
drain_total_bytes=0
drain_empty=false
drain_cursor=0

while [ "$drain_attempts" -lt "$SERIAL_DRAIN_ATTEMPTS" ]; do
    drain_payload="$(jq -n \
        --argjson timeout_seconds "$SERIAL_DRAIN_READ_TIMEOUT" \
        --argjson settle_ms "$SERIAL_DRAIN_SETTLE_MS" \
        --argjson max_bytes "$SERIAL_DRAIN_MAX_BYTES" \
        '{timeout_seconds: $timeout_seconds, settle_ms: $settle_ms, max_bytes: $max_bytes}')"
    curl -fsS -X POST -H 'Content-Type: application/json' \
        --data "$drain_payload" \
        "${API_BASE}/serial/read" > "$serial_drain_last"
    cat "$serial_drain_last" >> "$serial_drain_tmp"
    printf '\n' >> "$serial_drain_tmp"
    drain_attempts=$((drain_attempts + 1))
    drain_bytes="$(jq -r '(.bytes // ((.text // "") | length))' "$serial_drain_last")"
    drain_cursor="$(jq -r '(.cursor // 0)' "$serial_drain_last")"
    drain_total_bytes=$((drain_total_bytes + drain_bytes))
    if [ "$drain_bytes" -eq 0 ]; then
        drain_empty=true
        break
    fi
done

cp "$serial_drain_last" "$EVIDENCE_DIR/serial-read-empty-before-power.json"
jq -s \
    --argjson attempts "$drain_attempts" \
    --argjson attempt_limit "$SERIAL_DRAIN_ATTEMPTS" \
    --argjson total_bytes "$drain_total_bytes" \
    --argjson final_cursor "$drain_cursor" \
    --argjson empty_before_power "$drain_empty" \
    --argjson read_timeout_seconds "$SERIAL_DRAIN_READ_TIMEOUT" \
    --argjson settle_ms "$SERIAL_DRAIN_SETTLE_MS" \
    --argjson max_bytes_per_read "$SERIAL_DRAIN_MAX_BYTES" \
    '{
        action: "serial drain before power",
        ok: true,
        responses: .,
        talos_serial_drain: {
            contract_version: "pi5-capture-transaction-v2",
            attempts: $attempts,
            attempt_limit: $attempt_limit,
            read_timeout_seconds: $read_timeout_seconds,
            settle_ms: $settle_ms,
            max_bytes_per_read: $max_bytes_per_read,
            total_bytes: $total_bytes,
            final_cursor: $final_cursor,
            empty_before_power: $empty_before_power,
            discriminator: (if $empty_before_power then "empty-read-before-power" else "bounded-drain-exhausted-before-power" end),
            rule: "empty pre-power /serial/read is strong positive evidence; non-empty drain requires cursor-nonce-post-power-freshness-v1"
        }
    }' "$serial_drain_tmp" > "$EVIDENCE_DIR/serial-drain-before-power.json"

curl -fsS "${API_BASE}/tftp/logs?max_bytes=1048576&limit=1" > "$EVIDENCE_DIR/tftp-cursor-before-power.json"
serial_cursor="$(jq -r '.talos_serial_drain.final_cursor' "$EVIDENCE_DIR/serial-drain-before-power.json")"
tftp_cursor="$(jq -r '.tftp.cursor_end' "$EVIDENCE_DIR/tftp-cursor-before-power.json")"
printf '%s\n' "$serial_cursor" > "$EVIDENCE_DIR/serial-cursor-before-power.txt"
printf '%s\n' "$tftp_cursor" > "$EVIDENCE_DIR/tftp-cursor-before-power.txt"
append_capture_window_event pre_power_cursors \
    pre-power-serial-peek.json serial-drain-before-power.json \
    serial-read-empty-before-power.json tftp-cursor-before-power.json \
    serial-cursor-before-power.txt tftp-cursor-before-power.txt

curl -fsS -X POST "${API_BASE}/power/cycle" > "$EVIDENCE_DIR/power-cycle.json"
append_capture_window_event power_cycle power-cycle.json

set +e
./scripts/rpi5-observe-serial-window.sh \
    "$serial_cursor" "$SERIAL_TIMEOUT" "$SETTLE_MS" "$MAX_BYTES" "$SERIAL_MARKER" \
    > "$EVIDENCE_DIR/serial-observe-window.json"
serial_exit="$?"
set -e
printf '%s\n' "$serial_exit" > "$EVIDENCE_DIR/serial-observe-window.exit"
append_capture_window_event serial_observe_window \
    serial-observe-window.json serial-observe-window.exit

set +e
./scripts/rpi5-wait-tftp-delta.sh "$tftp_cursor" "$TFTP_TIMEOUT" "$STABLE_SAMPLES" \
    > "$EVIDENCE_DIR/tftp-delta-stable-pre-restore.json"
tftp_exit="$?"
set -e
printf '%s\n' "$tftp_exit" > "$EVIDENCE_DIR/tftp-delta-stable-pre-restore.exit"
append_capture_window_event tftp_delta_stable_pre_restore \
    tftp-delta-stable-pre-restore.json tftp-delta-stable-pre-restore.exit

curl -fsS "${API_BASE}/status" > "$EVIDENCE_DIR/final-pre-restore-status.json"
capture_root_endpoint "$EVIDENCE_DIR/final-pre-restore-root-endpoint-body.txt" "$EVIDENCE_DIR/final-pre-restore-root-endpoint.json"
curl -fsS "${API_BASE}/boot/files" > "$EVIDENCE_DIR/final-pre-restore-root.json"
curl -fsS "${API_BASE}/boot/files" > "$EVIDENCE_DIR/final-pre-restore-boot-files.json"
append_capture_window_event final_pre_restore_identity \
    final-pre-restore-status.json final-pre-restore-root-endpoint.json \
    final-pre-restore-root-endpoint-body.txt final-pre-restore-root.json \
    final-pre-restore-boot-files.json
curl -fsS -X POST "${API_BASE}/boot/restore?name=${RESTORE_SNAPSHOT}" > "$EVIDENCE_DIR/restore-snapshot.json"
append_capture_window_event restore_snapshot restore-snapshot.json
capture_root_endpoint "$EVIDENCE_DIR/post-restore-root-endpoint-body.txt" "$EVIDENCE_DIR/post-restore-root-endpoint.json"
curl -fsS "${API_BASE}/boot/files" > "$EVIDENCE_DIR/post-restore-root.json"
curl -fsS "${API_BASE}/status" > "$EVIDENCE_DIR/post-restore-status.json"
curl -fsS "${API_BASE}/boot/files" > "$EVIDENCE_DIR/post-restore-boot-files.json"
append_capture_window_event post_restore_identity \
    post-restore-root-endpoint.json post-restore-root-endpoint-body.txt \
    post-restore-root.json post-restore-status.json post-restore-boot-files.json
mark_capture_window_completed

jq -n \
    --arg proof_label "$LABEL" \
    --arg restore_snapshot "$RESTORE_SNAPSHOT" \
    --arg expected_tree_hash "$EXPECTED_TREE_HASH" \
    --arg expected_kernel "$EXPECTED_KERNEL" \
    --arg expected_fetch "$EXPECTED_FETCH" \
    --arg expected_fetch_bytes "$EXPECTED_FETCH_BYTES" \
    --arg marker_family "$MARKER_FAMILY" \
    --argjson serial_exit "$serial_exit" \
    --argjson tftp_exit "$tftp_exit" \
    --slurpfile preflight "$EVIDENCE_DIR/preflight-identity.json" \
    --slurpfile pre_power_peek "$EVIDENCE_DIR/pre-power-serial-peek.json" \
    --slurpfile drain "$EVIDENCE_DIR/serial-drain-before-power.json" \
    --slurpfile serial "$EVIDENCE_DIR/serial-observe-window.json" \
    --slurpfile tftp "$EVIDENCE_DIR/tftp-delta-stable-pre-restore.json" \
    --slurpfile final_status "$EVIDENCE_DIR/final-pre-restore-status.json" \
    --slurpfile final_files "$EVIDENCE_DIR/final-pre-restore-boot-files.json" \
    --slurpfile restore "$EVIDENCE_DIR/restore-snapshot.json" \
    --slurpfile post_restore "$EVIDENCE_DIR/post-restore-status.json" \
    '(($expected_fetch_bytes | tonumber?) // null) as $expected_bytes
     | ($pre_power_peek[0] // {}) as $pp
     | ($serial[0].talos_serial_window // {}) as $sw
     | ($drain[0].talos_serial_drain // {}) as $sd
     | ($tftp[0].talos_tftp_stability // {}) as $ts
     | (($tftp[0].tftp.events // []) | map(select(.filename == $expected_fetch))) as $fetch_events
     | ($fetch_events | map(select(.status == "served" and .bytes == $expected_bytes))) as $matching_fetch_events
     | (($final_files[0].boot.files // []) | map(select(.name == $expected_fetch)) | first) as $final_fetch
     | ($final_status[0].boot.tree_hash // null) as $final_tree_hash
     | ($final_status[0].boot.effective_kernel // null) as $final_kernel
     | ($post_restore[0].boot.tree_hash // null) as $post_restore_tree_hash
     | ($preflight[0].staging_publication_mismatch // false) as $preflight_mismatch
     | ($preflight[0].observed_tree_hash // null) as $selected_tree_hash
     | ($expected_tree_hash == "" or $final_tree_hash == $expected_tree_hash) as $final_tree_ok
     | ($selected_tree_hash != null and $final_tree_hash == $selected_tree_hash) as $final_selected_tree_ok
     | ($expected_bytes == null or ($final_fetch.bytes // null) == $expected_bytes) as $final_bytes_ok
     | (($ts.stable == true) and (($tftp[0].tftp.events // []) | length) == 0) as $stable_zero_tftp
     | (($sw.required_marker // "") | tostring) as $required_marker
     | (($sw.marker_nonce // "") | tostring) as $marker_nonce
     | (if $marker_nonce == "" then "" else ("capture-nonce=" + $marker_nonce) end) as $nonce_token
     | (($pp.text // "") | tostring) as $pre_power_retained_text
     | (($serial[0].text // "") | tostring) as $post_power_serial_text
     | (if $marker_family == "" then []
        else ($marker_family | split("|") | map(select(. != "")))
        end) as $configured_marker_family
     | (if ($configured_marker_family | length) == 0 then [$required_marker]
        else $configured_marker_family
        end) as $markers
     | (if $required_marker == "" then 0 else (($pre_power_retained_text | split($required_marker) | length) - 1) end) as $pre_power_marker_count
     | (if $required_marker == "" then 0 else (($post_power_serial_text | split($required_marker) | length) - 1) end) as $post_power_marker_count
     | ($markers | map(. as $family_marker
        | {
            marker: $family_marker,
            pre_power_occurrences: (if $pre_power_retained_text == "" then 0 else (($pre_power_retained_text | split($family_marker) | length) - 1) end),
            post_power_occurrences: (if $post_power_serial_text == "" then 0 else (($post_power_serial_text | split($family_marker) | length) - 1) end)
          }
        | . + {fresh_after_saved_cursor: (.pre_power_occurrences == 0 and .post_power_occurrences > 0)})) as $family_freshness
     | (if $nonce_token == "" then 0 else (($pre_power_retained_text | split($nonce_token) | length) - 1) end) as $pre_power_nonce_count
     | (if $nonce_token == "" then 0 else (($post_power_serial_text | split($nonce_token) | length) - 1) end) as $post_power_nonce_count
     | (($pp.cursor // null) | tonumber?) as $pre_power_peek_cursor
     | (($sd.final_cursor // null) | tonumber?) as $final_drain_cursor
     | (($serial[0].cursor_start // null) | tonumber?) as $post_power_cursor_start
     | (($serial[0].cursor_end // null) | tonumber?) as $post_power_cursor_end
     | (($sw.capture_mode // "") == "read" and ($sw.start_cursor_saturated // false) == true) as $saturated_direct_read
     | (($sw.capture_mode // "") == "observe") as $cursor_observe
     | ($marker_nonce != "" and $pre_power_nonce_count == 0 and $post_power_nonce_count > 0) as $nonce_fresh
     | ($required_marker != "" and $pre_power_marker_count == 0 and $post_power_marker_count > 0) as $marker_fresh
     | ($cursor_observe and $final_drain_cursor != null and $post_power_cursor_start == $final_drain_cursor and $post_power_cursor_end != null and $post_power_cursor_end >= $post_power_cursor_start) as $observe_cursor_bound
     | ($saturated_direct_read and ($sw.start_cursor_saturated // false) == true and (($sw.response_bytes // 0) > 0)) as $saturated_direct_read_bound
     | ($observe_cursor_bound or $saturated_direct_read_bound) as $post_power_capture_bound
     | ($nonce_fresh and $marker_fresh and $post_power_capture_bound) as $cursor_nonce_fresh
     | (($family_freshness | map(select(.fresh_after_saved_cursor)) | length) > 0 and $post_power_capture_bound) as $marker_family_fresh
     | (($sd.empty_before_power // false) == true or $cursor_nonce_fresh or $marker_family_fresh) as $serial_freshness_ok
     | [
         (if $proof_label == "" then "missing-run-label" else empty end),
         (if $preflight_mismatch then "preflight-staging-publication-mismatch" else empty end),
         (if (($selected_tree_hash // "") == "") then "missing-selected-tree-hash" else empty end),
         (if ($preflight[0].effective_kernel_matches // false) != true then "effective-kernel-mismatch" else empty end),
         (if ($preflight[0].expected_fetch_present // false) != true then "preflight-expected-fetch-missing" else empty end),
         (if ($preflight[0].expected_fetch_bytes_match // false) != true then "preflight-expected-fetch-byte-mismatch" else empty end),
         (if (($sd.contract_version // "") != "pi5-capture-transaction-v2") then "missing-v2-serial-drain-contract" else empty end),
         (if $serial_freshness_ok != true then "serial-freshness-v1-not-proven" else empty end),
         (if (($serial[0].cursor_start // null) == null) then "missing-serial-cursor-start" else empty end),
         (if (($serial[0].cursor_end // null) == null) then "missing-serial-cursor-end" else empty end),
         (if (($sw.capture_mode // "") == "") then "missing-serial-capture-mode" else empty end),
         (if ($ts.stable // false) != true then "tftp-delta-not-stable" else empty end),
         (if (($tftp[0].tftp.cursor_start // null) == null) then "missing-tftp-cursor-start" else empty end),
         (if (($tftp[0].tftp.cursor_end // null) == null) then "missing-tftp-cursor-end" else empty end),
         (if (($fetch_events | length) == 0) then "expected-fetch-not-observed-in-tftp-delta" else empty end),
         (if (($fetch_events | length) > 0 and ($matching_fetch_events | length) != ($fetch_events | length)) then "tftp-expected-fetch-byte-mismatch" else empty end),
         (if ($final_tree_ok != true) then "final-pre-restore-tree-mismatch" else empty end),
         (if ($final_selected_tree_ok != true) then "final-pre-restore-selected-tree-mismatch" else empty end),
         (if ($final_fetch == null) then "final-pre-restore-expected-fetch-missing" else empty end),
         (if ($final_bytes_ok != true) then "final-pre-restore-expected-fetch-byte-mismatch" else empty end),
         (if ($post_restore_tree_hash == null) then "missing-post-restore-tree-hash" else empty end)
       ] as $identity_join_rejection_reasons
     | {
         "label": $proof_label,
         restore_snapshot: $restore_snapshot,
         expected: {
             tree_hash: $expected_tree_hash,
             effective_kernel: $expected_kernel,
             fetch: $expected_fetch,
             fetch_bytes: $expected_bytes
         },
         exits: {
             serial_observe_window: $serial_exit,
             tftp_delta_stable_pre_restore: $tftp_exit
         },
         preflight_identity: $preflight[0],
         capture_chain_contract_version: "pi5-capture-chain-v4",
         final_pre_restore_identity: {
             tree_hash: $final_tree_hash,
             effective_kernel: $final_kernel,
             expected_tree_still_staged: $final_tree_ok,
             expected_fetch_present: ($final_fetch != null),
             expected_fetch_bytes_match: $final_bytes_ok
         },
         serial_window: $sw,
         tftp_stability: $ts,
         tftp_event_count: (($tftp[0].tftp.events // []) | length),
         expected_fetch_count: ($fetch_events | length),
         expected_fetch_byte_match_count: ($matching_fetch_events | length),
         expected_fetch_events: $fetch_events,
         proof_run_identity: {
             contract_version: "pi5-capture-transaction-v2",
             shared_run_label: $proof_label,
             selected_tree_hash: $selected_tree_hash,
             effective_kernel: ($preflight[0].observed_effective_kernel // null),
             expected_fetch_path: $expected_fetch,
             expected_fetch_byte_count: $expected_bytes,
             serial: {
                 pre_power_peek: {
                     cursor: $pre_power_peek_cursor,
                     bytes: ($pp.bytes // null),
                     required_marker_occurrences: $pre_power_marker_count,
                     nonce_token_occurrences: $pre_power_nonce_count
                 },
                 pre_power_drain: {
                     attempts: ($sd.attempts // null),
                     attempt_limit: ($sd.attempt_limit // null),
                     read_timeout_seconds: ($sd.read_timeout_seconds // null),
                     settle_ms: ($sd.settle_ms // null),
                     max_bytes_per_read: ($sd.max_bytes_per_read // null),
                     total_bytes: ($sd.total_bytes // null),
                     final_cursor: ($sd.final_cursor // null),
                     empty_before_power: ($sd.empty_before_power // false),
                     discriminator: ($sd.discriminator // null)
                 },
                 cursor_start: ($serial[0].cursor_start // null),
                 cursor_end: ($serial[0].cursor_end // null),
                 window_bytes: ($serial[0].bytes // null),
                 response_bytes: ($sw.response_bytes // null),
                 capture_mode: ($sw.capture_mode // null),
                 observe_contract: ($sw.observe_contract // null),
                 required_marker: ($sw.required_marker // null),
                 freshness_contract: {
                     contract_version: "cursor-nonce-post-power-freshness-v1",
                     marker_nonce: (if $marker_nonce == "" then null else $marker_nonce end),
                     nonce_token: (if $nonce_token == "" then null else $nonce_token end),
                     marker_occurrences_after_saved_cursor: $post_power_marker_count,
                     nonce_occurrences_after_saved_cursor: $post_power_nonce_count,
                     post_power_capture_bound: $post_power_capture_bound,
                     cursor_nonce_fresh: $cursor_nonce_fresh,
                     marker_family_fresh: $marker_family_fresh,
                     marker_family_freshness: $family_freshness,
                     serial_freshness_ok: $serial_freshness_ok
                 }
             },
             tftp: {
                 cursor_start: ($tftp[0].tftp.cursor_start // null),
                 cursor_end: ($tftp[0].tftp.cursor_end // null),
                 stable: ($ts.stable // false),
                 event_count: (($tftp[0].tftp.events // []) | length),
                 expected_fetch_count: ($fetch_events | length),
                 expected_fetch_byte_match_count: ($matching_fetch_events | length)
             },
             final_pre_restore: {
                 tree_hash: $final_tree_hash,
                 effective_kernel: $final_kernel,
                 selected_tree_still_staged: $final_selected_tree_ok,
                 expected_fetch_present: ($final_fetch != null),
                 expected_fetch_bytes_match: $final_bytes_ok
             },
             restore: {
                 snapshot_name: ($restore[0].archive.name // null),
                 post_restore_tree_hash: $post_restore_tree_hash
             }
         },
         identity_join_contract: {
             contract_version: "pi5-capture-transaction-v2",
             decisive_rp1_hardware_classification_allowed: (($identity_join_rejection_reasons | length) == 0),
             rejection_reasons: $identity_join_rejection_reasons,
             rejected_classification: (if (($identity_join_rejection_reasons | length) == 0) then null else "capture-staging-blocked" end),
             rule: "missing or mismatched candidate identity, serial-window, or TFTP fields prevent decisive RP1 hardware classification"
         },
         restore: {
             snapshot: $restore[0],
             post_restore_tree_hash: $post_restore_tree_hash
         },
         suggested_classification:
             (
             if (($identity_join_rejection_reasons | length) > 0) then "identity-join-mismatch"
             elif $preflight_mismatch then "staging-publication-mismatch"
             elif (($sw.has_required_marker == true) or ($post_power_marker_count > 0) or $marker_family_fresh) then "post-handoff-marker-visible"
             elif (($fetch_events | length) >= 2 and (($sw.firmware_network_occurrences // 0) >= 2)) then "reset-side-effect-without-visible-marker-candidate"
             elif (($fetch_events | length) > 0) then "candidate-fetch-observed-without-marker"
             elif ($stable_zero_tftp and ($sw.has_firmware_network == true) and $final_tree_ok and ($final_fetch != null) and $final_bytes_ok) then "tftp-capture-logging-blindness"
             elif ($sw.has_firmware_network == true) then "serial-only-firmware-reboot"
             else "still-blocked-without-fresh-boot-evidence"
             end),
         proof_contract: {
             serial_freshness_contract: "cursor-nonce-post-power-freshness-v1",
             serial_observe_contract: ($sw.observe_contract // "deadline-loop-accumulated-from-fresh-cursor"),
             serial_capture_mode: ($sw.capture_mode // "observe"),
             serial_start_cursor_saturated: ($sw.start_cursor_saturated // false),
             tftp_contract: "stable-same-cursor-delta-before-restore",
             zero_event_tftp_meaningful_only_before_restore: true,
             firmware_network_is_not_talos_entry: true
         }
       }' > "$EVIDENCE_DIR/capture-invariant-summary.json"

jq . "$EVIDENCE_DIR/capture-invariant-summary.json"
