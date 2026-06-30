# Phase 12 SSH Live TCP Pi 5 Minimal-Entry Console Boundary Preflight V22

Task id: phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22-20260630

Status: accepted after commit.

Classification: blocked-minimal-entry-console-boundary-marker-missing.

Evidence level: serialized Pi 5 hardware preflight with lab-controller
selected-tree identity, stable TFTP selected-byte evidence, drained fresh
serial window, final pre-restore identity, restore proof, task-owned JSON
evidence, docs build, and diff checks.

## Goal

Run the accepted v21 minimal-entry console-boundary archive contract on the Pi
5 and decide whether the repaired boundary marker path is retained after
selected-byte service.

## Scope Performed

- Promoted the queued v22 preflight after v21 accepted
  minimal-entry-console-boundary-repair-ready at commit
  3e5799bad06714b92c1792af8a4942ca860b3528.
- Acquired hardwareTestLock before lab publication, boot mutation, and Pi 5
  power action.
- Published the v21 selected archive with capture nonce
  phase12-console-boundary-v21-static.
- Captured selected-tree identity, fresh serial/TFTP cursors, post-power
  serial output, stable TFTP delta, final pre-restore identity, restore proof,
  and post-restore identity.
- Re-ran the same candidate once after the first helper session completed but
  a later manual same-cursor requery overwrote some raw first-run files whose
  byte labels depend on the current boot root. The clean rerun is the primary
  acceptance evidence.

## Hardware Result

Primary run:
minimal-entry-console-boundary-v22-rerun-20260630T215300Z.

The published selected archive retained the v21 static contract:

- selected path: da591740/kernel_2712.img;
- kernel byte count: 69,816;
- kernel SHA-256:
  22ed9e1b6f0c04a28a662c55ddb48769505001c53caab40a04cbea40fa397cb7;
- expected ready marker:
  TALOS: minimal-entry-control-ready capture-nonce=phase12-console-boundary-v21-static;
- expected direct early marker:
  TALOS: minimal-entry-console-boundary-start capture-nonce=phase12-console-boundary-v21-static;
- no-service/no-phase-transition guard tokens remained present.

The clean Pi 5 rerun retained decisive selected-byte and restore evidence:

- pre-power serial drain was empty on the first drain attempt;
- stable same-cursor TFTP delta captured 13 events, including two selected
  da591740/kernel_2712.img serves at 69,816 bytes;
- post-power direct-read serial captured firmware output but zero required
  minimal-entry ready marker occurrences and zero
  capture-nonce=phase12-console-boundary-v21-static occurrences;
- final pre-restore identity remained on selected tree
  1bf796cf078cef91ade341fb7e97562e2bd4be2f21c4215aaade53ad30366847;
- restore returned the lab to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Because identity, TFTP, serial freshness, final identity, and restore proof
were decisive, the terminal classification is marker-missing rather than
inconclusive.

## Terminal Classification

blocked-minimal-entry-console-boundary-marker-missing.

selected_next_task:
phase12-ssh-live-tcp-minimal-entry-console-boundary-closeout-v22-20260630.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, broad shell work,
fake/kernel-backed command expansion, and phase transition remain blocked.

## Findings

- fixed: executed the v21 console-boundary repair under hardwareTestLock with
  the exact selected archive contract and a clean rerun after first-run
  evidence hygiene became supporting-only.
- not-an-issue: selected-byte service, final pre-restore identity, and restore
  proof all matched the v21 candidate contract in the clean rerun.
- deferred: both repaired minimal-entry boundary markers were absent from the
  fresh post-power serial window; closeout must reconcile the first missing
  selected-kernel-entry fact before any broader live TCP work.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, broad shell work, fake command expansion,
  and phase transition as immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22/classification.json.
- Primary hardware run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22/minimal-entry-console-boundary-v22-rerun-20260630T215300Z/.
- Supporting first run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22/minimal-entry-console-boundary-v22-20260630T213900Z/.
- Static materialization:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-console-boundary-preflight-v22/static/.

## Redaction Review

Task-owned lab-controller serial and TFTP artifacts are retained under the run
evidence directories. This summary records task ids, labels, hashes, byte
counts, marker names, classifications, and validation outcomes; it omits
packet payloads, SSH/session/key material, and private user data.

## Validation

- git status before action: recorded.
- static archive review: pass.
- Lab API candidate identity before publication and before power action: pass.
- Fresh serial cursor/drain: pass; pre-power drain was empty.
- TFTP delta before restore: pass; two selected 69,816-byte fetches in the
  clean rerun.
- Final pre-restore identity before restore: pass.
- Restore to predecessor-named baseline and confirm with lab API: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

Commit: recorded in talos-supervisor-state.json after final commit.
