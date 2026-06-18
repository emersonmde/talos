#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <command0-live-write-window-evidence.json>" >&2
    exit 2
fi

EVIDENCE_JSON="$1"

result="$(
    jq '
    def text_of($x): ($x.text // $x.retained_text // "");
    def has($needle): contains($needle);
    def num_or_null($x):
      if $x == null then null
      elif ($x | type) == "number" then $x
      elif ($x | type) == "string" then ($x | tonumber?)
      else null
      end;
    def first_index($text; $needles):
      reduce $needles[] as $needle
        (null; if . == null then ($text | index($needle)) else . end);
    def command_advanced($text):
      if ($text | test("line command=4|dispatch command=4|ready command=4")) then 4
      elif ($text | test("line command=3|dispatch command=3|ready command=3")) then 3
      elif ($text | test("line command=2|dispatch command=2|ready command=2")) then 2
      elif ($text | test("line command=1|dispatch command=1|ready command=1")) then 1
      elif ($text | test("line command=0|dispatch command=0")) then 0
      else null
      end;
    def command0_consumed_before_write($text):
      ($text | has("input-error timeout"))
      or ($text | has("line command=0"))
      or ($text | has("dispatch command=0"))
      or ($text | has("responses=1"))
      or ((command_advanced($text) // null) != null);
    def readiness_ok($obj):
      text_of($obj) as $text
      | ($text | has("source=firmware-initramfs"))
        and ($text | has("reason=valid-artifact"))
        and ($text | has("ready command=0"))
        and ($text | has("talos>"))
        and (command0_consumed_before_write($text) | not)
        and (($obj.fresh_after_prompt // false) == true)
        and (($obj.cursor // null) != null);
    def live_window_ok($live; $readiness; $post):
      ($live.write_issued_after_boundary == true)
      and (($live.write_immediate_after_boundary // false) == true)
      and (($live.boundary_was_live // false) == true)
      and (($live.write_waited_for_timeout // false) == false)
      and (($live.pre_write_drain_attempts_after_boundary // 0) == 0)
      and ((num_or_null($live.max_boundary_to_write_ms) // 0) <= 5000)
      and (($live.boundary_cursor // $readiness.cursor) == $readiness.cursor)
      and (($post.cursor // $live.boundary_cursor // $readiness.cursor) == $readiness.cursor);
    def write_ok($obj):
      ($obj.ok == true)
      and (($obj.text // "") == "rootinfo")
      and (($obj.append_newline // false) == true)
      and ((num_or_null($obj.bytes) // 9) == 9);
    def responses_ok($obj):
      all(($obj.responses? // [])[];
        (.ok == true)
        and ((.encoding // "") == "utf-8")
        and ((.truncated // false) == false)
      );
    def command0_delivery_ok($obj):
      text_of($obj) as $text
      | first_index($text; [
          "rootinfo",
          "line command=0 hex=726f6f74696e666f",
          "line command=0 hex=72 6f 6f 74 69 6e 66 6f"
        ]) as $line_pos
      | ($text | index("dispatch command=0 status=handled")) as $dispatch_pos
      | ($text | index("responses=1")) as $responses_pos
      | ($text | index("ready command=1")) as $ready_pos
      | $line_pos != null
        and $dispatch_pos != null
        and $responses_pos != null
        and $ready_pos != null
        and $line_pos < $dispatch_pos
        and $dispatch_pos <= $responses_pos
        and $responses_pos < $ready_pos
        and ((command_advanced($text) // 0) <= 1)
        and (($text | has("line command=2")) | not)
        and (($text | has("dispatch command=2")) | not)
        and (($text | has("ready command=2")) | not)
        and responses_ok($obj);
    def contract:
      . as $root
      | ($root.evidence // {}) as $e
      | ($root.readiness // $e.readiness // {}) as $readiness
      | ($root.post_write // $root.serial_observe // $e.post_write // $e.serial_observe // {}) as $post
      | {
          selected_discriminator: ($root.selected_discriminator // $e.selected_discriminator),
          readiness: $readiness,
          live_write_window: ($root.live_write_window // $e.live_write_window // {}),
          serial_write: ($root.serial_write // $e.serial_write // {}),
          post_write: $post
        };
    def classify($c):
      (text_of($c.readiness) | command_advanced(.)) as $pre_advanced
      | (text_of($c.post_write) | command_advanced(.)) as $post_advanced
      | if ($c.selected_discriminator != "command0-live-write-window-v1") then
          "missing or unexpected selected discriminator"
        elif ($pre_advanced != null or (text_of($c.readiness) | has("input-error timeout"))) then
          "retained output already advanced past the live command0 window before write"
        elif (readiness_ok($c.readiness) | not) then
          "fresh live command0 readiness boundary missing"
        elif (live_window_ok($c.live_write_window; $c.readiness; $c.post_write) | not) then
          "rootinfo write not tied to an immediate live command0 window"
        elif (write_ok($c.serial_write) | not) then
          "rootinfo serial write missing or mismatched"
        elif ($post_advanced != null and $post_advanced > 1) then
          "post-write output timeout-advanced beyond command0 before ordered delivery"
        elif (command0_delivery_ok($c.post_write) | not) then
          "ordered command0 delivery missing or mismatched"
        else
          null
        end;
    contract as $contract
    | classify($contract) as $failure
    | {
        task_id: "phase10-pi5-command0-live-write-window-core-20260618",
        checker: "rpi5-command0-live-write-window-discriminator-v1",
        accepted: ($failure == null),
        classification: (if $failure == null then
          "command0-live-write-window-discriminator-accepted"
        else
          "command0-live-write-window-discriminator-rejected"
        end),
        selected_discriminator: $contract.selected_discriminator,
        first_failing_invariant: $failure,
        contract: {
          selected_discriminator: $contract.selected_discriminator,
          readiness_cursor: ($contract.readiness.cursor // null),
          post_write_cursor: ($contract.post_write.cursor // null),
          readiness_text_bytes: (text_of($contract.readiness) | length),
          post_write_text_bytes: (text_of($contract.post_write) | length),
          readiness_live_command0_boundary: readiness_ok($contract.readiness),
          live_write_window_ok: live_window_ok($contract.live_write_window; $contract.readiness; $contract.post_write),
          serial_write_ok: write_ok($contract.serial_write),
          ordered_command0_delivery: command0_delivery_ok($contract.post_write)
        },
        allowed_terminal_classifications: [
          "command0-live-write-window-accepted",
          "command0-live-write-window-blocked",
          "command0-live-write-window-inconclusive-triage-required"
        ],
        selected_next_task: (if $failure == null then
          "phase10-pi5-command0-live-write-window-pi5-proof-20260618"
        else
          null
        end),
        rejected_claims: [
          "command0 input delivery acceptance",
          "source-response retention",
          "generated-root command-input success",
          "storage",
          "networking",
          "SSH",
          "Phase 11/12 expansion",
          "phase transition"
        ]
      }
    ' "$EVIDENCE_JSON"
)"

printf '%s\n' "$result"

if [ "$(printf '%s\n' "$result" | jq -r '.accepted')" != "true" ]; then
    exit 1
fi
