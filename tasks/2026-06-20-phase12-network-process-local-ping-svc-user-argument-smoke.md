# Phase 12.4 Process-Local Ping SVC User-Argument Smoke

Task: phase12-network-process-local-ping-svc-user-argument-smoke-20260620

Status: accepted

Classification: phase12-network-process-local-ping-svc-user-argument-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the accepted experimental
process-local ping SVC user-argument decoder. This task is limited to a
task-owned smoke script, retained transcript evidence, task/docs updates, and
validation gates. It does not add runtime behavior.

## Findings And Dispositions

- fixed: Added scripts/qemu-process-local-ping-svc-user-argument-smoke.sh as
  the task-owned QEMU/substitute smoke command for the crate-internal
  user-argument decoder path.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-20-process-local-ping-svc-user-argument-smoke/. The
  transcript labels the accepted host-only boundary and runs the
  process_local_ping_user_arguments source/unit evidence through the documented
  QEMU runner path.
- fixed: The retained transcript demonstrates open, idle status copy-out,
  start from copied user payload, ARP-to-ICMP pump result copy-out,
  echo-reply completion, completed status copy-out, and close through
  dispatch_process_local_ping_descriptor_user_arguments.
- fixed: The retained transcript covers experimental selectors for open,
  start, pump_or_read_result, status, retry_arp, timeout, and close; payload
  copy-in; result/status copy-out; bounded kernel scratch; caller-owned
  buffers; malformed selector and reserved fields; missing owner; process
  descriptor capacity; invalid descriptors; output-buffer pressure; invalid
  user memory; scratch pressure; zero TTL; invalid route prefix; and unchanged
  stable SyscallNumber/TALOS_* vocabulary.
- removed: No shell ping command, public socket API, stable syscall ABI, socket
  syscall ABI, live driver adapter, live packet I/O, hardware reachability,
  SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
  link-hardware retry, broad socket expansion, or phase transition was added or
  accepted.
- deferred: Closeout of the retained user-argument decoder smoke evidence
  remains the dependency-gated follow-up task.
- not-an-issue: The smoke remains host/QEMU-substitute evidence rather than a
  Pi 5 hardware run. This task explicitly requires fake/trait-level evidence
  and rejects live packet I/O and hardware reachability claims.

## Evidence

- Smoke script:
  scripts/qemu-process-local-ping-svc-user-argument-smoke.sh.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-06-20-process-local-ping-svc-user-argument-smoke/qemu-process-local-ping-svc-user-argument-smoke.log.
- Source boundary under evidence:
  src/syscall.rs dispatch_process_local_ping_descriptor_user_arguments,
  PROCESS_LOCAL_PING_USER_SELECTOR_* experimental selectors,
  UserMapping copy-in/copy-out, process_local_ping_user_* scalar decoding, and
  fixed result/status encoders.
- Source boundary under evidence:
  src/syscall.rs ProcessLocalPingDispatchOperation,
  ProcessLocalPingDispatchOutputs,
  dispatch_process_local_ping_descriptor_operation,
  ProcessLocalPingDescriptorControl, DescriptorShapedPingControl,
  RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, and
  ProcessDescriptorStore.
- Prior accepted contract:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-contract.md.
- Prior accepted core:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-core.md.
- Prior accepted closeout:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-closeout.md.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed, 656 no_std tests.
- QEMU/substitute smoke:
  scripts/qemu-process-local-ping-svc-user-argument-smoke.sh: passed.
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

selected_next_task=phase12-network-process-local-ping-svc-user-argument-smoke-closeout-20260620.

The accepted evidence level is host/QEMU-substitute smoke evidence over
dispatch_process_local_ping_descriptor_user_arguments, experimental
user-argument decoding, UserMapping copy-in/copy-out, process-local descriptor
ownership, ProcessLocalPingDispatchOperation,
ProcessLocalPingDescriptorControl, DescriptorShapedPingControl,
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump,
fake/trait-level NetworkDevice behavior, caller-owned buffers, task-owned
result/status slots, and fixed-capacity state.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad Phase 12.4 socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
