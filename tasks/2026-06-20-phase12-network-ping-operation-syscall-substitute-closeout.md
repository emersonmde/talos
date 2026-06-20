# Phase 12.3 Ping Operation Syscall Substitute Closeout

Task: phase12-network-ping-operation-syscall-substitute-closeout-20260620

Status: accepted

Classification: phase12-network-ping-operation-syscall-substitute-closeout-accepted

## Scope

Reconcile the accepted host-only ping operation syscall-substitute adapter,
tests, task record, docs, and rejected claims before any retained smoke
evidence task starts.

## Findings And Dispositions

- fixed: Accepted the phase12-network-ping-operation-syscall-substitute-core
  implementation boundary as source/unit/QEMU-substitute evidence for a
  proof-only adapter in src/syscall.rs.
- fixed: Confirmed the accepted boundary remains
  NetworkPingOperationDescriptorTable, UserspacePingOperation,
  SinglePingPacketService, fake/trait-level NetworkDevice behavior,
  caller-owned receive/transmit/status buffers, and the proof-only
  PingOperationSyscallSubstitute adapter.
- fixed: Confirmed validation evidence covers unresolved ARP through echo-reply
  completion, terminal completed status, invalid and closed descriptors,
  zero-capacity open, duplicate active operation, retry exhaustion, explicit
  timeout with terminal timed-out status, receive/transmit IO errors, and
  pump-time transmit IO error through the adapter.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable POSIX socket ABI, UDP/TCP, smoltcp adoption, live driver
  adapter, live packet I/O, hardware reachability, SSH, lab mutation, boot
  publication, Phase 12.1 link-hardware retry, or phase transition is accepted
  by this closeout.
- deferred: Retained syscall-substitute smoke evidence remains the selected
  follow-up task,
  phase12-network-ping-operation-syscall-substitute-smoke-core-20260620.
- not-an-issue: The adapter's placement in src/syscall.rs does not accept a
  stable SVC syscall ABI because it is called explicitly by host/unit smoke
  code and leaves the stable dispatcher unchanged.

## Evidence Reviewed

- Task: tasks/2026-06-20-phase12-network-ping-operation-syscall-substitute-contract.md.
- Task: tasks/2026-06-20-phase12-network-ping-operation-syscall-substitute-core.md.
- Source: src/syscall.rs PingOperationSyscallSubstitute,
  PingOperationSyscallSubstituteStatus, PingOperationSyscallSubstituteStep,
  and ping_operation_syscall_substitute_* unit tests.
- Source: src/network.rs NetworkPingOperationDescriptorTable and accepted
  UserspacePingOperation boundaries referenced by the adapter.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- static/source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

No Rust source was changed by this closeout, so cargo fmt and cargo test were
not required by this task's gates. No hardware lock, lab mutation, boot
publication, live packet I/O, shell ping, socket API, SSH, or phase transition
was performed.

## Acceptance

Accepted. selected_next_task=phase12-network-ping-operation-syscall-substitute-smoke-core-20260620.

Commit: recorded in durable supervisor state after commit creation.

The accepted evidence level is host-only static/source/task closeout over the
source/unit/QEMU-substitute adapter evidence from
phase12-network-ping-operation-syscall-substitute-core-20260620. It does not
accept shell ping, public sockets, stable syscall ABI, socket syscall ABI, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP,
lab mutation, boot publication, Phase 12.1 link-hardware retry, Phase 12.4
socket expansion, or phase transition.
