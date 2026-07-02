# Phase 12 RP1 DMA RX Descriptor/Ring Source Core v88

Task: phase12-ssh-live-tcp-rp1-dma-rx-descriptor-ring-source-core-v88-20260702

Terminal classification: rp1-dma-rx-descriptor-ring-source-ready

Commit: recorded in talos-supervisor-state.json after final commit.

## Summary

This no-hardware source task implements the missing source-owned RP1 DMA RX
descriptor/ring boundary that v83 identified before packet stimulus or Pi 5 live
packet-ingress proof. The new boundary is local/source-only: it defines the
descriptor-ring owner, descriptor layout/state model, DMA/cache ownership
policy, polling/completion semantics, metadata retained, redaction policy, and
fail-closed states without retaining packet payload bytes or accepting live
packet I/O.

src/rp1_ethernet.rs now exposes a metadata-only RX descriptor/ring contract and
report. src/network.rs ties that report to the existing
DriverPacketAdapter/smoltcp listener descriptor-delivery boundary through a
handoff report that can accept descriptor metadata while keeping deterministic
DriverPacketAdapter frames labeled host-only and live packet claims false.
src/target/rpi5.rs updates the selected runtime-ready marker owner tokens to
name the source-owned descriptor/ring metadata boundary while keeping the packet
stimulus owner missing.

## Findings

- fixed: v83's missing source-owned RP1 DMA RX descriptor/ring ownership path now
  has a local source contract in src/rp1_ethernet.rs. The contract identifies the
  Talos source owner, RX descriptor layout, RX_USED/RX_WRAP ownership model,
  RBQP/RBQPH base-address handoff, cache/DMA policy, polling completion
  semantics, metadata retained, redaction policy, and fail-closed states.
- fixed: src/network.rs now has a descriptor-ring handoff report that binds the
  RX metadata-only source report to the existing descriptor-facing
  DriverPacketAdapter/smoltcp listener boundary without injecting RP1 payload
  bytes or reclassifying deterministic host-only frames as live ingress.
- fixed: src/target/rpi5.rs no longer reports the runtime-ready marker owner as
  missing-rp1-dma-rx-frame-provider on the accepted source route. It now reports
  source-owned-rp1-dma-rx-descriptor-ring-metadata-only, the source owner, and a
  metadata-only/no-payload redaction token while keeping packet-stimulus-owner
  missing.
- not-an-issue: the source task intentionally does not make packet payloads
  available. live_packet_io, live_reachability, remote_receipt, compatibility,
  service_success, ssh_ready, and phase-transition claims remain false.
- deferred: A bounded packet stimulus contract and serialized Pi 5
  packet-ingress proof remain future work; this task only makes the prerequisite
  descriptor/ring source boundary honest and testable.

## Source-Path Notes

- Source evidence comes from the retained Raspberry Pi Linux macb_main.c
  evidence: DEFAULT_RX_RING_SIZE/MIN_RX_RING_SIZE/MAX_RX_RING_SIZE, macb_rx_desc,
  macb_rx_ring_wrap, RBQP/RBQPH programming, macb_set_addr/macb_get_addr
  RX_USED masking, and macb_rx/gem_rx polling paths.
- The descriptor/ring state model is driver-owned-empty ->
  hardware-owned-ready -> hardware-completed-frame ->
  driver-reclaimed-metadata-only. Talos retains descriptor index, frame length,
  ring-wrap, and classification only.
- The DMA/cache policy is coherent-or-explicit-cache-maintenance before clearing
  RX_USED and after observing RX_USED set. Interrupt completion is not required
  for source acceptance; polling is the accepted source boundary until RP1_INT_ETH
  routing is hardware-proved.
- The network handoff report requires the existing descriptor-facing
  DriverPacketAdapter/smoltcp listener route to be ready and a non-payload RX
  metadata report to be present. It keeps packet_payload_available=false and
  live packet I/O false.

## Acceptance

- selected_next_task:
  phase12-ssh-live-tcp-bounded-packet-stimulus-contract-core-v84-20260702
- planningNeeded: false
- terminal classification: rp1-dma-rx-descriptor-ring-source-ready
- No lab publication, boot snapshot mutation, Pi 5 power action, serial/TFTP
  capture, packet stimulus, live packet-ingress hardware proof, remote receipt,
  OpenSSH/generated-root retry, compatibility claim, service success claim,
  ssh_ready=true, fake/kernel-backed command expansion, broad shell work, or
  phase transition was performed.

## Validation

- git status --short --branch before edits/action: ## main...origin/main [ahead 315]
- cargo fmt --all -- --check: passed after formatting
- cargo -Zjson-target-spec test --quiet: passed by exit status; 902 no_std tests
  passed
- cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json:
  passed by exit status
- sh -n for touched shell scripts: not run; no shell scripts touched
- jq empty on supervisor state and task-owned JSON evidence: passed
- /home/node/.cargo/bin/mdbook build: passed
- git diff --check: passed
- git diff --cached --check: passed before commit

## Redaction Review

No packet payloads, SSH key/session material, private data, raw hardware logs, or
external identifiers are retained. The task stores only source-path names,
classification strings, descriptor metadata field names, and validation labels.
