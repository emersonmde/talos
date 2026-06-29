# Phase 12 SSH Local-To-Live TCP Gap Closeout

Task id: phase12-ssh-local-to-live-tcp-gap-closeout-20260629

Status: accepted after commit.

Classification: ssh-local-to-live-tcp-gap-closeout-planning-needed.

Evidence level: static task/source/docs/state review, task-owned JSON
evidence, docs build, and diff checks. No Rust source change, Pi 5
hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root retry, OpenSSH retry, packet I/O, live TCP connection attempt,
remote receipt claim, compatibility claim, ssh-ready=true claim, fake command
expansion, runtime russh adoption, or phase transition was performed.

## Goal

Reconcile the accepted local-to-live TCP gap inventory and selected local-core
source boundary before any live TCP, OpenSSH, or hardware work is considered.

## Scope Performed

- Reviewed the accepted gap inventory, selected local-core task, current
  supervisor state, Phase 12 docs, and roadmap.
- Confirmed the accepted source boundary is local/static only:
  NetworkSocketDescriptorTable::live_tcp_listener_descriptor_boundary accepts
  AcceptedLocalSourceBoundary only for the host-only smoltcp descriptor bridge.
- Confirmed the remaining first missing fact is a selected device/interface
  ownership model that can bind live packet I/O to a TCP listener and Talos
  descriptor delivery without relabeling local modeled delivery as live
  reachability.
- Set planningNeeded for supervisor planning because no later queued task in
  this slice is mechanically unblocked.

## Non-goals Preserved

- No Pi 5 hardware/lab action, hardware lock acquisition, boot publication,
  generated-root retry, OpenSSH retry, packet I/O, live TCP attempt, remote
  receipt claim, compatibility claim, ssh-ready=true claim, fake command
  expansion, runtime russh adoption, or phase transition.
- No selection of a live/generated-root/OpenSSH retry. Any future live retry
  still requires candidate identity, fresh serial cursor, TFTP delta,
  known-good control, and candidate rerun requirements.
- No durable retention of peer identifiers, addresses, packet payloads, key
  material, session material, boot artifact bytes, hardware data, or private
  user data.

## Reconciliation

The inventory accepted the exact gap: Talos had local modeled socket delivery
but no accepted source contract from a smoltcp/device-side live TCP listener
into descriptor-facing accepted connections. The selected local-core task
closed the local/static portion by adding an explicit report for the
host-only smoltcp descriptor bridge.

That accepted report is deliberately not a live-network claim. It accepts only
an Established host-only smoltcp handshake with an accepted Talos descriptor
attachment and metadata-only payload transfer evidence. When the caller
requires device/interface binding, the same source path reports
BlockedNoDeviceInterfaceBinding and keeps device_interface_bound=false. The
public readiness labels remain live_packet_io_accepted=false,
live_reachability_accepted=false, remote_receipt_accepted=false,
compatibility_accepted=false, and ssh_ready=false.

The first missing fact after closeout is now narrower than the inventory:
there is still no selected device/interface ownership model that binds a real
network device or driver packet adapter to a smoltcp TCP listener and then
delivers accepted live connections into NetworkSocketDescriptorTable. The
retained Phase 12.1 Ethernet path remains paused with
selected_discriminator=null, and this closeout does not choose a new Ethernet,
hardware, OpenSSH, or local/static successor.

## Findings

- fixed: accepted local/static behavior is reconciled with the selected source
  boundary in src/network.rs and src/userspace_socket_abi.rs.
- fixed: the accepted local boundary remains distinct from unaccepted live TCP,
  packet I/O, remote receipt, OpenSSH compatibility, Pi 5 hardware proof, and
  ssh-ready=true.
- fixed: the next missing fact is named precisely as the absent selected
  device/interface ownership model for live TCP listener delivery into Talos
  descriptors.
- blocked: live/generated-root/OpenSSH retry, live TCP attempt, packet I/O,
  remote receipt, compatibility, hardware proof, runtime russh adoption, and
  ssh-ready=true remain blocked pending supervisor planning and later explicit
  live/hardware preconditions.
- deferred: Phase 12.1 Ethernet selected-discriminator work remains paused by
  selected_discriminator=null; future source evidence may select a bounded
  return task, but this closeout does not create or promote one.
- not-an-issue: no Rust source change was required for this closeout.
- removed: no source, docs, helper, task, or evidence artifact was removed.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Accepted local/static behavior is distinguished from live TCP, packet I/O,
  remote receipt, OpenSSH compatibility, Pi 5 hardware proof, and
  ssh-ready=true: satisfied.
- Closeout either selects a mechanically objective next task or sets
  planningNeeded/blocker fields with the first missing fact: satisfied with
  planningNeeded=true.
- No live/generated-root/OpenSSH retry is selected without candidate identity,
  fresh serial cursor, TFTP delta, known-good control, and candidate rerun
  requirements: satisfied.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-local-to-live-tcp-gap-closeout/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-local-to-live-tcp-gap-closeout/evidence-map.json.
- Accepted gap inventory:
  tasks/2026-06-29-phase12-ssh-local-to-live-tcp-gap-inventory.md.
- Accepted selected local core:
  tasks/2026-06-29-phase12-ssh-selected-live-tcp-local-core.md.
- Accepted predecessor closeout:
  tasks/2026-06-29-phase12-ssh-local-substrate-closeout.md.
- Live no-tcp-connect checkpoint:
  tasks/2026-06-24-phase12-ssh-no-tcp-connect-live-network-substrate-checkpoint.md.
- Ethernet selected-discriminator pause:
  tasks/2026-06-29-phase12-rp1-ethernet-link-not-ready-discriminator-reselection.md.

## Redaction Review

Durable evidence records task ids, source paths, public classifier names,
boolean readiness boundaries, validation commands, and metadata-only counts or
labels from predecessor evidence. It does not retain peer identifiers,
addresses, packet payloads, key material, session material, boot artifact
bytes, hardware data, or private user data.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static review of accepted inventory/local-core task records and current
  supervisor state: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- focused cargo tests: not run; no Rust source or expected diagnostics touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: The local/static source boundary is accepted, but no later
queued task has complete mechanically checkable dependencies for selecting the
device/interface ownership model that would bind live packet I/O to a smoltcp
TCP listener and Talos descriptor delivery. Supervisor planning is required
before any further Phase 12 live TCP, Ethernet, hardware, OpenSSH, or
local/static successor work.

No hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, live TCP attempt, packet I/O, remote receipt
claim, compatibility claim, ssh-ready=true claim, fake command expansion,
runtime russh adoption, or phase transition is accepted.
