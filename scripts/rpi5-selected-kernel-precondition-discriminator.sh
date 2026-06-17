#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <selected-kernel-precondition-evidence.json>" >&2
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
    def bool_or_false($xs):
      first_bool($xs) // false;
    def bytes_seen($tftp; $fallback):
      if ($tftp.expected_fetch_bytes_seen? | type) == "array" then
        ($tftp.expected_fetch_bytes_seen | map(num))
      elif ($fallback.expected_fetch_bytes_seen? | type) == "array" then
        ($fallback.expected_fetch_bytes_seen | map(num))
      elif ($fallback.kernel_fetches? | type) == "array" then
        ($fallback.kernel_fetches | map(.bytes | num))
      else
        []
      end;
    def source_contract:
      . as $root
      | ($root.evidence.selected_kernel_precondition
          // $root.selected_kernel_precondition
          // $root.evidence.selected_kernel_tftp_precondition
          // $root.direct_read_proof.boot.selected_kernel_tftp_precondition
          // $root.selected_kernel_tftp_precondition
          // {}) as $p
      | ($p.post_publish_identity
          // $p.post_publish
          // $root.evidence.post_publish_identity
          // $root.post_publish_identity
          // $root.evidence.preflight_identity
          // {}) as $post
      | ($p.tftp // $root.evidence.tftp // $root.tftp // {}) as $tftp
      | ($p.final_pre_restore
          // $p.final_pre_restore_identity
          // $root.evidence.final_pre_restore_identity
          // $root.final_pre_restore_identity
          // {}) as $final
      | ($p.restore // $root.evidence.restore // $root.restore // {}) as $restore
      | (first_num([
          $p.expected_kernel_2712_size,
          $p.expected_fetch_bytes,
          $root.evidence.expected_fetch_bytes,
          $root.direct_read_proof.boot.kernel_2712_size,
          $root.candidate_archive.kernel_2712_size
        ])) as $expected_bytes
      | (first_num([
          $tftp.expected_fetch_count,
          $root.evidence.expected_fetch_count
        ])) as $fetch_count
      | (first_num([
          $tftp.expected_fetch_byte_match_count,
          $root.evidence.expected_fetch_byte_match_count
        ])) as $match_count
      | (bytes_seen($tftp; $root.evidence)) as $seen
      | (bool_or_false([
          $p.no_command_write,
          $root.no_command_write,
          $root.command_write_absent
        ])) as $no_command_write
      | (bool_or_false([
          $p.command_write_present,
          $root.command_write_present,
          (($root.serial_write? // null) != null),
          ((($root.serial_writes? // []) | length) > 0)
        ])) as $command_write_present
      | {
          selected_discriminator: ($p.selected_discriminator
            // $root.selected_discriminator),
          durable_unblocker_claim: (bool_or_false([
            $p.durable_unblocker_claim,
            $root.durable_unblocker_claim,
            $root.single_run_durable_unblocker,
            $root.standing_precondition_pass
          ])),
          no_command_write: $no_command_write,
          command_write_present: $command_write_present,
          expected_fetch: ($p.expected_fetch // $root.evidence.expected_fetch),
          selected_tree_hash: ($post.selected_tree_hash // $post.tree_hash // $p.selected_tree_hash // $root.selected_tree_hash),
          expected_kernel_2712_size: $expected_bytes,
          post_publish_identity: {
            selected_tree: (bool_or_false([
              $post.selected_tree,
              $post.tree_matches,
              $post.selected_tree_staged,
              $post.selected_tree_identity,
              (($post.selected_tree_hash // $post.tree_hash // $p.selected_tree_hash // $root.selected_tree_hash) != null)
            ])),
            effective_kernel: (bool_or_false([
              $post.effective_kernel_matches,
              ($post.effective_kernel == "kernel_2712.img"),
              ($post.observed_effective_kernel == "kernel_2712.img")
            ])),
            expected_fetch_present: (bool_or_false([
              $post.expected_fetch_present,
              (($post.expected_fetch_bytes | num) != null),
              (($post.observed_fetch_bytes | num) != null)
            ])),
            expected_fetch_bytes_match: (bool_or_false([
              $post.expected_fetch_bytes_match,
              (($post.expected_fetch_bytes | num) == $expected_bytes),
              (($post.observed_fetch_bytes | num) == $expected_bytes)
            ]))
          },
          tftp: {
            cursor_start: (first_num([$tftp.cursor_start, $tftp.delta_cursor_start])),
            cursor_end: (first_num([$tftp.cursor_end, $tftp.delta_cursor_end])),
            stable: (($tftp.stable // $root.evidence.tftp_stability.stable) == true),
            expected_fetch_count: $fetch_count,
            expected_fetch_byte_match_count: $match_count,
            expected_fetch_bytes_seen: $seen
          },
          final_pre_restore_identity: {
            selected_tree_hash: ($final.selected_tree_hash // $final.tree_hash),
            selected_tree: (bool_or_false([
              $final.selected_tree,
              $final.expected_tree_still_staged,
              $final.selected_tree_still_staged,
              $final.selected_tree_identity,
              (($final.selected_tree_hash // $final.tree_hash) == ($post.selected_tree_hash // $post.tree_hash // $p.selected_tree_hash // $root.selected_tree_hash))
            ])),
            effective_kernel: (bool_or_false([
              $final.effective_kernel_matches,
              ($final.effective_kernel == "kernel_2712.img"),
              ($final.observed_effective_kernel == "kernel_2712.img")
            ])),
            expected_fetch_present: (bool_or_false([
              $final.expected_fetch_present,
              (($final.expected_fetch_bytes | num) != null),
              (($final.observed_fetch_bytes | num) != null)
            ])),
            expected_fetch_bytes_match: (bool_or_false([
              $final.expected_fetch_bytes_match,
              (($final.expected_fetch_bytes | num) == $expected_bytes),
              (($final.observed_fetch_bytes | num) == $expected_bytes)
            ]))
          },
          restore: {
            ok: (bool_or_false([
              $restore.ok,
              $root.evidence.restore_ok,
              $root.restore_ok
            ]))
          }
        };
    def classify($c):
      if ($c.selected_discriminator != "selected-kernel-tftp-precondition-lab-boundary-v1") then
        "missing or unexpected selected discriminator"
      elif ($c.expected_fetch != "da591740/kernel_2712.img") then
        "unexpected selected-kernel fetch path"
      elif ($c.expected_kernel_2712_size == null) then
        "missing expected selected kernel byte count"
      elif (($c.selected_tree_hash // null) == null) then
        "missing selected tree identity"
      elif ($c.durable_unblocker_claim == true) then
        "single-run durable unblocker claim is not accepted"
      elif ($c.command_write_present == true or ($c.no_command_write == true | not)) then
        "command write evidence present or no-command-write guard missing"
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
    source_contract as $contract
    | classify($contract) as $failure
    | {
        task_id: "phase10-pi5-command0-selected-kernel-precondition-discriminator-core-20260617",
        checker: "rpi5-selected-kernel-precondition-discriminator-v1",
        accepted: ($failure == null),
        classification: (if $failure == null then
          "selected-kernel-precondition-discriminator-accepted"
        else
          "selected-kernel-precondition-discriminator-rejected"
        end),
        selected_discriminator: $contract.selected_discriminator,
        first_failing_invariant: $failure,
        contract: $contract,
        selected_next_task: (if $failure == null then
          "phase10-pi5-command0-selected-kernel-precondition-pi5-proof-20260617"
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
