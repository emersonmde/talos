# Phase 12.6 SSH channel lifecycle EOF/exit-status closeout

Task id: phase12-ssh-channel-lifecycle-exit-status-closeout-20260623

Status: accepted

Classification: phase12-ssh-channel-lifecycle-exit-status-closeout-accepted

## Goal

Close out the accepted local modeled SSH channel lifecycle EOF, exit-status,
and close slice by reconciling the contract, implementation, feature smoke
evidence, validation, docs, redaction boundary, deferred scope, and readiness
counters.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-core.md.
- tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-channel-window-accounting-closeout.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/ssh_service_readiness.rs.

## Reconciled Frontier

Talos accepts only local modeled SSH shell lifecycle behavior for the single
accepted authenticated session channel, shell attachment, local process/session
ownership, fd0/fd1/fd2 stdio ownership, channel-data/stdio bridge, and
channel-window accounting path.

The accepted lifecycle layer records:

- inbound SSH_MSG_CHANNEL_EOF as local EOF receipt while the channel is open;
- local SSH_MSG_CHANNEL_REQUEST exit-status emission with request type
  exit-status, want_reply=false, and a public u32 exit status supplied by the
  caller;
- local SSH_MSG_CHANNEL_CLOSE send and receive state, with the modeled channel
  considered closed only after both directions have closed.

Malformed, unsupported, duplicate, missing-prerequisite, lifecycle-invalid,
local-execution-missing, over-limit, and redaction-sensitive paths fail closed
without changing the acceptance boundary. Later lifecycle or channel-data
operations after both close directions are recorded fail closed.

The accepted counters remain local modeled only: channel-data-stdio-local=true,
channel-window-management=true, and channel-lifecycle-local=true only on the
accepted local success paths. live-reachability=false, remote-receipt=false,
compatibility=false, and ssh-ready=false remain authoritative.

## Findings

- not-an-issue: the source implementation and feature smoke task agree on the
  local modeled lifecycle boundary and do not claim live delivery, remote
  receipt, process wait/exit, OpenSSH compatibility, or ssh-ready=true.
- not-an-issue: readiness surfaces preserve ssh_ready() == false and
  reachability_accepted() == false for lifecycle reports.
- fixed: added closeout task documentation that reconciles the accepted
  contract, source, feature evidence, validation, docs, readiness counters, and
  deferred scope.
- fixed: updated the Phase 12 networking/SSH architecture doc and roadmap with
  the accepted lifecycle closeout frontier.
- deferred: live encrypted socket delivery, hardware reachability,
  OpenSSH/POSIX/Linux compatibility, POSIX process wait/exit, EOF-driven
  userspace/process integration, broad shell command expansion, phase
  transition, and ssh-ready=true remain for later explicit tasks.

## Evidence

- Static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.
- Rust source and Cargo metadata were not touched, so cargo fmt and cargo test
  gates were not required for this closeout.

## Redaction Review

Pass. Retained closeout evidence is limited to task ids, source/doc paths,
public SSH message names and numbers, public request type names, public booleans,
fixed readiness counter names, validation commands, and classifications. It
retains no private user data, channel identifiers, request payload bytes,
command payload bytes, channel data payload bytes, key/session material, live
peer data, hardware data, or boot artifacts.

## Result

Accepted as the closeout for the local modeled SSH channel lifecycle
EOF/exit-status/close layer.

selected_next_task=null.

planningNeeded=true because no explicit queued or ready follow-up task exists
for the worker to promote without supervisor planning.
