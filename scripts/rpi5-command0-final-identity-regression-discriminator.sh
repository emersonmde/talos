#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <command0-final-identity-evidence.json>" >&2
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
    def path_bytes($identity; $path):
      if ($identity.files? | type) == "array" then
        ($identity.files[]? | select(.name == $path) | .bytes | num) // null
      else
        first_num([
          $identity.expected_fetch_bytes,
          $identity.observed_fetch_bytes,
          $identity.kernel_2712_size
        ])
      end;
    def identity_contract($identity; $selected_tree; $expected_bytes; $path):
      {
        captured: bool_or_false([
          $identity.captured,
          (($identity.tree_hash // $identity.selected_tree_hash) != null)
        ]),
        before_restore: bool_or_false([
          $identity.before_restore,
          ($identity.restore_already_performed == false)
        ]),
        before_cleanup: bool_or_false([
          $identity.before_cleanup,
          ($identity.cleanup_already_performed == false)
        ]),
        tree_hash: ($identity.tree_hash // $identity.selected_tree_hash),
        selected_tree: bool_or_false([
          $identity.selected_tree,
          $identity.selected_tree_identity,
          (($identity.tree_hash // $identity.selected_tree_hash) == $selected_tree)
        ]),
        effective_kernel: bool_or_false([
          $identity.effective_kernel_matches,
          ($identity.effective_kernel == "kernel_2712.img"),
          ($identity.observed_effective_kernel == "kernel_2712.img")
        ]),
        expected_fetch_present: ((path_bytes($identity; $path) // null) != null),
        expected_fetch_bytes: path_bytes($identity; $path),
        expected_fetch_bytes_match: ((path_bytes($identity; $path) // null) == $expected_bytes)
      };
    def source_contract:
      . as $root
      | ($root.contract // $root.evidence.contract // {}) as $contract
      | ($root.candidate_identity // $contract.candidate_identity // $root.evidence.candidate_identity // {}) as $candidate
      | ($root.pre_command // $root.evidence.pre_command // {}) as $pre
      | ($root.command0 // $root.evidence.command0 // {}) as $cmd
      | ($root.identity_samples // $root.evidence.identity_samples // {}) as $samples
      | ($root.restore // $root.evidence.restore // {}) as $restore
      | ($root.evidence_order // $root.ordering // []) as $order
      | ($candidate.selected_tree_hash // $root.selected_tree_hash) as $selected_tree
      | (first_num([
          $candidate.expected_kernel_2712_size,
          $candidate.expected_fetch_bytes,
          $root.expected_kernel_2712_size,
          $root.expected_fetch_bytes
        ])) as $expected_bytes
      | ($candidate.expected_fetch // $root.expected_fetch // "da591740/kernel_2712.img") as $expected_fetch
      | {
          selected_discriminator: ($root.selected_discriminator
            // $contract.selected_discriminator),
          selected_tree_hash: $selected_tree,
          expected_fetch: $expected_fetch,
          expected_kernel_2712_size: $expected_bytes,
          pre_command: {
            post_publish_identity_ok: bool_or_false([
              $pre.post_publish_identity_ok,
              $pre.selected_post_publish_identity_ok
            ]),
            selected_kernel_tftp_precondition_ok: bool_or_false([
              $pre.selected_kernel_tftp_precondition_ok,
              $pre.selected_kernel_precondition_ok
            ]),
            tftp_stable: bool_or_false([$pre.tftp_stable]),
            tftp_expected_fetch_count: first_num([$pre.tftp_expected_fetch_count]),
            tftp_expected_fetch_byte_match_count: first_num([$pre.tftp_expected_fetch_byte_match_count])
          },
          command0: {
            serial_write_ok: bool_or_false([$cmd.serial_write_ok, $cmd.write_ok]),
            serial_write_bytes: first_num([$cmd.serial_write_bytes, $cmd.write_bytes]),
            prearmed_read_ordered_command0: bool_or_false([
              $cmd.prearmed_read_ordered_command0,
              $cmd.ordered_command0
            ]),
            prearmed_read_has_rootinfo: bool_or_false([$cmd.prearmed_read_has_rootinfo]),
            prearmed_read_has_dispatch0: bool_or_false([$cmd.prearmed_read_has_dispatch0]),
            prearmed_read_has_responses1: bool_or_false([$cmd.prearmed_read_has_responses1]),
            prearmed_read_has_ready1: bool_or_false([$cmd.prearmed_read_has_ready1])
          },
          identity_samples: {
            post_publish: identity_contract(($samples.post_publish // {}); $selected_tree; $expected_bytes; $expected_fetch),
            immediate_post_command: identity_contract(($samples.immediate_post_command // $samples.post_command // {}); $selected_tree; $expected_bytes; $expected_fetch),
            final_pre_restore: identity_contract(($samples.final_pre_restore // {}); $selected_tree; $expected_bytes; $expected_fetch)
          },
          restore: {
            ok: bool_or_false([$restore.ok, $root.restore_ok]),
            after_restore_tree_hash: ($restore.after_restore_tree_hash // $restore.tree_hash)
          },
          ordering: {
            labels: (if ($order | type) == "array" then [$order[]? | if type == "object" then (.label // "") else . end] else [] end),
            immediate_post_command_before_restore: bool_or_false([
              $root.immediate_post_command_before_restore,
              $samples.immediate_post_command.before_restore,
              $samples.post_command.before_restore,
              (([$order[]? | if type == "object" then (.label // "") else . end] | index("immediate-post-command-identity")) != null
                and (([$order[]? | if type == "object" then (.label // "") else . end] | index("restore-request")) == null
                  or (([$order[]? | if type == "object" then (.label // "") else . end] | index("immediate-post-command-identity"))
                    < ([$order[]? | if type == "object" then (.label // "") else . end] | index("restore-request")))))
            ])
          }
        };
    def classify($c):
      if ($c.selected_discriminator != "command0-final-identity-regression-v1") then
        "missing or unexpected selected discriminator"
      elif ($c.expected_fetch != "da591740/kernel_2712.img") then
        "unexpected selected-kernel fetch path"
      elif (($c.selected_tree_hash // null) == null) then
        "missing selected tree identity"
      elif ($c.expected_kernel_2712_size == null) then
        "missing expected selected kernel byte count"
      elif ($c.pre_command.post_publish_identity_ok
            and $c.pre_command.selected_kernel_tftp_precondition_ok
            and $c.pre_command.tftp_stable
            and (($c.pre_command.tftp_expected_fetch_count // 0) > 0)
            and ($c.pre_command.tftp_expected_fetch_byte_match_count == $c.pre_command.tftp_expected_fetch_count)) | not then
        "selected-kernel/TFTP precondition missing or mismatched"
      elif ($c.command0.serial_write_ok
            and ($c.command0.serial_write_bytes == 9)
            and $c.command0.prearmed_read_ordered_command0
            and $c.command0.prearmed_read_has_rootinfo
            and $c.command0.prearmed_read_has_dispatch0
            and $c.command0.prearmed_read_has_responses1
            and $c.command0.prearmed_read_has_ready1) | not then
        "ordered command0 serial delivery missing or mismatched"
      elif ($c.identity_samples.immediate_post_command.captured
            and $c.identity_samples.immediate_post_command.before_restore
            and $c.identity_samples.immediate_post_command.before_cleanup
            and $c.ordering.immediate_post_command_before_restore) | not then
        "missing immediate post-command identity sample before restore/cleanup"
      elif ($c.identity_samples.immediate_post_command.selected_tree
            and $c.identity_samples.immediate_post_command.effective_kernel
            and $c.identity_samples.immediate_post_command.expected_fetch_present
            and $c.identity_samples.immediate_post_command.expected_fetch_bytes_match) | not then
        "immediate post-command selected identity missing or mismatched"
      elif ($c.identity_samples.final_pre_restore.captured
            and $c.identity_samples.final_pre_restore.before_restore
            and $c.identity_samples.final_pre_restore.selected_tree
            and $c.identity_samples.final_pre_restore.effective_kernel
            and $c.identity_samples.final_pre_restore.expected_fetch_present
            and $c.identity_samples.final_pre_restore.expected_fetch_bytes_match) | not then
        "final pre-restore selected identity missing or mismatched"
      elif ($c.restore.ok == true) | not then
        "restore proof missing or failed"
      else
        null
      end;
    source_contract as $contract
    | classify($contract) as $failure
    | {
        task_id: "phase10-pi5-command0-final-identity-regression-discriminator-core-20260618",
        checker: "rpi5-command0-final-identity-regression-discriminator-v1",
        accepted: ($failure == null),
        classification: (if $failure == null then
          "command0-final-identity-regression-discriminator-accepted"
        else
          "command0-final-identity-regression-discriminator-rejected"
        end),
        selected_discriminator: $contract.selected_discriminator,
        first_failing_invariant: $failure,
        contract: $contract,
        allowed_terminal_classifications: [
          "command0-final-identity-stable-input-delivery-accepted",
          "command0-final-identity-regressed-after-command0",
          "command0-final-identity-missing-immediate-post-command-sample",
          "command0-final-identity-command0-delivery-blocked",
          "command0-final-identity-precondition-blocked",
          "command0-final-identity-inconclusive-triage-required"
        ],
        selected_next_task: (if $failure == null then
          "phase10-pi5-command0-final-identity-regression-pi5-proof-20260618"
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
