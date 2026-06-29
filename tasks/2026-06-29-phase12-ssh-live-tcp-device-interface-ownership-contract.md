# Phase 12 SSH Live TCP Device/Interface Ownership Contract

Task id: phase12-ssh-live-tcp-device-interface-ownership-contract-20260629

Status: accepted after commit.

Classification: ssh-live-tcp-device-interface-ownership-contract-selected.

Evidence level: static source/task/docs/state review, task-owned JSON evidence,
docs build, and diff checks. No Rust source change, Pi 5 hardware/lab action,
hardwareTestLock acquisition, boot publication, generated-root retry, OpenSSH
retry, packet I/O, live TCP connection attempt, remote receipt claim,
compatibility claim, ssh-ready=true claim, fake command expansion, runtime
russh adoption, or phase transition was performed.

## Goal

Select the source-backed ownership contract that will let a later local/static
task represent how a real device/interface binding feeds a smoltcp TCP
listener and Talos descriptor delivery, without accepting live networking.

## Scope Performed

- Reviewed the accepted local-to-live TCP gap inventory, selected local-core
  boundary, closeout, current supervisor state, Phase 12 docs, roadmap, and
  source owners in src/network.rs, src/userspace_socket_abi.rs,
  src/ssh_service_readiness.rs, and src/rp1_ethernet.rs.
- Classified candidate ownership models and selected exactly one source/local
  contract.
- Kept all live TCP, packet I/O, hardware, OpenSSH, compatibility,
  ssh-ready=true, runtime russh adoption, fake command expansion, and phase
  transition claims rejected.

## Non-goals Preserved

- No Pi 5 hardware/lab action, hardware lock acquisition, boot publication,
  generated-root retry, OpenSSH retry, packet I/O, live TCP attempt, remote
  receipt claim, compatibility claim, ssh-ready=true claim, fake command
  expansion, runtime russh adoption, or phase transition.
- No Rust source change; this task selects the contract for the next local
  source task rather than implementing it.
- No durable retention of peer identifiers, addresses, packet payloads, key
  material, session material, boot artifact bytes, hardware data, or private
  user data.

## Candidate Dispositions

- selected:
  network-owned-smoltcp-interface-with-driver-packet-adapter-ingress-and-descriptor-table-delivery.
  src/network.rs owns the smoltcp interface/listener poll boundary,
  DriverPacketAdapter/NetworkDevice frame ingress-egress boundary,
  NetworkSocketDescriptorTable accepted descriptor delivery, and the
  fail-closed readiness report fields. A later explicit hardware task may bind
  an RP1 Ethernet driver as a NetworkDevice-style frame provider, but the
  driver does not own sockets, descriptors, SSH service state, or readiness
  labels.
- rejected: RP1-driver-owned-smoltcp-listener-and-descriptor-delivery. This
  would mix hardware bring-up, MAC/PHY lifecycle, protocol polling, socket
  state, and descriptor ownership in src/rp1_ethernet.rs, contradicting the
  accepted separation where src/network.rs owns protocol/socket boundaries and
  the Ethernet lane remains paused with selected_discriminator=null.
- rejected: userspace-syscall-dispatch-owned-device-binding.
  src/userspace_socket_abi.rs owns experimental syscall dispatch and ABI tests;
  making it own device polling or interface lifetime would couple syscall
  decoding to driver/protocol scheduling and weaken the fail-closed boundary.
- rejected: SSH-service-owned-live-listener-binding. src/ssh_service_readiness.rs
  owns local modeled SSH readiness labels and service diagnostics, not packet
  ingress, smoltcp interface polling, NetworkDevice frame movement, or socket
  descriptor allocation.
- deferred: runtime-russh-or-OpenSSH-owned-socket-binding. Runtime SSH server
  adoption remains a later strategy question after Talos has accepted live TCP
  descriptor delivery; it cannot be the device/interface ownership contract.
- blocked: no-device-host-only-continuation. The previous local-core task
  already accepted the host-only smoltcp descriptor bridge and reports
  BlockedNoDeviceInterfaceBinding when live binding is required; repeating that
  model would not close the selected missing fact.

## Selected Contract

selected_device_interface_model:
network-owned-smoltcp-interface-with-driver-packet-adapter-ingress-and-descriptor-table-delivery.

Source owners:

- src/network.rs owns the selected contract, including future local/static
  representation of a bound device/interface, the smoltcp listener poll
  boundary, DriverPacketAdapter and NetworkDevice frame movement, accepted
  NetworkSocketDescriptorTable delivery, metadata-only reports, and fail-closed
  labels.
- src/rp1_ethernet.rs remains the later hardware frame-provider owner only
  after an explicit live/hardware task selects and proves that path. It must
  not own descriptors, SSH service readiness, or socket ABI dispatch.
- src/userspace_socket_abi.rs remains syscall dispatch only and may assert the
  source-visible contract through focused local tests.
- src/ssh_service_readiness.rs remains readiness composition only and must keep
  live reachability, remote receipt, compatibility, hardware proof, and
  ssh-ready false until later explicit evidence accepts them.

Invariant for the next local/static implementation task:

1. A source-visible device/interface-bound path may be represented only inside
   src/network.rs from an explicit NetworkDevice/DriverPacketAdapter boundary
   into a smoltcp listener poll boundary.
2. Accepted descriptor delivery still requires an accepted
   NetworkSocketDescriptorTable descriptor attachment and metadata-only
   delivery report.
3. The local/static report may set selected ownership metadata and remove the
   no-contract gap, but it must keep live_packet_io_accepted=false,
   live_reachability_accepted=false, remote_receipt_accepted=false,
   compatibility_accepted=false, and ssh_ready=false until a later explicit
   live/hardware task accepts those claims.
4. Durable evidence may retain source paths, task ids, boolean labels,
   classification labels, validation commands/results, and metadata-only
   counters. It must not retain peer identifiers, addresses, packet payloads,
   key/session material, boot artifact bytes, hardware data, or stable
   secret-derived identifiers.

Implementation scope selected for the next task:

- Represent the selected ownership model in source with explicit fail-closed
  states for missing device/interface binding.
- Add focused local/static tests selected by the touched files.
- Do not run hardware, live TCP, packet I/O, OpenSSH, generated-root retry, or
  boot publication.

## Findings

- fixed: the first missing fact from the closeout is resolved into one selected
  source/local ownership model.
- fixed: source ownership is assigned to src/network.rs for protocol/socket,
  device adapter, descriptor table, report, and fail-closed label boundaries.
- fixed: src/rp1_ethernet.rs is limited to a future hardware frame-provider
  role and remains paused for this task.
- fixed: src/userspace_socket_abi.rs and src/ssh_service_readiness.rs keep
  syscall-dispatch and readiness-composition ownership respectively.
- rejected: RP1-driver-owned, syscall-dispatch-owned, SSH-service-owned, and
  no-device-host-only ownership models for the live TCP device/interface
  contract.
- deferred: runtime russh/OpenSSH socket binding remains a later strategy
  question after live TCP descriptor delivery exists.
- blocked: live TCP attempts, packet I/O, remote receipt, OpenSSH
  compatibility, Pi 5 hardware proof, ssh-ready=true, generated-root/OpenSSH
  retry, fake command expansion, runtime russh adoption, and phase transition
  remain blocked pending later explicit tasks and evidence.
- removed: no source, docs, helper, task, or evidence artifact was removed.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- Each candidate ownership model is classified as selected, deferred, rejected,
  or blocked with explicit reasons: satisfied.
- The selected contract names source owner files, invariant, accepted
  local/static evidence shape, fail-closed labels, non-retention/redaction
  rules, and exact implementation scope: satisfied.
- selected_device_interface_model is non-null and selected_next_task is
  phase12-ssh-live-tcp-device-interface-local-core-20260629: satisfied.
- The task does not accept live TCP, packet I/O, remote receipt, OpenSSH
  compatibility, Pi 5 hardware proof, ssh-ready=true, runtime russh adoption,
  fake command expansion, or phase transition: satisfied.

## Evidence Map

- Task-owned classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-device-interface-ownership-contract/classification.json.
- Task-owned evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-device-interface-ownership-contract/evidence-map.json.
- Accepted local-to-live TCP gap closeout:
  tasks/2026-06-29-phase12-ssh-local-to-live-tcp-gap-closeout.md.
- Accepted selected local core:
  tasks/2026-06-29-phase12-ssh-selected-live-tcp-local-core.md.
- Accepted gap inventory:
  tasks/2026-06-29-phase12-ssh-local-to-live-tcp-gap-inventory.md.

## Redaction Review

Durable evidence records task ids, source paths, public classifier names,
candidate model names, boolean readiness boundaries, validation
commands/results, and metadata-only source ownership labels. It does not retain
peer identifiers, addresses, packet payloads, key material, session material,
boot artifact bytes, hardware data, private user data, or stable
secret-derived identifiers.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos changes before task edits.
- static review of accepted predecessor task records, current supervisor state,
  source owners, Phase 12 docs, and roadmap: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Result

selected_device_interface_model:
network-owned-smoltcp-interface-with-driver-packet-adapter-ingress-and-descriptor-table-delivery.

selected_next_task:
phase12-ssh-live-tcp-device-interface-local-core-20260629.

No hardware/lab action, hardwareTestLock acquisition, boot publication,
generated-root/OpenSSH retry, live TCP attempt, packet I/O, remote receipt
claim, compatibility claim, ssh-ready=true claim, fake command expansion,
runtime russh adoption, or phase transition is accepted.
