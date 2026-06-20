# Shell Sockdiag Connect/Accept Smoke Transcript

Task: phase12-network-shell-sockdiag-connect-accept-smoke-20260620

Evidence level: host/QEMU-substitute smoke over shell-visible VFS/userspace
socket connect/accept execution.

Command transcript:

- scripts/qemu-shell-sockdiag-connect-accept-smoke.sh:
  tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/qemu-shell-sockdiag-connect-accept-smoke.log
- Source anchors:
  tasks/evidence/2026-06-20-shell-sockdiag-connect-accept-smoke/source-anchors.txt

The script records the shell-visible `/bin/sockdiag` boundary and invokes
focused test filters. The current no_std QEMU test runner executes the full
target test binary for each filtered invocation, so each filtered command
reports the full 673-test suite while the transcript labels the intended
boundary checks. The retained command log contains eleven passing 673-test
invocations and ends with
`classification=host-substitute-shell-sockdiag-connect-accept-smoke-complete`.

## Lifecycle

The retained positive shell path is
src/local_command_loop.rs::local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls.

1. The shell receives `exec /bin/sockdiag` and resolves the read-only
   initramfs executable identity through the existing VFS open/read execution
   path.
2. The exec path records startup ABI and lifecycle state before running the
   task-owned sockdiag diagnostic.
3. The diagnostic opens a listener with `AF_INET=2`, `SOCK_STREAM=1`,
   `protocol=0` through `TALOS_SOCKET_SYSCALL = 6`, binds it to
   `127.0.0.1:8080` through `TALOS_BIND_SYSCALL = 7`, and listens through
   `TALOS_LISTEN_SYSCALL = 8`.
4. The diagnostic opens a client descriptor, connects it through
   `TALOS_CONNECT_SYSCALL = 9`, and accepts it through
   `TALOS_ACCEPT_SYSCALL = 10`, producing listener fd 3, client fd 4, and
   accepted fd 6 in the retained shell output.
5. The listener remains in Listening state, the client records Connected state,
   and the accepted server-side descriptor records Accepted state.
6. The diagnostic closes accepted, client, and listener descriptors through
   `TALOS_CLOSE_SYSCALL = 2`, verifies backing drop, and records
   closed-descriptor `EBADF` behavior.
7. The shell reaps the diagnostic through `waitpid` and reports the same
   lifecycle result through `laststatus`.

This distinguishes the retained path from a kernel-backed fake shell command:
the command is resolved through VFS executable identity before exercising the
accepted private socket syscall, bind/listen/connect/accept state, process
descriptors, close/drop, waitpid, and laststatus layers.

## Deterministic Controls

The retained shell transcript includes:

- malformed `exec /bin/sockdiag` arguments.
- missing `/bin/sockdiag` executable identity.
- unsupported domain, unsupported type, and unsupported protocol controls.
- listen-before-bind, invalid bind endpoint, invalid backlog, repeated bind,
  repeated listen backlog update, accept-before-connect, missing listener, full
  pending queue, non-socket descriptor, invalid or closed descriptor, and
  double-close `EBADF`.
- `waitpid` and `laststatus` lifecycle observation.

src/syscall.rs::talos_connect_accept_records_local_handshake_state retains the
successful listener/client/accepted local handshake, synthetic client endpoint,
accepted socket state, and close/drop cleanup.

src/syscall.rs::talos_connect_accept_errors_are_all_or_nothing and
src/syscall.rs::talos_accept_rejects_capacity_failures_without_dequeueing_peer
retain scalar-dispatch `ENOTSUP`, reserved-argument, listener absence, empty
accept, full queue, process descriptor capacity, socket backing capacity,
non-socket descriptor, wrong-owner backing, and no-partial-dequeue controls.

src/syscall.rs::talos_bind_listen_records_socket_state_and_close_drops_backing
and src/syscall.rs::talos_bind_listen_errors_are_deterministic_and_do_not_mutate_state
retain unchanged accepted bind/listen behavior.

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
driver adapters, live packet I/O, hardware reachability, send, recv,
payload bytes, poll/blocking network I/O, UDP/TCP payload transport, SSH,
smoltcp, broad socket expansion, public stable socket ABI acceptance, or phase
transition.
