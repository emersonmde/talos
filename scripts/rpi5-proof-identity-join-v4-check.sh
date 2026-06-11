#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-proof-identity-join-v4-check.sh --evidence-dir DIR
       [--label LABEL] [--report-kind candidate|control] [--nonce NONCE]

Replays a retained Pi 5 proof bundle without hardware and checks the repaired
pi5-capture-chain-v4 contract: /boot/files selected-tree identity, same-power-
cycle stable TFTP-served kernel byte identity, final pre-restore identity,
run-unique serial freshness, direct serial marker retention, and
candidate/control marker expectations. API-visible /boot/files identity is not
sufficient by itself.
EOF
}

EVIDENCE_DIR=
EXPECTED_LABEL=
REPORT_KIND=
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
        --report-kind)
            REPORT_KIND="${2:-}"
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

case "$REPORT_KIND" in
    ""|candidate|control)
        ;;
    *)
        echo "--report-kind must be candidate or control" >&2
        exit 2
        ;;
esac

case "$EXPECTED_NONCE" in
    *[!A-Za-z0-9_.:-]*)
        echo "--nonce may contain only A-Z, a-z, 0-9, _, ., :, and -" >&2
        exit 2
        ;;
esac

require_file() {
    if [ ! -f "$EVIDENCE_DIR/$1" ]; then
        echo "missing required v4 proof bundle file: $EVIDENCE_DIR/$1" >&2
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

root_endpoint_file=/dev/null
if [ -f "$EVIDENCE_DIR/pre-root-endpoint.json" ]; then
    root_endpoint_file="$EVIDENCE_DIR/pre-root-endpoint.json"
fi

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
    --arg report_kind_arg "$REPORT_KIND" \
    --arg expected_nonce "$EXPECTED_NONCE" \
    --arg serial_cursor_file "$serial_cursor_file" \
    --arg tftp_cursor_file "$tftp_cursor_file" \
    --slurpfile root_endpoint "$root_endpoint_file" \
    --slurpfile summary "$summary_file" \
    --slurpfile preflight "$EVIDENCE_DIR/preflight-identity.json" \
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
    | ($root_endpoint[0] // {}) as $root
    | ($drain[0].talos_serial_drain // {}) as $sd
    | ($serial[0].talos_serial_window // {}) as $sw
    | ($tftp[0].talos_tftp_stability // {}) as $ts
    | ($summary_doc.label // "") as $summary_label
    | (if $expected_label != "" then $expected_label else $summary_label end) as $run_label
    | ($summary_doc.expected.fetch // $preflight[0].expected_fetch // "") as $expected_fetch
    | (($summary_doc.expected.fetch_bytes // $preflight[0].expected_fetch_bytes) | tonumber?) as $expected_bytes
    | (($sw.required_marker // "") | tostring) as $required_marker
    | (if $report_kind_arg != "" then $report_kind_arg
       elif ($required_marker | contains("control")) then "control"
       elif ($required_marker | contains("candidate")) then "candidate"
       else "" end) as $report_kind
    | (($required_marker | capture("capture-nonce=(?<nonce>[A-Za-z0-9_.:-]+)")? // {}) | .nonce // "") as $marker_nonce
    | (if $expected_nonce != "" then $expected_nonce else $marker_nonce end) as $required_nonce
    | (if $required_nonce == "" then "" else ("capture-nonce=" + $required_nonce) end) as $nonce_token
    | ((($drain[0].responses // []) | map(.text // "") | join("")) | tostring) as $pre_power_serial_text
    | (($serial[0].text // "") | tostring) as $post_power_serial_text
    | count_token($pre_power_serial_text; $required_marker) as $pre_power_marker_count
    | count_token($post_power_serial_text; $required_marker) as $post_power_marker_count
    | count_token($pre_power_serial_text; $nonce_token) as $pre_power_nonce_count
    | count_token($post_power_serial_text; $nonce_token) as $post_power_nonce_count
    | (($serial[0].text // "") | tostring | index($required_marker)) as $marker_index
    | (if $marker_index == null then ""
       else (($serial[0].text // "") | tostring)[
           (if ($marker_index - 80) < 0 then 0 else ($marker_index - 80) end):
           ($marker_index + ($required_marker | length) + 160)
       ] end) as $marker_excerpt
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
    | ($required_nonce != "" and $pre_power_nonce_count == 0 and $post_power_nonce_count > 0) as $nonce_fresh
    | ($empty_pre_power or $nonce_fresh) as $serial_freshness_ok
    | [
        (if $run_label == "" then "missing-run-label" else empty end),
        (if $report_kind == "" then "missing-report-kind" else empty end),
        (if ($preflight[0].staging_publication_mismatch // false) then "preflight-staging-publication-mismatch" else empty end),
        (if ($selected_tree_hash // "") == "" then "missing-selected-tree-hash" else empty end),
        (if ($selected_kernel // "") == "" then "missing-selected-effective-kernel" else empty end),
        (if ($expected_fetch // "") == "" then "missing-expected-fetch-path" else empty end),
        (if $expected_bytes == null then "missing-expected-fetch-byte-count" else empty end),
        (if ($preflight[0].expected_fetch_present // false) != true then "preflight-expected-fetch-missing" else empty end),
        (if ($preflight[0].expected_fetch_bytes_match // false) != true then "preflight-expected-fetch-byte-mismatch" else empty end),
        (if (($sd.contract_version // "") | startswith("pi5-capture-transaction-") | not) then "missing-serial-drain-contract" else empty end),
        (if ($serial_freshness_ok != true) then "run-unique-serial-freshness-not-proven" else empty end),
        (if $required_nonce == "" then "missing-run-unique-capture-nonce" else empty end),
        (if $required_nonce != "" and $pre_power_nonce_count > 0 then "run-unique-capture-nonce-present-before-power" else empty end),
        (if $required_nonce != "" and $post_power_nonce_count == 0 then "run-unique-capture-nonce-not-present-after-power" else empty end),
        (if $required_marker == "" then "missing-required-marker" else empty end),
        (if $post_power_marker_count == 0 then "required-marker-not-present-after-power" else empty end),
        (if $report_kind == "control" and $post_power_marker_count == 0 then "required-control-marker-not-retained" else empty end),
        (if ($saturated_direct_read and (($sw.response_bytes // null) == null)) then "missing-direct-serial-response-bytes" else empty end),
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
        (if ($final_kernel != $selected_kernel) then "final-pre-restore-effective-kernel-mismatch" else empty end),
        (if ($final_fetch == null) then "final-pre-restore-expected-fetch-missing" else empty end),
        (if ($expected_bytes != null and ($final_fetch.bytes // null) != $expected_bytes) then "final-pre-restore-expected-fetch-byte-mismatch" else empty end),
        (if ($restore[0].archive.name // "") == "" then "missing-restore-snapshot-name" else empty end),
        (if ($post_restore_tree_hash // "") == "" then "missing-post-restore-tree-hash" else empty end)
      ] as $rejection_reasons
    | {
        contract_version: "pi5-capture-chain-v4",
        base_contracts: ["pi5-capture-transaction-v2", "pi5-capture-transaction-v3", "pi5-capture-transaction-run-unique-v1"],
        run_label: $run_label,
        report_kind: $report_kind,
        decisive_rp1_hardware_classification_allowed: (($rejection_reasons | length) == 0),
        classification: (if (($rejection_reasons | length) == 0) then "capture-chain-v4-ready" else "capture-staging-blocked" end),
        rejection_reasons: $rejection_reasons,
        endpoint_identity: {
            root_endpoint: $root,
            selected_tree_identity_source: ($preflight[0].selected_tree_identity_source // "/boot/files"),
            fallback_used: ($root.fallback_used // null),
            trust_boundary: {
                api_visible_boot_files_sufficient: false,
                requires_same_power_cycle_tftp_served_kernel_byte_agreement: true,
                requires_final_pre_restore_identity: true
            }
        },
        proof_run_identity: {
            selected_tree_hash: $selected_tree_hash,
            effective_kernel: $selected_kernel,
            expected_fetch_path: $expected_fetch,
            expected_fetch_byte_count: $expected_bytes,
            tftp: {
                cursor_file: (if $tftp_cursor_file == "" then null else ($tftp_cursor_file | tonumber?) end),
                delta_cursor_start: ($tftp[0].tftp.cursor_start // null),
                delta_cursor_end: ($tftp[0].tftp.cursor_end // null),
                stable: ($ts.stable // false),
                event_count: (($tftp[0].tftp.events // []) | length),
                expected_fetch_count: ($fetch_events | length),
                expected_fetch_byte_match_count: ($matching_fetch_events | length),
                expected_fetch_bytes_seen: $fetch_bytes_seen
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
        serial_capture: {
            capture_mode: ($sw.capture_mode // null),
            observe_contract: ($sw.observe_contract // null),
            cursor_file: (if $serial_cursor_file == "" then null else ($serial_cursor_file | tonumber?) end),
            cursor_start: ($serial[0].cursor_start // null),
            cursor_end: ($serial[0].cursor_end // null),
            response_bytes: ($sw.response_bytes // ($serial[0].bytes // null)),
            required_marker: $required_marker,
            required_marker_occurrences: $post_power_marker_count,
            required_marker_excerpt: (if (($sw.required_marker_excerpt // "") != "") then $sw.required_marker_excerpt else $marker_excerpt end),
            nonce_token: (if $nonce_token == "" then null else $nonce_token end),
            nonce_token_occurrences: $post_power_nonce_count,
            pre_power_nonce_occurrences: $pre_power_nonce_count,
            empty_pre_power: $empty_pre_power,
            nonce_fresh: $nonce_fresh,
            serial_freshness_ok: $serial_freshness_ok
        },
        claim_boundary: {
            accepted_if_ready: "capture-chain identity/freshness only",
            rejected_runtime_claims: [
                "live-gem-visibility",
                "ethernet-driver-readiness",
                "rp1-mmio-dma-programming",
                "packet-io",
                "networking",
                "ssh",
                "phase-transition"
            ]
        },
        rule: "missing /boot/files identity, same-power-cycle expected TFTP-served kernel bytes, final selected-tree identity, run-unique marker freshness, direct serial marker retention, or paired control marker prevents decisive RP1 hardware classification; API-visible /boot/files identity alone is not sufficient"
      }
    ')"

printf '%s\n' "$output"

if printf '%s\n' "$output" | jq -e '.decisive_rp1_hardware_classification_allowed == true' >/dev/null; then
    exit 0
fi

exit 1
