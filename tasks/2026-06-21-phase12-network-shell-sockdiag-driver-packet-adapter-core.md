# Phase 12.4 Shell Sockdiag Driver Packet Adapter Core

Task: phase12-network-shell-sockdiag-driver-packet-adapter-core-20260621

Status: accepted

Classification: phase12-network-shell-sockdiag-driver-packet-adapter-core-accepted

## Scope

Expose the accepted DriverPacketAdapter substrate through the existing
shell-visible /bin/sockdiag VFS/userspace diagnostic as deterministic
source/unit plus host/QEMU-substitute evidence. The diagnostic remains routed
through read-only VFS executable lookup/open/read, startup ABI,
src/userspace_socket_abi.rs wrapper constructors, descriptor-backed socket
dispatch, private smoltcp TCP bridge reporting, waitpid, and laststatus.

This task does not accept retained smoke evidence, live driver programming,
live packet I/O, Pi 5 hardware behavior, hardware reachability, lab mutation,
boot publication, generated-root publication, SSH, UDP/raw sockets, libc/std
socket wrappers, POSIX/Linux compatibility, public stable ABI acceptance, broad
socket expansion, or phase transition.

## Findings And Dispositions

- fixed: /bin/sockdiag now reports driver-packet-rx, driver-packet-tx, observed
  TX length, post-pop TX queue state, and backpressure RX/TX queue state from
  the accepted DriverPacketAdapter substrate.
- fixed: The diagnostic labels adapter evidence as
  host-qemu-substitute-not-live-packet-io so the transcript cannot be read as
  hardware reachability or live packet I/O evidence.
- fixed: The existing shell-visible sockdiag test proves the path still reaches
  /bin/sockdiag through VFS executable lookup/open/read, startup ABI,
  userspace_socket_abi wrappers, descriptor-backed socket dispatch, the
  private smoltcp TCP bridge, waitpid, and laststatus.
- fixed: Existing local socket, smoltcp TCP bridge, userspace_socket_abi,
  /bin/pingdiag, malformed argument, missing executable, waitpid, and
  laststatus regression surfaces remain covered by the full cargo gate.
- removed: No smoke script, generated-root publication, lab mutation, boot
  publication, hardwareTestLock acquisition, Pi 5 run, live packet I/O claim,
  UDP/raw socket surface, SSH surface, public ABI claim, broad expansion, or
  phase transition was added.
- not-an-issue: The adapter diagnostic uses deterministic canned frame records
  because this task is explicitly host/source-unit substrate observation; live
  driver coupling remains a later explicit task.
- deferred: Retained host/QEMU-substitute smoke evidence remains the explicit
  follow-up task.

## Implementation

- src/local_command_loop.rs:
  - LocalCommandSockdiagRecord now carries driver packet adapter RX/TX and
    backpressure fields.
  - exec_shell_sockdiag_diagnostic injects one deterministic driver RX frame,
    observes one smoltcp-consumed RX step, records one smoltcp-produced TX
    frame, pops the driver-visible TX record, and separately records a
    TX-queue-full receive backpressure step while preserving the queued RX
    frame.
  - write_exec_sockdiag_line prints the adapter fields on the existing
    shell-visible sockdiag line with host-only evidence labeling.
- docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md record the
  accepted diagnostic frontier and rejected claims.

## Evidence

- Focused source/unit diagnostic test:
  - local_command_loop_execs_shell_visible_sockdiag_through_userspace_socket_abi
- Regression anchors:
  - driver_packet_adapter_moves_driver_rx_and_smoltcp_tx_with_copied_frames
  - driver_packet_adapter_preserves_rx_when_tx_backpressure_blocks_smoltcp_receive
  - driver_packet_adapter_maps_capacity_and_device_errors_deterministically
  - userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge
- Source anchors:
  - src/local_command_loop.rs /bin/sockdiag VFS/userspace diagnostic output.
  - src/network.rs DriverPacketAdapter, SmoltcpPacketDeviceAdapter,
    PacketQueueNetworkDevice, and SmoltcpSocketBridgeRecord.
  - src/userspace_socket_abi.rs private socket ABI wrappers and PollEntry
    codec.
  - src/syscall.rs descriptor-backed socket dispatch and poll/wait surfaces.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop_execs_shell_visible_sockdiag_through_userspace_socket_abi: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed with existing large search-index
  warning behavior.
- git diff --cached --check: passed before commit.

No retained smoke evidence, live driver programming, live packet I/O, Pi 5
hardware behavior, hardware reachability, lab mutation, boot publication,
generated-root publication, SSH, UDP/raw sockets, libc/std socket wrappers,
POSIX/Linux compatibility, public stable ABI acceptance, broad socket
expansion, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-driver-packet-adapter-smoke-20260621.

The accepted evidence level is source/unit plus host/QEMU-substitute only. The
shell-visible /bin/sockdiag diagnostic now exposes deterministic driver packet
adapter RX/TX and backpressure state through the accepted VFS/userspace/private
socket ABI path while preserving local socket, smoltcp TCP bridge,
userspace_socket_abi, /bin/pingdiag, waitpid, laststatus, and negative-control
regression surfaces.

Commit: recorded in durable supervisor state after commit creation.
