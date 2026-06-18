#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <command0-tail-stable-source-response-evidence.json>" >&2
    exit 2
fi

EVIDENCE_JSON="$1"

result="$(
    jq '
    def text_of($x): ($x.text // $x.retained_text // "");
    def has_text($text; $needle): ($text | contains($needle));
    def first_index($text; $needles):
      reduce $needles[] as $needle
        (null; if . == null then ($text | index($needle)) else . end);
    def responses_ok($obj):
      all(($obj.responses? // [])[];
        (.ok == true)
        and ((.encoding // "") == "utf-8")
        and ((.truncated // false) == false)
      );
    def command0_response_ok($obj):
      text_of($obj) as $text
      | first_index($text; [
          "rootinfo",
          "line command=0 hex=726f6f74696e666f",
          "line command=0 hex=72 6f 6f 74 69 6e 66 6f"
        ]) as $line_pos
      | ($text | index("source=firmware-initramfs")) as $source_pos
      | ($text | index("reason=valid-artifact")) as $reason_pos
      | ($text | index("dispatch command=0 status=handled")) as $dispatch_pos
      | ($text | index("responses=1")) as $responses_pos
      | ($text | index("ready command=1")) as $ready_pos
      | $line_pos != null
        and $source_pos != null
        and $reason_pos != null
        and $dispatch_pos != null
        and $responses_pos != null
        and $ready_pos != null
        and $source_pos <= $reason_pos
        and $reason_pos < $dispatch_pos
        and $line_pos < $dispatch_pos
        and $dispatch_pos <= $responses_pos
        and $responses_pos < $ready_pos
        and ((has_text($text; "line command=1") | not)
          and (has_text($text; "dispatch command=1") | not)
          and (has_text($text; "ready command=2") | not))
        and responses_ok($obj);
    def write_ok($obj):
      ($obj.ok == true)
      and (($obj.text // "") == "rootinfo")
      and (($obj.append_newline // false) == true)
      and ((($obj.bytes // 0) | tonumber?) == 9);
    def contract:
      . as $root
      | ($root.evidence // {}) as $e
      | {
          selected_discriminator: ($root.selected_discriminator // $e.selected_discriminator),
          serial_write: ($root.serial_write // $e.serial_write // {}),
          command0: ($root.command0 // $root.post_write // $root.direct_read // $e.command0 // $e.post_write // $e.direct_read // {})
        };
    def classify($c):
      if ($c.selected_discriminator != "command0-tail-stable-source-response-v1") then
        {kind: "inconclusive", reason: "missing or unexpected selected discriminator"}
      elif (write_ok($c.serial_write) | not) then
        {kind: "blocked", reason: "rootinfo serial write missing or mismatched"}
      elif (command0_response_ok($c.command0) | not) then
        {kind: "blocked", reason: "same-command0 source/reason response retention missing or unordered"}
      else
        {kind: "accepted", reason: null}
      end;
    contract as $contract
    | classify($contract) as $status
    | {
        task_id: "phase10-pi5-rootinfo-tail-stable-source-response-core-20260618",
        checker: "rpi5-command0-tail-stable-source-response-discriminator-v1",
        accepted: ($status.kind == "accepted"),
        classification: (
          if $status.kind == "accepted" then
            "command0-tail-stable-source-response-accepted"
          elif $status.kind == "inconclusive" then
            "command0-tail-stable-source-response-inconclusive-triage-required"
          else
            "command0-tail-stable-source-response-blocked"
          end
        ),
        selected_discriminator: $contract.selected_discriminator,
        first_failing_invariant: $status.reason,
        contract: {
          serial_write_ok: write_ok($contract.serial_write),
          same_command0_source_response_retained: command0_response_ok($contract.command0),
          command0_text_bytes: (text_of($contract.command0) | length)
        },
        accepted_evidence_shape: "same command0 response retaining source=firmware-initramfs reason=valid-artifact, rootinfo/line command=0, dispatch command=0 status=handled, responses=1, and ready command=1",
        rejected_evidence_shapes: [
          "dispatch-only command0 metadata without source/reason",
          "source/reason text without command0 line/rootinfo and dispatch ordering",
          "later-command or stale command1/later source response",
          "truncated or non-utf8 response summaries"
        ],
        selected_next_task: (if $status.kind == "accepted" then
          "phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof-20260618"
        else
          null
        end),
        rejected_claims: [
          "source-response hardware acceptance",
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
