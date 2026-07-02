# Phase 12 Live TCP RP1 Provider Runtime Marker Closeout V75

Task: phase12-ssh-live-tcp-rp1-provider-runtime-marker-closeout-v75-20260702
Status: accepted
Terminal classification: rp1-provider-runtime-marker-frontier-blocked

## Summary

This no-hardware closeout reconciles the v74 source route and the v75 Pi 5
preflight. The source route is accepted: the selected
rpi5_ssh_service_smoltcp_runtime_ready path consumes the source-bound RP1
Ethernet hardware frame-provider report and keeps live packet I/O and SSH
readiness false. The hardware frontier is blocked because the selected Pi 5 run
served the v74 selected kernel and preserved selected-tree identity, but the
serial window did not retain the expected provider-bound runtime-ready marker.

No successor is mechanically selected by this task. Supervisor planning is
required for the next bounded feature-led repair or discriminator.

## Evidence Disposition

- v74 source/route evidence: accepted. src/network.rs and src/target/rpi5.rs
  route the selected marker through the accepted source-bound RP1 provider
  binding and emit the provider classification while keeping
  live_packet_io_accepted=false, live_reachability_accepted=false,
  remote_receipt_accepted=false, compatibility_accepted=false,
  claims-service-success=false, ssh-ready=false, and
  claims-phase-transition=false.
- v75 Pi 5 evidence: blocked. The selected archive
  target/talos-rpi5-rp1-provider-runtime-marker-v74-boot.tar.gz has SHA-256
  15ac4dd758408676440babf4808adebbdd18bc0d9bbfc0e4a33a9f846b173665. The
  selected da591740/kernel_2712.img was 154,520 bytes with SHA-256
  8402ca01172608252eec7f6a933fb024b0cf782e57208e7102f5d8225e9d59f2. The
  selected run staged tree
  997bd5ddeaade62681e3d44481eb99b52bf253023a609c25d47fe1a1f11520dd, retained
  two selected kernel TFTP serves, and preserved final pre-restore selected
  identity, but retained zero occurrences of the expected provider-bound
  runtime-ready marker.
- Restore evidence: accepted. The lab was restored to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z with tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and
  hardwareTestLock was released.
- Stale marker-chain tasks: deferred. v72 kernel_main, v60 runtime-ready, and
  v53 packet-I/O remain blocked or deferred unless a supervisor task explicitly
  reselects one with refreshed dependencies, gates, and feature relevance.

## Findings

- fixed: The closeout records that source-bound provider route integration is
  accepted while the Pi 5 provider-bound marker frontier remains blocked.
- fixed: The first missing fact is explicit: the selected Pi 5 serial window did
  not retain the provider-bound runtime-ready marker after selected kernel TFTP
  service and final pre-restore selected identity.
- deferred: The next feature-led repair/discriminator is not chosen here because
  the hardware result gives the missing fact but not an objective mechanically
  queued successor with refreshed gates.
- deferred: v72, v60, and v53 remain deferred or blocked pending explicit
  supervisor reselection.
- not-an-issue: The accepted source-bound provider metadata is still local
  metadata only and does not accept live packet ingress, remote receipt,
  OpenSSH compatibility, service success, ssh_ready=true, or phase transition.
- not-an-issue: No additional hardware run is required for this closeout because
  v75 already captured serialized selected-window TFTP, serial, final identity,
  restore, and redaction evidence.

## Remaining Gap

first_missing_fact: selected Pi 5 serial window did not retain the
provider-bound runtime-ready marker after selected kernel TFTP service and final
pre-restore selected identity.

Before live packet ingress can be claimed, a future task must explain or repair
why the selected provider-bound marker path does not appear on Pi 5 despite the
selected v74 kernel being served. Any next task must keep live packet I/O,
remote receipt, compatibility, service success, ssh_ready=true, and phase
transition false until directly proved.

## Validation

- git status --short --branch before edits/action: ## main...origin/main [ahead
  306].
- jq empty: supervisor state JSON validated before promotion.
- static inspection: v74 task record, v75 task record, and v75 compact evidence
  map reviewed.
- docs: mdbook build required because docs/src files changed.
- diff hygiene: git diff --check and git diff --cached --check required before
  commit.

## Disposition

selected_next_task: null

planningNeeded: true

planningReason: v75 proves selected kernel service and selected-tree identity
but the provider-bound runtime-ready marker is absent, so the next bounded
feature-led repair/discriminator needs supervisor planning before packet-I/O,
OpenSSH/generated-root retry, stale marker-chain work, fake command expansion,
broad shell work, or phase transition.

Redaction review: retained closeout evidence is limited to task ids,
classifications, archive/kernel digests, tree hashes, TFTP event counts,
serial marker counts, restore identity, and disposition notes. No packet
payloads, SSH key/session material, private user data, hardware log bytes, or
stable external identifiers are newly retained by this closeout.
