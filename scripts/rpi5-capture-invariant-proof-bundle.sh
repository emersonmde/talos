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
SERIAL_TIMEOUT=90
SETTLE_MS=1000
MAX_BYTES=65536
TFTP_TIMEOUT=90
STABLE_SAMPLES=3

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
        --argjson serial_timeout "$SERIAL_TIMEOUT" \
        --argjson settle_ms "$SETTLE_MS" \
        --argjson max_bytes "$MAX_BYTES" \
        --argjson tftp_timeout "$TFTP_TIMEOUT" \
        --argjson stable_samples "$STABLE_SAMPLES" \
        '{dry_run: true,
          hardware_actions: [],
          would_write: {
              evidence_dir: $evidence_dir,
              pre_status: "pre-status.json",
              pre_boot_files: "pre-boot-files.json",
              pre_snapshots: "pre-snapshots.json",
              serial_peek_before_power: "serial-peek-before-power.json",
              tftp_cursor_before_power: "tftp-cursor-before-power.json",
              power_cycle: "power-cycle.json",
              serial_observe_window: "serial-observe-window.json",
              tftp_delta_stable_pre_restore: "tftp-delta-stable-pre-restore.json",
              final_pre_restore_status: "final-pre-restore-status.json",
              final_pre_restore_boot_files: "final-pre-restore-boot-files.json",
              restore_snapshot: "restore-snapshot.json",
              post_restore_status: "post-restore-status.json",
              post_restore_boot_files: "post-restore-boot-files.json",
              summary: "capture-invariant-summary.json"
          },
          contract: {
              "label": $proof_label,
              restore_snapshot: $restore_snapshot,
              expected_tree_hash: $expected_tree_hash,
              expected_kernel: $expected_kernel,
              expected_fetch: $expected_fetch,
              expected_fetch_bytes: $expected_fetch_bytes,
              serial_marker: $serial_marker,
              serial_observe_contract: "deadline-loop-accumulated-from-fresh-cursor",
              tftp_contract: "stable-same-cursor-delta-before-restore",
              serial_timeout_seconds: $serial_timeout,
              settle_ms: $settle_ms,
              max_bytes: $max_bytes,
              tftp_timeout_seconds: $tftp_timeout,
              stable_samples: $stable_samples
          }}'
    exit 0
fi

mkdir -p "$EVIDENCE_DIR"

curl -fsS "${API_BASE}/status" > "$EVIDENCE_DIR/pre-status.json"
curl -fsS "${API_BASE}/boot/files" > "$EVIDENCE_DIR/pre-boot-files.json"
curl -fsS "${API_BASE}/boot/snapshots" > "$EVIDENCE_DIR/pre-snapshots.json"

jq -n \
    --arg expected_tree_hash "$EXPECTED_TREE_HASH" \
    --arg expected_kernel "$EXPECTED_KERNEL" \
    --arg expected_fetch "$EXPECTED_FETCH" \
    --arg expected_fetch_bytes "$EXPECTED_FETCH_BYTES" \
    --slurpfile status "$EVIDENCE_DIR/pre-status.json" \
    --slurpfile files "$EVIDENCE_DIR/pre-boot-files.json" \
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

curl -fsS "${API_BASE}/serial/peek?max_bytes=500&drain=true" > "$EVIDENCE_DIR/serial-peek-before-power.json"
curl -fsS "${API_BASE}/tftp/logs?max_bytes=1048576&limit=1" > "$EVIDENCE_DIR/tftp-cursor-before-power.json"
serial_cursor="$(jq -r '.cursor' "$EVIDENCE_DIR/serial-peek-before-power.json")"
tftp_cursor="$(jq -r '.tftp.cursor_end' "$EVIDENCE_DIR/tftp-cursor-before-power.json")"
printf '%s\n' "$serial_cursor" > "$EVIDENCE_DIR/serial-cursor-before-power.txt"
printf '%s\n' "$tftp_cursor" > "$EVIDENCE_DIR/tftp-cursor-before-power.txt"

curl -fsS -X POST "${API_BASE}/power/cycle" > "$EVIDENCE_DIR/power-cycle.json"

set +e
./scripts/rpi5-observe-serial-window.sh \
    "$serial_cursor" "$SERIAL_TIMEOUT" "$SETTLE_MS" "$MAX_BYTES" "$SERIAL_MARKER" \
    > "$EVIDENCE_DIR/serial-observe-window.json"
serial_exit="$?"
set -e
printf '%s\n' "$serial_exit" > "$EVIDENCE_DIR/serial-observe-window.exit"

set +e
./scripts/rpi5-wait-tftp-delta.sh "$tftp_cursor" "$TFTP_TIMEOUT" "$STABLE_SAMPLES" \
    > "$EVIDENCE_DIR/tftp-delta-stable-pre-restore.json"
tftp_exit="$?"
set -e
printf '%s\n' "$tftp_exit" > "$EVIDENCE_DIR/tftp-delta-stable-pre-restore.exit"

curl -fsS "${API_BASE}/status" > "$EVIDENCE_DIR/final-pre-restore-status.json"
curl -fsS "${API_BASE}/boot/files" > "$EVIDENCE_DIR/final-pre-restore-boot-files.json"
curl -fsS -X POST "${API_BASE}/boot/restore?name=${RESTORE_SNAPSHOT}" > "$EVIDENCE_DIR/restore-snapshot.json"
curl -fsS "${API_BASE}/status" > "$EVIDENCE_DIR/post-restore-status.json"
curl -fsS "${API_BASE}/boot/files" > "$EVIDENCE_DIR/post-restore-boot-files.json"

jq -n \
    --arg proof_label "$LABEL" \
    --arg restore_snapshot "$RESTORE_SNAPSHOT" \
    --arg expected_tree_hash "$EXPECTED_TREE_HASH" \
    --arg expected_kernel "$EXPECTED_KERNEL" \
    --arg expected_fetch "$EXPECTED_FETCH" \
    --arg expected_fetch_bytes "$EXPECTED_FETCH_BYTES" \
    --argjson serial_exit "$serial_exit" \
    --argjson tftp_exit "$tftp_exit" \
    --slurpfile preflight "$EVIDENCE_DIR/preflight-identity.json" \
    --slurpfile serial "$EVIDENCE_DIR/serial-observe-window.json" \
    --slurpfile tftp "$EVIDENCE_DIR/tftp-delta-stable-pre-restore.json" \
    --slurpfile final_status "$EVIDENCE_DIR/final-pre-restore-status.json" \
    --slurpfile final_files "$EVIDENCE_DIR/final-pre-restore-boot-files.json" \
    --slurpfile restore "$EVIDENCE_DIR/restore-snapshot.json" \
    --slurpfile post_restore "$EVIDENCE_DIR/post-restore-status.json" \
    '(($expected_fetch_bytes | tonumber?) // null) as $expected_bytes
     | ($serial[0].talos_serial_window // {}) as $sw
     | ($tftp[0].talos_tftp_stability // {}) as $ts
     | (($tftp[0].tftp.events // []) | map(select(.filename == $expected_fetch))) as $fetch_events
     | (($final_files[0].boot.files // []) | map(select(.name == $expected_fetch)) | first) as $final_fetch
     | ($final_status[0].boot.tree_hash // null) as $final_tree_hash
     | ($final_status[0].boot.effective_kernel // null) as $final_kernel
     | ($post_restore[0].boot.tree_hash // null) as $post_restore_tree_hash
     | ($preflight[0].staging_publication_mismatch // false) as $preflight_mismatch
     | ($expected_tree_hash == "" or $final_tree_hash == $expected_tree_hash) as $final_tree_ok
     | ($expected_bytes == null or ($final_fetch.bytes // null) == $expected_bytes) as $final_bytes_ok
     | (($ts.stable == true) and (($tftp[0].tftp.events // []) | length) == 0) as $stable_zero_tftp
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
         expected_fetch_events: $fetch_events,
         restore: {
             snapshot: $restore[0],
             post_restore_tree_hash: $post_restore_tree_hash
         },
         suggested_classification:
             (
             if $preflight_mismatch then "staging-publication-mismatch"
             elif ($sw.has_required_marker == true) then "post-handoff-marker-visible"
             elif (($fetch_events | length) >= 2 and (($sw.firmware_network_occurrences // 0) >= 2)) then "reset-side-effect-without-visible-marker-candidate"
             elif (($fetch_events | length) > 0) then "candidate-fetch-observed-without-marker"
             elif ($stable_zero_tftp and ($sw.has_firmware_network == true) and $final_tree_ok and ($final_fetch != null) and $final_bytes_ok) then "tftp-capture-logging-blindness"
             elif ($sw.has_firmware_network == true) then "serial-only-firmware-reboot"
             else "still-blocked-without-fresh-boot-evidence"
             end),
         proof_contract: {
             serial_observe_contract: "deadline-loop-accumulated-from-fresh-cursor",
             tftp_contract: "stable-same-cursor-delta-before-restore",
             zero_event_tftp_meaningful_only_before_restore: true,
             firmware_network_is_not_talos_entry: true
         }
       }' > "$EVIDENCE_DIR/capture-invariant-summary.json"

jq . "$EVIDENCE_DIR/capture-invariant-summary.json"
