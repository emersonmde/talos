# Phase 12 Live TCP Pi 5 Selected-Kernel Entry Retention Preflight V79

Task: phase12-ssh-live-tcp-pi5-selected-kernel-entry-retention-preflight-v79-20260702
Status: accepted
Terminal classification: provider-runtime-ready-local-only-retained

## Summary

This task ran the accepted v78 selected-kernel entry-retention candidate on the
serialized Pi 5 path. The candidate staged, served, retained final selected
identity, and restored the lab baseline. Serial used the saturated-cursor
direct-read fallback and retained the repaired kernel_main, provider route-entry,
and runtime-ready retention hierarchy. This proves the v78 repair moved the
frontier beyond kernel_main and provider route-entry to a local-only runtime-ready
marker. It does not accept packet I/O, remote receipt, OpenSSH compatibility,
service success, ssh_ready=true, fake command expansion, broad shell work, or a
phase transition.

## Candidate Identity

- archive: target/talos-rpi5-selected-kernel-entry-retention-v78-boot.tar.gz
- archive SHA-256:
  7149fcad9aa29159b3e68e0875c89b41930cd1c39ca382a1cd409593972ebcb6
- archive bytes: 302,817
- selected da591740/kernel_2712.img bytes: 156,040
- selected da591740/kernel_2712.img SHA-256:
  ba899e55a5ebe6beeac441d74590985b6aa1be046f57d13324f2d9e953ea9650
- capture nonce: entry-retention-v78
- selected tree hash:
  c7699880cfc54c80ee1e6a9bd05c7ae7c32e9284f5a84cf08d409719ac5c4e3e
- restore target: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z
- restored tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

## Hardware Evidence

Accepted run:
tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-kernel-entry-retention-preflight-v79/candidate-selected-20260702T184500Z

- hardwareTestLock was acquired before selected archive publication and
  released only after restore proof.
- The v78 archive staged selected tree
  c7699880cfc54c80ee1e6a9bd05c7ae7c32e9284f5a84cf08d409719ac5c4e3e with
  effective kernel_2712.img.
- Stable same-cursor TFTP before restore retained 13 events, including two
  selected da591740/kernel_2712.img serves at 156,040 bytes.
- Final pre-restore identity still matched the selected tree.
- Serial used the saturated-cursor direct-read fallback, retained 70,015 bytes,
  and retained two firmware NETWORK markers.
- Serial retained 53 occurrences each of the v78 kernel_main retention marker,
  provider route-start marker, and runtime-ready marker. The runtime-blocked
  marker had zero occurrences because this path reached the local-only
  runtime-ready terminal marker.
- The lab restored to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z with tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings

- fixed: The v78 retention repair is proved on Pi 5 through kernel_main,
  provider route-entry, and runtime-ready marker retention after selected kernel
  TFTP service and final selected identity.
- not-an-issue: The runtime-blocked marker is absent because the accepted run
  reached runtime-ready-local-only; false packet I/O, reachability, remote
  receipt, compatibility, service success, ssh_ready, and phase-transition
  claims remain explicit.
- not-an-issue: The saturated-cursor direct-read fallback is acceptable here
  because the selected TFTP service, final selected identity, nonce-bearing
  marker family, and restore proof are all decisive.
- deferred: Packet I/O, remote receipt, OpenSSH/generated-root retry,
  compatibility, service success, ssh_ready=true, fake command expansion, broad
  shell work, and phase transition remain blocked.

## Evidence

- image/archive inspection: scripts/rpi5-archive-review.sh passed for the v78
  selected archive before publication.
- lab-controller API: selected publication identity matched tree
  c7699880cfc54c80ee1e6a9bd05c7ae7c32e9284f5a84cf08d409719ac5c4e3e and
  selected da591740/kernel_2712.img at 156,040 bytes.
- lab-controller API: TFTP stable same-cursor delta retained 13 events and two
  selected kernel serves at 156,040 bytes.
- serial hardware boot/output: direct-read serial retained 70,015 bytes, two
  firmware NETWORK markers, and 53 occurrences of each retained kernel_main,
  provider route-start, and runtime-ready marker.
- lab-controller API: final pre-restore identity matched the selected tree; the
  lab restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Disposition

first_missing_fact: live packet ingress and remote receipt remain unproved after
local-only runtime-ready marker retention.

selected_next_task:
phase12-ssh-live-tcp-pi5-selected-kernel-entry-retention-closeout-v79-20260702

planningNeeded: false

Redaction review: retained evidence is limited to archive/kernel digests,
file-event metadata without client identifiers, marker counts, concise serial
classifications without raw serial text, lab tree identities, and restore proof.
No packet payloads, SSH key/session material, private user data, raw hardware log
bytes, or stable external identifiers are retained.
