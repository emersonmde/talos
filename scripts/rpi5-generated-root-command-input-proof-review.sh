#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <candidate-archive.tar.gz>" >&2
    exit 2
fi

ARCHIVE="$1"
INITRAMFS_FILE="initramfs_2712"
SERIAL_PREFIX="da591740"
PROOF_LABEL="rpi5-generated-root-boot-transport-proof"
CLASSIFICATION="pi5-generated-root-boot-transport-complete"
PRELUDE_COMMAND="rootinfo"
COMMAND="cat /generated/manifest.txt"
COMMAND_PATH="/generated/manifest.txt"
EXPECTED_OUTPUT="Talos generated-root external artifact A"

./scripts/rpi5-generated-root-boot-transport-candidate-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-generated-root-command-input-review.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT INT TERM

extract_dir="$work_dir/extract"
mkdir -p "$extract_dir"
tar -xzf "$ARCHIVE" -C "$extract_dir"

artifact_strings="$work_dir/artifact-strings.txt"
kernel_strings="$work_dir/kernel-strings.txt"
strings "$extract_dir/$INITRAMFS_FILE" >"$artifact_strings"
strings "$extract_dir/kernel_2712.img" >"$kernel_strings"

require_string() {
    file="$1"
    expected="$2"
    label="$3"
    if ! grep -Fq "$expected" "$file"; then
        echo "missing $label proof string: $expected" >&2
        exit 1
    fi
}

for file in "$INITRAMFS_FILE" "$SERIAL_PREFIX/$INITRAMFS_FILE"; do
    if [ ! -s "$extract_dir/$file" ]; then
        echo "candidate missing generated-root artifact: $file" >&2
        exit 1
    fi
done

if ! cmp -s "$extract_dir/$INITRAMFS_FILE" "$extract_dir/$SERIAL_PREFIX/$INITRAMFS_FILE"; then
    echo "serial-prefixed generated-root artifact differs from root artifact" >&2
    exit 1
fi

require_string "$artifact_strings" "$COMMAND_PATH" "artifact command path"
require_string "$artifact_strings" "$EXPECTED_OUTPUT" "artifact manifest output"
require_string "$kernel_strings" "$PROOF_LABEL" "kernel proof label"
require_string "$kernel_strings" "$CLASSIFICATION" "kernel classification"
require_string "$kernel_strings" "firmware-initramfs" "kernel source value"
require_string "$kernel_strings" "valid-artifact" "kernel source reason"
require_string "$kernel_strings" "$COMMAND" "kernel expected command"
require_string "$kernel_strings" ": ready command=" "kernel prompt readiness marker"
require_string "$kernel_strings" ": dispatch command=" "kernel dispatch marker"
require_string "$kernel_strings" "status=" "kernel dispatch status field"
require_string "$kernel_strings" "responses=" "kernel dispatch response-count field"
require_string "$kernel_strings" ": ready-for-next prompt=" "kernel ready-for-next marker"
require_string "$kernel_strings" ": PASS" "kernel pass marker"

if grep -Fq "$EXPECTED_OUTPUT" "$kernel_strings"; then
    kernel_contains_expected_output=true
else
    kernel_contains_expected_output=false
fi

jq -n \
    --arg archive "$ARCHIVE" \
    --arg archive_sha256 "$(sha256sum "$ARCHIVE" | awk '{print $1}')" \
    --arg kernel_sha256 "$(sha256sum "$extract_dir/kernel_2712.img" | awk '{print $1}')" \
    --arg kernel_size "$(wc -c < "$extract_dir/kernel_2712.img" | tr -d ' ')" \
    --arg artifact_sha256 "$(sha256sum "$extract_dir/$INITRAMFS_FILE" | awk '{print $1}')" \
    --arg artifact_size "$(wc -c < "$extract_dir/$INITRAMFS_FILE" | tr -d ' ')" \
    --arg proof_label "$PROOF_LABEL" \
    --arg classification "$CLASSIFICATION" \
    --arg prelude_command "$PRELUDE_COMMAND" \
    --arg command "$COMMAND" \
    --arg expected_output "$EXPECTED_OUTPUT" \
    --argjson kernel_contains_expected_output "$kernel_contains_expected_output" \
    '{
      review: "rpi5-generated-root-command-input-capture-harness-core-v2",
      archive: {
        path: $archive,
        sha256: $archive_sha256,
        kernel_2712_sha256: $kernel_sha256,
        kernel_2712_size: ($kernel_size | tonumber),
        generated_root_artifact_sha256: $artifact_sha256,
        generated_root_artifact_size: ($artifact_size | tonumber),
        root_and_serial_prefixed_artifact_match: true
      },
      selected_command_contract: {
        scenario: "pi5-generated-root-manifest-command-input-v1",
        proof_label: $proof_label,
        classification: $classification,
        prompt_readiness: [
          "same boot serial retains source=firmware-initramfs reason=valid-artifact",
          "serial retains rpi5-generated-root-boot-transport-proof: ready command=0",
          "serial retains visible talos> prompt before saving the command=0 write cursor"
        ],
        command_sequence: [
          {
            command_index: 0,
            purpose: "source-gate prelude required by the generated-root proof harness",
            serial_write: {
              endpoint: "POST /serial/write",
              text: $prelude_command,
              append_newline: true
            },
            serial_observe: {
              endpoint: "POST /serial/observe",
              cursor: "saved command=0 post-prompt cursor",
              required_response_fragments: [
                $prelude_command,
                "talos: generated-root ... source=firmware-initramfs reason=valid-artifact",
                "dispatch command=0 status=handled",
                "responses=1",
                "rpi5-generated-root-boot-transport-proof: ready command=1"
              ]
            }
          },
          {
            command_index: 1,
            purpose: "selected generated-root command-input acceptance hinge",
            serial_write: {
              endpoint: "POST /serial/write",
              text: $command,
              append_newline: true
            },
            serial_observe: {
              endpoint: "POST /serial/observe",
              cursor: "saved command=1 post-prompt cursor",
              required_response_fragments: [
                $command,
                $expected_output,
                "dispatch command=1 status=handled",
                "responses=1",
                "rpi5-generated-root-boot-transport-proof: ready command=2"
              ]
            }
          }
        ],
        serial_write: {
          endpoint: "POST /serial/write",
          text: $command,
          append_newline: true,
          acceptance_command_index: 1,
          prerequisite_command: $prelude_command
        },
        serial_observe: {
          endpoint: "POST /serial/observe",
          cursor: "saved command=1 post-prompt cursor",
          required_response_fragments: [
            $command,
            $expected_output,
            "dispatch command=1 status=handled",
            "responses=1",
            "ready command=2 or final PASS"
          ]
        },
        capture_strategy: {
          first_failing_invariant: "post-prompt /serial/write accepted bytes must become shell-visible command text in retained serial",
          command_terminator: "append_newline=true for both prelude and acceptance commands",
          timing: "write each command only after the matching ready command=N marker and visible talos> prompt are retained in the same boot",
          cursor_contract: "save a cursor after each matching prompt and retain command response with /serial/observe from that cursor; if cursor saturation forces /serial/read fallback, preserve the saturated-cursor classification and do not accept command input unless the command text and response are retained",
          direct_read_fallback: "diagnostic only after observe/cursor evidence is saturated or unavailable; direct-read-only output can block or support rerun triage but cannot replace retained command-index evidence"
        },
        allowed_terminal_classifications: [
          "generated-root-command-input-accepted",
          "command-input-write-ingress-missing",
          "command-input-observe-cursor-saturated",
          "command-input-command0-prelude-blocked",
          "command-input-command1-manifest-blocked",
          "unexpected-boot-identity",
          "restore-blocked"
        ],
        expected_output: $expected_output,
        expected_output_source_gate: "accepted only when same-boot serial also proves source=firmware-initramfs reason=valid-artifact",
        kernel_contains_expected_output: $kernel_contains_expected_output
      },
      static_checks: {
        candidate_archive_review: "pass",
        initramfs_root_and_serial_prefixed_artifacts: "pass",
        artifact_contains_manifest_command_and_expected_output: "pass",
        kernel_contains_prompt_dispatch_ready_and_pass_markers: "pass",
        kernel_contains_firmware_initramfs_valid_artifact_markers: "pass"
      },
      rejected_claims: [
        "Pi 5 command-input success before serialized hardware proof",
        "serial transact as retained acceptance evidence",
        "prompt visibility alone proves generated-root command input",
        "persistence",
        "writable filesystem",
        "SD/USB/block storage",
        "networking",
        "SSH",
        "Phase 11/12 expansion",
        "phase transition"
      ],
      selected_next_task: "phase10-pi5-generated-root-command-input-capture-harness-pi5-proof-20260617"
    }'
