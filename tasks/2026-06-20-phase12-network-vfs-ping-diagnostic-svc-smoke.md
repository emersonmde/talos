# Phase 12.4 VFS Ping Diagnostic SVC Smoke

Task: phase12-network-vfs-ping-diagnostic-svc-smoke-20260620

Status: accepted

Classification: phase12-network-vfs-ping-diagnostic-svc-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the accepted VFS-backed
userspace ping diagnostic SVC bridge. This task is limited to a task-owned
smoke script, retained transcript evidence, task/docs updates, and validation
gates. It does not add runtime behavior.

## Findings And Dispositions

- fixed: Added scripts/qemu-vfs-ping-diagnostic-svc-smoke.sh as the
  task-owned host/QEMU-substitute smoke command for the crate-internal
  VfsPingDiagnosticSvcFixture path.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-20-vfs-ping-diagnostic-svc-smoke/. The transcript
  labels the accepted host-only boundary and runs the
  vfs_ping_diagnostic_svc_fixture source/unit evidence through the documented
  QEMU runner path.
- fixed: The retained transcript demonstrates VFS executable lookup, open,
  idle status copy-out, start from copied diagnostic payload memory,
  pump_or_read_result through ARP-to-ICMP result copy-out, echo-reply
  completion, completed status copy-out, and close through the accepted
  diagnostic bridge.
- fixed: The retained transcript covers deterministic controls for missing
  executable identity, malformed selector and payload, missing owner, invalid
  and closed descriptors, process descriptor capacity, output-buffer pressure,
  invalid user memory, scratch pressure, caller receive-buffer pressure, retry
  exhaustion, explicit timeout, device receive IO error, and unchanged stable
  SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI acceptance,
  live driver adapter, live packet I/O, hardware reachability, SSH, smoltcp,
  UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry,
  broad socket expansion, or phase transition was added or accepted.
- deferred: Closeout of the retained VFS-backed userspace ping diagnostic SVC
  smoke evidence remains the dependency-gated follow-up task.
- not-an-issue: The smoke remains host/QEMU-substitute evidence rather than a
  Pi 5 hardware run. This task explicitly requires fake/trait-level
  NetworkDevice behavior and rejects live packet I/O and hardware reachability
  claims.

## Evidence

- Smoke script:
  scripts/qemu-vfs-ping-diagnostic-svc-smoke.sh.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-06-20-vfs-ping-diagnostic-svc-smoke/qemu-vfs-ping-diagnostic-svc-smoke.log.
- Source boundary under evidence:
  src/syscall.rs VfsPingDiagnosticSvcFixture,
  dispatch_process_local_ping_descriptor_user_arguments,
  PROCESS_LOCAL_PING_USER_SELECTOR_* experimental selectors, UserMapping
  copy-in/copy-out, process-local descriptor ownership, and fixed
  result/status encoders.
- Source boundary under evidence:
  src/initramfs.rs ReadOnlyInitramfs regular-file lookup and src/syscall.rs
  ProcessLocalPingDispatchOperation, ProcessLocalPingDescriptorControl,
  DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
  NetworkRuntimeDevicePump, and ProcessDescriptorStore.
- Prior accepted contract:
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-svc-contract.md.
- Prior accepted core:
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-svc-core.md.
- Prior accepted closeout:
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-svc-closeout.md.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- QEMU/substitute smoke:
  scripts/qemu-vfs-ping-diagnostic-svc-smoke.sh: passed.
- diff validation: git diff --check: passed.
- docs build: /home/node/.cargo/bin/mdbook build: passed.
- staged diff validation: git diff --cached --check: passed.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, socket syscall ABI
acceptance, live packet I/O, SSH, smoltcp, UDP/TCP, Phase 12.1 link-hardware
retry, broad socket expansion, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-vfs-ping-diagnostic-svc-smoke-closeout-20260620.

The accepted evidence level is host/QEMU-substitute smoke evidence over a
VFS/userspace diagnostic SVC bridge, VFS/initramfs executable identity,
experimental user-argument decoding, UserMapping copy-in/copy-out,
process-local descriptor ownership, internal dispatch-shaped control,
fake/trait-level NetworkDevice behavior, caller-owned buffers, task-owned
result/status slots, and fixed-capacity state.

Shell ping, kernel-backed fake command expansion, public sockets, stable
syscall ABI acceptance, socket syscall ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, Phase 12.1 link-hardware retry, broad Phase 12.4 socket
expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
