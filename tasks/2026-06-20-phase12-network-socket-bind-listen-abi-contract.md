# Phase 12.4 Socket Bind/Listen ABI Contract

Task: phase12-network-socket-bind-listen-abi-contract-20260620

Status: accepted

Classification: phase12-network-socket-bind-listen-abi-contract-accepted

## Scope

Define the smallest descriptor-backed bind/listen state contract after the
accepted shell-visible socket open/close frontier. This task selects only a
private experimental AF_INET stream socket bind/listen ABI and backing-state
model for sockets created by the accepted `TALOS_SOCKET_SYSCALL = 6` open
path.

This contract does not add runtime behavior. It does not accept send, recv,
connect, accept, poll or blocking network I/O, UDP/TCP payload transport, live
packet I/O, live driver adapters, hardware reachability, hardwareTestLock
acquisition, Pi 5 hardware work, lab mutation, boot publication, SSH, smoltcp
adoption, broad socket expansion, public stable socket ABI acceptance, or a
phase transition.

## Findings And Dispositions

- fixed: The accepted open/close closeout ended at descriptor creation and
  close/drop only. This contract chooses the next bounded state-only socket
  step: bind a local IPv4 endpoint and transition that bound socket to
  listening state.
- fixed: The follow-up implementation should add private experimental
  `TALOS_BIND_SYSCALL = 7` and `TALOS_LISTEN_SYSCALL = 8` selectors on the
  existing `STABLE_SVC_IMMEDIATE = 0` path. These are Talos-private task-chain
  selectors, not Linux syscall-number compatibility claims.
- fixed: Bind arguments are scalar only: `x0=fd`, `x1=ipv4_be`,
  `x2=port`, and `x3..x5=0`. `ipv4_be` must fit in 32 bits and `port` must
  be in `1..=65535`; port 0 and out-of-range endpoint fields return
  `EINVAL` because this slice does not allocate ephemeral ports.
- fixed: Listen arguments are scalar only: `x0=fd`, `x1=backlog`, and
  `x2..x5=0`. Backlog must be in `1..=4`; the value is recorded for later
  accept-queue work but no accept queue, connection queue, wakeup, or packet
  transport is allocated in this slice.
- fixed: Socket backing state is explicit and mechanically implementable:
  `OpenUnbound`, `Bound { local_endpoint }`, and
  `Listening { local_endpoint, backlog }`.
- fixed: State transitions are bounded. `bind` is accepted only for an
  `OpenUnbound` socket and transitions to `Bound`; repeated bind or bind after
  listen returns `EINVAL`. `listen` is accepted for a `Bound` socket and
  transitions to `Listening`; repeated listen on an already listening socket
  updates the recorded backlog and returns success.
- fixed: Ownership checks follow the accepted open/close path. The descriptor
  must exist in the current process descriptor table, must be
  `DescriptorObjectKind::Socket`, and its backing entry must exist and be
  owned by the current process owner before bind/listen can mutate state.
- fixed: Error vocabulary is bounded to existing Talos POSIX errors:
  nonzero reserved arguments, invalid endpoint fields, invalid state, and
  listen-before-bind return `EINVAL`; invalid, closed, non-socket, missing
  owner, wrong-owner, or missing backing descriptors return `EBADF`.
- fixed: Close/drop behavior remains the accepted open/close behavior. Closing
  any unbound, bound, or listening socket descriptor through
  `TALOS_CLOSE_SYSCALL = 2` drops the process descriptor and the matching
  socket backing entry. Alias-aware reference counting and duplicate-socket fd
  lifetime semantics remain outside this contract.
- not-an-issue: The contract does not require a global port registry,
  address-conflict checks, routing policy, TCP control blocks, or packet
  queues. Endpoint uniqueness and address reuse policy are deferred until a
  later networking namespace or transport task.
- deferred: send, recv, connect, accept, poll/blocking behavior, UDP/TCP
  payload transport, explicit accept queues, smoltcp, live packet I/O, SSH,
  hardware work, public stable socket ABI acceptance, broad socket expansion,
  and phase transition remain deferred.
- removed: No runtime source cleanup or implementation was justified in this
  contract-only task.

## Accepted Contract

The next implementation task should add two private experimental syscall
selectors:

- `TALOS_BIND_SYSCALL = 7`
- `TALOS_LISTEN_SYSCALL = 8`
- enum variants: `SyscallNumber::TalosBind` and `SyscallNumber::TalosListen`
- SVC immediate: existing `STABLE_SVC_IMMEDIATE = 0`

Bind uses scalar arguments:

- `x0=fd`: current-process descriptor returned by the accepted
  `TALOS_SOCKET_SYSCALL = 6` open path.
- `x1=ipv4_be`: 32-bit IPv4 address encoded as a canonical big-endian integer
  in the low 32 bits of the scalar.
- `x2=port`: TCP-style local port number in `1..=65535`.
- `x3=0`, `x4=0`, `x5=0`: reserved.
- return on success: `0`.

Listen uses scalar arguments:

- `x0=fd`: current-process descriptor for a bound AF_INET stream socket.
- `x1=backlog`: requested backlog in `1..=4`.
- `x2=0`, `x3=0`, `x4=0`, `x5=0`: reserved.
- return on success: `0`.

The socket backing entry should retain the accepted owner/domain/type/protocol
fields and add local state:

- `OpenUnbound`
- `Bound { local_endpoint: Ipv4Endpoint }`
- `Listening { local_endpoint: Ipv4Endpoint, backlog: u8 }`

`Ipv4Endpoint` should record `ipv4_be: u32` and `port: u16` only. This is
state for deterministic descriptor tests and future shell diagnostics; it is
not a route, ARP, TCP, or hardware reachability claim.

The accepted implementation order should check reserved arguments and scalar
endpoint/backlog bounds first, then current process owner/descriptor/backing
identity, then state-transition validity. All mutation must be all-or-nothing:
failed bind/listen calls leave the previous socket state unchanged.

The selector numbers and scalar layout remain private experimental ABI details
for this task chain. They preserve a conventional bind/listen shape without
freezing libc, Linux ABI compatibility, public stable socket ABI, packet I/O,
or TCP behavior.

## Evidence

- static source/task/evidence review:
  - `src/syscall.rs` already owns the stable SVC scalar syscall vocabulary,
    `SyscallNumber`, `SyscallArguments`, `SyscallReturn`, and the accepted
    socket-table-aware process descriptor dispatch path.
  - `src/network.rs` already owns `NetworkSocketDescriptorTable`,
    `NetworkSocketDescriptor`, `NetworkSocket`, and the accepted
    `AF_INET=2`, `SOCK_STREAM=1`, `protocol=0` constants.
  - `src/posix.rs` already defines `DescriptorObjectKind::Socket`, process
    descriptor ownership, and existing POSIX error names used by this
    contract.
  - `tasks/2026-06-20-phase12-network-socket-open-close-abi-contract.md`,
    `tasks/2026-06-20-phase12-network-socket-open-close-core.md`,
    `tasks/2026-06-20-phase12-network-shell-sockdiag-open-close-core.md`,
    `tasks/2026-06-20-phase12-network-shell-sockdiag-open-close-smoke.md`,
    and
    `tasks/2026-06-20-phase12-network-shell-sockdiag-open-close-closeout.md`
    record the accepted predecessor chain this contract extends.
  - `tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/` retains the
    host/QEMU-substitute shell evidence for `/bin/sockdiag` over the accepted
    VFS/userspace socket open/close path.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No runtime source implementation, generated userland change, shell command
change, smoke harness, retained execution transcript, Pi 5 hardware run,
hardwareTestLock acquisition, boot archive publication, lab mutation, power
cycle, live driver adapter, live packet I/O, hardware reachability, send,
recv, connect, accept, UDP/TCP payload transport, SSH, smoltcp, broad socket
expansion, public stable socket ABI acceptance, or phase transition was
performed.

## Acceptance

Accepted.

The accepted boundary is a private, descriptor-backed AF_INET stream
bind/listen state contract only. It is precise enough for the follow-up core
to implement endpoint recording, state transitions, backlog bounds,
deterministic error mapping, and close/drop cleanup without adding live packet
I/O or transport behavior.

Selected next task:
phase12-network-socket-bind-listen-core-20260620.

Commit: recorded in durable supervisor state after commit creation.
