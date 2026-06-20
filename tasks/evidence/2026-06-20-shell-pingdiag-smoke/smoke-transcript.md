# Shell Pingdiag Smoke Transcript

Task: phase12-network-shell-pingdiag-smoke-20260620

Evidence level: host/QEMU-substitute smoke over shell-visible VFS/userspace
diagnostic execution.

Command transcript:

- scripts/qemu-shell-pingdiag-smoke.sh:
  tasks/evidence/2026-06-20-shell-pingdiag-smoke/qemu-shell-pingdiag-smoke.log
- Source anchors:
  tasks/evidence/2026-06-20-shell-pingdiag-smoke/source-anchors.txt

The script records the shell-visible `/bin/pingdiag` boundary and invokes
focused test filters. The current no_std QEMU test runner executes the full
target test binary for each invocation, so each filtered command reports the
full 663-test suite while the transcript labels the intended boundary checks.

## Lifecycle

The retained positive shell path is
src/local_command_loop.rs::local_command_loop_execs_shell_visible_pingdiag_through_vfs_diagnostic_layers.

1. The shell receives `exec /bin/pingdiag` and resolves the read-only
   initramfs executable identity through the existing VFS open/read execution
   path.
2. The existing exec path records startup ABI and lifecycle state before
   running the task-owned diagnostic SVC lifecycle.
3. UserMapping-backed memory provides copied input payload and caller-owned
   pump/status buffers for the diagnostic.
4. The process-local descriptor opens and starts the ping-like transaction.
5. `PacketQueueNetworkDevice::pump_driver` transfers the outbound ARP request
   to trait-level NetworkDevice transmit behavior; the smoke injects a matching
   ARP reply through the driver queue and pumps it back to diagnostic receive
   state.
6. `PacketQueueNetworkDevice::pump_driver` transfers the outbound IPv4/ICMP
   echo request to trait-level NetworkDevice transmit behavior; the smoke
   injects a matching ICMP echo reply through the driver queue and pumps it
   back to diagnostic receive state.
7. The shell transcript observes completed result/status copy-out, closes the
   process-local descriptor, reaps the process through `waitpid`, and reports
   the same lifecycle status through `laststatus`.

This distinguishes the retained path from a kernel-backed fake shell command:
the command is resolved through VFS executable identity and then exercises the
accepted diagnostic SVC, descriptor, UserMapping, packet queue, and pump layers.

## Deterministic Controls

The retained shell transcript includes:

- malformed `exec /bin/pingdiag` arguments.
- missing `/bin/pingdiag` executable identity.
- `waitpid` and `laststatus` lifecycle observation.

src/syscall.rs::vfs_ping_diagnostic_svc_fixture_maps_packet_queue_controls
retains packet-queue diagnostic controls for queue capacity/backpressure,
frame-capacity rejection, caller output-buffer pressure, malformed received
frames, retry/timeout, receive/transmit device errors, and invalid descriptor
handling.

src/syscall.rs::vfs_ping_diagnostic_svc_fixture_maps_contract_error_controls
and
src/syscall.rs::process_local_ping_user_arguments_reject_malformed_owner_descriptor_capacity_and_buffers
retain VFS/user-argument controls for missing owner, wrong owner, invalid and
closed descriptors, process descriptor capacity, invalid user memory, scratch
pressure, caller buffer pressure, close/drop behavior, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.

src/syscall.rs::vfs_ping_diagnostic_svc_fixture_records_packet_queue_lifecycle
retains the ARP and ICMP packet progression used by the shell-visible path.

## Rejected Claims

This transcript does not accept Pi 5 hardware behavior, live driver adapters,
live packet I/O, hardware reachability, lab mutation, boot publication, public
sockets, stable/socket ABI acceptance, SSH, smoltcp, UDP/TCP, broad shell
expansion, broad socket expansion, Phase 12.1 hardware retry, or phase
transition.
