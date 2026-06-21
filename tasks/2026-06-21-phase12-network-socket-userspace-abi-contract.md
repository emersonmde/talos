# Phase 12.4 Socket Userspace ABI Contract

Task: phase12-network-socket-userspace-abi-contract-20260621

Status: accepted

Classification: phase12-network-socket-userspace-abi-contract-accepted

## Scope

Record the exact private Talos userspace socket ABI selectors, argument
registers, user-memory layouts, return values, errno mappings, and accepted
evidence for the already accepted descriptor-backed AF_INET/SOCK_STREAM socket
and host-only smoltcp TCP diagnostic frontier.

This task is a static source/task/docs/evidence contract only. It does not
implement runtime behavior, add wrappers, change syscall dispatch, acquire
hardwareTestLock, mutate the lab, publish a boot image, perform live packet
I/O, accept SSH, add UDP/raw sockets, accept libc/std or POSIX/Linux
compatibility, broaden sockets, or transition phase.

## Findings And Dispositions

- fixed: Recorded the private Talos socket syscall selector vocabulary from
  src/syscall.rs: TALOS_SOCKET=6, TALOS_BIND=7, TALOS_LISTEN=8,
  TALOS_CONNECT=9, TALOS_ACCEPT=10, TALOS_SEND=11, TALOS_RECV=12,
  TALOS_POLL=13, TALOS_POLL_WAIT=14, and TALOS_CLOSE=2 for socket-aware close.
- fixed: Recorded the stable lower-AArch64 calling convention already accepted
  by the syscall frontier: svc #0, x8 syscall selector, x0 through x5 scalar
  arguments, x0 return value, and negative x0 as -errno.
- fixed: Recorded the accepted socket family/type/protocol subset as
  AF_INET=2, SOCK_STREAM=1, protocol=0 only.
- fixed: Recorded the accepted user-memory surfaces: send copies len bytes
  from a user buffer, recv copies up to len bytes to a user buffer, poll and
  poll_wait copy arrays of 16-byte little-endian entries, and all pointer
  movement uses the accepted copy_from_user/copy_to_user mapping checks.
- fixed: Tied each ABI claim to accepted source/unit or retained
  host/QEMU-substitute evidence for private local sockets, blocking poll wait,
  cross-process local sockets, private smoltcp TCP bridge behavior, and
  shell-visible /bin/sockdiag smoltcp TCP smoke evidence.
- not-an-issue: The ABI remains private experimental Talos vocabulary. This
  contract documents the current accepted surface so the next core task can
  compile constants/wrappers against it; it does not promote the surface to a
  public stable POSIX/Linux ABI.
- removed: No runtime source behavior, hardware artifact, boot publication,
  live packet I/O claim, SSH claim, UDP/raw socket claim, libc/std claim,
  POSIX/Linux compatibility claim, broad socket expansion, or phase-transition
  claim was introduced.
- deferred: Documented no_std/userspace wrappers, shell diagnostic use of those
  wrappers, retained ABI smoke evidence, live driver adapters, live packet I/O,
  hardware reachability, SSH strategy, UDP/raw sockets, and libc/std socket
  wrappers remain future explicit tasks.

## ABI Contract

The socket surface uses the accepted stable syscall trap convention:

- trap: lower-AArch64 svc #0.
- selector: x8.
- scalar arguments: x0, x1, x2, x3, x4, x5.
- return: x0; success is a non-negative scalar, failure is negative errno.
- unsupported scalar-only socket selectors outside the socket-aware dispatch
  path fail closed through the target-independent dispatcher with -ENOTSUP.

Accepted selectors:

| Selector | Name | Arguments | Success return |
| --- | --- | --- | --- |
| 2 | TALOS_CLOSE | x0=fd, x1..x5=0 | 0, and socket descriptors also drop socket/bridge state |
| 6 | TALOS_SOCKET | x0=domain, x1=type, x2=protocol, x3..x5=0 | process fd |
| 7 | TALOS_BIND | x0=fd, x1=ipv4_be, x2=port, x3..x5=0 | 0 |
| 8 | TALOS_LISTEN | x0=fd, x1=backlog, x2..x5=0 | 0 |
| 9 | TALOS_CONNECT | x0=fd, x1=ipv4_be, x2=port, x3..x5=0 | 0 |
| 10 | TALOS_ACCEPT | x0=listener_fd, x1..x5=0 | accepted process fd |
| 11 | TALOS_SEND | x0=fd, x1=user_buf, x2=len, x3=flags=0, x4..x5=0 | bytes sent |
| 12 | TALOS_RECV | x0=fd, x1=user_buf, x2=len, x3=flags=0, x4..x5=0 | bytes received |
| 13 | TALOS_POLL | x0=user_entries, x1=entry_count, x2=flags=0, x3..x5=0 | ready entry count |
| 14 | TALOS_POLL_WAIT | x0=user_entries, x1=entry_count, x2=timeout_ticks, x3=flags=0, x4..x5=0 | ready entry count, 0 on timeout, or blocked outcome internally |

Accepted socket constants:

- SOCKET_DOMAIN_AF_INET = 2.
- SOCKET_TYPE_STREAM = 1.
- SOCKET_PROTOCOL_DEFAULT = 0.
- SOCKET_LISTEN_BACKLOG_MIN = 1.
- SOCKET_LISTEN_BACKLOG_MAX = 4.
- SOCKET_PAYLOAD_QUEUE_CAPACITY = 64 bytes for the accepted local payload
  queue and bounded send length.
- Synthetic local loopback-style socket endpoints use
  SOCKET_SYNTHETIC_LOCAL_IPV4_BE = 0x7f000001 and client ports starting at
  SOCKET_SYNTHETIC_CLIENT_PORT_BASE = 49152.

Poll entry layout is a 16-byte little-endian record:

| Offset | Size | Field |
| --- | --- | --- |
| 0 | 8 | fd |
| 8 | 4 | events |
| 12 | 4 | revents |

Accepted poll constants:

- TALOS_POLL_ENTRY_SIZE = 16.
- TALOS_POLL_MAX_ENTRIES = 8.
- TALOS_POLL_WAIT_MAX_TICKS = 1024.
- supported event bits are READ, WRITE, HANGUP, and ERROR from
  NetworkSocketReadiness.
- unsupported event bits return -EINVAL.

Accepted error vocabulary for this contract is the currently encoded errno set
used by the socket surface: -EBADF, -EAGAIN, -EFAULT, -EBUSY, -EEXIST,
-EINVAL, -EMFILE, -ENOSPC, -EPIPE, -ENOSYS, and -ENOTSUP. Specific
deterministic controls retained in the accepted evidence include unsupported
domain/type/protocol as -ENOTSUP, reserved argument or invalid
endpoint/backlog/flags/events as -EINVAL, invalid/non-socket fd as -EBADF or
poll ERROR, empty accept/recv as -EAGAIN, descriptor table capacity as
-EMFILE, socket/payload/wait capacity as -ENOSPC, duplicate listener endpoint
as -EEXIST, duplicate poll wait for a task as -EBUSY, and peer close send as
-EPIPE.

## Evidence

- Syscall selector and return encoding source:
  src/syscall.rs.
- Socket state constants and descriptor table source:
  src/network.rs.
- Private smoltcp socket bridge core:
  tasks/2026-06-21-phase12-network-smoltcp-socket-bridge-core.md.
- Shell-visible smoltcp TCP diagnostic core:
  tasks/2026-06-21-phase12-network-shell-sockdiag-smoltcp-tcp-core.md.
- Retained shell-visible smoltcp TCP smoke:
  tasks/2026-06-21-phase12-network-shell-sockdiag-smoltcp-tcp-smoke.md.
- Retained smoke transcript:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/smoke-transcript.md.
- Retained smoke classification and evidence map:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/classification.json
  and tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/evidence-map.json.

## Accepted Boundary

The accepted evidence level is static source/task/docs/evidence review backed
by prior source/unit and host/QEMU-substitute evidence. The accepted boundary
is the private Talos userspace socket ABI contract for descriptor-backed
AF_INET/SOCK_STREAM behavior and shell-visible /bin/sockdiag observation of
the private host-only smoltcp TCP bridge.

This is not live driver integration, not live packet I/O, not Pi 5 hardware
reachability, not SSH readiness, not UDP/raw sockets, not libc/std socket
support, and not POSIX/Linux compatibility.

## Validation

- static source/task/docs/evidence review: passed. Reviewed src/syscall.rs,
  src/network.rs, accepted smoltcp socket bridge task, shell-visible diagnostic
  task, retained smoke task, smoke transcript, smoke classification/evidence
  map, Phase 12 networking doc, roadmap, and early POSIX shape note.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Rust source behavior change, cargo test, Pi 5 hardware run, hardwareTestLock
acquisition, lab mutation, boot publication, generated-root publication, live
packet I/O, hardware reachability, SSH, UDP/raw socket work, broad socket
expansion, public POSIX/Linux ABI acceptance, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-socket-userspace-abi-core-20260621.

The next bounded task may compile no_std/userspace ABI constants or wrappers
against this private contract. It must not broaden beyond the accepted
host-only socket/smoltcp TCP behavior or claim live packet I/O, hardware
reachability, SSH, UDP/raw sockets, libc/std, POSIX/Linux compatibility, public
stable ABI acceptance, broad socket expansion, or phase transition.

Commit: recorded in durable supervisor state after commit creation.
