# Phase 12.6 SSH selected-candidate fetch after root-cause v3

Task id: phase12-ssh-selected-candidate-fetch-after-root-cause-v3-20260623
Status: accepted
Owner: worker
Classification: stable-zero-tftp-after-selected-publish

## Goal

Run one root-cause-informed, no-OpenSSH selected-candidate TFTP fetch
discriminator after the no-power root proof accepted selected-root-visible=true.

## Reviewed Inputs

- memory/talos-supervisor-state.json currentTask and taskQueue entries for this
  task, retry-v4, and closeout-v4.
- tasks/2026-06-23-phase12-ssh-selected-candidate-no-power-publish-root-discriminator.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-boot-staging-root-cause-contract.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one queued task and acquired hardwareTestLock before
lab publication, power-cycle, TFTP/serial capture, and restore. The task reused
the selected archive proven visible by the no-power root discriminator:

- archive: target/phase12-ssh-live-openssh-retry-boot.tar.gz.
- archive sha256:
  2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01.
- kernel_2712.img hash:
  110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5.
- kernel_2712.img size: 87,432 bytes at both the archive root and
  da591740/kernel_2712.img.
- selected candidate tree:
  fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333.

Post-publish status and boot/files again exposed the selected tree,
effective_kernel=kernel_2712.img, and 87,432-byte kernel entries before power.
The worker then power-cycled the Pi and waited for a stable TFTP delta from the
fresh cursor 4656995. The delta was stable at cursor 4656995 with zero parsed
events: no selected 87,432-byte kernel fetch, and no 104,136-byte baseline
kernel fetch. Final pre-restore status and boot/files still showed the selected
tree and 87,432-byte kernel entries, so the prior baseline-fetch contradiction
did not repeat in this run. The pre-run snapshot
phase12-ssh-fetch-v3-pre-20260623T223129Z restored the baseline/control tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
104,136-byte kernel entries.

Because no same-run TFTP evidence proved that the selected 87,432-byte
kernel_2712.img was served after publication and before restore,
selected_candidate_fetch_observed=false. Live OpenSSH retry-v4 remains blocked.

## Evidence

- sanitized summary:
  tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-fetch-after-root-cause-v3/selected-fetch-v3.summary.sanitized.json.
- static boot artifact review:
  archive-review.txt, archive-sha256.txt, archive-kernel-sha256.txt, and
  archive-kernel-sizes.txt.
- selected root proof reference:
  tasks/2026-06-23-phase12-ssh-selected-candidate-no-power-publish-root-discriminator.md
  and its no-power-publish-root.summary.sanitized.json evidence.
- pre-run identity and cursors:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-snapshots.sanitized.json, pre-run-snapshot-name.txt,
  pre-run-snapshot.sanitized.json, pre-serial-peek.sanitized.json,
  serial-cursor.txt, pre-tftp-tail.sanitized.json, and tftp-cursor.txt.
- publication and hardware evidence:
  publish.sanitized.json, post-publish-status.sanitized.json,
  post-publish-boot-files.sanitized.json, power-cycle.sanitized.json,
  tftp-delta.sanitized.json, tftp-delta.exit-code.txt,
  serial-observe.sanitized.json, and serial-observe.exit-code.txt.
- final identity and restore proof:
  final-pre-restore-status.sanitized.json,
  final-pre-restore-boot-files.sanitized.json, final-restore.sanitized.json,
  final-status.sanitized.json, and final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before lab/hardware action and released it
  only after restoring the pre-run baseline/control boot tree.
- fixed: retained candidate identity, selected root proof reference, effective
  kernel, selected kernel hash/size category, fresh serial cursor, fresh TFTP
  cursor/delta, final pre-restore identity, restore identity, and redaction
  review.
- fixed: proved the selected tree remained visible through status and
  boot/files before restore after the power-cycle window.
- deferred: selected-candidate fetch was not accepted because stable same-run
  TFTP evidence contained zero events from the saved cursor.
- deferred: live OpenSSH retry-v4 and closeout-v4 remain dependency-gated and
  must not be promoted from this result.
- not-an-issue: no Rust/runtime code or lab helper change was needed; this was
  a bounded hardware evidence task.
- removed: retry-v3 and closeout-v3 remain blocked/superseded and are not
  revived by this discriminator.

## Validation

- static task/docs/source review: pass.
- serialized Pi 5 lab discriminator evidence with hardwareTestLock owned by
  this task: fail-closed blocker, stable-zero-tftp-after-selected-publish.
- inconclusive-run triage evidence before code changes: candidate identity,
  fresh serial cursor, fresh TFTP cursor/delta, no-power root proof, and final
  pre-restore identity retained; known-good control was not run because the
  result is a fixed fail-closed stable-zero TFTP blocker, not a new ambiguous
  selected-vs-baseline byte mismatch.
- restore proof showing hardwareTestLock.restored=true and the prior accepted
  boot identity restored: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static task/docs/source review, static boot artifact
inspection, lab-controller API, serialized Pi 5 hardware power/restore/TFTP/
serial evidence, JSON syntax check, docs build, and diff checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree
hashes, public archive/kernel hashes and sizes, boot configuration keys, cursor
numbers, TFTP event status/filename/byte categories, serial byte counts,
validation commands, and classifications. It retains no raw OpenSSH output, raw
serial text, raw TFTP lines, client identities, user names, addresses, MAC
addresses, host keys, authorized keys, fingerprints, signatures, session
identifiers, channel identifiers, command bytes, payload bytes, packet captures,
boot artifact bytes, stable peer identifiers, or private user data.

## Acceptance

Accepted only as fail-closed blocker evidence:
stable-zero-tftp-after-selected-publish.

selected_candidate_fetch_observed=false.

selected_next_task=null.

planningNeeded=true.

planningReason=Selected candidate publication and final pre-restore identity
showed the 87,432-byte selected tree, but stable same-run TFTP evidence from the
fresh cursor contained zero events after the power-cycle, so the selected
candidate fetch precondition is still unproved.

No OpenSSH execution, TCP reachability, remote receipt, compatibility,
PTY/SCP/SFTP, broad command expansion, phase transition, or ssh-ready=true is
accepted.
