# Phase 12 SSH Live TCP Known-Good Control Reselection Contract

Task id: phase12-ssh-live-tcp-known-good-control-reselection-contract-20260629

Status: accepted after commit.

Classification: selected-known-good-control-contract-ready.

Evidence level: static task/evidence review plus read-only lab-controller
GET /status, GET /boot/files, and GET /boot/snapshots. No hardware power
action, hardwareTestLock acquisition, boot publication, candidate run,
packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true, broad shell work,
or phase transition was performed.

## Goal

Select or reject a valid known-good/control baseline for live TCP Pi 5
preflight after the previously restored control failed to reach kernel entry or
the production-readiness marker.

## Selected Control Contract

selected_control_snapshot:
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

The snapshot is selected because retained v10 clean-rerun evidence records the
pre-run lab identity as the accepted baseline/control
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 tree before
the selected archive was published, and current read-only GET /boot/snapshots
confirms that the named snapshot is still available.

Expected selected-control identity:

- tree_hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
- configured_kernel: kernel_2712.img
- effective_kernel: kernel_2712.img
- kernel files: kernel_2712.img, kernel8.img, da591740/kernel_2712.img, and
  da591740/kernel8.img are expected to be 104,136 bytes.
- boot config files: config.txt and da591740/config.txt are expected to be
  118 bytes; cmdline.txt and da591740/cmdline.txt are expected to be 81 bytes.

Required serial marker contract:

- rpi5-production-timer-preemption: PASS is required for the accepted
  production-timer control readiness gate.
- TALOS: kernel_main is useful metadata if observed, but the accepted v3
  contract does not require it when the downstream PASS marker is present.

Required TFTP evidence:

- The next hardware proof must start from fresh TFTP cursor evidence after
  restoring/verifying the selected snapshot.
- Before any restore or later publication, stable same-cursor TFTP evidence
  must include the selected effective-kernel fetch, specifically
  da591740/kernel_2712.img served at 104,136 bytes.
- A stable zero-event window, a 82,045-byte kernel serve from the 6ead8933...
  tree, or post-restore replay byte labeling does not satisfy this contract.

Restore target:
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

The next hardware proof must acquire hardwareTestLock before restore, cursor
capture, power-cycle, or any other lab/hardware action; after the observation it
must restore this same selected snapshot and confirm post-restore GET /status
and GET /boot/files before releasing the lock.

## Findings

- fixed: selected the current named snapshot
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z as the control contract
  because retained v10 evidence identifies it with the a0452458... /
  104,136-byte baseline/control lineage.
- fixed: recorded the expected tree, effective kernel, byte metadata, serial
  marker contract, TFTP requirement, restore target, hardware lock lifecycle,
  and redaction rules for the next bounded proof.
- fixed: selected_next_task is
  phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof-20260629.
- deferred: the selected-control baseline still requires a serialized Pi 5
  hardware proof before any candidate preflight resumes.
- not-an-issue: current GET /status reports the restored 6ead8933... /
  82,045-byte tree; that remains real lab state but is not the selected
  production-timer known-good contract.
- not-an-issue: current GET /boot/files responds through .boot.files; the
  task evidence records that shape and does not rely on GET /.
- removed: no helper, classifier, source, task, or evidence artifact was
  removed.

## Evidence Map

- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-reselection-contract/evidence-map.json.
- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-reselection-contract/classification.json.
- Reviewed blocker reconciliation:
  tasks/2026-06-29-phase12-ssh-live-tcp-known-good-kernel-entry-blocker-reconciliation.md.
- Reviewed accepted v3 control proof:
  tasks/2026-06-11-phase12-pi5-known-good-bounded-runtime-readiness-v3-pi5-proof.md.
- Reviewed v10 clean-rerun lab identity:
  tasks/evidence/2026-06-24-phase12-ssh-live-openssh-preclient-fetch-gated-discriminator-v10/rerun-clean/pre-status.sanitized.json.

## Redaction Review

The task-owned JSON records task ids, snapshot names, tree hashes, byte counts,
file paths, command/gate labels, and classification labels. It does not add
packet payloads, peer identifiers, key material, session material, raw serial
output, boot artifact bytes, private user data, or stable secret-derived
identifiers.

## Validation

- git status --short --branch before edits/action: pass; main was ahead of
  origin with no uncommitted Talos changes before task promotion.
- Read-only lab-controller GET /status: pass; current lab identity was
  6ead8933... with effective_kernel=kernel_2712.img and 82,045-byte kernels,
  which confirms the old restored baseline remains current but not selected.
- Read-only lab-controller GET /boot/files: pass; endpoint returned
  .boot.files with the same current 6ead8933... / 82,045-byte identity.
- Read-only lab-controller GET /boot/snapshots: pass; snapshot
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z is present.
- jq empty on task-owned JSON evidence: pass.
- sh -n on touched shell helper/classifier scripts: not run; no shell scripts
  were touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof-20260629.

planningNeeded: false.

Candidate preflight v3/v4, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility, service success, ssh-ready=true, broad shell work, and
phase transition remain blocked until the selected-control baseline Pi 5 proof
accepts the contract.

Commit: recorded in talos-supervisor-state.json after final commit.
