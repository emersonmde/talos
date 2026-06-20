# Shell Sockdiag Open/Close Smoke Transcript

Task: phase12-network-shell-sockdiag-open-close-smoke-20260620

Evidence level: host/QEMU-substitute smoke over shell-visible VFS/userspace
socket open/close execution.

Command transcript:

- scripts/qemu-shell-sockdiag-open-close-smoke.sh:
  tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/qemu-shell-sockdiag-open-close-smoke.log
- Source anchors:
  tasks/evidence/2026-06-20-shell-sockdiag-open-close-smoke/source-anchors.txt

The script records the shell-visible `/bin/sockdiag` boundary and invokes
focused test filters. The current no_std QEMU test runner executes the full
target test binary for each filtered invocation, so each filtered command
reports the full 668-test suite while the transcript labels the intended
boundary checks.

## Lifecycle

The retained positive shell path is
src/local_command_loop.rs::local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls.

1. The shell receives `exec /bin/sockdiag` and resolves the read-only
   initramfs executable identity through the existing VFS open/read execution
   path.
2. The exec path records startup ABI and lifecycle state before running the
   task-owned sockdiag diagnostic.
3. The diagnostic opens `AF_INET=2`, `SOCK_STREAM=1`, `protocol=0` through
   `TALOS_SOCKET_SYSCALL = 6` and the socket-table-aware process descriptor
   dispatcher.
4. The returned process descriptor is observed as
   `DescriptorObjectKind::Socket` with read-write access, and the bounded
   socket backing entry records owner/domain/type/protocol.
5. The diagnostic closes through `TALOS_CLOSE_SYSCALL = 2`, verifies backing
   drop, and records closed-descriptor `EBADF` behavior.
6. The shell reaps the diagnostic through `waitpid` and reports the same
   lifecycle result through `laststatus`.

This distinguishes the retained path from a kernel-backed fake shell command:
the command is resolved through VFS executable identity before exercising the
accepted socket syscall, process descriptor, close/drop, waitpid, and
laststatus layers.

## Deterministic Controls

The retained shell transcript includes:

- malformed `exec /bin/sockdiag` arguments.
- missing `/bin/sockdiag` executable identity.
- unsupported domain, unsupported type, and unsupported protocol controls.
- invalid or closed descriptor control.
- `waitpid` and `laststatus` lifecycle observation.

src/syscall.rs::talos_socket_errors_are_deterministic_and_do_not_allocate_on_failure
retains missing-owner, descriptor-capacity, socket-backing-capacity,
unsupported tuple, and no-partial-allocation behavior.

src/syscall.rs::talos_socket_close_rejects_wrong_owner_socket_backing retains
wrong-owner socket backing rejection.

src/syscall.rs::socket_number_requires_socket_table_context_in_scalar_dispatch
retains unchanged scalar syscall vocabulary outside the socket-table-aware
process descriptor path.

src/local_command_loop.rs::local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers
retains the unchanged `/bin/pingdiag` regression/control surface.

## Rejected Claims

This transcript does not accept Pi 5 hardware behavior, hardwareTestLock
acquisition, lab mutation, boot publication, live driver adapters, live packet
I/O, network reachability, send, recv, bind, connect, listen, accept,
poll/blocking network I/O, UDP/TCP payload transport, SSH, smoltcp, broad
socket expansion, public stable socket ABI acceptance, or phase transition.
