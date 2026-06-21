# Phase 12.4 smoltcp TCP Frontier Closeout

Task: phase12-network-smoltcp-tcp-frontier-closeout-20260621

Status: accepted

Classification: phase12-network-smoltcp-tcp-frontier-closeout-accepted

## Scope

Close out the initial smoltcp/TCP host-only frontier after the accepted
adoption contract, no_std dependency core, packet-device adapter core, and
loopback TCP handshake core. This reconciliation records the accepted evidence
level, accepted claims, rejected claims, remaining bridge gaps, and planning
status before any socket syscall, shell diagnostic, live packet I/O, hardware,
SSH, public ABI, or broader network direction.

This task does not implement runtime behavior. It does not bridge smoltcp into
Talos socket syscalls, add /bin/sockdiag TCP diagnostics, retain a smoke
transcript, implement live driver adapters, perform live packet I/O, run Pi 5
hardware, acquire hardwareTestLock, mutate the lab, publish a boot image,
accept hardware reachability, accept SSH, accept a public stable socket ABI,
broaden socket behavior, or transition phase.

## Findings And Dispositions

- fixed: Reconciled the accepted smoltcp sequence from contract-only adoption,
  minimal no_std dependency core, host-only packet-device adapter core, and
  host-only deterministic TCP handshake source/unit evidence.
- fixed: The accepted frontier is only source/unit plus host/QEMU-substitute
  smoltcp TCP behavior over fixed Talos packet queues and
  SmoltcpPacketDeviceAdapter.
- fixed: The accepted handshake owns MAC/IP configuration, smoltcp Interface
  instances, SocketSet storage, TCP buffers, fixed packet queues, and
  one-millisecond time progression explicitly in the test harness.
- fixed: The success path reaches Established on both smoltcp TCP sockets after
  two poll steps, three client-to-server frames, and two server-to-client
  frames.
- fixed: The backpressure/control path keeps the client in SynSent and the
  server in Listen when the client transmit queue has zero capacity, records
  TransmitQueueFull, and moves zero frames.
- fixed: Roadmap and Phase 12 networking docs now state that this closeout
  freezes the initial smoltcp/TCP host-only frontier and requires supervisor
  planning before any bridge task or broader networking direction.
- not-an-issue: The accepted evidence is host/QEMU-substitute, not Pi 5
  hardware evidence. That is sufficient because this closeout accepts no live
  packet I/O, hardware reachability, SSH, or boot/lab claim.
- not-an-issue: Existing runtime ping, /bin/pingdiag, /bin/sockdiag, and
  private local socket surfaces remain separate from smoltcp; they are
  regression/control surfaces for later bridge work, not accepted smoltcp TCP
  syscall evidence.
- deferred: Talos socket syscall bridging, descriptor-backed smoltcp socket
  ownership, /bin/sockdiag TCP diagnostics, retained smoke evidence, live
  driver adapters, live packet I/O, hardware reachability, SSH, public stable
  socket ABI acceptance, broad socket expansion, and phase transition remain
  future supervisor-planned work.
- removed: No fake TCP shell behavior, public socket ABI claim, hardware
  action, lab mutation, boot publication, live packet I/O, SSH claim, broad
  expansion, or phase-transition claim was added.

## Evidence

- Adoption contract:
  tasks/2026-06-21-phase12-network-smoltcp-adoption-contract.md.
- No_std dependency core:
  tasks/2026-06-21-phase12-network-smoltcp-no-std-dependency-core.md.
- Packet-device adapter core:
  tasks/2026-06-21-phase12-network-smoltcp-packet-device-adapter-core.md.
- Host-only TCP handshake core:
  tasks/2026-06-21-phase12-network-smoltcp-loopback-tcp-handshake-core.md.
- Source anchors:
  - src/network.rs SmoltcpDependencyCore.
  - src/network.rs SmoltcpPacketDeviceAdapter and smoltcp::phy::Device
    implementation.
  - src/network.rs smoltcp_loopback_tcp_handshake_establishes_over_packet_device_adapters.
  - src/network.rs smoltcp_loopback_tcp_handshake_reports_client_transmit_backpressure.

## Validation

- static source/task/evidence review: passed. Reviewed src/network.rs smoltcp
  dependency, packet-device adapter, and handshake tests; the adoption,
  dependency-core, adapter-core, and handshake-core task records; roadmap
  frontier; and Phase 12 networking doc.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Talos socket syscall bridge, /bin/sockdiag TCP diagnostic, retained smoke
transcript, live driver adapter, live packet I/O, Pi 5 hardware run,
hardwareTestLock acquisition, boot archive publication, lab mutation, power
cycle, hardware reachability, SSH, broad socket expansion, public stable
socket ABI acceptance, or phase transition was performed.

## Acceptance

Accepted.

The accepted evidence level is source/unit plus host/QEMU-substitute only. The
accepted frontier includes smoltcp 0.13.1 with no default features, Talos-owned
fixed packet queues, SmoltcpPacketDeviceAdapter, explicit MAC/IP/time/socket
storage ownership, deterministic TCP Established handshake evidence, and a
deterministic transmit-backpressure control.

Talos socket syscall bridging, /bin/sockdiag TCP diagnostics, retained smoke
evidence, live driver adapters, live packet I/O, hardware reachability, SSH,
broad socket expansion, public stable socket ABI acceptance, and phase
transition remain rejected.

selected_next_task=null.

planningNeeded=true.

Commit: recorded in durable supervisor state after commit creation.
