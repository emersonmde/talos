# Phase 12 SSH Live TCP Listener Descriptor Accept Local Core

Task id: phase12-ssh-live-tcp-listener-descriptor-accept-local-core-20260629

Status: accepted after commit.

Classification: ssh-live-tcp-listener-descriptor-accept-local-core-accepted.

Evidence level: Rust source implementation, focused no_std unit tests,
task-owned JSON evidence, docs build, and diff checks. No Pi 5 hardware/lab
action, hardwareTestLock acquisition, boot publication, generated-root retry,
OpenSSH retry, packet I/O, live TCP connection attempt, remote receipt claim,
compatibility claim, ssh-ready=true claim, fake command expansion, runtime
russh adoption, or phase transition was performed.

## Goal

Implement the local/static descriptor-facing accepted-connection witness for
the selected smoltcp listener boundary without accepting live networking.

## Scope Performed

- Added LiveTcpAcceptedConnectionDeliveryState and
  LiveTcpListenerDescriptorAcceptReport to src/network.rs.
- Added NetworkSocketDescriptorTable::live_tcp_listener_descriptor_accept_delivery
  to classify descriptor-facing accepted-connection delivery from the selected
  LiveTcpListenerDescriptorBoundaryReport.
- Kept AcceptedLocalDescriptorDelivery limited to the local/static condition
  where the smoltcp bridge is Established, an accepted Talos descriptor is
  attached, and that descriptor is in NetworkSocketState::Accepted for the same
  connection id.
- Kept the real-device/interface-required path fail-closed as
  BlockedMissingDeviceInterfaceBinding with
  descriptor_facing_connection_delivered=false and all live/SSH-ready labels
  false.
- Extended focused network tests to cover pre-accept blocked state, accepted
  local descriptor delivery, and missing real device/interface binding.
- Updated Phase 12 docs and roadmap with the accepted descriptor-delivery
  labels and selected next readiness-label task.

## Non-goals Preserved

- No Pi 5 hardware/lab action, hardware lock acquisition, boot publication,
  generated-root retry, OpenSSH retry, packet I/O, live TCP attempt, remote
  receipt claim, compatibility claim, ssh-ready=true claim, fake command
  expansion, runtime russh adoption, or phase transition.
- No runtime russh adoption, no fake command expansion, no SSH service success,
  and no promotion of RP1 Ethernet into socket, descriptor, readiness, or
  syscall ownership.
- No durable retention of peer identifiers, addresses, packet payload contents,
  key/session material, boot artifacts, hardware data, private user data, or
  stable secret-derived identifiers.

## Findings

- fixed: src/network.rs now has a descriptor-accept report that proves local
  Talos descriptor delivery from the selected smoltcp listener boundary.
- fixed: the success report requires both an accepted descriptor attachment and
  NetworkSocketState::Accepted for the same connection id.
- fixed: focused tests cover accepted local descriptor delivery and the
  fail-closed real device/interface-required path.
- deferred: readiness composition for the accepted local/static descriptor
  prerequisite remains the next local/static task.
- deferred: Pi 5 hardware proof, OpenSSH compatibility, packet I/O, remote
  receipt, ssh-ready=true, generated-root retry, runtime russh adoption, and
  phase transition remain outside this task.
- not-an-issue: the host-only smoltcp bridge remains local/static witness
  evidence and is not relabeled as live packet I/O.
- removed: no source, docs, helper, task, or evidence artifact was removed.

## Accepted Local/Static States

- delivery_state: AcceptedLocalDescriptorDelivery.
- success prerequisites: Established smoltcp bridge, accepted Talos descriptor
  attachment, and Accepted socket state for the same connection id.
- fail-closed state: BlockedMissingDeviceInterfaceBinding when a real
  device/interface binding is required.
- rejected live claims: live_packet_io_accepted=false,
  live_reachability_accepted=false, remote_receipt_accepted=false,
  compatibility_accepted=false, and ssh_ready=false.

## Selected Next Task

selected_next_task:
phase12-ssh-live-tcp-readiness-label-local-core-20260629.

That successor remains mechanically objective because this task exposed the
accepted local/static descriptor-delivery prerequisite while preserving
fail-closed live labels. The next local implementation step can now compose
readiness labels without accepting live packet I/O, remote receipt,
compatibility, Pi 5 hardware proof, ssh-ready=true, runtime russh adoption,
fake command expansion, or phase transition.

## Evidence Map

- Source changed:
  - src/network.rs.
- Docs changed:
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/roadmap.md.
- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-listener-descriptor-accept-local-core/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-listener-descriptor-accept-local-core/evidence-map.json.
- Predecessor local core:
  tasks/2026-06-29-phase12-ssh-live-tcp-device-interface-local-core.md.

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
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet
  live_tcp_listener_descriptor_boundary_accepts_local_source_bridge_only: pass;
  the repo's no_std harness executed the full 888-test suite and reported ok.
- cargo -Zjson-target-spec test --quiet
  userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge: pass; the
  repo's no_std harness executed the full 888-test suite and reported ok.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; retained large search index
  warning.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

Accepted as ssh-live-tcp-listener-descriptor-accept-local-core-accepted.

Commit hash is recorded in durable supervisor state after commit.

selected_next_task:
phase12-ssh-live-tcp-readiness-label-local-core-20260629.

No hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, live TCP attempt, packet I/O, remote receipt
claim, compatibility claim, ssh-ready=true claim, fake command expansion,
runtime russh adoption, or phase transition is accepted.
