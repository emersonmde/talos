# Phase 12.6 SSH peer-output receipt feature smoke

Task id: phase12-ssh-peer-output-receipt-feature-smoke-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-peer-output-receipt-feature-smoke-accepted

## Goal

Record focused feature-smoke and regression evidence for the bounded local
modeled SSH peer-output receipt behavior, without accepting live reachability,
remote-receipt=true, OpenSSH/POSIX/Linux compatibility, broad command
expansion, a phase transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-peer-output-receipt-contract.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-core.md.
- src/ssh_service_readiness.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Findings

- fixed: retained focused source/test evidence that the accepted local modeled
  success path queues SSH channel-data stdout/stderr, channel EOF, exit-status,
  and close output from the service side and observes it through the connected
  peer's descriptor-backed stream-socket receive path.
- fixed: retained fail-closed feature evidence for missing prerequisites,
  missing channel-window prerequisite, lifecycle violation, redaction-sensitive
  input, would-block peer receive, output backpressure, closed peer, malformed
  output, zero-length output, and over-limit output.
- fixed: retained full regression evidence for the accepted SSH readiness,
  descriptor, syscall, userspace socket ABI, and stream-socket model surfaces.
- not-an-issue: the focused command uses the no_std QEMU harness, which reports
  the full 819-test harness while preserving the named ssh_peer_output_receipt
  filter command as the task's focused gate.
- not-an-issue: no Rust source or Cargo metadata changed in this task, so the
  cargo fmt gate is a conditional skip.
- deferred: live reachability, external OpenSSH, live remote receipt,
  compatibility, PTY/SCP/SFTP, broad command expansion, phase transition, and
  ssh-ready=true.

## Feature Smoke Coverage

- `ssh_peer_output_receipt_observes_output_lifecycle_on_modeled_peer_socket`
  proves the local modeled peer observes the accepted stdout/stderr data, EOF,
  exit-status, and close output classes through the peer-side receive path.
- `ssh_peer_output_receipt_fails_closed_for_prerequisites_lifecycle_and_redaction`
  covers missing accepted prerequisites, missing channel-window prerequisite,
  invalid lifecycle state, and redaction-sensitive input.
- `ssh_peer_output_receipt_fails_closed_for_backpressure_closed_peer_and_would_block`
  covers would-block receive, service-side output backpressure, and closed peer
  behavior.
- `ssh_peer_output_receipt_fails_closed_for_malformed_and_over_limit_output`
  covers malformed output, zero-length output, and over-limit output
  observations.

## Validation

- cargo fmt --all -- --check: conditional skip, no Rust source, tests, or Cargo
  metadata touched.
- cargo -Zjson-target-spec test ssh_peer_output_receipt --quiet: pass; no_std
  QEMU harness reported 819 tests.
- cargo -Zjson-target-spec test --quiet: pass; no_std QEMU harness reported
  819 passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, source implementation, external OpenSSH execution, live
reachability claim, remote-receipt=true claim, compatibility claim, broad
command expansion, phase transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public socket
ABI names, public readiness bits, public SSH message names or numbers, public
request type names, public status values, public count and length categories,
readiness counters, validation commands, fixed labels, and classifications. It
retains no private user data, channel identifiers, request payload bytes,
command payload bytes, channel data bytes, key/session material, live peer
data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH peer-output receipt feature smoke and
regression evidence.

selected_next_task=phase12-ssh-peer-output-receipt-closeout-20260623.
