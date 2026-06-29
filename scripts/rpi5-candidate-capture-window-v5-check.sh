#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-candidate-capture-window-v5-check.sh --evidence-dir DIR
       [--label LABEL] [--report-kind candidate|control] [--nonce NONCE]

Checks the fail-closed Pi 5 capture-window v5 contract. It reuses the v4
identity/freshness join and additionally requires helper-owned capture-window
ordering that proves TFTP and final pre-restore identity were captured before
restore.
EOF
}

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
V4_CHECKER="$SCRIPT_DIR/rpi5-proof-identity-join-v4-check.sh"

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

order_file="$EVIDENCE_DIR/capture-window-order.json"
order_missing=false
if [ ! -f "$order_file" ]; then
    order_file=/dev/null
    order_missing=true
fi

v4_out="$(mktemp)"
v4_err="$(mktemp)"
trap 'rm -f "$v4_out" "$v4_err"' EXIT INT TERM

set +e
"$V4_CHECKER" --evidence-dir "$EVIDENCE_DIR" --label "$EXPECTED_LABEL" \
    --report-kind "$REPORT_KIND" --nonce "$EXPECTED_NONCE" > "$v4_out" 2> "$v4_err"
v4_exit="$?"
set -e

jq empty "$v4_out"
jq empty "$order_file"

output="$(jq -n \
    --arg expected_label "$EXPECTED_LABEL" \
    --argjson v4_exit "$v4_exit" \
    --argjson order_missing "$order_missing" \
    --slurpfile v4 "$v4_out" \
    --slurpfile order "$order_file" \
    --rawfile v4_stderr "$v4_err" \
    '
    def idx($events; $stage):
      (($events | map(.stage) | index($stage)) // -1);
    def files_for($events; $stage):
      (($events[]? | select(.stage == $stage) | .evidence_files) // []);
    def has_files($events; $stage; $required):
      (($required - files_for($events; $stage)) == []);

    ($v4[0]) as $v4doc
    | ($order[0]) as $orderdoc
    | ($orderdoc.events // []) as $events
    | (if $expected_label != "" then $expected_label else ($v4doc.run_label // "") end) as $run_label
    | (idx($events; "preflight_identity")) as $preflight_i
    | (idx($events; "pre_power_cursors")) as $cursors_i
    | (idx($events; "power_cycle")) as $power_i
    | (idx($events; "serial_observe_window")) as $serial_i
    | (idx($events; "tftp_delta_stable_pre_restore")) as $tftp_i
    | (idx($events; "final_pre_restore_identity")) as $final_i
    | (idx($events; "restore_snapshot")) as $restore_i
    | (idx($events; "post_restore_identity")) as $post_i
    | ($orderdoc.helper_run_completed == true) as $helper_completed
    | ($orderdoc.completed_at // "") as $completed_at
    | (($orderdoc.completion_event_count // -1) == ($events | length)) as $completion_count_matches
    | [
        (if $order_missing or (($orderdoc.contract_version // "") != "pi5-candidate-capture-window-v5") then "missing-capture-window-v5-contract" else empty end),
        (if ($orderdoc.helper // "") != "scripts/rpi5-capture-invariant-proof-bundle.sh" then "capture-window-helper-not-authoritative" else empty end),
        (if $helper_completed then empty else "capture-window-helper-run-incomplete" end),
        (if $completed_at != "" then empty else "capture-window-helper-completion-missing" end),
        (if $completion_count_matches then empty else "capture-window-helper-completion-count-mismatch" end),
        (if ($orderdoc.run_label // "") != $run_label then "capture-window-run-label-mismatch" else empty end),
        (if ($events | length) == 0 then "missing-capture-window-events" else empty end),
        (if (($events | map(.sequence) | unique | length) != ($events | length)) then "capture-window-duplicate-sequence" else empty end),
        (if (($events | map(.stage) | unique | length) != ($events | length)) then "capture-window-duplicate-stage" else empty end),
        (if ($events | any(.sequence == null or .stage == null or .captured_at == null)) then "capture-window-incomplete-event" else empty end),
        (if [$preflight_i,$cursors_i,$power_i,$serial_i,$tftp_i,$final_i,$restore_i,$post_i] | any(. < 0) then "capture-window-missing-required-stage" else empty end),
        (if ($preflight_i < $cursors_i and $cursors_i < $power_i and $power_i < $serial_i and $serial_i < $tftp_i and $tftp_i < $final_i and $final_i < $restore_i and $restore_i < $post_i) then empty else "capture-window-stage-order-invalid" end),
        (if $final_i >= 0 and $restore_i >= 0 and $final_i < $restore_i then empty else "final-pre-restore-identity-not-before-restore" end),
        (if $tftp_i >= 0 and $restore_i >= 0 and $tftp_i < $restore_i then empty else "tftp-delta-not-before-restore" end),
        (if has_files($events; "preflight_identity"; ["preflight-identity.json"]) then empty else "capture-window-preflight-file-missing" end),
        (if has_files($events; "pre_power_cursors"; ["serial-cursor-before-power.txt","tftp-cursor-before-power.txt"]) then empty else "capture-window-cursor-file-missing" end),
        (if has_files($events; "tftp_delta_stable_pre_restore"; ["tftp-delta-stable-pre-restore.json","tftp-delta-stable-pre-restore.exit"]) then empty else "capture-window-tftp-file-missing" end),
        (if has_files($events; "final_pre_restore_identity"; ["final-pre-restore-status.json","final-pre-restore-boot-files.json"]) then empty else "capture-window-final-identity-file-missing" end),
        (if has_files($events; "restore_snapshot"; ["restore-snapshot.json"]) then empty else "capture-window-restore-file-missing" end)
      ] as $window_reasons
    | (($v4doc.rejection_reasons // []) + $window_reasons) as $reasons
    | $v4doc + {
        contract_version: "pi5-candidate-capture-window-v5",
        base_contracts: (($v4doc.base_contracts // []) + ["pi5-capture-chain-v4", "pi5-candidate-capture-window-v5"]),
        v4_exit: $v4_exit,
        v4_stderr: $v4_stderr,
        decisive_rp1_hardware_classification_allowed: (($reasons | length) == 0),
        classification: (if (($reasons | length) == 0) then "capture-chain-v5-ready" else "capture-staging-blocked" end),
        rejection_reasons: $reasons,
        capture_window_contract: {
            helper: ($orderdoc.helper // null),
            run_label: ($orderdoc.run_label // null),
            helper_run_completed: ($orderdoc.helper_run_completed // false),
            completed_at: ($orderdoc.completed_at // null),
            completion_event_count: ($orderdoc.completion_event_count // null),
            rule: ($orderdoc.rule // null),
            stage_order: ($events | map(.stage)),
            final_pre_restore_before_restore: ($final_i >= 0 and $restore_i >= 0 and $final_i < $restore_i),
            tftp_delta_before_restore: ($tftp_i >= 0 and $restore_i >= 0 and $tftp_i < $restore_i),
            failure_boundary: "restore/control identity cannot satisfy candidate pre-restore evidence; missing or out-of-order helper-owned window metadata fails closed"
        },
        rule: "v5 requires v4 identity/freshness plus helper-owned capture-window order proving TFTP and final pre-restore identity were captured before restore"
      }
    ')"

printf '%s\n' "$output"

if printf '%s\n' "$output" | jq -e '.decisive_rp1_hardware_classification_allowed == true' >/dev/null; then
    exit 0
fi

exit 1
