# Phase 12.4 Socket Send/Recv Core

Task: phase12-network-socket-send-recv-core-20260620

Status: accepted

Classification: phase12-network-socket-send-recv-core-accepted

## Scope

Implement only the accepted private local send/recv core selected by
phase12-network-socket-send-recv-abi-contract-20260620:

- `TALOS_SEND_SYSCALL = 11`
- `TALOS_RECV_SYSCALL = 12`
- socket-table-aware dispatch only
- caller-buffer copies through existing user-memory helpers
- 64-byte inbound queues owned by Connected and Accepted socket states
- all-or-nothing nonblocking send
- nonblocking short-read recv
- deterministic EAGAIN, ENOSPC, EFAULT, EPIPE, EINVAL, and EBADF boundaries

This task does not add shell `/bin/sockdiag` send/recv output, retained smoke
evidence, poll or blocking network I/O, readiness, wait queues, UDP/TCP payload
transport, live packet I/O, live driver adapters, hardware reachability,
hardwareTestLock acquisition, Pi 5 hardware work, lab mutation, boot
publication, SSH, smoltcp adoption, cross-process or global port semantics,
broad socket expansion, public stable socket ABI acceptance, or a phase
transition.

## Findings And Dispositions

- fixed: `src/syscall.rs` now defines private `TALOS_SEND_SYSCALL = 11` and
  `TALOS_RECV_SYSCALL = 12` selectors with `SyscallNumber::TalosSend` and
  `SyscallNumber::TalosRecv`. Scalar/default dispatch remains ENOTSUP outside
  socket-table-aware process descriptor dispatch.
- fixed: `src/network.rs` now gives each Connected and Accepted socket a
  `NetworkSocketPayloadQueue` with `SOCKET_PAYLOAD_QUEUE_CAPACITY = 64`.
  Payload queues are descriptor-backed local state only.
- fixed: Send validates reserved flags, descriptor identity, socket ownership,
  connected/accepted state, unique reverse-endpoint peer, queue capacity, and
  readable caller memory before appending bytes to the peer inbound queue.
- fixed: Recv validates reserved flags, descriptor identity, connected/accepted
  state, local queue state, and writable caller memory before consuming bytes
  from the caller socket's inbound queue.
- fixed: Unit tests cover bidirectional payload transfer, short reads,
  scalar-dispatch ENOTSUP, empty receive EAGAIN, malformed flags, non-socket
  descriptors, non-connected descriptors, oversize sends, full peer queues,
  caller-buffer EFAULT without queue mutation, close/drop cleanup, queued-byte
  drain after peer close, and EPIPE after the disconnected peer queue is empty.
- not-an-issue: The accepted reverse-endpoint peer lookup remains sufficient
  for this process-local payload slice; no global port registry, cross-process
  namespace, TCP state, packet queue, readiness wait queue, or live device
  adapter is needed.
- deferred: Shell-visible `/bin/sockdiag` send/recv diagnostics, retained smoke
  evidence, poll/blocking behavior, readiness/wait queues, UDP/TCP payload
  transport, cross-process sockets, live packet I/O, smoltcp, SSH, hardware
  work, public stable socket ABI acceptance, broad socket expansion, and phase
  transition remain deferred.
- removed: No dead code or broad refactor outside the accepted local socket
  payload path was justified.

## Evidence

- source anchors:
  - `src/syscall.rs`: private selector constants, `SyscallNumber` mapping,
    socket-table-aware `TalosSend`/`TalosRecv` dispatch, caller-buffer copy
    ordering, and unit tests.
  - `src/network.rs`: `NetworkSocketPayloadQueue`, Connected/Accepted
    `recv_queue`, `send_ready`, `send`, `recv_peek`, `recv_commit`, and
    reverse-endpoint peer lookup.
  - `src/local_command_loop.rs`: existing sockdiag connect/accept state checks
    updated for the newly explicit empty payload queues.
- source/unit tests:
  - `talos_send_recv_moves_local_payload_bytes_bidirectionally`
  - `talos_send_recv_errors_are_deterministic_and_all_or_nothing`
  - `talos_send_recv_reports_disconnected_peer_after_queue_drain`
- documentation:
  - `docs/src/project/phase12-networking-ssh.md`
  - `docs/src/roadmap.md`

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- diff validation: `git diff --check` passed.
- docs build: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff validation: `git diff --cached --check` passed before commit.

No shell `/bin/sockdiag` send/recv output, retained smoke transcript, Pi 5
hardware run, hardwareTestLock acquisition, boot archive publication, lab
mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, UDP/TCP payload transport, SSH, smoltcp, broad socket expansion,
public stable socket ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

The accepted boundary is source/unit host/QEMU-substitute evidence for private
descriptor-backed local payload transfer between accepted Connected and
Accepted AF_INET stream sockets only.

Selected next task:
phase12-network-shell-sockdiag-send-recv-core-20260620.

Commit: recorded in durable supervisor state after commit creation.
