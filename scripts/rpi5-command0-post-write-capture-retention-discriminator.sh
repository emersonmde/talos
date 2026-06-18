#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <command0-post-write-capture-retention-evidence.json>" >&2
    exit 2
fi

EVIDENCE_JSON="$1"

result="$(
    jq '
    def text_of($x): ($x.text // $x.retained_text // "");
    def has_text($needle): contains($needle);
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
    def responses_ok($obj):
      all(($obj.responses? // [])[];
        (.ok == true)
        and ((.encoding // "") == "utf-8")
        and ((.truncated // false) == false)
      );
    def selected_identity_ok($c):
      (($c.identity.selected_kernel_tftp_ok // false) == true)
      and (($c.identity.same_attempt_selected_identity // false) == true)
      and (($c.identity.tftp_delta_selected // false) == true);
    def immediate_identity_ok($c):
      (($c.immediate_identity.selected_identity_ok // false) == true);
    def final_identity_ok($c):
      (($c.final_pre_restore_identity.selected_identity_ok // false) == true);
    def restore_ok($c):
      (($c.restore.ok // false) == true)
      and (($c.restore.baseline_restored // false) == true);
    def readiness_consumed_before_write($text):
      ($text | has_text("input-error timeout"))
      or ($text | has_text("line command=0"))
      or ($text | has_text("dispatch command=0"))
      or ($text | has_text("responses=1"))
      or ((command_advanced($text) // null) != null);
    def readiness_ok($obj):
      text_of($obj) as $text
      | ($text | has_text("source=firmware-initramfs"))
        and ($text | has_text("reason=valid-artifact"))
        and ($text | has_text("ready command=0"))
        and ($text | has_text("talos>"))
        and (readiness_consumed_before_write($text) | not)
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
      and ((num_or_null($obj.bytes) // 0) == 9);
    def post_capture_source_ok($obj):
      (($obj.capture_from_saved_boundary // false) == true)
      or (
        (($obj.capture_mode // "") == "serial/read")
        and (($obj.direct_read_equivalent_when_saturated // false) == true)
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
        and (($text | has_text("line command=2")) | not)
        and (($text | has_text("dispatch command=2")) | not)
        and (($text | has_text("ready command=2")) | not)
        and responses_ok($obj);
    def contract:
      . as $root
      | ($root.evidence // {}) as $e
      | {
          selected_discriminator: ($root.selected_discriminator // $e.selected_discriminator),
          identity: ($root.selected_identity // $root.identity // $e.selected_identity // $e.identity // {}),
          readiness: ($root.readiness // $e.readiness // {}),
          live_write_window: ($root.live_write_window // $e.live_write_window // {}),
          serial_write: ($root.serial_write // $e.serial_write // {}),
          post_write: ($root.post_write // $root.serial_observe // $e.post_write // $e.serial_observe // {}),
          immediate_identity: ($root.immediate_identity // $e.immediate_identity // {}),
          final_pre_restore_identity: ($root.final_pre_restore_identity // $e.final_pre_restore_identity // {}),
          restore: ($root.restore // $e.restore // {})
        };
    def classify($c):
      (text_of($c.readiness) | command_advanced(.)) as $pre_advanced
      | (text_of($c.post_write) | command_advanced(.)) as $post_advanced
      | if ($c.selected_discriminator != "command0-post-write-capture-retention-v1") then
          {kind: "inconclusive", reason: "missing or unexpected selected discriminator"}
        elif (selected_identity_ok($c) | not) then
          {kind: "inconclusive", reason: "selected-kernel/TFTP identity missing or mismatched"}
        elif ($pre_advanced != null or (text_of($c.readiness) | has_text("input-error timeout"))) then
          {kind: "blocked", reason: "stale pre-write output or timeout consumed command0 before write"}
        elif (readiness_ok($c.readiness) | not) then
          {kind: "blocked", reason: "fresh command0 readiness boundary missing"}
        elif (live_window_ok($c.live_write_window; $c.readiness; $c.post_write) | not) then
          {kind: "blocked", reason: "post-write capture is not tied to the saved fresh command0 boundary"}
        elif (write_ok($c.serial_write) | not) then
          {kind: "blocked", reason: "rootinfo serial write missing or mismatched"}
        elif (post_capture_source_ok($c.post_write) | not) then
          {kind: "blocked", reason: "post-write capture source is not the saved boundary cursor or direct-read equivalent"}
        elif ((text_of($c.post_write) | length) <= 2) then
          {kind: "blocked", reason: "empty or two-byte post-write capture retained without ordered command0 output"}
        elif ($post_advanced != null and $post_advanced > 1) then
          {kind: "blocked", reason: "post-write output advanced beyond command1 before ordered command0 delivery"}
        elif (command0_delivery_ok($c.post_write) | not) then
          {kind: "blocked", reason: "ordered command0 delivery missing or mismatched"}
        elif (immediate_identity_ok($c) | not) then
          {kind: "inconclusive", reason: "immediate selected identity proof missing or mismatched"}
        elif (final_identity_ok($c) | not) then
          {kind: "inconclusive", reason: "final pre-restore selected identity proof missing or mismatched"}
        elif (restore_ok($c) | not) then
          {kind: "inconclusive", reason: "post-run restore proof missing or mismatched"}
        else
          {kind: "accepted", reason: null}
        end;
    contract as $contract
    | classify($contract) as $status
    | {
        task_id: "phase10-pi5-command0-post-write-capture-retention-helper-core-20260618",
        checker: "rpi5-command0-post-write-capture-retention-discriminator-v1",
        accepted: ($status.kind == "accepted"),
        classification: (
          if $status.kind == "accepted" then
            "command0-post-write-capture-retention-accepted"
          elif $status.kind == "inconclusive" then
            "command0-post-write-capture-retention-inconclusive-triage-required"
          else
            "command0-post-write-capture-retention-blocked"
          end
        ),
        selected_discriminator: $contract.selected_discriminator,
        first_failing_invariant: $status.reason,
        contract: {
          selected_identity_ok: selected_identity_ok($contract),
          readiness_cursor: ($contract.readiness.cursor // null),
          post_write_cursor: ($contract.post_write.cursor // null),
          readiness_live_command0_boundary: readiness_ok($contract.readiness),
          live_write_window_ok: live_window_ok($contract.live_write_window; $contract.readiness; $contract.post_write),
          serial_write_ok: write_ok($contract.serial_write),
          post_capture_source_ok: post_capture_source_ok($contract.post_write),
          post_write_text_bytes: (text_of($contract.post_write) | length),
          ordered_command0_delivery: command0_delivery_ok($contract.post_write),
          immediate_selected_identity_ok: immediate_identity_ok($contract),
          final_pre_restore_selected_identity_ok: final_identity_ok($contract),
          restore_ok: restore_ok($contract)
        },
        allowed_terminal_classifications: [
          "command0-post-write-capture-retention-accepted",
          "command0-post-write-capture-retention-blocked",
          "command0-post-write-capture-retention-inconclusive-triage-required"
        ],
        selected_next_task: (if $status.kind == "accepted" then
          "phase10-pi5-command0-post-write-capture-retention-pi5-proof-20260618"
        else
          null
        end),
        rejected_claims: [
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
