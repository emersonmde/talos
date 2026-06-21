# Phase 12.4 Driver Packet Adapter Closeout

Task: phase12-network-driver-packet-adapter-closeout-20260621

Status: accepted

Classification: phase12-network-driver-packet-adapter-closeout-accepted

## Scope

Reconcile the accepted contract, core adapter, shell diagnostic, retained
smoke evidence, documentation, accepted claims, rejected claims, and remaining
live I/O/SSH gaps for the Phase 12.4 driver packet adapter substrate.

This task is closeout and documentation work only. It does not add runtime
implementation, acquire hardwareTestLock, mutate the lab, publish a boot image,
publish a generated root, power-cycle the Pi 5, program a live driver, perform
live packet I/O, claim hardware reachability, accept SSH, add UDP/raw sockets,
add libc/std socket wrappers, accept POSIX/Linux compatibility, accept a public
stable socket ABI, broaden sockets, or transition phase.

## Findings And Dispositions

- fixed: Reconciled the accepted driver packet adapter chain across contract,
  source/unit core, shell-visible /bin/sockdiag diagnostic, and retained
  host/QEMU-substitute smoke evidence.
- fixed: Confirmed the accepted evidence level is source/unit plus retained
  host/QEMU-substitute evidence for a deterministic DriverPacketAdapter
  substrate through the accepted VFS/userspace/private socket ABI path.
- fixed: Confirmed the retained smoke evidence archives VFS executable
  lookup/open/read, startup ABI, userspace_socket_abi wrapper dispatch,
  descriptor-backed socket dispatch, private smoltcp TCP bridge continuity,
  deterministic driver RX consumption, smoltcp TX observation and driver-side
  pop, TX backpressure preserving queued RX, capacity/error controls, waitpid,
  laststatus, malformed/missing executable controls, unchanged local socket
  diagnostics, unchanged /bin/pingdiag behavior, and bounded syscall
  vocabulary.
- fixed: Updated the Phase 12 networking project doc and roadmap to freeze the
  accepted adapter boundary and remaining gaps before supervisor planning.
- removed: No source behavior, hardware/lab action, boot publication,
  generated-root publication, live packet I/O claim, hardware reachability
  claim, SSH claim, UDP/raw socket claim, libc/std wrapper claim, public ABI
  claim, POSIX/Linux compatibility claim, broad expansion claim, or phase
  transition claim was added.
- deferred: Real hardware RX/TX coupling, packet scheduling/backpressure on
  live hardware, reachability proof, SSH strategy, entropy, host keys, service
  shape, exposure controls, libc/std wrappers, UDP/raw sockets, and any public
  ABI/POSIX/Linux compatibility claim remain future supervisor-planned work.
- not-an-issue: The adapter remains driver-named while host/QEMU-substitute
  only because its accepted boundary is copied PacketQueueFrame records and
  smoltcp packet-device/socket bridge state, not RP1 GEM MMIO, DMA descriptors,
  interrupts, PHY/MDIO, or live hardware packet ownership.

## Accepted Boundary

The completed driver packet adapter slice accepts a Talos-owned,
fixed-capacity packet substrate between accepted packet queues and accepted
smoltcp socket bridge diagnostics:

- copied PacketQueueFrame records are the only accepted frame ownership
  boundary at the adapter layer;
- inject_driver_rx supplies deterministic driver-side RX input;
- receive_one_for_smoltcp consumes at most one RX token using a caller-supplied
  smoltcp::time::Instant;
- transmit_one_from_smoltcp records at most one smoltcp-produced TX frame;
- pop_driver_tx exposes the copied driver-visible TX record;
- queue full, frame too large, buffer too small, would-block, and I/O error
  outcomes remain explicit and deterministic;
- /bin/sockdiag observes the adapter through the accepted VFS/userspace/private
  socket ABI path and labels the evidence
  host-qemu-substitute-not-live-packet-io.

The accepted evidence is source/unit plus retained host/QEMU-substitute only.
It does not prove live driver programming, live packet I/O, Pi 5 hardware
behavior, hardware reachability, SSH, UDP/raw sockets, libc/std wrappers,
POSIX/Linux compatibility, public stable socket ABI acceptance, broad socket
expansion, or a phase transition.

## Remaining Gaps

- Live RP1/GEM RX/TX coupling with real driver buffer ownership, DMA handoff,
  interrupt or polling integration, and packet scheduling/backpressure on live
  hardware.
- Hardware reachability evidence, including serialized Pi 5 lab runs with
  artifact digests, TFTP/serial capture, and post-hardware review.
- SSH strategy, including entropy, host key generation/provisioning,
  authorized key storage, service shape, authentication policy, time handling,
  heap-pressure expectations, and exposure controls.
- libc/std socket wrappers, UDP/raw sockets, public stable ABI/POSIX/Linux
  compatibility, and any broad socket expansion beyond the private diagnostic
  ABI accepted so far.

## Evidence Reviewed

- Driver packet adapter contract:
  tasks/2026-06-21-phase12-network-driver-packet-adapter-contract.md.
- Driver packet adapter source/unit core:
  tasks/2026-06-21-phase12-network-driver-packet-adapter-core.md.
- Shell-visible /bin/sockdiag driver packet adapter core:
  tasks/2026-06-21-phase12-network-shell-sockdiag-driver-packet-adapter-core.md.
- Retained shell-visible smoke:
  tasks/2026-06-21-phase12-network-shell-sockdiag-driver-packet-adapter-smoke.md.
- Retained smoke artifacts:
  tasks/evidence/2026-06-21-shell-sockdiag-driver-packet-adapter-smoke/.
- Source anchors:
  src/network.rs DriverPacketAdapter,
  SmoltcpPacketDeviceAdapter, PacketQueueNetworkDevice, and source/unit tests;
  src/local_command_loop.rs /bin/sockdiag output and shell-visible tests;
  src/userspace_socket_abi.rs private socket ABI wrappers; and src/syscall.rs
  descriptor-backed socket dispatch.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed with existing large search-index
  warning behavior.
- git diff --cached --check: passed before commit.

No Rust/source behavior changed, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's gates.
No Pi 5 hardware run, hardwareTestLock acquisition, lab mutation, boot
publication, generated-root publication, power cycle, live driver programming,
live packet I/O, hardware reachability, SSH, UDP/raw sockets, libc/std socket
wrappers, POSIX/Linux compatibility, public stable socket ABI acceptance, broad
socket expansion, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=null.

planningNeeded=true.

The driver packet adapter substrate is closed out at source/unit plus retained
host/QEMU-substitute evidence. Supervisor planning is required before any next
bounded socket/network task, live hardware RX/TX coupling, hardware
reachability proof, SSH work, libc/std socket wrapper work, UDP/raw sockets,
public ABI/POSIX/Linux compatibility acceptance, broad expansion, or phase
transition.

Commit: recorded in durable supervisor state after commit creation.
