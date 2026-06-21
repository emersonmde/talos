# Phase 12.4 Driver Packet Adapter Contract

Task: phase12-network-driver-packet-adapter-contract-20260621

Status: accepted

Classification: phase12-network-driver-packet-adapter-contract-accepted

## Scope

Define the next Talos-owned driver packet adapter substrate after the accepted
private userspace socket ABI closeout. The contract ties already accepted
packet queue, driver packet pump, smoltcp packet-device adapter, private
descriptor-backed socket bridge, userspace_socket_abi, /bin/pingdiag, and
/bin/sockdiag evidence into a single bounded source/unit implementation target.

This task is contract and documentation work only. It does not implement runtime
behavior, change Cargo dependencies, acquire hardwareTestLock, mutate the lab,
publish a boot image, publish a generated root, power-cycle the Pi 5, program a
live driver, claim live packet I/O, prove hardware reachability, accept SSH,
add UDP/raw sockets, add libc/std socket wrappers, accept public POSIX/Linux
compatibility, broaden sockets, or transition phase.

## Findings And Dispositions

- fixed: Identified the concrete source anchors for the adapter substrate:
  src/network.rs NetworkDevice, DeviceError, PacketQueueFrame,
  FixedPacketQueue, PacketQueueNetworkDevice, PacketQueueDriverPumpStep,
  SmoltcpPacketDeviceAdapter, SmoltcpPacketDeviceAdapterReceiveResult,
  SmoltcpPacketDeviceAdapterTransmitResult, SmoltcpSocketBridgeRecord,
  NetworkSocketDescriptorTable, NetworkSocketState,
  NetworkSocketReadiness, and NetworkPingOperationDescriptorTable.
- fixed: Identified the shell and ABI anchors that the future diagnostics must
  preserve: src/userspace_socket_abi.rs SocketAbiCall and PollEntry helpers,
  src/syscall.rs socket dispatch, poll-wait dispatch, and TALOS_* selector
  vocabulary, plus src/local_command_loop.rs /bin/sockdiag and /bin/pingdiag
  execution records.
- fixed: Selected the next implementation contract as a host-only, fixed-
  capacity driver packet adapter substrate that exposes driver RX input and TX
  output through the already accepted PacketQueueNetworkDevice and
  SmoltcpPacketDeviceAdapter boundaries.
- fixed: Defined RX ownership. Future source/unit work may inject driver-owned
  received Ethernet frames into a fixed-capacity adapter RX queue; frames are
  copied into PacketQueueFrame storage before smoltcp receives them, so no
  borrowed driver buffer may outlive one adapter step.
- fixed: Defined TX ownership. Future source/unit work may observe smoltcp-
  produced transmit frames as fixed PacketQueueFrame records in an adapter TX
  queue; a later live driver task must separately own DMA/buffer handoff before
  any hardware claim.
- fixed: Defined bounded capacities and backpressure. RX queue full,
  TX queue full, receive scratch pressure, frame-too-large, WouldBlock, and
  DeviceError mappings must stay explicit and deterministic through source/unit
  evidence.
- fixed: Defined scheduling/time progression. The next core must remain
  caller-driven and deterministic: one adapter step may inject at most one RX
  frame, expose at most one TX frame, and poll smoltcp with an explicit
  smoltcp::time::Instant supplied by the test harness or diagnostic owner.
- fixed: Defined diagnostic observability for the next shell task. /bin/sockdiag
  may later report bounded adapter RX/TX queue counts, last RX/TX result names,
  smoltcp bridge continuity, waitpid, and laststatus, but it must label the
  evidence as host/QEMU-substitute adapter-substrate evidence only.
- removed: No runtime source behavior, hardware path, live driver adapter,
  lab artifact, boot artifact, shell command expansion, public ABI claim, or
  phase transition was added by this contract.
- deferred: Source/unit implementation, /bin/sockdiag diagnostic observation,
  retained smoke evidence, closeout reconciliation, live hardware RX/TX
  coupling, Pi 5 reachability proof, SSH strategy, libc/std wrappers, UDP/raw
  sockets, and public ABI/POSIX claims remain future explicit tasks.
- not-an-issue: The contract uses smoltcp for TCP/IP behavior because smoltcp
  was already accepted as the Phase 12.4 TCP/IP stack dependency; this task
  keeps Talos ownership at the packet adapter, descriptor, syscall, diagnostic,
  scheduling, and evidence boundaries.

## Selected Contract

The future core should add the thinnest source/unit driver packet adapter
substrate that couples accepted Talos packet queues to the accepted smoltcp
packet-device and socket bridge layers:

- frame ownership: copied fixed-capacity PacketQueueFrame records are the only
  accepted frame storage at the adapter boundary;
- RX input: a caller-driven adapter step may accept one driver RX frame and
  enqueue it for SmoltcpPacketDeviceAdapter receive handling;
- TX output: smoltcp-generated transmit records remain queued until the caller
  explicitly observes or drains them;
- queue bounds: RX_CAPACITY, TX_CAPACITY, and FRAME_CAPACITY must be compile-
  time capacities with deterministic Full and FrameTooLarge behavior;
- backpressure: transmit queue pressure must prevent smoltcp from consuming an
  RX frame when no paired TX token can be produced; receive queue pressure must
  avoid consuming driver-owned input;
- error mapping: DeviceError::WouldBlock, DeviceError::BufferTooSmall, and
  DeviceError::Io must map to explicit adapter observations, not silent loss;
- time progression: smoltcp polling uses caller-supplied Instant values, with
  tests fixing step counts and frame movement rather than relying on ambient
  hardware time;
- socket continuity: the private descriptor-backed AF_INET/SOCK_STREAM bridge
  must retain Established client/server states, accepted descriptor attachment,
  bounded payload transfer, poll/readiness behavior, and userspace_socket_abi
  helper coverage as regressions;
- shell diagnostics: later /bin/sockdiag output may expose adapter queue/result
  state only through the accepted VFS/userspace/private socket ABI path and
  must retain /bin/pingdiag and local socket controls.

The next core must not program RP1/GEM/MACB registers, own DMA descriptors,
handle interrupts, mutate boot or lab state, claim live packet I/O, claim Pi 5
hardware behavior, claim hardware reachability, add SSH, add UDP/raw sockets,
add libc/std wrappers, accept a public stable ABI, accept POSIX/Linux
compatibility, broadly expand sockets, or transition phase.

## Evidence Reviewed

- Accepted userspace ABI closeout:
  tasks/2026-06-21-phase12-network-shell-sockdiag-userspace-abi-closeout.md.
- Userspace socket ABI contract/core/shell/smoke records:
  tasks/2026-06-21-phase12-network-socket-userspace-abi-contract.md,
  tasks/2026-06-21-phase12-network-socket-userspace-abi-core.md,
  tasks/2026-06-21-phase12-network-shell-sockdiag-userspace-abi-core.md,
  and tasks/2026-06-21-phase12-network-shell-sockdiag-userspace-abi-smoke.md.
- Retained ABI smoke evidence:
  tasks/evidence/2026-06-21-shell-sockdiag-userspace-abi-smoke/.
- Driver packet pump contract/core/smoke/closeout records:
  tasks/2026-06-20-phase12-network-driver-packet-pump-contract.md,
  tasks/2026-06-20-phase12-network-driver-packet-pump-core.md,
  tasks/2026-06-20-phase12-network-driver-packet-pump-smoke.md,
  and tasks/2026-06-20-phase12-network-driver-packet-pump-smoke-closeout.md.
- smoltcp adoption, packet-device adapter, TCP handshake, socket bridge, and
  shell diagnostic records:
  tasks/2026-06-21-phase12-network-smoltcp-adoption-contract.md,
  tasks/2026-06-21-phase12-network-smoltcp-packet-device-adapter-core.md,
  tasks/2026-06-21-phase12-network-smoltcp-loopback-tcp-handshake-core.md,
  tasks/2026-06-21-phase12-network-smoltcp-socket-bridge-contract.md,
  tasks/2026-06-21-phase12-network-smoltcp-socket-bridge-core.md,
  tasks/2026-06-21-phase12-network-shell-sockdiag-smoltcp-tcp-core.md,
  tasks/2026-06-21-phase12-network-shell-sockdiag-smoltcp-tcp-smoke.md,
  and tasks/2026-06-21-phase12-network-shell-sockdiag-smoltcp-tcp-closeout.md.
- Source: src/network.rs, src/userspace_socket_abi.rs, src/syscall.rs,
  src/local_command_loop.rs, src/posix.rs, and src/initramfs.rs.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- static source/task/docs/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Rust source was touched, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's gates.
No Pi 5 hardware run, hardwareTestLock acquisition, lab mutation, boot
publication, generated-root publication, power cycle, live driver programming,
live packet I/O, hardware reachability, SSH, UDP/raw sockets, libc/std socket
wrappers, public stable ABI/POSIX/Linux compatibility acceptance, broad socket
expansion, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-driver-packet-adapter-core-20260621.

The accepted evidence level is static source/task/docs/evidence review for a
host-only Talos driver packet adapter substrate contract. The future core is
mechanically bounded to fixed-capacity RX/TX frame ownership, deterministic
backpressure/error mapping, caller-driven smoltcp time progression, and source/
unit regression coverage over the accepted packet queue, smoltcp packet-device,
private socket bridge, userspace_socket_abi, /bin/pingdiag, and /bin/sockdiag
surfaces. Live packet I/O, Pi 5 hardware behavior, hardware reachability, SSH,
UDP/raw sockets, libc/std wrappers, POSIX/Linux compatibility, public stable
ABI acceptance, broad socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
