# Phase 12.4 Shell Sockdiag Open/Close Core

Task: phase12-network-shell-sockdiag-open-close-core-20260620

Status: accepted

Classification: phase12-network-shell-sockdiag-open-close-core-accepted

## Scope

Add only the minimal shell-visible VFS/userspace diagnostic for the accepted
socket open/close descriptor path. The diagnostic is `/bin/sockdiag`; it is
looked up and read through the existing initramfs/VFS execution path, planned
as an ELF fixture, receives the normal startup ABI/lifecycle accounting, and
then opens/closes the accepted process-local socket descriptor through the
private experimental `TALOS_SOCKET_SYSCALL = 6` and existing
`TALOS_CLOSE_SYSCALL = 2`.

This task does not add send, recv, bind, connect, listen, accept,
poll/blocking network I/O, UDP/TCP payload transport, live packet I/O, live
driver adapters, hardware reachability, Pi 5 hardware work, hardwareTestLock
acquisition, lab mutation, boot publication, generated-root publication, SSH,
smoltcp adoption, broad socket expansion, public stable socket ABI acceptance,
or a phase transition.

## Findings And Dispositions

- fixed: `src/initramfs.rs` now includes `/bin/sockdiag` as a read-only
  executable fixture alongside `/bin/pingdiag`, so the diagnostic has a real
  VFS executable identity instead of a fake kernel-only command path.
- fixed: `src/local_command_loop.rs` recognizes `/bin/sockdiag` in shell exec
  resolution and `/bin` listings, preserving startup ABI, lifecycle,
  `waitpid`, and `laststatus` transcript behavior.
- fixed: The descriptor-backed shell I/O now owns a bounded socket backing
  table used only by the sockdiag diagnostic path.
- fixed: The sockdiag diagnostic opens `AF_INET=2`, `SOCK_STREAM=1`,
  `protocol=0`, verifies the read-write socket descriptor and backing record,
  closes through `TALOS_CLOSE_SYSCALL = 2`, then verifies double-close
  `EBADF` and backing-state drop.
- fixed: Focused controls cover unsupported domain, unsupported type,
  unsupported protocol, malformed sockdiag arguments, missing executable
  identity, and closed-descriptor behavior.
- fixed: `src/posix.rs` exposes a small descriptor-access name helper for
  diagnostic transcript output.
- not-an-issue: `/bin/pingdiag`, VFS open/read, generated-root,
  waitpid/laststatus, and process descriptor regression behavior remain
  covered by existing tests plus the new sockdiag shell transcript test.
- deferred: Retained smoke evidence, send/recv, bind/connect/listen/accept,
  poll/blocking network I/O, UDP/TCP payload transport, live packet I/O,
  hardware reachability, smoltcp, SSH, generated-root publication, public
  stable socket ABI acceptance, broad socket expansion, and phase transition
  remain deferred.
- removed: No dead-code removal was justified inside this bounded diagnostic
  slice.

## Evidence

- Source/unit evidence:
  - `src/initramfs.rs` adds `PHASE12_SOCKDIAG_PATH` and an executable fixture
    reachable as `/bin/sockdiag`.
  - `src/local_command_loop.rs` adds `LocalCommandSockdiagRecord`,
    `LocalCommandSockdiagControlRecord`, rendering for `sockdiag` transcript
    lines, and a descriptor-backed shell diagnostic that calls
    `dispatch_process_descriptor_with_socket_table`.
  - `src/local_command_loop.rs` test
    `local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls`
    proves shell-visible VFS/userspace execution, socket open/close, controls,
    `waitpid`, and `laststatus`.
  - Existing `/bin/pingdiag`, listing, syscall, and descriptor tests continue
    to pass in the full host/QEMU-substitute test suite.
- Accepted predecessor:
  - phase12-network-socket-open-close-core-20260620 accepted and committed at
    4a86169c5986cab3ce94558cfdaee5f2255c144c.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed; 668 talos no_std tests.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, live driver adapter, live packet I/O, hardware
reachability, generated-root publication, send/recv, bind/connect/listen/accept,
UDP/TCP payload transport, SSH, smoltcp, broad socket expansion, public stable
socket ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

The accepted evidence level is source/unit host/QEMU-substitute evidence over
shell-visible VFS/userspace `/bin/sockdiag` execution, selected socket
open/close syscall path, process descriptor ownership, close/drop behavior,
waitpid/laststatus observation, and unchanged accepted diagnostics.

Selected next task:
phase12-network-shell-sockdiag-open-close-smoke-20260620.

Commit: recorded in durable supervisor state after commit creation.
