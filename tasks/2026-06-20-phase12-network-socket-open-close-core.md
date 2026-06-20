# Phase 12.4 Socket Open/Close Core

Task: phase12-network-socket-open-close-core-20260620

Status: accepted

Classification: phase12-network-socket-open-close-core-accepted

## Scope

Implement only the bounded socket open/close descriptor core selected by
phase12-network-socket-open-close-abi-contract-20260620. This adds the private
experimental `TALOS_SOCKET_SYSCALL = 6` selector, accepts only
`AF_INET=2`, `SOCK_STREAM=1`, `protocol=0`, returns a process descriptor
backed by a fixed-capacity socket table entry, and closes that descriptor
through the existing `TALOS_CLOSE_SYSCALL = 2` lifetime path.

This task does not add `/bin/sockdiag`, generated-root content, send, recv,
bind, connect, listen, accept, poll or blocking network I/O, UDP/TCP payload
transport, live packet I/O, live driver adapters, hardware reachability, Pi 5
hardware work, hardwareTestLock acquisition, lab mutation, boot publication,
SSH, smoltcp adoption, broad socket expansion, public stable socket ABI
acceptance, or a phase transition.

## Findings And Dispositions

- fixed: `src/syscall.rs` now defines `TALOS_SOCKET_SYSCALL = 6` and
  `SyscallNumber::TalosSocket`. Scalar dispatch still returns `ENOTSUP`
  without the socket-table-aware process descriptor context.
- fixed: `src/network.rs` now has a fixed-capacity
  `NetworkSocketDescriptorTable` and socket backing records that retain owner,
  domain, type, and protocol. The only successful tuple is `AF_INET=2`,
  `SOCK_STREAM=1`, `protocol=0`; unsupported tuples return `ENOTSUP`.
- fixed: `dispatch_process_descriptor_with_socket_table` opens accepted
  sockets as `DescriptorObjectKind::Socket`, `DescriptorAccess::ReadWrite`,
  `DescriptorFlags::EMPTY` process descriptors and returns the lowest free
  process descriptor.
- fixed: Socket-aware close validates the process descriptor and backing owner,
  delegates non-socket descriptors to the existing close path, and for socket
  descriptors drops both the process descriptor and matching backing entry.
- fixed: Capacity and unwind behavior match the accepted contract. Full process
  descriptor tables return `EMFILE` before socket backing allocation; full
  socket backing tables return `ENOSPC` without process descriptor
  allocation; descriptor allocation failure after backing allocation rolls back
  the backing entry.
- fixed: Focused unit tests cover successful open/close, unsupported
  domain/type/protocol, reserved arguments, missing owner, process descriptor
  capacity, socket backing capacity, wrong-owner backing rejection, invalid and
  closed descriptor paths, and unchanged non-socket close behavior.
- not-an-issue: Existing VFS/open/read, generated-root, pingdiag,
  waitpid/laststatus, and non-socket descriptor behavior remain unchanged by
  the socket-table-aware test dispatch path.
- deferred: `/bin/sockdiag`, shell-visible diagnostics, generated-root
  content, send, recv, bind, connect, listen, accept, poll/blocking I/O,
  UDP/TCP payload transport, live packet I/O, smoltcp, SSH, hardware work,
  public stable socket ABI acceptance, broad socket expansion, and phase
  transition remain deferred.
- removed: No dead-code removal was justified inside this bounded socket
  open/close core.

## Evidence

- Source/unit evidence:
  - `src/syscall.rs` adds `TALOS_SOCKET_SYSCALL`,
    `SyscallNumber::TalosSocket`,
    `dispatch_process_descriptor_with_socket_table`,
    `dispatch_talos_socket`, and socket-aware close/drop handling.
  - `src/network.rs` adds `NetworkSocketDescriptorTable`,
    `NetworkSocketDescriptor`, `NetworkSocket`, and accepted tuple
    constants.
  - `src/posix.rs` adds `DescriptorTable::has_free_slot` so socket open can
    enforce process-descriptor-capacity-before-backing allocation.
  - Unit tests in `src/syscall.rs`:
    `socket_number_requires_socket_table_context_in_scalar_dispatch`,
    `talos_socket_opens_af_inet_stream_descriptor_and_close_drops_backing`,
    `talos_socket_errors_are_deterministic_and_do_not_allocate_on_failure`,
    and `talos_socket_close_rejects_wrong_owner_socket_backing`.
- Accepted predecessor:
  - phase12-network-socket-open-close-abi-contract-20260620 accepted and
    committed at e6c0f5117acfdbd29047dfc3766fd04a234142e2.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, generated-root publication, `/bin/sockdiag`, send/recv,
bind/connect/listen/accept, UDP/TCP payload transport, SSH, smoltcp, broad
socket expansion, public stable socket ABI acceptance, or phase transition was
performed.

## Acceptance

Accepted.

The accepted evidence level is source/unit host/QEMU-substitute evidence over
the private experimental socket-open selector, process descriptor ownership,
fixed-capacity socket backing, close/drop behavior, deterministic error
mapping, and unchanged non-socket descriptor surfaces.

Selected next task:
phase12-network-shell-sockdiag-open-close-core-20260620.

Commit: recorded in durable supervisor state after commit creation.
