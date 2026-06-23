# Phase 12.6 SSH selected-candidate no-power publish/root discriminator

Task id: phase12-ssh-selected-candidate-no-power-publish-root-discriminator-20260623
Status: accepted
Owner: worker
Classification: selected-root-visible

## Goal

Prove or refute the selected archive publication and lab API visible boot-root
invariant without power-cycling the Pi.

## Reviewed Inputs

- memory/talos-supervisor-state.json currentTask and taskQueue entries for this
  task, selected-fetch-v3, retry-v4, and closeout-v4.
- tasks/2026-06-23-phase12-ssh-selected-candidate-boot-staging-root-cause-contract.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2.md.
- tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2/archive-review.txt.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted this queued task after the accepted root-cause contract
selected it, acquired hardwareTestLock, and kept the run to no-power lab boot
publication and restore. No Pi power-cycle, serial capture, OpenSSH action, TCP
reachability test, runtime code change, compatibility claim, phase transition,
or broader networking work was performed.

The task reused the reviewed selected candidate archive:

- archive: target/phase12-ssh-live-openssh-retry-boot.tar.gz.
- archive sha256:
  2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01.
- kernel_2712.img hash:
  110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5.
- kernel_2712.img size: 87,432 bytes at both the archive root and
  da591740/kernel_2712.img.
- selected candidate tree:
  fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333.

The no-power discriminator accepted selected-root-visible=true:

- pre-publish status and boot/files showed the restored baseline/control tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  104,136-byte kernel_2712.img entries.
- PUT /boot/archive reported the selected tree
  fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333 and
  87,432-byte kernel_2712.img entries.
- immediate post-publish GET /status exposed the selected tree,
  effective_kernel=kernel_2712.img, and 87,432-byte root and
  da591740/kernel_2712.img entries.
- immediate post-publish GET /boot/files exposed the same selected tree,
  effective kernel, and 87,432-byte root and prefixed kernel entries.
- final restore returned status and boot/files to the pre-run baseline/control
  tree and 104,136-byte kernel entries.

This narrows the rerun-v2 contradiction to the power/TFTP/final-identity path:
the lab API visible publish/root layer is selected before power, so the next
bounded discriminator may test whether the selected 87,432-byte kernel is
actually served after power-cycle.

## Evidence

- sanitized summary:
  tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-no-power-publish-root-discriminator/no-power-publish-root.summary.sanitized.json.
- static boot artifact review:
  archive-review.txt, archive-sha256.txt, archive-kernel-sha256.txt, and
  archive-kernel-sizes.txt.
- pre-publish identity:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-snapshots.sanitized.json, pre-tftp-tail.sanitized.json,
  pre-run-snapshot-name.txt, and pre-run-snapshot.sanitized.json.
- no-power publication/root identity:
  publish.sanitized.json, post-publish-status.sanitized.json,
  post-publish-boot-files.sanitized.json, and
  post-publish-tftp-tail.sanitized.json.
- restore proof:
  final-restore.sanitized.json, final-status.sanitized.json,
  final-boot-files.sanitized.json, and final-tftp-tail.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before snapshot, boot publication, and
  restore, and released it only after final restore proof.
- fixed: retained archive identity, pre-publish identity, snapshot name,
  post-publish status identity, post-publish boot/files identity, TFTP tail
  cursor context, restore identity, and redaction review.
- fixed: proved selected-root-visible=true at the no-power lab API layer:
  post-publish status and boot/files both exposed the selected
  fe9a0d98... tree and 87,432-byte kernel_2712.img entries before restore.
- deferred: the power/TFTP path remains unproved; the selected-fetch-v3 task is
  the only mechanically selected next discriminator.
- deferred: live OpenSSH retry-v4 remains dependency-gated behind accepted
  selected-candidate-fetch-observed=true evidence from selected-fetch-v3.
- not-an-issue: no Talos runtime or helper source change was needed; the
  no-power publication/root API path behaved as expected.
- removed: retry-v3 and closeout-v3 remain blocked/superseded and are not
  revived by this no-power result.

## Validation

- static task/docs/source review: pass.
- serialized lab API publication/restore evidence with hardwareTestLock owned
  by this task: pass, selected-root-visible=true.
- no power-cycle and no OpenSSH action: pass.
- restore proof showing hardwareTestLock.restored=true and the prior boot
  identity restored: pass.
- jq empty on task-owned JSON evidence: pass.
- jq empty on memory/talos-supervisor-state.json: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static task/docs/source review, static boot artifact
inspection, lab-controller API no-power publication/restore evidence, JSON
syntax check, docs build, and diff checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree
hashes, public archive/kernel hashes and sizes, boot configuration keys, cursor
numbers, TFTP event status/filename/byte categories, validation commands, and
classifications. It retains no raw OpenSSH output, raw serial text, raw TFTP
lines, client identities, user names, addresses, MAC addresses, host keys,
authorized keys, fingerprints, signatures, session identifiers, channel
identifiers, command bytes, payload bytes, packet captures, boot artifact
bytes, stable peer identifiers, or private user data.

## Acceptance

Accepted as selected-root-visible.

selected_root_visible=true.

selected_next_task=phase12-ssh-selected-candidate-fetch-after-root-cause-v3-20260623.

planningNeeded=false.

No power-cycle, serial observation, live TFTP fetch, OpenSSH execution, TCP
reachability, remote receipt, compatibility, PTY/SCP/SFTP, broad command
expansion, phase transition, or ssh-ready=true is accepted.
