# Phase 12.4 Descriptor-Shaped Ping Control Smoke

Task: phase12-network-descriptor-shaped-ping-control-smoke-20260620

Status: accepted

Classification: phase12-network-descriptor-shaped-ping-control-smoke-accepted

## Scope

Retain host/QEMU-substitute smoke evidence for the accepted
DescriptorShapedPingControl core. This task is limited to a task-owned smoke
script, retained transcript evidence, task/docs updates, and validation gates.
It does not add runtime behavior.

## Findings And Dispositions

- fixed: Added scripts/qemu-descriptor-shaped-ping-control-smoke.sh as the
  task-owned QEMU/substitute smoke command for DescriptorShapedPingControl.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-20-descriptor-shaped-ping-control-smoke/. The
  transcript labels the accepted host-only boundary and runs the
  descriptor_shaped_ping_control target test filter.
- fixed: The retained smoke transcript demonstrates the descriptor-shaped
  lifecycle over fake/trait-level NetworkDevice behavior and caller-owned
  buffers: open, idle status, start to unresolved-ARP pending, runtime pump
  ARP advancement to inflight, runtime pump echo-reply completion, terminal
  completed status, and close.
- fixed: The retained smoke transcript also covers invalid and closed
  descriptors, zero descriptor capacity, duplicate active open, caller receive
  buffer pressure, retry exhaustion, explicit timeout, receive IO error, local
  transmit IO error, and active-ping transmit IO error.
- fixed: The smoke script now adds the local rustup cargo bin directory to
  PATH when present and preserves the cargo test exit status while retaining
  the transcript.
- removed: No shell ping command, public socket API, stable syscall ABI,
  socket syscall ABI, live driver adapter, live packet I/O, hardware
  reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
  12.1 retry, broad Phase 12.4 socket expansion, or phase transition was added
  or accepted.
- not-an-issue: The smoke remains host/QEMU-substitute evidence rather than a
  Pi 5 hardware run. This task explicitly requires fake/trait-level evidence
  and rejects live packet I/O and hardware reachability claims.

## Evidence

- Smoke script:
  scripts/qemu-descriptor-shaped-ping-control-smoke.sh.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-06-20-descriptor-shaped-ping-control-smoke/qemu-descriptor-shaped-ping-control-smoke.log.
- Source boundary:
  src/syscall.rs DescriptorShapedPingControl and
  RuntimePingOperationSyscallSubstitute.
- Prior accepted contract:
  tasks/2026-06-20-phase12-network-descriptor-shaped-ping-control-contract.md.
- Prior accepted core:
  tasks/2026-06-20-phase12-network-descriptor-shaped-ping-control-core.md.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed, 650 no_std tests.
- QEMU/substitute smoke:
  scripts/qemu-descriptor-shaped-ping-control-smoke.sh: passed.
- diff validation: git diff --check: passed.
- docs build: /home/node/.cargo/bin/mdbook build: passed, existing large
  search-index warning only.
- staged diff validation: git diff --cached --check: passed.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, live packet I/O, SSH,
or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=null.

planningNeeded=true.

The accepted evidence level is host/QEMU-substitute smoke evidence over
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, local ARP/ICMP responder behavior, active ping
descriptor dispatch, UserspacePingOperation, SinglePingPacketService,
fake/trait-level NetworkDevice behavior, caller-owned buffers, and
fixed-capacity state.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad Phase 12.4 socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
