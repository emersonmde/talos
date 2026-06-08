#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-proof-identity-join-v3-check.sh --evidence-dir DIR [--label LABEL]

Replays a retained Pi 5 capture-invariant proof bundle and checks the v3
freshness contract. V3 keeps the v2 identity/TFTP/final-restore join, but a
saturated direct-read serial window may be decisive without an empty pre-power
drain only when the required post-power marker is absent from every retained
pre-power drain response and present in the post-power serial window.
EOF
}

EVIDENCE_DIR=
EXPECTED_LABEL=

while [ "$#" -gt 0 ]; do
    case "$1" in
        --evidence-dir)
            EVIDENCE_DIR="${2:-}"
            shift 2
            ;;
        --label)
            EXPECTED_LABEL="${2:-}"
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

if [ -z "$EVIDENCE_DIR" ]; then
    usage
    exit 2
fi

require_file() {
    if [ ! -f "$EVIDENCE_DIR/$1" ]; then
        echo "missing required proof bundle file: $EVIDENCE_DIR/$1" >&2
        exit 2
    fi
}

require_file preflight-identity.json
require_file serial-drain-before-power.json
require_file serial-observe-window.json
require_file tftp-delta-stable-pre-restore.json
require_file final-pre-restore-status.json
require_file final-pre-restore-boot-files.json
require_file restore-snapshot.json
require_file post-restore-status.json

serial_cursor_file=""
tftp_cursor_file=""
if [ -f "$EVIDENCE_DIR/serial-cursor-before-power.txt" ]; then
    serial_cursor_file="$(cat "$EVIDENCE_DIR/serial-cursor-before-power.txt")"
fi
if [ -f "$EVIDENCE_DIR/tftp-cursor-before-power.txt" ]; then
    tftp_cursor_file="$(cat "$EVIDENCE_DIR/tftp-cursor-before-power.txt")"
fi

summary_file=/dev/null
if [ -f "$EVIDENCE_DIR/capture-invariant-summary.json" ]; then
    summary_file="$EVIDENCE_DIR/capture-invariant-summary.json"
fi

output="$(jq -n \
    --arg expected_label "$EXPECTED_LABEL" \
    --arg serial_cursor_file "$serial_cursor_file" \
    --arg tftp_cursor_file "$tftp_cursor_file" \
    --slurpfile summary "$summary_file" \
    --slurpfile drain_summary "$EVIDENCE_DIR/serial-drain-before-power.json" \
    --slurpfile preflight "$EVIDENCE_DIR/preflight-identity.json" \
    --slurpfile serial "$EVIDENCE_DIR/serial-observe-window.json" \
    --slurpfile tftp "$EVIDENCE_DIR/tftp-delta-stable-pre-restore.json" \
    --slurpfile final_status "$EVIDENCE_DIR/final-pre-restore-status.json" \
    --slurpfile final_files "$EVIDENCE_DIR/final-pre-restore-boot-files.json" \
    --slurpfile restore "$EVIDENCE_DIR/restore-snapshot.json" \
    --slurpfile post_restore "$EVIDENCE_DIR/post-restore-status.json" \
    '
    ($summary[0] // {}) as $summary_doc
    | ($drain_summary[0] // {}) as $drain_doc
    | ($summary_doc.label // "") as $summary_label
    | (if $expected_label != "" then $expected_label else $summary_label end) as $run_label
    | ($summary_doc.expected.fetch // $preflight[0].expected_fetch // "") as $expected_fetch
    | (($summary_doc.expected.fetch_bytes // $preflight[0].expected_fetch_bytes) | tonumber?) as $expected_bytes
    | ($drain_doc.talos_serial_drain // {}) as $sd
    | ($serial[0].talos_serial_window // {}) as $sw
    | ($tftp[0].talos_tftp_stability // {}) as $ts
    | (($sw.required_marker // "") | tostring) as $required_marker
    | (($drain_doc.responses // []) | map(.text // "") | join("")) as $pre_power_serial_text
    | (($serial[0].text // "") | tostring) as $post_power_serial_text
    | (if $required_marker == "" then 0 else (($pre_power_serial_text | split($required_marker) | length) - 1) end) as $pre_power_marker_count
    | (if $required_marker == "" then 0 else (($post_power_serial_text | split($required_marker) | length) - 1) end) as $post_power_marker_count
    | (($sw.capture_mode // "") == "read" and ($sw.start_cursor_saturated // false) == true) as $saturated_direct_read
    | (($sd.empty_before_power // false) == true) as $empty_pre_power
    | ($saturated_direct_read and ($empty_pre_power != true) and $required_marker != "" and $pre_power_marker_count == 0 and $post_power_marker_count > 0) as $fresh_marker_differential
    | ($empty_pre_power or $fresh_marker_differential) as $serial_freshness_ok
    | (($tftp[0].tftp.events // []) | map(select(.filename == $expected_fetch and .status == "served"))) as $fetch_events
    | ($fetch_events | map(select(.bytes == $expected_bytes))) as $matching_fetch_events
    | (($final_files[0].boot.files // []) | map(select(.name == $expected_fetch)) | first) as $final_fetch
    | ($preflight[0].observed_tree_hash // null) as $selected_tree_hash
    | ($final_status[0].boot.tree_hash // null) as $final_tree_hash
    | ($final_status[0].boot.effective_kernel // null) as $final_kernel
    | ($post_restore[0].boot.tree_hash // null) as $post_restore_tree_hash
    | ($selected_tree_hash != null and $final_tree_hash == $selected_tree_hash) as $final_selected_tree_ok
    | [
        (if $run_label == "" then "missing-run-label" else empty end),
        (if $expected_label != "" and $summary_label != "" and $summary_label != $expected_label then "summary-label-mismatch" else empty end),
        (if ($preflight[0].staging_publication_mismatch // false) then "preflight-staging-publication-mismatch" else empty end),
        (if ($selected_tree_hash // "") == "" then "missing-selected-tree-hash" else empty end),
        (if ($preflight[0].effective_kernel_matches // false) != true then "effective-kernel-mismatch" else empty end),
        (if ($expected_fetch // "") == "" then "missing-expected-fetch-path" else empty end),
        (if $expected_bytes == null then "missing-expected-fetch-byte-count" else empty end),
        (if ($preflight[0].expected_fetch_present // false) != true then "preflight-expected-fetch-missing" else empty end),
        (if ($preflight[0].expected_fetch_bytes_match // false) != true then "preflight-expected-fetch-byte-mismatch" else empty end),
        (if (($sd.contract_version // "") != "pi5-capture-transaction-v2") then "missing-v2-serial-drain-contract" else empty end),
        (if ($serial_freshness_ok != true) then "v3-serial-freshness-not-proven" else empty end),
        (if ($saturated_direct_read and ($empty_pre_power != true) and $required_marker == "") then "missing-required-marker-for-v3-differential" else empty end),
        (if ($saturated_direct_read and ($empty_pre_power != true) and $pre_power_marker_count > 0) then "required-marker-present-before-power" else empty end),
        (if ($saturated_direct_read and ($empty_pre_power != true) and $post_power_marker_count == 0) then "required-marker-not-present-after-power" else empty end),
        (if (($serial[0].cursor_start // null) == null and $serial_cursor_file == "") then "missing-serial-cursor-start" else empty end),
        (if (($serial[0].cursor_end // null) == null) then "missing-serial-cursor-end" else empty end),
        (if (($sw.capture_mode // "") == "") then "missing-serial-capture-mode" else empty end),
        (if (($sw.observe_contract // "") == "") then "missing-serial-window-contract" else empty end),
        (if ($tftp_cursor_file == "" and (($tftp[0].tftp.cursor_start // null) == null)) then "missing-tftp-cursor-start" else empty end),
        (if (($tftp[0].tftp.cursor_end // null) == null) then "missing-tftp-cursor-end" else empty end),
        (if ($ts.stable // false) != true then "tftp-delta-not-stable" else empty end),
        (if (($fetch_events | length) == 0) then "expected-fetch-not-observed-in-tftp-delta" else empty end),
        (if (($fetch_events | length) > 0 and ($matching_fetch_events | length) != ($fetch_events | length)) then "tftp-expected-fetch-byte-mismatch" else empty end),
        (if ($final_tree_hash // "") == "" then "missing-final-pre-restore-tree-hash" else empty end),
        (if $final_selected_tree_ok != true then "final-pre-restore-selected-tree-mismatch" else empty end),
        (if ($final_kernel != ($preflight[0].observed_effective_kernel // null)) then "final-pre-restore-effective-kernel-mismatch" else empty end),
        (if ($final_fetch == null) then "final-pre-restore-expected-fetch-missing" else empty end),
        (if ($expected_bytes != null and ($final_fetch.bytes // null) != $expected_bytes) then "final-pre-restore-expected-fetch-byte-mismatch" else empty end),
        (if ($restore[0].archive.name // "") == "" then "missing-restore-snapshot-name" else empty end),
        (if ($post_restore_tree_hash // "") == "" then "missing-post-restore-tree-hash" else empty end)
      ] as $rejection_reasons
    | {
        contract_version: "pi5-capture-transaction-v3",
        base_contract_version: "pi5-capture-transaction-v2",
        run_label: $run_label,
        summary_label: $summary_label,
        decisive_rp1_hardware_classification_allowed: (($rejection_reasons | length) == 0),
        classification: (if (($rejection_reasons | length) == 0) then "capture-transaction-v3-ready" else "capture-staging-blocked" end),
        rejection_reasons: $rejection_reasons,
        v3_serial_freshness: {
          rule: "empty pre-power drain, or saturated direct-read with required marker absent from all pre-power drain responses and present after power",
          empty_pre_power: $empty_pre_power,
          saturated_direct_read: $saturated_direct_read,
          required_marker: $required_marker,
          pre_power_marker_count: $pre_power_marker_count,
          post_power_marker_count: $post_power_marker_count,
          fresh_marker_differential: $fresh_marker_differential,
          serial_freshness_ok: $serial_freshness_ok
        },
        proof_run_identity: {
          selected_tree_hash: $selected_tree_hash,
          effective_kernel: ($preflight[0].observed_effective_kernel // null),
          expected_fetch_path: $expected_fetch,
          expected_fetch_byte_count: $expected_bytes,
          serial: {
            pre_power_drain: {
              attempts: ($sd.attempts // null),
              total_bytes: ($sd.total_bytes // null),
              final_cursor: ($sd.final_cursor // null),
              empty_before_power: ($sd.empty_before_power // false),
              discriminator: ($sd.discriminator // null)
            },
            cursor_file: (if $serial_cursor_file == "" then null else ($serial_cursor_file | tonumber?) end),
            window_cursor_start: ($serial[0].cursor_start // null),
            window_cursor_end: ($serial[0].cursor_end // null),
            window_bytes: ($serial[0].bytes // null),
            response_bytes: ($sw.response_bytes // null),
            capture_mode: ($sw.capture_mode // null),
            observe_contract: ($sw.observe_contract // null)
          },
          tftp: {
            cursor_file: (if $tftp_cursor_file == "" then null else ($tftp_cursor_file | tonumber?) end),
            delta_cursor_start: ($tftp[0].tftp.cursor_start // null),
            delta_cursor_end: ($tftp[0].tftp.cursor_end // null),
            stable: ($ts.stable // false),
            event_count: (($tftp[0].tftp.events // []) | length),
            expected_fetch_count: ($fetch_events | length),
            expected_fetch_byte_match_count: ($matching_fetch_events | length),
            expected_fetch_bytes_seen: ($fetch_events | map(.bytes))
          },
          final_pre_restore: {
            tree_hash: $final_tree_hash,
            effective_kernel: $final_kernel,
            selected_tree_still_staged: $final_selected_tree_ok,
            expected_fetch_present: ($final_fetch != null),
            expected_fetch_bytes: ($final_fetch.bytes // null)
          },
          restore: {
            snapshot_name: ($restore[0].archive.name // null),
            post_restore_tree_hash: $post_restore_tree_hash
          }
        },
        rule: "missing or mismatched candidate identity, v3 serial freshness, TFTP, final identity, or restore fields prevent decisive RP1 hardware classification"
      }
    ')"

printf '%s\n' "$output"

if printf '%s\n' "$output" | jq -e '.decisive_rp1_hardware_classification_allowed == true' >/dev/null; then
    exit 0
fi

exit 1
