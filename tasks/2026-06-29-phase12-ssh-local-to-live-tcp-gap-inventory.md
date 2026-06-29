# Phase 12 SSH Local-To-Live TCP Gap Inventory

Task id: phase12-ssh-local-to-live-tcp-gap-inventory-20260629

Status: accepted after commit.

Classification: ssh-local-to-live-tcp-gap-inventory-selected-local-core.

Evidence level: static source/docs/task review, task-owned JSON evidence, docs
build, and diff checks. No Rust source change, Pi 5 hardware/lab action,
hardwareTestLock acquisition, boot publication, generated-root retry, OpenSSH
retry, packet I/O, live TCP connection attempt, remote receipt claim,
compatibility claim, ssh-ready=true claim, fake command expansion, broad audit,
or phase transition was performed.

## Goal

Identify the smallest source/evidence boundary between accepted local modeled
SSH socket delivery and live TCP connection establishment, without unpausing
Ethernet hardware or retrying OpenSSH.

## Scope Performed

- Reviewed the accepted local SSH substrate closeout, accepted local socket
  delivery closeout, no-tcp-connect live network checkpoint, Ethernet
  selected_discriminator=null reselection, Phase 12 docs, roadmap, and current
  supervisor state.
- Inspected the source owners named by the task: src/network.rs,
  src/userspace_socket_abi.rs, src/ssh_service_readiness.rs, and
  src/rp1_ethernet.rs.
- Recorded the first missing source/evidence fact and selected one existing
  queued local/static successor.

## Non-goals Preserved

- No Rust implementation, live packet I/O, live TCP attempt, generated-root
  retry, OpenSSH retry, Pi 5 hardware/lab action, hardware lock acquisition,
  boot publication, remote receipt claim, compatibility claim, ssh-ready=true
  claim, fake command expansion, broad shell grammar expansion, broad repo
  audit, runtime russh dependency adoption, or phase transition.
- No durable retention of peer identifiers, addresses, packet payloads, key
  material, session material, boot artifact bytes, hardware data, or private
  user data.

## Reviewed Inputs

- memory/talos-supervisor-state.json current task, task queue, hardware lock,
  and intervention fields.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/network.rs.
- src/userspace_socket_abi.rs.
- src/ssh_service_readiness.rs.
- src/rp1_ethernet.rs.
- tasks/2026-06-29-phase12-ssh-local-substrate-closeout.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-closeout.md.
- tasks/2026-06-24-phase12-ssh-no-tcp-connect-live-network-substrate-checkpoint.md.
- tasks/2026-06-29-phase12-rp1-ethernet-link-not-ready-discriminator-reselection.md.

## Inventory

The accepted local SSH substrate is not empty: local modeled transport can drive
the SSH service pipeline through descriptor-backed stream sockets. The accepted
socket delivery path uses an in-memory listener and a synthetic local peer,
then dispatches bounded input/output through the previously accepted SSH
auth/session/channel/shell surfaces. Its public counters stay fail-closed for
live reachability, remote receipt, compatibility, and ssh-ready.

The accepted no-tcp-connect checkpoint proves that one bounded OpenSSH attempt
reached no TCP connection after selected boot service. That is below SSH
protocol handling: another same-shaped OpenSSH retry would not explain whether
Talos has a live listener, TCP stack ingress, packet driver path, or physical
Ethernet link capable of connection establishment.

The source gap is exact:

- src/network.rs NetworkSocketDescriptorTable::connect and accept are
  process-local synthetic handshakes. Listener lookup only matches an existing
  in-memory Listening socket endpoint and creates synthetic 127.0.0.1 peer
  state. No external SYN, smoltcp TCP socket, IP routing, ARP/neighbor path, or
  driver-backed frame ingress can create an accepted descriptor.
- src/network.rs SmoltcpDependencyCore::poll_without_device returns
  NoDeviceBound, and the smoltcp packet/device adapters are host-testable
  queue boundaries. They do not accept live TCP listener state, an interface
  poll loop, descriptor delivery, RP1 driver ownership, or hardware packet I/O.
- src/userspace_socket_abi.rs exposes the experimental socket syscalls over
  descriptor dispatch, but the retained acceptance is local/substitute. It
  does not convert incoming network packets into process descriptors or bind a
  socket table to a live network interface.
- src/ssh_service_readiness.rs builds local SSH listener transport by creating
  the same synthetic socket table and explicitly reports
  live_reachability=false, remote_receipt=false, compatibility=false, and
  ssh_ready=false for socket delivery and peer-output surfaces.
- src/rp1_ethernet.rs retains source contracts and discriminators for the RP1
  / BCM54213PE hardware path, but the accepted current Ethernet reselection has
  selected_discriminator=null and does not unblock live packet I/O or a
  hardware TCP reachability task.

## First Missing Fact

First missing source/evidence fact: there is no accepted owner-file invariant
that maps a live TCP listener path into Talos descriptors. More concretely,
Talos lacks an accepted source contract that binds all of these pieces:

1. a smoltcp-backed TCP listener on port 22 with an explicit device/interface
   boundary,
2. an ingress/egress pump between that listener and a NetworkDevice or driver
   packet adapter,
3. delivery of an accepted TCP connection into NetworkSocketDescriptorTable or
   an explicitly rejected narrower bridge,
4. fail-closed readiness labels proving that local modeled delivery remains
   distinct from live reachability, remote receipt, OpenSSH compatibility, and
   ssh-ready=true until a later live task accepts them.

Without that source contract/core, live TCP connection establishment is not an
objective worker task. With it, the worker can make local/static progress
without unpausing Ethernet hardware or retrying OpenSSH.

## Selected Local Core Invariant

selected_next_task: phase12-ssh-selected-live-tcp-local-core-20260629.

The selected task must stay local/static and use the owner files named by this
inventory. Its invariant is:

- Implement or formally block the smallest source boundary that can distinguish
  descriptor-local socket handshakes from a future live TCP listener bridge.
- If implemented, the boundary must have a deterministic source/unit path that
  proves whether a smoltcp/device-side TCP listener can create or feed a Talos
  descriptor-facing accepted connection without claiming live packet I/O.
- If blocked, the first failing owner-file fact must be recorded precisely,
  for example no selected interface/device binding, no safe descriptor bridge,
  no accepted TCP listener ownership model, or no fail-closed readiness counter
  shape.
- Public readiness must remain fail-closed for live reachability, remote
  receipt, compatibility, and ssh-ready unless a later explicit live task
  accepts those claims.

Expected terminal classifications for the selected local core are:

- ssh-live-tcp-local-core-accepted-source-boundary
- ssh-live-tcp-local-core-blocked-no-device-interface-binding
- ssh-live-tcp-local-core-blocked-no-descriptor-bridge
- ssh-live-tcp-local-core-blocked-readiness-boundary-ambiguous
- ssh-live-tcp-local-core-planning-needed

Required validation gates for the selected local core:

- git status --short --branch before edits.
- cargo fmt --all -- --check if Rust source is touched.
- cargo -Zjson-target-spec test --quiet with focused filters over network,
  userspace_socket_abi, and ssh_service_readiness when Rust source or expected
  diagnostics are touched.
- Existing local socket/SSH readiness regression tests if selected owner files
  affect those surfaces.
- git diff --check.
- /home/node/.cargo/bin/mdbook build if docs/src files or ADR index are
  touched.
- jq empty on any task-owned JSON evidence added or modified.
- git diff --cached --check before commit.

## Findings

- fixed: distinguished accepted local modeled SSH socket delivery from
  unaccepted live TCP connection establishment, remote receipt, OpenSSH
  compatibility, Pi 5 hardware proof, and ssh-ready=true.
- fixed: identified the first missing source/evidence fact as the absent
  live-TCP-listener-to-descriptor source contract across src/network.rs,
  src/userspace_socket_abi.rs, src/ssh_service_readiness.rs, and the retained
  RP1 Ethernet selected_discriminator=null boundary.
- fixed: selected exactly one existing queued local/static successor:
  phase12-ssh-selected-live-tcp-local-core-20260629.
- blocked: live/generated-root/OpenSSH retry, live TCP connection attempt,
  hardware proof, remote receipt, compatibility, and ssh-ready=true remain
  blocked until a later explicitly gated task accepts them.
- deferred: Ethernet hardware discriminator work remains dependency-blocked by
  selected_discriminator=null; any future live task still needs candidate
  identity, fresh serial cursor, TFTP delta, known-good control, candidate
  rerun requirements, hardware lock ownership, and post-hardware review.
- not-an-issue: no Rust source change was required for this inventory.
- removed: no source, task, helper, docs, or evidence artifact was removed.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Accepted local modeled socket delivery is distinguished from live TCP,
  remote receipt, compatibility, Pi 5 hardware proof, and ssh-ready=true:
  satisfied.
- First missing source/evidence fact is named with exact owner files and task
  records: satisfied.
- selected_next_task is exactly one concrete queued task id with mechanically
  checkable dependencies: satisfied.
- No live/generated-root/OpenSSH retry is selected without the required future
  hardware/live preconditions: satisfied.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-local-to-live-tcp-gap-inventory/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-local-to-live-tcp-gap-inventory/evidence-map.json.
- Accepted local SSH closeout:
  tasks/2026-06-29-phase12-ssh-local-substrate-closeout.md.
- Accepted local socket delivery closeout:
  tasks/2026-06-23-phase12-ssh-live-socket-delivery-closeout.md.
- Accepted no-tcp-connect checkpoint:
  tasks/2026-06-24-phase12-ssh-no-tcp-connect-live-network-substrate-checkpoint.md.
- Accepted Ethernet pause:
  tasks/2026-06-29-phase12-rp1-ethernet-link-not-ready-discriminator-reselection.md.
- Source reviewed:
  src/network.rs, src/userspace_socket_abi.rs, src/ssh_service_readiness.rs,
  and src/rp1_ethernet.rs.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static review of accepted task records, Phase 12 docs, roadmap, supervisor
  state, and named source owners: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- focused cargo tests: not run; no Rust source or expected diagnostics touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

Accepted as ssh-local-to-live-tcp-gap-inventory-selected-local-core.

selected_next_task: phase12-ssh-selected-live-tcp-local-core-20260629.

planningNeeded: false.

No hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, live TCP attempt, remote receipt claim,
compatibility claim, ssh-ready=true claim, fake command expansion, or phase
transition is accepted.
