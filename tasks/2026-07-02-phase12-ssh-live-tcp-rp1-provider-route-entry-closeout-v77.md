# Phase 12 Live TCP RP1 Provider Route-Entry Closeout V77

Task: phase12-ssh-live-tcp-rp1-provider-route-entry-closeout-v77-20260702
Status: accepted
Terminal classification: provider-route-entry-frontier-blocked

## Summary

This no-hardware closeout reconciles the v76 source route-entry discriminator
and the v77 serialized Pi 5 result. The source route now has a provider
route-start marker and fail-closed provider runtime markers, but the selected
Pi 5 run served the v76 kernel and retained final selected identity without
retaining kernel_main, route-start, runtime-blocked, or runtime-ready markers.
The live TCP frontier is blocked before kernel_main after selected kernel TFTP
service. This does not accept packet I/O, remote receipt, OpenSSH
compatibility, service success, ssh_ready=true, or a phase transition.

## Evidence Map

- v76 source discriminator:
  phase12-ssh-live-tcp-rp1-provider-route-entry-discriminator-v76-20260702
  accepted provider-route-entry-source-repaired. It materialized
  target/talos-rpi5-rp1-provider-route-entry-v76-boot.tar.gz with archive
  SHA-256 e2779312ef50ddb55573524c4079608c2d0ee0626cdb1e57e8e30b1c43269332,
  selected da591740/kernel_2712.img SHA-256
  6fc026100f0ea9e5157997eec12e1b3cc12000fdae243067f4349c7f4abffc20 at
  155,096 bytes, and capture nonce route-entry-v76.
- v77 Pi 5 preflight:
  phase12-ssh-live-tcp-pi5-rp1-provider-route-entry-preflight-v77-20260702
  accepted provider-route-entry-blocked-before-kernel-main. Under
  hardwareTestLock, the v76 archive staged selected tree
  051125b2e9111036f7b5310634078a1e4e673bacbd53f4d05803f0882a460a70, TFTP
  retained 13 events including two selected kernel serves at 155,096 bytes,
  final pre-restore identity still matched the selected tree, and restore
  returned to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- v77 serial classification: the saturated-cursor direct-read window retained
  4,475 bytes with two firmware NETWORK markers and no kernel_main,
  provider route-start, runtime-blocked, or runtime-ready marker.

## Findings

- fixed: The closeout records that v76 source repair is accepted and that v77
  selected hardware evidence is the authoritative Pi 5 boundary for this
  archive.
- fixed: The first missing fact is now explicit: selected Pi 5 serial retained
  no Talos kernel_main or provider route-entry marker after selected kernel
  TFTP service and final selected identity.
- deferred: v72 kernel_main, v60 runtime-ready, and v53 packet-I/O remain stale
  marker-chain tasks and are not mechanically promoted by this closeout.
- deferred: Packet I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh_ready=true, fake/kernel-backed command
  expansion, broad shell work, and phase transition require supervisor planning
  with refreshed dependencies and gates.
- not-an-issue: The firmware NETWORK markers and selected TFTP serves prove
  firmware/network service for the staged selected kernel, not Talos kernel
  entry or live TCP packet ingress.

## Evidence

- static inspection: v76 and v77 task records reviewed for source route-entry
  repair, archive identity, selected kernel byte/digest evidence, hardware
  lock/restore proof, and marker-family counts.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- JSON validation: jq empty passed on supervisor state.
- diff hygiene: git diff --check and git diff --cached --check passed.

## Disposition

first_missing_fact: selected Pi 5 serial retained no Talos kernel_main or
provider route-entry marker after selected kernel TFTP service and final
pre-restore selected identity.

selected_next_task: null

planningNeeded: true

Redaction review: retained evidence is limited to task ids, archive/kernel
digests and byte counts, marker-count classifications, lab tree identities,
and restore proof. No packet payloads, SSH key/session material, private user
data, unnecessary raw hardware log bytes, or stable external identifiers are
retained.
