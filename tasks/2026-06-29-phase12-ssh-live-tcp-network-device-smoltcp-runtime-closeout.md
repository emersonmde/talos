# Phase 12 SSH Live TCP Network-Device Smoltcp Runtime Closeout

Task id: phase12-ssh-live-tcp-network-device-smoltcp-runtime-closeout-20260629

Status: accepted after commit.

Classification: ssh-live-tcp-network-device-smoltcp-runtime-closeout-accepted.

Evidence level: static task/source/docs/state review, task-owned JSON
evidence, docs build, and diff checks. No Rust source change, Pi 5
hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root retry, OpenSSH retry, external live TCP attempt, remote receipt
claim, compatibility claim, hardware proof, ssh-ready=true claim, service
success claim, runtime russh adoption, fake command expansion, broad shell
work, or phase transition was performed.

## Goal

Reconcile the accepted deterministic network-device smoltcp runtime binding
before any Pi 5 hardware/live proof contract is considered.

## Scope Performed

- Reviewed the accepted runtime-core task record, changed src/network.rs
  symbols, focused test names, Phase 12 docs, roadmap, lab-controller hardware
  evidence contract, and current supervisor state.
- Confirmed the accepted runtime binding is local deterministic evidence only:
  DriverPacketAdapter implements smoltcp::phy::Device, and
  LiveTcpNetworkDeviceRuntimeReport connects a deterministic
  DriverPacketAdapter-backed smoltcp listener exchange to the already accepted
  descriptor-facing delivery witness.
- Confirmed the runtime-core fail-closed states still keep live packet I/O,
  live reachability, remote receipt, OpenSSH compatibility, hardware proof,
  service success, and ssh_ready false.
- Selected the already queued proof-contract task as the next mechanically
  objective step because it is contract-only and requires candidate identity,
  fresh serial cursor, TFTP delta, known-good control, candidate rerun,
  hardware lock ownership, evidence redaction, and restore requirements before
  any later hardware action.

## Non-goals Preserved

- No Pi 5 hardware/lab action, hardware lock acquisition, boot publication,
  generated-root retry, OpenSSH retry, external live TCP attempt, remote
  receipt claim, compatibility claim, ssh-ready=true claim, service success
  claim, runtime russh adoption, fake command expansion, broad shell work, or
  phase transition.
- No direct selection of a hardware run. The selected successor is the
  proof-contract task only; later Pi 5 preflight/discriminator work remains
  dependency-gated by that accepted contract and hardwareTestLock ownership.
- No durable retention of peer identifiers, addresses, packet payload contents,
  key material, session material, boot artifact bytes, hardware data, private
  user data, or stable secret-derived identifiers.

## Reconciliation

The accepted runtime frontier now has four bounded facts:

- DriverPacketAdapter is a smoltcp::phy::Device source boundary owned by
  src/network.rs.
- The deterministic DriverPacketAdapter-backed smoltcp listener/client exchange
  can move local frames through the selected network-device model.
- NetworkSocketDescriptorTable::live_tcp_network_device_smoltcp_runtime_binding
  reports accepted deterministic device-interface delivery only after the
  descriptor-facing accepted-connection witness is present.
- Missing descriptor delivery, missing deterministic device-interface binding,
  and missing hardware frame provider remain fail-closed.

That closes the local deterministic runtime-binding gap, but it does not become
a Pi 5 or external reachability claim. The accepted reports still keep live
packet I/O, live reachability, remote receipt, OpenSSH compatibility, hardware
proof, service success, and ssh_ready false.

The next mechanically objective task is the proof contract:
phase12-ssh-live-tcp-pi5-proof-contract-20260629. That task must define the
candidate identity, capture, redaction, restore, and hardwareTestLock contract
before any candidate preflight or packet-I/O discriminator can run.

## Findings

- fixed: reconciled the accepted runtime-core boundary with source-visible
  DriverPacketAdapter, LiveTcpNetworkDeviceRuntimeReport, and descriptor-facing
  delivery evidence.
- fixed: confirmed the accepted runtime binding remains local deterministic
  metadata evidence and does not accept Pi 5 packet I/O or external live TCP
  reachability.
- fixed: selected the queued proof-contract task as the next mechanically
  objective step because its acceptance gates require candidate identity, fresh
  serial cursor, TFTP delta, known-good control, candidate rerun,
  hardwareTestLock ownership, redaction, and restore policy before hardware.
- blocked: Pi 5 candidate preflight, packet-I/O discriminator, OpenSSH retry,
  generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, runtime russh adoption, fake command expansion, broad shell
  work, and phase transition remain blocked behind later explicit tasks.
- deferred: live RP1 Ethernet frame-provider binding and real packet I/O remain
  deferred until the proof contract and any required preflight accept them.
- not-an-issue: no Rust source change or focused test rerun was required for
  this closeout because no source or expected diagnostics changed.
- removed: no source, docs, helper, task, or evidence artifact was removed.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Closeout states exactly what the deterministic runtime binding accepted and
  what remains unaccepted for Pi 5 live packet I/O and SSH reachability:
  satisfied.
- Closeout selects phase12-ssh-live-tcp-pi5-proof-contract-20260629 only after
  confirming the successor requires candidate identity, fresh serial cursor,
  TFTP delta, known-good control, candidate rerun, hardware lock ownership,
  evidence redaction, and restore requirements: satisfied.
- Accepted closeout is committed before any hardware-proof contract or hardware
  task starts: satisfied after this task commit.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-network-device-smoltcp-runtime-closeout/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-network-device-smoltcp-runtime-closeout/evidence-map.json.
- Accepted runtime-core task:
  tasks/2026-06-29-phase12-ssh-live-tcp-network-device-smoltcp-runtime-core.md.
- Source reviewed:
  src/network.rs and src/ssh_service_readiness.rs.
- Hardware evidence contract reviewed:
  docs/src/project/lab-controller.md.
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
- static review of accepted task records, changed source, focused test output,
  Phase 12 docs, roadmap, lab-controller contract, and current supervisor
  state: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- focused cargo tests: not run; no Rust source or expected diagnostics touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: phase12-ssh-live-tcp-pi5-proof-contract-20260629.

planningNeeded: false.

No hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, external live TCP attempt, packet I/O, remote
receipt claim, compatibility claim, ssh-ready=true claim, service success
claim, runtime russh adoption, fake command expansion, broad shell work, or
phase transition is accepted.

Commit: recorded in talos-supervisor-state.json after final commit.
