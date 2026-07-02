# Phase 12 Live TCP Live Packet Ingress Discriminator Core V80

Task: phase12-ssh-live-tcp-live-packet-ingress-discriminator-core-v80-20260702
Status: accepted
Terminal classification: blocked-no-live-frame-provider

## Summary

This source/local task reviewed the selected runtime-ready route,
LiveTcpNetworkDeviceRuntimeReport, DriverPacketAdapter, and source-bound RP1
provider report boundary after v79 proved the Pi 5 reaches the local-only
runtime-ready marker. The implementation adds an explicit fail-closed live
packet ingress discriminator to the runtime report and Pi marker instead of
promoting deterministic DriverPacketAdapter traffic to live packet progress.

No lab publication, boot snapshot mutation, Pi 5 power action, serial/TFTP
capture, live packet proof, remote receipt, OpenSSH/generated-root retry,
compatibility claim, service success claim, ssh_ready=true, fake/kernel-backed
command expansion, broad shell work, or phase transition was performed.

## Findings

- fixed: LiveTcpNetworkDeviceRuntimeReport now carries
  live_packet_ingress_discriminator_classification so every runtime path labels
  packet-ingress status separately from descriptor delivery, deterministic
  adapter traffic, and RP1 provider metadata.
- fixed: The selected rpi5_ssh_service_smoltcp_runtime_ready marker now emits
  live-packet-ingress-discriminator=blocked-no-live-frame-provider with exact
  missing owners: live-frame-provider-owner=missing-rp1-dma-rx-frame-provider
  and packet-stimulus-owner=missing-lab-approved-packet-stimulus.
- fixed: The runtime-ready archive review now requires the fail-closed
  discriminator and missing-owner tokens before any later serialized Pi 5 proof
  can treat the candidate as review-clean.
- not-an-issue: Existing DriverPacketAdapter frame counts remain useful
  source/unit evidence for deterministic smoltcp descriptor delivery, but they
  are labeled host-only and do not satisfy live_packet_io_accepted.
- deferred: RP1 DMA RX/TX frame ownership, interrupt/completion ownership,
  packet stimulus, live packet ingress, remote receipt, OpenSSH compatibility,
  service success, ssh_ready, and phase transition remain unaccepted.

## Evidence

- static inspection: src/network.rs route/report review found the selected
  provider path still uses source-bound RP1 metadata and
  driver_packet_smoltcp_listener_transfer host-local frames; no honest live
  frame provider or packet-stimulus hook exists in this source boundary.
- source implementation: src/network.rs adds the packet-ingress discriminator
  classification field and tests for runtime-prerequisite-missing,
  deterministic-host-only, no-live-frame-provider, and provider-link-not-ready
  cases.
- marker implementation: src/target/rpi5.rs emits the discriminator and
  missing-owner tokens in runtime-ready and runtime-blocked markers.
- script implementation: scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh
  requires the new discriminator and owner tokens.
- validation: cargo fmt --all -- --check passed with the documented Talos Rust
  environment.
- validation: cargo -Zjson-target-spec test --quiet passed with the documented
  Talos Rust environment.
- validation: sh -n passed for the touched archive-review shell script.

## Disposition

first_missing_fact: no source-owned live RP1 DMA RX frame provider and no
bounded lab-approved packet stimulus hook are present after v79 local-only
runtime-ready marker retention.

selected_next_task: null

planningNeeded: true

No candidate archive was materialized, so no v81 archive path, archive SHA-256,
selected kernel SHA-256/byte count, or capture nonce is recorded.

Redaction review: retained evidence is limited to source paths, task ids,
classification strings, and validation results. No packet payloads, SSH
key/session material, private data, raw hardware logs, or external identifiers
are retained.

Commit: recorded in talos-supervisor-state.json after final commit.
