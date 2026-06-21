# Phase 12.4 Socket Blocking Poll Wait Core

Task: phase12-network-socket-blocking-poll-wait-core-20260621

Status: accepted

Classification: phase12-network-socket-blocking-poll-wait-core-accepted

## Goal

Implement the private process-local bounded blocking socket poll wait selected
by phase12-network-socket-blocking-poll-wait-contract-20260621 without changing
the accepted nonblocking TALOS_POLL readiness behavior.

## Scope

- Add TALOS_POLL_WAIT_SYSCALL = 14 to the private stable syscall number
  vocabulary while keeping scalar/default dispatch fail-closed with ENOTSUP.
- Add a socket-table-aware poll-wait dispatch wrapper that is explicit about
  scheduler/task context, current tick, user mappings, socket descriptor table,
  and a bounded wait table.
- Reuse the accepted 16-byte poll entry layout: fd:u64, events:u32,
  revents:u32. x0 is the user entry array, x1 is the entry count, x2 is the
  finite relative timeout in scheduler ticks, x3 is flags=0, and x4=x5 are
  reserved zero.
- Preserve immediate-ready behavior as a fast path that writes revents and
  returns success without blocking.
- If no requested entry is immediately ready, snapshot the current
  process-local socket descriptors and requested readiness bits, record one
  wait for the current task, and transition that task to TaskState::Blocked.
- Resume a waiting task through SingleCoreScheduler::make_runnable when local
  socket readiness appears or the deadline expires.
- Add focused source/unit tests for immediate-ready, wait registration,
  local send/recv wake, pending accept wake, peer close/hangup wake, timeout,
  malformed arguments, scalar fail-closed behavior, and nonblocking
  compatibility.

## Non-Goals

- No /bin/sockdiag blocking wait output, retained smoke transcript, UDP/TCP
  payload transport, smoltcp integration, cross-process/global poll sets, live
  driver adapters, live packet I/O, Pi 5 hardware run, hardwareTestLock
  acquisition, lab mutation, boot publication, SSH, public stable socket ABI
  acceptance, broad socket expansion, or phase transition.
- No broad scheduler refactor beyond using existing TaskState::Blocked and
  SingleCoreScheduler::make_runnable.

## Findings

- fixed: The private syscall vocabulary now includes TALOS_POLL_WAIT_SYSCALL =
  14 and SyscallNumber::TalosPollWait. Generic scalar/default descriptor
  dispatch keeps it ENOTSUP unless the wait-aware socket-table dispatch is
  explicitly used.
- fixed: The core wait path distinguishes immediate readiness from sleeping.
  Immediate-ready entries are copied back through user memory with the same
  revents semantics as TALOS_POLL and the task remains Runnable.
- fixed: Sleeping waits are not diagnostic retry loops. The wait table records
  one wait per task, descriptor snapshots, requested events, user entry
  address, and deadline tick, then marks the current task Blocked.
- fixed: Wake/resume is scheduler-visible. resume_ready_or_expired reevaluates
  accepted local socket readiness or deadline expiration, writes revents back
  to the caller's poll-entry array, removes the wait, and uses
  SingleCoreScheduler::make_runnable to enqueue the task.
- fixed: Wake sources covered by unit tests are inbound payload bytes after a
  local send, listener pending accept after local connect, peer close/hangup,
  and explicit timeout. Descriptor errors and malformed arguments are covered
  through scalar fail-closed, timeout bounds, unsupported event bits, and
  existing per-entry ERROR behavior.
- not-an-issue: TALOS_POLL_SYSCALL = 13 remains unchanged and covered as a
  nonblocking compatibility regression inside the poll-wait fast-path test.
- deferred: Shell /bin/sockdiag output, retained smoke evidence,
  cross-process/global poll sets, UDP/TCP payload transport, smoltcp
  integration, live packet I/O, hardware reachability, SSH, public socket ABI
  acceptance, broad socket expansion, and phase transition remain for later
  explicit tasks.

## Source Anchors

- src/syscall.rs: TALOS_POLL_WAIT_SYSCALL, TALOS_POLL_WAIT_MAX_TICKS, and
  SyscallNumber::TalosPollWait define the private selector and fail-closed
  scalar/default behavior.
- src/syscall.rs: SocketPollWaitTable, SocketPollWaitOutcome, and
  dispatch_process_descriptor_with_socket_table_and_poll_wait define the
  wait-aware socket dispatch boundary.
- src/syscall.rs: dispatch_talos_poll_wait validates arguments, copies the
  caller poll-entry array, preserves the immediate-ready fast path, snapshots
  socket descriptors, records the deadline, and marks the current task blocked.
- src/syscall.rs: SocketPollWaitTable::resume_ready_or_expired writes wake or
  timeout revents and resumes the task through SingleCoreScheduler.
- src/scheduler.rs: TaskState::Blocked, Task::set_state, and
  SingleCoreScheduler::make_runnable provide the accepted scheduler state
  primitives used by the wait path.
- src/network.rs: NetworkSocketDescriptorTable::readiness provides listener
  pending accept, inbound payload READ, peer FIFO WRITE, peer close HANGUP,
  and per-entry ERROR readiness semantics.

## Evidence

- static source/task/evidence review: reviewed the accepted blocking wait
  contract, src/syscall.rs socket dispatch/poll path, src/network.rs local
  socket readiness, and src/scheduler.rs blocked/runnable primitives.
- fmt/lint: cargo fmt --all -- --check passed.
- focused unit tests: cargo -Zjson-target-spec test --quiet talos_poll_wait
  passed.
- full unit tests: cargo -Zjson-target-spec test --quiet passed.

## Accepted Boundary

The accepted evidence is source/unit only. It proves a private process-local
bounded blocking wait over accepted local socket readiness states. It does not
accept shell-visible blocking diagnostics, retained smoke evidence, UDP/TCP,
live packet I/O, hardware reachability, SSH, public socket ABI compatibility,
broad socket expansion, or phase transition.

selected_next_task: phase12-network-shell-sockdiag-blocking-poll-wait-core-20260621
