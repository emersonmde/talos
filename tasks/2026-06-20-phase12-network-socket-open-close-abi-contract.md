# Phase 12.4 Socket Open/Close ABI Contract

Task: phase12-network-socket-open-close-abi-contract-20260620

Status: accepted

Classification: phase12-network-socket-open-close-abi-contract-accepted

## Scope

Define the smallest socket integration contract after the accepted
shell-visible `/bin/pingdiag` smoke frontier. This task selects only an
experimental AF_INET socket open/close surface through the existing stable SVC
dispatch and process descriptor model.

This contract does not add runtime behavior. It does not accept send, recv,
bind, connect, listen, accept, poll or blocking network I/O, UDP/TCP payload
transport, live packet I/O, live driver adapters, hardware reachability,
hardwareTestLock acquisition, Pi 5 hardware work, lab mutation, boot
publication, SSH, smoltcp adoption, broad socket expansion, public stable
socket ABI acceptance, or a phase transition.

## Findings And Dispositions

- fixed: The accepted pingdiag smoke closeout ended at a diagnostic descriptor
  frontier with no public socket task selected. This contract chooses the next
  bounded feature step: socket open/close descriptor creation only.
- fixed: The follow-up implementation should add a private
  `TALOS_SOCKET_SYSCALL = 6` selector to the existing `STABLE_SVC_IMMEDIATE`
  syscall vocabulary. Arguments are scalar only: `x0=domain`, `x1=type`,
  `x2=protocol`, and `x3..x5=0`.
- fixed: The only accepted successful tuple is `AF_INET=2`,
  `SOCK_STREAM=1`, `protocol=0`. Explicit `IPPROTO_TCP`, datagram, raw,
  IPv6, Unix-domain, and packet sockets remain deferred until later contracts.
- fixed: Successful socket open returns the lowest available process
  descriptor from the current process descriptor table, backed by a
  fixed-capacity kernel socket table entry and a `DescriptorObjectKind::Socket`
  `ReadWrite` descriptor.
- fixed: Close uses the existing `TALOS_CLOSE_SYSCALL = 2` path. Closing an
  accepted socket descriptor must drop the process descriptor and the matching
  socket backing entry in the same bounded operation.
- fixed: Capacity and unwind behavior are explicit. If the process descriptor
  table is full, return `EMFILE` and allocate no socket backing. If the socket
  backing table is full, return `ENOSPC` and allocate no process descriptor.
  If descriptor allocation fails after backing allocation, remove the backing
  before returning the descriptor-table error.
- fixed: Error vocabulary is bounded to existing Talos POSIX errors:
  unsupported domain/type/protocol returns `ENOTSUP`; nonzero reserved
  arguments return `EINVAL`; missing current process owner, wrong-owner
  descriptor lookup, invalid descriptor, and closed descriptor return `EBADF`;
  descriptor capacity returns `EMFILE`; socket backing capacity returns
  `ENOSPC`.
- not-an-issue: `DescriptorObjectKind::Socket` already exists in
  `src/posix.rs`, and the descriptor table already owns lowest-free
  allocation, close, dup, stdio inheritance, and per-process owner lookup
  mechanics. The follow-up core can reuse those surfaces instead of inventing
  a separate socket fd namespace.
- not-an-issue: The accepted `/bin/pingdiag` descriptor and packet-pump
  evidence remains diagnostic-only host/QEMU-substitute evidence. It informs
  process-local descriptor ownership and close/drop behavior, but it is not a
  public socket or live network reachability proof.
- deferred: send, recv, bind, connect, listen, accept, poll/blocking,
  UDP/TCP payload transport, explicit `IPPROTO_TCP`, datagram/raw sockets,
  live packet I/O, smoltcp, SSH, hardware retry, broad socket expansion, and
  phase transition remain deferred.
- removed: No runtime source cleanup or implementation was justified in this
  contract-only task.

## Accepted Contract

The next implementation task should add a private experimental syscall
selector:

- selector: `TALOS_SOCKET_SYSCALL = 6`;
- enum variant: `SyscallNumber::TalosSocket`;
- SVC immediate: existing `STABLE_SVC_IMMEDIATE = 0`;
- arguments: `x0=domain`, `x1=type`, `x2=protocol`, `x3=0`, `x4=0`,
  `x5=0`;
- accepted tuple: `AF_INET=2`, `SOCK_STREAM=1`, `protocol=0`;
- return on success: lowest available process descriptor number;
- close path: existing `TALOS_CLOSE_SYSCALL = 2`;
- descriptor entry: `DescriptorAccess::ReadWrite`,
  `DescriptorFlags::EMPTY`, `DescriptorObjectKind::Socket`, and a socket
  backing reference owned by the current process;
- unsupported operations: no send, recv, bind, connect, listen, accept, poll,
  blocking, packet transport, or network reachability semantics are accepted by
  opening the descriptor.

The socket backing table should be fixed-capacity and process-local in the
same ownership sense as the existing process descriptor store. A socket backing
entry records at least the owning `ProcessOwnerId`, domain, type, protocol,
and a closed/open state sufficient for deterministic close/drop tests. The
backing table is not a TCP state machine and must not expose packet queues,
driver adapters, or smoltcp state in this slice.

This contract keeps later POSIX compatibility viable because the selected
argument shape is the conventional `socket(domain, type, protocol)` shape and
the returned value is a normal process descriptor consumed by `close(fd)`.
The selector number remains a Talos-private experimental ABI detail for this
task chain; it is not a Linux syscall-number compatibility claim and does not
freeze libc or public ABI behavior beyond this bounded open/close experiment.

## Evidence

- static source/task/evidence review:
  - `src/syscall.rs` currently reserves stable syscall vocabulary through
    `TALOS_OPEN_SYSCALL = 5`; raw selector 6 is currently unknown and is the
    smallest private extension point for this contract.
  - `src/syscall.rs` uses `STABLE_SVC_IMMEDIATE = 0`, `SyscallNumber`,
    scalar `SyscallArguments`, `SyscallReturn`, and existing
    `TALOS_CLOSE_SYSCALL` dispatch surfaces.
  - `src/posix.rs` already defines `DescriptorObjectKind::Socket`,
    `DescriptorAccess::ReadWrite`, `DescriptorTable::allocate`,
    `DescriptorTable::close`, and `ProcessDescriptorStore` owner lookup.
  - `src/syscall.rs` and `src/network.rs` accepted process-local ping
    diagnostic descriptors, close/drop behavior, fixed capacity, and
    wrong-owner/invalid/closed descriptor controls as diagnostic evidence.
  - `tasks/2026-06-20-phase12-network-shell-pingdiag-smoke-closeout.md`
    records the accepted shell-visible host/QEMU-substitute diagnostic
    frontier that this socket contract follows.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No runtime source implementation, generated userland change, shell command
change, smoke harness, retained execution transcript, Pi 5 hardware run,
hardwareTestLock acquisition, boot archive publication, lab mutation, live
packet I/O, public socket API, stable socket ABI acceptance, SSH, broad socket
expansion, or phase transition was performed.

## Next Action

Selected next task:
phase12-network-socket-open-close-core-20260620.

The next task may implement only the bounded socket open/close descriptor core
defined here. It must preserve the rejected-claim boundaries above and must not
add send/recv, bind/connect, UDP/TCP transport, live packet I/O, SSH, hardware
work, or broad socket expansion.
