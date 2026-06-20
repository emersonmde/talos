# Phase 12.3 Runtime Ping Syscall Substitute Smoke Core

Task: phase12-network-runtime-ping-syscall-substitute-smoke-core-20260620

Status: accepted

Classification: phase12-network-runtime-ping-syscall-substitute-smoke-core-accepted

## Scope

Retain durable host/QEMU-substitute smoke evidence for the accepted
RuntimePingOperationSyscallSubstitute boundary. This task is limited to a
task-owned smoke script, retained transcript evidence, task/docs evidence, and
validation gates. It does not add runtime behavior or expose a stable syscall
ABI.

## Findings And Dispositions

- fixed: Added scripts/qemu-runtime-ping-syscall-substitute-smoke.sh as the
  task-owned QEMU/substitute command for the runtime-pump-backed ping syscall
  substitute/control boundary.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-20-runtime-ping-syscall-substitute-smoke/. The
  transcript labels the accepted host-only boundary and runs the
  runtime_ping_syscall_substitute test filter.
- fixed: The retained transcript covers open/start/status, unresolved ARP,
  runtime-pump ARP advancement to ICMP transmit, inflight status, runtime-pump
  echo-reply completion, terminal completed status, close, local ARP and ICMP
  reply dispatch while a descriptor is open, retry exhaustion, explicit
  timeout, invalid descriptor, closed descriptor, zero-capacity, busy-open,
  receive IO error, local transmit IO error, and active-ping transmit IO
  error.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI, UDP/TCP,
  smoltcp, live driver adapter, live packet I/O, hardware reachability, SSH,
  lab mutation, boot publication, Phase 12.1 retry, Phase 12.4 expansion, or
  phase transition was added or accepted.
- deferred: Closeout of the retained runtime-pump-backed ping syscall
  substitute smoke evidence remains the dependency-gated follow-up task.
- not-an-issue: The smoke task did not require source changes in src/syscall.rs
  or src/network.rs. The accepted adapter implementation already contains the
  tested behavior.

## Evidence

- Smoke script:
  scripts/qemu-runtime-ping-syscall-substitute-smoke.sh.
- Retained smoke transcript:
  tasks/evidence/2026-06-20-runtime-ping-syscall-substitute-smoke/qemu-runtime-ping-syscall-substitute-smoke.log.
- Source boundary under evidence: RuntimePingOperationSyscallSubstitute,
  NetworkRuntimeDevicePump, local ARP/ICMP responder behavior, active ping
  descriptor dispatch, UserspacePingOperation, SinglePingPacketService,
  fake/trait-level NetworkDevice behavior, caller-owned buffers, and
  fixed-capacity state.
- Accepted adapter core:
  tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-core.md.
- Accepted adapter closeout:
  tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-closeout.md.

## Validation

- cargo fmt --all -- --check: passed.
- QEMU/substitute smoke:
  scripts/qemu-runtime-ping-syscall-substitute-smoke.sh: passed.
- diff validation: git diff --check: passed.
- docs build: /home/node/.cargo/bin/mdbook build: passed.
- staged diff validation: git diff --cached --check: passed.

No hardware lock, lab mutation, boot publication, live packet I/O, shell ping,
public socket API, stable syscall ABI acceptance, SSH, smoltcp, UDP/TCP, Phase
12.1 retry, Phase 12.4 expansion, or phase transition was performed.

## Acceptance

Accepted. selected_next_task=phase12-network-runtime-ping-syscall-substitute-smoke-closeout-20260620.

The accepted evidence level remains host/QEMU-substitute only over
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, local ARP/ICMP
responder behavior, active ping descriptor dispatch, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior,
caller-owned receive/transmit/status buffers, and fixed-capacity state.

Shell ping, kernel-backed fake command expansion, public sockets, stable
syscall ABI acceptance, socket syscall ABI, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, Phase 12.4 socket expansion, and
phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
