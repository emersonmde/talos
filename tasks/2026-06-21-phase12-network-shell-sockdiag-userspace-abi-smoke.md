# Phase 12.4 Shell Sockdiag Userspace ABI Smoke

Task: phase12-network-shell-sockdiag-userspace-abi-smoke-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-userspace-abi-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence proving shell-visible /bin/sockdiag
exercises the documented private userspace_socket_abi helper path and accepted
host-only smoltcp TCP diagnostic observations through VFS/userspace.

This task does not change source behavior, acquire hardwareTestLock, mutate
the lab, publish a boot artifact, retain generated-root evidence, perform live
packet I/O, claim Pi 5 hardware behavior, claim hardware reachability, accept
SSH, add UDP/raw sockets, broaden sockets, accept POSIX/Linux compatibility,
accept a public stable socket ABI, or transition phase.

## Findings And Dispositions

- fixed: Added scripts/qemu-shell-sockdiag-userspace-abi-smoke.sh as the
  task-owned retained host/QEMU-substitute smoke command.
- fixed: Retained smoke output under
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/ with command
  log, source anchors, transcript, classification, and evidence map.
- fixed: The smoke records exec /bin/sockdiag through VFS executable
  lookup/open/read, startup ABI, userspace_socket_abi wrapper-built calls for
  socket, bind, listen, connect, accept, send, recv, poll, poll_wait, and
  close, the accepted smoltcp Established handshake observations, accepted
  descriptor attachment, one bounded payload-transfer observation, waitpid,
  and laststatus.
- fixed: Deterministic controls remain retained for malformed arguments,
  missing executable identity, ABI constant/wrapper coverage, unchanged local
  socket diagnostics, unchanged /bin/pingdiag behavior, and bounded syscall
  vocabulary.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The retained transcript records four
  passing 695-test invocations and labels the intended boundary checks.
- removed: No source behavior change, lab artifact, hardware claim, generated
  root publication, live packet I/O claim, public socket ABI claim, POSIX/Linux
  compatibility claim, UDP/raw socket claim, SSH claim, broad socket claim, or
  phase-transition claim was added.
- deferred: The selected closeout task remains responsible for reconciling
  this retained ABI smoke evidence before supervisor planning decides any
  broader socket/network direction.

## Evidence

- Smoke command:
  scripts/qemu-shell-sockdiag-userspace-abi-smoke.sh.
- Retained transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/smoke-transcript.md.
- Command transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/qemu-shell-sockdiag-userspace-abi-smoke.log.
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/source-anchors.txt.
- Classification:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/classification.json.
- Evidence map:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/evidence-map.json.
- Accepted predecessor:
  phase12-network-shell-sockdiag-userspace-abi-core-20260621 accepted and
  committed at 33832014ee22c25abef3a2152d1ec58a70d7b21b.

## Validation

- scripts/qemu-shell-sockdiag-userspace-abi-smoke.sh: passed, four
  host/QEMU-substitute test invocations each reporting 695 no_std tests passed.
- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: not run as a separate full gate
  because no source behavior changed; the task-owned smoke ran the focused
  QEMU-substitute filters.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, UDP/raw socket work, SSH, broad socket
expansion, POSIX/Linux compatibility acceptance, public stable socket ABI
acceptance, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-userspace-abi-closeout-20260621.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace /bin/sockdiag execution, VFS executable identity,
startup ABI, documented private userspace_socket_abi helper constructors,
ABI PollEntry layout, descriptor-backed socket dispatch, host-only smoltcp TCP
bridge records, Established handshake states, accepted-descriptor attachment,
one bounded payload-transfer observation, waitpid, laststatus, deterministic
controls, unchanged accepted local socket diagnostics, unchanged /bin/pingdiag,
and unchanged bounded syscall vocabulary. Kernel fake TCP commands, live
driver adapters, live packet I/O, hardware reachability, SSH, lab mutation,
boot publication, generated-root publication, broad socket expansion, UDP/raw
sockets, POSIX/Linux compatibility, public stable socket ABI acceptance, and
phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
