# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Runtime-Ready Continuation Preflight V52

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52-20260701

Status: accepted after final validation and commit.

Classification: blocked-selected-normal-runtime-runtime-ready-preflight.

Evidence level: static inspection, lab-controller API, serial hardware
boot/output, TFTP log delta, image/archive inspection, task-owned JSON
evidence, docs build, and diff checks.

## Goal

Run the bounded serialized Pi 5 preflight for the v51 selected normal-runtime
runtime-ready discriminator contract.

## Scope Performed

- Promoted the ready v52 hardware preflight after v51 accepted and selected
  this task.
- Acquired hardwareTestLock before publication, boot snapshot mutation,
  Pi 5 power action, and hardware capture.
- Republished the accepted v51 selected archive:
  target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz.
- Captured selected preflight identity, fresh serial cursor, serial drain,
  TFTP cursor/delta, serial observe windows, final pre-restore identity,
  restore proof, and redaction review.
- Ran a candidate rerun after the initial helper path was inconclusive.
- Restored the lab to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Terminal Classification

blocked-selected-normal-runtime-runtime-ready-preflight.

The v51 selected archive was materialized and published as expected:

- Archive SHA-256:
  44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9.
- Selected tree:
  c49997afe4dd2136706ad4f0dc05326d93abf60593c8a01104472984d5481bbc.
- Selected fetch: da591740/kernel_2712.img.
- Selected kernel size: 152,144 bytes.
- Selected kernel SHA-256:
  b3d4ff79d0790980f68ef446a7c41dcbe2858824bd29ce7f150f86fa053c7982.
- Required marker:
  TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static.

The candidate rerun retained selected preflight identity before power action, but
it did not retain the selected tree through the decisive hardware window:

- Stable same-cursor TFTP observed 13 events and two
  da591740/kernel_2712.img fetches, both 104,136 bytes rather than the expected
  152,144 bytes.
- Final pre-restore identity was the accepted baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, not the
  selected v51 tree.
- The saturated direct-read serial window retained 4,475 bytes, but zero
  occurrences of the required runtime-ready marker.
- boot-staging-identity-check rejected the run with
  tftp-expected-fetch-byte-mismatch, final-pre-restore-selected-tree-mismatch,
  final-pre-restore-is-baseline, and
  final-pre-restore-expected-fetch-byte-mismatch.
- capture-window-v5-check rejected runtime-ready claims with
  run-unique-capture-nonce-not-present-after-power,
  required-marker-not-present-after-power, TFTP byte mismatch, and final
  pre-restore identity mismatch.

Because the task allows a terminal blocked classification, v52 is accepted as a
bounded preflight result, not as runtime-ready marker proof.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-closeout-v52-20260701.

planningNeeded: false.

## Findings

- fixed: repaired the paused v52 record by using the retained candidate rerun
  and root-endpoint fallback evidence instead of classifying post-restore replay
  as selected-byte proof.
- fixed: recorded the selected v51 archive contract, publication identity,
  fresh serial/TFTP cursors, stable TFTP delta, final pre-restore identity, and
  restore proof in task-owned evidence.
- fixed: preserved GET / 404 as endpoint-semantics evidence only; /boot/files
  remains the selected-tree identity source.
- not-an-issue: the task's terminal classification set explicitly permits
  blocked-selected-normal-runtime-runtime-ready-preflight when the marker proof
  cannot be accepted.
- deferred: a follow-up closeout must reconcile this blocked preflight before
  any packet-I/O, OpenSSH/generated-root retry, remote receipt, service
  readiness, ssh-ready=true, fake command expansion, broad shell work, or phase
  transition.
- removed: runtime-ready marker-retained, packet-I/O, OpenSSH compatibility,
  remote receipt, service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase-transition claims from this task result.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52/evidence-map.json.
- Aggregate redacted run summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52/lab/run-summary-redacted.json.
- Candidate rerun redacted run summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52/lab/v52-candidate-rerun/run-summary-redacted.json.
- Candidate rerun capture summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52/lab/v52-candidate-rerun/capture-invariant-summary.json.
- Candidate rerun TFTP delta:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52/lab/v52-candidate-rerun/tftp-delta-stable-pre-restore.json.
- Candidate rerun serial observe window:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52/lab/v52-candidate-rerun/serial-observe-window.json.
- Candidate rerun final pre-restore identity:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52/lab/v52-candidate-rerun/final-pre-restore-status.json.
- Candidate rerun restore proof:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-runtime-ready-continuation-preflight-v52/lab/v52-candidate-rerun/post-restore-status.json.

## Redaction Review

The retained redacted summaries record raw serial text as redacted, serial
base64 as redacted, raw TFTP peer and line fields as redacted, packet payloads
as absent, SSH session/key material as absent, boot artifact bytes as not
retained in task JSON, and private user data as absent.

## Validation

- git status --short --branch before lab publication/hardware action:
  ## main...origin/main [ahead 271].
- Candidate identity via lab-controller API GET / and /boot/files: recorded;
  GET / returned HTTP 404 and /boot/files was used as the selected-tree identity
  source.
- Fresh serial cursor before candidate power action: recorded.
- TFTP cursor/delta from GET /tftp/logs: recorded; rerun stable delta observed
  baseline-sized fetches.
- Candidate run/rerun with fresh evidence windows: recorded; rerun is the
  terminal evidence.
- Pre-restore identity and TFTP/serial evidence review: recorded.
- Restore to accepted baseline snapshot and post-restore proof: recorded.
- Redaction review: pass.
- jq empty on supervisor state and task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-closeout-v52-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
