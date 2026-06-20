# Phase 12.4 Driver Packet Pump Smoke

Task: phase12-network-driver-packet-pump-smoke-20260620

Status: accepted

Classification: phase12-network-driver-packet-pump-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the VFS ping diagnostic packet
pump lifecycle over trait-level NetworkDevice behavior. This task records the
transcript and source anchors proving the accepted packet pump boundary moves
outbound ARP/ICMP request records to a trait-level driver queue and moves
injected ARP/ICMP replies back into the diagnostic receive queue before the
VFS-backed process-local descriptor observes progress.

This task does not add or accept Pi 5 hardware behavior, hardwareTestLock
acquisition, live driver adapters, live packet I/O, hardware reachability, lab
mutation, boot publication, shell ping, public sockets, stable/socket ABI
acceptance, SSH, smoltcp, UDP/TCP, broad socket expansion, or a phase
transition.

## Findings And Dispositions

- fixed: Added scripts/qemu-driver-packet-pump-smoke.sh as the task-owned
  host/QEMU-substitute smoke command for the packet pump boundary.
- fixed: Retained smoke output under
  tasks/evidence/2026-06-20-driver-packet-pump-smoke/ with command log, source
  anchors, transcript, classification, and evidence map.
- fixed: The retained positive path proves /bin/pingdiag VFS lookup,
  diagnostic SVC argument decoding, UserMapping copy-in/copy-out,
  process-local descriptor ownership, outbound ARP and IPv4/ICMP echo request
  transfer through PacketQueueNetworkDevice::pump_driver to trait-level
  NetworkDevice transmit behavior, injected ARP and ICMP echo reply transfer
  back through pump_driver, completed status/result copy-out, and close.
- fixed: Deterministic controls remain covered for missing VFS identity,
  malformed arguments, missing or wrong owner, invalid and closed descriptors,
  process descriptor capacity, queue capacity/backpressure, caller buffer
  pressure, malformed received frames, timeout/retry, transmit and receive
  device errors, close/drop behavior, and unchanged
  SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
- not-an-issue: The no_std QEMU runner executes the full target test binary for
  each filtered script invocation. The transcript records that behavior and
  labels the intended boundary checks; the evidence remains host/QEMU-substitute
  smoke and includes five passing 662-test invocations.
- removed: No runtime source behavior, public socket surface, stable ABI, shell
  command, live driver path, lab artifact, or hardware claim was added by this
  retained smoke task.
- deferred: Smoke closeout remains the next dependency-gated reconciliation
  task before supervisor planning decides any live driver adapter, live packet
  I/O, public socket, shell ping, SSH, or phase-transition direction.

## Evidence

- Smoke command:
  scripts/qemu-driver-packet-pump-smoke.sh.
- Retained transcript:
  tasks/evidence/2026-06-20-driver-packet-pump-smoke/smoke-transcript.md.
- Command transcript:
  tasks/evidence/2026-06-20-driver-packet-pump-smoke/qemu-driver-packet-pump-smoke.log.
- Source anchors:
  tasks/evidence/2026-06-20-driver-packet-pump-smoke/source-anchors.txt.
- Classification:
  tasks/evidence/2026-06-20-driver-packet-pump-smoke/classification.json.
- Evidence map:
  tasks/evidence/2026-06-20-driver-packet-pump-smoke/evidence-map.json.
- Accepted predecessor:
  phase12-network-driver-packet-pump-closeout-20260620 accepted and committed at
  bf6241384095986ad60fe0529e703ebd13170046.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed, 662 no_std tests.
- scripts/qemu-driver-packet-pump-smoke.sh: passed, five host/QEMU-substitute
  test invocations each reporting 662 no_std tests passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, RP1/GEM/MACB MMIO, DMA descriptor ownership,
interrupt completion, MDIO/PHY/GPIO32 action, live packet I/O, shell ping,
public socket API, stable/socket ABI acceptance, SSH, smoltcp, UDP/TCP, broad
socket expansion, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-driver-packet-pump-smoke-closeout-20260620.

The accepted evidence level is host/QEMU-substitute smoke evidence over
crate-internal packet queue records, trait-level NetworkDevice pump behavior,
the VFS/userspace diagnostic SVC bridge, experimental user-argument decoding,
UserMapping copy-in/copy-out, process-local descriptor ownership, caller-owned
buffers, task-owned state, fixed capacity, transmit FIFO ordering, explicit
receive polling, deterministic backpressure, device-error propagation,
timeout/retry controls, close/drop behavior, and unchanged
SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary. Shell ping, public
sockets, stable/socket ABI acceptance, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, broad socket expansion, and phase transition
remain rejected.

Commit: recorded in durable supervisor state after commit creation.
