# Phase 12.6 SSH POSIX EOF/wait feature smoke

Task id: phase12-ssh-posix-eof-wait-feature-smoke-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-posix-eof-wait-feature-smoke-accepted

## Goal

Record feature-smoke evidence for the accepted local modeled SSH POSIX
EOF/wait integration without accepting live reachability, remote receipt,
OpenSSH/POSIX/Linux compatibility, broad command expansion, a phase
transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-posix-eof-wait-contract.md.
- tasks/2026-06-23-phase12-ssh-posix-eof-wait-core.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-feature-smoke.md.
- src/ssh_service_readiness.rs.
- src/local_command_loop.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Smoke Surface

The accepted smoke evidence is local modeled only:

- Success path: the
  posix_eof_wait_integration_models_stdin_eof_wait_exit_status_and_close
  unit smoke composes accepted local socket delivery, authentication,
  session/channel, shell attachment, channel-data/stdio, channel-window,
  channel-lifecycle, inbound SSH_MSG_CHANNEL_EOF, one completed wait/status
  record, one local exit-status request, local stdout EOF, local close, and
  peer close.
- Negative controls:
  posix_eof_wait_integration_fails_closed_for_missing_and_consumed_wait_records
  covers missing and already-consumed process status, and
  posix_eof_wait_integration_fails_closed_for_prerequisites_lifecycle_and_output
  covers missing prerequisites, duplicate or invalid lifecycle ordering,
  output backpressure, closed peer, and redaction-sensitive inputs.
- Regression surface: cargo -Zjson-target-spec test --quiet re-runs the
  accepted local modeled SSH socket-delivery, channel-data/stdio,
  channel-window, channel-lifecycle, POSIX process status, userspace socket
  ABI, descriptor, and syscall regression surfaces.

This evidence accepts posix-eof-wait-local=true only for the modeled path.
live-reachability=false, remote-receipt=false, compatibility=false, and
ssh-ready=false remain authoritative.

## Findings

- fixed: retained a named feature-smoke record that ties the accepted
  success-path and negative-control tests to the Phase 12.6 EOF/wait
  contract.
- fixed: confirmed the focused smoke filter exercises the accepted
  EOF/wait success path and required fail-closed controls.
- fixed: confirmed the full cargo regression still covers the accepted
  socket-delivery and channel lifecycle surfaces alongside the EOF/wait tests.
- fixed: updated Phase 12 docs and roadmap to record the feature-smoke
  acceptance frontier and selected closeout task.
- not-an-issue: no Rust source, tests, Cargo metadata, hardware run, boot
  publication, live reachability proof, remote receipt proof, OpenSSH
  compatibility discriminator, broad command expansion, phase transition, or
  ssh-ready=true is required for this local modeled feature-smoke task.
- deferred: closeout reconciliation, Pi 5 reachability, remote receipt,
  OpenSSH/POSIX/Linux compatibility, multiple sessions, multiple children,
  blocking wait, scheduler-owned process lifetime, broad shell expansion,
  phase transition, and ssh-ready=true.

## Validation

- cargo fmt --all -- --check: conditional skip, no Rust source, tests, or
  Cargo metadata touched.
- cargo -Zjson-target-spec test ssh_posix_eof_wait --quiet: pass; qemu
  no_std harness ran 815 tests because the harness does not narrow execution by
  the substring filter.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, live reachability claim, remote receipt claim,
OpenSSH/POSIX/Linux compatibility claim, broad command expansion, phase
transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public SSH
message names and numbers, public request type names, public status values,
public length categories, boolean readiness counters, fixed labels,
validation commands, and classifications. It retains no private user data,
channel identifiers, request payload bytes, command payload bytes, channel data
bytes, key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH POSIX EOF/wait feature smoke.

selected_next_task=phase12-ssh-posix-eof-wait-closeout-20260623.
