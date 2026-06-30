# Phase 12 SSH Live TCP Candidate Entry Preflight Contract V10

Task id: phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10-20260630

Status: accepted after commit.

Classification: candidate-entry-preflight-contract-ready.

Evidence level: static accepted-evidence review, non-published Pi 5
candidate boot-tree/archive materialization, archive/helper validation,
task-owned JSON evidence, docs build, and diff checks. No hardware, lab
publication, boot snapshot mutation, Pi 5 power action, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, broad shell work, or phase transition was
performed.

## Goal

Define the direct live TCP candidate entry preflight contract selected by the
accepted scenario-specific reconciliation, before any candidate publication or
Pi 5 power action.

## Scope Performed

- Promoted this ready no-hardware contract only after the supervisor refreshed
  dependencies to the accepted
  phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation-20260630
  result.
- Reviewed the accepted scenario-specific reconciliation and current-tree
  production-timer Pi 5 proof.
- Materialized a fresh non-published
  rpi5_ssh_service_smoltcp_runtime_ready candidate boot tree/archive with
  capture nonce candidate-entry-preflight-v10.
- Ran the existing rpi5 archive/helper review, retained only manifests, hashes,
  Image header fields, selected mirror proof, marker contract, and validation
  output, then removed the generated boot tree and archive bytes.
- Stopped before hardware, lab publication, candidate publication, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, broad shell work, or phase transition.

## Terminal Classification

candidate-entry-preflight-contract-ready.

The v10 candidate preflight is now bound to source commit
fb7f371765f4e5022dc56ffa038c5a6c338d5bcd and boot scenario
rpi5_ssh_service_smoltcp_runtime_ready with capture nonce
candidate-entry-preflight-v10. Fresh non-published archive review proves the
selected da591740/kernel_2712.img path is present, matches the root kernel
image, has 152,152 bytes, SHA-256
3e260a556cbe39c0fd49c0649ce5ec523491cac77a342e843d1fbd62f23dbd3c, valid
Image header fields, and retains the expected route-start/runtime-ready marker
tokens. The tarball used for review had SHA-256
9e12d233123992bbf4187996214c4267696985d152ada7565240ca3f4ad7b143 and was
deleted after metadata retention.

The future hardware preflight must use the same source commit, capture nonce,
selected da591740/kernel_2712.img identity, and marker contract. It must acquire
hardwareTestLock before publication or power, retain fresh serial and TFTP
cursors, prove final pre-restore candidate identity, check marker order, restore
to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z /
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and release
the lock only after restore proof.

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-preflight-v10-20260630.

planningNeeded: false.

## Contract For Future Hardware Task

- Candidate source commit:
  fb7f371765f4e5022dc56ffa038c5a6c338d5bcd.
- Candidate boot scenario: rpi5_ssh_service_smoltcp_runtime_ready.
- Capture nonce: candidate-entry-preflight-v10.
- Selected fetch path: da591740/kernel_2712.img.
- Expected candidate kernel: 152,152 bytes, SHA-256
  3e260a556cbe39c0fd49c0649ce5ec523491cac77a342e843d1fbd62f23dbd3c.
- Expected markers, in order:
  TALOS: ssh-service-smoltcp-runtime-route-start
  capture-nonce=candidate-entry-preflight-v10, then
  TALOS: ssh-service-smoltcp-runtime-ready
  capture-nonce=candidate-entry-preflight-v10.
- Known-good control and restore target:
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z /
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, with
  104,136-byte da591740/kernel_2712.img selected fetches and
  rpi5-production-timer-preemption: PASS as the downstream Talos-side oracle.
- Allowed terminal classifications for the future hardware task:
  candidate-capture-ready, blocked-candidate-identity,
  blocked-candidate-kernel-not-starting, blocked-candidate-tftp-capture,
  blocked-runtime-marker-route, blocked-capture-window-contract,
  blocked-restore, or inconclusive-with-required-discriminator.

candidate-capture-ready remains the only classification that may select a
packet-I/O discriminator. Packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility, service success, ssh-ready=true, broad shell work, and
phase transition remain blocked for every blocked/inconclusive classification.

## Findings

- fixed: bound the v10 candidate preflight to the accepted scenario-specific
  reconciliation and selected successor task.
- fixed: recorded exact candidate source commit, capture nonce, archive/kernel
  metadata, selected fetch identity, marker contract, known-good control,
  restore target, lock lifecycle, and fail-closed classifications.
- not-an-issue: no source/script repair was required; static archive review
  already proves the candidate marker route and selected mirror contract.
- deferred: candidate publication, Pi 5 power, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, broad shell work, and phase transition remain blocked.
- removed: generated non-published candidate boot tree and archive bytes were
  deleted after retaining task-owned metadata.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10/evidence-map.json.
- Candidate archive review:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10/validation/candidate-archive-review.stdout.txt.
- Candidate metadata:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10/materialized/candidate/boot-tree-manifest.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10/materialized/candidate/archive-sha256.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10/static/candidate-kernel-sha256.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10/static/candidate-image-header-words.txt,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-preflight-contract-v10/static/marker-contract.json.

## Redaction Review

This no-hardware task retained no raw serial text, raw TFTP peer/log-line
fields, packet payloads, key material, session material, boot artifact bytes,
private user data, stable secret-derived identifiers, or unnecessary hardware
data. Evidence is limited to task ids, hashes, byte counts, static strings,
validation command results, snapshot names, tree hashes, and fixed
classification strings.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted tracked Talos changes before task-owned evidence.
- Static review of accepted repair/quarantine evidence: pass.
- Non-published candidate boot-tree/archive static review: pass.
- cargo fmt --all -- --check: not run; Rust source was not touched.
- cargo -Zjson-target-spec test --quiet: not run; Rust source or target
  routing was not touched.
- sh -n: not run; shell helpers/classifiers were not touched.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-preflight-v10-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
