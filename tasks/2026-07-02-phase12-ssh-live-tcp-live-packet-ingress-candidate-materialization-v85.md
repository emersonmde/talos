# Phase 12 Live Packet Ingress Candidate Materialization v85

Task: phase12-ssh-live-tcp-live-packet-ingress-candidate-materialization-v85-20260702

Terminal classification: live-packet-ingress-provider-candidate-ready

Commit: recorded in talos-supervisor-state.json after final commit.

## Summary

This no-hardware task materializes a non-published Pi 5 candidate archive that
combines the accepted v88 source-owned RP1 DMA RX descriptor/ring metadata
handoff with the accepted v84 bounded packet-stimulus contract. The candidate is
only archive/static-review evidence. It was not published to the lab, did not
mutate a boot snapshot, did not power-cycle the Pi 5, and does not claim live
packet ingress, remote receipt, OpenSSH compatibility, service success,
ssh_ready, broad shell behavior, or a phase transition.

The selected candidate archive is
target/tmp/phase12-ssh-live-packet-ingress-v85-20260702.tar.gz with SHA-256
6c7c8a60197b2ae27ada837b72db3e6b21ea6d49e17f5cbad4952b57d7e8ef79 and size
309638 bytes. The selected kernel_2712.img has SHA-256
8ada0e2dc610236569358f5cac0367516aa6c88ae369f4eb8d843089ee896921 and size
160088 bytes. Capture nonce:
live-packet-ingress-v85-20260702. Restore target for the later serialized proof:
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## Findings

- fixed: src/target/rpi5.rs now routes the runtime-ready candidate through the
  source-owned RP1 DMA RX descriptor/ring handoff and the bounded packet
  stimulus contract before printing the selected runtime-ready marker.
- fixed: the runtime-ready marker now records the descriptor/ring owner,
  canonical DMA RX redaction policy, bounded packet-stimulus owner, contract id,
  classification, permitted source, nonce strategy, payload redaction policy,
  timing window, descriptor-handoff readiness, host-only discriminator, and false
  payload-retention claim.
- fixed: scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh now
  fails closed unless archive string inspection sees both the descriptor/ring
  provider tokens and bounded packet-stimulus tokens.
- not-an-issue: dynamic key/value writes in the boot-visible marker can appear
  as separate strings in archive inspection; the review script now requires both
  key and value tokens rather than assuming every field is a single contiguous
  literal.
- deferred: serialized Pi 5 publication, packet stimulus, serial/TFTP capture,
  packet-ingress observation, and restore proof remain v86 work.

## Candidate Contract

- archive:
  target/tmp/phase12-ssh-live-packet-ingress-v85-20260702.tar.gz
- archive SHA-256:
  6c7c8a60197b2ae27ada837b72db3e6b21ea6d49e17f5cbad4952b57d7e8ef79
- archive bytes: 309638
- selected kernel SHA-256:
  8ada0e2dc610236569358f5cac0367516aa6c88ae369f4eb8d843089ee896921
- selected kernel bytes: 160088
- capture nonce: live-packet-ingress-v85-20260702
- restore target:
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z
- expected runtime-ready/provider/stimulus/packet-ingress marker fields:
  capture-nonce, runtime-binding, hardware-frame-provider-bound,
  hardware-frame-provider-classification, live-frame-provider-owner,
  dma-rx-descriptor-ring-owner, dma-rx-redaction, packet-stimulus-owner,
  packet-stimulus-contract-id, packet-stimulus-classification,
  packet-stimulus-source, packet-stimulus-nonce-strategy,
  packet-stimulus-redaction, packet-stimulus-timing-window,
  packet-stimulus-descriptor-handoff-ready,
  packet-stimulus-host-only-discriminator,
  packet-stimulus-distinguishes-host-only, packet-stimulus-payload-retained,
  live-packet-ingress-discriminator, live-packet-io-accepted,
  live-reachability-accepted, remote-receipt-accepted,
  compatibility-accepted, ssh-ready, claims-service-success, and
  claims-phase-transition.
- fail-closed labels:
  remote-receipt-accepted=false, compatibility-accepted=false,
  ssh-ready=false, claims-service-success=false,
  claims-phase-transition=false, and live-packet-ingress-discriminator remains
  blocked-no-live-frame-provider until a later hardware proof observes source
  owned packet ingress.

## Acceptance

- selected_next_task:
  phase12-ssh-live-tcp-pi5-live-packet-ingress-provider-preflight-v86-20260702
- planningNeeded: false
- first_missing_fact: null
- terminal classification: live-packet-ingress-provider-candidate-ready
- No lab publication, boot snapshot mutation, Pi 5 power action, serial/TFTP
  capture, live packet proof, remote receipt, OpenSSH/generated-root retry,
  compatibility claim, service success claim, ssh_ready=true,
  fake/kernel-backed command expansion, broad shell work, or phase transition was
  performed.

## Validation

- git status --short --branch before edits/action:
  ## main...origin/main [ahead 317]
- cargo fmt --all: passed
- cargo fmt --all -- --check: passed
- cargo -Zjson-target-spec test --quiet bounded_packet_stimulus_contract:
  passed by exit status and reported 904 no_std tests passed.
- cargo -Zjson-target-spec build --target
  targets/aarch64-talos-rpi5-bcm2712.json: passed.
- non-published archive materialization: passed; archive and kernel identities
  are recorded above.
- scripts/rpi5-archive-review.sh on the accepted candidate archive: passed.
- scripts/rpi5-ssh-service-smoltcp-runtime-ready-archive-review.sh on the
  accepted candidate archive and nonce: passed after the review gate was updated
  for provider/stimulus tokens and dynamic key/value string inspection.
- sh -n for touched shell scripts: passed.
- jq empty on supervisor state and task-owned JSON evidence: run before commit.
- /home/node/.cargo/bin/mdbook build: run before commit.
- git diff --check: run before commit.
- git diff --cached --check: run before commit.

## Redaction Review

The accepted evidence keeps only archive/kernel hashes and byte counts, marker
field names, the non-secret capture nonce, descriptor metadata labels, static
contract tokens, and review transcripts. No packet payloads, SSH key/session
material, private data, raw hardware logs, remote receipt, or external packet
identifiers are retained.
