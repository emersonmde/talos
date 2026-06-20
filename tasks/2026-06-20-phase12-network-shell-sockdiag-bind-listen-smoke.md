# Phase 12.4 Shell Sockdiag Bind/Listen Smoke

Task: phase12-network-shell-sockdiag-bind-listen-smoke-20260620

Status: accepted

Classification: phase12-network-shell-sockdiag-bind-listen-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the shell-visible
`/bin/sockdiag` bind/listen diagnostic path. The smoke records a command-loop
transcript that resolves `/bin/sockdiag` through VFS/userspace executable
lookup, opens an accepted socket descriptor through
`TALOS_SOCKET_SYSCALL = 6`, binds it through `TALOS_BIND_SYSCALL = 7`,
listens through `TALOS_LISTEN_SYSCALL = 8`, closes it through
`TALOS_CLOSE_SYSCALL = 2`, verifies descriptor/backing lifetime behavior, and
observes `waitpid` plus `laststatus`.

This task does not add source runtime behavior beyond the task-owned smoke
script and retained evidence. It does not accept Pi 5 hardware behavior,
hardwareTestLock acquisition, lab mutation, boot publication, generated-root
publication, live driver adapters, live packet I/O, hardware reachability,
send, recv, connect, accept, poll/blocking network I/O, UDP/TCP payload
transport, SSH, smoltcp, broad socket expansion, public stable socket ABI
acceptance, or a phase transition.

## Findings And Dispositions

- fixed: Added `scripts/qemu-shell-sockdiag-bind-listen-smoke.sh` as the
  task-owned host/QEMU-substitute smoke command for the shell-visible
  `/bin/sockdiag` bind/listen boundary.
- fixed: Retained smoke output under
  `tasks/evidence/2026-06-20-shell-sockdiag-bind-listen-smoke/` with command
  log, source anchors, transcript, classification, and evidence map.
- fixed: The retained positive path proves `exec /bin/sockdiag` through VFS
  executable lookup, VFS open/read, startup ABI, `TALOS_SOCKET` open,
  `DescriptorObjectKind::Socket` process descriptor ownership,
  `TALOS_BIND` endpoint state, `TALOS_LISTEN` listening/backlog state,
  `TALOS_CLOSE` close/drop, `waitpid`, and `laststatus`.
- fixed: Deterministic controls remain retained for malformed arguments,
  missing executable identity, unsupported domain/type/protocol,
  listen-before-bind, invalid bind endpoint, invalid backlog, repeated bind,
  repeated listen backlog update, invalid and closed descriptors, wrong-owner
  backing, descriptor capacity, socket backing capacity, no-partial-allocation
  failure, scalar dispatch `ENOTSUP` outside the socket-table-aware path,
  bounded syscall vocabulary, unchanged socket open/close behavior, and
  unchanged `/bin/pingdiag` behavior.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The transcript records that behavior and
  labels the intended boundary checks; the evidence remains
  host/QEMU-substitute smoke and includes eight passing 670-test invocations.
- removed: No runtime source behavior, send/recv path, connect/accept path,
  live driver path, lab artifact, hardware claim, public socket ABI claim, or
  phase-transition claim was added by this retained smoke task.
- deferred: Smoke closeout remains the next dependency-gated reconciliation
  task before supervisor planning decides any send/recv, connect/accept,
  UDP/TCP, live packet I/O, SSH, public sockets, or phase-transition direction.

## Evidence

- Smoke command:
  `scripts/qemu-shell-sockdiag-bind-listen-smoke.sh`.
- Retained transcript:
  `tasks/evidence/2026-06-20-shell-sockdiag-bind-listen-smoke/smoke-transcript.md`.
- Command transcript:
  `tasks/evidence/2026-06-20-shell-sockdiag-bind-listen-smoke/qemu-shell-sockdiag-bind-listen-smoke.log`.
- Source anchors:
  `tasks/evidence/2026-06-20-shell-sockdiag-bind-listen-smoke/source-anchors.txt`.
- Classification:
  `tasks/evidence/2026-06-20-shell-sockdiag-bind-listen-smoke/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-20-shell-sockdiag-bind-listen-smoke/evidence-map.json`.
- Accepted predecessor:
  phase12-network-shell-sockdiag-bind-listen-core-20260620 accepted and
  committed at 5c18af895e42fe147f17b9fc2e9e1506f44037a2.

## Validation

- scripts/qemu-shell-sockdiag-bind-listen-smoke.sh: passed, eight
  host/QEMU-substitute test invocations each reporting 670 no_std tests
  passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, send, recv, connect, accept,
poll/blocking network I/O, UDP/TCP payload transport, SSH, smoltcp, broad
socket expansion, public stable socket ABI acceptance, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-bind-listen-closeout-20260620.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace `/bin/sockdiag` execution, VFS executable identity,
startup ABI, selected socket open/bind/listen/close syscall path,
process descriptor ownership, descriptor-backed listening socket state,
close/drop behavior, waitpid, laststatus, deterministic controls, unchanged
socket open/close behavior, unchanged `/bin/pingdiag`, and unchanged bounded
syscall vocabulary. Kernel fake commands, send/recv, connect/accept,
poll/blocking network I/O, UDP/TCP payload transport, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, lab mutation, boot
publication, generated-root publication, broad socket expansion, public stable
socket ABI acceptance, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
