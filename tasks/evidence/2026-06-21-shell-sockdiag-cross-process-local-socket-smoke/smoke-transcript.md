# Shell Sockdiag Cross-Process Local Socket Smoke Transcript

Task: phase12-network-shell-sockdiag-cross-process-local-socket-smoke-20260621

Status: accepted

Evidence level: host/QEMU-substitute.

Retained artifacts:

- scripts/qemu-shell-sockdiag-cross-process-local-socket-smoke.sh:
  tasks/evidence/2026-06-21-shell-sockdiag-cross-process-local-socket-smoke/qemu-shell-sockdiag-cross-process-local-socket-smoke.log
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-cross-process-local-socket-smoke/source-anchors.txt

The smoke records the shell-visible /bin/sockdiag cross-process local socket
boundary and invokes the no_std target test binary through the repo QEMU
runner. The retained log ends with:

classification=host-substitute-shell-sockdiag-cross-process-local-socket-smoke-complete.

The command-loop transcript is covered by
src/local_command_loop.rs::local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls.

Accepted path:

1. The shell receives exec /bin/sockdiag and resolves the read-only executable
   through VFS executable lookup/open/read.
2. The VFS/userspace launch boundary starts the task-owned sockdiag diagnostic.
3. The diagnostic creates a server owner and a distinct client owner, each with
   its own process descriptor table.
4. The server owner creates the listener; the client owner connects; accept
   creates a server-owned accepted descriptor without installing that fd in the
   client descriptor table.
5. Payload moves client-to-server and server-to-client over the private
   cross-process connection.
6. Bounded TALOS_POLL_WAIT evidence covers listener accept wake, payload wake,
   and peer-close hangup readiness.
7. Cleanup closes the accepted and listener descriptors and releases the backing
   socket state.
8. waitpid and laststatus observe the VFS/userspace process lifecycle.

Deterministic retained controls:

- same descriptor number under different process owners does not collapse
  ownership.
- malformed exec /bin/sockdiag arguments and missing /bin/sockdiag executable
  identity remain rejected.
- process-local open/close, bind/listen, connect/accept, send/recv,
  readiness/poll, and blocking poll-wait diagnostic markers remain covered.
- /bin/pingdiag remains covered through its prior VFS diagnostic layer.
- UDP/TCP payload transport, smoltcp integration, live packet I/O, hardware
  reachability, SSH, public socket ABI acceptance, broad socket expansion, and
  phase transition remain rejected.
