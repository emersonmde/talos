# Phase 12.6 SSH session channel-open closeout

Task id: phase12-ssh-session-channel-open-closeout-20260622

Status: accepted.

Classification: phase12-ssh-session-channel-open-closeout-accepted.

## Goal

Close out the bounded local modeled SSH session channel-open frontier by
reconciling the accepted contract, source implementation, retained smoke
evidence, documentation, redaction boundary, validation, and deferred work.

## Scope

- Reconciled the accepted session channel-open contract, core source task, and
  retained host/QEMU-substitute smoke evidence.
- Confirmed the accepted frontier is exactly one local modeled protocol
  bookkeeping session channel after local modeled publickey USERAUTH_SUCCESS.
- Confirmed the only accepted success counters are authentication-success=true
  from the prerequisite plus session-count=1 and channel-count=1 for the
  modeled channel-open success path.
- Confirmed shell-attached=false, live-reachability=false, and ssh-ready=false
  remain authoritative.
- Updated Phase 12 project docs and roadmap with the closed channel-open
  frontier.

## Non-goals preserved

No Rust source feature work, smoke expansion, hardware/lab action, boot
publication, PTY/TTY/process/shell attachment, channel data,
EOF/close/window flow control, shell/pty/exec/subsystem request behavior,
live socket reachability, hardware proof, OpenSSH/POSIX/Linux compatibility
claim, broad expansion, phase transition, or ssh-ready=true is accepted.

## Reconciled Evidence

- Contract: tasks/2026-06-22-phase12-ssh-session-channel-open-contract.md.
- Core implementation: tasks/2026-06-22-phase12-ssh-session-channel-open-core.md.
- Smoke evidence:
  - scripts/qemu-shell-ssh-session-channel-open-smoke.sh.
  - tasks/evidence/2026-06-22-ssh-session-channel-open-smoke/qemu-shell-ssh-session-channel-open-smoke.log.
  - tasks/evidence/2026-06-22-ssh-session-channel-open-smoke/evidence-map.md.
- Source boundary: src/ssh_service_readiness.rs.
- Documentation:
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/roadmap.md.
  - docs/src/decisions/README.md.

The accepted chain agrees that Talos classifies
SSH_MSG_CHANNEL_OPEN_CONFIRMATION only after the accepted local publickey
USERAUTH_SUCCESS account-policy prerequisite and only for one
SSH_MSG_CHANNEL_OPEN request whose channel type is the public SSH string
session, whose bounded five-field shape is exact, whose policy is enabled,
where no modeled session/channel already exists, and where the path is not
redaction-sensitive.

Missing authentication, disabled policy, duplicate/existing channel,
redaction-sensitive paths, wrong message number, unsupported channel type,
malformed packet, over-limit shape, and trailing data fail closed with
SSH_MSG_CHANNEL_OPEN_FAILURE and fixed labels.

## Findings and Disposition

- fixed: reconciled contract, implementation, and smoke evidence into one
  closed channel-open frontier.
- fixed: documented that the accepted channel is only local protocol
  bookkeeping and not shell, PTY/TTY, process, descriptor, filesystem, live
  socket, or compatibility behavior.
- fixed: confirmed readiness counters advance only to authentication-success,
  session-count=1, and channel-count=1 on the modeled success path.
- fixed: preserved shell-attached=false, live-reachability=false, and
  ssh-ready=false across all accepted evidence.
- fixed: updated docs/src/project/phase12-networking-ssh.md and
  docs/src/roadmap.md with the accepted closeout frontier.
- not-an-issue: docs/src/decisions/README.md already records the durable
  architecture policy from the contract; this closeout adds no new policy.
- deferred: shell request contract, PTY/process ownership, channel data,
  EOF/close/window handling, live reachability, hardware proof,
  OpenSSH/POSIX/Linux compatibility, broad expansion, phase transition, and
  ssh-ready=true remain future tasks.

## Validation

- static task/docs/evidence review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this closeout
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, Rust source implementation, shell attachment, live
reachability claim, compatibility claim, broad expansion, or phase transition
was performed.

## Redaction Review

Pass. Retained closeout evidence contains only task ids, source/doc paths,
fixed labels, public SSH message names/numbers, public field-count and public
channel-type length categories, readiness counters, validation commands, test
names, and classifications. It retains no request payload bytes, channel
identifiers, window sizes, packet sizes, user/operator identity, key material,
key-derived identifiers, stable identifiers, session-id bytes, signatures,
hardware data, or boot artifacts.

## Acceptance

Accepted as the closed local modeled SSH session channel-open frontier.

selected_next_task=phase12-ssh-session-shell-request-contract-20260622.
