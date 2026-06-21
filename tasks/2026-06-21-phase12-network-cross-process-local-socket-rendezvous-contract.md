# Phase 12.4 Cross-Process Local Socket Rendezvous Contract

Task: phase12-network-cross-process-local-socket-rendezvous-contract-20260621

Status: accepted

Classification: phase12-network-cross-process-local-socket-rendezvous-contract-accepted

## Scope

Define the smallest useful private cross-process local socket rendezvous after
the accepted process-local socket open/bind/listen/connect/accept/send/recv,
readiness, and bounded blocking poll wait frontier. This contract selects a
bounded kernel-local listener and connection table so two distinct process
descriptor stores can rendezvous through the existing descriptor-backed socket
vocabulary while preserving per-process descriptor ownership.

This is a source/task/docs contract only. It does not add runtime
implementation, shell-visible behavior, retained smoke evidence, UDP/TCP
payload transport, smoltcp integration, live driver adapters, live packet I/O,
Pi 5 hardware work, lab mutation, boot publication, hardware reachability,
SSH, public stable socket ABI acceptance, broad socket expansion, or a phase
transition.

## Findings And Dispositions

- fixed: The accepted process-local frontier deliberately rejected
  cross-process/global socket behavior. This contract chooses the next bounded
  feature step: private local rendezvous across two distinct process
  descriptor stores only.
- fixed: Source review found the current local connect and reverse-peer lookup
  are owner-local. A cross-process implementation must not pass by reusing the
  same owner for both endpoints or by relabeling the existing process-local
  tests as global evidence.
- fixed: The ownership boundary is explicit. Process descriptor tables remain
  per-owner; socket file descriptors are never transferred across processes.
  Connect and accept create or reference socket backing entries owned by their
  own process, then join them through a bounded kernel-local connection record.
- fixed: The listener boundary is explicit. A bound/listening socket publishes
  one private local listener keyed by local endpoint and listener owner/socket
  backing descriptor. Cross-process connect may target that listener by
  endpoint, but accepted descriptor creation still occurs only in the listener
  process descriptor store during accept.
- fixed: The connection boundary is explicit. A pending connection records the
  client owner, client socket backing descriptor, listener owner, listener
  backing descriptor, synthetic client endpoint, server endpoint, and a bounded
  connection id. Accept consumes that pending record and creates a server-owned
  accepted socket backing descriptor paired to the same connection id.
- fixed: Payload ownership is connection-local rather than process-global. Each
  accepted connection owns two bounded byte queues, one in each direction.
  Send enqueues into the peer direction only after descriptor ownership, state,
  capacity, and caller-copy checks; recv consumes from the caller's inbound
  direction only after copy-out succeeds.
- fixed: Readiness and wait wakeups are the accepted socket readiness semantics
  lifted across the connection record: listener pending-accept READ, inbound
  payload READ, peer queue capacity WRITE, peer close/drop HANGUP, descriptor
  invalidation ERROR, and finite timeout for bounded poll wait.
- fixed: Cleanup is explicit. Close/drop/exit of a listener removes future
  acceptance for that listener and wakes pending clients with hangup/error.
  Close/drop/exit of one connected endpoint marks that side gone, wakes the
  peer with HANGUP while preserving queued bytes until drained, and releases
  the connection slot only after both endpoints are closed and queues are
  drained.
- fixed: Capacity/error behavior is bounded. Listener-table exhaustion,
  pending-queue exhaustion, accepted-socket backing exhaustion, connection
  slot exhaustion, and payload queue exhaustion fail deterministically through
  existing PosixError vocabulary such as ENOSPC, EAGAIN, EEXIST, EBADF,
  EINVAL, EPIPE, EBUSY, EFAULT, and ENOTSUP as appropriate.
- not-an-issue: The existing ProcessDescriptorStore,
  DescriptorObjectKind::Socket, scheduler TaskState/ProcessOwnerId,
  SocketPollWaitTable, and VFS/userspace /bin/sockdiag diagnostic path are
  sufficient source owners for a later core implementation. This contract does
  not need a public Linux socket ABI or hardware evidence.
- deferred: Runtime implementation, /bin/sockdiag cross-process diagnostics,
  retained smoke evidence, public socket/libc ABI acceptance, UDP/TCP payload
  transport, smoltcp integration, live packet I/O, hardware reachability, SSH,
  broad socket expansion, and phase transition remain deferred to later
  explicit tasks.
- removed: No runtime source cleanup or implementation was justified in this
  contract-only task.

## Accepted Contract

The next implementation task should keep the existing private syscall
vocabulary and descriptor-backed socket surface, then add only the internal
cross-process local rendezvous state needed by that surface. No new public
syscall number is selected by this contract.

The core model is:

- A bounded LocalSocketRendezvousTable owns listener records, pending
  connection records, and connected-pair records for private local sockets.
- Listener records are keyed by local endpoint and point to the listener owner
  plus listener socket backing descriptor. Duplicate active listeners for the
  same endpoint fail deterministically.
- Pending connection records contain the client owner, client socket backing
  descriptor, listener owner, listener descriptor, synthetic client endpoint,
  server endpoint, and a connection id. They are bounded by the listener
  backlog and by total pending capacity.
- Accept consumes one pending record, allocates a server-owned socket backing
  descriptor, installs a descriptor-table entry in the accepting process, and
  joins the accepted socket to the pending client's connection id.
- Connected-pair records carry the client owner/socket descriptor, server
  owner/socket descriptor, local/remote endpoints, close state for each side,
  and two bounded payload queues.

Ownership rules:

- Every process-visible fd remains in that process's own descriptor table.
- Every socket backing descriptor records exactly one owning ProcessOwnerId.
- Connect never creates an fd in the listener process.
- Accept never exposes or mutates the client's process descriptor table.
- Send, recv, poll, poll-wait, and close first prove the caller owns the
  process descriptor and backing socket before touching rendezvous state.
- Process exit cleanup closes all socket descriptors owned by that process and
  applies the same wake/hangup/capacity-release rules as explicit close.

Readiness and bounded wait semantics:

- A listening socket reports READ and wakes READ waiters when at least one
  pending cross-process connection exists.
- A connected or accepted socket reports READ when its inbound queue contains
  bytes, or when peer close/drop means recv can observe the terminal EPIPE
  boundary after draining.
- A connected or accepted socket reports WRITE when the peer side exists and
  the peer inbound queue has capacity for at least one byte.
- Peer close/drop reports HANGUP. Queued bytes after peer close report
  READ | HANGUP until drained; an empty queue after peer close still reports
  READ | HANGUP so recv can expose the existing EPIPE boundary.
- Descriptor close/drop, process exit cleanup, missing backing entries,
  wrong-owner discovery, or connection-table invalidation wakes affected waits
  with ERROR where possible.
- Bounded TALOS_POLL_WAIT behavior remains finite and scheduler-owned. It must
  prove blocked-to-runnable transitions through accepted cross-process local
  socket state changes or timeout, not through a retry loop.

Error and capacity behavior:

- ENOSPC covers listener table, pending queue, accepted socket backing,
  connection table, descriptor table, and payload queue exhaustion.
- EEXIST covers duplicate active listener endpoints when the endpoint registry
  can identify the conflict.
- EAGAIN remains the empty nonblocking accept/recv boundary.
- EPIPE remains the terminal peer-gone send/recv boundary.
- EBADF covers invalid, closed, wrong-owner, or non-socket descriptors.
- EINVAL covers malformed state transitions, unsupported address/socket shapes,
  unsupported flags, and impossible scalar arguments.
- EFAULT covers caller-buffer copy failure.
- ENOTSUP remains the fail-closed path for dispatch surfaces that do not carry
  descriptor, socket, rendezvous, wait, and user-memory context.

The accepted claim is only a private source/task/docs contract for
cross-process local socket rendezvous. It does not promise Linux socket ABI
compatibility, libc compatibility, stable public syscall behavior,
cross-machine reachability, Ethernet/IP/ARP/ICMP/UDP/TCP behavior, live packet
I/O, hardware reachability, SSH readiness, or phase transition.

The selected next bounded task is
phase12-network-cross-process-local-socket-rendezvous-core-20260621.

## Evidence

- static source/task/doc review:
  - src/network.rs owns NetworkSocketDescriptorTable, owner-checked socket
    backing entries, local bind/listen/connect/accept/send/recv/readiness, and
    process-local peer lookup that the follow-up must replace or extend with a
    bounded cross-process rendezvous table.
  - src/syscall.rs owns the private stable socket syscall vocabulary,
    socket-table-aware dispatch, TALOS_POLL_SYSCALL = 13, and
    TALOS_POLL_WAIT_SYSCALL = 14 with SocketPollWaitTable wake/timeout
    behavior.
  - src/posix.rs owns ProcessDescriptorStore, per-owner descriptor tables,
    DescriptorObjectKind::Socket, user-copy helpers, and the PosixError
    vocabulary used by this private contract.
  - src/scheduler.rs owns ProcessOwnerId, TaskId, TaskState::Blocked, and
    runnable transitions used by the accepted bounded wait path.
  - src/local_command_loop.rs owns the VFS/userspace /bin/sockdiag diagnostic
    surface that later tasks may extend without adding fake kernel-backed shell
    commands.
- Documentation updates:
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/roadmap.md.

## Rejected Claims

- No runtime implementation is accepted by this contract task.
- No shell-visible /bin/sockdiag cross-process output or retained smoke
  evidence is accepted.
- No UDP/TCP payload transport, smoltcp integration, live driver adapter, live
  packet I/O, Pi 5 hardware run, lab mutation, boot publication,
  generated-root publication, hardware reachability, SSH, public stable socket
  ABI acceptance, broad socket expansion, or phase transition is accepted.
- No cross-machine, Ethernet, IP, ARP, ICMP, UDP, TCP, DNS, routing, or packet
  device behavior is accepted.

## Validation

- static source/task/doc review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Next Action

Promote only the dependency-gated
phase12-network-cross-process-local-socket-rendezvous-core-20260621 task next.
It may implement only the accepted private cross-process local socket
rendezvous core over bounded kernel-local listener/connection state, distinct
process descriptor ownership, accepted readiness/wait semantics, and
deterministic cleanup/capacity behavior. Shell diagnostics, retained smoke,
UDP/TCP, smoltcp, live packet I/O, hardware work, SSH, public ABI acceptance,
broad expansion, and phase transition remain rejected.
