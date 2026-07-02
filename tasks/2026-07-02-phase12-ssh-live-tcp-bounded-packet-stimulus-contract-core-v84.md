# Phase 12 Bounded Packet Stimulus Contract Core v84

Task: phase12-ssh-live-tcp-bounded-packet-stimulus-contract-core-v84-20260702

Terminal classification: bounded-packet-stimulus-contract-ready

Commit: recorded in talos-supervisor-state.json after final commit.

## Summary

This no-hardware task defines the bounded packet stimulus contract required
after the accepted v88 RP1 DMA RX descriptor/ring source boundary and before a
serialized Pi 5 live packet-ingress proof. The contract is metadata-only: it
permits a lab network peer ICMP echo request to the documented Talos Pi 5 target,
requires a run-unique ASCII nonce, retains only nonce SHA-256 plus length and RX
descriptor metadata, and explicitly separates that lab stimulus from deterministic
DriverPacketAdapter host-only frames.

The implementation lives in src/network.rs as a
LivePacketStimulusContractReport. It is ready only when the source-owned RP1 DMA
RX descriptor/ring metadata handoff is ready and no payload bytes, remote
receipt, OpenSSH compatibility, service success, ssh_ready, or phase transition
claim is accepted.

## Findings

- fixed: src/network.rs now records the permitted stimulus source,
  run-unique nonce strategy, payload redaction policy, timing window, expected
  marker/report fields, and distinguishing rules for a later serialized Pi 5
  packet-ingress proof.
- fixed: The report fails closed when the RP1 descriptor/ring metadata handoff is
  absent, so packet stimulus cannot bypass the v88 source-owned RX descriptor
  prerequisite.
- fixed: The contract names deterministic DriverPacketAdapter host-only delivery
  as a regression/control surface that cannot satisfy lab stimulus evidence.
- not-an-issue: The lab controller docs do not expose a packet-injection API.
  The contract avoids network-controller mutation and credentials by naming only
  a bounded lab-network ICMP echo stimulus for the later hardware proof.
- deferred: The candidate archive/materialization and serialized Pi 5 packet
  stimulus execution remain v85/v86 work.

## Contract

- permitted stimulus source:
  lab-network-peer-icmp-echo-to-documented-talos-pi5-target
- nonce strategy:
  run-unique-ascii-nonce-in-icmp-echo-payload-retain-only-sha256-and-length
- payload redaction:
  retain-protocol-length-nonce-sha256-and-descriptor-metadata-no-payload-bytes
- timing window:
  after-runtime-ready-marker-and-serial-cursor-before-final-pre-restore-identity
- expected report fields:
  contract-id, permitted-stimulus-source, nonce-sha256, nonce-length,
  stimulus-protocol, descriptor-index, frame-length, ring-wrap,
  rp1-descriptor-ring-classification, host-only-frame-count,
  live-packet-io-accepted
- distinguishing rules:
  lab stimulus must come from the lab network peer, the nonce hash must be absent
  from deterministic host-only frames, RP1 descriptor metadata must come from the
  source-owned RX descriptor/ring handoff, payload bytes are never retained, and
  deterministic host-only DriverPacketAdapter delivery remains a control only.

## Acceptance

- selected_next_task:
  phase12-ssh-live-tcp-live-packet-ingress-candidate-materialization-v85-20260702
- planningNeeded: false
- first_missing_fact: null
- terminal classification: bounded-packet-stimulus-contract-ready
- No lab publication, boot snapshot mutation, Pi 5 power action, serial/TFTP
  capture, live packet proof, remote receipt, OpenSSH/generated-root retry,
  compatibility claim, service success claim, ssh_ready=true,
  fake/kernel-backed command expansion, broad shell work, or phase transition was
  performed.

## Validation

- git status --short --branch before edits/action:
  ## main...origin/main [ahead 316]
- cargo fmt --all: passed
- cargo fmt --all -- --check: passed
- cargo -Zjson-target-spec test --quiet bounded_packet_stimulus_contract:
  first attempt failed because QEMU_SYSTEM_AARCH64/PATH was not set; rerun with
  the documented QEMU PATH passed by exit status and reported 904 no_std tests
  passed.
- sh -n for touched shell scripts: not run; no shell scripts touched
- jq empty on supervisor state and task-owned JSON evidence: passed
- /home/node/.cargo/bin/mdbook build: passed
- git diff --check: passed
- git diff --cached --check: run before commit

## Redaction Review

No packet payloads, SSH key/session material, private data, raw hardware logs, or
external identifiers are retained. The contract retains only metadata field
names, nonce SHA-256/length requirements, descriptor metadata names, and
validation labels.
