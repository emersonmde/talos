# Phase 12 RP1 Live Frame-Provider Source Core v83

Task: phase12-ssh-live-tcp-rp1-live-frame-provider-source-core-v83-20260702

Terminal classification: blocked-rp1-live-frame-provider-prerequisite-missing

Commit: recorded in talos-supervisor-state.json after final commit.

## Summary

This source task reviewed src/rp1_ethernet.rs, src/network.rs, the selected
rpi5_ssh_service_smoltcp_runtime_ready marker path, and the runtime-ready
archive review boundary after v80. The current source has an RP1 Ethernet
hardware frame-provider binding/report, but that report is explicitly
metadata-only: it does not own or program RP1 DMA descriptor rings, does not
produce RX frames, and cannot hand source-owned live ingress frames into
DriverPacketAdapter.

The task therefore fails closed instead of promoting deterministic host-local
frames or source metadata to live packet I/O. The next bounded work needs a
source-owned RP1 DMA RX descriptor/ring ownership path, with cache/DMA ownership
and provider polling/completion semantics, before packet stimulus or Pi 5 live
packet ingress proof can proceed.

## Findings

- not-an-issue: src/network.rs keeps deterministic DriverPacketAdapter traffic
  separate from live packet claims. Even when a source-bound RP1 provider report
  is present, the runtime classification remains blocked-no-live-frame-provider,
  with live packet I/O, reachability, remote receipt, compatibility, service
  success, ssh_ready, and phase transition all false.
- not-an-issue: src/rp1_ethernet.rs already records the source-bound provider
  contract as local/source metadata only. Its source evidence says there is no
  DMA descriptor ownership or packet I/O, and retained risks name missing RP1 DMA
  descriptor rings, completion/interrupt proof, and packet ingress.
- not-an-issue: src/target/rpi5.rs emits the missing owner token
  live-frame-provider-owner=missing-rp1-dma-rx-frame-provider in both
  runtime-ready and runtime-blocked markers, so v80/v79 retained marker evidence
  does not overclaim a live provider.
- deferred: A real RP1 live frame provider still needs a source-owned DMA RX
  descriptor/ring path, cache/DMA ownership policy, frame metadata handoff into
  DriverPacketAdapter, and fail-closed provider polling/completion states. That
  is outside the current source boundary and must be planned before packet
  stimulus or hardware packet-ingress proof.

## Source-Path Notes

- RP1 provider ownership currently stops at
  rp1_ethernet_hardware_frame_provider_binding_report: the report can classify
  missing provider, link-not-ready provider, or source-bound local-only provider.
- The local-only provider report is consumed by
  live_tcp_network_device_smoltcp_runtime_binding_with_rp1_provider, but the
  runtime path still performs driver_packet_smoltcp_listener_transfer using
  deterministic host-local frames.
- The accepted handoff into DriverPacketAdapter remains host-local descriptor
  delivery evidence. No source-owned RP1 RX frame bytes, frame metadata, DMA
  descriptor completion, or cache ownership state reaches the adapter.

## Acceptance

- selected_next_task: null
- planningNeeded: true
- first_missing_fact: source-owned RP1 DMA RX descriptor/ring ownership path,
  including cache/DMA ownership and provider polling/completion semantics, before
  the network DriverPacketAdapter can receive metadata-only live ingress frames.
- No archive was materialized.
- No lab publication, boot snapshot mutation, Pi 5 power action, serial/TFTP
  capture, live packet proof, remote receipt, OpenSSH/generated-root retry,
  compatibility claim, service success claim, ssh_ready=true, fake/kernel-backed
  command expansion, broad shell work, or phase transition was performed.

## Validation

- git status --short --branch before edits/action: ## main...origin/main [ahead 314]
- cargo fmt --all -- --check: passed
- cargo -Zjson-target-spec test --quiet: passed by exit status
- sh -n for touched shell scripts: not run; no shell scripts touched
- jq empty on supervisor state and task-owned JSON evidence: passed before commit;
  final post-commit supervisor-state check recorded in durable state
- /home/node/.cargo/bin/mdbook build: passed
- git diff --check: passed
- git diff --cached --check: passed before commit

## Redaction Review

No packet payloads, SSH key/session material, private data, raw hardware logs, or
external identifiers are retained. The record contains only source-path names,
classification strings, and validation labels.
