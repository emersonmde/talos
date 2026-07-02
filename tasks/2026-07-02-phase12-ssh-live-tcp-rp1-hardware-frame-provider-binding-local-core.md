# Phase 12 Live TCP RP1 Hardware Frame-Provider Binding Local Core

Task: phase12-ssh-live-tcp-rp1-hardware-frame-provider-binding-local-core-20260702
Status: accepted
Terminal classification: rp1-hardware-frame-provider-binding-local-core-accepted

## Summary

This task accepts the smallest source-local RP1 Ethernet hardware frame-provider
binding needed by the DriverPacketAdapter/smoltcp/listener/descriptor-delivery
lineage. It does not perform hardware action and does not accept live packet I/O
or SSH readiness.

## Source Files Changed

- src/rp1_ethernet.rs
- src/network.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

## Provider Boundary

src/rp1_ethernet.rs now exposes
Rp1EthernetHardwareFrameProviderContractEvidence and
Rp1EthernetHardwareFrameProviderBindingReport. The report distinguishes:

- no-rp1-ethernet-hardware-frame-provider-bound
- rp1-ethernet-hardware-frame-provider-link-not-ready-fail-closed
- rp1-ethernet-hardware-frame-provider-source-bound-local-only

src/network.rs consumes that report through
live_tcp_network_device_smoltcp_runtime_binding_with_rp1_provider. The
source-bound case may set hardware_frame_provider_bound=true as a local source
label only. live_packet_io_accepted, live_reachability_accepted,
remote_receipt_accepted, compatibility_accepted, service success, and ssh_ready
remain false.

## Findings

- fixed: Missing RP1 provider and source-bound provider were previously not
  distinguishable in the accepted runtime report path.
- fixed: Link-not-ready provider metadata now fails closed with an explicit
  classification before deterministic DriverPacketAdapter transfer evidence is
  composed.
- fixed: Missing descriptor delivery remains a separate fail-closed state even
  when RP1 provider metadata is present.
- deferred: v72 kernel_main, v60 route/runtime-ready, and v53 packet-I/O remain
  blocked or deferred until a later supervisor task explicitly reselects one as
  the next feature-required proof.
- not-an-issue: No live RP1 DMA, descriptor rings, interrupts, MDIO, PHY reset,
  remote receipt, OpenSSH compatibility, service success, or ssh_ready claim is
  required for this local source binding.

## Validation

- static inspection: source changes are bounded to src/rp1_ethernet.rs and
  src/network.rs plus task-owned docs/evidence.
- fmt/lint: cargo fmt --all -- --check passed.
- unit/substitute: cargo -Zjson-target-spec test --quiet passed with QEMU on
  PATH; 896 no_std tests passed.
- docs: mdbook build required because docs/src files changed.
- JSON: task-owned evidence JSON and supervisor state validated with jq empty.
- diff hygiene: git diff --check and git diff --cached --check required before
  commit.

## Disposition

selected_next_task: null
planningNeeded: true
first_missing_fact: A later supervisor task must select the next feature-led
proof after the source-bound provider label, likely a bounded hardware evidence
contract for live packet ingress if its gates are explicit.

Redaction review: retained evidence contains deterministic metadata and test
summaries only; no packet payload captures beyond fixture labels, secret
seed/key/session material, private user data, hardware log bytes, or unnecessary
identifiers are retained.
