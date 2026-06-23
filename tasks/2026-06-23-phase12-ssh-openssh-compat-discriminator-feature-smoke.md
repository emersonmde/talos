# Phase 12.6 SSH OpenSSH compatibility discriminator feature smoke

Task id: phase12-ssh-openssh-compat-discriminator-feature-smoke-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-openssh-compat-discriminator-feature-smoke-accepted

## Goal

Record focused feature-smoke evidence for the local/offline OpenSSH
compatibility discriminator without running OpenSSH, using Pi 5 hardware,
publishing a boot archive, claiming live reachability, remote receipt,
compatibility=true, broad command expansion, PTY/SCP/SFTP, phase transition, or
ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-contract.md.
- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-core.md.
- src/ssh_service_readiness.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- memory/talos-supervisor-state.json task
  phase12-ssh-openssh-compat-discriminator-feature-smoke-20260623.

## Feature-Smoke Evidence

Focused no_std unit evidence exercises the accepted local/offline discriminator
success path over sanitized public transcript categories only:

1. optional SSH_MSG_CHANNEL_DATA stdout categories;
2. optional SSH_MSG_CHANNEL_EXTENDED_DATA stderr categories;
3. SSH_MSG_CHANNEL_EOF;
4. SSH_MSG_CHANNEL_REQUEST exit-status with request type exit-status and
   want_reply=false;
5. SSH_MSG_CHANNEL_CLOSE.

The focused discriminator tests also cover representative fail-closed controls
for missing accepted local prerequisites, missing modeled peer receipt,
redaction-sensitive input, data after EOF, close before exit-status, duplicate
EOF or terminal messages, unsupported request shape, exit-status
want_reply=true, malformed/zero-length data shape, over-limit data length, and
over-limit transcript event count.

The only accepted readiness frontier remains
openssh-compat-discriminator-local=true for this local/offline shape.
live-reachability=false, remote-receipt=false, compatibility=false, and
ssh-ready=false remain authoritative.

## Findings And Disposition

- fixed: retained focused feature-smoke evidence for the accepted local/offline
  discriminator success transcript.
- fixed: retained focused fail-closed evidence for missing prerequisites,
  missing modeled peer receipt, lifecycle/order violations, duplicate terminal
  messages, unsupported request shape, malformed/over-limit shape, and
  redaction-sensitive input.
- fixed: retained full no_std regression evidence preserving accepted SSH
  readiness, descriptor, syscall, userspace socket ABI, and stream-socket model
  surfaces.
- not-an-issue: no source change was required because the accepted core task
  already implemented the discriminator and focused tests.
- deferred: live OpenSSH client execution, Pi 5 hardware proof, live TCP
  reachability, remote-receipt=true, compatibility=true, PTY/SCP/SFTP, broad
  command expansion, phase transition, and ssh-ready=true remain outside this
  task and require later queued tasks.

## Validation

- cargo fmt --all -- --check: conditional skip, no Rust source, tests, or Cargo
  metadata touched.
- cargo -Zjson-target-spec test ssh_openssh_compat_discriminator --quiet: pass;
  no_std QEMU/substitute harness reported 823 passed.
- cargo -Zjson-target-spec test --quiet: pass; no_std QEMU/substitute harness
  reported 823 passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: focused unit tests, full QEMU/substitute no_std regression,
diff checks, and docs build. No hardware/lab evidence was claimed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public readiness
labels, public SSH message names/numbers, public request type names, public
status values/categories, public length/count categories, validation commands,
and fixed classifications. It retains no private user data, channel
identifiers, request payload bytes, command payload bytes, channel data bytes,
key/session material, user names, fingerprints, signatures, session
identifiers, live peer data, hardware data, or boot artifacts.

## Acceptance

Accepted as
phase12-ssh-openssh-compat-discriminator-feature-smoke-accepted. The next
selected task is
phase12-ssh-openssh-compat-discriminator-closeout-20260623.
