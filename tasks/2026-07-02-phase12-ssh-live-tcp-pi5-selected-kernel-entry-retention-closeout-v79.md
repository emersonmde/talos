# Phase 12 Live TCP Pi 5 Selected-Kernel Entry Retention Closeout V79

Task: phase12-ssh-live-tcp-pi5-selected-kernel-entry-retention-closeout-v79-20260702
Status: accepted
Terminal classification: provider-runtime-ready-local-only-proved

## Summary

This no-hardware closeout reconciles the v78 source/archive repair and the v79
serialized Pi 5 result. The selected runtime-ready provider route now has
decisive hardware evidence through selected kernel TFTP service, final selected
identity, kernel_main retention, provider route-entry retention, and the
local-only runtime-ready marker. The first missing fact moves beyond marker
retention to live packet ingress and remote receipt.

No code implementation, lab publication, boot snapshot mutation, Pi 5 power
cycle, serial capture, TFTP capture, packet-I/O proof, remote receipt,
OpenSSH/generated-root retry, compatibility claim, service success claim,
ssh_ready=true, fake/kernel-backed command expansion, broad shell work, or
phase transition was performed.

## Evidence Map

- v78 source/archive repair:
  phase12-ssh-live-tcp-pi5-selected-kernel-entry-retention-repair-v78-20260702
  accepted selected-kernel-entry-retention-repair-ready. It materialized
  target/talos-rpi5-selected-kernel-entry-retention-v78-boot.tar.gz with
  archive SHA-256
  7149fcad9aa29159b3e68e0875c89b41930cd1c39ca382a1cd409593972ebcb6,
  archive bytes 302,817, selected da591740/kernel_2712.img SHA-256
  ba899e55a5ebe6beeac441d74590985b6aa1be046f57d13324f2d9e953ea9650 at
  156,040 bytes, and capture nonce entry-retention-v78.
- v78 marker contract: after the source-bound RP1 provider report reaches a
  local terminal outcome, the selected route continuously replays the retained
  kernel_main, provider route-start, and terminal runtime ready/blocked/error
  marker hierarchy with live packet I/O, reachability, remote receipt,
  compatibility, service success, ssh_ready, and phase-transition claims false.
- v79 Pi 5 preflight:
  phase12-ssh-live-tcp-pi5-selected-kernel-entry-retention-preflight-v79-20260702
  accepted provider-runtime-ready-local-only-retained. Under hardwareTestLock,
  the v78 archive staged selected tree
  c7699880cfc54c80ee1e6a9bd05c7ae7c32e9284f5a84cf08d409719ac5c4e3e, TFTP
  retained 13 events including two selected kernel serves at 156,040 bytes,
  final pre-restore identity still matched the selected tree, and restore
  returned to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- v79 serial classification: the saturated-cursor direct-read window retained
  70,015 bytes with two firmware NETWORK markers and 53 occurrences each of
  the v78 kernel_main, provider route-start, and runtime-ready markers. The
  runtime-blocked marker had zero occurrences because the path reached the
  local-only runtime-ready marker.

## Findings

- fixed: The v78 retention repair is accepted as the hardware-backed selected
  path through kernel_main, provider route-entry, and local-only runtime-ready
  marker retention.
- fixed: The v77 first missing fact before kernel_main is closed by v79
  selected TFTP, final selected identity, and repeated nonce-bearing
  kernel_main/provider/runtime marker retention.
- deferred: Live packet ingress, remote receipt, OpenSSH/generated-root retry,
  compatibility, service success, ssh_ready=true, fake/kernel-backed command
  expansion, broad shell work, and phase transition remain outside the accepted
  evidence.
- deferred: v72 kernel_main, v60 runtime-ready, and v53 packet-I/O remain stale
  marker-chain tasks and are not mechanically promoted by this closeout.
- not-an-issue: The missing runtime-blocked marker does not weaken this
  classification because the accepted run reached the local-only runtime-ready
  terminal marker and preserved fail-closed readiness claims.

## Evidence

- static inspection: v78 and v79 task records reviewed for source/archive
  repair, archive identity, selected kernel byte/digest evidence, hardware
  lock/restore proof, and marker-family counts.
- task-owned evidence:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-kernel-entry-retention-closeout-v79/evidence-map.json.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- JSON validation: jq empty passed on supervisor state and task-owned evidence.
- diff hygiene: git diff --check and git diff --cached --check passed.

## Disposition

first_missing_fact: live packet ingress and remote receipt remain unproved after
selected Pi 5 TFTP service, final selected identity, and local-only
runtime-ready marker retention.

selected_next_task: null

planningNeeded: true

Redaction review: retained evidence is limited to task ids, archive/kernel
digests and byte counts, marker-count classifications, lab tree identities,
and restore proof. No packet payloads, SSH key/session material, private user
data, unnecessary raw hardware log bytes, or stable external identifiers are
retained.

Commit: recorded in talos-supervisor-state.json after final commit.
