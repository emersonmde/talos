# Phase 12 SSH Live TCP Device/Interface Frontier Closeout

Task id: phase12-ssh-live-tcp-device-interface-frontier-closeout-20260629

Status: accepted after commit.

Classification: ssh-live-tcp-device-interface-frontier-closeout-planning-needed.

Evidence level: static task/source/docs/state review, task-owned JSON
evidence, docs build, and diff checks. No Rust source change, Pi 5
hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root retry, OpenSSH retry, packet I/O, live TCP connection attempt,
remote receipt claim, compatibility claim, ssh-ready=true claim, service
success claim, runtime russh adoption, fake command expansion, or phase
transition was performed.

## Goal

Reconcile the accepted local/static device/interface ownership, descriptor
delivery, and readiness-label chain before any live TCP, OpenSSH, generated
root, or Pi 5 hardware proof is considered.

## Scope Performed

- Reviewed the accepted ownership contract, device/interface local core,
  listener descriptor-accept local core, readiness-label local core, current
  supervisor state, source-visible labels, Phase 12 docs, and roadmap.
- Confirmed the accepted frontier is local/static and metadata-only:
  src/network.rs carries the selected ownership model and local descriptor
  delivery reports, while src/ssh_service_readiness.rs carries separate
  local/static descriptor prerequisite labels.
- Confirmed live reachability, packet I/O, remote receipt, OpenSSH
  compatibility, hardware proof, service success, and ssh-ready=true remain
  false or unaccepted.
- Set planningNeeded because no later queued task is mechanically objective for
  the next live/hardware proof step.

## Non-goals Preserved

- No Pi 5 hardware/lab action, hardware lock acquisition, boot publication,
  generated-root retry, OpenSSH retry, packet I/O, live TCP attempt, remote
  receipt claim, compatibility claim, ssh-ready=true claim, service success
  claim, runtime russh adoption, fake command expansion, or phase transition.
- No selection of a live/generated-root/OpenSSH retry. Any future live proof
  still requires candidate identity, fresh serial cursor, TFTP delta,
  known-good control, candidate rerun, hardware lock ownership, evidence
  redaction, and restore requirements.
- No durable retention of peer identifiers, addresses, packet payload contents,
  key material, session material, boot artifact bytes, hardware data, private
  user data, or stable secret-derived identifiers.

## Reconciliation

The accepted local/static frontier now has three parts:

- The selected ownership model is
  network-owned-smoltcp-interface-with-driver-packet-adapter-ingress-and-descriptor-table-delivery.
  src/network.rs owns the future smoltcp interface/listener poll boundary,
  DriverPacketAdapter/NetworkDevice frame movement, accepted
  NetworkSocketDescriptorTable delivery, metadata-only reports, and fail-closed
  labels.
- LiveTcpListenerDescriptorAcceptReport reports
  AcceptedLocalDescriptorDelivery only for the local/static host-only smoltcp
  bridge when the listener accepted a Talos descriptor and that descriptor is in
  the Accepted socket state for the same connection id.
- SshLiveTcpDescriptorPrerequisiteReport reports
  LocalStaticDescriptorPrerequisiteAccepted only when the local source boundary
  and descriptor-facing accepted-connection delivery are both accepted. Its
  labels distinguish local-source/descriptor-delivered from
  live-reachability-unaccepted.

That chain removes the local/source uncertainty from the previous gap, but it
does not become a live-networking claim. The accepted reports still keep
live_packet_io_accepted=false, live_reachability_accepted=false,
remote_receipt_accepted=false, compatibility_accepted=false,
hardware_proof_accepted=false, service_success_accepted=false, and
ssh_ready=false.

The first missing fact is now outside local/static implementation evidence:
there is no explicit queued live/hardware proof task with complete
mechanically checkable dependencies and gates for candidate identity, fresh
serial cursor, TFTP delta, known-good control, candidate rerun, hardware lock
ownership, evidence redaction, and restore requirements. This closeout
therefore selects no next task and requests supervisor planning.

## Findings

- fixed: accepted local/static device/interface ownership metadata is
  reconciled with descriptor delivery and readiness labels.
- fixed: source-visible reports keep accepted local/static descriptor
  prerequisite labels separate from live reachability.
- fixed: docs now record that the local/static frontier is closed before
  hardware/live/OpenSSH work resumes.
- blocked: live/generated-root/OpenSSH retry, live TCP attempt, packet I/O,
  remote receipt, OpenSSH compatibility, Pi 5 hardware proof, service success,
  runtime russh adoption, ssh-ready=true, fake command expansion, and phase
  transition remain blocked pending supervisor planning and later explicit
  hardware/live proof gates.
- deferred: the future RP1 Ethernet frame-provider path remains deferred to an
  explicit hardware/live task; this closeout does not select a discriminator or
  publish a boot artifact.
- not-an-issue: no Rust source change was required because this task reconciles
  accepted task/source/docs evidence.
- removed: no source, docs, helper, task, or evidence artifact was removed.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Accepted local/static device/interface ownership, descriptor delivery, and
  readiness labels are distinguished from unaccepted live packet I/O, remote
  receipt, OpenSSH compatibility, Pi 5 hardware proof, and ssh-ready=true:
  satisfied.
- Closeout either selects one mechanically objective next task or sets
  planningNeeded/blocker fields with the first missing fact: satisfied with
  planningNeeded=true and selected_next_task=null.
- No live/generated-root/OpenSSH retry is selected without candidate identity,
  fresh serial cursor, TFTP delta, known-good control, candidate rerun,
  hardware lock ownership, evidence redaction, and restore requirements:
  satisfied.
- Accepted closeout is committed before any follow-up starts: satisfied after
  this task commit.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-device-interface-frontier-closeout/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-device-interface-frontier-closeout/evidence-map.json.
- Accepted ownership contract:
  tasks/2026-06-29-phase12-ssh-live-tcp-device-interface-ownership-contract.md.
- Accepted device/interface local core:
  tasks/2026-06-29-phase12-ssh-live-tcp-device-interface-local-core.md.
- Accepted listener descriptor delivery local core:
  tasks/2026-06-29-phase12-ssh-live-tcp-listener-descriptor-accept-local-core.md.
- Accepted readiness-label local core:
  tasks/2026-06-29-phase12-ssh-live-tcp-readiness-label-local-core.md.
- Source evidence:
  src/network.rs and src/ssh_service_readiness.rs.
- Docs changed:
  docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Redaction Review

Durable evidence records task ids, source paths, public classifier names,
validation commands/results, metadata-only enum labels, and boolean readiness
boundaries. It does not retain peer identifiers, addresses, packet payload
contents, key material, session material, boot artifact bytes, hardware data,
private user data, or stable secret-derived identifiers.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static review of accepted task records, docs, source-visible labels, and
  current supervisor state: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- focused cargo tests: not run; no Rust source or expected diagnostics touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: The local/static live TCP device/interface frontier is closed,
but no later queued task has complete mechanically checkable dependencies for a
live/hardware proof with candidate identity, fresh serial cursor, TFTP delta,
known-good control, candidate rerun, hardware lock ownership, evidence
redaction, and restore requirements. Supervisor planning is required before any
further Phase 12 live TCP, hardware, generated-root, OpenSSH, or compatibility
work.

No hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, live TCP attempt, packet I/O, remote receipt
claim, compatibility claim, ssh-ready=true claim, service success claim,
runtime russh adoption, fake command expansion, or phase transition is
accepted.
