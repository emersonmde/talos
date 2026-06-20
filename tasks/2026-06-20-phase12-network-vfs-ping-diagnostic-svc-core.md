# Phase 12.4 VFS Ping Diagnostic SVC Core

Task: phase12-network-vfs-ping-diagnostic-svc-core-20260620

Status: accepted

Classification: phase12-network-vfs-ping-diagnostic-svc-core-accepted

## Scope

Implement the thinnest host-only VFS-backed userspace ping diagnostic fixture
boundary selected by the accepted contract. The core connects a task-owned
ReadOnlyInitramfs executable-shaped fixture identity to the accepted
dispatch_process_local_ping_descriptor_user_arguments bridge without adding a
stable syscall number, shell ping command, public socket API, live driver
adapter, hardware reachability claim, SSH, or phase transition.

## Findings And Dispositions

- fixed: Added VfsPingDiagnosticSvcFixture in src/syscall.rs. The fixture first
  resolves a task-selected executable path through ReadOnlyInitramfs
  regular_file_bytes, records that VFS-backed executable identity, and then
  drives the accepted experimental process-local ping user-argument dispatcher.
- fixed: Preserved the accepted operation sequence. Source/unit evidence covers
  open, idle status copy-out, start from diagnostic-owned user payload memory,
  pump_or_read_result through ARP-to-ICMP progression, completed status
  copy-out, and close.
- fixed: Kept user-memory ownership explicit. Diagnostic payload, pump/result,
  and status slots live in caller-owned user memory described by UserMapping;
  payload copy-in and result/status copy-out still flow through the accepted
  copy_from_user/copy_to_user bridge and bounded kernel scratch.
- fixed: Added source/unit controls for malformed selector and malformed
  payload, missing owner, invalid and closed descriptors, process descriptor
  capacity, VFS executable lookup failure, output-buffer pressure, invalid user
  memory, scratch pressure, retry exhaustion, explicit timeout, caller receive
  buffer pressure, and device receive IO error.
- fixed: Verified SyscallNumber, STABLE_SVC_IMMEDIATE, and public TALOS_*
  constants remain unchanged. The fixture is crate-internal and host-only.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI acceptance, live
  driver adapter, live packet I/O, hardware reachability, SSH, smoltcp,
  UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry,
  broad socket expansion, or phase transition was added or accepted.
- deferred: Retained host/QEMU-substitute smoke evidence for this diagnostic
  bridge remains deferred to phase12-network-vfs-ping-diagnostic-svc-smoke-20260620
  after the closeout task selects it.
- not-an-issue: The executable-shaped fixture bytes are used only as a
  diagnostic VFS identity in this core. They do not claim a public userspace
  ABI, libc surface, executable compatibility contract, shell command, socket
  behavior, live network behavior, or hardware reachability.

## Implementation

src/syscall.rs now provides VfsPingDiagnosticSvcFixture. The fixture accepts a
ReadOnlyInitramfs, executable path, UserMapping slice, user memory, bounded
kernel scratch, and task-owned user offsets for payload, pump/result, and
status records. Construction fails unless the executable path resolves to a
regular file in the supplied VFS. Dispatch helpers produce only the accepted
experimental process-local ping selectors and delegate to
dispatch_process_local_ping_descriptor_user_arguments.

The accepted lifecycle remains host-only:

- open returns the process-local descriptor through the accepted scalar outcome;
- status copies the accepted status record to the diagnostic status slot;
- start copies diagnostic-owned payload bytes through user memory into bounded
  kernel scratch before delegating to ProcessLocalPingDescriptorControl;
- pump_or_read_result copies the accepted pump/result record to diagnostic
  result memory;
- timeout, retry_arp, and close remain explicit caller-driven controls.

## Evidence

- Source/unit validation transcript:
  tasks/evidence/2026-06-20-vfs-ping-diagnostic-svc-core/cargo-test.log.
- Format validation transcript:
  tasks/evidence/2026-06-20-vfs-ping-diagnostic-svc-core/cargo-fmt-check.log.
- Source: src/syscall.rs VfsPingDiagnosticSvcFixture,
  dispatch_process_local_ping_descriptor_user_arguments, experimental
  PROCESS_LOCAL_PING_USER_SELECTOR_* selectors, UserMapping copy-in/copy-out
  path, and new fixture tests.
- Source: src/initramfs.rs ReadOnlyInitramfs regular-file identity model.
- Predecessor contract:
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-svc-contract.md.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed with QEMU on PATH; 658 no_std
  tests passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, socket syscall ABI
acceptance, live packet I/O, SSH, smoltcp, UDP/TCP, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-vfs-ping-diagnostic-svc-closeout-20260620.

The accepted evidence level is source/unit host/QEMU-substitute evidence over a
task-owned VFS/initramfs executable-shaped diagnostic identity, experimental
user-argument decoding, UserMapping copy-in/copy-out, process-local descriptor
ownership, internal dispatch-shaped control, fake/trait-level NetworkDevice
behavior, caller-owned buffers, task-owned result/status slots, and
fixed-capacity state. Shell ping, kernel-backed fake command expansion, public
sockets, stable syscall ABI acceptance, socket syscall ABI acceptance, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry, broad
socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
