#!/bin/sh
set -eu

usage() {
    cat >&2 <<'EOF'
usage: rpi5-capture-chain-v4-retained-fixtures.sh

Builds local replay fixtures from retained Pi 5 capture-chain blocker evidence
and checks the pi5-capture-chain-v4 helper/checker contract without hardware.
EOF
}

if [ "$#" -gt 0 ]; then
    case "$1" in
        -h|--help)
            usage
            exit 0
            ;;
        *)
            usage
            exit 2
            ;;
    esac
fi

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
CHECKER="$SCRIPT_DIR/rpi5-proof-identity-join-v4-check.sh"
SOURCE_ROOT="$REPO_ROOT/tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM

BASE_MARKER="TALOS: rp1-ethernet-gem-mid-decode-discriminator-candidate"
CONTROL_MARKER="TALOS: rp1-ethernet-gem-mid-decode-discriminator-control"
NONCE="fixture-v4"
MARKER="$BASE_MARKER capture-nonce=$NONCE"
CONTROL_MARKER_WITH_NONCE="$CONTROL_MARKER capture-nonce=$NONCE"

add_endpoint_fallback() {
    dir="$1"
    jq -n \
        '{
            endpoint: "GET /",
            curl_exit: 0,
            http_code: 404,
            body_bytes: 19,
            usable_for_selected_tree_identity: false,
            selected_tree_identity_source: "/boot/files",
            fallback_used: true,
            fallback_reason: "root-endpoint-http-non-2xx"
        }' > "$dir/pre-root-endpoint.json"
    jq '.selected_tree_identity_source = "/boot/files"' \
        "$dir/preflight-identity.json" > "$dir/preflight-identity.json.tmp"
    mv "$dir/preflight-identity.json.tmp" "$dir/preflight-identity.json"
}

make_candidate_ready() {
    dir="$1"
    cp -R "$SOURCE_ROOT/candidate-full-run" "$dir"
    add_endpoint_fallback "$dir"

    expected_fetch="$(jq -r '.expected_fetch' "$dir/preflight-identity.json")"
    expected_bytes="$(jq -r '.expected_fetch_bytes' "$dir/preflight-identity.json")"
    selected_tree="$(jq -r '.observed_tree_hash' "$dir/preflight-identity.json")"
    selected_kernel="$(jq -r '.observed_effective_kernel' "$dir/preflight-identity.json")"

    jq \
        --arg old "$BASE_MARKER" \
        --arg marker "$MARKER" \
        --arg nonce "$NONCE" \
        '(.text // "") as $old_text
         | (.text = ($old_text | gsub($old; $marker)))
         | .talos_serial_window.required_marker = $marker
         | .talos_serial_window.has_required_marker = true
         | .talos_serial_window.required_marker_occurrences = (((.text // "") | split($marker) | length) - 1)
         | .talos_serial_window.marker_nonce = $nonce
         | .talos_serial_window.nonce_token = ("capture-nonce=" + $nonce)
         | .talos_serial_window.nonce_token_occurrences = (((.text // "") | split("capture-nonce=" + $nonce) | length) - 1)
         | .talos_serial_window.required_marker_excerpt =
            ((.text // "") | .[(index($marker) - 80 | if . < 0 then 0 else . end):(index($marker) + ($marker | length) + 160)])' \
        "$dir/serial-observe-window.json" > "$dir/serial-observe-window.json.tmp"
    mv "$dir/serial-observe-window.json.tmp" "$dir/serial-observe-window.json"

    jq \
        --arg fetch "$expected_fetch" \
        --argjson bytes "$expected_bytes" \
        '(.tftp.events[] | select(.filename == $fetch and .status == "served") | .bytes) = $bytes
         | .talos_tftp_stability.stable = true' \
        "$dir/tftp-delta-stable-pre-restore.json" > "$dir/tftp-delta-stable-pre-restore.json.tmp"
    mv "$dir/tftp-delta-stable-pre-restore.json.tmp" "$dir/tftp-delta-stable-pre-restore.json"

    for file in final-pre-restore-status.json final-pre-restore-boot-files.json; do
        jq \
            --arg tree "$selected_tree" \
            --arg kernel "$selected_kernel" \
            --arg fetch "$expected_fetch" \
            --argjson bytes "$expected_bytes" \
            '.boot.tree_hash = $tree
             | .boot.effective_kernel = $kernel
             | (.boot.files[] | select(.name == $fetch) | .bytes) = $bytes' \
            "$dir/$file" > "$dir/$file.tmp"
        mv "$dir/$file.tmp" "$dir/$file"
    done
}

run_case() {
    name="$1"
    dir="$2"
    kind="$3"
    expected_allowed="$4"
    expected_classification="$5"
    expected_reasons="$6"
    out_file="$TMP_DIR/$name.out.json"
    err_file="$TMP_DIR/$name.err.txt"

    set +e
    "$CHECKER" --evidence-dir "$dir" --label "$name" --report-kind "$kind" --nonce "$NONCE" \
        > "$out_file" 2> "$err_file"
    exit_code="$?"
    set -e

    jq -n \
        --arg name "$name" \
        --arg dir "$dir" \
        --argjson exit_code "$exit_code" \
        --argjson expected_allowed "$expected_allowed" \
        --arg expected_classification "$expected_classification" \
        --argjson expected_reasons "$expected_reasons" \
        --slurpfile result "$out_file" \
        --rawfile stderr "$err_file" \
        '($result[0]) as $r
         | {
             name: $name,
             evidence_dir: $dir,
             exit_code: $exit_code,
             expected: {
                 allowed: $expected_allowed,
                 classification: $expected_classification,
                 rejection_reasons: $expected_reasons
             },
             stderr: $stderr,
             result: $r,
             checks: {
                 exit_code_matches:
                    ((($expected_allowed == true) and ($exit_code == 0))
                     or (($expected_allowed == false) and ($exit_code == 1))),
                 allowed_matches:
                    (($r.decisive_rp1_hardware_classification_allowed // false) == $expected_allowed),
                 classification_matches:
                    (($r.classification // "") == $expected_classification),
                 reasons_include_expected:
                    (($expected_reasons - ($r.rejection_reasons // [])) == []),
                 endpoint_fallback_retained:
                    (($r.endpoint_identity.fallback_used // null) == true),
                 selected_tree_source_retained:
                    (($r.endpoint_identity.selected_tree_identity_source // "") == "/boot/files")
             }
         } as $case
         | $case + {passed: ([$case.checks[]] | all)}'
}

accepted="$TMP_DIR/accepted-candidate"
make_candidate_ready "$accepted"

missing_identity="$TMP_DIR/missing-identity"
cp -R "$accepted" "$missing_identity"
jq '.observed_tree_hash = null' "$missing_identity/preflight-identity.json" > "$missing_identity/preflight-identity.json.tmp"
mv "$missing_identity/preflight-identity.json.tmp" "$missing_identity/preflight-identity.json"

missing_tftp="$TMP_DIR/missing-tftp"
cp -R "$accepted" "$missing_tftp"
expected_fetch="$(jq -r '.expected_fetch' "$missing_tftp/preflight-identity.json")"
jq --arg fetch "$expected_fetch" \
    '(.tftp.events) |= map(select(.filename != $fetch))' \
    "$missing_tftp/tftp-delta-stable-pre-restore.json" > "$missing_tftp/tftp-delta-stable-pre-restore.json.tmp"
mv "$missing_tftp/tftp-delta-stable-pre-restore.json.tmp" "$missing_tftp/tftp-delta-stable-pre-restore.json"

missing_final="$TMP_DIR/missing-final"
cp -R "$accepted" "$missing_final"
jq '.boot.tree_hash = "not-selected-tree"' \
    "$missing_final/final-pre-restore-status.json" > "$missing_final/final-pre-restore-status.json.tmp"
mv "$missing_final/final-pre-restore-status.json.tmp" "$missing_final/final-pre-restore-status.json"

missing_marker="$TMP_DIR/missing-marker"
cp -R "$accepted" "$missing_marker"
jq --arg marker "$MARKER" \
    '(.text // "") as $old_text
     | .text = ($old_text | gsub($marker; ""))
     | .talos_serial_window.has_required_marker = false
     | .talos_serial_window.required_marker_occurrences = 0
     | .talos_serial_window.nonce_token_occurrences = 0
     | .talos_serial_window.required_marker_excerpt = ""' \
    "$missing_marker/serial-observe-window.json" > "$missing_marker/serial-observe-window.json.tmp"
mv "$missing_marker/serial-observe-window.json.tmp" "$missing_marker/serial-observe-window.json"

stale_nonce="$TMP_DIR/stale-nonce"
cp -R "$accepted" "$stale_nonce"
jq --arg nonce "$NONCE" \
    '.responses[0].text = ((.responses[0].text // "") + " stale capture-nonce=" + $nonce)' \
    "$stale_nonce/serial-drain-before-power.json" > "$stale_nonce/serial-drain-before-power.json.tmp"
mv "$stale_nonce/serial-drain-before-power.json.tmp" "$stale_nonce/serial-drain-before-power.json"

missing_control_marker="$TMP_DIR/missing-control-marker"
cp -R "$SOURCE_ROOT/control-direct-run" "$missing_control_marker"
add_endpoint_fallback "$missing_control_marker"
jq \
    --arg marker "$CONTROL_MARKER_WITH_NONCE" \
    --arg nonce "$NONCE" \
    '.talos_serial_window.required_marker = $marker
     | .talos_serial_window.marker_nonce = $nonce
     | .talos_serial_window.nonce_token = ("capture-nonce=" + $nonce)
     | .talos_serial_window.nonce_token_occurrences = 0' \
    "$missing_control_marker/serial-observe-window.json" > "$missing_control_marker/serial-observe-window.json.tmp"
mv "$missing_control_marker/serial-observe-window.json.tmp" "$missing_control_marker/serial-observe-window.json"

case_accepted="$(run_case accepted-candidate "$accepted" candidate true capture-chain-v4-ready '[]')"
case_missing_identity="$(run_case missing-identity "$missing_identity" candidate false capture-staging-blocked '["missing-selected-tree-hash","final-pre-restore-selected-tree-mismatch"]')"
case_missing_tftp="$(run_case missing-tftp "$missing_tftp" candidate false capture-staging-blocked '["expected-fetch-not-observed-in-tftp-delta"]')"
case_missing_final="$(run_case missing-final "$missing_final" candidate false capture-staging-blocked '["final-pre-restore-selected-tree-mismatch"]')"
case_missing_marker="$(run_case missing-marker "$missing_marker" candidate false capture-staging-blocked '["required-marker-not-present-after-power","run-unique-capture-nonce-not-present-after-power"]')"
case_stale_nonce="$(run_case stale-nonce "$stale_nonce" candidate false capture-staging-blocked '["run-unique-capture-nonce-present-before-power"]')"
case_missing_control="$(run_case missing-control-marker "$missing_control_marker" control false capture-staging-blocked '["required-marker-not-present-after-power","required-control-marker-not-retained","run-unique-capture-nonce-not-present-after-power"]')"

printf '%s\n' "$case_accepted" > "$TMP_DIR/accepted.json"
printf '%s\n' "$case_missing_identity" > "$TMP_DIR/missing-identity.json"
printf '%s\n' "$case_missing_tftp" > "$TMP_DIR/missing-tftp.json"
printf '%s\n' "$case_missing_final" > "$TMP_DIR/missing-final.json"
printf '%s\n' "$case_missing_marker" > "$TMP_DIR/missing-marker.json"
printf '%s\n' "$case_stale_nonce" > "$TMP_DIR/stale-nonce.json"
printf '%s\n' "$case_missing_control" > "$TMP_DIR/missing-control.json"

result="$(jq -s \
    '{
        contract_version: "pi5-capture-chain-v4-retained-fixtures",
        source_evidence: "tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof",
        fixture_count: length,
        passed: all(.[]; .passed == true),
        fixtures: .
      }' \
    "$TMP_DIR/accepted.json" \
    "$TMP_DIR/missing-identity.json" \
    "$TMP_DIR/missing-tftp.json" \
    "$TMP_DIR/missing-final.json" \
    "$TMP_DIR/missing-marker.json" \
    "$TMP_DIR/stale-nonce.json" \
    "$TMP_DIR/missing-control.json")"

printf '%s\n' "$result"
printf '%s\n' "$result" | jq -e '.passed == true' >/dev/null
