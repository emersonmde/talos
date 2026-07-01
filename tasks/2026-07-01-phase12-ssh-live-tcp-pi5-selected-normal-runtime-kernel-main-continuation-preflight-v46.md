# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Kernel Main Continuation Preflight V46

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v46-20260701

Status: accepted as blocked before hardware action.

Classification: blocked-selected-normal-runtime-kernel-main-preflight.

Evidence level: static archive/image inspection, task-owned JSON evidence,
docs build, and diff checks. No hardwareTestLock acquisition, lab publication,
boot snapshot mutation, Pi 5 power cycle, serial/TFTP capture, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Run the serialized Pi 5 discriminator for the accepted v45 selected
normal-runtime kernel_main marker contract, or block before hardware if the
pre-publication contract cannot satisfy the task validation gates.

## Scope Performed

- Promoted this ready task after v45 was accepted at commit
  773dae203cbe36a0795e3a1d861587da8540e2d3 and supervisor planning added the
  explicit v46 Pi 5 preflight.
- Rechecked the non-published v45 archive before any hardwareTestLock
  acquisition, lab publication, boot snapshot mutation, or Pi 5 power action.
- Confirmed the archive SHA-256 matches the v46 validation gate, but the
  extracted selected/root kernel bytes hash to the v45 static evidence value
  rather than the SHA-256 embedded in the v46 task gate.
- Stopped before lab mutation because publishing an archive whose kernel hash
  does not match the explicit v46 validation gate would make the selected-byte
  claim ambiguous.

## Terminal Classification

blocked-selected-normal-runtime-kernel-main-preflight.

The v45 archive exists and still has archive SHA-256
72c28bafe9bedd1474fd1dfb19db101e9078122506db1b6f13bbbebdad383f19. The
archive review reports a valid kernel_main marker-loop contract with
TALOS: kernel_main capture-nonce=runtime-marker-route-static and a 152,896-byte
kernel. However, the v46 validation gate requires the published/kernel identity
to match selected kernel SHA-256
96057d2f8970808011a308f7b3a92da6feb85097b44590947a1ac145f85c6be6, while the
committed v45 static evidence and the current extracted archive both report
2f89f98cf9403ccee58c20f8014f3c5fd83accb2f99da2f35b4b1eec6928cdc5 for the
kernel bytes. The archive was therefore not published.

candidate identity: not captured because the candidate was not staged.

fresh serial cursor: not captured because no Pi 5 power action was allowed
after the pre-publication static contract mismatch.

TFTP delta: not captured because no candidate window was opened.

known-good control decision: not run. A control distinguishes capture/staging
failures after a candidate hardware attempt; here the first missing fact is a
static contract mismatch before lab mutation.

candidate rerun decision: not run. A rerun would publish an archive whose
kernel hash does not match the task gate.

selected_next_task: null.

planningNeeded: true.

first missing fact: supervisor must reconcile whether v46 should require the
actual v45 static/archive kernel SHA-256
2f89f98cf9403ccee58c20f8014f3c5fd83accb2f99da2f35b4b1eec6928cdc5, or whether
the v45 archive must be regenerated to match
96057d2f8970808011a308f7b3a92da6feb85097b44590947a1ac145f85c6be6 before any
hardware preflight.

Later facts remain unproved: selected Pi 5 kernel_main marker retention,
route-start, runtime-ready, packet-I/O, OpenSSH compatibility, remote receipt,
service readiness, ssh-ready=true, fake command expansion, broad shell work,
and phase transition.

## Findings

- fixed: stopped before hardware publication when the explicit v46 kernel hash
  gate did not match the v45 static evidence.
- not-an-issue: the archive SHA-256 and kernel_main archive review still match
  the committed v45 archive contract.
- deferred: a future serialized Pi 5 preflight must use one reconciled kernel
  hash contract before publishing.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v46/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v46/evidence-map.json.
- Archive SHA:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v46/static/archive-sha256.txt.
- Kernel SHA:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v46/static/kernel-sha256.txt.
- Kernel_main archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v46/validation/kernel-main-archive-review.stdout.txt.

## Redaction Review

Task-owned evidence retains task ids, hashes, byte counts, marker names,
classifications, and validation outcomes. It does not retain raw serial text,
raw TFTP peer/log-line fields, packet payloads, SSH/session/key material, boot
artifact bytes, private data, or stable secret-derived identifiers.

## Validation

- git status --short --branch before action: pass.
- Pre-publication archive SHA check: pass for archive, fail closed on kernel
  SHA contract mismatch.
- hardwareTestLock acquisition: not run because no lab mutation was allowed
  after the static contract mismatch.
- lab API candidate identity, fresh serial cursor, TFTP delta, known-good
  control, candidate rerun, and restore proof: not run; no candidate was
  staged and no hardware action occurred.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
