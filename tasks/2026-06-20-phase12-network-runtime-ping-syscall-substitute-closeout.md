# Phase 12.3 Runtime Ping Syscall Substitute Closeout

Task: phase12-network-runtime-ping-syscall-substitute-closeout-20260620

Status: accepted

Classification: phase12-network-runtime-ping-syscall-substitute-closeout-accepted

## Scope

Close out the accepted runtime-pump-backed ping syscall substitute/control
adapter and decide whether the retained substitute smoke task is objectively
unblocked.

## Findings And Dispositions

- fixed: Reconciled the accepted RuntimePingOperationSyscallSubstitute
  boundary with the committed core task record, source/tests, project docs,
  roadmap status, and durable state.
- fixed: Preserved the evidence level as host-only source/unit/QEMU-substitute
  over NetworkRuntimeDevicePump, local ARP/ICMP responder behavior, active ping
  descriptor dispatch, UserspacePingOperation, SinglePingPacketService,
  fake/trait-level NetworkDevice behavior, caller-owned buffers, and
  fixed-capacity state.
- fixed: Selected
  phase12-network-runtime-ping-syscall-substitute-smoke-core-20260620 as the
  next mechanically unblocked task because the core was accepted and committed
  at f99a0039dca5f22ff778d1478eee91fc04f67244 with complete host-only
  validation evidence.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI, live driver
  adapter, live packet I/O, hardware path, lab mutation, boot publication, SSH,
  smoltcp, UDP/TCP, Phase 12.1 link-hardware retry, Phase 12.4 socket
  expansion, or phase transition was accepted.
- deferred: Retained substitute smoke execution remains a separate bounded task
  so this closeout does not broaden from reconciliation into new smoke
  evidence.
- not-an-issue: The core adapter's host-only control boundary does not change
  the stable syscall dispatcher or accept a userspace-visible syscall ABI.

## Accepted Boundary

The accepted boundary is RuntimePingOperationSyscallSubstitute over
NetworkRuntimeDevicePump. The adapter borrows caller-owned receive and transmit
buffers, uses a fake/trait-level NetworkDevice supplied by the caller, routes
open/start/status/retry_arp/timeout/close through the runtime pump, and exposes
one pump step that keeps local ARP/ICMP responder priority ahead of active ping
descriptor dispatch.

The closeout accepts the core validation evidence recorded in
tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-core.md and
commit f99a0039dca5f22ff778d1478eee91fc04f67244:

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet runtime_ping_syscall_substitute:
  passed, 647 no_std tests.
- cargo -Zjson-target-spec test --quiet: passed, 647 no_std tests.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.
- post-commit git status --short: clean.

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

Accepted. selected_next_task=phase12-network-runtime-ping-syscall-substitute-smoke-core-20260620.

planningNeeded=false.

Commit: recorded in durable supervisor state after commit creation.
