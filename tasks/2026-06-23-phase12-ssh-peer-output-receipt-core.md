# Phase 12.6 SSH peer-output receipt core

Task id: phase12-ssh-peer-output-receipt-core-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-peer-output-receipt-core-accepted

## Goal

Implement the bounded local modeled peer-output receipt behavior defined by the
accepted contract, without accepting live reachability, remote-receipt=true,
OpenSSH/POSIX/Linux compatibility, broad command expansion, a phase transition,
or ssh-ready=true.

## Scope

- Modified only \`src/ssh_service_readiness.rs\` for source behavior and focused
  no_std tests.
- Added \`SshPeerOutputReceiptReport\` and
  \`classify_ssh_peer_output_receipt\`.
- Modeled the same descriptor-backed AF_INET stream-socket listener,
  connected peer, and accepted service-side connection used by the accepted
  local SSH socket-delivery foundation.
- Queued accepted SSH output classes from the service side and observed them
  through the peer-side stream-socket receive path:
  channel-data stdout/stderr, SSH_MSG_CHANNEL_EOF,
  SSH_MSG_CHANNEL_REQUEST exit-status with request type exit-status and
  want_reply=false, and SSH_MSG_CHANNEL_CLOSE.

## Findings

- fixed: added fixed peer-output receipt labels and a local
  peer-output-receipt-local readiness counter that remains separate from
  remote-receipt=true.
- fixed: implemented the accepted local modeled success path through
  send/recv_peek/recv_commit on the descriptor-backed stream-socket model.
- fixed: retained only public message classes, status values, length/count
  categories, and readiness counters; channel data bytes are generated and
  observed only in-memory and are not retained in durable evidence.
- fixed: added fail-closed behavior for missing listener/connection and SSH
  prerequisites, missing socket-delivery/POSIX EOF-wait prerequisites, missing
  channel-window prerequisite, would-block peer receive, output backpressure,
  closed peer, malformed/zero-length output, over-limit output, lifecycle
  violation, and redaction-sensitive input.
- not-an-issue: the implementation intentionally keeps live-reachability=false,
  remote-receipt=false, compatibility=false, and ssh-ready=false.
- deferred: feature-smoke task record, live reachability, external OpenSSH,
  remote receipt, compatibility, PTY/SCP/SFTP, broad command expansion, phase
  transition, and ssh-ready=true.

## Changed Files

- \`src/ssh_service_readiness.rs\`
- \`tasks/2026-06-23-phase12-ssh-peer-output-receipt-core.md\`
- \`docs/src/project/phase12-networking-ssh.md\`
- \`docs/src/roadmap.md\`

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test ssh_peer_output_receipt --quiet: pass.
- cargo -Zjson-target-spec test --quiet: pass with documented Talos QEMU PATH;
  no_std QEMU harness ran 819 tests.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; warning only for existing large
  search index.
- git diff --cached --check: pass.

An earlier full test invocation without the documented QEMU PATH failed before
the test harness with \`qemu-system-aarch64 not found\`; the documented Talos
environment rerun passed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public socket
ABI names, public readiness bits, public SSH message names/numbers, public
request type names, public status values, public count/length categories,
readiness counters, validation commands, fixed labels, and classifications. It
retains no private user data, channel identifiers, request payload bytes,
command payload bytes, channel data bytes, key/session material, live peer
data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH peer-output receipt source
implementation.

selected_next_task=phase12-ssh-peer-output-receipt-feature-smoke-20260623.
