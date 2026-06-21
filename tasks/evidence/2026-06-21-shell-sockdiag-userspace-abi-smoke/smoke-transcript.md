# Shell Sockdiag Userspace ABI Smoke Transcript

Task: phase12-network-shell-sockdiag-userspace-abi-smoke-20260621

Classification: host-substitute-shell-sockdiag-userspace-abi-smoke-complete

Evidence level: host/QEMU-substitute.

Artifacts:

- scripts/qemu-shell-sockdiag-userspace-abi-smoke.sh:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/qemu-shell-sockdiag-userspace-abi-smoke.log
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/source-anchors.txt

The smoke records shell-visible /bin/sockdiag execution through VFS/userspace,
the documented private userspace_socket_abi helper surface, the existing
descriptor-backed socket dispatch, and host-only SmoltcpSocketBridgeRecord
evidence.

Command summary:

- local_command_loop_execs_shell_visible_sockdiag_through_userspace_socket_abi:
  passed with 695 no_std tests reported by the QEMU-substitute runner.
- userspace_socket_abi_constants_match_private_kernel_contract:
  passed with 695 no_std tests reported by the QEMU-substitute runner.
- userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge:
  passed with 695 no_std tests reported by the QEMU-substitute runner.
- local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers:
  passed with 695 no_std tests reported by the QEMU-substitute runner.
- Final line:
  qemu-shell-sockdiag-userspace-abi-smoke: PASS classification=host-substitute-shell-sockdiag-userspace-abi-smoke-complete.

Accepted path retained:

1. The shell receives exec /bin/sockdiag and resolves the read-only executable
   identity through VFS lookup, open, and read.
2. The /bin/sockdiag diagnostic records userspace-socket-abi-v1 and builds
   userspace_socket_abi SocketAbiCall values for socket, bind, listen,
   connect, accept, send, recv, poll, poll_wait, and close.
3. The ABI-routed calls reach the existing descriptor-backed socket dispatch
   and the accepted host-only smoltcp TCP bridge diagnostic.
4. The diagnostic observes Established client/server handshake state,
   deterministic frame/step counters, accepted-descriptor attachment, one
   bounded payload-transfer observation, and descriptor-backed recv delivery.
5. waitpid, laststatus, malformed arguments, missing executable identity,
   unchanged local socket diagnostics, unchanged /bin/pingdiag behavior, ABI
   constant/wrapper coverage, and bounded syscall vocabulary remain
   deterministic controls.

Rejected claims:

- No live packet I/O.
- No Pi 5 hardware behavior or hardware reachability.
- No hardwareTestLock acquisition, lab mutation, boot publication, or
  generated-root publication.
- No SSH, UDP/raw sockets, broad socket expansion, POSIX/Linux compatibility,
  public stable socket ABI acceptance, or phase transition.
