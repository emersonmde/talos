# Shell Sockdiag Readiness/Poll Smoke Transcript

Task: phase12-network-shell-sockdiag-readiness-poll-smoke-20260621

Evidence level: host/QEMU-substitute smoke over shell-visible VFS/userspace
socket readiness/poll execution.

Command transcript:

- scripts/qemu-shell-sockdiag-readiness-poll-smoke.sh:
  tasks/evidence/2026-06-21-shell-sockdiag-readiness-poll-smoke/qemu-shell-sockdiag-readiness-poll-smoke.log
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-readiness-poll-smoke/source-anchors.txt

The script records the shell-visible /bin/sockdiag readiness/poll boundary
and invokes focused test filters. The current no_std QEMU test runner executes
the full target test binary for each filtered invocation, so each filtered
command reports the full 679-test suite while the transcript labels the
intended boundary checks. The retained command log contains five passing
679-test invocations and ends with
classification=host-substitute-shell-sockdiag-readiness-poll-smoke-complete.

## Lifecycle

The retained positive shell path is
src/local_command_loop.rs::local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls.

1. The shell receives exec /bin/sockdiag and resolves the read-only
   initramfs executable identity through the existing VFS open/read execution
   path.
2. The exec path records startup ABI and lifecycle state before running the
   task-owned sockdiag diagnostic.
3. The diagnostic opens a listener with AF_INET=2, SOCK_STREAM=1, protocol=0,
   binds it to 127.0.0.1:8080, listens, connects a local client, and accepts
   the pending peer.
4. Before and after those state transitions, the diagnostic records private
   TALOS_POLL_SYSCALL = 13 revents for listener accept-readiness, empty and
   nonempty recv queues, peer send capacity, peer FIFO backpressure, peer
   close/hangup, invalid descriptors, and non-socket descriptors.
5. The retained shell output includes poll-empty-listener=0,
   poll-pending-listener=READ, poll-empty-recv=0, poll-payload-recv=READ,
   poll-write-ready=WRITE, poll-write-backpressure=0,
   poll-peer-hangup=READ | HANGUP, poll-invalid-descriptor=ERROR, and
   poll-non-socket-descriptor=ERROR.
6. The diagnostic preserves accepted local payload movement, closes accepted,
   client, and listener descriptors through TALOS_CLOSE_SYSCALL = 2, verifies
   backing drop, and records closed-descriptor EBADF behavior.
7. The shell reaps the diagnostic through waitpid and reports the same
   lifecycle result through laststatus.

This distinguishes the retained path from a kernel-backed fake shell command:
the command is resolved through VFS executable identity before exercising the
accepted private socket syscall, poll, descriptor store, socket table,
UserMapping copy path, close/drop, waitpid, and laststatus layers.

## Deterministic Controls

The retained shell transcript includes:

- unsupported poll-event EINVAL, invalid-descriptor ERROR, and
  non-socket-descriptor ERROR controls.
- malformed exec /bin/sockdiag arguments and missing /bin/sockdiag executable
  identity.
- unchanged socket open/close, bind/listen, connect/accept, send/recv, close,
  waitpid, and laststatus behavior.
- unchanged /bin/pingdiag regression/control coverage.
- scalar poll dispatch ENOTSUP, malformed poll flags/count/buffer controls,
  bounded entry count, and bounded syscall vocabulary.

src/syscall.rs::talos_poll_reports_listener_local_payload_and_peer_hangup_readiness
retains listener pending-accept, local read/write readiness, queued payload,
peer close, and hangup readiness.

src/syscall.rs::talos_poll_reports_write_backpressure_and_deterministic_entry_errors
retains FIFO backpressure and per-entry invalid/non-socket/no-owner ERROR
behavior.

src/syscall.rs::talos_poll_rejects_malformed_calls_and_scalar_dispatch_fails_closed
retains whole-call malformed argument failures and scalar-dispatch ENOTSUP.

src/local_command_loop.rs::local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers
retains the unchanged /bin/pingdiag regression/control surface.

## Rejected Claims

This transcript does not accept runtime implementation beyond smoke/evidence
harness work, Pi 5 hardware behavior, hardwareTestLock acquisition, lab
mutation, boot publication, generated-root publication, live driver adapters,
live packet I/O, hardware reachability, blocking waits, scheduler wait queues,
timeout handling, UDP/TCP payload transport, SSH, smoltcp, cross-process/global
poll sets, broad socket expansion, public stable socket ABI acceptance, or
phase transition.
