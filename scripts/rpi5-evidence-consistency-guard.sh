#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-evidence-consistency-guard.sh --evidence-dir DIR --task-record FILE

Checks that a Pi 5 proof task's markdown, aggregate classification JSON,
capture summary JSON, evidence map, and candidate/control capture-chain-v4 JSON
agree about whether the proof is ready or blocked. The task-owned v4 JSON is
authoritative for the capture identity/freshness gate.
EOF
}

EVIDENCE_DIR=
TASK_RECORD=

while [ "$#" -gt 0 ]; do
    case "$1" in
        --evidence-dir)
            EVIDENCE_DIR="$2"
            shift 2
            ;;
        --task-record)
            TASK_RECORD="$2"
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

if [ -z "$EVIDENCE_DIR" ] || [ -z "$TASK_RECORD" ]; then
    usage
    exit 2
fi

require_file() {
    if [ ! -f "$1" ]; then
        echo "missing required evidence-consistency input: $1" >&2
        exit 2
    fi
}

require_file "$TASK_RECORD"
require_file "$EVIDENCE_DIR/classification.json"
require_file "$EVIDENCE_DIR/capture-summary.json"
require_file "$EVIDENCE_DIR/evidence-map.json"

repo_root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"

resolve_path() {
    path="$1"
    if [ -z "$path" ] || [ "$path" = "null" ]; then
        return 1
    fi
    case "$path" in
        /*)
            printf '%s\n' "$path"
            ;;
        *)
            printf '%s/%s\n' "$repo_root" "$path"
            ;;
    esac
}

candidate_v4_path="$(jq -r '
    .candidate.v4_check
    // (.artifacts[]? | select((.label // "") | test("candidate.*(v4|identity join)"; "i")) | .path)
    // empty
' "$EVIDENCE_DIR/evidence-map.json" | head -n 1)"

control_v4_path="$(jq -r '
    .control.v4_check
    // (.artifacts[]? | select((.label // "") | test("control.*(v4|identity join)"; "i")) | .path)
    // empty
' "$EVIDENCE_DIR/evidence-map.json" | head -n 1)"

if [ -z "$candidate_v4_path" ] || [ -z "$control_v4_path" ]; then
    echo "evidence map must reference candidate and control v4 check JSON" >&2
    exit 2
fi

candidate_v4="$(resolve_path "$candidate_v4_path")"
control_v4="$(resolve_path "$control_v4_path")"
require_file "$candidate_v4"
require_file "$control_v4"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT INT TERM

awk '
    /^## Candidate/ {flag=1; next}
    /^## Control/ {if (flag) exit}
    /^## [A-Z]/ {if (flag) exit}
    flag {print}
' "$TASK_RECORD" > "$tmp_dir/candidate-section.txt"

awk '
    /^## Control/ {flag=1; next}
    /^## [A-Z]/ {if (flag) exit}
    flag {print}
' "$TASK_RECORD" > "$tmp_dir/control-section.txt"

jq -n \
    --arg task_record "$TASK_RECORD" \
    --arg evidence_dir "$EVIDENCE_DIR" \
    --arg candidate_v4_path "$candidate_v4_path" \
    --arg control_v4_path "$control_v4_path" \
    --slurpfile classification "$EVIDENCE_DIR/classification.json" \
    --slurpfile capture "$EVIDENCE_DIR/capture-summary.json" \
    --slurpfile evidence_map "$EVIDENCE_DIR/evidence-map.json" \
    --slurpfile candidate_v4 "$candidate_v4" \
    --slurpfile control_v4 "$control_v4" \
    --rawfile task_text "$TASK_RECORD" \
    --rawfile candidate_text "$tmp_dir/candidate-section.txt" \
    --rawfile control_text "$tmp_dir/control-section.txt" \
    '
    def allowed_key:
      if has("decisive_rp1_hardware_classification_allowed") then
        .decisive_rp1_hardware_classification_allowed
      elif has("allowed") then
        .allowed
      else
        null
      end;

    def side_from_capture($side):
      ($capture[0][$side].capture_chain_v4
       // $capture[0][$side].identity_join
       // empty);

    def side_from_classification($side):
      ($classification[0][$side].capture_chain_v4
       // (if ($classification[0][$side].identity_join_classification? != null) then {
            classification: $classification[0][$side].identity_join_classification,
            decisive_rp1_hardware_classification_allowed:
              (if ($classification[0][$side] | has("decisive_rp1_hardware_classification_allowed")) then
                 $classification[0][$side].decisive_rp1_hardware_classification_allowed
               else
                 null
               end)
          } else empty end));

    def norm($doc):
      {
        classification: ($doc.classification // null),
        allowed: ($doc | allowed_key),
        rejection_reasons: ($doc.rejection_reasons // []),
        selected_tree_hash:
          ($doc.proof_run_identity.selected_tree_hash
           // $doc.selected_tree_hash
           // null),
        expected_fetch_count:
          ($doc.proof_run_identity.tftp.expected_fetch_count
           // $doc.expected_fetch_count
           // null),
        expected_fetch_byte_match_count:
          ($doc.proof_run_identity.tftp.expected_fetch_byte_match_count
           // $doc.expected_fetch_byte_match_count
           // null),
        final_pre_restore_tree_hash:
          ($doc.proof_run_identity.final_pre_restore.tree_hash
           // $doc.final_pre_restore_tree_hash
           // null),
        serial_marker_occurrences:
          ($doc.serial_capture.required_marker_occurrences
           // $doc.serial_marker_occurrences
           // null),
        serial_freshness_ok:
          (if ((($doc.serial_capture | type) == "object")
               and ($doc.serial_capture | has("serial_freshness_ok"))) then
             $doc.serial_capture.serial_freshness_ok
           elif $doc | has("serial_freshness_ok") then
             $doc.serial_freshness_ok
           else
             null
           end)
      };

    def has_ready_overclaim($text; $side):
      (($text | ascii_downcase) as $t
       | ($t | contains("capture-chain-v4-ready"))
         or ($t | contains("capture chain v4 ready"))
         or ($t | contains("capture-chain-v4 proof is ready"))
         or ($t | contains("capture-chain-v4 candidate/control identity and freshness ready"))
         or ($side == "candidate" and ($t | contains("candidate v4 replay accepted")))
         or ($side == "control" and ($t | contains("control v4 replay accepted")))
         or ($t | contains("decisive_rp1_hardware_classification_allowed=true")));

    def mismatch_reasons($side; $v4; $aggregate; $summary):
      [
        (if ($aggregate != null and ($aggregate.classification // null) != null and
             ($aggregate.classification != $v4.classification))
         then ($side + "-aggregate-classification-mismatch") else empty end),
        (if ($aggregate != null and ($aggregate.allowed // null) != null and
             ($aggregate.allowed != $v4.allowed))
         then ($side + "-aggregate-allowed-mismatch") else empty end),
        (if ($summary != null and ($summary.classification // null) != null and
             ($summary.classification != $v4.classification))
         then ($side + "-capture-summary-classification-mismatch") else empty end),
        (if ($summary != null and ($summary.allowed // null) != null and
             ($summary.allowed != $v4.allowed))
         then ($side + "-capture-summary-allowed-mismatch") else empty end),
        (if ($summary != null and ($summary.expected_fetch_byte_match_count // null) != null and
             ($v4.expected_fetch_byte_match_count // null) != null and
             ($summary.expected_fetch_byte_match_count != $v4.expected_fetch_byte_match_count))
         then ($side + "-capture-summary-tftp-byte-match-count-mismatch") else empty end),
        (if ($summary != null and ($summary.final_pre_restore_tree_hash // null) != null and
             ($v4.final_pre_restore_tree_hash // null) != null and
             ($summary.final_pre_restore_tree_hash != $v4.final_pre_restore_tree_hash))
         then ($side + "-capture-summary-final-tree-mismatch") else empty end),
        (if ($summary != null and ($summary.serial_marker_occurrences // null) != null and
             ($v4.serial_marker_occurrences // null) != null and
             ($summary.serial_marker_occurrences != $v4.serial_marker_occurrences))
         then ($side + "-capture-summary-serial-marker-count-mismatch") else empty end),
        (if ($summary != null and ($summary.serial_freshness_ok // null) != null and
             ($v4.serial_freshness_ok // null) != null and
             ($summary.serial_freshness_ok != $v4.serial_freshness_ok))
         then ($side + "-capture-summary-serial-freshness-mismatch") else empty end)
      ];

    ($candidate_v4[0] | norm(.)) as $candidate
    | ($control_v4[0] | norm(.)) as $control
    | (side_from_classification("candidate") | norm(.)) as $candidate_aggregate
    | (side_from_classification("control") | norm(.)) as $control_aggregate
    | (side_from_capture("candidate") | norm(.)) as $candidate_summary
    | (side_from_capture("control") | norm(.)) as $control_summary
    | (($classification[0].accepted_claims // []) + ($capture[0].accepted_claims // []) + ($evidence_map[0].accepted_claims // [])) as $accepted_claims
    | ($accepted_claims | map(tostring) | join("\n")) as $accepted_claims_text
    | (
        mismatch_reasons("candidate"; $candidate; $candidate_aggregate; $candidate_summary)
        + mismatch_reasons("control"; $control; $control_aggregate; $control_summary)
        + (if ($candidate.classification != "capture-chain-v4-ready" or $candidate.allowed != true)
           then [
             (if has_ready_overclaim($accepted_claims_text; "candidate") then "aggregate-claims-candidate-ready-overclaim" else empty end),
             (if has_ready_overclaim($candidate_text; "candidate") then "task-markdown-candidate-ready-overclaim" else empty end)
           ] else [] end)
        + (if ($control.classification != "capture-chain-v4-ready" or $control.allowed != true)
           then [
             (if has_ready_overclaim($accepted_claims_text; "control") then "aggregate-claims-control-ready-overclaim" else empty end),
             (if has_ready_overclaim($control_text; "control") then "task-markdown-control-ready-overclaim" else empty end)
           ] else [] end)
      ) as $reasons
    | {
        contract_version: "pi5-evidence-consistency-guard-v1",
        task_record: $task_record,
        evidence_dir: $evidence_dir,
        authoritative_inputs: {
          candidate_v4_check: $candidate_v4_path,
          control_v4_check: $control_v4_path
        },
        candidate: {
          v4: $candidate,
          aggregate_classification: $candidate_aggregate,
          capture_summary: $candidate_summary
        },
        control: {
          v4: $control,
          aggregate_classification: $control_aggregate,
          capture_summary: $control_summary
        },
        accepted_claims_checked: $accepted_claims,
        task_markdown_claims: {
          candidate_section_ready_overclaim: has_ready_overclaim($candidate_text; "candidate"),
          control_section_ready_overclaim: has_ready_overclaim($control_text; "control")
        },
        consistent: (($reasons | length) == 0),
        classification: (if (($reasons | length) == 0)
                         then "evidence-consistency-ready"
                         else "evidence-consistency-blocked" end),
        rejection_reasons: $reasons,
        rule: "candidate/control v4 JSON is authoritative; task markdown, aggregate classification JSON, capture summary, and evidence map accepted claims must not claim capture-chain-v4 readiness or matching identity/freshness when v4 JSON blocks it"
      }
    ' > "$tmp_dir/result.json"

cat "$tmp_dir/result.json"

if jq -e '.consistent == true' "$tmp_dir/result.json" >/dev/null; then
    exit 0
fi

exit 1
