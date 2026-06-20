# Phase 12.4 Shell Sockdiag Open/Close Smoke

Task: phase12-network-shell-sockdiag-open-close-smoke-20260620

Status: accepted

Classification: phase12-network-shell-sockdiag-open-close-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the shell-visible
`/bin/sockdiag` diagnostic path. The smoke records a command-loop transcript
that resolves `/bin/sockdiag` through VFS/userspace executable lookup, opens an
accepted socket descriptor through `TALOS_SOCKET_SYSCALL = 6`, closes it
through `TALOS_CLOSE_SYSCALL = 2`, verifies descriptor/backing lifetime
behavior, and observes `waitpid` plus `laststatus`.

This task does not add source runtime behavior beyond the task-owned smoke
script and retained evidence. It does not accept Pi 5 hardware behavior,
hardwareTestLock acquisition, lab mutation, boot publication, live driver
adapters, live packet I/O, network reachability, send, recv, bind, connect,
listen, accept, poll/blocking network I/O, UDP/TCP payload transport, SSH,
smoltcp, broad socket expansion, public stable socket ABI acceptance, or a
phase transition.

## Findings And Dispositions

- fixed: Added `scripts/qemu-shell-sockdiag-open-close-smoke.sh` as the
  task-owned host/QEMU-substitute smoke command for the shell-visible
  `/bin/sockdiag` boundary.
- fixed: Retained smoke output under
  `tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/` with command
  log, source anchors, transcript, classification, and evidence map.
- fixed: The retained positive path proves `exec /bin/sockdiag` through VFS
  executable lookup, VFS open/read, startup ABI, `TALOS_SOCKET` open,
  `DescriptorObjectKind::Socket` process descriptor ownership, bounded socket
  backing state, `TALOS_CLOSE` close/drop, `waitpid`, and `laststatus`.
- fixed: Deterministic controls remain retained for malformed arguments,
  missing executable identity, unsupported domain/type/protocol, invalid and
  closed descriptors, wrong-owner backing, descriptor capacity, socket backing
  capacity, no-partial-allocation failure, scalar dispatch ENOTSUP outside the
  socket-table-aware path, bounded syscall vocabulary, and unchanged
  `/bin/pingdiag` behavior.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The transcript records that behavior and
  labels the intended boundary checks; the evidence remains
  host/QEMU-substitute smoke and includes six passing 668-test invocations.
- removed: No runtime source behavior, send/recv path, live driver path, lab
  artifact, hardware claim, public socket ABI claim, or phase-transition claim
  was added by this retained smoke task.
- deferred: Smoke closeout remains the next dependency-gated reconciliation
  task before supervisor planning decides any send/recv, UDP/TCP, live packet
  I/O, SSH, public sockets, or phase-transition direction.

## Evidence

- Smoke command:
  `scripts/qemu-shell-sockdiag-open-close-smoke.sh`.
- Retained transcript:
  `tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/smoke-transcript.md`.
- Command transcript:
  `tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/qemu-shell-sockdiag-open-close-smoke.log`.
- Source anchors:
  `tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/source-anchors.txt`.
- Classification:
  `tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/evidence-map.json`.
- Accepted predecessor:
  phase12-network-shell-sockdiag-open-close-core-20260620 accepted and
  committed at 1a80c724818a1a6656391c8e17c171c5ed621484.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed, 668 no_std tests.
- scripts/qemu-shell-sockdiag-open-close-smoke.sh: passed, six
  host/QEMU-substitute test invocations each reporting 668 no_std tests
  passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, send, recv, bind, connect, listen, accept, poll/blocking network
I/O, UDP/TCP payload transport, SSH, smoltcp, broad socket expansion, public
stable socket ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-open-close-closeout-20260620.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace `/bin/sockdiag` execution, VFS executable identity,
startup ABI, selected socket open/close syscall path, process descriptor
ownership, bounded socket backing state, close/drop behavior, waitpid,
laststatus, deterministic controls, unchanged `/bin/pingdiag`, and unchanged
bounded syscall vocabulary. Kernel fake commands, send/recv, bind/connect,
UDP/TCP payload transport, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, lab mutation, boot publication, broad socket
expansion, public stable socket ABI acceptance, and phase transition remain
rejected.

Commit: recorded in durable supervisor state after commit creation.
