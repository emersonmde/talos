# Phase 12.3 Ping Operation Syscall Substitute Contract

Task: phase12-network-ping-operation-syscall-substitute-contract-20260620

Status: accepted

Classification: phase12-network-ping-operation-syscall-substitute-contract-accepted

## Scope

Select the smallest host-only syscall-substitute binding that can exercise the
accepted descriptor-shaped ping operation without accepting a stable syscall
ABI, shell command, public socket API, live driver adapter, live packet I/O,
hardware reachability, SSH, or Phase 12.4 socket expansion.

## Findings And Dispositions

- fixed: Selected a proof-only ping-operation syscall-substitute adapter in
  src/syscall.rs as the next binding. It should be explicitly separate from
  the stable SVC syscall dispatcher and should borrow the existing
  NetworkPingOperationDescriptorTable, fake/trait-level NetworkDevice, and
  caller-owned receive/transmit buffers from its caller.
- fixed: Kept descriptor identity and lifecycle owned by
  NetworkPingOperationDescriptorTable; the substitute adapter must not create
  a second descriptor table, socket table, packet queue, or autonomous timer
  layer.
- fixed: Selected operation vocabulary: open, start, pump, status, retry,
  timeout, and close. open allocates one ping-operation descriptor; start
  begins one route-aware ping-like operation; pump advances receive/transmit
  progress; status observes idle/pending/in-flight/completed/timed-out state;
  retry retransmits pending ARP when the operation is pending; timeout moves
  pending or in-flight work to terminal timed-out state; and close invalidates
  the descriptor.
- fixed: Selected error mapping remains the accepted POSIX vocabulary:
  invalid or closed descriptor is EBADF, zero descriptor capacity is EMFILE,
  duplicate active open is EBUSY, retry exhaustion and nonmatching frames are
  EAGAIN, explicit timeout reports a terminal timed-out status,
  receive/transmit device IO maps through the existing EIO/EAGAIN/ENOSPC
  paths, malformed or unsupported packet/frame inputs keep the existing
  EINVAL/ENOTSUP/ENOSPC/ERANGE mappings.
- fixed: Terminal status observation is part of the selected contract. After a
  matching echo reply or explicit timeout, the caller can observe completed or
  timed-out status through the adapter until close or a new start clears the
  terminal status through the underlying UserspacePingOperation.
- removed: Direct shell ping is rejected for this slice because Matthew's
  feature-led policy bars fake/kernel-backed command expansion unless it is
  backed by accepted userspace, descriptor, syscall, and VFS layers.
- deferred: Public sockets, stable syscall ABI acceptance, socket syscall
  numbers, live driver adapters, live packet I/O, hardware reachability,
  smoltcp, UDP/TCP, SSH, lab mutation, boot publication, Phase 12.1
  link-hardware retry, and Phase 12.4 socket integration remain outside this
  contract.
- not-an-issue: The stable syscall vocabulary in src/syscall.rs can continue
  to reject unsupported calls while this proof-only adapter is developed. The
  accepted Phase 8 open/read syscall-substitute pattern already separates
  host/QEMU substitute dispatch from broader POSIX or trap-entry acceptance.

## Selected Binding

The next implementation task should add a host-only proof adapter around the
accepted descriptor table rather than wiring a public syscall number. The
adapter may live in src/syscall.rs because it is a syscall-substitute
boundary, but it must be called explicitly by tests/smoke code with typed
kernel-owned context:

- a mutable NetworkPingOperationDescriptorTable;
- a mutable fake/trait-level NetworkDevice;
- caller-owned receive and transmit buffers;
- explicit endpoint, route policy, destination IPv4, identifier, sequence,
  TTL, payload, and ARP retry budget for start;
- scalar descriptor and command/result values that can later inform a real ABI
  without becoming one now.

This keeps the accepted path mechanically implementable over
NetworkPingOperationDescriptorTable, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
caller-owned buffers only.

## Rejected Alternatives

- Direct shell ping command: rejected because it would be user-visible command
  behavior before the syscall substitute has retained evidence.
- Public socket API or Phase 12.4 socket expansion: rejected because this task
  only proves one ping-operation lifecycle, not general socket semantics.
- Stable SVC syscall ABI acceptance: rejected because no user ABI, trap entry,
  or userspace program call path is accepted by this slice.
- Live driver adapter or hardware packet I/O: rejected because Phase 12.1 live
  link/packet hardware remains paused and no live packet path is accepted.
- Autonomous timers, queues, or scheduler wakeups: rejected because the
  accepted operation remains caller-driven through start/pump/retry/timeout.

## Evidence Reviewed

- Source: src/syscall.rs stable dispatch and descriptor-substitute dispatch
  patterns.
- Source: src/posix.rs PosixError, descriptor table/object vocabulary, and
  accepted descriptor error names.
- Source: src/network.rs UserspacePingOperation,
  NetworkPingOperationDescriptor, and NetworkPingOperationDescriptorTable.
- Task: tasks/2026-06-03-phase8-open-read-syscall-surface.md for the
  accepted open/read syscall-substitute precedent.
- Task: tasks/2026-05-30-phase7-file-descriptor-table-closeout-checkpoint.md
  for accepted descriptor lifetime/error boundaries.
- Task: tasks/2026-06-20-phase12-network-ping-operation-descriptor-contract-core.md
  and tasks/2026-06-20-phase12-network-ping-operation-descriptor-substitute-smoke-closeout.md
  for the accepted ping descriptor boundary and retained substitute evidence.

## Validation

- static/source/task/doc review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

No Rust source was touched, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's
conditional gates. No hardware lock, lab mutation, boot publication, QEMU run,
or Pi 5 run was performed.

## Acceptance

Accepted. selected_next_task=phase12-network-ping-operation-syscall-substitute-core-20260620.

Commit: recorded in durable supervisor state after commit creation.

The accepted boundary is host-only contract evidence for a proof-only
syscall-substitute adapter over the already accepted ping operation descriptor
table. It does not accept shell ping, public sockets, stable syscall ABI,
socket syscall ABI, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
12.1 link-hardware retry, or phase transition.
