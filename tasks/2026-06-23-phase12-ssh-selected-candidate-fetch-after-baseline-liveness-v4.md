# Phase 12.6 SSH selected-candidate fetch after baseline liveness v4

Task id: phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4-20260623
Status: accepted
Owner: worker
Classification: selected-candidate-fetch-observed

## Goal

Run one selected-candidate TFTP fetch discriminator after the restored
baseline/control boot tree proved fresh same-run TFTP liveness.

## Reviewed Inputs

- memory/talos-supervisor-state.json currentTask and taskQueue entries for this
  task, live OpenSSH retry-v5, and live OpenSSH closeout-v5.
- tasks/2026-06-23-phase12-ssh-boot-request-liveness-baseline-control.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-fetch-after-root-cause-v3.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one queued task and acquired hardwareTestLock before
selected boot publication, power-cycle, TFTP/serial capture, and restore. The
task reused the selected archive from the earlier selected-fetch lineage:

- archive: target/phase12-ssh-live-openssh-retry-boot.tar.gz.
- archive sha256:
  2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01.
- kernel_2712.img hash:
  110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5.
- kernel_2712.img size: 87,432 bytes at both the archive root and
  da591740/kernel_2712.img.
- selected candidate tree:
  fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333.

Pre-run identity was the restored baseline/control tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
104,136-byte kernel entries. The worker created snapshot
phase12-ssh-fetch-v4-pre-20260623T235923Z, saved fresh serial and TFTP cursors,
published the selected archive, verified post-publish status exposed the
87,432-byte selected tree, power-cycled the Pi once, and retained stable
same-cursor TFTP evidence before restore. The TFTP delta advanced from cursor
4659697 to 4661048 and retained 13 parsed events, including two served
da591740/kernel_2712.img fetches at 87,432 bytes. Serial observe used the
saturated-cursor direct-read fallback, retained 5,369 bytes with firmware
NETWORK markers, and did not observe TALOS: kernel_main.

Final pre-restore identity still exposed the selected 87,432-byte tree, and the
restore returned to the baseline/control tree with 104,136-byte kernel entries.
Because same-run TFTP evidence proves the selected kernel_2712.img was served
after publication and before restore, selected_candidate_fetch_observed=true.
The selected next bounded task is
phase12-ssh-live-openssh-client-discriminator-retry-v5-20260623.

## Evidence

- sanitized summary:
  tasks/evidence/2026-06-23-phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4/selected-fetch-v4.summary.sanitized.json.
- baseline/control liveness proof reference:
  tasks/2026-06-23-phase12-ssh-boot-request-liveness-baseline-control.md and
  commit 223b14a92abaf4f1fb02ac8bbb6ec329bf2f3247.
- static boot artifact review:
  archive-review.txt, archive-sha256.txt, archive-kernel-sha256.txt, and
  archive-kernel-sizes.txt.
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

- fixed: acquired hardwareTestLock before selected publication and lab/hardware
  action, then released it only after restoring the pre-run baseline/control
  boot tree.
- fixed: retained selected archive identity, accepted baseline-control liveness
  proof, effective kernel, selected kernel hash/size category, fresh serial
  cursor, fresh TFTP cursor/delta, final pre-restore identity, restore
  identity, and redaction review.
- fixed: proved selected-candidate fetch liveness through two fresh same-run
  da591740/kernel_2712.img TFTP serves at 87,432 bytes.
- deferred: serial runtime readiness remains unaccepted because the saturated
  direct-read window showed firmware NETWORK markers but not TALOS:
  kernel_main.
- deferred: no OpenSSH action was performed; live OpenSSH retry-v5 is only
  dependency-unblocked for a future bounded task.
- not-an-issue: no Rust/runtime code or lab helper change was needed for this
  bounded discriminator.
- removed: retry-v4 and closeout-v4 remain blocked/superseded and are not
  revived by this selected-fetch proof.

## Validation

- static task/docs/source review: pass.
- serialized Pi 5 lab selected-candidate evidence with hardwareTestLock owned
  by this task: pass, selected-candidate-fetch-observed.
- inconclusive-run triage evidence before code changes: candidate identity,
  fresh serial cursor, fresh TFTP cursor/delta, accepted baseline-control
  liveness proof, final pre-restore identity, and restore identity retained.
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
serial text, raw serial base64, raw TFTP lines, client identities, user names,
addresses, MAC addresses, host keys, authorized keys, fingerprints, signatures,
session identifiers, channel identifiers, command bytes, payload bytes, packet
captures, boot artifact bytes, stable peer identifiers, or private user data.

## Acceptance

Accepted as selected-candidate-fetch-observed.

selected_candidate_fetch_observed=true.

selected_next_task=phase12-ssh-live-openssh-client-discriminator-retry-v5-20260623.

planningNeeded=false.

No OpenSSH execution, TCP reachability, remote receipt, compatibility,
PTY/SCP/SFTP, broad command expansion, phase transition, or ssh-ready=true is
accepted.
