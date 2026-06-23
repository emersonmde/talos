# Phase 12.6 SSH live socket-delivery closeout

Task id: phase12-ssh-live-socket-delivery-closeout-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-live-socket-delivery-closeout-accepted-planning-needed

## Goal

Reconcile the accepted local modeled SSH live socket-delivery contract, source
behavior, feature smoke evidence, docs, redaction posture, readiness counters,
and deferred scope without accepting Pi 5 reachability, remote receipt,
OpenSSH/POSIX/Linux compatibility, a phase transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-live-socket-delivery-contract.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-core.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-feature-smoke.md.
- src/ssh_service_readiness.rs.
- src/network.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Closeout

Talos now accepts only bounded local modeled in-kernel stream socket delivery
into and out of the accepted SSH service pipeline. The accepted path is:

- a modeled AF_INET/SOCK_STREAM port-22 listener accepts one local connected
  peer;
- recv_peek reads bounded peer input from the accepted stream descriptor;
- SSH-owned classification dispatches the same bounded input through the
  previously accepted local authentication/session/channel, shell attachment,
  channel-data/stdio, channel-window, and channel-lifecycle surfaces;
- recv_commit consumes only classified input bytes;
- accepted stdout/stderr output is classified through the local SSH output
  surfaces;
- send_ready/send queues accepted output back to the peer stream socket under
  the same local model.

The reconciled counter frontier is socket-delivery-local=true only for that
local modeled descriptor path. Existing local counters may also remain true on
the same path when their accepted prerequisites are satisfied:
authentication-success, session-count=1, channel-count=1, shell-attached,
channel-data-stdio-local, channel-window-management, and
channel-lifecycle-local.

live-reachability=false, remote-receipt=false, compatibility=false, and
ssh-ready=false remain authoritative. The accepted evidence does not prove Pi 5
reachability, remote host receipt, OpenSSH client compatibility, POSIX/Linux
behavior, boot publication, hardware readiness, a phase transition, or
ssh-ready=true.

## Deferred Scope

Supervisor planning is required before the next feature slice. Objective
follow-up candidates include Pi 5 hardware reachability with the serialized
hardware lock and inconclusive-run triage, remote receipt contract, local
OpenSSH compatibility discriminator, or POSIX process EOF/wait foundation.
This worker did not choose among those directions because no explicit queued or
ready follow-up task exists after this closeout.

## Findings

- fixed: reconciled the accepted contract, source behavior, feature smoke
  tests, docs, validation, redaction, and readiness counter frontier.
- fixed: stated that Talos accepts only local modeled in-kernel stream socket
  delivery through the accepted SSH pipeline.
- fixed: preserved hard false live-reachability, remote-receipt,
  compatibility, and ssh-ready counters.
- not-an-issue: no Rust source changes were needed; the accepted core and
  feature-smoke tasks already supplied the source behavior and no_std
  descriptor-backed evidence.
- deferred: Pi 5 hardware reachability, remote receipt, OpenSSH/POSIX/Linux
  compatibility, POSIX process EOF/wait integration, boot publication, broad
  command expansion, phase transition, and ssh-ready=true.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this task
touched no Rust source or Cargo metadata.

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

Accepted as the bounded local modeled SSH live socket-delivery closeout.

selected_next_task=null; planningNeeded=true because no explicit queued or
ready follow-up task exists after this closeout for the worker to promote
without supervisor planning.
