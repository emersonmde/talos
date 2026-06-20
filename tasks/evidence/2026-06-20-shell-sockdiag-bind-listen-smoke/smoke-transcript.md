# Shell Sockdiag Bind/Listen Smoke Transcript

Task: phase12-network-shell-sockdiag-bind-listen-smoke-20260620

Evidence level: host/QEMU-substitute smoke over shell-visible VFS/userspace
socket bind/listen execution.

Command transcript:

- scripts/qemu-shell-sockdiag-bind-listen-smoke.sh:
  tasks/evidence/2026-06-20-shell-sockdiag-bind-listen-smoke/qemu-shell-sockdiag-bind-listen-smoke.log
- Source anchors:
  tasks/evidence/2026-06-20-shell-sockdiag-bind-listen-smoke/source-anchors.txt

The script records the shell-visible `/bin/sockdiag` boundary and invokes
focused test filters. The current no_std QEMU test runner executes the full
target test binary for each filtered invocation, so each filtered command
reports the full 670-test suite while the transcript labels the intended
boundary checks. The retained command log contains eight passing 670-test
invocations and ends with
`classification=host-substitute-shell-sockdiag-bind-listen-smoke-complete`.

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
4. The diagnostic binds the returned process descriptor to
   `127.0.0.1:8080` through `TALOS_BIND_SYSCALL = 7`, then listens through
   `TALOS_LISTEN_SYSCALL = 8`.
5. Repeated listen updates the bounded backlog from 2 to 4, leaving the socket
   in `Listening { local_endpoint, backlog }` state.
6. The diagnostic closes through `TALOS_CLOSE_SYSCALL = 2`, verifies backing
   drop, and records closed-descriptor `EBADF` behavior.
7. The shell reaps the diagnostic through `waitpid` and reports the same
   lifecycle result through `laststatus`.

This distinguishes the retained path from a kernel-backed fake shell command:
the command is resolved through VFS executable identity before exercising the
accepted socket syscall, bind/listen state, process descriptor, close/drop,
waitpid, and laststatus layers.

## Deterministic Controls

The retained shell transcript includes:

- malformed `exec /bin/sockdiag` arguments.
- missing `/bin/sockdiag` executable identity.
- unsupported domain, unsupported type, and unsupported protocol controls.
- bind on a closed descriptor, listen-before-bind, invalid bind endpoint,
  invalid backlog, repeated bind, repeated listen backlog update, and double
  close `EBADF`.
- `waitpid` and `laststatus` lifecycle observation.

src/syscall.rs::talos_bind_listen_records_socket_state_and_close_drops_backing
retains successful open, bind, repeated-bind rejection, listen, repeated-listen
backlog update, listening state observation, and close/drop cleanup.

src/syscall.rs::talos_bind_listen_errors_are_deterministic_and_do_not_mutate_state
retains scalar-dispatch `ENOTSUP`, listen-before-bind, reserved-argument,
endpoint, backlog, non-socket descriptor, wrong-owner backing, and
state-preservation controls.

src/syscall.rs::talos_socket_opens_af_inet_stream_descriptor_and_close_drops_backing,
src/syscall.rs::talos_socket_errors_are_deterministic_and_do_not_allocate_on_failure,
and src/syscall.rs::talos_socket_close_rejects_wrong_owner_socket_backing retain
the unchanged accepted open/close diagnostic behavior.

src/syscall.rs::socket_number_requires_socket_table_context_in_scalar_dispatch
retains unchanged scalar syscall vocabulary outside the socket-table-aware
process descriptor path.

src/local_command_loop.rs::local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers
retains the unchanged `/bin/pingdiag` regression/control surface.

## Rejected Claims

This transcript does not accept Pi 5 hardware behavior, hardwareTestLock
acquisition, lab mutation, boot publication, generated-root publication, live
driver adapters, live packet I/O, hardware reachability, send, recv, connect,
accept, poll/blocking network I/O, UDP/TCP payload transport, SSH, smoltcp,
broad socket expansion, public stable socket ABI acceptance, or phase
transition.
