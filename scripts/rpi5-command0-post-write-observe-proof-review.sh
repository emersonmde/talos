#!/bin/sh
set -eu

if [ "$#" -gt 1 ]; then
    echo "usage: $0 [post-write-observe-evidence.json]" >&2
    exit 2
fi

EVIDENCE_JSON="${1:-}"
GUARD="command0-post-write-observe-guard-v1"
PRELUDE_COMMAND="rootinfo"
PRELUDE_COMMAND_LINE_HEX="line command=0 hex=726f6f74696e666f"
PRELUDE_COMMAND_LINE_HEX_SPACED="line command=0 hex=72 6f 6f 74 69 6e 66 6f"
CANDIDATE_SELECTED_TREE="06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212"
EXPECTED_FETCH="da591740/kernel_2712.img"
EXPECTED_KERNEL_2712_SIZE="208984"

evidence_validation_status="not-requested"
if [ -n "$EVIDENCE_JSON" ]; then
    jq -e \
        --arg guard "$GUARD" \
        --arg prelude_command "$PRELUDE_COMMAND" \
        --arg prelude_command_line_hex "$PRELUDE_COMMAND_LINE_HEX" \
        --arg prelude_command_line_hex_spaced "$PRELUDE_COMMAND_LINE_HEX_SPACED" \
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
        def readiness_ok:
          text_of(.readiness // {}) as $text
          | ($text | has("source=firmware-initramfs"))
            and ($text | has("reason=valid-artifact"))
            and ($text | has("ready command=0"))
            and ($text | has("talos>"))
            and (($text | has("ready command=1")) | not)
            and (($text | has("dispatch command=0")) | not)
            and ((.readiness.fresh_after_prompt // true) == true);
        def pre_write_ok:
          text_of(.pre_write_boundary // .pre_write_read // {}) as $text
          | (($text | has($prelude_command)) | not)
            and (($text | has("line command=0")) | not)
            and (($text | has("dispatch command=0")) | not)
            and (($text | has("responses=1")) | not)
            and (($text | has("ready command=1")) | not)
            and ((.pre_write_boundary.fresh_after_prompt // .pre_write_read.fresh_after_prompt // true) == true);
        def write_ok:
          .serial_write.ok == true
          and .serial_write.text == $prelude_command
          and .serial_write.append_newline == true
          and ((.serial_write.bytes // 9) == 9);
        def observe_text: text_of(.serial_observe // .post_write_observe // {});
        def observe_ok:
          observe_text as $text
          | first_index($text; [$prelude_command, $prelude_command_line_hex, $prelude_command_line_hex_spaced]) as $line_pos
          | ($text | index("dispatch command=0 status=handled")) as $dispatch_pos
          | ($text | index("responses=1")) as $responses_pos
          | ($text | index("ready command=1")) as $ready_pos
          | ($text | index("dispatch command=1 status=input-error")) as $stale_later_dispatch_pos
          | ($text | index("ready command=2")) as $stale_later_ready_pos
          | $line_pos != null
            and $dispatch_pos != null
            and $responses_pos != null
            and $ready_pos != null
            and $line_pos < $dispatch_pos
            and $dispatch_pos <= $responses_pos
            and $responses_pos < $ready_pos
            and ($stale_later_dispatch_pos == null or $ready_pos < $stale_later_dispatch_pos)
            and ($stale_later_ready_pos == null or $ready_pos < $stale_later_ready_pos)
            and responses_ok(.serial_observe // .post_write_observe // {});
        (.label == "command0-post-write-observe")
          and (.guard == $guard)
          and readiness_ok
          and pre_write_ok
          and write_ok
          and ((.serial_observe.cursor // .post_write_observe.cursor // null) != null)
          and observe_ok
        ' "$EVIDENCE_JSON" >/dev/null
    evidence_validation_status="pass"
fi

jq -n \
    --arg guard "$GUARD" \
    --arg prelude_command "$PRELUDE_COMMAND" \
    --arg prelude_command_line_hex "$PRELUDE_COMMAND_LINE_HEX" \
    --arg prelude_command_line_hex_spaced "$PRELUDE_COMMAND_LINE_HEX_SPACED" \
    --arg candidate_selected_tree "$CANDIDATE_SELECTED_TREE" \
    --arg expected_fetch "$EXPECTED_FETCH" \
    --arg expected_kernel_2712_size "$EXPECTED_KERNEL_2712_SIZE" \
    --arg evidence_validation_status "$evidence_validation_status" \
    '{
      review: "rpi5-command0-post-write-observe-helper-core-v1",
      selected_command_contract: {
        guard: $guard,
        scenario: "cursor-bound-post-write-observe-command0-v1",
        command_index: 0,
        readiness: {
          required_fragments: [
            "source=firmware-initramfs",
            "reason=valid-artifact",
            "ready command=0",
            "talos>"
          ],
          rejected_fragments: [
            "dispatch command=0",
            "ready command=1"
          ]
        },
        pre_write_boundary: {
          required: true,
          cursor: "saved immediately after the visible command=0 prompt or immediately before the rootinfo write",
          must_not_contain: [
            $prelude_command,
            "line command=0",
            "dispatch command=0",
            "responses=1",
            "ready command=1"
          ]
        },
        serial_write: {
          endpoint: "POST /serial/write",
          text: $prelude_command,
          append_newline: true,
          required_bytes: 9
        },
        serial_observe: {
          endpoint: "POST /serial/observe",
          cursor: "saved pre-write cursor",
          required_ordered_fragments: [
            ($prelude_command + " or " + $prelude_command_line_hex + " or " + $prelude_command_line_hex_spaced),
            "dispatch command=0 status=handled",
            "responses=1",
            "ready command=1"
          ]
        },
        pi5_proof_requirements: {
          candidate_identity: {
            selected_tree_hash: $candidate_selected_tree,
            expected_fetch: $expected_fetch,
            expected_kernel_2712_size: ($expected_kernel_2712_size | tonumber)
          },
          pre_write_cursor_and_observe: [
            "record the fresh readiness cursor before the rootinfo write",
            "write rootinfo with append_newline=true",
            "capture post-write serial with POST /serial/observe from the saved cursor"
          ],
          selected_kernel_tftp: [
            "GET /tftp/logs delta must include selected da591740/kernel_2712.img fetches",
            "all selected-kernel TFTP fetch byte counts must equal 208984",
            "same-cursor re-query must keep the selected-kernel delta stable before restore"
          ],
          final_identity_and_restore: [
            "final pre-restore lab API identity still reports effective_kernel=kernel_2712.img and selected tree staged",
            "final pre-restore selected fetch bytes remain 208984",
            "post-run restore proof is present and ok before hardwareTestLock release"
          ]
        },
        terminal_classifications: {
          accepted: "command0-post-write-observe-accepted",
          blocked: "command0-post-write-observe-blocked",
          inconclusive: "command0-post-write-observe-inconclusive-triage-required"
        },
        evidence_validator: {
          optional_argument: "post-write-observe-evidence.json",
          status: $evidence_validation_status,
          accepts: [
            "ordered command 0 rootinfo or line marker after saved cursor",
            "dispatch command=0 status=handled after command text",
            "responses=1 before ready command=1",
            "utf-8 non-truncated observe responses when response metadata is present"
          ],
          rejects: [
            "/serial/write byte acceptance alone",
            "empty post-write observe windows",
            "stale pre-write output that already contains command0 output",
            "unordered command0 line/dispatch/responses/ready output",
            "stale later-command readiness such as ready command=2 before command0 completion",
            "source-response-only evidence without command0 line/rootinfo and dispatch ordering"
          ]
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
      selected_next_task: "phase10-pi5-serial-command0-post-write-observe-pi5-proof-20260617"
    }'
