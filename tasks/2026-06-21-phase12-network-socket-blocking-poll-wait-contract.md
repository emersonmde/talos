# Phase 12.4 Socket Blocking Poll Wait Contract

Task: phase12-network-socket-blocking-poll-wait-contract-20260621

Status: accepted

Classification: phase12-network-socket-blocking-poll-wait-contract-accepted

## Scope

Define the smallest private bounded blocking poll wait after the accepted
nonblocking TALOS_POLL readiness frontier. This contract selects a new
socket-table-aware TALOS_POLL_WAIT_SYSCALL = 14 path for process-local socket
descriptors only. It preserves the accepted TALOS_POLL_SYSCALL = 13
nonblocking behavior unchanged and does not add runtime implementation in this
task.

This task does not accept shell diagnostic behavior, retained smoke evidence,
UDP/TCP payload transport, smoltcp integration, cross-process/global poll
sets, live driver adapters, live packet I/O, hardware reachability,
hardwareTestLock acquisition, Pi 5 hardware work, lab mutation, boot
publication, SSH, public stable socket ABI acceptance, broad socket expansion,
or a phase transition.

## Findings And Dispositions

- fixed: The accepted readiness/poll closeout deliberately rejected blocking
  waits, scheduler wait queues, and timeout handling. This contract chooses the
  next feature step: one bounded scheduler-owned wait over the accepted
  process-local socket readiness states.
- fixed: Source review found the private stable SVC socket namespace currently
  ends at TALOS_POLL_SYSCALL = 13, so the next bounded private selector is
  TALOS_POLL_WAIT_SYSCALL = 14 on the existing STABLE_SVC_IMMEDIATE = 0 path.
  This is a Talos-private task-chain selector, not Linux syscall-number or libc
  compatibility.
- fixed: The accepted nonblocking TALOS_POLL ABI remains unchanged. Blocking
  behavior is not smuggled into TALOS_POLL flags or reserved registers, so
  existing readiness tests and shell diagnostics remain stable regressions.
- fixed: The user-memory entry ABI reuses the accepted fixed 16-byte poll entry
  layout: fd:u64, events:u32, revents:u32. TALOS_POLL_WAIT takes x0 as the
  entry array pointer, x1 as an entry count from 1 through 8, x2 as a bounded
  relative timeout in scheduler ticks, x3 as flags=0, and x4=x5 as reserved
  zero registers.
- fixed: Timeout is finite and deterministic. x2 must be in the inclusive range
  1 through TALOS_POLL_WAIT_MAX_TICKS = 1024. Zero timeout remains the existing
  TALOS_POLL nonblocking contract; oversized timeouts return EINVAL.
- fixed: Scheduler wait invariants are explicit. If any requested entry is
  immediately ready, TALOS_POLL_WAIT returns without registering a wait. If no
  entry is ready, the kernel records one wait object tied to the current task,
  process owner, descriptor snapshot, requested readiness bits, and deadline
  tick; transitions the task to TaskState::Blocked; and resumes it only through
  an accepted wake source or timeout expiration.
- fixed: Wake sources are the accepted local socket readiness transitions:
  listener pending-accept enqueue, inbound payload bytes, peer FIFO capacity
  becoming available after recv/consume, peer close/drop or hangup, local
  descriptor/backing invalidation, and deadline expiration.
- fixed: Timeout expiration wakes the task and returns success value 0 with all
  revents zero. It is not EAGAIN, not EOF, and not a busy-loop observation.
- fixed: Descriptor errors remain per-entry where possible. Invalid, closed,
  wrong-owner, non-socket, missing-backing, or locally invalidated descriptors
  set TALOS_POLL_ERROR in revents and wake a sleeping wait. Malformed syscall
  arguments, unsupported events, unsupported flags, reserved registers,
  impossible entry counts, and user-copy failures remain whole-call errors.
- fixed: Cancellation behavior is bounded. Signals and restart semantics are
  not implemented by this contract. Process/task teardown removes any owned
  wait record without returning to userspace; a future explicit cancellation
  mechanism must get its own task and may not be inferred from this contract.
- not-an-issue: Existing scheduler primitives already model TaskState::Blocked,
  local runnable queues, make-runnable transitions, and target-owned wake
  consumption. The follow-up core may extend those primitives, but it must
  prove a real blocked/runnable transition rather than a host-test retry loop.
- not-an-issue: Existing socket readiness code already exposes listener
  pending peers, inbound FIFO length, peer FIFO capacity, reverse peer lookup,
  and close/drop hangup. Those surfaces are sufficient to define the wake
  contract without accepting TCP, packet I/O, a global port registry, or
  hardware evidence.
- deferred: Runtime source implementation, /bin/sockdiag blocking wait output,
  retained smoke evidence, cross-process/global poll sets, UDP/TCP payload
  transport, smoltcp integration, live packet I/O, SSH, public socket ABI
  acceptance, broad socket expansion, and phase transition remain deferred to
  later explicit tasks.
- removed: No runtime source cleanup or implementation was justified in this
  contract-only task.

## Accepted Contract

The next implementation task should add one private experimental selector:

- TALOS_POLL_WAIT_SYSCALL = 14
- enum variant: SyscallNumber::TalosPollWait
- SVC immediate: existing STABLE_SVC_IMMEDIATE = 0

The scalar arguments are:

- x0=user_poll_entries: readable and writable user-memory address for the
  entry array.
- x1=entry_count: number of 16-byte entries, from 1 through 8.
- x2=timeout_ticks: relative scheduler tick budget, from 1 through 1024.
- x3=flags: must be 0.
- x4=0, x5=0: reserved.
- return on immediate or wake success: number of entries whose revents is
  nonzero.
- return on timeout success: 0, with every revents field written as 0.

Each user entry has the accepted TALOS_POLL native layout:

    offset  size  field
    0       8     fd
    8       4     events
    12      4     revents

The caller initializes fd and events; the kernel overwrites revents. The
supported requested events remain READ, WRITE, HANGUP, and ERROR. HANGUP and
ERROR may be reported even when not requested because they are terminal/control
readiness conditions. Unsupported event bits return EINVAL for the whole call
before any wait registration.

Wait registration is process-local and task-local:

- The current task and current process owner must be known before the syscall
  may sleep. Missing current owner or missing current task returns EINVAL.
- The wait object stores a snapshot of fd, requested events, current process
  owner, current task id, and deadline tick. It does not store raw user pointers
  beyond the syscall boundary.
- Copy-in and validation complete before sleeping. Copy-out occurs only after
  immediate readiness, wake, or timeout. Copy faults return EFAULT and do not
  leave a registered wait.
- A task may own at most one socket poll wait in this bounded contract. A
  second wait for the same task before completion returns EBUSY.
- The syscall must not pass acceptance through a bounded retry loop. Source/unit
  evidence must show the task reaches TaskState::Blocked, leaves the runnable
  queue, and is made runnable by an accepted socket wake source or timeout.

Readiness and wake semantics are the accepted TALOS_POLL readiness semantics
plus a sleep boundary:

- Listening sockets wake READ waiters when a local connect enqueues at least
  one pending peer.
- Connected and Accepted sockets wake READ waiters when inbound FIFO bytes
  become available.
- Connected and Accepted sockets wake WRITE waiters when the unique local
  peer's inbound FIFO gains at least one byte of capacity after recv/consume.
- Peer close/drop wakes READ and HANGUP waiters. Queued bytes after peer close
  report READ | HANGUP; an empty queue after peer close reports READ | HANGUP
  so a following recv can expose the accepted EPIPE boundary.
- Local descriptor close/drop, backing removal, wrong-owner discovery, or
  non-socket descriptor substitution wakes with ERROR in revents.
- Timeout expiration wakes the blocked task with no ready entries and returns
  0. It must not report spurious READ, WRITE, HANGUP, or ERROR bits.

Whole-call error vocabulary:

- EINVAL: bad entry_count, timeout_ticks outside 1..=1024, unsupported flags,
  nonzero reserved registers, unsupported event bits, missing current owner, or
  missing current task.
- EFAULT: copy-in or copy-out failure.
- EBUSY: the current task already owns a pending socket poll wait.
- ENOTSUP: scalar/default dispatch without socket-table/scheduler wait context.

Per-entry ERROR remains the descriptor-level reporting path for invalid,
closed, wrong-owner, non-socket, missing-backing, oversized fd, or invalidated
descriptors after a wait is registered.

The accepted claim is only private process-local bounded blocking waits over
local socket descriptor state. This contract does not promise Linux poll(2)
ABI compatibility, libc compatibility, public ABI stability, cross-process
polling, packet readiness, TCP readiness, live driver readiness, hardware
reachability, SSH readiness, signals, restart semantics, or arbitrary blocking
I/O.

The selected next bounded task is
phase12-network-socket-blocking-poll-wait-core-20260621.

## Evidence

- static source/task/evidence review:
  - src/syscall.rs currently defines TALOS_POLL_SYSCALL = 13, the fixed
    16-byte poll entry layout, entry count bound 1 through 8, supported
    READ/WRITE/HANGUP/ERROR event bits, and nonblocking socket-table-aware
    dispatch.
  - src/network.rs owns NetworkSocketDescriptorTable::readiness,
    NetworkSocketPendingQueue::len, NetworkSocketPayloadQueue capacity and
    consume behavior, peer lookup, send_ready, recv_peek, recv_commit, and
    close/drop removal of socket backing entries.
  - src/scheduler.rs owns TaskState::Blocked, local runnable queues,
    SingleCoreScheduler::make_runnable, and target-owned wake consumption
    primitives that a follow-up wait core can extend or wrap with focused
    source/unit evidence.
  - src/arch/aarch64/generic_timer.rs exposes monotonic tick accounting for
    bounded timeout classification, while tests may model timeout expiration
    deterministically without a hardware run.
  - tasks/2026-06-21-phase12-network-shell-sockdiag-readiness-poll-closeout.md
    accepts only source/unit plus host/QEMU-substitute private nonblocking
    local socket readiness and explicitly rejects blocking waits, scheduler
    wait queues, timeout handling, UDP/TCP payload transport, live packet I/O,
    hardware reachability, SSH, public socket ABI acceptance, broad socket
    expansion, and phase transition.
- Documentation updates:
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/roadmap.md.
  - docs/src/decisions/README.md.

## Rejected Claims

- No runtime implementation is accepted by this contract task.
- No shell-visible /bin/sockdiag blocking poll wait output or retained smoke
  evidence is accepted.
- No busy-loop, task-local retry loop, or diagnostic-only shim can satisfy the
  follow-up core acceptance gate.
- No cross-process/global poll set, UDP/TCP payload transport, smoltcp
  integration, live packet I/O, live driver adapter, hardware reachability, lab
  mutation, boot publication, SSH, public stable socket ABI acceptance, broad
  socket expansion, or phase transition is accepted.
- No signal, restart, arbitrary cancellation, descriptor flags, pipe readiness,
  terminal readiness, file readiness, or non-socket blocking I/O behavior is
  accepted.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Next Action

Promote only the dependency-gated
phase12-network-socket-blocking-poll-wait-core-20260621 task next. It may
implement only the accepted private process-local bounded blocking poll wait
core over existing descriptor ownership, socket table state, scheduler
blocked/runnable primitives, monotonic timeout accounting, UserMapping copy
helpers, and PosixError vocabulary. Shell diagnostics, retained smoke,
cross-process/global poll sets, UDP/TCP payload transport, live packet I/O,
hardware work, SSH, public socket ABI acceptance, broad socket expansion, and
phase transition remain rejected.
