# Phase 12.4 Shell Pingdiag Contract

Task: phase12-network-shell-pingdiag-contract-20260620

Status: accepted

Classification: phase12-network-shell-pingdiag-contract-accepted

## Scope

Define the smallest shell-visible Phase 12.4 feature step after the accepted
VFS diagnostic SVC, packet queue, and driver-facing packet pump evidence. The
next implementation must expose `/bin/pingdiag` through VFS/userspace-backed
execution or command-loop transcript plumbing and must prove that the visible
command path opens, starts, pumps, reports, and closes through the accepted
descriptor and packet-pump layers.

This contract does not add runtime behavior. It does not accept kernel-backed
fake shell commands, public sockets, stable syscall ABI acceptance, socket ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, Pi 5
hardware work, lab mutation, boot publication, SSH, smoltcp, UDP/TCP, broad
socket expansion, Phase 12.1 hardware retry, or a phase transition.

## Findings And Dispositions

- fixed: The driver packet pump smoke closeout left supervisor planning as the
  required next step. The bounded next step is now a shell-visible diagnostic
  contract, not live driver I/O or socket expansion.
- fixed: The future user-visible feature must be backed by the accepted VFS
  `/bin/pingdiag` identity, diagnostic SVC user-argument bridge,
  process-local descriptor ownership, UserMapping copy-in/copy-out, packet
  queues, and PacketQueueNetworkDevice::pump_driver.
- fixed: The future core evidence must include a command-loop or
  VFS/userspace execution transcript that opens, starts, pumps, observes
  status/result, and closes `/bin/pingdiag` through those layers.
- fixed: Future deterministic coverage is required for successful ARP/ICMP
  diagnostic progression plus malformed arguments, missing VFS executable
  identity, owner/descriptor failures, invalid and closed descriptors, queue
  capacity/backpressure, timeout/retry, transmit and receive device errors,
  close/drop behavior, and unchanged
  SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
- not-an-issue: The accepted packet pump smoke remains host/QEMU-substitute
  evidence only. It proves trait-level packet pump behavior but not a live
  adapter, public socket, real network reachability, or SSH.
- not-an-issue: Existing shell execution and lifecycle code already provides
  shell-visible VFS-backed command surfaces; the future core may connect to
  that surface or add a narrowly bounded transcript hook if it remains backed
  by VFS/userspace diagnostic plumbing.
- deferred: Public sockets, stable/socket ABI acceptance, live driver adapters,
  live packet I/O, hardware reachability, smoltcp, UDP/TCP, SSH, Phase 12.1
  hardware retry, broad socket expansion, and phase transition remain deferred.
- removed: No source cleanup or runtime source edit was justified for this
  contract-only task.

## Accepted Contract

`/bin/pingdiag` is the next smallest useful user-visible Phase 12.4 step
because Talos already has accepted host/QEMU-substitute evidence for:

- VFS `/bin/pingdiag` executable identity and diagnostic SVC dispatch;
- experimental user-argument decoding and UserMapping copy-in/copy-out;
- process-local descriptor ownership and close/drop behavior;
- fixed-capacity packet queues recording outbound ARP and IPv4/ICMP echo
  request frames;
- injected ARP and ICMP reply progression through trait-level NetworkDevice
  behavior; and
- PacketQueueNetworkDevice::pump_driver moving records between the diagnostic
  queue and trait-level driver behavior.

The future core task must turn that accepted internal lifecycle into a
shell-visible transcript. The transcript may be implemented through the
command loop or through VFS/userspace execution plumbing, but it must show a
visible `/bin/pingdiag` request opening the VFS-backed diagnostic identity,
starting the descriptor-owned operation, pumping ARP and ICMP packets through
PacketQueueNetworkDevice::pump_driver, observing status/result copy-out, and
closing or dropping the descriptor.

The feature test must remain host/QEMU-substitute and deterministic. It must
not claim real ICMP reachability, live packet I/O, a public socket API, a
stable syscall ABI, SSH readiness, boot publication, or hardware acceptance.

## Evidence

- static source/task/doc review:
  - `src/syscall.rs` owns
    `dispatch_process_local_ping_descriptor_user_arguments` and
    `VfsPingDiagnosticSvcFixture`, including `/bin/pingdiag` VFS lookup and
    UserMapping-backed argument/result/status handling.
  - `src/network.rs` owns NetworkDevice, PacketQueueNetworkDevice,
    PacketQueueDriverPumpStep, and PacketQueueNetworkDevice::pump_driver.
  - `src/local_command_loop.rs` owns the existing shell-visible VFS exec,
    lifecycle, waitpid, and laststatus transcript surfaces that can constrain
    a future `/bin/pingdiag` feature test.
  - `tasks/2026-06-20-phase12-network-driver-packet-pump-smoke-closeout.md`
    records the accepted retained host/QEMU-substitute packet pump smoke
    frontier.

## Validation

- static source/task/doc review: passed
- git diff --check: passed
- /home/node/.cargo/bin/mdbook build: passed
- git diff --cached --check: passed

## Next Action

Selected next task:
phase12-network-shell-pingdiag-core-20260620.

The next task may implement only the bounded shell-visible `/bin/pingdiag`
core described here. It must preserve the rejected-claim boundaries above and
must not start public sockets, live driver adapters, hardware reachability,
SSH, or phase-transition work.
