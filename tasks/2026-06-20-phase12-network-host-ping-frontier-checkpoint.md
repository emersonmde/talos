# Phase 12.3 Host Ping Frontier Checkpoint

Task: phase12-network-host-ping-frontier-checkpoint-20260620

Status: accepted

Classification: phase12-network-host-ping-frontier-checkpoint-accepted

## Scope

Checkpoint the accepted Phase 12.3 host-only ping frontier after the retained
runtime-pump-backed ping syscall substitute smoke closeout. This task is limited
to static/source/task/evidence review, task recording, docs, and durable state.
It does not add runtime behavior.

## Findings And Dispositions

- fixed: Reconciled the accepted host-only ping stack through
  RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, local
  ARP/ICMP responder behavior, active ping descriptor dispatch,
  UserspacePingOperation, SinglePingPacketService, fake/trait-level
  NetworkDevice behavior, caller-owned buffers, and fixed-capacity state.
- fixed: Confirmed the retained smoke evidence remains host/QEMU-substitute
  only and covers open/start/status, unresolved ARP, runtime-pump ARP
  advancement to ICMP transmit, inflight status, runtime-pump echo-reply
  completion, terminal completed status, close, local ARP and ICMP reply
  dispatch while a descriptor is open, retry exhaustion, explicit timeout,
  invalid descriptor, closed descriptor, zero-capacity, busy-open, receive IO
  error, local transmit IO error, and active-ping transmit IO error.
- fixed: Updated the Phase 12 docs and roadmap to record this checkpoint and
  select phase12-network-descriptor-shaped-ping-control-contract-20260620 as
  the next bounded task.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI acceptance,
  UDP/TCP, smoltcp, live driver adapter, live packet I/O, hardware
  reachability, SSH, lab mutation, boot publication, Phase 12.1 link-hardware
  retry, Phase 12.4 socket expansion, or phase transition was accepted.
- deferred: Descriptor-shaped ping control work is deferred to the selected
  follow-up task; this checkpoint does not implement or accept that boundary.
- not-an-issue: The checkpoint found no source inconsistency in the retained
  RuntimePingOperationSyscallSubstitute or NetworkRuntimeDevicePump evidence.

## Accepted Evidence

The accepted host-only frontier is grounded in:

- implementation and smoke commit:
  425f9abd29acfa540345ed3ab87bd1a75273aaf9.
- smoke closeout commit:
  500f7543b4e95c3c6e2f75cda39625f93e5d940a.
- retained smoke transcript:
  tasks/evidence/2026-06-20-runtime-ping-syscall-substitute-smoke/qemu-runtime-ping-syscall-substitute-smoke.log.
- smoke command:
  scripts/qemu-runtime-ping-syscall-substitute-smoke.sh.
- task records:
  tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-core.md,
  tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-closeout.md,
  tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-smoke-core.md,
  and
  tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-smoke-closeout.md.
- source boundary:
  src/syscall.rs RuntimePingOperationSyscallSubstitute and src/network.rs
  NetworkRuntimeDevicePump.

## Checkpoint Validation

- static/source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

No hardware lock, Pi 5 boot, lab mutation, boot publication, live packet I/O,
shell ping, public socket API, stable syscall ABI acceptance, SSH, smoltcp,
UDP/TCP, Phase 12.1 retry, Phase 12.4 socket expansion, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-descriptor-shaped-ping-control-contract-20260620.

The accepted evidence level remains host/QEMU-substitute only over
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, local ARP/ICMP
responder behavior, active ping descriptor dispatch, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior,
caller-owned receive/transmit/status buffers, and fixed-capacity state.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, Phase 12.4 socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
