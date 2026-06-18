#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <command0-readiness-timeout-boundary-evidence.json>" >&2
    exit 2
fi

EVIDENCE_JSON="$1"

result="$(
    jq '
    def text_of($x): ($x.text // $x.retained_text // "");
    def has($needle): contains($needle);
    def bool_or_false($xs):
      ($xs | map(select(. == true or . == false)) | first) // false;
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
      if ($text | test("line command=4|dispatch command=4")) then 4
      elif ($text | test("line command=3|dispatch command=3")) then 3
      elif ($text | test("line command=2|dispatch command=2")) then 2
      elif ($text | test("line command=1|dispatch command=1")) then 1
      else null
      end;
    def timeout_advanced($text): command_advanced($text);
    def readiness_ok($obj):
      text_of($obj) as $text
      | ($text | has("source=firmware-initramfs"))
        and ($text | has("reason=valid-artifact"))
        and ($text | has("ready command=0"))
        and ($text | has("talos>"))
        and (($text | has("input-error timeout")) | not)
        and (($text | has("line command=0")) | not)
        and (($text | has("dispatch command=0")) | not)
        and (($text | has("ready command=1")) | not)
        and (($obj.fresh_after_prompt // true) == true);
    def pre_write_ok($obj):
      text_of($obj) as $text
      | (($text | has("rootinfo")) | not)
        and (($text | has("line command=0")) | not)
        and (($text | has("dispatch command=0")) | not)
        and (($text | has("responses=1")) | not)
        and (($text | has("ready command=1")) | not)
        and ((timeout_advanced($text) // null) == null)
        and (($obj.fresh_after_prompt // true) == true);
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
        and ((timeout_advanced($text) // null) == null)
        and responses_ok($obj);
    def contract:
      . as $root
      | ($root.evidence // {}) as $e
      | {
          selected_discriminator: ($root.selected_discriminator // $e.selected_discriminator),
          readiness: ($root.readiness // $e.readiness // {}),
          pre_write_boundary: ($root.pre_write_boundary // $root.pre_write_read // $e.pre_write_boundary // $e.pre_write_read // {}),
          serial_write: ($root.serial_write // $e.serial_write // {}),
          post_write: ($root.post_write // $root.serial_observe // $root.prearmed_read // $root.direct_read // $e.post_write // $e.serial_observe // $e.prearmed_read // $e.direct_read // {})
        };
    def classify($c):
      (text_of($c.post_write) | timeout_advanced(.)) as $advanced
      | if ($c.selected_discriminator != "command0-readiness-timeout-boundary-v1") then
          "missing or unexpected selected discriminator"
        elif ($advanced != null) then
          "readiness-wait-timeout-advanced-to-command\($advanced)"
        elif (readiness_ok($c.readiness) | not) then
          "fresh command0 readiness boundary missing or stale"
        elif (pre_write_ok($c.pre_write_boundary) | not) then
          "stale retained output before rootinfo write"
        elif (write_ok($c.serial_write) | not) then
          "rootinfo serial write missing or mismatched"
        elif (command0_delivery_ok($c.post_write) | not) then
          "ordered command0 delivery missing or mismatched"
        else
          null
        end;
    contract as $contract
    | classify($contract) as $failure
    | {
        task_id: "phase10-pi5-command0-readiness-timeout-boundary-discriminator-core-20260618",
        checker: "rpi5-command0-readiness-timeout-boundary-discriminator-v1",
        accepted: ($failure == null),
        classification: (if $failure == null then
          "command0-readiness-timeout-boundary-discriminator-accepted"
        else
          "command0-readiness-timeout-boundary-discriminator-rejected"
        end),
        selected_discriminator: $contract.selected_discriminator,
        first_failing_invariant: $failure,
        contract: {
          selected_discriminator: $contract.selected_discriminator,
          readiness_text_bytes: (text_of($contract.readiness) | length),
          pre_write_text_bytes: (text_of($contract.pre_write_boundary) | length),
          post_write_text_bytes: (text_of($contract.post_write) | length),
          readiness_fresh_command0_boundary: readiness_ok($contract.readiness),
          pre_write_boundary_clean: pre_write_ok($contract.pre_write_boundary),
          serial_write_ok: write_ok($contract.serial_write),
          ordered_command0_delivery: command0_delivery_ok($contract.post_write)
        },
        allowed_terminal_classifications: [
          "command0-readiness-timeout-boundary-accepted",
          "command0-readiness-timeout-boundary-blocked",
          "command0-readiness-timeout-boundary-inconclusive-triage-required"
        ],
        selected_next_task: (if $failure == null then
          "phase10-pi5-command0-readiness-timeout-boundary-pi5-proof-20260618"
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
