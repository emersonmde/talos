#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-staging-identity-gate-v1-check.sh --v4-check FILE
       [--evidence-consistency-guard FILE] [--label LABEL]

Replays retained pi5-capture-chain-v4 JSON and checks only the staging
identity durability fields needed before another register-vector hardware
proof: selected tree, same-power-cycle TFTP-served kernel byte/count
agreement, final pre-restore identity, restore identity, and run-unique serial
freshness. API-visible /status or /boot/files identity is not sufficient by
itself.
EOF
}

V4_CHECK=
EVIDENCE_GUARD=
EXPECTED_LABEL=

while [ "$#" -gt 0 ]; do
    case "$1" in
        --v4-check)
            if [ "$#" -lt 2 ]; then usage; exit 2; fi
            V4_CHECK="$2"
            shift 2
            ;;
        --evidence-consistency-guard)
            if [ "$#" -lt 2 ]; then usage; exit 2; fi
            EVIDENCE_GUARD="$2"
            shift 2
            ;;
        --label)
            if [ "$#" -lt 2 ]; then usage; exit 2; fi
            EXPECTED_LABEL="$2"
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

if [ -z "$V4_CHECK" ]; then
    usage
    exit 2
fi

if [ ! -f "$V4_CHECK" ]; then
    echo "missing v4 check JSON: $V4_CHECK" >&2
    exit 2
fi

guard_file=/dev/null
if [ -n "$EVIDENCE_GUARD" ]; then
    if [ ! -f "$EVIDENCE_GUARD" ]; then
        echo "missing evidence-consistency guard JSON: $EVIDENCE_GUARD" >&2
        exit 2
    fi
    guard_file="$EVIDENCE_GUARD"
fi

output="$(jq -n \
    --arg expected_label "$EXPECTED_LABEL" \
    --slurpfile v4 "$V4_CHECK" \
    --slurpfile guard "$guard_file" \
    '
    def first_or_null($a): if (($a // []) | length) == 0 then null else $a[0] end;
    def invariant_for($reason):
      if $reason == null then null
      elif ($reason | test("evidence-consistency")) then "evidence-consistency-failure"
      elif ($reason | test("serial|nonce|marker")) then "serial-freshness-mismatch"
      elif ($reason | test("tftp|fetch.*byte|expected-fetch")) then "expected-fetch-byte-mismatch"
      elif ($reason | test("final-pre-restore")) then "final-pre-restore-mismatch"
      elif ($reason | test("restore")) then "restore-mismatch"
      elif ($reason | test("selected-tree")) then "selected-tree-mismatch"
      else $reason end;

    ($v4[0] // {}) as $doc
    | ($guard[0] // {}) as $guard_doc
    | ($doc.proof_run_identity // {}) as $id
    | ($doc.serial_capture // {}) as $serial
    | ($id.tftp // {}) as $tftp
    | ($id.final_pre_restore // {}) as $final
    | ($id.restore // {}) as $restore
    | (($doc.rejection_reasons // []) | map(select(. != null and . != ""))) as $v4_reasons
    | (($doc.classification // "") == "capture-chain-v4-ready") as $v4_ready
    | [
        (if (($id.selected_tree_hash // "") == "") then "missing-selected-tree-hash" else empty end),
        (if (($id.expected_fetch_path // "") == "") then "missing-expected-fetch-path" else empty end),
        (if (($id.expected_fetch_byte_count // null) == null) then "missing-expected-fetch-byte-count" else empty end),
        (if (($serial.serial_freshness_ok // false) != true) then "serial-freshness-mismatch" else empty end),
        (if (($tftp.stable // false) != true) then "tftp-delta-not-stable" else empty end),
        (if (($tftp.expected_fetch_count // 0) == 0) then "expected-fetch-not-observed-in-tftp-delta" else empty end),
        (if (($tftp.expected_fetch_count // 0) > 0 and (($tftp.expected_fetch_byte_match_count // null) != ($tftp.expected_fetch_count // null))) then "tftp-expected-fetch-byte-mismatch" else empty end),
        (if (($final.tree_hash // "") == "") then "missing-final-pre-restore-tree-hash" else empty end),
        (if (($final.selected_tree_still_staged // false) != true) then "final-pre-restore-selected-tree-mismatch" else empty end),
        (if (($final.expected_fetch_present // false) != true) then "final-pre-restore-expected-fetch-missing" else empty end),
        (if (($id.expected_fetch_byte_count // null) != null and (($final.expected_fetch_bytes // null) != ($id.expected_fetch_byte_count // null))) then "final-pre-restore-expected-fetch-byte-mismatch" else empty end),
        (if (($restore.post_restore_tree_hash // "") == "") then "missing-post-restore-tree-hash" else empty end)
      ] as $local_reasons
    | (if (($guard_doc | length) == 0) then []
       elif (($guard_doc.classification // "") == "evidence-consistency-ready") then []
       else ["evidence-consistency-failure"] end) as $guard_reasons
    | (if ($v4_ready and (($local_reasons | length) != 0)) then ["capture-chain-v4-local-identity-disagreement"] else [] end) as $agreement_reasons
    | ($guard_reasons + $local_reasons + $agreement_reasons) as $rejection_reasons
    | first_or_null($rejection_reasons) as $first_reason
    | {
        contract_version: "pi5-staging-identity-gate-v1",
        source_contract_version: ($doc.contract_version // null),
        run_label: ($doc.run_label // null),
        report_kind: ($doc.report_kind // null),
        expected_label: (if $expected_label == "" then null else $expected_label end),
        staging_identity_ready: (($rejection_reasons | length) == 0),
        classification: (if (($rejection_reasons | length) == 0) then "selected-tree-identity-ready" else "selected-tree-identity-blocked" end),
        first_failing_rejection_reason: $first_reason,
        first_failing_invariant: invariant_for($first_reason),
        capture_chain_v4: {
          classification: ($doc.classification // null),
          first_failing_rejection_reason: first_or_null($v4_reasons),
          rejection_reasons: $v4_reasons,
          agrees_with_gate_first_failure: (first_or_null($v4_reasons) == $first_reason),
          agrees_with_gate_readiness: ($v4_ready == (($rejection_reasons | length) == 0))
        },
        rejection_reasons: $rejection_reasons,
        selected: {
          tree_hash: ($id.selected_tree_hash // null),
          effective_kernel: ($id.effective_kernel // null),
          expected_fetch_path: ($id.expected_fetch_path // null),
          expected_fetch_byte_count: ($id.expected_fetch_byte_count // null)
        },
        tftp: {
          cursor_file: ($tftp.cursor_file // null),
          delta_cursor_start: ($tftp.delta_cursor_start // null),
          delta_cursor_end: ($tftp.delta_cursor_end // null),
          stable: ($tftp.stable // false),
          event_count: ($tftp.event_count // null),
          expected_fetch_count: ($tftp.expected_fetch_count // null),
          expected_fetch_byte_match_count: ($tftp.expected_fetch_byte_match_count // null),
          expected_fetch_bytes_seen: ($tftp.expected_fetch_bytes_seen // [])
        },
        final_pre_restore: {
          tree_hash: ($final.tree_hash // null),
          effective_kernel: ($final.effective_kernel // null),
          selected_tree_still_staged: ($final.selected_tree_still_staged // false),
          expected_fetch_present: ($final.expected_fetch_present // false),
          expected_fetch_bytes: ($final.expected_fetch_bytes // null)
        },
        restore: {
          snapshot_name: ($restore.snapshot_name // null),
          post_restore_tree_hash: ($restore.post_restore_tree_hash // null)
        },
        serial_freshness: {
          serial_freshness_ok: ($serial.serial_freshness_ok // false),
          nonce_fresh: ($serial.nonce_fresh // false),
          empty_pre_power: ($serial.empty_pre_power // false),
          required_marker_occurrences: ($serial.required_marker_occurrences // null),
          nonce_token_occurrences: ($serial.nonce_token_occurrences // null),
          pre_power_nonce_occurrences: ($serial.pre_power_nonce_occurrences // null)
        },
        evidence_consistency: {
          guard_supplied: (($guard_doc | length) != 0),
          classification: ($guard_doc.classification // null)
        },
        trust_boundary: {
          api_visible_boot_files_sufficient: false,
          requires_same_power_cycle_tftp_served_kernel_byte_agreement: true,
          requires_final_pre_restore_identity: true,
          quarantine_reason: "lab API-visible selected-tree identity has diverged from actual dnsmasq-served TFTP bytes in retained MDIO register-vector evidence"
        },
        claim_boundary: {
          accepted_if_ready: "staging identity durability and reporting-path freshness only",
          rejected_runtime_claims: [
            "register-vector-man-data-values",
            "broad-mdio-phy-ownership",
            "phy-absence",
            "ethernet-behavior",
            "networking",
            "ssh",
            "phase-transition"
          ]
        },
        rule: "future hardware proof evidence is not decisive unless selected-tree identity survives same-power-cycle TFTP-served kernel byte/count checks, final pre-restore identity, restore proof, serial freshness, and evidence-consistency checks; API-visible /status or /boot/files identity alone is not sufficient"
      }
    ')"

printf '%s\n' "$output"

if printf '%s\n' "$output" | jq -e '.staging_identity_ready == true' >/dev/null; then
    exit 0
fi

exit 1
