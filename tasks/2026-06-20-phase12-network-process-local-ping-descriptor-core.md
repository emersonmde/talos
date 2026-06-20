# Phase 12 Process-Local Ping Descriptor Core

Task: phase12-network-process-local-ping-descriptor-core-20260620
Status: accepted
Classification: phase12-network-process-local-ping-descriptor-core-accepted
Evidence level: host/QEMU-substitute unit/source evidence over fake/trait-level NetworkDevice behavior

## Scope

This task implements only the bounded process-local descriptor core accepted by
phase12-network-process-local-ping-descriptor-contract-20260620. It routes one
DescriptorShapedPingControl operation through the existing
ProcessDescriptorStore and per-ProcessOwnerId DescriptorTable model without
exposing public sockets, a stable syscall ABI, shell ping, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 retry, broad socket expansion, or a phase transition.

## Implementation

- fixed: Added ProcessLocalPingDescriptorControl in src/syscall.rs. The wrapper
  borrows a current process owner, ProcessDescriptorStore, NetworkRuntimeDevicePump,
  and caller-owned receive/transmit buffers, then delegates network behavior to
  DescriptorShapedPingControl.
- fixed: open validates the current process owner, opens one backing
  DescriptorShapedPingControl descriptor, allocates a process-local
  DescriptorTable entry with DescriptorObjectKind::OtherKernelObject, and unwinds
  the backing descriptor if the process table is full.
- fixed: start, pump_or_read_result, status, retry_arp, timeout, and close map
  the process-local descriptor back to the backing ping-control descriptor and
  preserve EBADF for invalid, closed, stdio/wrong-kind, or stale handles.
- fixed: close validates the backing ping-control descriptor before removing the
  process descriptor and then closes the backing control descriptor.
- not-an-issue: The implementation uses OtherKernelObject rather than Socket
  because this is not public socket API or socket syscall ABI acceptance.

## Evidence

- Source: src/syscall.rs ProcessLocalPingDescriptorControl.
- Unit/source evidence: process_local_ping_descriptor_control_completes_lifecycle_through_process_descriptor.
- Unit/source evidence: process_local_ping_descriptor_control_maps_capacity_busy_closed_retry_timeout_and_io_errors.

The source/unit evidence covers process-local open at descriptor 3 after inherited
stdio, idle status, start to pending ARP, runtime-pump ARP advancement to
inflight, echo-reply completion, terminal completed status, close, closed
descriptor EBADF, missing current owner EBADF, full process descriptor table
EMFILE with backing-descriptor unwind, duplicate active operation EBUSY,
wrong-kind stdio descriptor EBADF, retry exhaustion EAGAIN, explicit timeout,
receive IO error, and local transmit IO error.

## Validation

- host/QEMU-substitute unit tests: cargo -Zjson-target-spec test --quiet passed.
- formatting: cargo fmt --all applied; cargo fmt --all -- --check passed after formatting.
- diff validation: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed with the pre-existing large-search-index warning.
- staged diff validation: git diff --cached --check passed before commit.

## Rejected Claims

This task does not accept shell ping, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, broad socket expansion, or phase transition.

## Result

Accepted. selected_next_task is
phase12-network-process-local-ping-descriptor-closeout-20260620.
