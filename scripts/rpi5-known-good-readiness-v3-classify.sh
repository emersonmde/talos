#!/bin/sh
set -eu

if [ "$#" -lt 4 ] || [ "$#" -gt 5 ]; then
    echo "usage: $0 <retained_primary_json> <tftp_delta_json> <pre_status_json> <final_status_json> [success_marker]" >&2
    exit 2
fi

PRIMARY_JSON="$1"
TFTP_DELTA_JSON="$2"
PRE_STATUS_JSON="$3"
FINAL_STATUS_JSON="$4"
SUCCESS_MARKER="${5:-}"

for path in "$PRIMARY_JSON" "$TFTP_DELTA_JSON" "$PRE_STATUS_JSON" "$FINAL_STATUS_JSON"; do
    if [ ! -f "$path" ]; then
        echo "missing input artifact: $path" >&2
        exit 2
    fi
done

case "$PRIMARY_JSON" in
    *latest*|*LATEST*)
        echo "refusing mutable/latest primary artifact path: $PRIMARY_JSON" >&2
        exit 3
        ;;
    *-runtime-readiness-primary.json)
        ;;
    *)
        echo "primary artifact must be a retained run-label-qualified runtime-readiness primary JSON" >&2
        exit 3
        ;;
esac

classification="$(jq -n \
    --arg primary_path "$PRIMARY_JSON" \
    --arg tftp_path "$TFTP_DELTA_JSON" \
    --arg pre_status_path "$PRE_STATUS_JSON" \
    --arg final_status_path "$FINAL_STATUS_JSON" \
    --arg success_marker_arg "$SUCCESS_MARKER" \
    --slurpfile primary "$PRIMARY_JSON" \
    --slurpfile tftp "$TFTP_DELTA_JSON" \
    --slurpfile pre_status "$PRE_STATUS_JSON" \
    --slurpfile final_status "$FINAL_STATUS_JSON" \
    '
    ($primary[0]) as $p |
    ($tftp[0]) as $t |
    ($pre_status[0]) as $pre |
    ($final_status[0]) as $final |
    ($p.talos_runtime_readiness // {}) as $trr |
    (if $success_marker_arg != "" then $success_marker_arg
     else ($trr.required_success_marker // "rpi5-production-timer-preemption: PASS")
     end) as $success_marker |
    (($trr.has_required_success_marker == true) or (($p.text // "") | contains($success_marker))) as $has_success_marker |
    (($trr.has_kernel_main == true) or (($p.text // "") | contains("TALOS: kernel_main"))) as $has_kernel_main |
    ($pre.boot.tree_hash // "") as $pre_tree |
    ($final.boot.tree_hash // "") as $final_tree |
    ($pre.boot.effective_kernel // $pre.boot.configured_kernel // "") as $pre_kernel |
    ($final.boot.effective_kernel // $final.boot.configured_kernel // "") as $final_kernel |
    (($pre.ok == true) and ($final.ok == true) and
     ($pre_tree != "") and ($final_tree != "") and ($pre_tree == $final_tree) and
     ($pre_kernel != "") and ($final_kernel != "") and ($pre_kernel == $final_kernel)) as $stable_identity |
    ($t.talos_tftp_stability.stable == true) as $stable_tftp |
    (($t.tftp.events // []) | map(select(.status == "served" and (.filename // "" | endswith("/" + $final_kernel)))) | length) as $kernel_fetch_count |
    ($kernel_fetch_count > 0) as $has_kernel_fetch |
    (($p.cursor_start // null) != null and ($p.cursor_end // null) != null) as $has_primary_cursor_window |
    (($t.tftp.cursor_start // null) != null and ($t.tftp.cursor_end // null) != null) as $has_tftp_cursor_window |
    ([
      (if $has_success_marker then empty else "missing-production-success-marker" end),
      (if $stable_identity then empty else "missing-or-unstable-boot-identity-join" end),
      (if $stable_tftp then empty else "missing-stable-tftp-delta" end),
      (if $has_kernel_fetch then empty else "missing-served-effective-kernel-fetch" end),
      (if $has_primary_cursor_window then empty else "missing-retained-primary-cursor-window" end),
      (if $has_tftp_cursor_window then empty else "missing-tftp-cursor-window" end)
    ]) as $rejection_reasons |
    ($rejection_reasons | length == 0) as $valid |
    {
      contract: "valid-known-good-talos-readiness-v3",
      contract_version: 3,
      input_artifacts: {
        retained_primary_artifact: $primary_path,
        tftp_delta: $tftp_path,
        pre_status: $pre_status_path,
        final_status: $final_status_path
      },
      contract_policy: {
        primary_artifact_policy: "retained-run-label-qualified-artifact-only",
        mutable_latest_artifact_policy: "rejected-by-wrapper",
        same_run_join_policy: "caller-provided retained primary, stable TFTP delta, and pre/final status tuple from one proof run",
        success_marker_policy: "required",
        kernel_main_policy: "metadata-only-not-required-for-v3-known-good-readiness",
        required_success_marker: $success_marker
      },
      checks: {
        has_required_success_marker: $has_success_marker,
        has_kernel_main_metadata: $has_kernel_main,
        stable_boot_identity: $stable_identity,
        pre_tree_hash: $pre_tree,
        final_tree_hash: $final_tree,
        effective_kernel: $final_kernel,
        stable_tftp_delta: $stable_tftp,
        served_effective_kernel_fetch_count: $kernel_fetch_count,
        retained_primary_cursor_window: $has_primary_cursor_window,
        tftp_cursor_window: $has_tftp_cursor_window
      },
      retained_risks: [
        (if $has_kernel_main then empty else "TALOS: kernel_main absent from retained primary serial window; v3 records this as metadata rather than a mandatory readiness marker" end)
      ],
      rejection_reasons: $rejection_reasons,
      talos_runtime_readiness_v3: {
        valid_known_good_talos_readiness_v3: $valid,
        classification: (if $valid then "valid-known-good-talos-readiness-v3" else "known-good-readiness-v3-blocked" end),
        evidence_level: "local/static classification over retained serial hardware artifact plus lab-controller status and stable TFTP evidence",
        required_success_marker: $success_marker,
        has_required_success_marker: $has_success_marker,
        has_kernel_main: $has_kernel_main,
        kernel_main_policy: "metadata-only-not-required",
        stable_boot_identity: $stable_identity,
        stable_tftp_delta: $stable_tftp,
        served_effective_kernel_fetch_count: $kernel_fetch_count
      },
      rejected_claims: [
        "GPIO32 write/restore authorization by classifier alone",
        "PHY reset behavior",
        "Ethernet driver behavior",
        "DMA/descriptors",
        "interrupts",
        "packet I/O",
        "networking",
        "sockets",
        "SSH",
        "Phase 12.2",
        "phase transition"
      ]
    }')"

printf '%s\n' "$classification"

printf '%s' "$classification" |
    jq -e '.talos_runtime_readiness_v3.valid_known_good_talos_readiness_v3' >/dev/null
