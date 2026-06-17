#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <selected-kernel-stability-evidence.json>" >&2
    exit 2
fi

EVIDENCE_JSON="$1"

result="$(
    jq '
    def num:
      if type == "number" then .
      elif type == "string" then tonumber?
      else null
      end;
    def first_num($xs):
      $xs | map(num) | map(select(. != null)) | first;
    def first_bool($xs):
      $xs | map(select(. == true or . == false)) | first;
    def selected_contract:
      . as $root
      | ($root.evidence.selected_kernel_stability
          // $root.selected_kernel_stability
          // $root.direct_read_proof.boot.selected_kernel_tftp_precondition
          // $root.evidence.selected_kernel_tftp_precondition
          // {}) as $p
      | ($p.tftp // $root.evidence.tftp // $root.tftp // {}) as $tftp
      | ($p.post_publish_identity
          // $root.evidence.post_publish_identity
          // $root.post_publish_identity
          // $root.evidence.preflight_identity
          // {}) as $post
      | ($p.final_pre_restore
          // $root.evidence.final_pre_restore_identity
          // $root.final_pre_restore_identity
          // {}) as $final
      | ($p.restore
          // $root.evidence.restore
          // $root.restore
          // {}) as $restore
      | (first_num([
          $p.expected_kernel_2712_size,
          $p.expected_fetch_bytes,
          $root.evidence.expected_fetch_bytes,
          $root.candidate_archive.kernel_2712_size,
          $root.evidence.preflight_identity.expected_fetch_bytes,
          $root.direct_read_proof.boot.kernel_2712_size
        ])) as $expected_bytes
      | (first_num([
          $tftp.expected_fetch_count,
          $root.evidence.expected_fetch_count
        ])) as $fetch_count
      | (first_num([
          $tftp.expected_fetch_byte_match_count,
          $root.evidence.expected_fetch_byte_match_count
        ])) as $match_count
      | (if ($tftp.expected_fetch_bytes_seen? | type) == "array" then
           ($tftp.expected_fetch_bytes_seen | map(num))
         elif ($root.evidence.expected_fetch_events? | type) == "array" then
           ($root.evidence.expected_fetch_events | map(.bytes | num))
         else
           []
         end) as $bytes_seen
      | (first_bool([
          $post.tree_matches,
          $post.selected_tree_staged,
          $post.selected_tree_identity,
          $post.selected_tree_still_staged
        ]) // false) as $post_tree_ok
      | (first_bool([
          $post.effective_kernel_matches,
          ($post.effective_kernel == "kernel_2712.img"),
          ($post.observed_effective_kernel == "kernel_2712.img")
        ]) // false) as $post_kernel_ok
      | (first_bool([
          $post.expected_fetch_present
        ]) // false) as $post_fetch_present
      | (first_bool([
          $post.expected_fetch_bytes_match,
          (($post.expected_fetch_bytes | num) == $expected_bytes),
          (($post.observed_fetch_bytes | num) == $expected_bytes)
        ]) // false) as $post_bytes_ok
      | (first_bool([
          $final.expected_tree_still_staged,
          $final.selected_tree_still_staged,
          $final.selected_tree_identity
        ]) // false) as $final_tree_ok
      | (first_bool([
          ($final.effective_kernel == "kernel_2712.img")
        ]) // false) as $final_kernel_ok
      | (first_bool([
          $final.expected_fetch_present
        ]) // false) as $final_fetch_present
      | (first_bool([
          $final.expected_fetch_bytes_match,
          (($final.expected_fetch_bytes | num) == $expected_bytes),
          (($final.observed_fetch_bytes | num) == $expected_bytes)
        ]) // false) as $final_bytes_ok
      | (first_bool([
          $restore.ok,
          $root.evidence.restore_ok,
          $root.restore_ok
        ]) // false) as $restore_ok
      | {
          checker: "rpi5-selected-kernel-stability-discriminator-v1",
          expected_fetch: ($p.expected_fetch // $root.evidence.expected_fetch // "da591740/kernel_2712.img"),
          expected_kernel_2712_size: $expected_bytes,
          post_publish_identity: {
            selected_tree: $post_tree_ok,
            effective_kernel: $post_kernel_ok,
            expected_fetch_present: $post_fetch_present,
            expected_fetch_bytes_match: $post_bytes_ok
          },
          tftp: {
            cursor_start: ($tftp.cursor_start | num),
            cursor_end: ($tftp.cursor_end | num),
            stable: (($tftp.stable // $root.evidence.tftp_stability.stable) == true),
            expected_fetch_count: $fetch_count,
            expected_fetch_byte_match_count: $match_count,
            expected_fetch_bytes_seen: $bytes_seen
          },
          final_pre_restore_identity: {
            selected_tree: $final_tree_ok,
            effective_kernel: $final_kernel_ok,
            expected_fetch_present: $final_fetch_present,
            expected_fetch_bytes_match: $final_bytes_ok
          },
          restore: {
            ok: $restore_ok
          }
        };
    def classify:
      selected_contract as $c
      | if ($c.expected_kernel_2712_size == null) then
          "missing expected selected kernel byte count"
        elif ($c.post_publish_identity.selected_tree
              and $c.post_publish_identity.effective_kernel
              and $c.post_publish_identity.expected_fetch_present
              and $c.post_publish_identity.expected_fetch_bytes_match) | not then
          "post-publish selected identity missing or mismatched"
        elif ($c.tftp.cursor_start == null
              or $c.tftp.cursor_end == null
              or ($c.tftp.cursor_end <= $c.tftp.cursor_start)) then
          "stale or ambiguous TFTP cursor boundary"
        elif (($c.tftp.stable == true)
              and (($c.tftp.expected_fetch_count // 0) > 0)) | not then
          "no stable fresh same-power-cycle TFTP fetch"
        elif ($c.tftp.expected_fetch_byte_match_count != $c.tftp.expected_fetch_count
              or (($c.tftp.expected_fetch_bytes_seen | length) != $c.tftp.expected_fetch_count)
              or (all($c.tftp.expected_fetch_bytes_seen[]; . == $c.expected_kernel_2712_size) | not)) then
          "same-power-cycle TFTP served bytes do not match selected kernel"
        elif ($c.final_pre_restore_identity.selected_tree
              and $c.final_pre_restore_identity.effective_kernel
              and $c.final_pre_restore_identity.expected_fetch_present
              and $c.final_pre_restore_identity.expected_fetch_bytes_match) | not then
          "final pre-restore selected identity missing or mismatched"
        elif ($c.restore.ok == true) | not then
          "restore proof missing or failed"
        else
          null
        end;
    selected_contract as $contract
    | classify as $failure
    | {
        task_id: "phase10-pi5-command0-selected-kernel-stability-discriminator-core-20260617",
        checker: $contract.checker,
        accepted: ($failure == null),
        classification: (if $failure == null then
          "selected-kernel-stability-discriminator-accepted"
        else
          "selected-kernel-stability-discriminator-rejected"
        end),
        first_failing_invariant: $failure,
        contract: $contract,
        selected_next_task: (if $failure == null then
          "phase10-pi5-command0-selected-kernel-stability-pi5-proof-20260617"
        else
          null
        end),
        rejected_claims: [
          "command0 write-delivery success",
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
