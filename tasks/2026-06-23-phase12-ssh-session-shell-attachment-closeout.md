# Phase 12.6 SSH session shell attachment closeout

Task id: phase12-ssh-session-shell-attachment-closeout-20260623

Status: accepted

Classification: phase12-ssh-session-shell-attachment-closeout-accepted

## Goal

Close the bounded local modeled SSH shell attachment slice by reconciling the
contract, implementation, feature smoke evidence, docs, redaction boundaries,
readiness counters, and deferred scope.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-session-shell-attachment-contract.md.
- tasks/2026-06-23-phase12-ssh-session-shell-attachment-core.md.
- tasks/2026-06-23-phase12-ssh-session-shell-attachment-feature-smoke.md.
- src/ssh_service_readiness.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Closeout Decision

Talos has accepted local modeled SSH shell attachment only. The accepted
frontier covers one authenticated local modeled SSH session channel with one
recognized shell request attached to accepted local process/session and
fd0/fd1/fd2 stdio ownership. The accepted success path may record
SSH_MSG_CHANNEL_SUCCESS for want-reply=true or no response for want-reply=false,
and it may report authentication-success=true, session-count=1,
channel-count=1, shell-request-count=1, and shell-attached=true only for that
local modeled attachment path.

This closeout does not accept live SSH reachability, encrypted channel data
delivery, socket delivery, channel window management, OpenSSH/POSIX/Linux
compatibility, hardware reachability, a phase transition, or ssh-ready=true.
live-reachability=false and ssh-ready=false remain authoritative.

## Reconciled Evidence

- Contract: phase12-ssh-session-shell-attachment-contract-20260623 defined the
  bounded CHANNEL_SUCCESS eligibility, attachment ownership, failure behavior,
  redaction boundary, and readiness counter semantics.
- Implementation: phase12-ssh-session-shell-attachment-core-20260623 added the
  local modeled attachment classifier and fixed fail-closed controls without
  broadening into fake/kernel-backed remote shell behavior.
- Feature smoke: phase12-ssh-session-shell-attachment-feature-smoke-20260623
  retained no_std coverage for the success path, no-reply attachment,
  ownership/lifecycle failures, missing prerequisites, duplicates,
  malformed/trailing data, redaction-sensitive input, and the prior
  shell-request no-attachment controls.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md now
  record the local modeled attachment frontier and deferred live/foundation
  scope.

## Findings

- fixed: reconciled the shell attachment contract, source implementation,
  feature smoke evidence, docs, redaction review, and readiness counters into a
  single accepted local modeled frontier.
- fixed: preserved the distinction between local modeled attachment and live
  SSH reachability; live-reachability=false and ssh-ready=false remain
  authoritative.
- fixed: kept fake/kernel-backed remote shell command expansion rejected as
  progress.
- fixed: recorded that the next step requires supervisor planning because no
  explicit queued/ready live-reachability or foundation follow-up task exists
  for the worker to promote without inventing scope.
- deferred: live encrypted channel data delivery, socket delivery, channel
  window management, EOF/close/window-flow behavior beyond the modeled local
  lifecycle controls, hardware proof, OpenSSH/POSIX/Linux compatibility,
  broad expansion, phase transition, and ssh-ready=true.
- not-an-issue: no Pi 5 hardware run is required because this closeout accepts
  only local modeled source/docs/unit-test evidence and publishes no boot
  artifact.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this closeout
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, live reachability claim, compatibility claim, broad
expansion, phase transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public SSH
message names and numbers, fixed test/function names, public counters/booleans,
validation commands, and classifications. It retains no private user data,
channel identifiers, request payload bytes, key/session material, hardware
data, or boot artifacts.

## Result

Accepted as the local modeled SSH shell attachment closeout.

selected_next_task=null. planningNeeded=true because no explicit queued/ready
live-reachability or foundation follow-up task exists after this closeout; the
supervisor must plan exactly one bounded next task before further worker
execution.
