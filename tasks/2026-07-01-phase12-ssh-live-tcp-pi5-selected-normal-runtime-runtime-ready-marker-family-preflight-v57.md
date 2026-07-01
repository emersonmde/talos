# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Runtime-Ready Marker-Family Preflight V57

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-no-route-start-marker-retained.

Evidence level: static archive/image review, lab-controller API, serial hardware
boot/output summary, TFTP log delta, known-good control, candidate rerun,
restore proof, task-owned JSON evidence, docs build, and redaction review.

## Goal

Run the selected v56 marker-family discriminator on the Pi 5 and determine the
deepest retained normal-runtime marker before any packet-I/O or OpenSSH
successor is allowed.

## Scope Performed

- Promoted the queued v57 hardware preflight after accepted v56 selected this
  exact task.
- Acquired hardwareTestLock before lab publication, boot snapshot mutation,
  Pi 5 power action, or hardware capture.
- Re-reviewed the authoritative v51 selected runtime-ready archive:
  target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz.
- Published the v51 selected archive and began a candidate capture. That first
  evidence set became inconclusive because the capture-helper/manual
  continuation evidence was contaminated after restore.
- Ran the required known-good control before accepting a rerun; the control
  proved the same TFTP cursor/delta path reported baseline
  da591740/kernel_2712.img serves at 104,136 bytes.
- Reran the selected candidate from a fresh cursor. The selected tree remained
  staged immediately after power and through final pre-restore identity, and
  TFTP served da591740/kernel_2712.img twice at 152,144 bytes.
- Restored the lab to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and
  released hardwareTestLock.

## Terminal Classification

selected-normal-runtime-no-route-start-marker-retained.

The decisive candidate rerun facts are:

- Archive SHA-256:
  44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9.
- Selected tree:
  c49997afe4dd2136706ad4f0dc05326d93abf60593c8a01104472984d5481bbc.
- Selected fetch: da591740/kernel_2712.img.
- Selected kernel size: 152,144 bytes.
- Selected kernel SHA-256:
  b3d4ff79d0790980f68ef446a7c41dcbe2858824bd29ce7f150f86fa053c7982.
- Post-power/pre-observe identity: selected tree retained.
- Same-window TFTP: two selected da591740/kernel_2712.img serves at
  152,144 bytes.
- Final pre-restore identity: selected tree retained with the selected fetch at
  152,144 bytes.
- Serial marker family: zero retained occurrences of TALOS: asm_start,
  TALOS: asm_pre_rust_entry, TALOS: kernel_main, TALOS:
  ssh-service-smoltcp-runtime-route-start capture-nonce=runtime-ready-static,
  TALOS: ssh-service-smoltcp-runtime-blocked capture-nonce=runtime-ready-static,
  and TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static.
- Restore proof: post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

This is not runtime-ready proof. Packet-I/O, OpenSSH/generated-root retry,
remote receipt, compatibility claim, service readiness, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, and phase transition
remain blocked.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-closeout-v57-20260701.

planningNeeded: false.

## Findings

- fixed: the first v57 candidate evidence was treated as inconclusive after the
  capture-helper/manual continuation overwrote the final/TFTP evidence after
  restore; it was not used for acceptance.
- fixed: the required known-good control was run before accepting a candidate
  rerun after that evidence contamination.
- not-an-issue: the lab TFTP cursor/delta path was healthy in the control and
  selected rerun; it reported baseline 104,136-byte serves for the control and
  selected 152,144-byte serves for the candidate.
- deferred: no selected serial marker-family member was retained, so the next
  closeout must reconcile a selected no-route-start frontier before any
  packet-I/O or OpenSSH successor.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57/evidence-map.json.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57/validation/archive-review.stdout.txt.
- Known-good control summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57/lab/v57-control/control-summary.json.
- Candidate rerun summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-marker-family-preflight-v57/lab/v57-candidate-rerun/candidate-rerun-summary.json.

## Redaction Review

Task-owned JSON evidence was redacted after derived summaries were created. Raw
serial text/base64 and TFTP peer/log-line fields were replaced with REDACTED
while retaining counts, filenames, byte counts, cursor metadata, tree hashes,
marker-family classifications, and restore status. No private key, seed,
public-key blob, signature, fingerprint, operator identity, or stable
secret-derived identifier is retained.

## Validation

- git status --short --branch before lab publication/hardware action:
  ## main...origin/main [ahead 278].
- jq empty on task-owned JSON evidence and supervisor state before and after
  lock changes: pass.
- Static archive/image review: pass; archive SHA-256
  44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9 and selected
  kernel SHA-256 b3d4ff79d0790980f68ef446a7c41dcbe2858824bd29ce7f150f86fa053c7982.
- Lab API candidate identity before publication, after publication,
  post-power/pre-observe, final pre-restore, and after restore: recorded.
- Fresh serial cursor and marker-family serial observation: recorded.
- GET /tftp/logs cursor delta: recorded; selected rerun served the selected
  fetch twice at 152,144 bytes.
- Known-good control: recorded after the first candidate evidence became
  inconclusive for capture/evidence reasons.
- Candidate rerun: recorded and used for terminal classification.
- Restore proof to accepted baseline before releasing hardwareTestLock: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-marker-family-closeout-v57-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
