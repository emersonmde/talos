# Phase 12.4 Shell Sockdiag Connect/Accept Smoke

Task: phase12-network-shell-sockdiag-connect-accept-smoke-20260620

Status: accepted

Classification: phase12-network-shell-sockdiag-connect-accept-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the shell-visible
`/bin/sockdiag` local connect/accept diagnostic path. The smoke records a
command-loop transcript that resolves `/bin/sockdiag` through VFS/userspace
executable lookup, opens listener and client socket descriptors through
`TALOS_SOCKET_SYSCALL = 6`, binds through `TALOS_BIND_SYSCALL = 7`,
listens through `TALOS_LISTEN_SYSCALL = 8`, connects through
`TALOS_CONNECT_SYSCALL = 9`, accepts through `TALOS_ACCEPT_SYSCALL = 10`,
closes through `TALOS_CLOSE_SYSCALL = 2`, verifies descriptor/backing
lifetime behavior, and observes `waitpid` plus `laststatus`.

This task does not add source runtime behavior beyond the task-owned smoke
script and retained evidence. It does not accept Pi 5 hardware behavior,
hardwareTestLock acquisition, lab mutation, boot publication, generated-root
publication, live driver adapters, live packet I/O, hardware reachability,
send, recv, payload bytes, poll/blocking network I/O, UDP/TCP payload
transport, SSH, smoltcp, broad socket expansion, public stable socket ABI
acceptance, or a phase transition.

## Findings And Dispositions

- fixed: Added `scripts/qemu-shell-sockdiag-connect-accept-smoke.sh` as the
  task-owned host/QEMU-substitute smoke command for the shell-visible
  `/bin/sockdiag` connect/accept boundary.
- fixed: Retained smoke output under
  `tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/` with
  command log, source anchors, transcript, classification, and evidence map.
- fixed: The retained positive path proves `exec /bin/sockdiag` through VFS
  executable lookup, VFS open/read, startup ABI, `TALOS_SOCKET` listener and
  client open, `DescriptorObjectKind::Socket` process descriptor ownership,
  `TALOS_BIND` endpoint state, `TALOS_LISTEN` listening state,
  `TALOS_CONNECT` connected client state, `TALOS_ACCEPT` accepted
  server-side descriptor state, `TALOS_CLOSE` close/drop, `waitpid`, and
  `laststatus`.
- fixed: Deterministic controls remain retained for malformed arguments,
  missing executable identity, unsupported domain/type/protocol,
  listen-before-bind, invalid bind endpoint, invalid backlog, repeated bind,
  repeated listen backlog update, accept-before-connect, missing listener, full
  pending queue, non-socket descriptor, invalid and closed descriptors,
  wrong-owner backing, descriptor capacity, socket backing capacity,
  no-partial-allocation/dequeue failure, scalar dispatch `ENOTSUP` outside
  the socket-table-aware path, bounded syscall vocabulary, unchanged socket
  open/close behavior, unchanged bind/listen behavior, and unchanged
  `/bin/pingdiag` behavior.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The transcript records that behavior and
  labels the intended boundary checks; the evidence remains
  host/QEMU-substitute smoke and includes eleven passing 673-test invocations.
- removed: No runtime source behavior, send/recv path, payload transport,
  live driver path, lab artifact, hardware claim, public socket ABI claim, or
  phase-transition claim was added by this retained smoke task.
- deferred: Smoke closeout remains the next dependency-gated reconciliation
  task before supervisor planning decides any send/recv, UDP/TCP payload
  transport, live packet I/O, SSH, public sockets, or phase-transition
  direction.

## Evidence

- Smoke command:
  `scripts/qemu-shell-sockdiag-connect-accept-smoke.sh`.
- Retained transcript:
  `tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/smoke-transcript.md`.
- Command transcript:
  `tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/qemu-shell-sockdiag-connect-accept-smoke.log`.
- Source anchors:
  `tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/source-anchors.txt`.
- Classification:
  `tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/evidence-map.json`.
- Accepted predecessor:
  phase12-network-shell-sockdiag-connect-accept-core-20260620 accepted and
  committed at 20a1ce001f91a667c2de208a8755c36f8837f5ff.

## Validation

- scripts/qemu-shell-sockdiag-connect-accept-smoke.sh: passed, eleven
  host/QEMU-substitute test invocations each reporting 673 no_std tests
  passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, send, recv, payload bytes,
poll/blocking network I/O, UDP/TCP payload transport, SSH, smoltcp, broad
socket expansion, public stable socket ABI acceptance, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-connect-accept-closeout-20260620.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace `/bin/sockdiag` execution, VFS executable
identity, startup ABI, selected socket open/bind/listen/connect/accept/close
syscall path, process descriptor ownership, descriptor-backed local
listener/client/accepted socket state, close/drop behavior, waitpid,
laststatus, deterministic controls, unchanged socket open/close behavior,
unchanged bind/listen behavior, unchanged `/bin/pingdiag`, and unchanged
bounded syscall vocabulary. Kernel fake commands, send/recv, payload
transport, poll/blocking network I/O, UDP/TCP payload transport, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, lab mutation,
boot publication, generated-root publication, broad socket expansion, public
stable socket ABI acceptance, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
