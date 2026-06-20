# Phase 12.4 Socket Bind/Listen Core

Task: phase12-network-socket-bind-listen-core-20260620

Status: accepted

Classification: phase12-network-socket-bind-listen-core-accepted

## Scope

Implement only the descriptor-backed AF_INET stream socket bind/listen state
core selected by phase12-network-socket-bind-listen-abi-contract-20260620. This
adds private experimental `TALOS_BIND_SYSCALL = 7` and
`TALOS_LISTEN_SYSCALL = 8` selectors on the existing stable SVC path for
sockets created by the accepted `TALOS_SOCKET_SYSCALL = 6` open path.

This task does not add shell `/bin/sockdiag` bind/listen output, generated-root
content, send, recv, connect, accept, poll or blocking network I/O, UDP/TCP
payload transport, live packet I/O, live driver adapters, hardware
reachability, Pi 5 hardware work, hardwareTestLock acquisition, lab mutation,
boot publication, SSH, smoltcp adoption, broad socket expansion, public stable
socket ABI acceptance, or a phase transition.

## Findings And Dispositions

- fixed: `src/syscall.rs` now defines `TALOS_BIND_SYSCALL = 7`,
  `TALOS_LISTEN_SYSCALL = 8`, `SyscallNumber::TalosBind`, and
  `SyscallNumber::TalosListen`. Scalar dispatch still returns `ENOTSUP`
  without the socket-table-aware process descriptor context.
- fixed: `src/network.rs` now records socket backing state as
  `OpenUnbound`, `Bound { local_endpoint }`, or
  `Listening { local_endpoint, backlog }`, with `Ipv4Endpoint` carrying only a
  32-bit big-endian IPv4 scalar and a 16-bit port.
- fixed: Bind validates reserved arguments, 32-bit IPv4 bounds, port
  `1..=65535`, current process owner, process descriptor kind, backing socket
  owner, and `OpenUnbound` state before mutating the socket to `Bound`.
- fixed: Listen validates reserved arguments, backlog `1..=4`, current
  process owner, process descriptor kind, backing socket owner, and socket
  state before mutating a bound socket to `Listening` or updating an already
  listening socket backlog.
- fixed: Focused unit tests cover successful bind/listen transitions, repeated
  bind rejection, repeated listen backlog update, close/drop cleanup for
  listening sockets, scalar-dispatch `ENOTSUP`, malformed scalar arguments,
  listen-before-bind, non-socket descriptors, wrong-owner backing rejection,
  and all-or-nothing state preservation on failures.
- not-an-issue: The existing close path already drops the socket backing entry
  after validating process descriptor ownership; the new bound/listening state
  is contained in that same backing record, so no separate cleanup path is
  needed.
- deferred: Shell-visible `/bin/sockdiag` bind/listen reporting, send, recv,
  connect, accept, poll/blocking behavior, UDP/TCP payload transport,
  explicit accept queues, global port registry, address-conflict policy,
  smoltcp, live packet I/O, SSH, hardware work, public stable socket ABI
  acceptance, broad socket expansion, and phase transition remain deferred.
- removed: No dead-code removal was justified inside this bounded socket
  bind/listen core.

## Evidence

- Source/unit host/QEMU-substitute evidence:
  - `src/syscall.rs` adds the private bind/listen selector vocabulary,
    socket-table-aware dispatch cases, and bounded scalar argument handling.
  - `src/network.rs` adds endpoint/state storage and all-or-nothing
    `NetworkSocketDescriptorTable::bind` and `listen` transitions.
  - Unit tests in `src/syscall.rs`:
    `talos_bind_listen_records_socket_state_and_close_drops_backing` and
    `talos_bind_listen_errors_are_deterministic_and_do_not_mutate_state`.
- Accepted predecessor:
  - phase12-network-socket-bind-listen-abi-contract-20260620 accepted and
    committed at 20c42358df1ae3ef63deb72fd377f892231d627b.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, generated-root publication, shell `/bin/sockdiag` bind/listen
output, send, recv, connect, accept, UDP/TCP payload transport, SSH, smoltcp,
broad socket expansion, public stable socket ABI acceptance, or phase
transition was performed.

## Acceptance

Accepted.

The accepted evidence level is source/unit host/QEMU-substitute evidence over
private descriptor-backed AF_INET stream socket bind/listen state, endpoint and
backlog bounds, current-process descriptor ownership, close/drop cleanup, and
deterministic error mapping without live packet I/O.

Selected next task:
phase12-network-shell-sockdiag-bind-listen-core-20260620.

Commit: recorded in durable supervisor state after commit creation.
