# Phase 12 SSH Live TCP Known-Good Capture Discriminator

Task id: phase12-ssh-live-tcp-known-good-capture-discriminator-20260629

Status: accepted after commit.

Classification: blocked-known-good-marker-absent.

Evidence level: lab-controller API identity, serial hardware boot/output,
stable TFTP delta, known-good readiness classifier replay, restore proof,
task-owned JSON evidence, docs build, and diff checks. No candidate archive,
packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, runtime russh adoption, fake
command expansion, broad shell work, or phase transition was performed.

## Goal

Run the known-good-only lab capture discriminator selected by the accepted
lab-capture contract reconciliation, proving whether the lab can join a stable
known-good boot identity, TFTP fetch, and serial readiness marker before any
candidate retry.

## Scope Performed

- Promoted the mechanically selected known-good discriminator and acquired
  hardwareTestLock before lab action.
- Retained pre-power GET /status, GET /boot/files, GET /boot/snapshots, fresh
  serial cursor, and fresh TFTP tail cursor.
- Power-cycled the restored known-good boot tree only; no candidate archive was
  published.
- Retained the primary runtime-readiness artifact with the accepted wrapper and
  retained a stable same-cursor TFTP delta before restore.
- Replayed scripts/rpi5-known-good-readiness-v3-classify.sh over the retained
  readiness, status, and TFTP artifacts.
- Restored the lab to abcontrol-secondary-workload-pre-20260524T231449Z and
  retained post-restore GET /status, GET /boot/files, and GET /boot/snapshots.

## Terminal Classification

blocked-known-good-marker-absent.

The known-good-only run repaired the prior identity contradiction: pre-power,
final pre-restore, and post-restore GET /status all reported tree
6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef with
effective_kernel=kernel_2712.img, and GET /boot/files reported
da591740/kernel_2712.img as 82045 bytes before and after the run. The stable
same-cursor TFTP delta before restore included two served
da591740/kernel_2712.img fetches, both 82045 bytes.

The serial readiness artifact still did not contain TALOS: kernel_main or the
required rpi5-production-timer-preemption: PASS marker. The readiness helper
used the saturated-cursor direct-read fallback, retained 4473 response bytes,
and classified the primary artifact as
known-good-fetch-observed-without-talos-readiness. The v3 classifier therefore
rejected only missing-production-success-marker.

Because the lab now proves stable known-good identity plus stable TFTP capture
but does not prove the production success marker, this task does not select
candidate preflight v3. planningNeeded is true before any candidate retry,
packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, runtime russh adoption, fake
command expansion, broad shell work, or phase transition.

## Findings

- fixed: hardwareTestLock was acquired for the known-good-only lab evidence
  window and released only after restore.
- fixed: selected known-good identity stayed stable across pre-power,
  final-pre-restore, and post-restore GET /status.
- fixed: TFTP capture is observable for the known-good boot; the stable delta
  retained 13 events and two 82045-byte da591740/kernel_2712.img serves.
- blocked: the retained primary serial readiness window omitted both TALOS:
  kernel_main and rpi5-production-timer-preemption: PASS.
- not-an-issue: the previous missing-or-unstable-boot-identity-join rejection
  did not reproduce in this known-good-only run.
- not-an-issue: no Rust source gate is required because no Rust source changed.
- removed: no source, docs, task, or evidence artifact was removed.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-capture-discriminator/known-good-capture-discriminator-20260629T124233Z/.
- Summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-capture-discriminator/known-good-capture-discriminator-20260629T124233Z/known-good-capture-summary.json.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-capture-discriminator/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-capture-discriminator/evidence-map.json.
- Primary serial readiness:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-capture-discriminator/known-good-capture-discriminator-20260629T124233Z/known-good-capture-discriminator-20260629T124233Z-runtime-readiness-primary.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-capture-discriminator/known-good-capture-discriminator-20260629T124233Z/tftp-delta-stable-pre-restore.json.
- Known-good readiness classifier:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-capture-discriminator/known-good-capture-discriminator-20260629T124233Z/known-good-readiness-v3-classification.json.

## Redaction Review

Task-owned JSON evidence redacts local peer IP/MAC strings and TFTP client
fields. It does not retain packet payloads, key material, session material,
boot artifact bytes, private user data, stable secret-derived identifiers, or
unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass; tree/effective kernel restored to the same known-good
  identity.
- Fresh serial cursor captured before power: pass; retained in task-owned
  evidence.
- GET /tftp/logs tail cursor and stable same-cursor delta captured before
  restore: pass; stable helper exited 0 with two served kernel_2712.img fetches.
- Known-good readiness classifier: blocked; helper/classifier exited 1 with
  missing-production-success-marker.
- Restore to named snapshot/control state: pass; post-restore GET /status
  reports tree 6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef
  and effective_kernel=kernel_2712.img.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: known-good control now proves stable identity and TFTP capture,
but still lacks the production serial success marker; supervisor planning is
required before any candidate preflight v3, packet-I/O discriminator,
OpenSSH/generated-root retry, remote receipt, compatibility, service success,
ssh-ready=true, runtime russh adoption, fake command expansion, broad shell
work, or phase transition.

Commit: recorded in talos-supervisor-state.json after final commit.
