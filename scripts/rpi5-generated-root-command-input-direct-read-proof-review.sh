#!/bin/sh
set -eu

if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
    echo "usage: $0 <candidate-archive.tar.gz> [direct-read-evidence.json]" >&2
    exit 2
fi

ARCHIVE="$1"
EVIDENCE_JSON="${2:-}"
INITRAMFS_FILE="initramfs_2712"
SERIAL_PREFIX="da591740"
PROOF_LABEL="rpi5-generated-root-boot-transport-proof"
CLASSIFICATION="pi5-generated-root-boot-transport-complete"
PRELUDE_COMMAND="rootinfo"
PRELUDE_COMMAND_LINE_HEX="line command=0 hex=726f6f74696e666f"
COMMAND="cat /generated/manifest.txt"
COMMAND_LINE_HEX="line command=1 hex=636174202f67656e6572617465642f6d616e69666573742e747874"
COMMAND_PATH="/generated/manifest.txt"
EXPECTED_OUTPUT="Talos generated-root external artifact A"

./scripts/rpi5-generated-root-boot-transport-candidate-review.sh "$ARCHIVE" >/dev/null

tmp_root="target/tmp"
mkdir -p "$tmp_root"
work_dir="$(mktemp -d "$tmp_root/talos-rpi5-generated-root-command-input-direct-read-review.XXXXXX")"
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
require_string "$kernel_strings" "$PRELUDE_COMMAND" "kernel prelude command"
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

evidence_validation_status="not-requested"
if [ -n "$EVIDENCE_JSON" ]; then
    jq -e \
        --arg prelude_command "$PRELUDE_COMMAND" \
        --arg prelude_command_line_hex "$PRELUDE_COMMAND_LINE_HEX" \
        --arg command "$COMMAND" \
        --arg command_line_hex "$COMMAND_LINE_HEX" \
        --arg expected_output "$EXPECTED_OUTPUT" \
        '
        def text_of($x): ($x.text // $x.retained_text // "");
        def has($needle): contains($needle);
        def cmd($idx): .direct_read_proof.commands[] | select(.command_index == $idx);
        def first_index($text; $needles):
          reduce $needles[] as $needle
            (null; if . == null then ($text | index($needle)) else . end);
        def boot_ok:
          .direct_read_proof.boot.source == "firmware-initramfs"
          and .direct_read_proof.boot.reason == "valid-artifact"
          and .direct_read_proof.boot.selected_tree_identity == true
          and .direct_read_proof.boot.stable_tftp_delta == true
          and .direct_read_proof.boot.final_pre_restore_identity == true
          and .direct_read_proof.boot.restore_ok == true;
        def readiness_ok:
          (text_of(.direct_read_proof.readiness) | has("source=firmware-initramfs") and has("reason=valid-artifact") and has("ready command=0") and has("talos>"));
        def fresh_before($idx; $command_text; $line_text):
          (text_of((cmd($idx).pre_write_read // {})) | (has($command_text) or has($line_text) or has("dispatch command=\($idx)") or has($expected_output)) | not)
          and ((cmd($idx).pre_write_read.fresh_after_prompt // false) == true);
        def write_ok($idx; $command_text):
          cmd($idx).serial_write.ok == true
          and cmd($idx).serial_write.text == $command_text
          and cmd($idx).serial_write.append_newline == true;
        def direct_read_text($idx): text_of(cmd($idx).direct_read);
        def command0_ok:
          fresh_before(0; $prelude_command; $prelude_command_line_hex)
          and write_ok(0; $prelude_command)
          and (
            direct_read_text(0) as $text
            | first_index($text; [$prelude_command, $prelude_command_line_hex]) as $line_pos
            | ($text | index("talos: generated-root source=firmware-initramfs")) as $source_pos
            | ($text | index("reason=valid-artifact")) as $reason_pos
            | ($text | index("dispatch command=0 status=handled")) as $dispatch_pos
            | ($text | index("responses=1")) as $responses_pos
            | ($text | index("ready command=1")) as $ready_pos
            | ($text | index("dispatch command=1 status=input-error")) as $next_timeout_pos
            | $line_pos != null
              and $source_pos != null
              and $reason_pos != null
              and $dispatch_pos != null
              and $responses_pos != null
              and $ready_pos != null
              and $line_pos < $source_pos
              and $source_pos <= $reason_pos
              and $reason_pos < $dispatch_pos
              and $dispatch_pos <= $responses_pos
              and $responses_pos < $ready_pos
              and ($next_timeout_pos == null or $ready_pos < $next_timeout_pos)
          );
        def command1_ok:
          fresh_before(1; $command; $command_line_hex)
          and write_ok(1; $command)
          and (
            direct_read_text(1) as $text
            | first_index($text; [$command, $command_line_hex]) as $line_pos
            | ($text | index($expected_output)) as $output_pos
            | ($text | index("dispatch command=1 status=handled")) as $dispatch_pos
            | ($text | index("responses=1")) as $responses_pos
            | first_index($text; ["ready command=2", "ready-for-next prompt=true", "PASS"]) as $done_pos
            | $line_pos != null
              and $output_pos != null
              and $dispatch_pos != null
              and $responses_pos != null
              and $done_pos != null
              and $line_pos < $output_pos
              and $output_pos < $dispatch_pos
              and $dispatch_pos <= $responses_pos
              and $responses_pos < $done_pos
          );
        boot_ok and readiness_ok and command0_ok and command1_ok
        ' "$EVIDENCE_JSON" >/dev/null
    evidence_validation_status="pass"
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
    --arg prelude_command_line_hex "$PRELUDE_COMMAND_LINE_HEX" \
    --arg command "$COMMAND" \
    --arg command_line_hex "$COMMAND_LINE_HEX" \
    --arg expected_output "$EXPECTED_OUTPUT" \
    --arg evidence_validation_status "$evidence_validation_status" \
    --argjson kernel_contains_expected_output "$kernel_contains_expected_output" \
    '{
      review: "rpi5-generated-root-command-input-direct-read-harness-core-v1",
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
        scenario: "direct-read-after-saturated-cursor-command-input-v1",
        proof_label: $proof_label,
        classification: $classification,
        readiness: {
          endpoint: "POST /serial/read",
          required_fragments: [
            "source=firmware-initramfs",
            "reason=valid-artifact",
            "rpi5-generated-root-boot-transport-proof: ready command=0",
            "talos> "
          ]
        },
        command_sequence: [
          {
            command_index: 0,
            purpose: "source-gate prelude required before the manifest command",
            pre_write_freshness_read: {
              endpoint: "POST /serial/read",
              required: true,
              must_not_contain: [
                $prelude_command,
                "dispatch command=0",
                $expected_output
              ]
            },
            serial_write: {
              endpoint: "POST /serial/write",
              text: $prelude_command,
              append_newline: true
            },
            direct_read_window: {
              endpoint: "POST /serial/read",
              required_response_fragments: [
                ($prelude_command + " or " + $prelude_command_line_hex),
                "source=firmware-initramfs",
                "reason=valid-artifact",
                "dispatch command=0 status=handled",
                "responses=1",
                "rpi5-generated-root-boot-transport-proof: ready command=1"
              ]
            }
          },
          {
            command_index: 1,
            purpose: "selected generated-root manifest command-input acceptance hinge",
            pre_write_freshness_read: {
              endpoint: "POST /serial/read",
              required: true,
              must_not_contain: [
                $command,
                $command_line_hex,
                $expected_output,
                "dispatch command=1"
              ]
            },
            serial_write: {
              endpoint: "POST /serial/write",
              text: $command,
              append_newline: true
            },
            direct_read_window: {
              endpoint: "POST /serial/read",
              required_response_fragments: [
                ($command + " or " + $command_line_hex),
                $expected_output,
                "dispatch command=1 status=handled",
                "responses=1",
                "ready command=2 or ready-for-next prompt=true or final PASS"
              ]
            }
          }
        ],
        hardware_identity_requirements: [
          "candidate archive hash and selected tree identity",
          "stable same-power-cycle TFTP delta before restore",
          "final pre-restore identity matching the selected tree",
          "post-run restore proof"
        ],
        evidence_validator: {
          guard: "command0-write-to-next-ready-guard-v1",
          optional_argument: "direct-read-evidence.json",
          status: $evidence_validation_status,
          rejects: [
            "prompt-only evidence without command text and response output",
            "/serial/write-only evidence without a following direct-read window",
            "stale pre-write direct-read windows that already contain the command response",
            "missing dispatch status=handled responses=1 evidence",
            "missing firmware-initramfs valid-artifact source gate",
            "missing TFTP/final-identity/restore proof fields"
          ]
        },
        expected_output: $expected_output,
        expected_output_source_gate: "accepted only when same-boot direct-read output also proves source=firmware-initramfs reason=valid-artifact",
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
        "direct-read output without command-indexed pre-write freshness",
        "/serial/write byte acceptance alone proves command input",
        "prompt visibility alone proves generated-root command input",
        "persistence",
        "writable filesystem",
        "SD/USB/block storage",
        "networking",
        "SSH",
        "Phase 11/12 expansion",
        "phase transition"
      ],
      selected_next_task: "phase10-pi5-serial-command0-prelude-pi5-proof-20260617"
    }'
