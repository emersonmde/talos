# Phase 12.3 Network Runtime Device Pump Substitute Smoke Core

Task: phase12-network-runtime-device-pump-substitute-smoke-core-20260620

Status: accepted

Classification: phase12-network-runtime-device-pump-substitute-smoke-core-accepted

## Scope

Retain durable host/QEMU-substitute smoke evidence for the accepted
NetworkRuntimeDevicePump boundary. This task is limited to a task-owned smoke
script, retained transcripts, task/docs evidence, and validation gates. It does
not add runtime behavior.

## Findings And Dispositions

- fixed: Added scripts/qemu-network-runtime-device-pump-smoke.sh as the
  task-owned QEMU/substitute command for the runtime pump boundary.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-20-network-runtime-device-pump-substitute-smoke/.
  The transcript labels the accepted host-only boundary and runs the
  network_runtime_device_pump test filter.
- fixed: The retained transcript covers local ARP reply transmit, local ICMP
  echo reply transmit, local responder priority, one active ping operation
  from unresolved ARP through ICMP transmit and echo-reply completion,
  terminal completed status, retry exhaustion, explicit timeout, no-frame,
  receive-buffer-too-small, receive IO error, local transmit IO error, and
  active transmit IO error.
- fixed: Ran the full cargo no_std unit suite after the smoke script to
  preserve descriptor/syscall-substitute and broader networking regressions.
- removed: No shell ping command, public socket API, stable syscall ABI,
  socket syscall ABI, live driver adapter, live packet I/O, hardware
  reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
  12.1 retry, Phase 12.4 expansion, or phase transition was added or accepted.
- deferred: Closeout of the retained runtime pump smoke evidence remains the
  dependency-gated follow-up task.
- not-an-issue: The smoke task did not require source changes in src/network.rs.
  The accepted runtime pump implementation already contains the tested
  behavior.

## Evidence

- Smoke script:
  scripts/qemu-network-runtime-device-pump-smoke.sh.
- Retained smoke transcript:
  tasks/evidence/2026-06-20-network-runtime-device-pump-substitute-smoke/qemu-network-runtime-device-pump-smoke.log.
- Retained full-suite transcript:
  tasks/evidence/2026-06-20-network-runtime-device-pump-substitute-smoke/cargo-test-quiet.log.
- Source boundary under evidence: NetworkRuntimeDevicePump, local ARP/ICMP
  dispatch, NetworkPingOperationDescriptorTable, UserspacePingOperation,
  SinglePingPacketService, fake/trait-level NetworkDevice behavior,
  caller-owned receive/transmit buffers, and fixed-capacity state.

## Validation

- scripts/qemu-network-runtime-device-pump-smoke.sh: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- git diff --check: passed.
- docs build: /home/node/.cargo/bin/mdbook build: passed.
- staged diff validation: git diff --cached --check: passed.

No hardware lock, lab mutation, boot publication, live packet I/O, shell ping,
public socket API, stable syscall ABI acceptance, SSH, smoltcp, UDP/TCP, Phase
12.1 retry, Phase 12.4 expansion, or phase transition was performed.

## Acceptance

Accepted. selected_next_task=phase12-network-runtime-device-pump-substitute-smoke-closeout-20260620.

The accepted evidence level remains host/QEMU-substitute only over
NetworkRuntimeDevicePump, local ARP/ICMP responder behavior,
NetworkPingOperationDescriptorTable, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior,
caller-owned receive/transmit buffers, and fixed-capacity state.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI,
live driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry, Phase
12.4 socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
