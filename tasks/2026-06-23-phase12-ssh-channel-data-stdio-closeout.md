# Phase 12.6 SSH channel-data stdio closeout

Task id: phase12-ssh-channel-data-stdio-closeout-20260623

Status: accepted

Classification: phase12-ssh-channel-data-stdio-closeout-accepted

## Goal

Close out the bounded local modeled SSH channel-data/stdio bridge slice by
reconciling its contract, source, feature evidence, docs, deferred risks, and
readiness counters.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-channel-data-stdio-contract.md.
- tasks/2026-06-23-phase12-ssh-channel-data-stdio-core.md.
- tasks/2026-06-23-phase12-ssh-channel-data-stdio-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-session-shell-attachment-closeout.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/ssh_service_readiness.rs.

## Reconciled Frontier

Talos has accepted only local modeled channel-data/stdio byte plumbing below
live encrypted transport. The accepted inbound path classifies one decrypted
SSH_MSG_CHANNEL_DATA payload as local stdin delivery only after accepted local
authentication success, one open session channel, one recognized shell request,
one local shell attachment, local process/session ownership, fd0/fd1/fd2 stdio
ownership, open lifecycle state, and bounded nonzero data are all present.

The accepted outbound path constructs local packet-shape reports only:

- stdout maps to SSH_MSG_CHANNEL_DATA with a public data length;
- stderr maps to SSH_MSG_CHANNEL_EXTENDED_DATA with SSH_EXTENDED_DATA_STDERR
  and a public data length.

These reports are not live encrypted socket writes, do not prove remote
receipt, and do not accept OpenSSH/POSIX/Linux compatibility.

## Readiness Counters

The reconciled success frontier may report these local-only counters:

- authentication-success=true;
- session-count=1;
- channel-count=1;
- shell-request-count=1;
- shell-attached=true;
- channel-data-stdio-local=true.

live-reachability=false, channel-window-management=false, and ssh-ready=false
remain authoritative.

## Deferred Scope

Live encrypted channel data delivery, socket delivery, channel-window
accounting and adjustment, EOF/close/exit-status behavior, PTY/job-control
semantics, hardware reachability, OpenSSH/POSIX/Linux compatibility, broad
command expansion, phase transition, and ssh-ready=true remain deferred to
later explicit supervisor-planned work. Fake/kernel-backed remote command
expansion remains rejected as progress.

## Findings

- fixed: reconciled the accepted contract, source, feature-smoke evidence,
  docs, validation, redaction boundary, and readiness counters for the local
  channel-data/stdio bridge.
- fixed: recorded that the accepted frontier is local modeled byte plumbing
  only; live encrypted socket delivery and remote reachability remain
  unaccepted.
- fixed: preserved channel-window-management=false and ssh-ready=false as
  authoritative readiness counters.
- deferred: channel-window accounting, live socket delivery, hardware
  reachability, EOF/close/exit-status behavior, compatibility, broad command
  expansion, phase transition, and ssh-ready=true require later explicit
  supervisor-planned tasks.
- not-an-issue: this closeout requires no new Rust source behavior and no Pi 5
  hardware proof.

## Evidence

- Reviewed inputs listed above: pass by static task/docs/source inspection.
- Existing focused feature evidence from
  phase12-ssh-channel-data-stdio-feature-smoke-20260623:
  cargo -Zjson-target-spec test channel_data_stdio --quiet: pass with the
  custom no_std harness running 799 tests.
- Existing full unit gate from the core and feature-smoke tasks:
  cargo -Zjson-target-spec test --quiet: pass with 799 no_std tests.
- Closeout validation:
  git diff --check: pass.
- Closeout docs validation:
  /home/node/.cargo/bin/mdbook build: pass.
- Commit preflight:
  git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not rerun in this closeout because
this task touched no Rust source or Cargo metadata. The listed unit-test
evidence is retained from the accepted core and feature-smoke tasks.

No live socket delivery, lab-controller API action, hardwareTestLock
acquisition, Pi 5 hardware run, boot publication, compatibility claim, broad
expansion, phase transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, fixed labels,
public SSH message names and numbers, public counters and booleans, public
length fields, validation commands, and classifications. It retains no private
user data, channel identifiers, request payload bytes, command payload bytes,
key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as the local modeled SSH channel-data/stdio closeout.

selected_next_task=null.

planningNeeded=true because no explicit queued/ready channel-window,
live-socket, hardware-reachability, or foundation follow-up task exists after
this accepted closeout for the worker to promote without supervisor planning.
