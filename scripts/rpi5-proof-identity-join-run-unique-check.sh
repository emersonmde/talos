#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-proof-identity-join-run-unique-check.sh --evidence-dir DIR [--label LABEL] [--nonce NONCE]

Replays a retained Pi 5 proof bundle through the v3 checker, then requires the
serial freshness marker to include a run-unique capture nonce. This prevents a
same-shaped constant marker from satisfying the saturated direct-read
freshness rule.
EOF
}

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
V3_CHECK="$SCRIPT_DIR/rpi5-proof-identity-join-v3-check.sh"
EVIDENCE_DIR=
EXPECTED_LABEL=
EXPECTED_NONCE=

while [ "$#" -gt 0 ]; do
    case "$1" in
        --evidence-dir)
            if [ "$#" -lt 2 ]; then usage; exit 2; fi
            EVIDENCE_DIR="$2"
            shift 2
            ;;
        --label)
            if [ "$#" -lt 2 ]; then usage; exit 2; fi
            EXPECTED_LABEL="$2"
            shift 2
            ;;
        --nonce)
            if [ "$#" -lt 2 ]; then usage; exit 2; fi
            EXPECTED_NONCE="$2"
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

tmp_v3="$(mktemp)"
tmp_v3_err="$(mktemp)"
tmp_out="$(mktemp)"
trap 'rm -f "$tmp_v3" "$tmp_v3_err" "$tmp_out"' EXIT INT TERM

set +e
if [ -n "$EXPECTED_LABEL" ]; then
    "$V3_CHECK" --evidence-dir "$EVIDENCE_DIR" --label "$EXPECTED_LABEL" > "$tmp_v3" 2> "$tmp_v3_err"
else
    "$V3_CHECK" --evidence-dir "$EVIDENCE_DIR" > "$tmp_v3" 2> "$tmp_v3_err"
fi
v3_exit="$?"
set -e

jq -n \
    --arg expected_nonce "$EXPECTED_NONCE" \
    --argjson v3_exit "$v3_exit" \
    --slurpfile v3 "$tmp_v3" \
    --slurpfile drain "$EVIDENCE_DIR/serial-drain-before-power.json" \
    --slurpfile serial "$EVIDENCE_DIR/serial-observe-window.json" \
    --rawfile v3_stderr "$tmp_v3_err" \
    '($v3[0] // {}) as $base
     | (($base.v3_serial_freshness.required_marker // "") | tostring) as $marker
     | (($marker | capture("capture-nonce=(?<nonce>[A-Za-z0-9_.:-]+)")? // {}) | .nonce // "") as $marker_nonce
     | (if $expected_nonce != "" then $expected_nonce else $marker_nonce end) as $required_nonce
     | (if $required_nonce == "" then "" else ("capture-nonce=" + $required_nonce) end) as $nonce_token
     | ((($drain[0].responses // []) | map(.text // "") | join("")) | tostring) as $pre_power_serial_text
     | (($serial[0].text // "") | tostring) as $post_power_serial_text
     | (if $nonce_token == "" then 0 else ([((($pre_power_serial_text | split($nonce_token) | length) - 1)), 0] | max) end) as $pre_power_nonce_count
     | (if $nonce_token == "" then 0 else ([((($post_power_serial_text | split($nonce_token) | length) - 1)), 0] | max) end) as $post_power_nonce_count
     | ($required_nonce != "" and $pre_power_nonce_count == 0 and $post_power_nonce_count > 0) as $nonce_token_differential
     | (($base.rejection_reasons // [])
        - ["v3-serial-freshness-not-proven", "required-marker-not-present-after-power"]) as $base_rejections_without_nonce_repairable_serial
     | [
         (if $marker == "" then "missing-required-marker" else empty end),
         (if $required_nonce == "" then "missing-run-unique-capture-nonce" else empty end),
         (if $expected_nonce != "" and $marker_nonce != "" and $marker_nonce != $expected_nonce then "run-unique-capture-nonce-mismatch" else empty end),
         (if $required_nonce != "" and $pre_power_nonce_count > 0 then "run-unique-capture-nonce-present-before-power" else empty end),
         (if $required_nonce != "" and $post_power_nonce_count == 0 then "run-unique-capture-nonce-not-present-after-power" else empty end)
       ] as $run_unique_rejections
     | (if $nonce_token_differential
        then $base_rejections_without_nonce_repairable_serial
        else ($base.rejection_reasons // [])
        end) as $base_rejection_reasons
     | ($base_rejection_reasons + $run_unique_rejections) as $rejection_reasons
     | (((($base.decisive_rp1_hardware_classification_allowed // false) == true)
         or ($nonce_token_differential and ($base_rejections_without_nonce_repairable_serial | length) == 0))
        and ($run_unique_rejections | length) == 0) as $allowed
     | {
         contract_version: "pi5-capture-transaction-run-unique-v1",
         base_contract_version: ($base.contract_version // "pi5-capture-transaction-v3"),
         run_label: ($base.run_label // ""),
         decisive_rp1_hardware_classification_allowed: $allowed,
         classification: (if $allowed then "capture-transaction-run-unique-ready" else "capture-staging-blocked" end),
         rejection_reasons: $rejection_reasons,
         run_unique_serial_freshness: {
             rule: "v3 identity/freshness plus a run-unique capture-nonce token absent before power and present after power",
             required_marker: $marker,
             expected_nonce: (if $expected_nonce == "" then null else $expected_nonce end),
             required_nonce: (if $required_nonce == "" then null else $required_nonce end),
             nonce_token: (if $nonce_token == "" then null else $nonce_token end),
             marker_nonce: (if $marker_nonce == "" then null else $marker_nonce end),
             pre_power_nonce_count: $pre_power_nonce_count,
             post_power_nonce_count: $post_power_nonce_count,
             nonce_token_differential: $nonce_token_differential,
             run_unique_marker_ok: (($run_unique_rejections | length) == 0)
         },
         base_v3: $base,
         base_v3_exit: $v3_exit,
         base_v3_stderr: $v3_stderr,
         rule: "missing v3 proof or missing/mismatched run-unique capture nonce prevents decisive RP1 hardware classification"
       }' > "$tmp_out"

cat "$tmp_out"

if jq -e '.decisive_rp1_hardware_classification_allowed == true' "$tmp_out" >/dev/null; then
    exit 0
fi

exit 1
