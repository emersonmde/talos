# Phase 12.6 SSH live socket-delivery core

Task id: phase12-ssh-live-socket-delivery-core-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-live-socket-delivery-core-accepted

## Goal

Implement the accepted local modeled SSH socket-delivery contract in source
without accepting Pi 5 reachability, remote receipt, OpenSSH/POSIX/Linux
compatibility, a phase transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-live-socket-delivery-contract.md.
- src/ssh_service_readiness.rs.
- src/network.rs.
- src/userspace_socket_abi.rs.
- src/syscall.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Source Diff Summary

- Added fixed socket-delivery labels to the SSH service readiness vocabulary:
  local delivery, input dispatched, output queued, and fail-closed labels for
  missing listener, missing connection, prerequisite missing, would-block,
  backpressure, closed peer, malformed input, over-limit input, lifecycle
  violation, and redaction-sensitive input.
- Added SshSocketDeliveryInput, SshSocketDeliveryPeerInput,
  SshSocketDeliveryResult, SshSocketDeliveryReport, and
  classify_ssh_socket_delivery in src/ssh_service_readiness.rs.
- The implementation creates a modeled AF_INET/SOCK_STREAM port-22 listener in
  NetworkSocketDescriptorTable, accepts one local connected peer, receives
  bounded peer bytes through recv_peek, dispatches them through the accepted
  channel-data/stdio classifier, commits only classified bytes with
  recv_commit, classifies bounded stdout/stderr output, checks send readiness,
  and sends output through the accepted descriptor.
- The report exposes local readiness bits, received/committed/sent lengths,
  accepted local SSH counters, socket-delivery-local, and hard false values for
  live_reachability, remote_receipt, compatibility, and ssh_ready.

## Findings

- fixed: implemented the accepted local in-kernel stream socket path from
  modeled listener accept through recv_peek/recv_commit, SSH input dispatch,
  output classification, send readiness, and send.
- fixed: mapped missing listener/connection, missing prerequisite, would-block,
  output backpressure, closed peer, malformed input, over-limit input,
  lifecycle-invalid input, and redaction-sensitive input to fixed fail-closed
  socket-delivery labels.
- fixed: added unit coverage for successful local socket input/output delivery
  and the required negative controls.
- fixed: preserved accepted local authentication/session/channel/shell,
  channel-data/stdio, channel-window, and channel-lifecycle counters only on
  the local modeled success path.
- not-an-issue: no Rust changes were needed in network.rs, syscall.rs, or
  userspace_socket_abi.rs because the accepted socket table already provides
  the needed local accept/recv/send/readiness semantics.
- deferred: feature-smoke evidence through broader local modeled descriptor
  flows, Pi 5 reachability, remote receipt, OpenSSH compatibility, POSIX
  process wait/exit integration, boot publication, phase transition, and
  ssh-ready=true.

## Validation

- static task/docs/source review: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass, 812 no_std tests.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, live reachability claim, remote receipt claim, OpenSSH/POSIX
compatibility claim, broad command expansion, phase transition, or
ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public socket
ABI names, public readiness bits, public SSH message names, public count and
length categories, fixed labels, validation commands, readiness counters, and
classifications. It retains no private user data, channel identifiers, request
payload bytes, command payload bytes, channel data bytes, key/session material,
live peer data, hardware data, or boot artifacts.

## Result

Accepted as bounded local modeled SSH live socket-delivery source behavior.

selected_next_task=phase12-ssh-live-socket-delivery-feature-smoke-20260623.
