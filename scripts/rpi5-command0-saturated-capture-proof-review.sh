#!/bin/sh
set -eu

if [ "$#" -gt 1 ]; then
    echo "usage: $0 [saturated-capture-evidence.json]" >&2
    exit 2
fi

EVIDENCE_JSON="${1:-}"
GUARD="command0-saturated-capture-guard-v1"
CAPTURE_CONTRACT="deadline-loop-direct-read-after-saturated-cursor"
PRELUDE_COMMAND="rootinfo"
PRELUDE_COMMAND_LINE_HEX="line command=0 hex=726f6f74696e666f"
PRELUDE_COMMAND_LINE_HEX_SPACED="line command=0 hex=72 6f 6f 74 69 6e 66 6f"
CANDIDATE_SELECTED_TREE="06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212"
EXPECTED_FETCH="da591740/kernel_2712.img"
EXPECTED_KERNEL_2712_SIZE="208984"
SATURATION_CURSOR="4194304"

evidence_validation_status="not-requested"
if [ -n "$EVIDENCE_JSON" ]; then
    jq -e \
        --arg guard "$GUARD" \
        --arg capture_contract "$CAPTURE_CONTRACT" \
        --arg prelude_command "$PRELUDE_COMMAND" \
        --arg prelude_command_line_hex "$PRELUDE_COMMAND_LINE_HEX" \
        --arg prelude_command_line_hex_spaced "$PRELUDE_COMMAND_LINE_HEX_SPACED" \
        --arg expected_fetch "$EXPECTED_FETCH" \
        --argjson expected_kernel_2712_size "$EXPECTED_KERNEL_2712_SIZE" \
        --argjson saturation_cursor "$SATURATION_CURSOR" \
        '
        def text_of($x): ($x.text // $x.retained_text // "");
        def has($needle): contains($needle);
        def first_index($text; $needles):
          reduce $needles[] as $needle
            (null; if . == null then ($text | index($needle)) else . end);
        def responses_ok($obj):
          all(($obj.responses? // [])[];
            (.ok == true)
            and (.encoding // "") == "utf-8"
            and (.truncated == false)
          );
        def tftp_precondition:
          .selected_kernel_tftp_precondition
          // .boot.selected_kernel_tftp_precondition
          // .direct_read_proof.boot.selected_kernel_tftp_precondition
          // {};
        def selected_kernel_tftp_precondition_ok:
          tftp_precondition as $p
          | (($p.expected_kernel_2712_size // .expected_kernel_2712_size // $expected_kernel_2712_size) | tonumber?) as $expected_bytes
          | (($p.tftp.expected_fetch_count // $p.expected_fetch_count // 0) | tonumber?) as $fetch_count
          | (($p.tftp.expected_fetch_byte_match_count // $p.expected_fetch_byte_match_count // 0) | tonumber?) as $match_count
          | (($p.tftp.expected_fetch_bytes_seen // $p.expected_fetch_bytes_seen // []) | map(tonumber?)) as $bytes_seen
          | (($p.final_pre_restore.effective_kernel // $p.final_pre_restore_effective_kernel // "") == "kernel_2712.img")
            and (($p.contract // "") == "selected-kernel-tftp-precondition-v1")
            and (($p.expected_fetch // "") == $expected_fetch)
            and $expected_bytes == $expected_kernel_2712_size
            and (($p.tftp.stable // $p.stable // false) == true)
            and $fetch_count > 0
            and $match_count == $fetch_count
            and ($bytes_seen | length) == $fetch_count
            and all($bytes_seen[]; . == $expected_kernel_2712_size)
            and (($p.final_pre_restore.selected_tree_still_staged // $p.final_pre_restore_selected_tree_still_staged // false) == true)
            and (($p.final_pre_restore.expected_fetch_present // $p.final_pre_restore_expected_fetch_present // false) == true)
            and (($p.final_pre_restore.expected_fetch_bytes // $p.final_pre_restore_expected_fetch_bytes // 0) | tonumber?) == $expected_kernel_2712_size
            and (($p.restore.ok // $p.restore_ok // false) == true);
        def readiness_ok:
          text_of(.readiness // {}) as $text
          | ($text | has("source=firmware-initramfs"))
            and ($text | has("reason=valid-artifact"))
            and ($text | has("ready command=0"))
            and ($text | has("talos>"))
            and (($text | has("dispatch command=0")) | not)
            and (($text | has("ready command=1")) | not)
            and (($text | has("ready command=2")) | not)
            and (($text | has("ready command=3")) | not)
            and ((.readiness.fresh_after_prompt // true) == true);
        def pre_write_ok:
          text_of(.pre_write_read // .pre_write_boundary // {}) as $text
          | (($text | has($prelude_command)) | not)
            and (($text | has("line command=0")) | not)
            and (($text | has("dispatch command=0")) | not)
            and (($text | has("responses=1")) | not)
            and (($text | has("ready command=1")) | not)
            and (($text | has("ready command=2")) | not)
            and (($text | has("ready command=3")) | not)
            and (($text | has("source=firmware-initramfs")) | not)
            and ((.pre_write_read.fresh_after_prompt // .pre_write_boundary.fresh_after_prompt // true) == true);
        def write_ok:
          .serial_write.ok == true
          and .serial_write.text == $prelude_command
          and .serial_write.append_newline == true
          and ((.serial_write.bytes // 9) == 9);
        def direct_read: .post_write_direct_read // .direct_read // .post_write_read // {};
        def saturated_capture_contract_ok:
          direct_read as $read
          | (($read.capture_contract // $read.observe_contract // $read.talos_serial_window.observe_contract // "") == $capture_contract)
            and (($read.capture_mode // $read.talos_serial_window.capture_mode // "read") == "read")
            and (($read.start_cursor_saturated // $read.talos_serial_window.start_cursor_saturated // (($read.cursor_start // $saturation_cursor) >= $saturation_cursor)) == true)
            and (($read.cursor_start // $read.talos_serial_window.cursor_start // $saturation_cursor) >= $saturation_cursor)
            and (($read.response_bytes // $read.bytes // $read.talos_serial_window.response_bytes // 0) | tonumber?) > 0;
        def command0_delivery_text_ok($text):
          first_index($text; [$prelude_command, $prelude_command_line_hex, $prelude_command_line_hex_spaced]) as $line_pos
          | ($text | index("dispatch command=0 status=handled")) as $dispatch_pos
          | ($text | index("responses=1")) as $responses_pos
          | ($text | index("ready command=1")) as $ready_pos
          | ($text | index("dispatch command=1 status=input-error")) as $stale_command1_pos
          | ($text | index("dispatch command=2")) as $stale_command2_pos
          | ($text | index("ready command=2")) as $stale_ready2_pos
          | ($text | index("ready command=3")) as $stale_ready3_pos
          | $line_pos != null
            and $dispatch_pos != null
            and $responses_pos != null
            and $ready_pos != null
            and $line_pos < $dispatch_pos
            and $dispatch_pos <= $responses_pos
            and $responses_pos < $ready_pos
            and ($stale_command1_pos == null or $ready_pos < $stale_command1_pos)
            and ($stale_command2_pos == null or $ready_pos < $stale_command2_pos)
            and ($stale_ready2_pos == null or $ready_pos < $stale_ready2_pos)
            and ($stale_ready3_pos == null or $ready_pos < $stale_ready3_pos);
        def post_write_direct_read_ok:
          saturated_capture_contract_ok
          and (direct_read as $read | text_of($read) as $text | command0_delivery_text_ok($text))
          and responses_ok(direct_read);
        (.label == "command0-saturated-capture")
          and (.guard == $guard)
          and selected_kernel_tftp_precondition_ok
          and readiness_ok
          and pre_write_ok
          and write_ok
          and post_write_direct_read_ok
        ' "$EVIDENCE_JSON" >/dev/null
    evidence_validation_status="pass"
fi

jq -n \
    --arg guard "$GUARD" \
    --arg capture_contract "$CAPTURE_CONTRACT" \
    --arg prelude_command "$PRELUDE_COMMAND" \
    --arg prelude_command_line_hex "$PRELUDE_COMMAND_LINE_HEX" \
    --arg prelude_command_line_hex_spaced "$PRELUDE_COMMAND_LINE_HEX_SPACED" \
    --arg candidate_selected_tree "$CANDIDATE_SELECTED_TREE" \
    --arg expected_fetch "$EXPECTED_FETCH" \
    --argjson expected_kernel_2712_size "$EXPECTED_KERNEL_2712_SIZE" \
    --argjson saturation_cursor "$SATURATION_CURSOR" \
    --arg evidence_validation_status "$evidence_validation_status" \
    '{
      review: "rpi5-command0-saturated-capture-guard-core-v1",
      selected_command_contract: {
        guard: $guard,
        scenario: "command0-saturated-capture-after-observe-cursor-cap-v1",
        command_index: 0,
        capture_contract: $capture_contract,
        saturation_cursor: $saturation_cursor,
        selected_kernel_tftp_precondition: {
          guard: "selected-kernel-tftp-precondition-v1",
          selected_tree_hash: $candidate_selected_tree,
          expected_fetch: $expected_fetch,
          expected_kernel_2712_size: $expected_kernel_2712_size,
          required_boundaries: [
            "same-power-cycle TFTP delta is stable before restore",
            "all selected-kernel fetches are 208984 bytes",
            "final pre-restore identity still exposes kernel_2712.img at 208984 bytes",
            "post-run restore proof is present and ok"
          ]
        },
        readiness: {
          required_fragments: [
            "source=firmware-initramfs",
            "reason=valid-artifact",
            "ready command=0",
            "talos>"
          ],
          rejected_fragments: [
            "dispatch command=0",
            "ready command=1",
            "ready command=2",
            "ready command=3"
          ]
        },
        pre_write_read: {
          endpoint: "POST /serial/read",
          required: true,
          must_not_contain: [
            $prelude_command,
            "line command=0",
            "dispatch command=0",
            "responses=1",
            "ready command=1",
            "ready command=2",
            "ready command=3",
            "source=firmware-initramfs"
          ]
        },
        serial_write: {
          endpoint: "POST /serial/write",
          text: $prelude_command,
          append_newline: true,
          required_bytes: 9
        },
        post_write_direct_read: {
          endpoint: "POST /serial/read",
          required_contract: $capture_contract,
          required_start_cursor_saturated: true,
          required_ordered_fragments: [
            ($prelude_command + " or " + $prelude_command_line_hex + " or " + $prelude_command_line_hex_spaced),
            "dispatch command=0 status=handled",
            "responses=1",
            "ready command=1"
          ]
        },
        evidence_validator: {
          optional_argument: "saturated-capture-evidence.json",
          status: $evidence_validation_status,
          accepts: [
            "selected-kernel/TFTP agreement for the 208984-byte da591740/kernel_2712.img candidate",
            "same-boot firmware-initramfs valid-artifact ready command=0 and visible prompt",
            "fresh pre-write direct-read boundary with no command0/later-command/source-response output",
            "accepted rootinfo write with append_newline=true and 9 bytes",
            "bounded post-write direct read labeled deadline-loop-direct-read-after-saturated-cursor",
            "ordered command0 rootinfo/line, dispatch command=0 status=handled, responses=1, and ready command=1"
          ],
          rejects: [
            "empty saturated observe or direct-read windows",
            "/serial/write byte acceptance alone",
            "prompt-only evidence",
            "stale pre-write output that already contains command0 output",
            "stale later-command-only output, including rootinfo after command=1 or command=2 timeouts",
            "unordered command0 fragments",
            "source-response-only evidence without command0 line/rootinfo and dispatch ordering"
          ]
        },
        terminal_classifications: {
          accepted: "command0-saturated-capture-accepted",
          blocked: "command0-saturated-capture-blocked",
          inconclusive: "command0-saturated-capture-inconclusive-triage-required"
        }
      },
      rejected_claims: [
        "command0 source-response retention success",
        "generated-root command-input success",
        "storage",
        "networking",
        "SSH",
        "Phase 11/12 expansion",
        "phase transition"
      ],
      selected_next_task: "phase10-pi5-serial-command0-saturated-capture-pi5-proof-20260617"
    }'
