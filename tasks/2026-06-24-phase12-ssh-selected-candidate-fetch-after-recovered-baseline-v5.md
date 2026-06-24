# Phase 12.6 SSH selected-candidate fetch after recovered baseline v5

Task id: phase12-ssh-selected-candidate-fetch-after-recovered-baseline-v5-20260624
Status: accepted
Owner: worker
Classification: selected-candidate-fetch-observed

## Goal

Retry the no-OpenSSH selected-candidate fetch proof after the immediately
preceding same-task baseline/control proof recovered TFTP liveness.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task and taskQueue entries.
- tasks/2026-06-24-phase12-ssh-baseline-control-tftp-liveness-recovery-pi5-proof.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one queued task and acquired hardwareTestLock before
selected archive publication, Pi 5 power/TFTP/serial capture, and restore. The
task did not launch OpenSSH, attempt live reachability, change Talos runtime
code, or claim SSH readiness.

The selected archive was target/phase12-ssh-live-openssh-retry-boot.tar.gz with
archive sha256
2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01 and
kernel_2712.img sha256
110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5. The archive
root and da591740-prefixed kernel_2712.img entries were both 87,432 bytes.

Pre-publication identity was the restored baseline/control a0452458... tree
with effective_kernel=kernel_2712.img. The worker created snapshot
phase12-ssh-fetch-v5-pre-20260624T044538Z, published the selected archive,
verified post-publish status exposed selected tree
fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333, saved fresh
serial and TFTP cursors, and power-cycled the Pi once. Stable same-cursor TFTP
evidence advanced from cursor 4666452 to 4667803 with 13 parsed events,
including two da591740/kernel_2712.img serves at 87,432 bytes. No baseline
104,136-byte kernel fetch appeared after selected publication.

Serial observation used the saturated-cursor direct-read fallback from cursor
4194304 and retained only sanitized byte counts and marker booleans. The retained
window captured 0 bytes, did not observe TALOS: kernel_main, and did not accept
serial runtime readiness. Final pre-restore identity still exposed the selected tree;
restore returned the boot tree to baseline/control
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

selected_candidate_fetch_observed=true. The selected next bounded task is
phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v8-20260624.

## Evidence

- summary:
  tasks/evidence/2026-06-24-phase12-ssh-selected-candidate-fetch-after-recovered-baseline-v5/selected-fetch-v5.summary.sanitized.json.
- baseline/control liveness proof reference:
  tasks/2026-06-24-phase12-ssh-baseline-control-tftp-liveness-recovery-pi5-proof.md
  and commit 34e95af7a1014bb19ebf1f4424bcf0a2b039e2ef.
- static boot artifact review:
  archive-review.txt, archive-sha256.txt, archive-kernel-sha256.txt, and
  archive-kernel-sizes.txt.
- pre-publication identity and restore point:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-snapshots.sanitized.json, pre-root-endpoint.sanitized.json,
  pre-run-snapshot-name.txt, and pre-run-snapshot.sanitized.json.
- publication and hardware evidence:
  publish.sanitized.json, post-publish-status.sanitized.json,
  post-publish-boot-files.sanitized.json, post-publish-root-endpoint.sanitized.json,
  pre-serial-peek.sanitized.json, serial-cursor.txt, pre-tftp-tail.sanitized.json,
  tftp-cursor.txt, power-cycle.sanitized.json, tftp-delta.sanitized.json,
  tftp-delta.exit-code.txt, serial-window.sanitized.json, and
  serial-window.exit-code.txt.
- final identity and restore proof:
  final-pre-restore-status.sanitized.json,
  final-pre-restore-boot-files.sanitized.json, final-restore.sanitized.json,
  final-status.sanitized.json, and final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before selected publication and lab/hardware
  action, then restored the pre-run baseline/control boot tree before
  acceptance.
- fixed: retained selected archive identity, post-publish identity, fresh
  cursors, stable same-cursor selected TFTP delta, final pre-restore identity,
  restore identity, validation outputs, and redaction review.
- fixed: proved selected-candidate fetch liveness with two fresh same-run
  da591740/kernel_2712.img serves at 87,432 bytes.
- deferred: serial runtime readiness remains unaccepted because retained serial
  evidence did not observe TALOS: kernel_main.
- deferred: OpenSSH remains gated behind the v8 pre-client selected-fetch
  discriminator and was not launched by this task.
- not-an-issue: no Talos runtime source, lab helper, packet capture, remote
  receipt, compatibility, PTY/SCP/SFTP, phase transition, or ssh-ready claim was
  required or accepted.

## Validation

- serialized Pi 5 selected-candidate no-OpenSSH fetch proof under
  hardwareTestLock: pass, selected-candidate-fetch-observed.
- candidate identity via lab API GET / before power-cycle: pass as endpoint
  semantics evidence; GET / returned the deployed 404 response, while
  post-publish GET /status and GET /boot/files retained authoritative selected
  identity.
- fresh serial cursor/nonce evidence where available: pass as sanitized cursor
  and marker-summary evidence; serial runtime readiness remains unaccepted.
- TFTP delta via GET /tftp/logs from a fresh cursor: pass. Stable same-cursor
  delta advanced from 4666452 to 4667803 with two selected 87,432-byte kernel
  serves.
- restore proof showing hardwareTestLock.restored=true and prior boot identity
  restored: pass.
- jq empty on task-owned JSON evidence: pass.
- redaction review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.

Evidence levels: static task/docs/source review, static boot artifact
inspection, lab-controller API, serialized Pi 5 hardware power/restore/TFTP/
serial evidence, JSON syntax check, redaction grep review, docs build, and diff
checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree
hashes, public archive/kernel hashes and sizes, boot configuration keys, cursor
numbers, TFTP event status/filename/byte categories, serial byte counts and
marker booleans, validation commands, and classifications. It retains no raw
serial text, raw serial base64, raw TFTP lines, client identities, user names,
addresses, MAC addresses, OpenSSH logs, known_hosts, host keys, authorized keys,
key material, fingerprints, signatures, session identifiers, channel
identifiers, command bytes, payload bytes, packet captures, boot artifact
bytes, stable peer identifiers, or private user data.

## Acceptance

Accepted as selected-candidate-fetch-observed.

selected_candidate_fetch_observed=true.

selected_next_task=phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v8-20260624.

planningNeeded=false.

No OpenSSH execution, TCP reachability, remote receipt, compatibility,
PTY/SCP/SFTP, broad command expansion, phase transition, or ssh-ready=true is
accepted from this task.
