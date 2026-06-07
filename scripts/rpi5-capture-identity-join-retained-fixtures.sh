#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
CHECK_SCRIPT="$SCRIPT_DIR/rpi5-proof-identity-join-check.sh"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

run_fixture() {
    name="$1"
    evidence_dir="$2"
    label="$3"
    expected_allowed="$4"
    expected_classification="$5"
    expected_reasons_json="$6"
    expected_empty_before_power="$7"
    expected_final_selected_tree="$8"
    expected_fetch_byte_count="$9"
    shift 9
    expected_tftp_fetch_count="$1"

    out_file="$TMP_DIR/$name.json"
    err_file="$TMP_DIR/$name.err"

    set +e
    "$CHECK_SCRIPT" --evidence-dir "$REPO_ROOT/$evidence_dir" --label "$label" \
        > "$out_file" 2> "$err_file"
    exit_code="$?"
    set -e

    jq -n \
        --arg name "$name" \
        --arg evidence_dir "$evidence_dir" \
        --argjson exit_code "$exit_code" \
        --argjson expected_allowed "$expected_allowed" \
        --arg expected_classification "$expected_classification" \
        --argjson expected_reasons "$expected_reasons_json" \
        --argjson expected_empty_before_power "$expected_empty_before_power" \
        --argjson expected_final_selected_tree "$expected_final_selected_tree" \
        --argjson expected_fetch_byte_count "$expected_fetch_byte_count" \
        --argjson expected_tftp_fetch_count "$expected_tftp_fetch_count" \
        --slurpfile result "$out_file" \
        --rawfile stderr "$err_file" \
        '($result[0]) as $r
         | {
             exit_code_matches:
                 ((($expected_allowed == true) and ($exit_code == 0))
                  or (($expected_allowed == false) and ($exit_code == 1))),
             allowed_matches:
                 (($r.decisive_rp1_hardware_classification_allowed // false)
                  == $expected_allowed),
             classification_matches:
                 (($r.classification // "") == $expected_classification),
             reasons_match:
                 ((($r.rejection_reasons // []) | sort)
                  == ($expected_reasons | sort)),
             empty_before_power_matches:
                 (($r.proof_run_identity.serial.pre_power_drain.empty_before_power // false)
                  == $expected_empty_before_power),
             final_selected_tree_matches:
                 (($r.proof_run_identity.final_pre_restore.selected_tree_still_staged // false)
                  == $expected_final_selected_tree),
             expected_fetch_byte_count_matches:
                 (($r.proof_run_identity.expected_fetch_byte_count // null)
                  == $expected_fetch_byte_count),
             tftp_expected_fetch_count_matches:
                 (($r.proof_run_identity.tftp.expected_fetch_count // null)
                  == $expected_tftp_fetch_count)
           } as $checks
         | {
             name: $name,
             evidence_dir: $evidence_dir,
             exit_code: $exit_code,
             expected: {
                 allowed: $expected_allowed,
                 classification: $expected_classification,
                 rejection_reasons: $expected_reasons,
                 empty_before_power: $expected_empty_before_power,
                 final_selected_tree: $expected_final_selected_tree,
                 fetch_byte_count: $expected_fetch_byte_count,
                 tftp_fetch_count: $expected_tftp_fetch_count
             },
             stderr: $stderr,
             result: $r,
             checks: $checks,
             passed: ([$checks[]] | all)
           }'
}

marker_visible="$TMP_DIR/marker-visible-rejected.json"
clean_rerun="$TMP_DIR/clean-candidate-rerun.json"
control="$TMP_DIR/no-mmio-control.json"

run_fixture \
    marker-visible-rejected \
    tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-pi5/diagnostic-rerun-selected-tree \
    phase11-rp1-gpio-status-diagnostic-pi5-20260607/diagnostic-rerun-selected-tree \
    false \
    capture-staging-blocked \
    '[
      "serial-drain-not-empty-before-power",
      "saturated-direct-read-without-empty-pre-power-drain",
      "tftp-expected-fetch-byte-mismatch",
      "final-pre-restore-selected-tree-mismatch",
      "final-pre-restore-expected-fetch-byte-mismatch"
    ]' \
    false \
    false \
    46336 \
    2 > "$marker_visible"

run_fixture \
    clean-candidate-rerun \
    tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-pi5/diagnostic-rerun-after-kg \
    phase11-rp1-gpio-status-diagnostic-pi5-20260607/diagnostic-rerun-after-kg \
    false \
    capture-staging-blocked \
    '["expected-fetch-not-observed-in-tftp-delta"]' \
    true \
    true \
    46336 \
    0 > "$clean_rerun"

run_fixture \
    no-mmio-control \
    tasks/evidence/2026-06-07-phase11-rp1-gpio-status-no-mmio-control-pi5/control-rerun-after-kg \
    phase11-rp1-gpio-status-no-mmio-control-pi5-20260607/control-rerun-after-kg \
    true \
    capture-transaction-v2-ready \
    '[]' \
    true \
    true \
    46160 \
    2 > "$control"

result="$(jq -s \
    '{
        contract_version: "pi5-capture-identity-join-retained-fixtures-v1",
        fixture_count: length,
        passed: all(.[]; .passed == true),
        fixtures: .
      }' \
    "$marker_visible" "$clean_rerun" "$control")"

printf '%s\n' "$result"
printf '%s\n' "$result" | jq -e '.passed == true' >/dev/null
