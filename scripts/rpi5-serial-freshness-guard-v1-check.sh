#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-serial-freshness-guard-v1-check.sh --evidence-dir DIR
       [--label LABEL] [--nonce NONCE]

Replays a retained Pi 5 capture bundle without hardware and checks
cursor-nonce-post-power-freshness-v1. The guard accepts only when a
run-unique marker/nonce is absent from the immediate pre-power retained serial
sample, present after power from the saved cursor or saturated direct-read
fallback, and joined with selected-tree, TFTP, final identity, and restore
evidence.
EOF
}

EVIDENCE_DIR=
EXPECTED_LABEL=
EXPECTED_NONCE=

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
        --nonce)
            EXPECTED_NONCE="${2:-}"
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

case "$EXPECTED_NONCE" in
    *[!A-Za-z0-9_.:-]*)
        echo "--nonce may contain only A-Z, a-z, 0-9, _, ., :, and -" >&2
        exit 2
        ;;
esac

require_file() {
    if [ ! -f "$EVIDENCE_DIR/$1" ]; then
        echo "missing required serial freshness bundle file: $EVIDENCE_DIR/$1" >&2
        exit 2
    fi
}

require_file preflight-identity.json
require_file pre-power-serial-peek.json
require_file serial-drain-before-power.json
require_file serial-observe-window.json
require_file tftp-delta-stable-pre-restore.json
require_file final-pre-restore-status.json
require_file final-pre-restore-boot-files.json
require_file restore-snapshot.json
require_file post-restore-status.json

summary_file=/dev/null
if [ -f "$EVIDENCE_DIR/capture-invariant-summary.json" ]; then
    summary_file="$EVIDENCE_DIR/capture-invariant-summary.json"
fi

serial_cursor_file=""
tftp_cursor_file=""
if [ -f "$EVIDENCE_DIR/serial-cursor-before-power.txt" ]; then
    serial_cursor_file="$(cat "$EVIDENCE_DIR/serial-cursor-before-power.txt")"
fi
if [ -f "$EVIDENCE_DIR/tftp-cursor-before-power.txt" ]; then
    tftp_cursor_file="$(cat "$EVIDENCE_DIR/tftp-cursor-before-power.txt")"
fi

output="$(jq -n \
    --arg expected_label "$EXPECTED_LABEL" \
    --arg expected_nonce "$EXPECTED_NONCE" \
    --arg serial_cursor_file "$serial_cursor_file" \
    --arg tftp_cursor_file "$tftp_cursor_file" \
    --slurpfile summary "$summary_file" \
    --slurpfile preflight "$EVIDENCE_DIR/preflight-identity.json" \
    --slurpfile peek "$EVIDENCE_DIR/pre-power-serial-peek.json" \
    --slurpfile drain "$EVIDENCE_DIR/serial-drain-before-power.json" \
    --slurpfile serial "$EVIDENCE_DIR/serial-observe-window.json" \
    --slurpfile tftp "$EVIDENCE_DIR/tftp-delta-stable-pre-restore.json" \
    --slurpfile final_status "$EVIDENCE_DIR/final-pre-restore-status.json" \
    --slurpfile final_files "$EVIDENCE_DIR/final-pre-restore-boot-files.json" \
    --slurpfile restore "$EVIDENCE_DIR/restore-snapshot.json" \
    --slurpfile post_restore "$EVIDENCE_DIR/post-restore-status.json" \
    '
    def count_token($text; $token):
      if (($token // "") == "") then 0
      else ([((($text // "") | tostring | split($token) | length) - 1), 0] | max)
      end;

    ($summary[0] // {}) as $summary_doc
    | ($peek[0] // {}) as $peek_doc
    | ($drain[0].talos_serial_drain // {}) as $sd
    | ($serial[0].talos_serial_window // {}) as $sw
    | ($tftp[0].talos_tftp_stability // {}) as $ts
    | ($summary_doc.label // "") as $summary_label
    | (if $expected_label != "" then $expected_label else $summary_label end) as $run_label
    | ($summary_doc.expected.fetch // $preflight[0].expected_fetch // "") as $expected_fetch
    | (($summary_doc.expected.fetch_bytes // $preflight[0].expected_fetch_bytes) | tonumber?) as $expected_bytes
    | (($sw.required_marker // "") | tostring) as $required_marker
    | (($required_marker | capture("capture-nonce=(?<nonce>[A-Za-z0-9_.:-]+)")? // {}) | .nonce // "") as $marker_nonce
    | (if $expected_nonce != "" then $expected_nonce else $marker_nonce end) as $required_nonce
    | (if $required_nonce == "" then "" else ("capture-nonce=" + $required_nonce) end) as $nonce_token
    | (($peek_doc.text // "") | tostring) as $pre_power_retained_text
    | (($serial[0].text // "") | tostring) as $post_power_serial_text
    | count_token($pre_power_retained_text; $required_marker) as $pre_power_marker_count
    | count_token($post_power_serial_text; $required_marker) as $post_power_marker_count
    | count_token($pre_power_retained_text; $nonce_token) as $pre_power_nonce_count
    | count_token($post_power_serial_text; $nonce_token) as $post_power_nonce_count
    | (($peek_doc.cursor // null) | tonumber?) as $pre_power_peek_cursor
    | (($sd.final_cursor // null) | tonumber?) as $final_drain_cursor
    | (($serial[0].cursor_start // null) | tonumber?) as $post_power_cursor_start
    | (($serial[0].cursor_end // null) | tonumber?) as $post_power_cursor_end
    | (($tftp[0].tftp.events // []) | map(select(.filename == $expected_fetch and .status == "served"))) as $fetch_events
    | ($fetch_events | map(.bytes)) as $fetch_bytes_seen
    | ($fetch_events | map(select(.bytes == $expected_bytes))) as $matching_fetch_events
    | (($final_files[0].boot.files // []) | map(select(.name == $expected_fetch)) | first) as $final_fetch
    | ($preflight[0].observed_tree_hash // $preflight[0].selected_tree_hash // null) as $selected_tree_hash
    | ($preflight[0].observed_effective_kernel // null) as $selected_kernel
    | ($final_status[0].boot.tree_hash // null) as $final_tree_hash
    | ($final_status[0].boot.effective_kernel // null) as $final_kernel
    | ($post_restore[0].boot.tree_hash // null) as $post_restore_tree_hash
    | ($selected_tree_hash != null and $final_tree_hash == $selected_tree_hash) as $final_selected_tree_ok
    | (($sd.empty_before_power // false) == true) as $empty_pre_power
    | (($sw.capture_mode // "") == "read" and ($sw.start_cursor_saturated // false) == true) as $saturated_direct_read
    | (($sw.capture_mode // "") == "observe") as $cursor_observe
    | ($required_nonce != "" and $pre_power_nonce_count == 0 and $post_power_nonce_count > 0) as $nonce_fresh
    | ($required_marker != "" and $pre_power_marker_count == 0 and $post_power_marker_count > 0) as $marker_fresh
    | ($cursor_observe and $final_drain_cursor != null and $post_power_cursor_start == $final_drain_cursor and $post_power_cursor_end != null and $post_power_cursor_end >= $post_power_cursor_start) as $observe_cursor_bound
    | ($saturated_direct_read and ($sw.start_cursor_saturated // false) == true and (($sw.response_bytes // 0) > 0)) as $saturated_direct_read_bound
    | ($observe_cursor_bound or $saturated_direct_read_bound) as $post_power_capture_bound
    | ($nonce_fresh and $marker_fresh and $post_power_capture_bound) as $cursor_nonce_fresh
    | ($empty_pre_power or $cursor_nonce_fresh) as $serial_freshness_ok
    | [
        (if $run_label == "" then "missing-run-label" else empty end),
        (if $expected_label != "" and $summary_label != "" and $summary_label != $expected_label then "summary-label-mismatch" else empty end),
        (if ($preflight[0].staging_publication_mismatch // false) then "selected-tree-tftp-mismatch" else empty end),
        (if ($selected_tree_hash // "") == "" then "selected-tree-tftp-mismatch" else empty end),
        (if ($selected_kernel // "") == "" then "selected-tree-tftp-mismatch" else empty end),
        (if ($expected_fetch // "") == "" then "selected-tree-tftp-mismatch" else empty end),
        (if $expected_bytes == null then "selected-tree-tftp-mismatch" else empty end),
        (if ($preflight[0].expected_fetch_present // false) != true then "selected-tree-tftp-mismatch" else empty end),
        (if ($preflight[0].expected_fetch_bytes_match // false) != true then "selected-tree-tftp-mismatch" else empty end),
        (if (($sd.contract_version // "") | startswith("pi5-capture-transaction-") | not) then "inconclusive-capture" else empty end),
        (if ($pre_power_peek_cursor == null) then "inconclusive-capture" else empty end),
        (if ($final_drain_cursor == null) then "inconclusive-capture" else empty end),
        (if ($pre_power_peek_cursor != null and $final_drain_cursor != null and $final_drain_cursor < $pre_power_peek_cursor) then "cursor-mismatch" else empty end),
        (if $required_marker == "" then "missing-marker" else empty end),
        (if $post_power_marker_count == 0 then "missing-marker" else empty end),
        (if $required_nonce == "" then "nonce-not-unique" else empty end),
        (if $expected_nonce != "" and $marker_nonce != "" and $marker_nonce != $expected_nonce then "nonce-not-unique" else empty end),
        (if $required_nonce != "" and $pre_power_nonce_count > 0 then "stale-backlog" else empty end),
        (if $required_marker != "" and $pre_power_marker_count > 0 then "stale-backlog" else empty end),
        (if $required_nonce != "" and $post_power_nonce_count == 0 then "missing-marker" else empty end),
        (if ($post_power_capture_bound != true) then "cursor-mismatch" else empty end),
        (if ($saturated_direct_read and $nonce_fresh != true) then "saturated-direct-read-without-nonce-proof" else empty end),
        (if ($tftp_cursor_file == "" and (($tftp[0].tftp.cursor_start // null) == null)) then "selected-tree-tftp-mismatch" else empty end),
        (if (($tftp[0].tftp.cursor_end // null) == null) then "selected-tree-tftp-mismatch" else empty end),
        (if ($ts.stable // false) != true then "selected-tree-tftp-mismatch" else empty end),
        (if (($fetch_events | length) == 0) then "selected-tree-tftp-mismatch" else empty end),
        (if (($fetch_events | length) > 0 and ($matching_fetch_events | length) != ($fetch_events | length)) then "selected-tree-tftp-mismatch" else empty end),
        (if ($final_tree_hash // "") == "" then "final-identity-mismatch" else empty end),
        (if $final_selected_tree_ok != true then "final-identity-mismatch" else empty end),
        (if ($final_kernel != $selected_kernel) then "final-identity-mismatch" else empty end),
        (if ($final_fetch == null) then "final-identity-mismatch" else empty end),
        (if ($expected_bytes != null and ($final_fetch.bytes // null) != $expected_bytes) then "final-identity-mismatch" else empty end),
        (if ($restore[0].archive.name // "") == "" then "restore-failure" else empty end),
        (if ($post_restore_tree_hash // "") == "" then "restore-failure" else empty end)
      ] as $rejection_reasons
    | {
        contract_id: "phase12-rp1-ethernet-serial-freshness-contract-v1",
        contract_version: "cursor-nonce-post-power-freshness-v1",
        run_label: $run_label,
        serial_freshness_proven: (($rejection_reasons | length) == 0),
        classification: (if (($rejection_reasons | length) == 0) then "serial-freshness-guard-v1-ready" else "serial-freshness-guard-v1-rejected" end),
        rejection_reasons: ($rejection_reasons | unique),
        freshness_fields: {
            pre_power_peek_cursor: $pre_power_peek_cursor,
            pre_power_retained_bytes: ($peek_doc.bytes // (($peek_doc.text // "") | length)),
            pre_power_marker_occurrences: $pre_power_marker_count,
            pre_power_nonce_occurrences: $pre_power_nonce_count,
            serial_drain_attempts: ($sd.attempts // null),
            serial_drain_terminal_state: ($sd.discriminator // null),
            serial_drain_empty_before_power: $empty_pre_power,
            serial_drain_final_cursor: $final_drain_cursor,
            post_power_observe_cursor_start: $post_power_cursor_start,
            post_power_observe_cursor_end: $post_power_cursor_end,
            serial_capture_mode: ($sw.capture_mode // null),
            observe_contract: ($sw.observe_contract // null),
            start_cursor_saturated: ($sw.start_cursor_saturated // false),
            observed_byte_range_or_response_bytes: (if $saturated_direct_read then ($sw.response_bytes // null) else (if $post_power_cursor_start != null and $post_power_cursor_end != null then ($post_power_cursor_end - $post_power_cursor_start) else null end) end),
            required_marker: $required_marker,
            marker_occurrences_after_saved_cursor: $post_power_marker_count,
            run_unique_nonce: (if $required_nonce == "" then null else $required_nonce end),
            nonce_occurrences_after_saved_cursor: $post_power_nonce_count,
            cursor_nonce_fresh: $cursor_nonce_fresh,
            serial_freshness_ok: $serial_freshness_ok
        },
        proof_join: {
            selected_tree_identity: {
                tree_hash: $selected_tree_hash,
                effective_kernel: $selected_kernel,
                expected_kernel_path: $expected_fetch,
                expected_kernel_bytes: $expected_bytes
            },
            tftp: {
                cursor_file: (if $tftp_cursor_file == "" then null else ($tftp_cursor_file | tonumber?) end),
                cursor_start: ($tftp[0].tftp.cursor_start // null),
                cursor_end: ($tftp[0].tftp.cursor_end // null),
                stable: ($ts.stable // false),
                expected_fetch_count: ($fetch_events | length),
                expected_fetch_byte_match_count: ($matching_fetch_events | length),
                expected_fetch_bytes_seen: $fetch_bytes_seen
            },
            final_pre_restore_identity: {
                tree_hash: $final_tree_hash,
                effective_kernel: $final_kernel,
                selected_tree_still_staged: $final_selected_tree_ok,
                expected_fetch_present: ($final_fetch != null),
                expected_fetch_bytes: ($final_fetch.bytes // null)
            },
            restore_proof: {
                snapshot_name: ($restore[0].archive.name // null),
                post_restore_tree_hash: $post_restore_tree_hash
            }
        },
        claim_boundary: {
            accepted_if_ready: "serial freshness and capture-chain identity only",
            rejected_runtime_claims: [
                "BCM54213PE register values",
                "link readiness",
                "Ethernet readiness",
                "GPIO32 or PHY reset ownership",
                "BMCR or autoneg writes",
                "Broadcom shadow/MMD/AUX access",
                "interrupt ownership",
                "packet I/O",
                "networking",
                "SSH",
                "Phase 12.2",
                "phase transition"
            ]
        },
        rule: "missing or mismatched pre-power peek absence, post-power cursor/nonce proof, selected-tree/TFTP identity, final identity, or restore evidence rejects serial freshness"
      }
    ')"

printf '%s\n' "$output"

if printf '%s\n' "$output" | jq -e '.serial_freshness_proven == true' >/dev/null; then
    exit 0
fi

exit 1
