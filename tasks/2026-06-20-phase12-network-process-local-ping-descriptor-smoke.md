# Phase 12.4 Process-Local Ping Descriptor Smoke

Task: phase12-network-process-local-ping-descriptor-smoke-20260620

Status: accepted

Classification: phase12-network-process-local-ping-descriptor-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the accepted
ProcessLocalPingDescriptorControl path. This task is limited to a task-owned
smoke script, retained transcript evidence, task/docs updates, and validation
gates. It does not add runtime behavior.

## Findings And Dispositions

- fixed: Added scripts/qemu-process-local-ping-descriptor-smoke.sh as the
  task-owned QEMU/substitute smoke command for the process-local ping
  descriptor path.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-20-process-local-ping-descriptor-smoke/. The
  transcript labels the accepted host-only boundary and runs the
  process_local_ping_descriptor_control target test filter.
- fixed: The retained smoke transcript demonstrates the accepted process-local
  lifecycle over fake/trait-level NetworkDevice behavior: open process
  descriptor, idle status, start to unresolved-ARP pending, runtime-pump ARP
  advancement to inflight, runtime-pump echo-reply completion, terminal
  completed status, and close process descriptor.
- fixed: The retained smoke transcript covers deterministic descriptor
  lifecycle/error controls matching the accepted core risk profile: missing
  owner, full process descriptor table with backing-descriptor unwind,
  duplicate active open, wrong-kind stdio descriptor, closed descriptor, retry
  exhaustion, explicit timeout, receive IO error, and local transmit IO error.
- removed: No shell ping command, public socket API, stable syscall ABI,
  socket syscall ABI, live driver adapter, live packet I/O, hardware
  reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
  12.1 link-hardware retry, broad socket expansion, or phase transition was
  added or accepted.
- deferred: Closeout of the retained process-local ping descriptor smoke
  evidence remains the dependency-gated follow-up task.
- not-an-issue: The smoke remains host/QEMU-substitute evidence rather than a
  Pi 5 hardware run. This task explicitly requires fake/trait-level evidence
  and rejects live packet I/O and hardware reachability claims.

## Evidence

- Smoke script:
  scripts/qemu-process-local-ping-descriptor-smoke.sh.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-06-20-process-local-ping-descriptor-smoke/qemu-process-local-ping-descriptor-smoke.log.
- Source boundary under evidence:
  src/syscall.rs ProcessLocalPingDescriptorControl,
  DescriptorShapedPingControl, and RuntimePingOperationSyscallSubstitute.
- Prior accepted contract:
  tasks/2026-06-20-phase12-network-process-local-ping-descriptor-contract.md.
- Prior accepted core:
  tasks/2026-06-20-phase12-network-process-local-ping-descriptor-core.md.
- Prior accepted closeout:
  tasks/2026-06-20-phase12-network-process-local-ping-descriptor-closeout.md.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed, 652 no_std tests.
- QEMU/substitute smoke:
  scripts/qemu-process-local-ping-descriptor-smoke.sh: passed.
- diff validation: git diff --check: passed.
- docs build: /home/node/.cargo/bin/mdbook build: passed, existing large
  search-index warning only.
- staged diff validation: git diff --cached --check: passed.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, socket syscall ABI
acceptance, live packet I/O, SSH, smoltcp, UDP/TCP, Phase 12.1 link-hardware
retry, broad socket expansion, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-process-local-ping-descriptor-smoke-closeout-20260620.

The accepted evidence level is host/QEMU-substitute smoke evidence over
ProcessLocalPingDescriptorControl, ProcessDescriptorStore process-local
descriptor ownership, DescriptorShapedPingControl,
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump,
fake/trait-level NetworkDevice behavior, caller-owned buffers, and
fixed-capacity state.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad Phase 12.4 socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
