# Phase 12.4 Shell Sockdiag Readiness/Poll Smoke

Task: phase12-network-shell-sockdiag-readiness-poll-smoke-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-readiness-poll-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the shell-visible /bin/sockdiag
readiness/poll diagnostic path. The smoke records a command-loop transcript
that resolves /bin/sockdiag through VFS/userspace executable lookup, opens
listener, client, and accepted socket descriptors through the accepted private
local socket syscalls, records TALOS_POLL_SYSCALL = 13 readiness over
descriptor-backed local socket states, closes descriptors, and observes
waitpid plus laststatus.

This task does not add runtime source behavior beyond the task-owned smoke
script and retained evidence. It does not accept Pi 5 hardware behavior,
hardwareTestLock acquisition, lab mutation, boot publication, generated-root
publication, live driver adapters, live packet I/O, hardware reachability,
blocking waits, scheduler wait queues, timeout handling, UDP/TCP payload
transport, SSH, smoltcp, cross-process/global poll sets, broad socket
expansion, public stable socket ABI acceptance, or a phase transition.

## Findings And Dispositions

- fixed: Added scripts/qemu-shell-sockdiag-readiness-poll-smoke.sh as the
  task-owned host/QEMU-substitute smoke command for the shell-visible
  /bin/sockdiag readiness/poll boundary.
- fixed: Retained smoke output under
  tasks/evidence/2026-06-21-shell-sockdiag-readiness-poll-smoke/ with
  command log, source anchors, transcript, classification, and evidence map.
- fixed: The retained positive path proves exec /bin/sockdiag through VFS
  executable lookup, VFS open/read, startup ABI, accepted socket
  open/bind/listen/connect/accept/send/recv/poll/close, waitpid, and
  laststatus.
- fixed: Deterministic controls remain retained for listener pending accept
  READ, empty recv queue zero readiness, queued payload READ, writable peer
  FIFO WRITE, full peer FIFO zero write readiness, peer close READ | HANGUP,
  invalid descriptor ERROR, non-socket descriptor ERROR, unsupported poll
  events EINVAL, malformed poll calls, scalar dispatch ENOTSUP, unchanged
  accepted socket diagnostics, and unchanged /bin/pingdiag behavior.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The transcript records that behavior and
  labels the intended boundary checks; the evidence remains
  host/QEMU-substitute smoke and includes five passing 679-test invocations.
- removed: No runtime source behavior, live driver path, lab artifact,
  hardware claim, public socket ABI claim, UDP/TCP transport claim,
  blocking/wait-queue claim, or phase-transition claim was added by this
  retained smoke task.
- deferred: Smoke closeout remains the next dependency-gated reconciliation
  task before supervisor planning decides any blocking waits, scheduler wakeup
  queues, timeout handling, UDP/TCP payload transport, live packet I/O, SSH,
  public sockets, or phase-transition direction.

## Evidence

- Smoke command:
  scripts/qemu-shell-sockdiag-readiness-poll-smoke.sh.
- Retained transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-readiness-poll-smoke/smoke-transcript.md.
- Command transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-readiness-poll-smoke/qemu-shell-sockdiag-readiness-poll-smoke.log.
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-readiness-poll-smoke/source-anchors.txt.
- Classification:
  tasks/evidence/2026-06-21-shell-sockdiag-readiness-poll-smoke/classification.json.
- Evidence map:
  tasks/evidence/2026-06-21-shell-sockdiag-readiness-poll-smoke/evidence-map.json.
- Accepted predecessor:
  phase12-network-shell-sockdiag-readiness-poll-core-20260621 accepted and
  committed at 0cf2177e2fdb8de77bacf9330f1c975bdb64fce2.

## Validation

- scripts/qemu-shell-sockdiag-readiness-poll-smoke.sh: passed, five
  host/QEMU-substitute test invocations each reporting 679 no_std tests
  passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, blocking wait, scheduler wait queue,
timeout handling, UDP/TCP payload transport, SSH, smoltcp, broad socket
expansion, public stable socket ABI acceptance, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-readiness-poll-closeout-20260621.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace /bin/sockdiag execution, VFS executable identity,
startup ABI, selected socket open/bind/listen/connect/accept/send/recv/poll/
close syscall path, process descriptor ownership, descriptor-backed local
listener/client/accepted socket state, private nonblocking READ/WRITE/HANGUP/
ERROR readiness bits, waitpid, laststatus, deterministic controls, unchanged
accepted socket diagnostics, unchanged /bin/pingdiag, and unchanged bounded
syscall vocabulary. Kernel fake commands, blocking waits, scheduler wait
queues, timeout handling, UDP/TCP payload transport, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, lab mutation, boot
publication, generated-root publication, cross-process/global poll sets, broad
socket expansion, public stable socket ABI acceptance, and phase transition
remain rejected.

Commit: recorded in durable supervisor state after commit creation.
