#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <selected-kernel-paired-sentinel-evidence.json>" >&2
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
    def source_pair:
      . as $root
      | ($root.evidence.selected_kernel_paired_sentinel
          // $root.selected_kernel_paired_sentinel
          // $root.paired_sentinel
          // (if (($root.candidate? // null) != null and ($root.control? // null) != null) then $root else {} end)) as $p
      | {
          selected_discriminator: ($p.selected_discriminator
            // $root.selected_discriminator
            // $root.checkpoint_decision.selected_discriminator
            // (if (($root.proof_result.checker // "") == "rpi5-selected-kernel-stability-discriminator-v1")
                then "selected-kernel-paired-sentinel-publication-boundary-v1"
                else null
                end)),
          no_command_write: (first_bool([$p.no_command_write, $root.no_command_write])),
          candidate: ($p.candidate
            // $root.candidate
            // (if (($root.proof_result.contract? // null) != null)
                then ($root.proof_result.contract + {selected_tree_hash: $root.post_publish_tree_hash})
                else {}
                end)),
          control: ($p.control // $root.control // {})
        };
    def normalized_run($run; $pair_no_command_write):
      ($run.v4 // {}) as $v4
      | ($run.post_publish_identity
          // $run.post_publish
          // $run.pre_power_identity
          // {}) as $post
      | ($run.tftp // $v4.tftp // {}) as $tftp
      | ($run.final_pre_restore
          // $run.final_pre_restore_identity
          // $v4.final_pre_restore
          // {}) as $final
      | ($run.restore // $v4.restore // {}) as $restore
      | (first_num([
          $run.expected_kernel_2712_size,
          $run.expected_fetch_bytes,
          $run.expected_fetch_byte_count,
          $v4.expected_fetch_byte_count,
          $post.expected_fetch_bytes,
          $post.observed_fetch_bytes
        ])) as $expected_bytes
      | (first_num([
          $tftp.expected_fetch_count,
          $run.expected_fetch_count
        ])) as $fetch_count
      | (first_num([
          $tftp.expected_fetch_byte_match_count,
          $run.expected_fetch_byte_match_count
        ])) as $match_count
      | (bytes_seen($tftp; $run)) as $seen
      | (bool_or_false([
          $run.no_command_write,
          $run.command_write_absent,
          $pair_no_command_write,
          (($run.serial_write? // null) == null and (($run.serial_writes? // []) | length) == 0)
        ])) as $no_write
      | {
          kind: ($run.kind // null),
          label: ($run.label // null),
          selected_tree_hash: ($post.selected_tree_hash // $post.tree_hash // $run.selected_tree_hash // $v4.selected_tree_hash),
          expected_fetch: ($run.expected_fetch // $post.expected_fetch // "da591740/kernel_2712.img"),
          expected_kernel_2712_size: $expected_bytes,
          no_command_write: $no_write,
          post_publish_identity: {
            selected_tree: (bool_or_false([
              $post.selected_tree,
              $post.tree_matches,
              $post.selected_tree_staged,
              $post.selected_tree_identity,
              $post.selected_tree_still_staged,
              (($post.selected_tree_hash // $post.tree_hash // $run.selected_tree_hash // $v4.selected_tree_hash) != null),
              (($run.gate.classification // "") == "selected-tree-identity-ready")
            ])),
            effective_kernel: (bool_or_false([
              $post.effective_kernel,
              $post.effective_kernel_matches,
              ($post.effective_kernel == "kernel_2712.img"),
              ($post.observed_effective_kernel == "kernel_2712.img"),
              ($v4.classification == "capture-chain-v4-ready")
            ])),
            expected_fetch_present: (bool_or_false([
              $post.expected_fetch_present,
              $post.expected_fetch_present,
              ($expected_bytes != null)
            ])),
            expected_fetch_bytes_match: (bool_or_false([
              $post.expected_fetch_bytes_match,
              $post.expected_fetch_bytes_match,
              (($post.expected_fetch_bytes | num) == $expected_bytes),
              (($post.observed_fetch_bytes | num) == $expected_bytes),
              ($v4.expected_fetch_byte_count == $expected_bytes)
            ]))
          },
          tftp: {
            cursor_start: (first_num([$tftp.cursor_start, $tftp.delta_cursor_start, $tftp.cursor_file])),
            cursor_end: (first_num([$tftp.cursor_end, $tftp.delta_cursor_end])),
            stable: (($tftp.stable // $run.tftp_stable) == true),
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
              (($final.selected_tree_hash // $final.tree_hash) == ($post.selected_tree_hash // $post.tree_hash // $run.selected_tree_hash // $v4.selected_tree_hash))
            ])),
            effective_kernel: (bool_or_false([
              $final.effective_kernel,
              ($final.effective_kernel == "kernel_2712.img"),
              ($final.observed_effective_kernel == "kernel_2712.img")
            ])),
            expected_fetch_present: (bool_or_false([
              $final.expected_fetch_present,
              $final.expected_fetch_present,
              (($final.expected_fetch_bytes | num) != null)
            ])),
            expected_fetch_bytes_match: (bool_or_false([
              $final.expected_fetch_bytes_match,
              $final.expected_fetch_bytes_match,
              (($final.expected_fetch_bytes | num) == $expected_bytes),
              (($final.observed_fetch_bytes | num) == $expected_bytes)
            ]))
          },
          restore: {
            ok: (bool_or_false([
              $restore.ok,
              $run.restore_ok,
              (($restore.post_restore_tree_hash // null) != null)
            ])),
            post_restore_tree_hash: ($restore.post_restore_tree_hash // $restore.tree_hash // null)
          }
        };
    def run_failure($r):
      if ($r.expected_kernel_2712_size == null) then
        "missing expected selected kernel byte count"
      elif (($r.selected_tree_hash // null) == null) then
        "missing selected tree identity"
      elif ($r.no_command_write == true) | not then
        "command write evidence present or no-command-write guard missing"
      elif ($r.post_publish_identity.selected_tree
            and $r.post_publish_identity.effective_kernel
            and $r.post_publish_identity.expected_fetch_present
            and $r.post_publish_identity.expected_fetch_bytes_match) | not then
        "post-publish selected identity missing or mismatched"
      elif ($r.tftp.cursor_start == null
            or $r.tftp.cursor_end == null
            or ($r.tftp.cursor_end <= $r.tftp.cursor_start)) then
        "stale or ambiguous TFTP cursor boundary"
      elif (($r.tftp.stable == true)
            and (($r.tftp.expected_fetch_count // 0) > 0)) | not then
        "no stable fresh same-power-cycle TFTP fetch"
      elif ($r.tftp.expected_fetch_byte_match_count != $r.tftp.expected_fetch_count
            or (($r.tftp.expected_fetch_bytes_seen | length) != $r.tftp.expected_fetch_count)
            or (all($r.tftp.expected_fetch_bytes_seen[]; . == $r.expected_kernel_2712_size) | not)) then
        "same-power-cycle TFTP served bytes do not match selected kernel"
      elif ($r.final_pre_restore_identity.selected_tree
            and $r.final_pre_restore_identity.effective_kernel
            and $r.final_pre_restore_identity.expected_fetch_present
            and $r.final_pre_restore_identity.expected_fetch_bytes_match) | not then
        "final pre-restore selected identity missing or mismatched"
      elif ($r.restore.ok == true) | not then
        "restore proof missing or failed"
      else
        null
      end;
    source_pair as $p
    | (normalized_run($p.candidate; $p.no_command_write)) as $candidate
    | (normalized_run($p.control; $p.no_command_write)) as $control
    | (run_failure($candidate)) as $candidate_failure
    | (run_failure($control)) as $control_failure
    | (if ($p.selected_discriminator != "selected-kernel-paired-sentinel-publication-boundary-v1") then
         "missing or unexpected selected discriminator"
       elif $candidate_failure != null then
         ("candidate: " + $candidate_failure)
       elif $control_failure != null then
         ("control: " + $control_failure)
       elif ($candidate.selected_tree_hash == $control.selected_tree_hash
             or $candidate.expected_kernel_2712_size == $control.expected_kernel_2712_size) then
         "candidate/control selected identities are not distinct"
       else
         null
       end) as $failure
    | {
        task_id: "phase10-pi5-command0-selected-kernel-paired-sentinel-core-20260617",
        checker: "rpi5-selected-kernel-paired-sentinel-discriminator-v1",
        accepted: ($failure == null),
        classification: (if $failure == null then
          "selected-kernel-paired-sentinel-discriminator-accepted"
        else
          "selected-kernel-paired-sentinel-discriminator-rejected"
        end),
        selected_discriminator: $p.selected_discriminator,
        first_failing_invariant: $failure,
        contract: {
          candidate: $candidate,
          control: $control
        },
        selected_next_task: (if $failure == null then
          "phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof-20260617"
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
