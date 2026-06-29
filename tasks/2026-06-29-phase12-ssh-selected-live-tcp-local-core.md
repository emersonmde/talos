# Phase 12 SSH Selected Live TCP Local Core

Task id: phase12-ssh-selected-live-tcp-local-core-20260629

Status: accepted after commit.

Classification: ssh-live-tcp-local-core-accepted-source-boundary.

Evidence level: Rust source implementation, focused unit tests, task-owned JSON
evidence, docs build, and diff checks. No Pi 5 hardware/lab action,
hardwareTestLock acquisition, boot publication, generated-root retry, OpenSSH
retry, packet I/O, live TCP connection attempt, remote receipt claim,
compatibility claim, ssh-ready=true claim, fake command expansion, broad shell
grammar expansion, runtime russh adoption, or phase transition was performed.

## Goal

Implement the selected local/static source boundary that distinguishes the
descriptor-local smoltcp bridge from a future live TCP listener/device bridge.

## Scope Performed

- Added LiveTcpListenerDescriptorBoundary and
  LiveTcpListenerDescriptorBoundaryReport to src/network.rs.
- Added NetworkSocketDescriptorTable::live_tcp_listener_descriptor_boundary.
- Covered the boundary with a focused network test that proves:
  - pre-accept state is BlockedNoDescriptorBridge,
  - accepted local source boundary requires an Established host-only smoltcp
    handshake, accepted Talos descriptor attachment, and payload transfer
    metadata,
  - a future device/interface requirement reports
    BlockedNoDeviceInterfaceBinding with device_interface_bound=false.
- Extended the private userspace socket ABI regression to assert the accepted
  local source boundary is reachable through descriptor-backed syscall
  dispatch.
- Updated Phase 12 docs and roadmap with the accepted source boundary and
  remaining device/interface gap.

## Non-goals Preserved

- No Pi 5 hardware/lab action, hardware lock acquisition, boot publication,
  generated-root retry, OpenSSH retry, packet I/O, live TCP attempt, remote
  receipt claim, compatibility claim, ssh-ready=true claim, fake command
  expansion, broad shell grammar expansion, runtime russh adoption, or phase
  transition.
- No durable retention of peer identifiers, addresses, packet payload contents,
  key/session material, boot artifacts, hardware data, or private user data.

## Findings

- fixed: src/network.rs now has an explicit source report for the selected
  local boundary. It accepts only the host-only descriptor-backed smoltcp bridge
  path and names the future live device/interface binding as absent.
- fixed: src/userspace_socket_abi.rs now asserts private socket syscall
  dispatch reaches the accepted local source boundary.
- fixed: readiness-adjacent booleans remain fail-closed. The new report always
  returns live_packet_io_accepted=false, live_reachability_accepted=false,
  remote_receipt_accepted=false, compatibility_accepted=false, and
  ssh_ready=false.
- blocked: live TCP listener/device ownership remains blocked on a selected
  device/interface binding model and later live/hardware evidence.
- deferred: Pi 5 hardware proof, OpenSSH compatibility, packet I/O, remote
  receipt, ssh-ready=true, and Ethernet selected-discriminator work remain
  outside this task.
- removed: no source, docs, helper, task, or evidence artifact was removed.
- not-an-issue: the existing host-only smoltcp bridge was already accepted as a
  private local bridge; this task does not relabel it as live TCP.

## Accepted Boundary

NetworkSocketDescriptorTable::live_tcp_listener_descriptor_boundary accepts
AcceptedLocalSourceBoundary only when the retained SmoltcpSocketBridgeRecord
shows an Established client/server smoltcp handshake and an accepted Talos
descriptor has been attached. The report exposes payload transfer count and
last payload length as metadata-only proof that the bridge can feed a
descriptor-facing accepted connection.

The same report deliberately does not claim live behavior. If called in a mode
that requires device/interface binding, it reports
BlockedNoDeviceInterfaceBinding and keeps device_interface_bound=false. Live
packet I/O, live reachability, remote receipt, compatibility, and ssh_ready are
hard-coded false until a later explicit live task accepts them.

## Selected Next Task

selected_next_task: phase12-ssh-local-to-live-tcp-gap-closeout-20260629.

The closeout is mechanically objective because this task accepted the exact
local source boundary selected by the inventory and preserved the future live
device/interface gap without unblocking hardware, OpenSSH, packet I/O, remote
receipt, compatibility, ssh-ready=true, fake command expansion, or phase
transition work.

## Evidence Map

- Source changed:
  - src/network.rs.
  - src/userspace_socket_abi.rs.
- Docs changed:
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/roadmap.md.
- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-selected-live-tcp-local-core/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-selected-live-tcp-local-core/evidence-map.json.
- Predecessor inventory:
  tasks/2026-06-29-phase12-ssh-local-to-live-tcp-gap-inventory.md.

## Redaction Review

The new tests use synthetic local endpoint metadata and fixed test payload
lengths only. Durable evidence records source paths, classification labels,
validation commands, counts, and boolean readiness boundaries. It does not
retain real peer identifiers, addresses, packet payload contents, key/session
material, boot artifacts, hardware data, or private user data.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- cargo fmt --all -- --check: pass after formatting.
- cargo -Zjson-target-spec test --quiet
  live_tcp_listener_descriptor_boundary_accepts_local_source_bridge_only: pass.
- cargo -Zjson-target-spec test --quiet
  userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge: pass.
- cargo -Zjson-target-spec test --quiet
  socket_delivery_local_model_delivers_input_and_output_through_stream_socket:
  pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; retained large search index
  warning.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

Accepted as ssh-live-tcp-local-core-accepted-source-boundary.

selected_next_task: phase12-ssh-local-to-live-tcp-gap-closeout-20260629.

No hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, live TCP attempt, packet I/O, remote receipt
claim, compatibility claim, ssh-ready=true claim, fake command expansion, or
phase transition is accepted.
