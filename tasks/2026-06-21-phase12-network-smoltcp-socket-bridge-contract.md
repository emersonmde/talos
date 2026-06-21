# Phase 12.4 smoltcp Socket Bridge Contract

Task: phase12-network-smoltcp-socket-bridge-contract-20260621

Status: accepted

Classification: phase12-network-smoltcp-socket-bridge-contract-accepted

## Scope

Define the first private bridge from the accepted descriptor-backed AF_INET
stream socket surface to the accepted host-only smoltcp TCP frontier. This is
a source/task/docs contract only. It selects a bounded host-only core task that
may prove TCP Established and one payload transfer through existing syscall
dispatch, process descriptor ownership, fixed packet queues, and smoltcp TCP
state.

This task does not implement runtime behavior, add shell-visible /bin/sockdiag
TCP output, retain smoke evidence, run Pi 5 hardware, acquire
hardwareTestLock, mutate the lab, publish a boot image, power-cycle hardware,
add live driver adapters, perform live packet I/O, accept hardware
reachability, accept SSH, accept a public stable socket ABI, broaden socket
behavior, add UDP/raw sockets, or transition phase.

## Contract

- Source owners:
  - src/network.rs owns Talos packet queues, NetworkDevice,
    PacketQueueNetworkDevice, SmoltcpPacketDeviceAdapter, smoltcp
    Interface/SocketSet/TCP storage, fixed TCP buffers, and bridge result
    mapping.
  - src/syscall.rs owns the private syscall dispatch path for
    TALOS_SOCKET_SYSCALL, TALOS_BIND_SYSCALL, TALOS_LISTEN_SYSCALL,
    TALOS_CONNECT_SYSCALL, TALOS_ACCEPT_SYSCALL, TALOS_SEND_SYSCALL,
    TALOS_RECV_SYSCALL, TALOS_POLL_SYSCALL, and TALOS_POLL_WAIT_SYSCALL.
  - src/posix.rs owns process descriptor tables, DescriptorObjectKind
    identity, file-descriptor allocation, close/drop behavior, and
    copy_from_user/copy_to_user validation.
  - src/scheduler.rs owns task identity, TaskState::Blocked state, and
    runnable restoration used by bounded poll-wait evidence.
- Descriptor mapping:
  - A process-visible socket remains a normal process descriptor with
    DescriptorObjectKind::Socket and a private NetworkSocketDescriptorTable
    reference.
  - Owner checks remain mandatory before bind/listen/connect/accept/send/recv,
    poll, poll-wait, or close. A descriptor owned by another ProcessOwnerId is
    EBADF.
  - Accepted local socket rendezvous behavior remains a regression/control
    surface. The bridge core may add task-owned bridge backing state, but it
    must not erase the accepted local rendezvous semantics or same-owner and
    cross-process controls.
- smoltcp state ownership:
  - Bridge state is kernel-owned and fixed-capacity. It may contain smoltcp
    Interface values, SocketSet storage, TCP socket handles, TCP RX/TX buffers,
    and endpoint metadata, but no heap allocation or host OS phy backend.
  - SocketSet and TCP buffers live for the bridge backing record or connection
    record lifetime. Process descriptors hold private references, not borrowed
    smoltcp objects.
  - Time progression is explicit and deterministic. The first core may advance
    smoltcp by caller/test-owned ticks or bounded dispatch steps only; no
    autonomous timer, interrupt, background task, or wall-clock claim is
    accepted.
- Packet queue ownership:
  - All packets remain in Talos-owned fixed packet queues and fixed frame
    buffers. SmoltcpPacketDeviceAdapter is the only accepted smoltcp phy
    boundary for this slice.
  - The first core may use host-only/fake endpoint queues to connect a client
    and listener; it must not bind to RP1/GEM/MACB/PHY hardware or live driver
    packet I/O.
- Syscall subset:
  - The first core may exercise private AF_INET/SOCK_STREAM behavior through
    socket, bind, listen, connect, accept, send, recv, poll, poll-wait, and
    close only.
  - AF_INET=2, SOCK_STREAM=1, and protocol=0 remain the only accepted socket
    open tuple. Explicit IPPROTO_TCP, datagram sockets, raw sockets, ICMP
    sockets, UDP, multicast, DNS, DHCP, routing, TLS, async networking, and
    interface discovery remain rejected.
- Bounded failure modes:
  - Unsupported socket tuple or bridge feature: ENOTSUP.
  - Bad descriptor, wrong owner, or non-socket descriptor: EBADF.
  - Invalid scalar arguments, reserved register use, invalid endpoint/port, or
    invalid state transition: EINVAL.
  - Process descriptor capacity exhausted: EMFILE.
  - Socket/bridge/queue/buffer capacity exhausted: ENOSPC.
  - Nonblocking no-progress, no pending accept, or no received bytes while the
    peer remains live: EAGAIN.
  - Peer gone after queued bytes are drained or the smoltcp connection is
    closed/reset: EPIPE.
  - User copy-in/copy-out validation failure: EFAULT.
- Cleanup semantics:
  - Close/drop/owner cleanup must release process-visible descriptors and
    bridge backing records deterministically, wake affected local waiters, and
    leave no stale peer reference that can be reused by a different owner.
  - EOF/hangup behavior must be deterministic: queued readable bytes may be
    drained before the final peer-gone result, and poll readiness must report
    READ/HANGUP/ERROR according to the accepted private readiness vocabulary.
- ABI status:
  - All behavior in this chain is private and experimental. It is not a Linux
    syscall-number compatibility claim, not a libc ABI promise, and not a
    public stable socket ABI acceptance.

## Findings And Dispositions

- fixed: Reviewed accepted socket open/close, bind/listen, connect/accept,
  send/recv, readiness/poll, blocking poll-wait, cross-process local
  rendezvous, packet queue, smoltcp adapter, and host-only smoltcp TCP
  handshake evidence before selecting the bridge boundary.
- fixed: The first implementation boundary is host-only/source-unit TCP
  behavior over fixed packet queues through existing private socket syscall
  dispatch and descriptor ownership.
- fixed: Source ownership is split across src/network.rs, src/syscall.rs,
  src/posix.rs, and src/scheduler.rs so the core task can implement without
  broadening into shell diagnostics, live packet I/O, hardware, SSH, or public
  ABI stability.
- fixed: The contract explicitly preserves accepted local socket rendezvous,
  poll-wait, pingdiag, and smoltcp host-only handshake surfaces as regression
  controls.
- not-an-issue: The contract accepts only static source/task/docs evidence.
  That is sufficient because no runtime bridge behavior, shell-visible TCP,
  live packet I/O, hardware reachability, SSH, or public ABI claim is accepted
  here.
- deferred: Runtime bridge implementation, shell-visible /bin/sockdiag TCP
  diagnostic, retained smoke transcript, live driver adapters, live packet I/O,
  hardware reachability, SSH, public stable socket ABI acceptance, broad socket
  expansion, and phase transition remain later explicit tasks.
- removed: No fake/kernel-only TCP shell command, hardware action, lab
  mutation, boot publication, live packet I/O, SSH claim, public ABI claim, or
  phase-transition claim was added.

## Evidence

- Accepted local socket and syscall surfaces:
  - tasks/2026-06-20-phase12-network-socket-open-close-core.md.
  - tasks/2026-06-20-phase12-network-socket-bind-listen-core.md.
  - tasks/2026-06-20-phase12-network-socket-connect-accept-core.md.
  - tasks/2026-06-20-phase12-network-socket-send-recv-core.md.
  - tasks/2026-06-20-phase12-network-socket-readiness-poll-core.md.
  - tasks/2026-06-21-phase12-network-socket-blocking-poll-wait-core.md.
  - tasks/2026-06-21-phase12-network-cross-process-local-socket-rendezvous-core.md.
- Accepted shell/control surfaces:
  - tasks/2026-06-21-phase12-network-shell-sockdiag-cross-process-local-socket-closeout.md.
  - tasks/2026-06-20-phase12-network-shell-pingdiag-core.md.
- Accepted smoltcp frontier:
  - tasks/2026-06-21-phase12-network-smoltcp-adoption-contract.md.
  - tasks/2026-06-21-phase12-network-smoltcp-no-std-dependency-core.md.
  - tasks/2026-06-21-phase12-network-smoltcp-packet-device-adapter-core.md.
  - tasks/2026-06-21-phase12-network-smoltcp-loopback-tcp-handshake-core.md.
  - tasks/2026-06-21-phase12-network-smoltcp-tcp-frontier-closeout.md.
- Source anchors:
  - src/network.rs NetworkSocketDescriptorTable, NetworkSocketState,
    NetworkSocketReadiness, PacketQueueNetworkDevice,
    SmoltcpPacketDeviceAdapter, and host-only smoltcp TCP handshake tests.
  - src/syscall.rs private socket syscall selectors and socket-table-aware
    dispatch helpers.
  - src/posix.rs process descriptor store, descriptor object identity, and
    user copy helpers.
  - src/scheduler.rs task state and scheduler-owned blocked/runnable state.

## Validation

- static source/task/docs review: passed. Reviewed the source/task anchors
  listed above, docs/src/project/phase12-networking-ssh.md,
  docs/src/roadmap.md, and docs/src/decisions/README.md.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed with existing large search-index
  warning.
- git diff --cached --check: passed before commit.

No runtime implementation, /bin/sockdiag TCP diagnostic, retained smoke
transcript, Pi 5 hardware run, hardwareTestLock acquisition, lab mutation,
boot publication, power cycle, live driver adapter, live packet I/O, hardware
reachability, SSH, entropy/crypto/key management, public stable socket ABI
acceptance, broad socket expansion, or phase transition was performed.

## Acceptance

Accepted.

The accepted evidence level is static source/task/docs contract evidence only.
The selected implementation boundary is a private descriptor-backed AF_INET
SOCK_STREAM bridge to smoltcp TCP over fixed host packet queues through the
existing syscall dispatch and descriptor ownership surfaces.

The contract is sufficient to make the next bounded implementation task
objective. Runtime behavior, shell-visible /bin/sockdiag TCP diagnostics,
retained smoke evidence, live driver adapters, live packet I/O, hardware
reachability, SSH, public stable socket ABI acceptance, broad socket
expansion, and phase transition remain rejected.

Selected next task:
phase12-network-smoltcp-socket-bridge-core-20260621.

Commit: recorded in durable supervisor state after commit creation.
