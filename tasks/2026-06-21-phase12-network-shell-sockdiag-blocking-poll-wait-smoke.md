# Phase 12.4 Shell Sockdiag Blocking Poll Wait Smoke

Task: phase12-network-shell-sockdiag-blocking-poll-wait-smoke-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-blocking-poll-wait-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the shell-visible /bin/sockdiag
bounded blocking poll-wait diagnostic path. The smoke records a command-loop
transcript that resolves /bin/sockdiag through VFS/userspace executable
lookup, opens listener, client, and accepted socket descriptors through the
accepted private local socket syscalls, records TALOS_POLL_WAIT_SYSCALL = 14
immediate-ready, wake, timeout, and hangup cases over descriptor-backed local
socket states, closes descriptors, and observes waitpid plus laststatus.

This task does not add runtime source behavior beyond the task-owned smoke
script and retained evidence. It does not accept Pi 5 hardware behavior,
hardwareTestLock acquisition, lab mutation, boot publication, generated-root
publication, live driver adapters, live packet I/O, hardware reachability,
UDP/TCP payload transport, SSH, smoltcp, cross-process/global poll sets, broad
socket expansion, public stable socket ABI acceptance, or a phase transition.

## Findings And Dispositions

- fixed: Added scripts/qemu-shell-sockdiag-blocking-poll-wait-smoke.sh as the
  task-owned host/QEMU-substitute smoke command for the shell-visible
  /bin/sockdiag bounded blocking poll-wait boundary.
- fixed: Retained smoke output under
  tasks/evidence/2026-06-21-shell-sockdiag-blocking-poll-wait-smoke/ with
  command log, source anchors, transcript, classification, and evidence map.
- fixed: The retained positive path proves exec /bin/sockdiag through VFS
  executable lookup, VFS open/read, startup ABI, accepted socket
  open/bind/listen/connect/accept/send/recv/poll/wait/close, scheduler
  blocked/resume state, waitpid, and laststatus.
- fixed: Deterministic controls remain retained for immediate-ready,
  pending-listener wake, payload-read wake, timeout/no-false-ready, peer
  close/hangup wake, scalar dispatch ENOTSUP, invalid timeout EINVAL,
  unsupported events EINVAL, malformed arguments, missing executable identity,
  unchanged accepted socket diagnostics, unchanged nonblocking TALOS_POLL, and
  unchanged /bin/pingdiag behavior.
- not-an-issue: The no_std QEMU runner executes the full target test binary
  for each filtered smoke invocation. The transcript records that behavior and
  labels the intended boundary checks; the evidence remains
  host/QEMU-substitute smoke and includes six passing 683-test invocations.
- removed: No runtime source behavior, live driver path, lab artifact,
  hardware claim, public socket ABI claim, UDP/TCP transport claim, broad
  socket claim, or phase-transition claim was added by this retained smoke
  task.
- deferred: Smoke closeout remains the next dependency-gated reconciliation
  task before supervisor planning decides any cross-process/global poll sets,
  UDP/TCP payload transport, live packet I/O, SSH, public sockets, broad
  socket expansion, or phase-transition direction.

## Evidence

- Smoke command:
  scripts/qemu-shell-sockdiag-blocking-poll-wait-smoke.sh.
- Retained transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-blocking-poll-wait-smoke/smoke-transcript.md.
- Command transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-blocking-poll-wait-smoke/qemu-shell-sockdiag-blocking-poll-wait-smoke.log.
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-blocking-poll-wait-smoke/source-anchors.txt.
- Classification:
  tasks/evidence/2026-06-21-shell-sockdiag-blocking-poll-wait-smoke/classification.json.
- Evidence map:
  tasks/evidence/2026-06-21-shell-sockdiag-blocking-poll-wait-smoke/evidence-map.json.
- Accepted predecessor:
  phase12-network-shell-sockdiag-blocking-poll-wait-core-20260621 accepted and
  committed at b96cc7efc34dabe3d4a4001430a6d51ee4000e86.

## Validation

- scripts/qemu-shell-sockdiag-blocking-poll-wait-smoke.sh: passed, six
  host/QEMU-substitute test invocations each reporting 683 no_std tests
  passed.
- cargo fmt --all -- --check: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, generated-root publication, live driver adapter,
live packet I/O, hardware reachability, UDP/TCP payload transport, SSH,
smoltcp, broad socket expansion, public stable socket ABI acceptance, or phase
transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-blocking-poll-wait-closeout-20260621.

The accepted evidence level is host/QEMU-substitute smoke evidence over
shell-visible VFS/userspace /bin/sockdiag execution, VFS executable identity,
startup ABI, selected socket open/bind/listen/connect/accept/send/recv/poll/
wait/close syscall path, process descriptor ownership, descriptor-backed local
listener/client/accepted socket state, private process-local bounded
TALOS_POLL_WAIT immediate-ready, wake, timeout, and hangup behavior, waitpid,
laststatus, deterministic controls, unchanged accepted socket diagnostics,
unchanged /bin/pingdiag, and unchanged bounded syscall vocabulary. Kernel fake
commands, UDP/TCP payload transport, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, lab mutation, boot publication,
generated-root publication, cross-process/global poll sets, broad socket
expansion, public stable socket ABI acceptance, and phase transition remain
rejected.

Commit: recorded in durable supervisor state after commit creation.
