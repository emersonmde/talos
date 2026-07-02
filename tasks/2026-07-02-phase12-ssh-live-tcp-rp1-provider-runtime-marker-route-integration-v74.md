# Phase 12 Live TCP RP1 Provider Runtime Marker Route Integration V74

Task: phase12-ssh-live-tcp-rp1-provider-runtime-marker-route-integration-v74-20260702
Status: accepted
Terminal classification: rp1-provider-runtime-marker-route-integration-accepted

## Summary

This task switches the selected Pi 5
rpi5_ssh_service_smoltcp_runtime_ready marker route from the older
deterministic-only runtime report to the accepted source-bound RP1 Ethernet
hardware frame-provider boundary. The route still accepts only local/source
metadata: live packet I/O, reachability, remote receipt, compatibility, service
success, ssh_ready, and phase transition remain false.

## Source Files Changed

- src/network.rs
- src/target/rpi5.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

## Route And Provider Summary

src/network.rs now has a provider-aware runtime marker report helper and a
selected source-bound RP1 provider route:

- live_tcp_runtime_marker_route_report remains the deterministic no-provider
  regression/control path.
- live_tcp_runtime_marker_route_report_with_source_bound_rp1_provider binds the
  accepted Rp1EthernetHardwareFrameProviderBindingReport into the
  DriverPacketAdapter/smoltcp/listener/descriptor-delivery lineage.
- Missing provider, link-not-ready provider, missing descriptor delivery, and
  missing deterministic device-interface binding remain explicit fail-closed
  states.

src/target/rpi5.rs now calls the provider-bound marker report for the selected
rpi5_ssh_service_smoltcp_runtime_ready scenario and prints
hardware-frame-provider-classification in both ready and blocked marker lines.

## Candidate Archive For V75

Non-published archive: target/talos-rpi5-rp1-provider-runtime-marker-v74-boot.tar.gz

- archive SHA-256: 15ac4dd758408676440babf4808adebbdd18bc0d9bbfc0e4a33a9f846b173665
- archive file count: 19
- selected da591740/kernel_2712.img bytes: 154520
- selected da591740/kernel_2712.img SHA-256: 8402ca01172608252eec7f6a933fb024b0cf782e57208e7102f5d8225e9d59f2
- arm64 Image header_image_size: 154520
- arm64 Image text_offset: 0
- arm64 Image flags: 12
- restore target for v75: phase12-ssh-v10-openssh-clean-pre-20260624T074100Z
- expected provider classification:
  rp1-ethernet-hardware-frame-provider-source-bound-local-only
- expected selected serial marker:

  TALOS: ssh-service-smoltcp-runtime-ready runtime-binding=accepted-deterministic-device-interface-delivery descriptor-facing-connection-delivered=true deterministic-device-interface-bound=true hardware-frame-provider-bound=true hardware-frame-provider-classification=rp1-ethernet-hardware-frame-provider-source-bound-local-only driver-packet-rx-frames=6 driver-packet-tx-frames=6 live-packet-io-accepted=false live-reachability-accepted=false remote-receipt-accepted=false compatibility-accepted=false ssh-ready=false claims-service-success=false claims-phase-transition=false

## Findings

- fixed: The selected Pi 5 runtime marker route previously consumed the
  no-provider deterministic-only report and would emit
  hardware-frame-provider-bound=false.
- fixed: Provider-bound route output now includes
  hardware-frame-provider-classification for both ready and blocked cases.
- fixed: The selected marker route now distinguishes missing provider and
  link-not-ready provider states through explicit fail-closed classifications.
- fixed: Unit/substitute coverage now proves the provider-bound route accepts
  only source-local provider metadata and keeps live packet I/O and SSH
  readiness false.
- deferred: v72 kernel_main, v60 route/runtime-ready, and v53 packet-I/O remain
  blocked or deferred until a later supervisor task explicitly reselects one
  with refreshed dependencies and gates.
- not-an-issue: The no-provider live_tcp_runtime_marker_route_report remains as
  a local regression/control surface and is not the selected Pi 5 route.

## Validation

- git status --short --branch before edits/action: ## main...origin/main [ahead
  303].
- jq empty: supervisor state JSON validated before promotion.
- fmt/lint: cargo fmt --all -- --check passed after formatting.
- unit/substitute: cargo -Zjson-target-spec test --quiet passed with QEMU on
  PATH; 898 no_std tests passed.
- image/archive inspection: non-published rpi5_ssh_service_smoltcp_runtime_ready
  archive materialized and scripts/rpi5-archive-review.sh passed with metadata
  above.
- docs: mdbook build required because docs/src files changed.
- diff hygiene: git diff --check and git diff --cached --check required before
  commit.

## Disposition

selected_next_task:
phase12-ssh-live-tcp-pi5-rp1-provider-runtime-marker-preflight-v75-20260702

planningNeeded: false

Redaction review: retained evidence contains deterministic source metadata,
test summaries, archive digests, and marker vocabulary only; no packet payload
captures, SSH key/session material, private user data, hardware log bytes, or
unnecessary stable external identifiers are retained.

Commit: pending
