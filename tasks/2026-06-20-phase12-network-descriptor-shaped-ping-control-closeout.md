# Phase 12.4 Descriptor-Shaped Ping Control Closeout

Task: phase12-network-descriptor-shaped-ping-control-closeout-20260620

Status: accepted

Classification: phase12-network-descriptor-shaped-ping-control-closeout-accepted

## Scope

Close out the accepted DescriptorShapedPingControl source/unit and retained
host/QEMU-substitute smoke evidence. This task is limited to static source,
task, doc, and evidence review plus closeout documentation. It does not add
runtime behavior, shell ping, public sockets, stable syscall ABI, live packet
I/O, hardware reachability, SSH, lab mutation, boot publication, or a phase
transition.

## Findings And Dispositions

- fixed: Reconciled the accepted DescriptorShapedPingControl implementation in
  src/syscall.rs with the contract task. The control remains a thin
  crate-internal wrapper over RuntimePingOperationSyscallSubstitute and borrows
  the caller-provided NetworkRuntimeDevicePump plus caller-owned receive and
  transmit buffers.
- fixed: Reconciled the accepted source/unit evidence with the smoke task. The
  covered lifecycle is open, idle status, start to unresolved-ARP pending,
  runtime-pump ARP advancement to inflight, runtime-pump echo-reply completion,
  terminal completed status, and close.
- fixed: Reconciled deterministic negative/error coverage: invalid and closed
  descriptors, zero descriptor capacity, duplicate active open, caller receive
  buffer pressure, retry exhaustion, explicit timeout, receive IO error, local
  transmit IO error, and active-ping transmit IO error.
- fixed: Reconciled the retained host/QEMU-substitute transcript at
  tasks/evidence/2026-06-20-descriptor-shaped-ping-control-smoke/
  qemu-descriptor-shaped-ping-control-smoke.log with the task-owned smoke
  command scripts/qemu-descriptor-shaped-ping-control-smoke.sh.
- removed: No extra implementation, shell ping command, kernel-backed fake
  command expansion, public socket API, stable syscall ABI acceptance, socket
  syscall ABI acceptance, live driver adapter, live packet I/O, hardware
  reachability, SSH, smoltcp, UDP/TCP, packet queue, autonomous timer, lab
  mutation, boot publication, Phase 12.1 link-hardware retry, broad Phase 12.4
  socket expansion, or phase transition was added or accepted by this closeout.
- deferred: Any user-visible shell ping, public socket/syscall ABI,
  live-driver, hardware-reachability, SSH, or broader Phase 12.4 socket work is
  deferred until supervisor planning selects a new bounded task with explicit
  acceptance gates.
- not-an-issue: This closeout performs no Pi 5 hardware run. The accepted
  descriptor-shaped ping control evidence is intentionally host/QEMU-substitute
  over fake/trait-level NetworkDevice behavior and caller-owned buffers.

## Evidence Reviewed

- Source: src/syscall.rs DescriptorShapedPingControl and
  RuntimePingOperationSyscallSubstitute.
- Source tests: src/syscall.rs descriptor_shaped_ping_control_* tests.
- Smoke command: scripts/qemu-descriptor-shaped-ping-control-smoke.sh.
- Retained transcript:
  tasks/evidence/2026-06-20-descriptor-shaped-ping-control-smoke/
  qemu-descriptor-shaped-ping-control-smoke.log.
- Contract task:
  tasks/2026-06-20-phase12-network-descriptor-shaped-ping-control-contract.md.
- Core task:
  tasks/2026-06-20-phase12-network-descriptor-shaped-ping-control-core.md.
- Smoke task:
  tasks/2026-06-20-phase12-network-descriptor-shaped-ping-control-smoke.md.

## Validation

- static/source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed, existing large search-index
  warning only.
- git diff --cached --check: passed.

No Rust source or scripts were touched by this closeout, so cargo fmt --all
-- --check and cargo -Zjson-target-spec test --quiet were not required by this
task's gates. No Pi 5 hardware run, hardwareTestLock acquisition, boot archive
publication, lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA
work, shell ping, public socket API, stable syscall ABI acceptance, live packet
I/O, SSH, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=null.

planningNeeded=true.

The accepted evidence level is host/QEMU-substitute over
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, local ARP/ICMP responder behavior, active ping
descriptor dispatch, UserspacePingOperation, SinglePingPacketService,
fake/trait-level NetworkDevice behavior, caller-owned buffers, fixed-capacity
state, retained smoke transcript, and task/doc review.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad Phase 12.4 socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
