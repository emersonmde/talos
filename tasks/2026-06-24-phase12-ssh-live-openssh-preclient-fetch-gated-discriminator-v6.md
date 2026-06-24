# Phase 12.6 SSH live OpenSSH pre-client fetch-gated discriminator v6

Task id: phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v6-20260624
Status: accepted
Owner: worker
Classification: stable-zero-tftp-after-selected-publish

## Goal

Run the next live OpenSSH discriminator only behind a same-task pre-client
selected-fetch gate, so OpenSSH evidence is accepted only if the selected Talos
candidate was proven fetched before launching the client.

## Reviewed Inputs

- memory/talos-supervisor-state.json task
  phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v6-20260624.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry-v5.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4.md.
- tasks/2026-06-23-phase12-ssh-boot-request-liveness-baseline-control.md.
- tasks/2026-06-23-phase12-ssh-runner-openssh-client-provisioning-preflight.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one ready task and acquired hardwareTestLock before
lab, hardware, and conditional OpenSSH action. It reused the reviewed selected
archive target/phase12-ssh-live-openssh-retry-boot.tar.gz. Archive inspection
recorded kernel_2712.img size 87,432 bytes at both the archive root and the
da591740-prefixed path, with kernel hash
110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5.

The worker created snapshot
phase12-ssh-preclient-fetch-v6-pre-20260624T010900Z, saved fresh serial and
TFTP cursors, published the selected archive, verified post-publish status at
tree fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333 with
effective_kernel=kernel_2712.img, and power-cycled the Pi 5 once.

The same-task pre-client selected-fetch gate failed closed. Stable TFTP
evidence from cursor 4662399 stayed at cursor 4662399 with zero parsed events:
no selected 87,432-byte da591740/kernel_2712.img fetch and no baseline
104,136-byte fetch. Final pre-client and final pre-restore status still
reported the selected fe9a0d98... tree, and restore returned to the
baseline/control a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
tree.

Because selected kernel_2712.img was not proven served after publication and
before restore, the worker did not launch OpenSSH. The accepted classification
is stable-zero-tftp-after-selected-publish. selected_next_task=null and
planningNeeded=true.

## Evidence

- summary:
  tasks/evidence/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v6/preclient-fetch-gated-discriminator-v6.summary.sanitized.json.
- archive identity:
  archive-sha256.txt, archive-review.txt, archive-kernel-sha256.txt, and
  archive-kernel-sizes.txt.
- lab identity, publication, power, and capture evidence:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-run-snapshot.sanitized.json, pre-serial-peek.sanitized.json,
  pre-tftp-tail.sanitized.json, publish.sanitized.json,
  post-publish-status.sanitized.json, post-publish-boot-files.sanitized.json,
  power-cycle.sanitized.json, tftp-delta.sanitized.json,
  serial-observe.sanitized.json, final-pre-client-status.sanitized.json, and
  final-pre-client-boot-files.sanitized.json. The boot-files artifacts are
  derived from the retained status.boot.files field after the direct
  /boot/files sanitizer mismatched the endpoint response shape.
- conditional OpenSSH evidence:
  openssh-attempt.sanitized.json records launched=false because the
  pre-client selected-fetch gate failed.
- final identity and restore proof:
  final-pre-restore-status.sanitized.json,
  final-pre-restore-boot-files.sanitized.json,
  final-restore.sanitized.json, final-status.sanitized.json, and
  final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before lab/hardware/OpenSSH-eligible action
  and released it only after restoring the pre-run baseline/control boot tree.
- fixed: retained selected archive identity, post-publish candidate identity,
  fresh serial and TFTP cursors, same-task pre-client TFTP gate evidence,
  final pre-client identity, final pre-restore identity, restore identity, and
  validation outputs.
- fixed: enforced the OpenSSH launch gate; no OpenSSH client was launched
  because the selected kernel fetch was not proven in this task.
- deferred: live OpenSSH discriminator observation remains unaccepted.
  Supervisor planning is required before another retry or any closeout/remote
  receipt task.
- not-an-issue: no Rust source, helper source, Cargo metadata, package
  installation, code repair, packet capture, raw OpenSSH transcript retention,
  remote-receipt claim, compatibility claim, phase transition, or ssh-ready
  claim was required or accepted.

## Validation

- static task/docs/source review: pass.
- serialized Pi 5 lab/OpenSSH discriminator evidence with hardwareTestLock
  owned by this task: pass, fail-closed stable-zero-tftp-after-selected-publish.
- pre-client selected-fetch gate evidence before any OpenSSH launch: pass,
  gate failed closed and OpenSSH evidence is absent.
- restore proof showing hardwareTestLock.restored=true and the prior accepted
  boot identity restored: pass, final status tree is the baseline/control
  a0452458... tree.
- jq empty on task-owned JSON evidence: pass.
- redaction grep review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with the existing large
  search-index warning.
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
numbers, TFTP event counts/status/filename/byte categories, serial byte counts
and fixed marker booleans, OpenSSH invocation categories, validation commands,
and classifications. It retains no raw OpenSSH output, raw serial text, raw
TFTP log lines, user names, addresses, MAC addresses, host keys, authorized
keys, fingerprints, signatures, session identifiers, channel identifiers,
payload bytes, command bytes, packet captures, or private user data.

## Acceptance

Accepted only as fail-closed same-task pre-client selected-fetch gate evidence:
stable-zero-tftp-after-selected-publish.

preclient_selected_fetch_gate_passed=false.
live_openssh_client_discriminator_observed=false.
selected_next_task=null.
planningNeeded=true.

No OpenSSH launch, TCP reachability, remote receipt, compatibility,
PTY/SCP/SFTP, broad command expansion, phase transition, or ssh-ready=true is
accepted.
