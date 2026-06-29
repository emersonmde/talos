# Phase 12 SSH Live TCP Network-Device Smoltcp Runtime Core

Task id: phase12-ssh-live-tcp-network-device-smoltcp-runtime-core-20260629

Status: accepted after commit.

Classification: ssh-live-tcp-network-device-smoltcp-runtime-core-accepted.

Evidence level: source implementation, focused unit tests, docs build, diff
checks, task-owned JSON evidence, and static redaction review. No Pi 5
hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root retry, OpenSSH retry, external live TCP attempt, remote receipt
claim, compatibility claim, hardware proof, ssh-ready=true claim, service
success claim, runtime russh adoption, fake command expansion, broad shell
work, or phase transition was performed.

## Goal

Implement the thinnest local deterministic runtime binding that proves
src/network.rs can drive a NetworkDevice/DriverPacketAdapter-backed smoltcp TCP
listener path and connect that evidence to the existing descriptor-facing
accepted-connection witness, while preserving all live/hardware readiness
claims as false.

## Scope Performed

- Added a smoltcp::phy::Device implementation for DriverPacketAdapter so the
  selected driver-packet boundary can own a smoltcp interface/listener poll
  path instead of being only a packet queue helper.
- Added LiveTcpNetworkDeviceRuntimeBindingState and
  LiveTcpNetworkDeviceRuntimeReport in src/network.rs.
- Added NetworkSocketDescriptorTable::live_tcp_network_device_smoltcp_runtime_binding.
  The report requires the accepted descriptor-facing delivery witness before it
  can report deterministic device-interface delivery.
- Added a deterministic DriverPacketAdapter-backed smoltcp listener transfer
  path that moves frames through driver packet adapters, establishes client and
  server TCP socket states, and transfers a metadata-only fixed payload.
- Added focused tests that distinguish the accepted host-only descriptor bridge
  from the new deterministic device-interface runtime report and prove missing
  descriptor, device-interface, and hardware-frame-provider prerequisites fail
  closed.

## Non-goals Preserved

- No Pi 5 hardware/lab action, hardware lock acquisition, boot publication,
  generated-root retry, OpenSSH retry, external live TCP attempt, remote receipt
  claim, compatibility claim, hardware proof, ssh-ready=true claim, service
  success claim, runtime russh adoption, fake command expansion, broad shell
  work, or phase transition.
- No durable retention of peer identifiers, addresses, packet payload contents,
  key material, session material, boot artifact bytes, private user data,
  stable secret-derived identifiers, or hardware data.
- No promotion of RP1 Ethernet into socket, descriptor, SSH readiness, syscall
  dispatch, or service-success ownership.

## Findings

- fixed: DriverPacketAdapter now implements smoltcp::phy::Device, giving the
  selected network-owned driver-packet boundary a real smoltcp interface poll
  surface.
- fixed: src/network.rs now exposes LiveTcpNetworkDeviceRuntimeReport and
  LiveTcpNetworkDeviceRuntimeBindingState for the deterministic runtime binding.
- fixed: the deterministic runtime report is connected to
  LiveTcpListenerDescriptorAcceptReport and only accepts deterministic
  device-interface delivery after descriptor-facing accepted delivery is present.
- fixed: focused tests prove the deterministic runtime binding is distinct from
  the accepted host-only bridge: host-only reports device_interface_bound=false,
  while the runtime report requires the DriverPacketAdapter path and records
  deterministic_device_interface_bound=true.
- fixed: missing descriptor delivery, missing deterministic device-interface
  binding, and missing hardware frame provider remain fail-closed and keep live
  packet I/O, live reachability, remote receipt, compatibility, service
  success, and ssh_ready false.
- deferred: Pi 5 packet/device-interface proof, RP1 Ethernet frame-provider
  binding, remote receipt, OpenSSH compatibility, generated-root retry, service
  success, ssh-ready=true, runtime russh adoption, and phase transition remain
  deferred to later explicit tasks.
- removed: no source, docs, helper, task, or evidence artifact was removed.
- not-an-issue: the deterministic payload used inside unit tests is fixed local
  metadata and is not retained as durable evidence.

## Accepted Runtime Boundary

Accepted in this task:

- DriverPacketAdapter can be used directly as a smoltcp device.
- src/network.rs can run a deterministic smoltcp TCP listener/client exchange
  through DriverPacketAdapter frame movement.
- NetworkSocketDescriptorTable can report that the deterministic runtime path
  is connected to an already accepted descriptor-facing connection witness.
- The accepted report is local deterministic evidence only.

Still unaccepted:

- Pi 5 packet I/O, real hardware frame-provider binding, remote receipt,
  OpenSSH compatibility, hardware proof, service success, ssh-ready=true,
  generated-root/OpenSSH retry, runtime russh adoption, fake command expansion,
  broad shell work, and phase transition.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Source contains an explicit deterministic NetworkDevice/DriverPacketAdapter to
  smoltcp listener path and report owned by src/network.rs and connected to
  descriptor-facing accepted delivery: satisfied by
  DriverPacketAdapter's smoltcp::phy::Device impl,
  driver_packet_smoltcp_listener_transfer, and
  live_tcp_network_device_smoltcp_runtime_binding.
- Focused tests demonstrate local deterministic device/interface delivery
  reaches the descriptor-facing accepted-connection witness and distinguish it
  from the accepted host-only bridge: satisfied by
  live_tcp_network_device_smoltcp_runtime_binding_reaches_descriptor_delivery.
- Missing real device/interface or hardware frame-provider conditions remain
  fail-closed and keep live reachability, remote receipt, compatibility,
  hardware proof, service success, and ssh-ready false: satisfied by
  live_tcp_network_device_smoltcp_runtime_binding_fails_closed_without_runtime_prerequisites.
- Durable evidence retains only paths, public labels, metadata counters,
  commands/results, and task ids: satisfied.
- Accepted work is committed and selects
  phase12-ssh-live-tcp-network-device-smoltcp-runtime-closeout-20260629 as the
  next task: satisfied after commit.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-network-device-smoltcp-runtime-core/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-network-device-smoltcp-runtime-core/evidence-map.json.
- Source changed:
  src/network.rs.
- Docs changed:
  docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Predecessor closeout:
  tasks/2026-06-29-phase12-ssh-live-tcp-device-interface-frontier-closeout.md.

## Redaction Review

Durable evidence records task ids, source paths, public classifier names,
validation commands/results, metadata-only enum/report labels, and boolean or
count fields. It does not retain peer identifiers, addresses, packet payload
contents, key material, session material, boot artifact bytes, private user
data, stable secret-derived identifiers, or hardware data.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet
  live_tcp_network_device_smoltcp_runtime_binding: pass.
- cargo -Zjson-target-spec test --quiet live_tcp_listener_descriptor_boundary:
  pass.
- cargo -Zjson-target-spec test --quiet driver_packet_adapter: pass.
- cargo -Zjson-target-spec test --quiet ssh_service_readiness: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-network-device-smoltcp-runtime-closeout-20260629.

No Pi 5 hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, external live TCP attempt, remote receipt claim,
compatibility claim, hardware proof, ssh-ready=true claim, service success
claim, runtime russh adoption, fake command expansion, broad shell work, or
phase transition was performed.

Commit: recorded in talos-supervisor-state.json after final commit.
