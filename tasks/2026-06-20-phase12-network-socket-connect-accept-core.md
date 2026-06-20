# Phase 12.4 Socket Connect/Accept Core

Task: phase12-network-socket-connect-accept-core-20260620

Status: accepted

Classification: phase12-network-socket-connect-accept-core-accepted

## Scope

Implement only the private descriptor-backed local connect/accept core selected
by phase12-network-socket-connect-accept-abi-contract-20260620. This adds
experimental TALOS_CONNECT_SYSCALL = 9 and TALOS_ACCEPT_SYSCALL = 10 selectors
to the socket-table-aware process descriptor dispatch path for sockets created,
bound, and listened through the accepted AF_INET stream socket path.

This task does not add shell /bin/sockdiag connect/accept output, retained
smoke evidence, generated-root content, send, recv, poll or blocking network
I/O, UDP/TCP payload transport, live packet I/O, live driver adapters, hardware
reachability, Pi 5 hardware work, hardwareTestLock acquisition, lab mutation,
boot publication, SSH, smoltcp adoption, broad socket expansion, public stable
socket ABI acceptance, or a phase transition.

## Findings And Dispositions

- fixed: src/syscall.rs now defines TALOS_CONNECT_SYSCALL = 9,
  TALOS_ACCEPT_SYSCALL = 10, SyscallNumber::TalosConnect, and
  SyscallNumber::TalosAccept. Scalar dispatch still returns ENOTSUP without
  the socket-table-aware process descriptor context.
- fixed: src/network.rs now extends socket backing state with a bounded
  listener pending-peer queue, Connected { local_endpoint, remote_endpoint },
  and Accepted { local_endpoint, remote_endpoint }.
- fixed: Connect validates reserved scalars, IPv4/port bounds,
  current-process descriptor ownership, DescriptorObjectKind::Socket, backing
  owner, open-unbound client state, unique process-local listener lookup,
  synthetic client endpoint capacity, and listener backlog capacity before
  all-or-nothing mutation.
- fixed: Accept validates reserved scalars, current-process descriptor
  ownership, socket kind, backing owner, listener state, non-empty pending
  queue, process descriptor capacity, and socket backing capacity before
  creating the accepted server-side descriptor/backing state.
- fixed: Close/drop cleanup remains descriptor-owned. Closing a listener drops
  queued local peers; closing connected client or accepted server descriptors
  drops only that socket backing state.
- fixed: Focused unit tests cover successful local connect/accept state,
  scalar-dispatch ENOTSUP, malformed scalar arguments, listener absence, empty
  accept, full listener queue, process descriptor capacity, socket backing
  capacity, non-socket descriptors, close/drop cleanup, and unchanged state on
  failures.
- not-an-issue: The existing /bin/sockdiag bind/listen assertion in
  src/local_command_loop.rs now names the empty pending queue field required by
  the extended Listening state; no shell connect/accept behavior was added.
- deferred: Shell-visible /bin/sockdiag connect/accept reporting, retained
  smoke evidence, send, recv, poll/blocking behavior, UDP/TCP payload
  transport, cross-process sockets, live packet I/O, smoltcp, SSH, hardware
  work, public stable socket ABI acceptance, broad socket expansion, and phase
  transition remain deferred.
- removed: No dead-code removal was justified inside this bounded socket
  connect/accept core.

## Evidence

- Source/unit host/QEMU-substitute evidence:
  - src/syscall.rs adds the private connect/accept selector vocabulary,
    socket-table-aware dispatch cases, scalar argument checks, and process
    descriptor allocation path for accepted sockets.
  - src/network.rs adds NetworkSocketPendingQueue,
    NetworkSocketPendingLocalPeer, NetworkSocketState::Connected,
    NetworkSocketState::Accepted, and all-or-nothing
    NetworkSocketDescriptorTable::connect and accept transitions.
  - src/local_command_loop.rs updates the existing bind/listen diagnostic
    state assertion to match the extended Listening state shape without adding
    connect/accept output.
  - Unit tests in src/syscall.rs:
    talos_connect_accept_records_local_handshake_state,
    talos_connect_accept_errors_are_all_or_nothing, and
    talos_accept_rejects_capacity_failures_without_dequeueing_peer.
- Accepted predecessor:
  - phase12-network-socket-connect-accept-abi-contract-20260620 accepted and
    committed at c881aa3f90fd64219f5ebfec8a815e9348e56c7f.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, generated-root publication, shell /bin/sockdiag connect/accept
output, retained smoke evidence, send, recv, UDP/TCP payload transport, SSH,
smoltcp, broad socket expansion, public stable socket ABI acceptance, or phase
transition was performed.

## Acceptance

Accepted.

The accepted evidence level is source/unit host/QEMU-substitute evidence over
private descriptor-backed local AF_INET stream connect/accept state,
process-local listener lookup, bounded pending queue behavior, deterministic
synthetic client endpoints, accepted server-side descriptor creation,
close/drop cleanup, and deterministic error mapping without payload I/O or live
packet I/O.

Selected next task:
phase12-network-shell-sockdiag-connect-accept-core-20260620.

Commit: recorded in durable supervisor state after commit creation.
