# Phase 12.6 SSH OpenSSH compatibility discriminator core

Task id: phase12-ssh-openssh-compat-discriminator-core-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-openssh-compat-discriminator-core-accepted

## Goal

Implement the local/offline OpenSSH-compatible closeout transcript discriminator
defined by the accepted contract, without running OpenSSH, using Pi 5 hardware,
publishing a boot archive, claiming live reachability, remote receipt,
compatibility=true, broad command expansion, phase transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-contract.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-closeout.md.
- src/ssh_service_readiness.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- memory/talos-supervisor-state.json task
  phase12-ssh-openssh-compat-discriminator-core-20260623.

## Implementation Summary

src/ssh_service_readiness.rs now exposes a local/offline discriminator over a
sanitized public transcript shape:

1. optional stdout SSH_MSG_CHANNEL_DATA categories;
2. optional stderr SSH_MSG_CHANNEL_EXTENDED_DATA categories;
3. SSH_MSG_CHANNEL_EOF;
4. SSH_MSG_CHANNEL_REQUEST exit-status with request type exit-status and
   want_reply=false;
5. SSH_MSG_CHANNEL_CLOSE.

The accepted label is only
openssh-compat-discriminator-local=true through the readiness report method.
live-reachability=false, remote-receipt=false, compatibility=false, and
ssh-ready=false remain hard-coded false.

The report retains only public message/order/status categories, public
length/count categories, readiness/frontier labels, and fixed classifications.
It does not retain channel identifiers, payload bytes, key/session material,
user names, fingerprints, live peer data, hardware data, boot artifacts, or
private user data.

## Findings And Disposition

- fixed: added SshOpenSshCompatTranscriptEvent and
  classify_ssh_openssh_compat_discriminator for the local/offline
  transcript-shaped closeout discriminator.
- fixed: success evidence requires accepted local socket delivery,
  authentication/session/channel, shell, channel-data/stdio, channel-window,
  channel-lifecycle, POSIX EOF/wait, and peer-output receipt prerequisites.
- fixed: fail-closed controls reject missing prerequisites, missing modeled
  peer receipt, data after EOF, close before exit-status, duplicate EOF or
  terminal messages, unsupported request shape, exit-status want_reply=true,
  malformed/zero-length data shape, over-limit data/events, and
  redaction-sensitive input.
- fixed: focused tests cover the accepted success transcript with stdout,
  stderr, EOF, exit-status want_reply=false, close, and the required
  fail-closed controls.
- deferred: live OpenSSH client execution, Pi 5 hardware proof, live TCP
  reachability, remote-receipt=true, compatibility=true, PTY/SCP/SFTP, broad
  command expansion, phase transition, and ssh-ready=true remain outside this
  task and require later queued tasks.

## Source And Test Summary

New source/test surfaces:

- SshOpenSshCompatTranscriptEvent.
- SshOpenSshCompatDiscriminatorInput.
- SshOpenSshCompatDiscriminatorResult.
- SshOpenSshCompatDiscriminatorReport.
- classify_ssh_openssh_compat_discriminator.
- ssh_openssh_compat_discriminator_accepts_local_offline_closeout_transcript.
- ssh_openssh_compat_discriminator_fails_closed_for_missing_prerequisites.
- ssh_openssh_compat_discriminator_fails_closed_for_ordering_and_duplicates.
- ssh_openssh_compat_discriminator_fails_closed_for_shape_and_limits.

Public labels/counters only:

- sshservicediag-openssh-compat-discriminator-prerequisite-only.
- sshservicediag-openssh-compat-discriminator-stdout-data-accepted.
- sshservicediag-openssh-compat-discriminator-stderr-extended-data-accepted.
- sshservicediag-openssh-compat-discriminator-eof-accepted.
- sshservicediag-openssh-compat-discriminator-exit-status-accepted.
- sshservicediag-openssh-compat-discriminator-close-accepted.
- sshservicediag-openssh-compat-discriminator-local.
- sshservicediag-openssh-compat-discriminator-failure-*.
- transcript event count, stdout/stderr count, observed public exit status,
  EOF/exit-status/close observed booleans, peer-output-receipt-local, and
  openssh-compat-discriminator-local.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test ssh_openssh_compat_discriminator --quiet: pass.
- cargo -Zjson-target-spec test ssh_peer_output_receipt --quiet: pass.
- cargo -Zjson-target-spec test --quiet: pass after exporting
  PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH.
  An initial full-test attempt without that PATH failed with
  qemu-system-aarch64 not found; that was an environment setup miss, not a
  source failure.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: fmt/lint/typecheck, focused unit tests, full QEMU/substitute
no_std test harness, diff checks, and docs build. No hardware/lab evidence was
claimed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public
readiness labels, public SSH message names/numbers, public request type names,
public status values/categories, public length/count categories, validation
commands, and fixed classifications. It retains no private user data, channel
identifiers, request payload bytes, command payload bytes, channel data bytes,
key/session material, user names, fingerprints, signatures, session
identifiers, live peer data, hardware data, or boot artifacts.

## Acceptance

Accepted as phase12-ssh-openssh-compat-discriminator-core-accepted. The next
selected task is
phase12-ssh-openssh-compat-discriminator-feature-smoke-20260623.
