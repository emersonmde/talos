#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-boot-staging-identity-check.sh --evidence-dir DIR
       [--baseline-tree-hash HASH]

Replays retained Pi 5 proof-bundle evidence and checks only boot-staging
identity: the selected tree before power, the expected TFTP kernel fetch bytes,
the final pre-restore selected tree, and restore identity. It intentionally
does not inspect serial markers or RP1 diagnostic output.
EOF
}

EVIDENCE_DIR=
BASELINE_TREE_HASH=

while [ "$#" -gt 0 ]; do
    case "$1" in
        --evidence-dir)
            if [ "$#" -lt 2 ]; then usage; exit 2; fi
            EVIDENCE_DIR="$2"
            shift 2
            ;;
        --baseline-tree-hash)
            if [ "$#" -lt 2 ]; then usage; exit 2; fi
            BASELINE_TREE_HASH="$2"
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
        echo "missing required staging identity file: $EVIDENCE_DIR/$1" >&2
        exit 2
    fi
}

require_file preflight-identity.json
require_file tftp-delta-stable-pre-restore.json
require_file final-pre-restore-status.json
require_file final-pre-restore-boot-files.json
require_file post-restore-status.json

tftp_cursor_file=""
if [ -f "$EVIDENCE_DIR/tftp-cursor-before-power.txt" ]; then
    tftp_cursor_file="$(cat "$EVIDENCE_DIR/tftp-cursor-before-power.txt")"
fi

output="$(jq -n \
    --arg baseline_tree_hash "$BASELINE_TREE_HASH" \
    --arg tftp_cursor_file "$tftp_cursor_file" \
    --slurpfile preflight "$EVIDENCE_DIR/preflight-identity.json" \
    --slurpfile tftp "$EVIDENCE_DIR/tftp-delta-stable-pre-restore.json" \
    --slurpfile final_status "$EVIDENCE_DIR/final-pre-restore-status.json" \
    --slurpfile final_files "$EVIDENCE_DIR/final-pre-restore-boot-files.json" \
    --slurpfile post_restore "$EVIDENCE_DIR/post-restore-status.json" \
    '
    ($preflight[0].expected_fetch // "") as $expected_fetch
    | (($preflight[0].expected_fetch_bytes // null) | tonumber?) as $expected_bytes
    | ($preflight[0].observed_tree_hash // null) as $selected_tree_hash
    | ($preflight[0].observed_effective_kernel // null) as $selected_kernel
    | ($tftp[0].talos_tftp_stability // {}) as $ts
    | (($tftp[0].tftp.events // []) | map(select(.filename == $expected_fetch and .status == "served"))) as $fetch_events
    | ($fetch_events | map(select(.bytes == $expected_bytes))) as $matching_fetch_events
    | ($final_status[0].boot.tree_hash // null) as $final_tree_hash
    | ($final_status[0].boot.effective_kernel // null) as $final_kernel
    | (($final_files[0].boot.files // []) | map(select(.name == $expected_fetch)) | first) as $final_fetch
    | ($post_restore[0].boot.tree_hash // null) as $post_restore_tree_hash
    | [
        (if ($preflight[0].staging_publication_mismatch // false) then "preflight-staging-publication-mismatch" else empty end),
        (if ($selected_tree_hash // "") == "" then "missing-selected-tree-hash" else empty end),
        (if $baseline_tree_hash != "" and $selected_tree_hash == $baseline_tree_hash then "selected-tree-is-baseline" else empty end),
        (if ($selected_kernel // "") == "" then "missing-selected-effective-kernel" else empty end),
        (if ($expected_fetch // "") == "" then "missing-expected-fetch-path" else empty end),
        (if $expected_bytes == null then "missing-expected-fetch-byte-count" else empty end),
        (if ($preflight[0].expected_fetch_present // false) != true then "preflight-expected-fetch-missing" else empty end),
        (if ($preflight[0].expected_fetch_bytes_match // false) != true then "preflight-expected-fetch-byte-mismatch" else empty end),
        (if ($tftp_cursor_file == "" and (($tftp[0].tftp.cursor_start // null) == null)) then "missing-tftp-cursor-start" else empty end),
        (if (($tftp[0].tftp.cursor_end // null) == null) then "missing-tftp-cursor-end" else empty end),
        (if ($ts.stable // false) != true then "tftp-delta-not-stable" else empty end),
        (if (($fetch_events | length) == 0) then "expected-fetch-not-observed-in-tftp-delta" else empty end),
        (if (($fetch_events | length) > 0 and ($matching_fetch_events | length) != ($fetch_events | length)) then "tftp-expected-fetch-byte-mismatch" else empty end),
        (if ($final_tree_hash // "") == "" then "missing-final-pre-restore-tree-hash" else empty end),
        (if ($selected_tree_hash != null and $final_tree_hash != $selected_tree_hash) then "final-pre-restore-selected-tree-mismatch" else empty end),
        (if $baseline_tree_hash != "" and $final_tree_hash == $baseline_tree_hash and $selected_tree_hash != $baseline_tree_hash then "final-pre-restore-is-baseline" else empty end),
        (if ($final_kernel != $selected_kernel) then "final-pre-restore-effective-kernel-mismatch" else empty end),
        (if ($final_fetch == null) then "final-pre-restore-expected-fetch-missing" else empty end),
        (if ($expected_bytes != null and ($final_fetch.bytes // null) != $expected_bytes) then "final-pre-restore-expected-fetch-byte-mismatch" else empty end),
        (if ($post_restore_tree_hash // "") == "" then "missing-post-restore-tree-hash" else empty end)
      ] as $rejection_reasons
    | {
        contract_version: "pi5-boot-staging-identity-v1",
        staging_identity_decisive: (($rejection_reasons | length) == 0),
        classification: (if (($rejection_reasons | length) == 0) then "boot-staging-identity-ready" else "boot-staging-identity-blocked" end),
        rejection_reasons: $rejection_reasons,
        baseline_tree_hash: (if $baseline_tree_hash == "" then null else $baseline_tree_hash end),
        selected: {
          tree_hash: $selected_tree_hash,
          effective_kernel: $selected_kernel,
          expected_fetch_path: $expected_fetch,
          expected_fetch_byte_count: $expected_bytes
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
          selected_tree_still_staged: ($selected_tree_hash != null and $final_tree_hash == $selected_tree_hash),
          expected_fetch_present: ($final_fetch != null),
          expected_fetch_bytes: ($final_fetch.bytes // null)
        },
        restore: {
          post_restore_tree_hash: $post_restore_tree_hash
        },
        rule: "candidate serial output is not decisive unless selected-tree identity, expected TFTP fetch bytes, final pre-restore identity, and restore identity all match one staged boot tree"
      }
    ')"

printf '%s\n' "$output"

if printf '%s\n' "$output" | jq -e '.staging_identity_decisive == true' >/dev/null; then
    exit 0
fi

exit 1
