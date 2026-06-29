# Phase 12 SSH Live TCP Readiness Label Local Core

Task id: phase12-ssh-live-tcp-readiness-label-local-core-20260629

Status: accepted after commit.

Classification: ssh-live-tcp-readiness-label-local-core-accepted.

Evidence level: Rust source implementation, focused no_std unit tests,
task-owned JSON evidence, docs build, and diff checks. No Pi 5 hardware/lab
action, hardwareTestLock acquisition, boot publication, generated-root retry,
OpenSSH retry, packet I/O, live TCP connection attempt, remote receipt claim,
compatibility claim, ssh-ready=true claim, service success claim, fake command
expansion, runtime russh adoption, or phase transition was performed.

## Goal

Compose the accepted local/static listener-to-descriptor boundary into
readiness diagnostic labels while keeping live reachability and ssh-ready false.

## Scope Performed

- Added sshservicediag labels for the live TCP descriptor prerequisite:
  local source accepted, descriptor delivered, live reachability unaccepted,
  missing local source, missing descriptor delivery, and missing real
  device/interface binding.
- Added SshLiveTcpDescriptorPrerequisiteReport and
  classify_ssh_live_tcp_descriptor_prerequisite to
  src/ssh_service_readiness.rs.
- Kept LocalStaticDescriptorPrerequisiteAccepted limited to an accepted
  LiveTcpListenerDescriptorAcceptReport where local source boundary and
  descriptor-facing accepted-connection delivery are both accepted.
- Kept fail-closed reports for no accepted local source boundary, no descriptor
  delivery, and required real device/interface binding.
- Added focused tests for accepted local/static prerequisite labels and the
  fail-closed real-device binding path.
- Updated Phase 12 docs and roadmap with the accepted readiness-label boundary
  and selected next closeout task.

## Non-goals Preserved

- No Pi 5 hardware/lab action, hardware lock acquisition, boot publication,
  generated-root retry, OpenSSH retry, packet I/O, live TCP attempt, remote
  receipt claim, compatibility claim, ssh-ready=true claim, service success
  claim, fake command expansion, runtime russh adoption, or phase transition.
- No new host-key, authentication, session, channel, shell, packet crypto, or
  remote compatibility behavior.
- No durable retention of peer identifiers, addresses, packet payload contents,
  key/session material, boot artifacts, hardware data, private user data, or
  stable secret-derived identifiers.

## Findings

- fixed: src/ssh_service_readiness.rs now exposes metadata-only readiness labels
  for accepted local/static live TCP descriptor prerequisites.
- fixed: the accepted report distinguishes local source/descriptor delivery from
  live_reachability_accepted=false.
- fixed: focused tests cover the accepted label composition and fail-closed
  required-real-device binding path.
- deferred: live packet I/O, remote receipt, OpenSSH compatibility, Pi 5
  hardware proof, ssh-ready=true, service success, generated-root retry,
  runtime russh adoption, and phase transition remain outside this task.
- not-an-issue: sshservicediag default command remains fail-closed because no
  live/network context is available in the default diagnostic dispatch path.
- removed: no source, docs, helper, task, or evidence artifact was removed.

## Accepted Local/Static Labels

- result: LocalStaticDescriptorPrerequisiteAccepted.
- labels:
  - sshservicediag-live-tcp-descriptor-prerequisite-local-source-accepted.
  - sshservicediag-live-tcp-descriptor-prerequisite-descriptor-delivered.
  - sshservicediag-live-tcp-descriptor-prerequisite-live-reachability-unaccepted.
- rejected live/service claims: live_packet_io_accepted=false,
  live_reachability_accepted=false, remote_receipt_accepted=false,
  compatibility_accepted=false, hardware_proof_accepted=false,
  service_success_accepted=false, and ssh_ready=false.

## Selected Next Task

selected_next_task:
phase12-ssh-live-tcp-device-interface-frontier-closeout-20260629.

That successor remains mechanically objective because this task completed the
planned local/static readiness-label composition. The closeout can now reconcile
the accepted ownership, descriptor delivery, and readiness-label evidence before
any hardware/live/OpenSSH successor is selected.

## Evidence Map

- Source changed:
  - src/ssh_service_readiness.rs.
- Docs changed:
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/roadmap.md.
- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-readiness-label-local-core/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-readiness-label-local-core/evidence-map.json.
- Predecessor descriptor accept:
  tasks/2026-06-29-phase12-ssh-live-tcp-listener-descriptor-accept-local-core.md.

## Redaction Review

The code and tests use synthetic local descriptor state, enum labels, boolean
readiness boundaries, and metadata-only report state. Durable evidence records
task ids, source paths, public classifier names, validation commands, and
metadata-only state labels. It does not retain peer identifiers, addresses,
packet payload contents, key material, session material, boot artifact bytes,
hardware data, private user data, or stable secret-derived identifiers.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet live_tcp_descriptor_prerequisite: pass.
- cargo -Zjson-target-spec test --quiet ssh_service_readiness: pass; the
  repo's no_std harness executed the focused ssh_service_readiness coverage.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; retained large search index
  warning.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

Accepted as ssh-live-tcp-readiness-label-local-core-accepted.

Commit hash is recorded in durable supervisor state after commit.

selected_next_task:
phase12-ssh-live-tcp-device-interface-frontier-closeout-20260629.

No hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, live TCP attempt, packet I/O, remote receipt
claim, compatibility claim, ssh-ready=true claim, service success claim, fake
command expansion, runtime russh adoption, or phase transition is accepted.
