# Phase 12 Process-Local Ping Descriptor Closeout

Task: phase12-network-process-local-ping-descriptor-closeout-20260620
Status: accepted
Classification: phase12-network-process-local-ping-descriptor-closeout-accepted
Evidence level: host/QEMU-substitute source/unit evidence over fake/trait-level NetworkDevice behavior

## Scope

This closeout reconciles the accepted process-local ping descriptor contract,
core implementation, source/unit evidence, task records, docs, durable state,
and rejected claims. It does not add shell ping, public sockets, stable syscall
ABI acceptance, socket syscall ABI acceptance, live driver adapters, live
packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, or a phase
transition.

## Findings And Dispositions

- not-an-issue: The accepted core remains bounded to
  ProcessLocalPingDescriptorControl in src/syscall.rs. It maps one
  process-local DescriptorTable handle to one backing DescriptorShapedPingControl
  descriptor and leaves ARP, IPv4, ICMP, route policy, retry, timeout, local
  responder behavior, and fake/trait-level NetworkDevice I/O delegated to the
  existing runtime pump stack.
- not-an-issue: The process-local descriptor entry uses
  DescriptorObjectKind::OtherKernelObject rather than Socket. That preserves the
  host-only crate-internal boundary and does not accept a public socket API or a
  stable socket syscall ABI.
- not-an-issue: Source/unit evidence covers the core lifecycle through a
  process-local descriptor: open after inherited stdio, idle status, start to
  pending ARP, runtime-pump ARP advancement to inflight, echo-reply completion,
  terminal completed status, close, and closed-descriptor EBADF.
- not-an-issue: Source/unit evidence covers deterministic error behavior for
  missing current owner, process descriptor capacity pressure with backing
  descriptor unwind, duplicate active operation EBUSY, stdio/wrong-kind EBADF,
  retry exhaustion EAGAIN, explicit timeout, receive IO error, and local
  transmit IO error.
- deferred: Retained smoke transcript evidence for the process-local descriptor
  path is intentionally deferred to
  phase12-network-process-local-ping-descriptor-smoke-20260620. The accepted
  core evidence is source/unit host/QEMU-substitute evidence, not a retained
  smoke transcript.
- removed: No shell command, public socket API, stable syscall ABI, live driver
  adapter, packet queue, retry timer scheduler, UDP/TCP path, SSH path,
  hardware path, lab mutation, boot publication, or phase transition was added
  or accepted.

## Evidence Reviewed

- Contract task:
  tasks/2026-06-20-phase12-network-process-local-ping-descriptor-contract.md.
- Core task:
  tasks/2026-06-20-phase12-network-process-local-ping-descriptor-core.md.
- Source: src/syscall.rs ProcessLocalPingDescriptorControl,
  DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute, and
  source/unit tests
  process_local_ping_descriptor_control_completes_lifecycle_through_process_descriptor
  and
  process_local_ping_descriptor_control_maps_capacity_busy_closed_retry_timeout_and_io_errors.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Durable state: currentTask
  phase12-network-process-local-ping-descriptor-core-20260620 accepted with
  selected_next_task=phase12-network-process-local-ping-descriptor-closeout-20260620.

## Accepted Evidence Boundary

The accepted evidence level remains host/QEMU-substitute source/unit evidence
over fake/trait-level NetworkDevice behavior, process-local descriptor
ownership, DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned receive/transmit/status storage, and
fixed-capacity state.

The closeout accepts that a process-local descriptor handle can open, start,
pump/read-result, observe status, retry_arp, timeout, and close one
DescriptorShapedPingControl operation in source/unit tests. It also accepts that
descriptor lifecycle errors and capacity/busy cases are deterministic within
the crate-internal host-only boundary.

This closeout does not accept shell ping, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, or phase
transition.

## Validation

- static/source/task/evidence review: passed.
- diff validation: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed with the pre-existing
  large-search-index warning.
- staged diff validation: git diff --cached --check passed before commit.

No Rust source was touched by this closeout, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's gates.
The accepted source/unit evidence is inherited from the committed core task.
No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, live packet I/O, SSH,
or phase transition was performed.

## Result

Accepted. selected_next_task is
phase12-network-process-local-ping-descriptor-smoke-20260620.
