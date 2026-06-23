# Phase 12.6 SSH live socket-delivery feature smoke

Task id: phase12-ssh-live-socket-delivery-feature-smoke-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-live-socket-delivery-feature-smoke-accepted

## Goal

Retain bounded local feature smoke and regression evidence for the accepted SSH
socket-delivery source slice without accepting Pi 5 reachability, remote
receipt, OpenSSH/POSIX/Linux compatibility, a phase transition, or
ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-live-socket-delivery-contract.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-core.md.
- src/ssh_service_readiness.rs.
- src/network.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Feature Smoke Evidence

- socket_delivery_local_model_delivers_input_and_output_through_stream_socket
  proves that a modeled AF_INET/SOCK_STREAM port-22 listener accepts one local
  peer, receives CHANNEL_DATA bytes through recv_peek/recv_commit into the
  accepted SSH service pipeline, and queues accepted stdout output back through
  the stream socket.
- socket_delivery_fails_closed_for_missing_listener_connection_and_prerequisite
  covers missing listener, missing connection, and missing prerequisite
  controls.
- socket_delivery_fails_closed_for_would_block_backpressure_and_closed_peer
  covers would-block, output backpressure, and closed-peer controls.
- socket_delivery_fails_closed_for_malformed_over_limit_lifecycle_and_redaction
  covers malformed input, over-limit input, lifecycle-invalid input, and
  redaction-sensitive controls.
- Regression surfaces from prior accepted SSH slices remain covered by the same
  no_std test run: authentication/session/channel counters, shell attachment,
  channel-data/stdio, channel-window management, channel-lifecycle, and hard
  false live_reachability, remote_receipt, compatibility, and ssh_ready.

## Findings

- fixed: retained feature smoke evidence through the accepted modeled stream
  socket descriptor path rather than a helper-only protocol classifier path.
- fixed: retained negative evidence for missing listener/connection,
  missing prerequisites, would-block, backpressure, closed peer, malformed or
  over-limit input, lifecycle-invalid input, and redaction-sensitive input.
- fixed: preserved accepted local-only SSH counters while keeping
  live-reachability=false, remote-receipt=false, compatibility=false, and
  ssh-ready=false authoritative.
- not-an-issue: no new Rust source was required for this task because the
  accepted core task already added the descriptor-backed socket-delivery tests
  needed for feature smoke evidence.
- deferred: Pi 5 hardware/lab action, boot publication, live reachability,
  remote receipt, OpenSSH/POSIX/Linux compatibility, POSIX process wait/exit,
  broad command expansion, phase transition, and ssh-ready=true.

## Validation

- static task/docs/source review: pass.
- cargo -Zjson-target-spec test socket_delivery --quiet: pass, 812 no_std
  tests.
- cargo -Zjson-target-spec test --quiet: pass, 812 no_std tests.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Rust source or Cargo metadata was touched for this feature-smoke task, so the
cargo fmt gate was conditional and not run. No Pi 5 hardware run,
lab-controller API action, hardwareTestLock acquisition, boot publication, live
reachability claim, remote receipt claim, OpenSSH/POSIX compatibility claim,
broad command expansion, phase transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public socket
ABI names, public readiness bits, public SSH message names, public count and
length categories, fixed labels, validation commands, readiness counters, and
classifications. It retains no private user data, channel identifiers, request
payload bytes, command payload bytes, channel data bytes, key/session material,
live peer data, hardware data, or boot artifacts.

## Result

Accepted as bounded local modeled SSH live socket-delivery feature smoke and
regression evidence.

selected_next_task=phase12-ssh-live-socket-delivery-closeout-20260623.
