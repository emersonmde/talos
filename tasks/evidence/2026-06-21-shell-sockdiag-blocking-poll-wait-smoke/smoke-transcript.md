# Shell Sockdiag Blocking Poll Wait Smoke Transcript

Task: phase12-network-shell-sockdiag-blocking-poll-wait-smoke-20260621

Evidence level: host/QEMU-substitute smoke over shell-visible VFS/userspace
bounded blocking socket poll-wait execution.

Command transcript:

- scripts/qemu-shell-sockdiag-blocking-poll-wait-smoke.sh:
  tasks/evidence/2026-06-21-shell-sockdiag-blocking-poll-wait-smoke/qemu-shell-sockdiag-blocking-poll-wait-smoke.log
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-blocking-poll-wait-smoke/source-anchors.txt

The script records the shell-visible /bin/sockdiag blocking poll-wait boundary
and invokes focused test filters. The current no_std QEMU test runner executes
the full target test binary for each filtered invocation, so each filtered
command reports the full 683-test suite while the transcript labels the
intended boundary checks. The retained command log contains six passing
683-test invocations and ends with
classification=host-substitute-shell-sockdiag-blocking-poll-wait-smoke-complete.

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
4. The retained blocking wait path calls TALOS_POLL_WAIT_SYSCALL = 14 through
   the accepted wait-aware socket-table dispatch, SocketPollWaitTable,
   scheduler TaskState::Blocked registration, and
   SingleCoreScheduler::make_runnable resume surface.
5. The retained shell output includes poll-wait-immediate=READ,
   poll-wait-pending-listener=READ, poll-wait-payload-recv=READ,
   poll-wait-timeout=0, poll-wait-peer-hangup=READ | HANGUP,
   poll-wait-blocked-state=blocked, poll-wait-ready-state=runnable,
   poll-wait-timeout-state=runnable, poll-wait-ready-count=1, and
   poll-wait-timeout-tick=22.
6. The diagnostic preserves accepted open/close, bind/listen, connect/accept,
   send/recv, nonblocking poll, and close/drop behavior, then records waitpid
   plus laststatus for the VFS/userspace process lifecycle.

This distinguishes the retained path from a fake shell command or busy loop:
the command is resolved through VFS executable identity before exercising the
accepted private socket syscall, wait-aware dispatch, SocketPollWaitTable,
task state, scheduler resume, UserMapping copy path, close/drop, waitpid, and
laststatus layers.

## Deterministic Controls

The retained shell transcript includes:

- scalar TALOS_POLL_WAIT dispatch ENOTSUP, invalid timeout EINVAL, and
  unsupported wait events EINVAL controls.
- malformed exec /bin/sockdiag arguments and missing /bin/sockdiag executable
  identity.
- unchanged socket open/close, bind/listen, connect/accept, send/recv,
  nonblocking TALOS_POLL, close, waitpid, and laststatus behavior.
- unchanged /bin/pingdiag regression/control coverage.
- bounded private syscall vocabulary including TALOS_POLL_WAIT only inside
  the explicit wait-aware dispatch context.

src/syscall.rs::talos_poll_wait_fast_path_preserves_nonblocking_poll_readiness
retains the immediate-ready fast path and compatibility with nonblocking
TALOS_POLL readiness.

src/syscall.rs::talos_poll_wait_blocks_and_wakes_on_local_socket_payload_readiness
retains process-local blocking wait registration and wake after local payload
send/readiness.

src/syscall.rs::talos_poll_wait_wakes_listener_accept_and_peer_hangup retains
pending listener accept wake and peer close/hangup wake behavior.

src/syscall.rs::talos_poll_wait_timeout_and_malformed_calls_are_bounded
retains deterministic timeout/no-false-ready and malformed argument controls.

src/local_command_loop.rs::local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers
retains the unchanged /bin/pingdiag regression/control surface.

## Rejected Claims

This transcript does not accept runtime implementation beyond smoke/evidence
harness work, Pi 5 hardware behavior, hardwareTestLock acquisition, lab
mutation, boot publication, generated-root publication, live driver adapters,
live packet I/O, hardware reachability, UDP/TCP payload transport, SSH,
smoltcp, cross-process/global poll sets, broad socket expansion, public stable
socket ABI acceptance, or phase transition.
