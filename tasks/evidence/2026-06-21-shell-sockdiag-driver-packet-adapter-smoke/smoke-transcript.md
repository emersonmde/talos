# Shell Sockdiag Driver Packet Adapter Smoke Transcript

Task: phase12-network-shell-sockdiag-driver-packet-adapter-smoke-20260621

Status: accepted

Evidence level: host/QEMU-substitute.

Retained artifacts:

- scripts/qemu-shell-sockdiag-driver-packet-adapter-smoke.sh:
  tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke/qemu-shell-sockdiag-driver-packet-adapter-smoke.log
- Source anchors:
  tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke/source-anchors.txt

The smoke records the shell-visible /bin/sockdiag driver packet adapter
boundary and invokes the no_std target test binary through the repo QEMU
runner. The retained log ends with:

classification=host-substitute-shell-sockdiag-driver-packet-adapter-smoke-complete.

The command-loop transcript is covered by
src/local_command_loop.rs::local_command_loop_execs_shell_visible_sockdiag_through_userspace_socket_abi.

Accepted path:

1. The shell receives exec /bin/sockdiag and resolves the read-only executable
   through VFS executable lookup/open/read.
2. The VFS/userspace launch boundary starts the task-owned sockdiag diagnostic.
3. The diagnostic reaches private userspace_socket_abi wrappers and the
   descriptor-backed socket dispatch path for socket, bind, listen, connect,
   accept, send, recv, poll, poll_wait, and close.
4. The accepted private smoltcp TCP bridge path still reports Established
   handshake states, accepted descriptor attachment, and one bounded payload
   transfer observation.
5. DriverPacketAdapter evidence records one driver RX frame consumed by the
   smoltcp packet-device path, one smoltcp-produced TX frame observed and
   popped by the driver side, and a separate TX-queue-full backpressure step
   preserving a queued RX frame.
6. waitpid and laststatus observe the VFS/userspace process lifecycle.

Deterministic retained controls:

- malformed exec /bin/sockdiag arguments and missing /bin/sockdiag executable
  identity remain rejected.
- userspace_socket_abi wrapper dispatch, local socket diagnostics, smoltcp TCP
  bridge continuity, and /bin/pingdiag remain covered.
- DriverPacketAdapter copied RX/TX movement, TX backpressure, capacity errors,
  and deterministic DeviceError mapping remain covered.
- Live packet I/O, Pi 5 hardware behavior, hardware reachability, SSH,
  UDP/raw sockets, public ABI/POSIX/Linux compatibility, broad socket
  expansion, and phase transition remain rejected.
