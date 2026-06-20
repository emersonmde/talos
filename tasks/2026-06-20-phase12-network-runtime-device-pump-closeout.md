# Phase 12.3 Network Runtime Device Pump Closeout

Task: phase12-network-runtime-device-pump-closeout-20260620

Status: accepted

Classification: phase12-network-runtime-device-pump-closeout-accepted

## Scope

Close out the accepted host-only NetworkRuntimeDevicePump implementation
frontier without expanding into live packet I/O, sockets, hardware, SSH, or a
phase transition. This task reconciles the core implementation, tests, task
record, docs, durable state, and follow-up selection.

## Findings And Dispositions

- fixed: Reviewed the accepted runtime pump core record and implementation.
  The accepted boundary is a caller-driven NetworkDevice service over
  caller-owned receive/transmit buffers, fixed-capacity local ARP state,
  fixed-capacity ping-operation descriptors, local ARP/ICMP dispatch, and the
  existing ping operation stack.
- fixed: Confirmed the accepted evidence covers no-frame, nonlocal/no-reply,
  local ARP reply transmit, local ICMP echo reply transmit, active ping
  ARP-to-ICMP advancement, echo-reply completion, receive-buffer pressure,
  receive error, local and active transmit errors, retry, timeout, terminal
  status observation, and responder-vs-active ordering.
- fixed: Reconciled docs and roadmap so the selected next mechanically
  dependency-gated task is
  phase12-network-runtime-device-pump-substitute-smoke-core-20260620.
- removed: No shell ping command, public socket API, stable syscall ABI,
  socket syscall ABI, live driver adapter, live packet I/O, hardware
  reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
  12.1 retry, Phase 12.4 expansion, or phase transition was added or accepted.
- deferred: Retained QEMU/substitute smoke evidence for the runtime pump is
  selected as the next bounded task before any broader runtime, driver, socket,
  hardware, or SSH work.
- not-an-issue: The closeout found no requirement to alter the accepted
  NetworkRuntimeDevicePump source. The implementation commit already contains
  source, unit-test, docs, and task-record evidence for the accepted core.

## Evidence Reviewed

- Core task record:
  tasks/2026-06-20-phase12-network-runtime-device-pump-core.md.
- Core implementation commit:
  f8cf1d4133282933aef0385e3fc394068cde3cb3.
- Source boundary: src/network.rs exposes NetworkRuntimeDevicePump,
  NetworkRuntimeDevicePumpStepResult, pump_received adapters, and deterministic
  local-responder-before-active-ping ordering.
- Validation retained by the core: cargo fmt --all -- --check;
  cargo -Zjson-target-spec test --quiet network_runtime_device_pump;
  cargo -Zjson-target-spec test --quiet ping_operation_syscall_substitute;
  git diff --check; /home/node/.cargo/bin/mdbook build; git diff --cached
  --check.

## Acceptance

Accepted.

The runtime pump core is accepted as host-only source/unit/QEMU-substitute
evidence over NetworkDevice/fake-device behavior, caller-owned buffers,
fixed-capacity state, local ARP/ICMP dispatch, NetworkPingOperationDescriptorTable,
UserspacePingOperation, SinglePingPacketService, and the proof-only syscall
substitute lineage. It does not accept live driver adapters, live packet I/O,
network reachability, sockets, shell ping, SSH, smoltcp, UDP/TCP, hardware
work, lab mutation, boot publication, Phase 12.1 retry, Phase 12.4 expansion,
or a phase transition.

selected_next_task=phase12-network-runtime-device-pump-substitute-smoke-core-20260620.

Commit: recorded in durable supervisor state after commit creation.
