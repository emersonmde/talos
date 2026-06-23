# Phase 12.6 SSH lab boot-capture preflight

Task id: phase12-ssh-lab-boot-capture-preflight-20260623
Status: accepted
Classification: lab-boot-capture-fresh

## Goal

Prove the lab boot-capture precondition is fresh and decisive before retrying
any live OpenSSH client discriminator.

## Scope

The worker promoted exactly one queued Phase 12.6 task after the accepted
OpenSSH client provisioning preflight selected this task. The task acquired
hardwareTestLock before lab API mutation, serial cursor capture, TFTP
observation, power-cycle, and restore actions. It used the current restored
known-good/control boot identity only; no new candidate was published and no
OpenSSH or live TCP attempt was run.

## Findings

- fixed: hardwareTestLock was owned by this task for the serialized lab
  preflight window, then released after the pre-run snapshot was restored.
- fixed: sanitized lab API identity evidence recorded the restored control tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10,
  effective kernel kernel_2712.img, and the 104,136-byte kernel_2712.img /
  da591740/kernel_2712.img boot files.
- fixed: stable same-cursor TFTP delta before restore recorded 13 sanitized
  events with raw log and client identity removed, including two served
  da591740/kernel_2712.img fetches at 104,136 bytes.
- fixed: final pre-restore and post-restore identities matched the pre-run
  control tree and kernel selection.
- deferred: serial cursor freshness remained saturated. The bounded direct-read
  observation recorded saturated-cursor-capture-blocked with zero retained
  serial bytes and no raw serial text. This is recorded as a serial status, not
  as the proof of freshness.
- not-an-issue: OpenSSH execution, live TCP reachability, remote receipt,
  compatibility, PTY/SCP/SFTP, broad command expansion, phase transition, and
  ssh-ready=true were out of scope and remain unaccepted.

## Evidence

- summary:
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/lab-boot-capture-preflight.summary.sanitized.json.
- pre-run identity:
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/pre-status.sanitized.json.
- pre-run files/snapshots:
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/pre-boot-files.sanitized.json and
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/pre-snapshots.sanitized.json.
- fresh cursors:
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/pre-serial-cursor.sanitized.json and
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/pre-tftp-cursor.sanitized.json.
- hardware action and TFTP delta:
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/power-cycle.sanitized.json and
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/tftp-delta-before-restore.sanitized.json.
- serial status:
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/runtime-readiness.sanitized.json.
- restore proof:
  tasks/evidence/2026-06-23-phase12-ssh-lab-boot-capture-preflight/restore.sanitized.json,
  post-restore-status.sanitized.json, and post-restore-boot-files.sanitized.json.

## Redaction Review

Retained evidence is sanitized to public boot hashes/sizes, file names, cursor
offsets, TFTP event status/filename/bytes, helper classifications, and boolean
readiness/restore results. Raw serial text, raw TFTP log lines, client IP/MAC,
OpenSSH logs, key material, user/address identifiers, stable peer identifiers,
boot artifact bytes, and private user data were not retained.

## Validation

- static task/docs/source review: pass.
- serialized Pi 5 lab preflight evidence with hardwareTestLock owned by this
  task: pass.
- restore proof showing hardwareTestLock.restored=true and the prior accepted
  boot identity restored: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, scripts
  that generate Rust artifacts, Cargo metadata, or lab helper source touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted lab-boot-capture-fresh=true based on a stable same-cursor TFTP delta
that proves the restored control fetched da591740/kernel_2712.img at 104,136
bytes before restore, plus matching final pre-restore and post-restore
identities. selected_next_task is
phase12-ssh-live-openssh-client-discriminator-retry-20260623.
