# Phase 12.6 SSH selected-candidate fetch liveness bracket Pi 5 proof

Task id: phase12-ssh-selected-candidate-fetch-liveness-bracket-pi5-proof-20260624
Status: accepted
Owner: worker
Classification: baseline-control-fetch-missing

## Goal

Run a no-OpenSSH Pi 5 boot-request liveness bracket that proves whether TFTP
logging and selected-candidate fetch remain live around the selected SSH
candidate before any further live OpenSSH attempt.

## Reviewed Inputs

- memory/talos-supervisor-state.json task
  phase12-ssh-selected-candidate-fetch-liveness-bracket-pi5-proof-20260624.
- tasks/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v6.md.
- tasks/2026-06-23-phase12-ssh-selected-candidate-fetch-after-baseline-liveness-v4.md.
- tasks/2026-06-23-phase12-ssh-boot-request-liveness-baseline-control.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Execution Summary

The worker promoted exactly one ready task and acquired hardwareTestLock before
lab and hardware action. It recorded the reviewed selected archive identity for
target/phase12-ssh-live-openssh-retry-boot.tar.gz: archive hash
2f88dfdabaeb38e0f90fe597a874df04424e7d639646aa6e8729930766e2ca01, root and
da591740-prefixed kernel_2712.img size 87,432 bytes, and kernel hash
110e66ef6867a70cc8b72f52c0786a8f4037796b4058ee4a27ba1371ba8c12d5.

The baseline/control leg started from the restored
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 tree with
effective_kernel=kernel_2712.img and 104,136-byte kernel entries. The worker
created snapshot phase12-ssh-fetch-bracket-pre-20260624T023738Z, saved fresh
serial and TFTP cursors, and power-cycled the Pi 5 without publishing a
selected archive or launching OpenSSH.

The same-task baseline/control TFTP liveness gate failed closed. Stable TFTP
evidence from cursor 4663750 stayed at cursor 4663750 with zero parsed events:
no baseline/control 104,136-byte da591740/kernel_2712.img fetch was observed.
Because the task required same-task baseline/control TFTP liveness before using
selected no-fetch evidence, the selected archive was not published and no
selected power-cycle was run. Final pre-restore and final restored status both
reported the baseline/control a0452458... tree.

The accepted classification is baseline-control-fetch-missing.
selected_candidate_fetch_bracket_observed=false, selected_next_task=null, and
planningNeeded=true. No OpenSSH launch, TCP reachability, remote receipt,
compatibility, PTY/SCP/SFTP, broad command expansion, phase transition, or
ssh-ready=true is accepted.

## Evidence

- summary:
  tasks/evidence/2026-06-24-phase12-ssh-selected-candidate-fetch-liveness-bracket-pi5-proof/selected-candidate-fetch-liveness-bracket.summary.sanitized.json.
- archive identity:
  archive-sha256.txt, archive-review.txt, archive-kernel-sha256.txt, and
  archive-kernel-sizes.txt.
- baseline/control lab identity, power, and capture evidence:
  baseline-pre-status.sanitized.json, baseline-pre-boot-files.sanitized.json,
  pre-run-snapshot.sanitized.json, baseline-pre-serial-peek.sanitized.json,
  baseline-pre-tftp-tail.sanitized.json, baseline-power-cycle.sanitized.json,
  baseline-tftp-delta.sanitized.json, baseline-serial-observe.sanitized.json,
  baseline-final-status.sanitized.json, and baseline-final-boot-files.sanitized.json.
- selected leg:
  selected-skipped.sanitized.json records skipped=true because
  baseline-control-fetch-missing failed the same-task liveness precondition.
- final identity and restore proof:
  final-pre-restore-status.sanitized.json,
  final-pre-restore-boot-files.sanitized.json,
  final-restore.sanitized.json, final-status.sanitized.json, and
  final-boot-files.sanitized.json.

## Findings And Disposition

- fixed: acquired hardwareTestLock before lab/hardware action and released it
  only after recording restored=true for the pre-run baseline/control boot
  tree.
- fixed: retained baseline/control pre-run identity, fresh serial and TFTP
  cursors, baseline/control TFTP delta, selected archive identity, final
  pre-restore identity, restore identity, redaction review, and validation
  outputs.
- fixed: enforced the same-task baseline/control liveness prerequisite; selected
  no-fetch evidence was not collected or used after the baseline/control TFTP
  gate failed.
- deferred: selected-candidate fetch reproducibility and live OpenSSH retry
  remain unaccepted. Supervisor planning is required before another retry or
  closeout task.
- not-an-issue: no Rust source, helper source, Cargo metadata, OpenSSH launch,
  package installation, code repair, packet capture, raw OpenSSH transcript
  retention, remote-receipt claim, compatibility claim, phase transition, or
  ssh-ready claim was required or accepted.

## Validation

- serialized Pi 5 lab boot-request bracket under hardwareTestLock: pass,
  fail-closed baseline-control-fetch-missing.
- candidate identity via lab API before power-cycle: pass, baseline/control
  a0452458... tree retained before the baseline/control leg.
- TFTP delta via GET /tftp/logs for baseline/control leg: pass, stable zero
  events from cursor 4663750; selected leg skipped by contract.
- known-good baseline/control evidence in the same task before selected no-fetch
  is treated as meaningful: pass, gate failed closed, so selected no-fetch
  evidence was not used.
- restore proof showing hardwareTestLock.restored=true and prior boot identity
  restored: pass, final status tree is the baseline/control a0452458... tree.
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
and fixed marker booleans, validation commands, and classifications. It
retains no raw OpenSSH output, raw serial text, raw TFTP log lines, user names,
addresses, MAC addresses, host keys, authorized keys, fingerprints, signatures,
session identifiers, channel identifiers, payload bytes, command bytes, packet
captures, or private user data.

## Acceptance

Accepted only as fail-closed same-task baseline/control boot-request liveness
bracket evidence: baseline-control-fetch-missing.

baseline_control_fetch_observed=false.
selected_candidate_fetch_bracket_observed=false.
selected_next_task=null.
planningNeeded=true.

No OpenSSH launch, TCP reachability, remote receipt, compatibility,
PTY/SCP/SFTP, broad command expansion, phase transition, or ssh-ready=true is
accepted.
