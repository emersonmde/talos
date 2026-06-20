# Phase 12.3 Runtime Ping Syscall Substitute Smoke Closeout

Task: phase12-network-runtime-ping-syscall-substitute-smoke-closeout-20260620

Status: accepted

Classification: phase12-network-runtime-ping-syscall-substitute-smoke-closeout-accepted

## Scope

Close out the retained runtime-pump-backed ping syscall substitute smoke
evidence and reconcile it with the accepted RuntimePingOperationSyscallSubstitute
boundary. This task is limited to static/source/task/evidence review, task
recording, docs, and durable state. It does not add implementation behavior.

## Findings And Dispositions

- fixed: Reconciled the retained
  scripts/qemu-runtime-ping-syscall-substitute-smoke.sh transcript with the
  accepted RuntimePingOperationSyscallSubstitute implementation, smoke task
  record, project docs, roadmap status, and durable state.
- fixed: Preserved the evidence level as host/QEMU-substitute only over
  RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, local
  ARP/ICMP responder behavior, active ping descriptor dispatch,
  UserspacePingOperation, SinglePingPacketService, fake/trait-level
  NetworkDevice behavior, caller-owned buffers, and fixed-capacity state.
- fixed: Confirmed the retained transcript includes local responder
  preservation and active ping completion through NetworkRuntimeDevicePump:
  open/start/status, unresolved ARP, runtime-pump ARP advancement to ICMP
  transmit, inflight status, runtime-pump echo-reply completion, terminal
  completed status, close, local ARP and ICMP reply dispatch while a descriptor
  is open, retry exhaustion, explicit timeout, invalid descriptor, closed
  descriptor, zero-capacity, busy-open, receive IO error, local transmit IO
  error, and active-ping transmit IO error.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI, UDP/TCP,
  smoltcp, live driver adapter, live packet I/O, hardware reachability, SSH,
  lab mutation, boot publication, Phase 12.1 link-hardware retry, Phase 12.4
  socket expansion, or phase transition was accepted.
- deferred: No later Phase 12.3 or Phase 12.4 task is selected by this closeout.
  Supervisor planning is required before further ping, socket, live-driver,
  hardware, SSH, or phase-transition work.
- not-an-issue: The smoke evidence is reproducible by the task-owned script and
  retained under tasks/evidence; no source change was needed for closeout.

## Accepted Evidence

The accepted smoke evidence is the retained host/QEMU-substitute transcript:

- command: scripts/qemu-runtime-ping-syscall-substitute-smoke.sh
- transcript:
  tasks/evidence/2026-06-20-runtime-ping-syscall-substitute-smoke/qemu-runtime-ping-syscall-substitute-smoke.log
- terminal classification:
  host-substitute-runtime-ping-syscall-substitute-smoke-complete
- test result: 647 no_std tests passed

The accepted implementation and smoke records are:

- tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-core.md
- tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-closeout.md
- tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-smoke-core.md
- commit 425f9abd29acfa540345ed3ab87bd1a75273aaf9

## Closeout Validation

- static/source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

No hardware lock, Pi 5 boot, lab mutation, boot publication, live packet I/O,
shell ping, public socket API, stable syscall ABI acceptance, SSH, smoltcp,
UDP/TCP, Phase 12.1 retry, Phase 12.4 socket expansion, or phase transition was
performed.

## Acceptance

Accepted. selected_next_task=null.

planningNeeded=true.

Commit: recorded in durable supervisor state after commit creation.
