# Phase 12.6 SSH OpenSSH compatibility discriminator closeout

Task id: phase12-ssh-openssh-compat-discriminator-closeout-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-openssh-compat-discriminator-closeout-accepted

## Goal

Reconcile the accepted OpenSSH compatibility discriminator contract, core
implementation, and feature-smoke evidence without adding source feature scope,
running OpenSSH, using Pi 5 hardware, publishing a boot archive, claiming live
reachability, remote receipt, compatibility=true, broad command expansion,
PTY/SCP/SFTP, a phase transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-contract.md.
- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-core.md.
- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-closeout.md.
- src/ssh_service_readiness.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- memory/talos-supervisor-state.json task
  phase12-ssh-openssh-compat-discriminator-closeout-20260623.

## Reconciled Frontier

The accepted OpenSSH compatibility discriminator frontier remains local/offline
only. The discriminator consumes the already accepted modeled SSH foundations:
one descriptor-backed in-kernel stream-socket peer, accepted local SSH service
delivery, authentication/session/channel, shell attachment, channel-data/stdio,
channel-window, channel-lifecycle, POSIX EOF/wait, and peer-output receipt.

The accepted success transcript shape is limited to sanitized public message
categories in this order:

1. optional SSH_MSG_CHANNEL_DATA stdout categories;
2. optional SSH_MSG_CHANNEL_EXTENDED_DATA stderr categories;
3. SSH_MSG_CHANNEL_EOF;
4. SSH_MSG_CHANNEL_REQUEST exit-status with request type exit-status and
   want_reply=false;
5. SSH_MSG_CHANNEL_CLOSE.

The accepted readiness label is
openssh-compat-discriminator-local=true only for that local/offline modeled
shape. live-reachability=false, remote-receipt=false, compatibility=false, and
ssh-ready=false remain authoritative. External OpenSSH execution, Pi 5
reachability, live remote receipt, OpenSSH/POSIX/Linux compatibility,
PTY/SCP/SFTP, broad command expansion, phase transition, and ssh-ready=true
remain deferred.

## Deferred Live Compatibility Risks

- The local/offline discriminator does not prove a live TCP path, Pi 5 Ethernet
  reachability, host OpenSSH client behavior against Talos, remote receipt, or
  interoperability.
- The discriminator does not retain or compare host keys, authorized keys,
  signatures, session identifiers, channel identifiers, payload bytes, user
  names, fingerprints, live peer identifiers, hardware serial data, or boot
  artifacts.
- A future live-client task must separately define prerequisites, redaction
  rules, public observations, failure labels, hardwareTestLock ownership if Pi
  5 lab evidence is required, and inconclusive-run triage before running any
  external client or claiming compatibility.

## Findings And Disposition

- fixed: reconciled the accepted contract, core source behavior, focused
  feature-smoke evidence, docs frontier, redaction boundaries, and deferred live
  compatibility risks.
- fixed: recorded that openssh-compat-discriminator-local=true is only a
  local/offline modeled discriminator label, not a live reachability, remote
  receipt, OpenSSH/POSIX/Linux compatibility, phase transition, or
  ssh-ready=true claim.
- fixed: preserved representative fail-closed coverage for missing
  prerequisites, missing modeled peer receipt, lifecycle/order violations,
  duplicate terminal messages, unsupported request shape, malformed/over-limit
  shape, and redaction-sensitive input.
- fixed: selected
  phase12-ssh-live-openssh-client-contract-20260623 as the next bounded task
  because the accepted closeout leaves a live-client contract as the explicit
  queued follow-up and no blocker remains.
- not-an-issue: no source change was required because the accepted core and
  feature-smoke tasks already implemented and exercised the discriminator.
- deferred: live OpenSSH client execution, Pi 5 hardware proof, live TCP
  reachability, remote-receipt=true, compatibility=true, PTY/SCP/SFTP, broad
  command expansion, phase transition, and ssh-ready=true remain outside this
  closeout.

## Validation

- static task/docs/source review: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, or Cargo
  metadata touched.
- cargo -Zjson-target-spec test ssh_openssh_compat_discriminator --quiet:
  conditional skip, no Rust source, tests, or Cargo metadata touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: static source/task/docs review, diff checks, and docs build.
No hardware/lab evidence, external OpenSSH execution, boot publication, live
reachability, remote receipt, compatibility=true, broad command expansion,
phase transition, or ssh-ready=true was claimed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public
readiness labels, public SSH message names/numbers, public request type names,
public status values/categories, public length/count categories, validation
commands, fixed labels, and classifications. It retains no private user data,
channel identifiers, request payload bytes, command payload bytes, channel data
bytes, key/session material, user names, fingerprints, signatures, session
identifiers, live peer data, hardware data, or boot artifacts.

## Acceptance

Accepted as
phase12-ssh-openssh-compat-discriminator-closeout-accepted. The next selected
task is phase12-ssh-live-openssh-client-contract-20260623.
