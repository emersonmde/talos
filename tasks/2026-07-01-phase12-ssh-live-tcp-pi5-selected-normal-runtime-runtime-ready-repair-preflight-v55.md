# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Runtime-Ready Repair Preflight V55

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55-20260701

Status: accepted after final validation and commit.

Classification: blocked-selected-normal-runtime-runtime-ready-preflight.

Evidence level: image/archive inspection, lab-controller API, serial hardware
boot/output summary, TFTP log delta, restore proof, task-owned JSON evidence,
and redaction review.

## Goal

Run exactly one bounded Pi 5 runtime-ready repair/discriminator preflight after
v54 accepted the post-power identity control discriminator.

## Scope Performed

- Promoted the queued v55 hardware preflight after accepted v54 selected this
  exact task.
- Acquired hardwareTestLock before lab publication, boot snapshot mutation,
  Pi 5 power action, or hardware capture.
- Republished the accepted v51 runtime-ready archive:
  target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz.
- Ran the revised capture helper with the immediate post-power,
  pre-serial-observe identity checkpoint.
- Restored the lab to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and
  released hardwareTestLock.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  service readiness, ssh-ready=true, fake command expansion, broad shell work,
  or phase transition.

## Terminal Classification

blocked-selected-normal-runtime-runtime-ready-preflight.

The v51 selected archive and staged candidate identity were decisive:

- Archive SHA-256:
  44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9.
- Selected tree:
  c49997afe4dd2136706ad4f0dc05326d93abf60593c8a01104472984d5481bbc.
- Selected fetch: da591740/kernel_2712.img.
- Selected kernel size: 152,144 bytes.
- Required marker:
  TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static.

The revised capture bundle discriminated the v52 ambiguity. The selected tree
remained staged immediately after power cycle and before serial observation,
TFTP served da591740/kernel_2712.img twice at the expected 152,144 bytes, and
final pre-restore identity still matched the selected tree. Restore returned to
baseline tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The first missing fact is now narrower than v52: the selected runtime-ready
marker was not retained in the candidate serial window. The v4 marker check
therefore rejected runtime-ready proof with
run-unique-capture-nonce-not-present-after-power and
required-marker-not-present-after-power.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-repair-closeout-v55-20260701.

planningNeeded: false.

## Findings

- fixed: reran the selected runtime-ready candidate with the v54 post-power
  identity checkpoint, eliminating the stale/baseline publication ambiguity.
- fixed: recorded selected post-power identity, selected same-window TFTP
  service, final selected pre-restore identity, restore proof, and marker-check
  rejection in task-owned evidence.
- not-an-issue: known-good control was not required because the candidate
  evidence was decisive rather than stale, missing, or internally contradicted.
- deferred: no source repair was attempted after the decisive hardware result;
  the closeout must reconcile the runtime-ready frontier before any packet-I/O
  or OpenSSH successor.
- removed: packet-I/O, OpenSSH/generated-root retry, service readiness,
  ssh-ready=true, fake command expansion, broad shell work, and phase
  transition from this preflight result.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55/lab/v55-candidate/capture-invariant-summary.json.
- TFTP delta:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55/lab/v55-candidate/tftp-delta-stable-pre-restore.json.
- Serial summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55/lab/v55-candidate/serial-observe-window.json.
- Marker check:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55/lab/v55-candidate/proof-identity-join-v4-check.json.
- Restore proof:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-repair-preflight-v55/lab/v55-candidate/post-restore-status.json.

## Redaction Review

Task-owned JSON evidence was redacted after derived checks. Raw serial text,
serial base64, TFTP peer fields, and raw TFTP log lines were replaced with
REDACTED while retaining counts, filenames, byte counts, cursor metadata,
identity fields, and classifications. No private key, seed, public-key blob,
signature, fingerprint, operator identity, or stable secret-derived identifier
is retained.

## Validation

- git status --short --branch before lab publication/hardware action:
  ## main...origin/main [ahead 274].
- jq empty on task-owned JSON evidence and supervisor state before and after
  lock changes: pass.
- Archive inspection with scripts/rpi5-archive-review.sh: pass; selected
  kernel size and header image size are 152,144 bytes.
- Lab API candidate identity before publication and before power action:
  recorded.
- Fresh serial cursor before candidate power-cycle: recorded.
- GET /tftp/logs candidate delta and cursor_end retention: recorded.
- Known-good control: not run; candidate evidence was decisive, selected, and
  internally consistent.
- Candidate rerun: not run; the first run was not inconclusive.
- Restore proof to accepted baseline snapshot before releasing
  hardwareTestLock: pass.
- git diff --check: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-repair-closeout-v55-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
