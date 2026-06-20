# Phase 12.4 Process-Local Ping SVC Dispatch Smoke

Task: phase12-network-process-local-ping-svc-dispatch-smoke-20260620

Status: accepted

Classification: phase12-network-process-local-ping-svc-dispatch-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the accepted internal
process-local ping dispatch facade. This task is limited to a task-owned smoke
script, retained transcript evidence, task/docs updates, and validation gates.
It does not add runtime behavior.

## Findings And Dispositions

- fixed: Added scripts/qemu-process-local-ping-svc-dispatch-smoke.sh as the
  task-owned QEMU/substitute smoke command for the crate-internal dispatch path.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-20-process-local-ping-svc-dispatch-smoke/. The
  transcript labels the accepted host-only boundary and runs the
  process_local_ping_dispatch source/unit evidence through the documented QEMU
  runner path.
- fixed: The retained transcript demonstrates open, start, pump_or_read_result,
  status, retry_arp, timeout, and close through
  dispatch_process_local_ping_descriptor_operation over
  ProcessLocalPingDescriptorControl.
- fixed: The retained transcript covers unresolved ARP, ARP-to-ICMP
  advancement, echo-reply completion, terminal status observation, invalid and
  closed descriptors, missing owner, process descriptor table capacity unwind,
  duplicate active operation, retry exhaustion, explicit timeout, receive-buffer
  pressure, receive IO error, local transmit IO error, and active transmit IO
  error controls.
- removed: No shell ping command, public socket API, stable syscall ABI, socket
  syscall ABI, live driver adapter, live packet I/O, hardware reachability,
  SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
  link-hardware retry, broad socket expansion, or phase transition was added or
  accepted.
- deferred: Closeout of the retained process-local ping SVC dispatch smoke
  evidence remains the dependency-gated follow-up task.
- not-an-issue: The smoke remains host/QEMU-substitute evidence rather than a
  Pi 5 hardware run. This task explicitly requires fake/trait-level evidence
  and rejects live packet I/O and hardware reachability claims.

## Evidence

- Smoke script:
  scripts/qemu-process-local-ping-svc-dispatch-smoke.sh.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-06-20-process-local-ping-svc-dispatch-smoke/qemu-process-local-ping-svc-dispatch-smoke.log.
- Source boundary under evidence:
  src/syscall.rs ProcessLocalPingDispatchOperation,
  ProcessLocalPingDispatchOutcome, ProcessLocalPingDispatchOutputs, and
  dispatch_process_local_ping_descriptor_operation.
- Source boundary under evidence:
  src/syscall.rs ProcessLocalPingDescriptorControl,
  DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute, and
  NetworkRuntimeDevicePump.
- Prior accepted contract:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-dispatch-contract.md.
- Prior accepted core:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-dispatch-core.md.
- Prior accepted closeout:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-dispatch-closeout.md.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed, 654 no_std tests.
- QEMU/substitute smoke:
  scripts/qemu-process-local-ping-svc-dispatch-smoke.sh: passed.
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

selected_next_task=phase12-network-process-local-ping-svc-dispatch-smoke-closeout-20260620.

The accepted evidence level is host/QEMU-substitute smoke evidence over
dispatch_process_local_ping_descriptor_operation, process-local descriptor
ownership, ProcessLocalPingDescriptorControl, DescriptorShapedPingControl,
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump,
fake/trait-level NetworkDevice behavior, caller-owned buffers, task-owned
result/status slots, and fixed-capacity state.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad Phase 12.4 socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
