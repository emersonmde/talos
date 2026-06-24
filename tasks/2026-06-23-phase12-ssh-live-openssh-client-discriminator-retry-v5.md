# Phase 12.6 SSH live OpenSSH client discriminator retry v5

Task id: phase12-ssh-live-openssh-client-discriminator-retry-v5-20260623
Status: accepted
Owner: worker
Classification: lab-capture-regressed

## Goal

Run one hardware-serialized live OpenSSH client discriminator after selected
fetch v4 proved the selected 87,432-byte kernel can be served by TFTP.

## Reviewed Inputs

- memory/talos-supervisor-state.json task
  phase12-ssh-live-openssh-client-discriminator-retry-v5-20260623.
- tasks/2026-06-23-phase12-ssh-runner-openssh-client-provisioning-preflight.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4.md.
- tasks/2026-06-23-phase12-ssh-live-openssh-client-contract.md.
- docs/src/project/lab-controller.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one queued task and acquired hardwareTestLock
before selected boot publication, power-cycle, TFTP/serial capture, the
OpenSSH client attempt, and restore. A first pre-lab script pass failed during
static archive extraction because it used non-prefixed tar member names; no
snapshot, publication, power-cycle, OpenSSH action, or restore occurred in that
failed pass. The worker corrected the member paths and continued within the
same active task.

The accepted selected archive was
target/phase12-ssh-live-openssh-retry-boot.tar.gz with kernel_2712.img size
87,432 bytes at both the archive root and da591740/kernel_2712.img. The worker
created snapshot phase12-ssh-live-openssh-retry-v5-pre-20260624T001939Z,
published the selected archive, and verified post-publish status exposed tree
fe9a0d98aae7e38310a18adf7902d59346cbdef943250f16c948eae6a3f64333 with
effective_kernel=kernel_2712.img. It then power-cycled the Pi once, captured a
stable same-cursor TFTP delta before restore, captured a saturated-cursor
direct-read serial window, ran exactly one workspace-local OpenSSH client
attempt, and restored the pre-run baseline/control tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The same-run TFTP delta from cursor 4661048 remained stable at cursor 4661048
with zero events. The direct-read serial window retained 4,764 bytes and saw
firmware NETWORK markers but not TALOS: kernel_main. The OpenSSH client
launched against the public port-22 target class and failed as no-tcp-connect /
tcp-timeout; raw OpenSSH output was counted and removed. Because the selected
candidate fetch was not observed in this same live-client run, the OpenSSH
tcp-timeout is retained only as secondary public client evidence, and the
accepted fail-closed classification is lab-capture-regressed.

## Evidence

- sanitized summary:
  tasks/evidence/2026-06-23-phase12-ssh-live-openssh-client-discriminator-retry-v5/live-openssh-retry-v5.summary.sanitized.json.
- accepted OpenSSH provisioning preflight reference:
  tasks/2026-06-23-phase12-ssh-runner-openssh-client-provisioning-preflight.md.
- accepted selected fetch v4 proof reference:
  tasks/2026-06-23-phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4.md.
- static boot artifact review:
  archive-review.txt, archive-sha256.txt, archive-kernel-sha256.txt, and
  archive-kernel-sizes.txt.
- lab identity, publication, power, and capture evidence:
  pre-status.sanitized.json, pre-boot-files.sanitized.json,
  pre-run-snapshot.sanitized.json, publish.sanitized.json,
  post-publish-status.sanitized.json, post-publish-boot-files.sanitized.json,
  power-cycle.sanitized.json, tftp-delta.sanitized.json,
  serial-observe.sanitized.json, and openssh-attempt.sanitized.json.
- final identity and restore proof:
  final-pre-restore-status.sanitized.json,
  final-pre-restore-boot-files.sanitized.json, final-restore.sanitized.json,
  final-status.sanitized.json, and final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before lab/hardware/OpenSSH action and
  released it only after restoring the pre-run baseline/control boot tree.
- fixed: retained selected fetch v4 and OpenSSH provisioning prerequisites,
  selected archive identity, fresh boot/TFTP/serial evidence, sanitized OpenSSH
  result, final pre-restore identity, restore identity, and validation outputs.
- fixed: fail-closed classification is lab-capture-regressed because same-run
  TFTP stayed at zero events after selected publication and power-cycle.
- fixed: OpenSSH launched through the workspace-local client and produced only
  sanitized no-tcp-connect / tcp-timeout secondary evidence.
- deferred: no live OpenSSH discriminator observation is accepted; retry-v5
  selects no closeout and requires supervisor planning.
- deferred: live-reachability=true, remote-receipt=true, compatibility=true,
  PTY/SCP/SFTP, broad command expansion, phase transition, and ssh-ready=true
  remain rejected.
- not-an-issue: no Rust source or lab helper change was required for this
  fail-closed hardware/client evidence task.

## Validation

- static task/docs/source review: pass.
- serialized Pi 5 lab/OpenSSH discriminator evidence with hardwareTestLock
  owned by this task: pass, fail-closed lab-capture-regressed.
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
serial evidence, sanitized workspace-local OpenSSH client evidence, JSON syntax
check, docs build, and diff checks.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, public boot tree
hashes, public archive/kernel hashes and sizes, boot configuration keys, cursor
numbers, TFTP event counts/status/filename/byte categories, serial byte counts
and fixed marker booleans, OpenSSH invocation and phase categories, validation
commands, and classifications. It retains no raw OpenSSH output, raw serial
text, raw TFTP log lines, user names, addresses, MAC addresses, host keys,
authorized keys, fingerprints, signatures, session identifiers, channel
identifiers, command bytes, payload bytes, packet captures, stable peer
identifiers, boot artifact bytes, or private user data.

## Acceptance

Accepted only as fail-closed blocker evidence: lab-capture-regressed.

live_openssh_client_discriminator_observed=false.

selected_next_task=null.

planningNeeded=true.

planningReason=Selected candidate was published and OpenSSH launched, but
same-run TFTP evidence from the fresh cursor stayed at zero events before
restore; the OpenSSH tcp-timeout is retained only as secondary public client
evidence because the exercised candidate was not proven in this run.

No live reachability, remote receipt, compatibility, PTY/SCP/SFTP, broad
command expansion, phase transition, or ssh-ready=true is accepted.
