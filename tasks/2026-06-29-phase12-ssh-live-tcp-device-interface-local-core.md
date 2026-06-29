# Phase 12 SSH Live TCP Device/Interface Local Core

Task id: phase12-ssh-live-tcp-device-interface-local-core-20260629

Status: accepted after commit.

Classification: ssh-live-tcp-device-interface-local-core-accepted.

Evidence level: Rust source implementation, focused no_std unit tests,
task-owned JSON evidence, docs build, and diff checks. No Pi 5 hardware/lab
action, hardwareTestLock acquisition, boot publication, generated-root retry,
OpenSSH retry, packet I/O, live TCP connection attempt, remote receipt claim,
compatibility claim, ssh-ready=true claim, fake command expansion, runtime
russh adoption, or phase transition was performed.

## Goal

Represent the selected
network-owned-smoltcp-interface-with-driver-packet-adapter-ingress-and-descriptor-table-delivery
ownership model in source with explicit fail-closed states for missing
device/interface binding.

## Scope Performed

- Added LiveTcpDeviceInterfaceOwnershipModel to src/network.rs and attached
  the selected ownership model to LiveTcpListenerDescriptorBoundaryReport.
- Added LiveTcpDeviceInterfaceBindingState to distinguish the accepted
  host-only local source boundary from the required real device/interface path.
- Kept the accepted host-only descriptor bridge behavior unchanged:
  AcceptedLocalSourceBoundary still requires an Established smoltcp bridge,
  accepted Talos descriptor attachment, and payload-transfer metadata.
- Preserved fail-closed real-device binding behavior:
  BlockedNoDeviceInterfaceBinding now carries
  BlockedMissingDeviceInterfaceBinding with device_interface_bound=false and
  live_packet_io_accepted=false.
- Extended focused network tests to assert the selected ownership metadata and
  blocked real-device binding labels without live packet I/O.
- Updated Phase 12 docs and roadmap with the accepted local/static source
  states and the selected next descriptor-accept task.

## Non-goals Preserved

- No Pi 5 hardware/lab action, hardware lock acquisition, boot publication,
  generated-root retry, OpenSSH retry, packet I/O, live TCP attempt, remote
  receipt claim, compatibility claim, ssh-ready=true claim, fake command
  expansion, runtime russh adoption, or phase transition.
- No runtime russh adoption, no fake command expansion, no SSH service success,
  and no promotion of RP1 Ethernet into socket, descriptor, readiness, or
  syscall ownership.
- No durable retention of peer identifiers, addresses, packet payload
  contents, key/session material, boot artifacts, hardware data, private user
  data, or stable secret-derived identifiers.

## Findings

- fixed: src/network.rs now exposes the selected ownership model as typed
  metadata on LiveTcpListenerDescriptorBoundaryReport.
- fixed: src/network.rs now exposes the missing real device/interface binding
  as BlockedMissingDeviceInterfaceBinding instead of only a boolean false.
- fixed: focused tests assert both the accepted local/static ownership metadata
  and the blocked real-device binding labels while live_packet_io_accepted,
  live_reachability_accepted, remote_receipt_accepted,
  compatibility_accepted, and ssh_ready stay false.
- deferred: descriptor-facing accept delivery from a selected live listener is
  still the next local/static task.
- deferred: Pi 5 hardware proof, OpenSSH compatibility, packet I/O, remote
  receipt, ssh-ready=true, generated-root retry, runtime russh adoption, and
  phase transition remain outside this task.
- not-an-issue: the existing host-only smoltcp descriptor bridge remains the
  accepted local/static control and is not relabeled as live networking.
- removed: no source, docs, helper, task, or evidence artifact was removed.

## Accepted Local/Static States

- ownership_model:
  NetworkOwnedSmoltcpInterfaceWithDriverPacketAdapterIngressAndDescriptorTableDelivery.
- accepted local bridge state:
  LocalSourceBoundaryDoesNotRequireDeviceInterface with
  AcceptedLocalSourceBoundary.
- blocked real-device state:
  BlockedMissingDeviceInterfaceBinding with
  BlockedNoDeviceInterfaceBinding, device_interface_bound=false, and
  live_packet_io_accepted=false.
- rejected live claims:
  live_reachability_accepted=false, remote_receipt_accepted=false,
  compatibility_accepted=false, and ssh_ready=false.

## Selected Next Task

selected_next_task:
phase12-ssh-live-tcp-listener-descriptor-accept-local-core-20260629.

That successor remains mechanically objective because this task represented the
selected ownership model and the fail-closed missing device/interface binding
state. The next local implementation step can now focus on descriptor-facing
accepted-connection delivery while still rejecting live packet I/O, remote
receipt, compatibility, Pi 5 hardware proof, ssh-ready=true, runtime russh
adoption, fake command expansion, and phase transition.

## Evidence Map

- Source changed:
  - src/network.rs.
- Docs changed:
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/roadmap.md.
- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-device-interface-local-core/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-device-interface-local-core/evidence-map.json.
- Predecessor ownership contract:
  tasks/2026-06-29-phase12-ssh-live-tcp-device-interface-ownership-contract.md.

## Redaction Review

The code and tests use synthetic local descriptor state, enum labels, boolean
readiness boundaries, and payload length/count metadata only. Durable evidence
records task ids, source paths, public classifier names, validation commands,
and metadata-only state labels. It does not retain peer identifiers, addresses,
packet payload contents, key material, session material, boot artifact bytes,
hardware data, private user data, or stable secret-derived identifiers.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- cargo fmt --all -- --check: pass after formatting.
- cargo -Zjson-target-spec test --quiet
  live_tcp_listener_descriptor_boundary_accepts_local_source_bridge_only: pass;
  the repo's no_std harness executed the full 888-test suite and reported
  ok.
- cargo -Zjson-target-spec test --quiet
  userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge: pass; the
  repo's no_std harness executed the full 888-test suite and reported ok.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

Accepted as ssh-live-tcp-device-interface-local-core-accepted.

selected_next_task:
phase12-ssh-live-tcp-listener-descriptor-accept-local-core-20260629.

No hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, live TCP attempt, packet I/O, remote receipt
claim, compatibility claim, ssh-ready=true claim, fake command expansion,
runtime russh adoption, or phase transition is accepted.
