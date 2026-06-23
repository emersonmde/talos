# Phase 12.6 SSH selected-candidate lab-capture rerun v2

Task id: phase12-ssh-selected-candidate-lab-capture-rerun-v2-20260623
Status: accepted
Owner: worker
Classification: baseline-fetch-after-selected-publish

## Goal

Rerun the selected-candidate lab-capture discriminator after the selected
candidate evidence contradiction repair, without running OpenSSH.

## Reviewed Inputs

- memory/talos-supervisor-state.json currentTask and taskQueue entries for this
  task, retry-v3, and closeout-v3.
- tasks/2026-06-23-phase12-ssh-selected-candidate-evidence-contradiction-repair.md.
- tasks/2026-06-23-phase12-ssh-lab-capture-selected-candidate-discriminator.md.
- tasks/2026-06-23-phase12-ssh-lab-boot-capture-preflight.md.
- tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry/archive-review.txt.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one queued task and acquired hardwareTestLock before
lab publication and Pi 5 power/TFTP/serial observation. The task reused the
previously reviewed selected candidate archive:

- archive: target/phase12-ssh-live-openssh-retry-boot.tar.gz.
- archive sha256:
  2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01.
- kernel_2712.img size: 87,432 bytes at both the archive root and
  da591740/kernel_2712.img.
- selected candidate tree:
  fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333.

The first run was treated as capture-chain-inconclusive because later evidence
collection overwrote the pre-restore TFTP/status files after restore. Following
the inconclusive-run triage policy, the worker ran a same-task restored-tree
control power cycle and then an unchanged selected-candidate rerun before any
code changes. The unchanged rerun produced the decisive blocker:

- candidate publication reported the selected tree
  fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333.
- the retained same-root TFTP delta before restore observed two served
  da591740/kernel_2712.img fetches at 104,136 bytes, not the selected
  87,432-byte candidate.
- final pre-restore status was already the restored baseline/control tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- final post-restore status remained the same baseline/control tree.

Therefore selected_candidate_fetch_observed=false and no live OpenSSH retry is
mechanically unblocked.

## Evidence

- sanitized summary:
  tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-lab-capture-rerun-v2/selected-candidate-rerun-v2.summary.sanitized.json.
- static boot artifact review:
  archive-review.txt, archive-sha256.txt, and archive-sizes.txt.
- initial selected run evidence:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-snapshots.sanitized.json, pre-run-snapshot.sanitized.json,
  publish.sanitized.json, post-publish-status.sanitized.json,
  post-publish-boot-files.sanitized.json, power-cycle.sanitized.json,
  tftp-delta.sanitized.json, final-pre-restore-status.sanitized.json, and
  final-status.sanitized.json.
- triage control evidence:
  control-pre-status.sanitized.json, control-pre-tftp-tail.sanitized.json,
  control-power-cycle.sanitized.json, control-tftp-delta.sanitized.json, and
  control-final-status.sanitized.json.
- unchanged candidate rerun evidence:
  candidate-rerun-publish.sanitized.json,
  candidate-rerun-post-publish-status.sanitized.json,
  candidate-rerun-post-publish-boot-files.sanitized.json,
  candidate-rerun-power-cycle.sanitized.json,
  candidate-rerun-tftp-delta.sanitized.json,
  candidate-rerun-final-pre-restore-status.sanitized.json,
  candidate-rerun-final-pre-restore-boot-files.sanitized.json,
  candidate-rerun-final-restore.sanitized.json,
  candidate-rerun-final-status.sanitized.json, and
  candidate-rerun-final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before publication and hardware/lab action,
  then released it after restoring the pre-run baseline/control tree.
- fixed: retained candidate identity, public archive/kernel hashes and sizes,
  serial cursor status, TFTP cursor/delta summaries, final pre-restore identity,
  and restore identity.
- fixed: applied the inconclusive-run triage policy after the first capture
  chain was overwritten by later restore-time evidence collection.
- deferred: live OpenSSH retry-v3 remains blocked because the selected
  candidate fetch precondition was not accepted.
- removed: retry-v2 and closeout-v2 remain superseded/quarantined and are not
  revived by this task.
- not-an-issue: no Rust/runtime code changed; the blocker is in the selected
  publish/TFTP/final-identity path, not in OpenSSH runtime behavior.

## Validation

- static task/docs/source review: pass.
- serialized Pi 5 lab discriminator evidence with hardwareTestLock owned by
  this task: fail-closed blocker, baseline-fetch-after-selected-publish.
- inconclusive-run triage evidence before code changes: candidate identity,
  fresh serial/TFTP cursors, restored-tree control attempt, and unchanged
  selected-candidate rerun retained.
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
numbers, TFTP event status/filename/byte categories, serial byte counts and
fixed marker booleans, validation commands, and classifications. It retains no
raw OpenSSH output, raw serial text, raw TFTP lines, client identities, user
names, addresses, MAC addresses, host keys, authorized keys, fingerprints,
signatures, session identifiers, channel identifiers, command bytes, payload
bytes, packet captures, boot artifact bytes, stable peer identifiers, or private
user data.

## Acceptance

Accepted only as fail-closed blocker evidence:
baseline-fetch-after-selected-publish.

selected_candidate_fetch_observed=false.

selected_next_task=null.

planningNeeded=true.

planningReason=Selected candidate publication reported the 87,432-byte
candidate tree, but same-root TFTP evidence before restore served
104,136-byte baseline kernel_2712.img and final pre-restore status was already
the baseline tree.

No OpenSSH execution, TCP reachability, remote receipt, compatibility,
PTY/SCP/SFTP, broad command expansion, phase transition, or ssh-ready=true is
accepted.
