# Phase 12.4 Socket Send/Recv ABI Contract

Task: phase12-network-socket-send-recv-abi-contract-20260620

Status: accepted

Classification: phase12-network-socket-send-recv-abi-contract-accepted

## Scope

Define the smallest descriptor-backed local payload-transfer contract after the
accepted shell-visible socket connect/accept frontier. This task selects only
private experimental AF_INET stream `send` and `recv` selectors for sockets
already connected or accepted through the accepted local socket chain.

This contract does not add runtime behavior. It does not accept shell
`/bin/sockdiag` send/recv output, poll or blocking network I/O, readiness,
wait queues, UDP/TCP payload transport, live packet I/O, live driver adapters,
hardware reachability, hardwareTestLock acquisition, Pi 5 hardware work, lab
mutation, boot publication, SSH, smoltcp adoption, cross-process/global port
semantics, broad socket expansion, public stable socket ABI acceptance, or a
phase transition.

## Findings And Dispositions

- fixed: The accepted connect/accept closeout ended at descriptor-backed local
  handshake state with no payload bytes. This contract chooses the next bounded
  feature step: byte payload transfer only between the accepted connected
  client socket and accepted server-side socket.
- fixed: The follow-up implementation should add private experimental
  `TALOS_SEND_SYSCALL = 11` and `TALOS_RECV_SYSCALL = 12` selectors on the
  existing `STABLE_SVC_IMMEDIATE = 0` path. These are Talos-private task-chain
  selectors, not Linux syscall-number compatibility claims.
- fixed: Send arguments are scalar only: `x0=fd`, `x1=user_buffer_start`,
  `x2=len`, `x3=flags=0`, and `x4=x5=0`. Send copies bytes from caller
  readable user memory into the peer socket's inbound payload queue.
- fixed: Recv arguments are scalar only: `x0=fd`, `x1=user_buffer_start`,
  `x2=len`, `x3=flags=0`, and `x4=x5=0`. Recv copies bytes from the local
  socket's inbound payload queue into caller writable user memory.
- fixed: Payload queue ownership is explicit. Each `Connected` or `Accepted`
  socket owns a fixed-capacity inbound byte queue. Sending to a peer appends to
  the peer inbound queue; receiving consumes from the caller socket's inbound
  queue. The queue is descriptor-backed local state, not a TCP receive buffer,
  NIC ring, packet queue, or smoltcp object.
- fixed: Queue bounds and partial-I/O policy are explicit. The next core
  should introduce `SOCKET_PAYLOAD_QUEUE_CAPACITY = 64` bytes per connected or
  accepted socket. Send is nonblocking and all-or-nothing: `len=0` returns 0,
  `len > 64` or insufficient peer queue capacity returns `ENOSPC`, and no
  partial send is accepted. Recv is nonblocking and may return a short positive
  byte count: `len=0` returns 0, an empty queue returns `EAGAIN` while the
  peer still exists, otherwise recv copies `min(len, queued_bytes)` bytes and
  consumes exactly that count.
- fixed: Peer matching remains local and deterministic. A connected or accepted
  socket's peer is the unique current-process socket whose local endpoint is
  this socket's remote endpoint and whose remote endpoint is this socket's
  local endpoint. Missing, wrong-owner, duplicate, closed, or non-connected
  peers return `EPIPE` for send; recv may drain already queued bytes and then
  returns `EPIPE` once the queue is empty and no peer exists.
- fixed: State and copy failures are deterministic and all-or-nothing. Invalid
  reserved arguments, invalid state, or non-connected/non-accepted sockets
  return `EINVAL`; invalid, closed, non-socket, missing-owner, wrong-owner, or
  missing-backing descriptors return `EBADF`; caller-buffer validation failures
  return `EFAULT`; empty receive queues return `EAGAIN`; full send queues or
  oversize sends return `ENOSPC`; disconnected peers return `EPIPE`.
- fixed: The implementation order should validate reserved arguments and
  length bounds, current owner/descriptor/backing identity, socket state, peer
  identity where required, queue capacity/availability, and caller buffer
  access before mutating any queue. Failed calls leave socket states and queue
  contents unchanged.
- not-an-issue: The accepted `Connected` and `Accepted` endpoint state is
  enough to locate the local peer by reverse endpoint matching. No global port
  registry, cross-process namespace, routing policy, packet queue, readiness
  wait queue, TCP control block, or live network device is needed for this
  bounded payload slice.
- deferred: Shell-visible send/recv diagnostics, retained smoke evidence,
  poll/blocking behavior, readiness/wait queues, UDP/TCP payload transport,
  cross-process sockets, live packet I/O, smoltcp, SSH, hardware work, public
  stable socket ABI acceptance, broad socket expansion, and phase transition
  remain deferred.
- removed: No runtime source cleanup or implementation was justified in this
  contract-only task.

## Accepted Contract

The next implementation task should add two private experimental syscall
selectors:

- `TALOS_SEND_SYSCALL = 11`
- `TALOS_RECV_SYSCALL = 12`
- enum variants: `SyscallNumber::TalosSend` and `SyscallNumber::TalosRecv`
- SVC immediate: existing `STABLE_SVC_IMMEDIATE = 0`

Send uses scalar arguments:

- `x0=fd`: current-process descriptor for a `Connected` or `Accepted` AF_INET
  stream socket.
- `x1=user_buffer_start`: readable user-memory address containing bytes to
  send.
- `x2=len`: byte count to transfer.
- `x3=flags`: must be 0.
- `x4=0`, `x5=0`: reserved.
- return on success: bytes sent, equal to `len`.

Recv uses scalar arguments:

- `x0=fd`: current-process descriptor for a `Connected` or `Accepted` AF_INET
  stream socket.
- `x1=user_buffer_start`: writable user-memory address to receive bytes.
- `x2=len`: maximum byte count to transfer.
- `x3=flags`: must be 0.
- `x4=0`, `x5=0`: reserved.
- return on success: bytes received, which may be less than `len`.

The socket backing entry should retain owner/domain/type/protocol and the
accepted local endpoint state while extending only `Connected` and `Accepted`
states with an inbound byte queue:

- `Connected { local_endpoint, remote_endpoint, recv_queue }`
- `Accepted { local_endpoint, remote_endpoint, recv_queue }`

The queue is a fixed 64-byte FIFO per socket. Send appends to the peer's
`recv_queue`; recv removes bytes from the caller's own `recv_queue`. Queue
contents are local descriptor state and carry no packet, TCP sequence,
readiness, interrupt, DMA, driver, or hardware meaning.

The selector numbers, scalar layout, nonblocking policy, 64-byte queue bound,
all-or-nothing send, and short-read recv behavior remain private experimental
ABI details for this task chain. They preserve a conventional send/recv shape
without freezing libc, Linux ABI compatibility, public stable socket ABI,
packet I/O, or TCP behavior.

## Evidence

- static source/task/evidence review:
  - `src/syscall.rs` currently owns stable SVC scalar syscall vocabulary,
    `SyscallNumber`, `SyscallArguments`, `SyscallReturn`, POSIX errno mapping,
    user-memory copy helpers through `src/posix.rs`, and the accepted
    socket-table-aware process descriptor dispatch path.
  - `src/network.rs` owns `NetworkSocketDescriptorTable`,
    `NetworkSocketDescriptor`, `Ipv4Endpoint`, `NetworkSocketState`, accepted
    `Connected` and `Accepted` local peer states, synthetic endpoint policy,
    and current fixed-capacity socket backing storage.
  - `src/posix.rs` owns `DescriptorObjectKind::Socket`, process descriptor
    ownership, user-memory `copy_from_user`/`copy_to_user`, and the POSIX
    errors used by this contract.
  - `tasks/2026-06-20-phase12-network-socket-connect-accept-abi-contract.md`,
    `tasks/2026-06-20-phase12-network-socket-connect-accept-core.md`,
    `tasks/2026-06-20-phase12-network-shell-sockdiag-connect-accept-core.md`,
    `tasks/2026-06-20-phase12-network-shell-sockdiag-connect-accept-smoke.md`,
    and
    `tasks/2026-06-20-phase12-network-shell-sockdiag-connect-accept-closeout.md`
    record the accepted predecessor chain this contract extends.
  - `tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/` retains
    the host/QEMU-substitute shell evidence for `/bin/sockdiag` over accepted
    local connect/accept state.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No runtime source implementation, generated userland change, shell command
change, smoke harness, retained execution transcript, Pi 5 hardware run,
hardwareTestLock acquisition, boot archive publication, lab mutation, power
cycle, live driver adapter, live packet I/O, hardware reachability, UDP/TCP
payload transport, SSH, smoltcp, broad socket expansion, public stable socket
ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

The accepted boundary is a private, descriptor-backed local AF_INET stream
send/recv payload contract only. It is precise enough for the follow-up core
to implement selector dispatch, caller-buffer copies, local inbound queues,
peer lookup, deterministic nonblocking behavior, queue cleanup through existing
close/drop, and focused source/unit tests without adding live packet I/O or
transport behavior.

Selected next task:
phase12-network-socket-send-recv-core-20260620.

Commit: recorded in durable supervisor state after commit creation.
