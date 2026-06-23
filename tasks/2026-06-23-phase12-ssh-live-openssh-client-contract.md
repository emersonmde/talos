# Phase 12.6 SSH live OpenSSH client contract

Task id: phase12-ssh-live-openssh-client-contract-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-live-openssh-client-contract-accepted

## Goal

Define the smallest future live OpenSSH client discriminator after accepted
local/offline OpenSSH compatibility-discriminator evidence, without running
OpenSSH, mutating lab or hardware state, publishing a boot archive, claiming
live reachability, remote receipt, compatibility=true, a phase transition,
broad command expansion, PTY/SCP/SFTP, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-contract.md.
- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-core.md.
- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-openssh-compat-discriminator-closeout.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-closeout.md.
- src/ssh_service_readiness.rs.
- src/network.rs.
- src/userspace_socket_abi.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- memory/talos-supervisor-state.json task
  phase12-ssh-live-openssh-client-contract-20260623.

## Accepted Inputs

The future live-client evidence task may consume only the accepted local/offline
OpenSSH-compatible closeout discriminator and its prerequisites:

- one descriptor-backed in-kernel stream-socket peer and accepted local SSH
  service delivery;
- accepted local authentication/session/channel, shell attachment,
  channel-data/stdio, channel-window, channel-lifecycle, POSIX EOF/wait,
  peer-output receipt, and local/offline OpenSSH-compatible closeout transcript
  classification;
- the existing authoritative counters
  openssh-compat-discriminator-local=true, live-reachability=false,
  remote-receipt=false, compatibility=false, and ssh-ready=false.

These inputs do not prove that a host OpenSSH client can reach Talos, receive
Talos output, complete an interoperable session, or run a useful shell.

## Live Client Discriminator Contract

The smallest future live OpenSSH client discriminator is a single
hardware-serialized evidence task that runs one host OpenSSH client attempt
against a selected Talos Pi 5 candidate only after all prerequisites are
captured. The contract is a discriminator, not a compatibility acceptance gate.

The future task must capture, before any client attempt:

1. hardwareTestLock ownership and pre-run restore/identity state;
2. candidate build identity, boot archive identity, configured/effective kernel,
   and TFTP-served kernel hash/size evidence;
3. a fresh serial cursor before publication or power action;
4. the exact public OpenSSH invocation class, limited to client version family,
   target port category, disabled PTY/allocation mode if used, command class if
   used, timeout category, and option category names only;
5. redaction rules that prohibit retaining host keys, authorized keys,
   signatures, session identifiers, fingerprints, user names, addresses,
   payload bytes, command bytes, channel data bytes, and live peer identifiers.

The future task may retain only public non-secret observations:

- whether the host client process was launched under the task-owned timeout;
- whether TCP connect reached the selected Talos candidate;
- public SSH phase reached, limited to categories such as no-tcp-connect,
  transport-banner, kex-started, userauth-started, session-channel-opened,
  shell-requested, command-output-observed, eof-observed, exit-status-observed,
  close-observed, client-timeout, or client-exit;
- public OpenSSH exit category, not raw logs containing identifiers or payloads;
- sanitized Talos serial readiness labels and fixed SSH readiness counters;
- TFTP delta and final pre-restore identity proving the selected candidate was
  the image exercised.

The future discriminator may introduce at most a live-client-discriminator
classification such as live-openssh-client-discriminator-observed=true when the
public evidence proves that a host OpenSSH client reached the selected Talos
candidate and observed the same public closeout categories already accepted by
the local/offline discriminator. It still must leave live-reachability,
remote-receipt, compatibility, and ssh-ready false unless a later, explicitly
scoped acceptance task upgrades those claims with stronger evidence.

## Failure Contract

The future evidence task must fail closed and avoid compatibility claims for:

- missing hardwareTestLock ownership, stale restore state, mismatched
  candidate/boot/TFTP identity, stale serial cursor, or inconclusive TFTP delta;
- OpenSSH not available in the runner, missing network route, timeout before
  TCP connect, timeout during transport/userauth/session/channel closeout, or
  nonzero client exit without the required public observations;
- any raw OpenSSH log, serial line, packet capture, host key, authorized key,
  signature, session identifier, user name, address, fingerprint, payload byte,
  command byte, channel data byte, or stable live peer identifier in retained
  evidence;
- any attempt to infer broad reachability, remote receipt, interoperability,
  PTY/SCP/SFTP, POSIX/Linux compatibility, broad command expansion, phase
  transition, or ssh-ready=true from a single discriminator run.

If a Pi 5 hardware run is inconclusive, the future task must follow the
standard triage sequence before changing code: candidate identity, fresh serial
cursor, TFTP delta, known-good control, then candidate rerun.

## Findings And Disposition

- fixed: defined the future live OpenSSH client work as a hardware-serialized
  discriminator rather than a broad compatibility or ssh-ready acceptance gate.
- fixed: named exact prerequisites: hardwareTestLock ownership, selected
  candidate identity, boot/TFTP identity, fresh serial cursor, public
  invocation categories, and strict redaction rules.
- fixed: limited retained observations to public phase categories, sanitized
  readiness labels, public exit categories, TFTP delta, and final candidate
  identity.
- fixed: preserved live-reachability=false, remote-receipt=false,
  compatibility=false, and ssh-ready=false after this contract.
- fixed: recorded that no explicit live-client evidence task is currently
  queued, so supervisor planning is required before a worker can run OpenSSH or
  mutate lab/hardware state.
- not-an-issue: no Rust source change, external OpenSSH run, Pi 5 hardware
  action, boot publication, or ADR entry is required for this contract-only
  task.
- deferred: live OpenSSH client execution, hardware proof, any live-client
  discriminator implementation/evidence task, live-reachability=true,
  remote-receipt=true, compatibility=true, PTY/SCP/SFTP, broad command
  expansion, phase transition, and ssh-ready=true.

## Validation

- static task/docs/source review: pass.
- cargo fmt --all -- --check: conditional skip, no Rust source, tests, or Cargo
  metadata touched.
- cargo -Zjson-target-spec test --quiet: conditional skip, no Rust source,
  tests, or Cargo metadata touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

Evidence levels: static task/docs/source review, diff checks, and docs build.
No hardware/lab evidence, external OpenSSH execution, boot publication, live
reachability claim, remote receipt claim, compatibility=true claim, broad
command expansion, phase transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public SSH
message names/numbers, public request type names, public status
values/categories, public length/count categories, public OpenSSH invocation
and phase categories, readiness counters, validation commands, fixed labels,
and classifications. It retains no private user data, channel identifiers,
request payload bytes, command payload bytes, channel data bytes, key/session
material, user names, addresses, fingerprints, signatures, session identifiers,
live peer identifiers, hardware serial content, raw OpenSSH logs, or boot
artifact bytes.

## Acceptance

Accepted as phase12-ssh-live-openssh-client-contract-accepted.

selected_next_task=null.
planningNeeded=true.
planningReason=Supervisor must queue an explicit live-client evidence task
before the worker can run OpenSSH, mutate lab/hardware state, publish boot
archives, or claim any live reachability, remote receipt, compatibility, phase
transition, or ssh-ready frontier.
