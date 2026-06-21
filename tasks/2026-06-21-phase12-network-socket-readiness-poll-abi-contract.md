# Phase 12.4 Socket Readiness/Poll ABI Contract

Task: phase12-network-socket-readiness-poll-abi-contract-20260621

Status: accepted

Classification: phase12-network-socket-readiness-poll-abi-contract-accepted

## Scope

Define the smallest private, nonblocking readiness query after the accepted
shell-visible local socket send/recv frontier. This contract selects a bounded
socket-table-aware TALOS_POLL_SYSCALL = 13 path for process-local descriptors
only. It reports readiness for already accepted local socket states and does
not add runtime behavior in this task.

This task does not accept blocking sleep, scheduler wait queues, wakeup
registration, timeout handling, cross-process/global poll sets, UDP/TCP payload
transport, live driver adapters, live packet I/O, hardware reachability,
hardwareTestLock acquisition, Pi 5 hardware work, lab mutation, boot
publication, SSH, smoltcp adoption, public stable socket ABI acceptance, broad
socket expansion, or a phase transition.

## Findings And Dispositions

- fixed: The accepted send/recv closeout left readiness and wait queues
  explicitly rejected. This contract chooses only the next feature step:
  nonblocking readiness observation for accepted process-local socket states.
- fixed: Source review found the existing private syscall namespace ends at
  TALOS_RECV_SYSCALL = 12, so the next bounded private selector is
  TALOS_POLL_SYSCALL = 13 on the existing STABLE_SVC_IMMEDIATE = 0 path.
  This is a Talos-private task-chain selector, not Linux syscall-number or
  libc compatibility.
- fixed: The user-memory ABI shape is explicit. x0 points to a caller-owned
  array of fixed 16-byte poll entries, x1 is the entry count, x2 is flags=0,
  and x3=x4=x5=0 are reserved. Each entry is fd:u64, events:u32, revents:u32
  in native little-endian layout.
- fixed: The selected entry count bound is 1 through 8 entries. Count 0,
  unsupported flags, nonzero reserved registers, unsupported event bits, or an
  entry count above 8 returns EINVAL. Caller copy-in or copy-out failure
  returns EFAULT and leaves user results unchanged.
- fixed: Readiness bits are private constants for this chain:
  TALOS_POLL_READ = 0x01, TALOS_POLL_WRITE = 0x02,
  TALOS_POLL_HANGUP = 0x04, and TALOS_POLL_ERROR = 0x08.
- fixed: Listener read-readiness means a Listening socket has at least one
  pending local peer and a subsequent accepted nonblocking accept would not
  return EAGAIN for an empty pending queue.
- fixed: Connected or accepted socket read-readiness means the inbound FIFO has
  bytes available, or peer hangup is observable so a later recv can drain
  queued bytes and then report terminal EPIPE.
- fixed: Connected or accepted socket write-readiness means a one-byte send
  through the accepted all-or-nothing path would currently fit in the peer's
  inbound FIFO. Full peer FIFOs clear write-readiness; disconnected peers set
  hangup instead of write-readiness.
- fixed: Peer close/drop is observable through TALOS_POLL_HANGUP. If queued
  bytes remain after peer close, readiness reports both READ and HANGUP; after
  the local queue drains and the peer remains absent, readiness reports
  READ | HANGUP so a following nonblocking recv can observe the EPIPE boundary.
- fixed: Per-entry descriptor errors are represented by TALOS_POLL_ERROR in
  revents so a mixed entry array can still return other ready entries. Whole
  syscall errors remain reserved for malformed syscall arguments, malformed
  user buffers, unsupported flags, unsupported event bits, and impossible
  descriptor-table plumbing.
- not-an-issue: The existing NetworkSocketPendingQueue::len,
  NetworkSocketDescriptorTable::send_ready, recv_peek, reverse peer lookup,
  and close/drop behavior are enough to define a nonblocking readiness
  contract without introducing scheduler wait queues, TCP state, packet I/O, a
  global port registry, or hardware evidence.
- deferred: Runtime source implementation, shell-visible /bin/sockdiag
  readiness output, retained smoke evidence, blocking waits, wakeup queues,
  timeout handling, UDP/TCP payload transport, live packet I/O, SSH, public
  socket ABI acceptance, broad socket expansion, and phase transition remain
  deferred to later explicit tasks.
- removed: No runtime source cleanup or implementation was justified in this
  contract-only task.

## Accepted Contract

The next implementation task should add one private experimental selector:

- TALOS_POLL_SYSCALL = 13
- enum variant: SyscallNumber::TalosPoll
- SVC immediate: existing STABLE_SVC_IMMEDIATE = 0

The scalar arguments are:

- x0=user_poll_entries: readable and writable user-memory address for the
  entry array.
- x1=entry_count: number of 16-byte entries, from 1 through 8.
- x2=flags: must be 0.
- x3=0, x4=0, x5=0: reserved.
- return on success: number of entries whose revents is nonzero.

Each user entry has this fixed native layout:

    offset  size  field
    0       8     fd
    8       4     events
    12      4     revents

The caller initializes fd and events; the kernel overwrites revents.
Unsupported event bits are invalid for the whole call. The supported requested
events are READ, WRITE, HANGUP, and ERROR. HANGUP and ERROR may be reported
even when not requested because they are terminal/control readiness conditions.

Readiness is process-local and descriptor-backed:

- Listening sockets report READ when their pending accept queue length is
  nonzero.
- Connected and Accepted sockets report READ when their inbound FIFO has bytes
  available.
- Connected and Accepted sockets report WRITE when their unique local peer
  exists and has at least one byte of inbound FIFO capacity.
- Connected and Accepted sockets report HANGUP when the unique local peer is
  absent, closed, wrong-owner, or otherwise no longer connected by the accepted
  reverse-endpoint match.
- Queued inbound bytes after peer close report READ | HANGUP; an empty queue
  after peer close still reports READ | HANGUP so nonblocking recv can expose
  the terminal EPIPE boundary.
- OpenUnbound and Bound sockets report no readiness unless their descriptor or
  backing relationship is invalid, in which case they report ERROR.
- Non-socket, invalid, closed, wrong-owner, or missing-backing descriptors
  report per-entry ERROR.

The accepted claim is only private nonblocking readiness for process-local
socket descriptor state. This contract does not promise Linux poll(2) ABI
compatibility, libc compatibility, blocking semantics, timeouts, wakeup
registration, cross-process polling, packet readiness, TCP readiness, live
driver readiness, hardware reachability, or SSH readiness.

The selected next bounded task is
phase12-network-socket-readiness-poll-core-20260621.

## Evidence

- static source/task/evidence review:
  - src/syscall.rs currently defines the private stable SVC vocabulary from
    TALOS_NOP_SYSCALL = 0 through TALOS_RECV_SYSCALL = 12, with unsupported
    socket selectors returning ENOTSUP outside socket-table-aware dispatch.
  - src/network.rs owns NetworkSocketDescriptorTable, NetworkSocketState,
    NetworkSocketPendingQueue::len, NetworkSocketDescriptorTable::send_ready,
    recv_peek, reverse local peer lookup, and close/drop removal of socket
    backing entries.
  - tasks/2026-06-20-phase12-network-shell-sockdiag-send-recv-closeout.md
    accepts only source/unit plus host/QEMU-substitute local payload transfer
    and explicitly rejects readiness, wait queues, UDP/TCP payload transport,
    live packet I/O, hardware reachability, SSH, public socket ABI acceptance,
    broad socket expansion, and phase transition.
- Documentation updates:
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/roadmap.md.

## Rejected Claims

- No runtime implementation is accepted by this contract task.
- No shell-visible /bin/sockdiag readiness/poll output or retained smoke
  evidence is accepted.
- No blocking sleep, scheduler wait queues, wakeup registration, timeout
  handling, cancellation semantics, cross-process/global poll set, UDP/TCP
  payload transport, smoltcp integration, live packet I/O, live driver adapter,
  hardware reachability, lab mutation, boot publication, SSH, public stable
  socket ABI acceptance, broad socket expansion, or phase transition is
  accepted.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Next Action

Promote only the dependency-gated
phase12-network-socket-readiness-poll-core-20260621 task next. It may implement
only the accepted private nonblocking readiness/poll core over existing
process-local descriptor ownership, socket table state, UserMapping copy
helpers, and PosixError vocabulary. Shell diagnostics, retained smoke,
blocking waits, scheduler wait queues, timeout handling, UDP/TCP payload
transport, live packet I/O, hardware work, SSH, public socket ABI acceptance,
broad socket expansion, and phase transition remain rejected.
