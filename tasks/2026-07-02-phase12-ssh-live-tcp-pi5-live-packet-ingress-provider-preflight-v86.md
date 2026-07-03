# Phase 12 Pi 5 Live Packet Ingress Provider Preflight v86

Task: phase12-ssh-live-tcp-pi5-live-packet-ingress-provider-preflight-v86-20260702

Terminal classification: inconclusive-after-control

Commit: recorded in talos-supervisor-state.json after final commit.

## Summary

This serialized Pi 5 preflight published the accepted v85 candidate archive,
power-cycled the Pi, captured serial/TFTP evidence, restored the lab to the
named baseline, then ran the required known-good control plus one candidate
rerun with a changed direct-read serial capture condition after the first
candidate serial window was contaminated by delayed buffered output.

The candidate archive was
target/tmp/phase12-ssh-live-packet-ingress-v85-20260702.tar.gz with SHA-256
6c7c8a60197b2ae27ada837b72db3e6b21ea6d49e17f5cbad4952b57d7e8ef79 and size
309638 bytes. The selected kernel_2712.img has SHA-256
8ada0e2dc610236569358f5cac0367516aa6c88ae369f4eb8d843089ee896921 and size
160088 bytes. The selected boot tree was
4c2590ce0a003847fe098f729a1e8ea5aaec71dd6de5d1314a89d1c03a260fb6. The lab
was restored to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z, proving
restored tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings

- fixed: hardwareTestLock was acquired before lab publication, Pi 5 power,
  serial/TFTP capture, and restore workflow, and the lab was restored before
  lock release.
- fixed: the accepted v85 candidate was staged and served twice as
  da591740/kernel_2712.img with the expected 160088-byte selected kernel size in
  both candidate runs.
- deferred: delayed serial bytes exposed v85 runtime-ready markers and
  live-packet-ingress-discriminator=blocked-no-live-frame-provider, but those
  bytes arrived in pre-control drain windows rather than a clean candidate
  post-power/pre-restore serial window.
- deferred: because the clean runtime-ready window was not established, bounded
  packet stimulus was not attempted as accepted evidence; no packet payload was
  retained.
- not-an-issue: remote receipt, OpenSSH/generated-root retry, compatibility,
  service success, ssh_ready, fake command expansion, broad shell work, and phase
  transition all remain unaccepted.

## Evidence

- candidate archive: tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-live-packet-ingress-provider-preflight-v86/evidence-summary.json
- first candidate TFTP delta: stable, two selected kernel_2712.img serves, both
  160088 bytes.
- known-good control: stable TFTP evidence captured after baseline power-cycle.
- candidate rerun: staged selected tree and final pre-restore selected identity
  matched; TFTP delta again saw two 160088-byte selected kernel serves.
- classification reason: selected candidate TFTP service and final pre-restore
  identity were decisive, but serial marker ordering remained contaminated after
  the required known-good control and one rerun with manual direct /serial/read,
  so the proof is inconclusive-after-control rather than accepted live ingress.

## Acceptance

- terminal classification: inconclusive-after-control
- selected_next_task:
  phase12-ssh-live-tcp-live-packet-ingress-provider-closeout-v87-20260702
- planningNeeded: false
- hardwareTestLock: acquired before hardware/lab action and released after
  restored identity proof.
- No remote receipt, OpenSSH/generated-root retry, compatibility claim, service
  success claim, ssh_ready=true, fake/kernel-backed command expansion, broad
  shell work, or phase transition was accepted.

## Validation

- git status --short --branch before lab action: ## main...origin/main [ahead 318]
- scripts/rpi5-archive-review.sh on v85 archive: passed.
- scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh on v85
  archive and nonce: passed.
- lab API GET /status before publication, after publication, final pre-restore,
  and post-restore: retained in task-owned evidence.
- serial capture: retained first runtime-readiness primary, control direct-read,
  and candidate rerun direct-read artifacts.
- TFTP capture: retained stable first candidate delta, control delta, and rerun
  candidate delta.
- jq empty on task-owned JSON evidence: passed before commit.
- mdbook build, git diff --check, and git diff --cached --check: run before
  commit.

## Redaction Review

No packet payloads, SSH key/session material, private data, remote receipt, or
external packet identifiers are retained. Packet-stimulus evidence records no
payload and only the metadata required to show stimulus was not accepted.
