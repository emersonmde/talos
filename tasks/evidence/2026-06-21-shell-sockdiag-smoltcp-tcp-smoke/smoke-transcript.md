# Shell Sockdiag smoltcp TCP Smoke Transcript

Task: phase12-network-shell-sockdiag-smoltcp-tcp-smoke-20260621

Classification: host-substitute-shell-sockdiag-smoltcp-tcp-smoke-complete

Evidence level: host/QEMU-substitute.

Artifacts:

- scripts/qemu-shell-sockdiag-smoltcp-tcp-smoke.sh:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/qemu-shell-sockdiag-smoltcp-tcp-smoke.log
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-smoltcp-tcp-smoke/source-anchors.txt

The smoke records the shell-visible /bin/sockdiag smoltcp TCP bridge boundary
through VFS/userspace executable lookup, the existing private socket syscall
path, and host-only SmoltcpSocketBridgeRecord evidence.

Command summary:

- local_command_loop_execs_shell_visible_sockdiag_through_vfs_socket_syscalls:
  passed with 693 no_std tests reported by the QEMU-substitute runner.
- talos_smoltcp_socket_bridge_transfers_payload_through_private_syscalls:
  passed with 693 no_std tests reported by the QEMU-substitute runner.
- local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers:
  passed with 693 no_std tests reported by the QEMU-substitute runner.
- Final line:
  qemu-shell-sockdiag-smoltcp-tcp-smoke: PASS classification=host-substitute-shell-sockdiag-smoltcp-tcp-smoke-complete.

Accepted path retained:

1. The shell receives exec /bin/sockdiag and resolves the read-only executable
   identity through VFS lookup, open, and read.
2. The existing /bin/sockdiag userspace diagnostic runs socket, bind, listen,
   connect, accept, send, recv, poll, poll-wait, and close through the private
   syscall dispatch and descriptor ownership path.
3. The diagnostic observes a host-only SmoltcpSocketBridgeRecord whose
   handshake reaches Established on both client and server sides.
4. The accepted descriptor is attached to the bridge record after accept.
5. A bounded client payload send records one smoltcp payload-transfer
   observation, and the existing descriptor-backed recv path reads the payload.
6. The diagnostic emits the smoltcp connection id, handshake states,
   handshake step/frame counters, accepted-descriptor attachment,
   payload-transfer count/length, and Established payload states.
7. waitpid, laststatus, malformed arguments, missing executable identity,
   unchanged local socket diagnostics, unchanged pingdiag, and bounded syscall
   vocabulary remain deterministic controls.

Rejected claims:

- No live packet I/O.
- No Pi 5 hardware behavior or hardware reachability.
- No hardwareTestLock acquisition, lab mutation, boot publication, or
  generated-root publication.
- No SSH, public stable socket ABI acceptance, broad socket expansion,
  UDP/raw sockets, or phase transition.
