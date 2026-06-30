# Phase 12 SSH Live TCP Pi 5 Selected-Image Handoff Discriminator v19

Task id: phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19-20260630

Status: accepted after commit.

Classification: selected-image-handoff-entry-reached.

Evidence level: serialized Pi 5 hardware discriminator with lab-controller
selected-tree identity, stable TFTP selected-byte evidence, direct-read serial
capture after saturated cursor handling, final pre-restore identity, restore
proof, task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the exact current-tree production-timer selected-path control discriminator
selected by the v19 handoff reconciliation, and decide whether selected bytes
can reach the first accepted downstream handoff marker.

## Scope Performed

- Promoted the ready v19 hardware task after the accepted v19 reconciliation
  selected this exact successor.
- Acquired hardwareTestLock before lab publication, Pi 5 power action, and
  restore-affecting operation.
- Rematerialized the current-tree production-timer selected-path control:
  selected da591740/kernel_2712.img at 104,136 bytes with SHA-256
  2343a009a14972d050ccf0fc706539163b6b5cb3ee3717b9cb6753f2ec7c2328.
- Published only that archive, power-cycled the Pi 5, captured TFTP, serial,
  final pre-restore identity, and restored
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- Treated the first hardware pass as an inconclusive capture/freshness result
  because the pre-power drain did not empty. Reran the same selected archive
  with a changed capture condition: 64-attempt pre-power serial drain. The rerun
  drained empty before power and produced decisive evidence.

## Result

The decisive rerun
selected-image-handoff-v19-rerun-drain64-20260630T184819Z observed:

- selected tree hash
  4edd4f1dad12ea06e3c45b1435f9a2d16e9c2046226d8963a0d8413a9f7226d1;
- effective kernel kernel_2712.img;
- two stable selected da591740/kernel_2712.img TFTP serves at 104,136 bytes;
- empty pre-power serial drain followed by direct-read post-power serial bytes;
- firmware NETWORK output and one
  rpi5-production-timer-preemption: PASS marker;
- final pre-restore identity still staged on the selected tree; and
- post-restore tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

TALOS: kernel_main was not retained in the direct-read window, but the v19
predecessor contract explicitly made rpi5-production-timer-preemption: PASS
the earliest decisive marker and treated earlier phase/kernel_main lines as
metadata-only when PASS is present.

## Terminal Classification

selected-image-handoff-entry-reached.

selected_next_task:
phase12-ssh-live-tcp-selected-image-handoff-discriminator-closeout-v19-20260630.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, minimal-entry route repair, broad shell work,
and phase transition remain blocked until a later accepted closeout explicitly
unblocks them. This task does not directly start route repair.

## Findings

- fixed: executed the successor-selected Pi 5 current-tree production-timer
  selected-path control discriminator under hardwareTestLock.
- fixed: reran with a changed capture condition after the first pass had a
  serial-freshness rejection; the rerun drained empty before power and allowed
  decisive classification.
- not-an-issue: selected byte service, final pre-restore identity, and restore
  proof all matched the predecessor contract.
- not-an-issue: the PASS marker is decisive for this discriminator even though
  TALOS: kernel_main was absent from the retained direct-read window.
- deferred: closeout must reconcile what this handoff-entry proof unblocks
  before minimal-entry repair, packet I/O, OpenSSH/generated-root retry, service
  readiness, broad shell work, or phase transition can resume.
- removed: same-shape v18 retry, packet-I/O/OpenSSH retry, remote receipt, and
  service-readiness claims as immediate successors.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19/classification.json.
- Decisive run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19/selected-image-handoff-v19-rerun-drain64-20260630T184819Z/.
- First inconclusive capture/freshness run:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19/selected-image-handoff-v19-20260630T184630Z/.

## Validation

- git status before action: recorded.
- archive materialization and review: pass.
- Lab API candidate identity after publication and before power: pass.
- Fresh serial cursor/capture: pass on rerun with empty pre-power drain and
  direct-read post-power window from saturated cursor.
- TFTP cursor/delta before restore: pass; stable selected fetch evidence.
- Final pre-restore identity before restore: pass.
- Restore to predecessor-named baseline and confirm with lab API: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Task-owned lab-controller serial and TFTP artifacts are retained under the run
evidence directories. This summary records task ids, labels, hashes, byte
counts, marker names, classifications, and validation outcomes; it omits packet
payloads, SSH/session/key material, and private user data.

Commit: accepted hardware evidence committed; final SHA recorded in durable
supervisor state.
