# Phase 12.6 SSH live OpenSSH pre-client fetch-gated discriminator v8

Task id: phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v8-20260624
Status: accepted
Owner: worker
Classification: preclient-selected-fetch-gate-failed

## Goal

Run one live OpenSSH discriminator only behind a same-task pre-client
selected-fetch gate, after selected-candidate fetch v5 accepted that the
reviewed selected archive can be served by TFTP.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task and taskQueue entries.
- tasks/2026-06-24-phase12-ssh-selected-candidate-fetch-after-recovered-baseline-v5.md.
- tasks/2026-06-23-phase12-ssh-runner-openssh-client-provisioning-preflight.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-contract.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one queued task and acquired hardwareTestLock
before lab, hardware, or OpenSSH-eligible action. A first static archive review
attempt used non-prefixed tar member names, failed before snapshot,
publication, power-cycle, OpenSSH launch, or restore, and was corrected to the
archive's ./ member paths before continuing in the same active task.

The selected archive was target/phase12-ssh-live-openssh-retry-boot.tar.gz with
archive sha256
2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01 and
kernel_2712.img sha256
110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5. The archive
root and da591740-prefixed kernel_2712.img entries were both 87,432 bytes.

Pre-run identity was the restored baseline/control a0452458... tree with
effective_kernel=kernel_2712.img. The worker created snapshot
phase12-ssh-preclient-fetch-v8-pre-20260624T052111Z, published the selected
archive, verified post-publish status exposed selected tree
fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333, saved fresh
serial and TFTP cursors, and power-cycled the Pi 5 once.

The same-task pre-client selected-fetch gate failed closed. Stable TFTP
evidence from cursor 4667803 stayed at cursor 4667803 with zero parsed events:
no selected 87,432-byte da591740/kernel_2712.img fetch and no baseline
104,136-byte fetch. Serial observation used the saturated-cursor direct-read
fallback, retained only byte counts and marker booleans, and did not observe
the Talos runtime marker. Final pre-client and pre-restore status still
reported the selected tree, and restore returned to the baseline/control
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 tree.

Because selected kernel_2712.img was not proven served after publication and
before restore, the worker did not launch OpenSSH. The accepted classification
is preclient-selected-fetch-gate-failed. selected_next_task=null and
planningNeeded=true.

## Evidence

- summary:
  tasks/evidence/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v8/live-openssh-preclient-fetch-gated-discriminator-v8.summary.sanitized.json.
- selected fetch v5 proof reference:
  tasks/2026-06-24-phase12-ssh-selected-candidate-fetch-after-recovered-baseline-v5.md
  and commit 63507f3de6b202174dd1905a9b22bd9b1ddd2551.
- OpenSSH provisioning preflight reference:
  tasks/2026-06-23-phase12-ssh-runner-openssh-client-provisioning-preflight.md.
- static boot artifact review:
  archive-review.txt, archive-sha256.txt, archive-kernel-sha256.txt, and
  archive-kernel-sizes.txt.
- lab identity, publication, power, and pre-client capture evidence:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-snapshots.sanitized.json, pre-run-snapshot-name.txt,
  pre-run-snapshot.sanitized.json, publish.sanitized.json,
  post-publish-status.sanitized.json, post-publish-boot-files.sanitized.json,
  pre-serial-peek.sanitized.json, serial-cursor.txt,
  pre-tftp-tail.sanitized.json, tftp-cursor.txt, power-cycle.sanitized.json,
  tftp-delta.sanitized.json, tftp-delta.exit-code.txt,
  serial-observe.sanitized.json, and serial-observe.exit-code.txt.
- conditional OpenSSH evidence:
  openssh-attempt.sanitized.json records launched=false because the pre-client
  selected-fetch gate failed.
- final identity and restore proof:
  final-pre-client-status.sanitized.json,
  final-pre-client-boot-files.sanitized.json,
  final-pre-restore-status.sanitized.json,
  final-pre-restore-boot-files.sanitized.json, final-restore.sanitized.json,
  final-status.sanitized.json, and final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before lab/hardware/OpenSSH-eligible action
  and released it only after restoring the pre-run baseline/control boot tree.
- fixed: retained selected archive identity, post-publish selected identity,
  fresh serial and TFTP cursors, stable same-cursor pre-client TFTP gate
  evidence, final pre-client identity, final pre-restore identity, restore
  identity, and validation outputs.
- fixed: enforced the OpenSSH launch gate; no OpenSSH client was launched
  because the selected kernel fetch was not proven in this task.
- fixed: corrected the static archive member path review before any
  post-archive lab action in the accepted pass.
- deferred: live OpenSSH discriminator observation remains unaccepted.
  Supervisor planning is required before another retry or any closeout/remote
  receipt task.
- not-an-issue: no Rust source, lab helper source, Cargo metadata, package
  installation, raw OpenSSH transcript retention, remote-receipt claim,
  compatibility claim, phase transition, or ssh-ready claim was required or
  accepted.

## Validation

- serialized Pi 5 live discriminator under hardwareTestLock: pass, fail-closed
  preclient-selected-fetch-gate-failed.
- candidate identity via lab API GET / before power-cycle: pass via
  post-publish GET /status and GET /boot/files selected identity.
- fresh serial cursor/nonce evidence where available: pass as sanitized cursor
  and marker-summary evidence; serial runtime readiness remains unaccepted.
- pre-client selected fetch gate via GET /tftp/logs from a fresh cursor: pass,
  gate failed closed with stable zero parsed events.
- bounded OpenSSH client discriminator only after pre-client gate passes: pass,
  OpenSSH was not launched because the gate failed.
- restore proof showing hardwareTestLock.restored=true and the prior accepted
  boot identity restored: pass.
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
checks. No OpenSSH launch, live reachability, remote receipt, compatibility,
phase transition, or ssh-ready=true was accepted.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree
hashes, public archive/kernel hashes and sizes, boot configuration keys, cursor
numbers, TFTP event counts and public filename/byte categories, serial byte
counts and fixed marker booleans, OpenSSH invocation categories, validation
commands, and classifications. It retains no raw OpenSSH output, raw serial
text, raw serial base64, raw TFTP log lines, client identities, user names,
addresses, MAC addresses, host keys, authorized keys, fingerprints, signatures,
session identifiers, channel identifiers, payload bytes, command bytes, packet
captures, boot artifact bytes, stable peer identifiers, or private user data.

## Acceptance

Accepted only as fail-closed same-task pre-client selected-fetch gate evidence:
preclient-selected-fetch-gate-failed.

preclient_selected_fetch_gate_passed=false.
live_openssh_client_discriminator_observed=false.
selected_next_task=null.
planningNeeded=true.

No OpenSSH launch, TCP reachability, remote receipt, compatibility,
PTY/SCP/SFTP, broad command expansion, phase transition, or ssh-ready=true is
accepted.
