# Phase 12.4 Socket Connect/Accept ABI Contract

Task: phase12-network-socket-connect-accept-abi-contract-20260620

Status: accepted

Classification: phase12-network-socket-connect-accept-abi-contract-accepted

## Scope

Define the smallest descriptor-backed local connect/accept handshake contract
after the accepted shell-visible socket bind/listen frontier. This task selects
only private experimental AF_INET stream connect/accept selectors and backing
state for sockets created by TALOS_SOCKET_SYSCALL = 6 and listeners created by
TALOS_BIND_SYSCALL = 7 plus TALOS_LISTEN_SYSCALL = 8.

This contract does not add runtime behavior. It does not accept send, recv,
poll or blocking network I/O, UDP/TCP payload transport, live packet I/O, live
driver adapters, hardware reachability, hardwareTestLock acquisition, Pi 5
hardware work, lab mutation, boot publication, SSH, smoltcp adoption, broad
socket expansion, public stable socket ABI acceptance, or a phase transition.

## Findings And Dispositions

- fixed: The accepted bind/listen closeout ended at descriptor-backed listening
  state only. This contract chooses the next bounded state-only step: a local
  connect from one current-process socket to one current-process listener, plus
  accept of the queued local peer into a new server-side socket descriptor.
- fixed: The follow-up implementation should add private experimental
  TALOS_CONNECT_SYSCALL = 9 and TALOS_ACCEPT_SYSCALL = 10 selectors on the
  existing STABLE_SVC_IMMEDIATE = 0 path. These are Talos-private task-chain
  selectors, not Linux syscall-number compatibility claims.
- fixed: Connect arguments are scalar only: x0=fd, x1=ipv4_be, x2=port, and
  x3..x5=0. The endpoint identifies an already listening socket owned by the
  current process. ipv4_be must fit in 32 bits and port must be in 1..=65535.
- fixed: Accept arguments are scalar only: x0=listener_fd and x1..x5=0. On
  success it returns the lowest available current-process descriptor for a new
  server-side accepted socket backing entry.
- fixed: Listener lookup is intentionally local and deterministic. The
  implementation should scan only the fixed-capacity socket table for a
  Listening socket owned by the current process whose local endpoint matches
  the connect target. Zero or multiple matching listeners return EINVAL and
  leave all socket state unchanged.
- fixed: Accept queue behavior is bounded by the accepted listen backlog.
  Connect succeeds only when the listener's pending local peer queue length is
  below the recorded backlog. Full queues return ENOSPC; accept on an empty
  queue returns EAGAIN.
- fixed: The client endpoint is synthetic local state only:
  127.0.0.1:(49152 + client_socket_descriptor.raw()). If the computed port
  would exceed 65535, connect returns ENOSPC without mutation. This gives
  deterministic shell/test state without accepting ephemeral port allocation,
  routing policy, or TCP behavior.
- fixed: Socket backing state is explicit and mechanically implementable:
  keep OpenUnbound, Bound, and Listening, and add a listener pending-peer queue
  plus Connected and Accepted local peer states. The accepted server-side
  socket records the listener local endpoint and synthetic client endpoint;
  the client socket records the same endpoints from its side.
- fixed: Ownership checks follow the accepted socket path. Client and listener
  descriptors must exist in the current process descriptor table, must be
  DescriptorObjectKind::Socket, and their backing entries must be owned by the
  current process before connect/accept can mutate or allocate state.
- fixed: Error vocabulary is bounded to existing Talos POSIX errors: nonzero
  reserved arguments, invalid endpoint fields, invalid state, no unique local
  listener, and accept on a non-listener return EINVAL; full pending queue,
  socket backing capacity exhaustion, synthetic endpoint exhaustion, or no
  process descriptor slot for accept return ENOSPC or EMFILE as appropriate;
  empty accept queue returns EAGAIN; invalid, closed, non-socket,
  missing-owner, wrong-owner, or missing-backing descriptors return EBADF.
- fixed: All mutation is all-or-nothing. Connect must not mark the client
  connected unless it also queues the pending peer on exactly one listener.
  Accept must not dequeue the pending peer unless it also creates both the
  server-side socket backing entry and process descriptor. Failures leave prior
  socket state unchanged.
- fixed: Close/drop cleanup remains descriptor-owned. Closing a listening
  socket drops its queued local peers and does not mutate already connected
  client sockets. Closing a connected client or accepted server descriptor
  drops only that descriptor's backing state. Peer shutdown, half-close, and
  payload readiness are deferred.
- not-an-issue: This contract does not require a global port registry,
  address-conflict policy, TCP control block, packet queue, wake queue, or
  cross-process listener namespace. The handshake is process-local descriptor
  state for deterministic implementation and shell diagnostics.
- deferred: send, recv, poll/blocking behavior, UDP/TCP payload transport,
  cross-process sockets, live packet I/O, smoltcp, SSH, hardware work, public
  stable socket ABI acceptance, broad socket expansion, and phase transition
  remain deferred.
- removed: No runtime source cleanup or implementation was justified in this
  contract-only task.

## Accepted Contract

The next implementation task should add two private experimental syscall
selectors:

- TALOS_CONNECT_SYSCALL = 9
- TALOS_ACCEPT_SYSCALL = 10
- enum variants: SyscallNumber::TalosConnect and SyscallNumber::TalosAccept
- SVC immediate: existing STABLE_SVC_IMMEDIATE = 0

Connect uses scalar arguments:

- x0=fd: current-process descriptor for an open unbound AF_INET stream socket
  returned by the accepted TALOS_SOCKET_SYSCALL = 6 open path.
- x1=ipv4_be: 32-bit IPv4 address encoded as a canonical big-endian integer in
  the low 32 bits of the scalar.
- x2=port: target local listener port in 1..=65535.
- x3=0, x4=0, x5=0: reserved.
- return on success: 0.

Accept uses scalar arguments:

- x0=listener_fd: current-process descriptor for a listening AF_INET stream
  socket.
- x1=0, x2=0, x3=0, x4=0, x5=0: reserved.
- return on success: new current-process descriptor for the accepted
  server-side socket.

The socket backing entry should retain owner/domain/type/protocol and extend
local state from the accepted bind/listen contract:

- OpenUnbound
- Bound { local_endpoint: Ipv4Endpoint }
- Listening { local_endpoint: Ipv4Endpoint, backlog: u8, pending: ... }
- Connected { local_endpoint: Ipv4Endpoint, remote_endpoint: Ipv4Endpoint }
- Accepted { local_endpoint: Ipv4Endpoint, remote_endpoint: Ipv4Endpoint }

The pending listener queue records only synthetic local peer endpoints and the
client backing descriptor needed for deterministic state tests. It does not
store bytes, TCP sequence state, packet references, readiness waiters, or
hardware state.

The accepted implementation order should check reserved arguments and scalar
endpoint bounds first, then current owner/descriptor/backing identity, then
state-transition validity, listener uniqueness, queue capacity, and allocation
capacity. All mutation must be all-or-nothing.

The selector numbers, synthetic local endpoint rule, and scalar layout remain
private experimental ABI details for this task chain. They preserve a
conventional connect/accept shape without freezing libc, Linux ABI
compatibility, public stable socket ABI, packet I/O, or TCP behavior.

## Evidence

- static source/task/evidence review:
  - src/syscall.rs owns the stable SVC scalar syscall vocabulary,
    SyscallNumber, SyscallArguments, SyscallReturn, and the accepted
    socket-table-aware process descriptor dispatch path.
  - src/network.rs owns NetworkSocketDescriptorTable, NetworkSocketDescriptor,
    NetworkSocket, NetworkSocketState, the accepted AF_INET stream tuple,
    bind/listen state transitions, and fixed backlog bounds.
  - src/posix.rs defines DescriptorObjectKind::Socket, process descriptor
    ownership, EBADF, EINVAL, EAGAIN, ENOSPC, EMFILE, and ENOTSUP.
  - tasks/2026-06-20-phase12-network-socket-bind-listen-abi-contract.md,
    tasks/2026-06-20-phase12-network-socket-bind-listen-core.md,
    tasks/2026-06-20-phase12-network-shell-sockdiag-bind-listen-core.md,
    tasks/2026-06-20-phase12-network-shell-sockdiag-bind-listen-smoke.md, and
    tasks/2026-06-20-phase12-network-shell-sockdiag-bind-listen-closeout.md
    record the accepted predecessor chain this contract extends.
  - tasks/evidence/2026-06-20-shell-sockdiag-bind-listen-smoke/ retains the
    host/QEMU-substitute shell evidence for /bin/sockdiag over the accepted
    VFS/userspace socket bind/listen path.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No runtime source implementation, generated userland change, shell command
change, smoke harness, retained execution transcript, Pi 5 hardware run,
hardwareTestLock acquisition, boot archive publication, lab mutation, power
cycle, live driver adapter, live packet I/O, hardware reachability, send,
recv, UDP/TCP payload transport, SSH, smoltcp, broad socket expansion, public
stable socket ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

The accepted boundary is a private, descriptor-backed AF_INET stream local
connect/accept state contract only. It is precise enough for the follow-up core
to implement listener lookup, pending peer queue bounds, client/server socket
state transitions, accepted descriptor creation, deterministic error mapping,
and close/drop cleanup without adding payload I/O, live packet I/O, or
transport behavior.

Selected next task:
phase12-network-socket-connect-accept-core-20260620.

Commit: recorded in durable supervisor state after commit creation.
