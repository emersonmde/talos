# Phase 12.4 Driver Packet Pump Closeout

Task: phase12-network-driver-packet-pump-closeout-20260620

Status: accepted

Classification: phase12-network-driver-packet-pump-closeout-accepted

## Scope

Close out the accepted host-only packet pump core before retaining broader smoke
evidence or planning any live adapter step. This reconciles source, tests, task
evidence, docs, and accepted/rejected claims from
phase12-network-driver-packet-pump-core-20260620.

This closeout does not add new runtime behavior. It does not accept shell ping,
public sockets, stable/socket ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 retry, broad socket expansion, or a phase transition.

## Findings And Dispositions

- not-an-issue: The packet pump core is accepted at host/QEMU-substitute
  source/unit evidence level. It remains crate-internal and host-only, with the
  external side limited to the NetworkDevice trait.
- not-an-issue: PacketQueueNetworkDevice::pump_driver drains outbound queue
  records before polling receive. This preserves deterministic FIFO transmit
  ordering and avoids consuming received frames while outbound work is pending.
- not-an-issue: Receive queue backpressure is checked before calling the
  trait-level driver, so a full diagnostic receive queue does not consume a
  driver-owned frame.
- not-an-issue: Caller receive-buffer pressure is reported as
  ReceiveBufferTooSmall and leaves the trait-level driver's queued frame
  available for retry.
- not-an-issue: Oversized frames delivered by a trait-level driver are reported
  as ReceiveFrameTooLarge at the diagnostic queue boundary. They are not accepted
  as inbound diagnostic records.
- not-an-issue: The VFS-backed ping diagnostic lifecycle routes outbound ARP
  and ICMP request records through pump_driver and receives injected ARP and
  ICMP replies back through the same pump boundary before descriptor progress.
- not-an-issue: Existing accepted negative/control coverage still covers
  malformed received frames, missing VFS identity, malformed user arguments,
  owner and descriptor failures, timeout/retry, close/drop behavior, user-memory
  faults, process descriptor capacity, scratch pressure, device errors, and
  unchanged SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
- deferred: Retained transcript capture remains the next explicit queued task:
  phase12-network-driver-packet-pump-smoke-20260620.
- removed: No closeout-only source cleanup was justified; no runtime behavior
  was changed in this closeout.

## Evidence

- Accepted predecessor:
  - phase12-network-driver-packet-pump-core-20260620 accepted and committed at
    49528a0e92f0588e71fbdd9e6b667d04986fcc60.
- Source/task review:
  - src/network.rs PacketQueueDriverPumpStep,
    PacketQueueNetworkDevice::pump_driver, FixedPacketQueue backpressure, and
    NetworkDevice trait behavior.
  - src/syscall.rs
    vfs_ping_diagnostic_svc_fixture_records_packet_queue_lifecycle and stable
    syscall vocabulary checks.
  - tasks/2026-06-20-phase12-network-driver-packet-pump-core.md accepted
    source/unit validation and rejected-claim boundary.

## Validation

- static source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, lab mutation, boot
publication, live packet I/O, shell ping, public socket API, stable/socket ABI
acceptance, SSH, smoltcp, UDP/TCP, broad socket expansion, or phase transition
was performed.

## Acceptance

Accepted.

The accepted evidence level remains host/QEMU-substitute source/unit evidence
over crate-internal packet queue records, trait-level NetworkDevice pump
behavior, the VFS/userspace diagnostic SVC bridge, UserMapping copy-in/copy-out,
process-local descriptor ownership, caller-owned buffers, and task-owned state.

selected_next_task=phase12-network-driver-packet-pump-smoke-20260620.

The retained smoke task is objective and bounded: it keeps evidence host-only
and crate-internal, retains transcript/output under task-owned evidence paths,
and explicitly rejects live driver adapters, live packet I/O, shell ping,
public sockets, stable/socket ABI acceptance, hardware reachability, SSH, lab
mutation, boot publication, broad socket expansion, and phase transition.

Commit: recorded in durable supervisor state after commit creation.
