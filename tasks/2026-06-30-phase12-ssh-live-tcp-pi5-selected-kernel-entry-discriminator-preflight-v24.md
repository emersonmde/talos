# Phase 12 SSH Live TCP Pi 5 Selected-Kernel-Entry Discriminator Preflight V24

Task id: phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24-20260630

Status: accepted after commit.

Classification: blocked-selected-kernel-entry-marker-missing.

Evidence level: serialized Pi 5 hardware preflight with lab-controller
selected-tree identity, stable same-cursor TFTP selected-byte evidence before
restore, fresh post-power serial window, final pre-restore identity, restore
proof, task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the accepted v23 selected-kernel-entry discriminator archive contract on the
Pi 5 and decide whether the _start-level entry marker is retained after
selected-byte service.

## Scope Performed

- Promoted the queued v24 preflight after v23 accepted
  selected-kernel-entry-discriminator-repair-ready at commit
  53e53d83c7f1fc1c31065df73ac1b6f7d03ef02c.
- Acquired hardwareTestLock before lab publication, boot mutation, and Pi 5
  power action.
- Re-materialized and reviewed the v23 selected-kernel-entry discriminator
  archive before publication.
- Published the selected archive, captured selected-tree identity, fresh
  serial/TFTP cursors, post-power serial output, stable TFTP delta, final
  pre-restore identity, restore proof, and post-restore identity.

## Hardware Result

Primary run:
selected-kernel-entry-discriminator-v24-20260630T231402Z.

The published selected archive retained the v23 contract:

- selected path: da591740/kernel_2712.img;
- kernel byte count: 87,432;
- kernel SHA-256:
  8051d7a600fe0867cfe093ffc6322ccdb532abaf58f323ece3f4013cca8054c7;
- expected _start marker:
  TALOS: selected-kernel-entry-discriminator-v23;
- later runtime/service markers remained absent.

The Pi 5 run retained decisive selected-byte and restore evidence:

- pre-power serial drain was empty on the first drain attempt;
- stable same-cursor TFTP delta captured 13 events, including two selected
  da591740/kernel_2712.img serves at 87,432 bytes;
- post-power direct-read serial captured firmware output but zero
  TALOS: selected-kernel-entry-discriminator-v23 occurrences;
- final pre-restore identity remained on selected tree
  dbe73980d5c5aa7876e49665ad528b8c9a0696f0d16c4a0d82bb99cd924294ce;
- restore returned the lab to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Because identity, TFTP, serial freshness, final identity, and restore proof
were decisive, the terminal classification is marker-missing rather than
inconclusive.

## Terminal Classification

blocked-selected-kernel-entry-marker-missing.

selected_next_task:
phase12-ssh-live-tcp-selected-kernel-entry-discriminator-closeout-v24-20260630.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, broad shell work,
fake/kernel-backed command expansion, and phase transition remain blocked.

## Findings

- fixed: executed the v23 _start selected-kernel-entry discriminator under
  hardwareTestLock with the exact selected archive contract.
- not-an-issue: selected-byte service, final pre-restore identity, serial
  freshness, and restore proof matched the v23 discriminator contract.
- deferred: the _start-level selected-kernel-entry marker was absent from the
  fresh post-power serial window; closeout must reconcile the first missing
  selected-kernel-entry fact before broader live TCP work.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, broad shell work, fake command expansion,
  and phase transition as immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24/classification.json.
- Primary hardware run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24/selected-kernel-entry-discriminator-v24-20260630T231402Z/.
- Static materialization:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-kernel-entry-discriminator-preflight-v24/static/.

## Redaction Review

Task-owned lab-controller serial and TFTP artifacts are retained under the run
evidence directory. This summary records task ids, labels, hashes, byte counts,
marker names, classifications, and validation outcomes; it omits packet
payloads, SSH/session/key material, private user data, and unnecessary stable
hardware identifiers.

## Validation

- git status before action: recorded.
- static archive review: pass.
- Lab API candidate identity before publication and before power action: pass.
- Fresh serial cursor/drain: pass; pre-power drain was empty.
- TFTP delta before restore: pass; two selected 87,432-byte fetches.
- Final pre-restore identity before restore: pass.
- Restore to predecessor-named baseline and confirm with lab API: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

Commit: recorded in talos-supervisor-state.json after final commit.
