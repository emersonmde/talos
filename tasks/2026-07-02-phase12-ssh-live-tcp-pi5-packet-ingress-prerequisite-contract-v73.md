# Phase 12 SSH Live TCP Pi 5 Packet Ingress Prerequisite Contract V73

Task id: phase12-ssh-live-tcp-pi5-packet-ingress-prerequisite-contract-v73-20260702

Status: accepted after no-hardware contract closeout.

Classification: packet-ingress-prerequisite-blocked-for-source-repair.

Evidence level: git status inspection, supervisor-state JSON validation,
roadmap/static architecture review, Phase 12 task-record review, source
inspection, task-owned JSON evidence, docs build, and diff checks. No hardware
action, lab publication, boot snapshot mutation, Pi 5 power cycle, serial
capture, TFTP capture, kernel_main proof, route-start proof, runtime-ready
proof, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed.

## Goal

Connect the accepted exceptions-ready Pi 5 frontier to the first live TCP
packet-ingress prerequisite without promoting stale marker-chain tasks by
default.

## Scope Performed

- Promoted this ready no-hardware contract after the supervisor selected it as
  the single feature-led successor to the feature-frontier checkpoint.
- Reviewed the roadmap, Phase 12 networking/SSH architecture notes, early
  POSIX/VFS frontier, v71 exceptions-ready closeout, the feature-frontier
  checkpoint, accepted local/static smoltcp descriptor-delivery records, and
  current source.
- Reconciled the accepted local deterministic runtime path in src/network.rs
  against the selected Pi 5 exceptions-ready evidence.
- Classified kernel_main, route-start, runtime-ready, v72, v60, and v53
  against the live TCP packet-ingress feature path.

## Contract Result

The accepted local/static networking chain already proves descriptor-facing
smoltcp delivery in a deterministic host-owned model. The accepted reports in
src/network.rs deliberately keep live_packet_io_accepted,
live_reachability_accepted, remote_receipt_accepted, compatibility_accepted,
hardware_frame_provider_bound, service success, and ssh_ready false. The
runtime marker route can report accepted deterministic device-interface
delivery, but only when require_hardware_frame_provider is false.

The accepted Pi 5 frontier is v71 selected-normal-runtime exceptions-ready:
selected TFTP served da591740/kernel_2712.img four times at 152,880 bytes, the
serial window retained 881 occurrences of TALOS: exceptions ready
capture-nonce=runtime-marker-route-static, final pre-restore identity remained
selected, and restore returned to the clean baseline. That proves the selected
candidate reaches the exceptions-ready point on device, but it does not prove
kernel_main, route-start, runtime-ready, live frame ingress, packet I/O, remote
receipt, compatibility, service success, or ssh-ready.

The first live TCP packet-ingress prerequisite is therefore not a generic
kernel_main marker. It is the missing source-level hardware frame-provider
binding between the RP1 Ethernet/MAC frame source and the already accepted
DriverPacketAdapter/smoltcp/listener/descriptor-delivery path. Source
inspection found only the host/local PacketQueueNetworkDevice,
DriverPacketAdapter, and smoltcp runtime witness; no RP1 Ethernet hardware
frame provider is accepted as a NetworkDevice or as a source that can make
LiveTcpNetworkDeviceRuntimeReport.hardware_frame_provider_bound true. The
existing Phase 12.1 Ethernet frontier also remains paused at link-not-ready and
has no accepted frame ingress.

Because the required next step is source repair/implementation rather than an
already queued mechanically unblocked task, this contract selects no successor.
The supervisor must plan exactly one bounded source-repair implementation task
that binds or models the RP1 hardware frame-provider boundary before any
kernel_main, route-start, runtime-ready, packet-I/O hardware run, OpenSSH retry,
or generated-root retry resumes.

## Terminal Classification

packet-ingress-prerequisite-blocked-for-source-repair.

first_missing_prerequisite:
RP1 Ethernet hardware frame-provider binding into the accepted
DriverPacketAdapter/smoltcp/listener/descriptor-delivery path.

selected_next_task: null.

planningNeeded: true.

planningReason: The first live TCP packet-ingress prerequisite is a missing
source-level hardware frame-provider binding, and no queued task defines that
bounded repair with explicit scope, gates, evidence, and non-goals. Supervisor
planning is required before implementation or hardware proof resumes.

## Stale Task Dispositions

- deferred: v72
  phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-after-exceptions-reconciliation-v72-20260702
  remains paused. Kernel_main may be retained as metadata or a later
  reachability control, but it is not the first live TCP packet-ingress
  prerequisite without a hardware frame-provider binding task.
- deferred: v60
  phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-after-no-route-start-repair-reconciliation-v60-20260702
  remains blocked/deferred. Its runtime-ready dependency chain is stale and
  still assumes marker-chain route-start work before the hardware
  frame-provider gap is repaired.
- deferred: v53
  phase12-ssh-live-tcp-selected-normal-runtime-packet-io-continuation-reconciliation-v53-20260701
  remains blocked/deferred. Packet-I/O cannot be selected before the source
  owns the RP1 hardware frame provider and its evidence gate is refreshed.

## Findings

- fixed: identified the first feature-led missing prerequisite for live TCP
  packet ingress as source-level hardware frame-provider binding, not a generic
  kernel_main marker.
- fixed: preserved the accepted local/static smoltcp descriptor-delivery
  frontier as support while keeping live packet I/O and ssh-ready false.
- fixed: preserved v71 as the Pi 5 selected exceptions-ready frontier without
  adding kernel_main, route-start, runtime-ready, packet-I/O, or service
  claims.
- deferred: v72, v60, and v53 remain stale until a future source-repair or
  explicitly refreshed feature-led task reselects them.
- blocked: the next worker task must be supervisor-planned because no queued
  task currently owns the hardware frame-provider source repair.
- not-an-issue: no Pi 5 hardware triage is required; this task used only
  retained state/docs/task/source evidence and made no hardware-dependent
  acceptance claim.
- removed: stale marker-chain auto-promotion as the default path to live TCP
  packet ingress.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-packet-ingress-prerequisite-contract-v73/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-packet-ingress-prerequisite-contract-v73/evidence-map.json.
- Static contract summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-packet-ingress-prerequisite-contract-v73/static/prerequisite-contract-summary.md.
- Roadmap:
  docs/src/roadmap.md.
- Phase 12 frontier:
  docs/src/project/phase12-networking-ssh.md.
- Local/static descriptor-delivery source:
  src/network.rs and src/ssh_service_readiness.rs.
- Accepted local descriptor-delivery task:
  tasks/2026-06-29-phase12-ssh-live-tcp-listener-descriptor-accept-local-core.md.
- Accepted deterministic runtime closeout:
  tasks/2026-06-29-phase12-ssh-live-tcp-network-device-smoltcp-runtime-closeout.md.
- v71 closeout:
  tasks/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-closeout-v71.md.
- Feature-frontier checkpoint:
  tasks/2026-07-02-phase12-ssh-live-tcp-feature-frontier-realignment-checkpoint.md.

## Redaction Review

Task-owned contract evidence retains task ids, source/path labels, hashes, byte
counts, marker counts, classifications, public enum/function names, and
validation outcomes. It does not retain private user data, credentials, packet
payloads, peer identifiers, SSH/session/key material, public-key blobs,
signatures, fingerprints, operator identities, or external account data.

## Validation

- git status --short --branch before edits/action: pass.
- jq empty on supervisor state and task-owned JSON evidence: pass.
- Roadmap, Phase 12 architecture, early POSIX/VFS frontier, v71 closeout,
  feature-frontier checkpoint, local/static smoltcp descriptor-delivery
  records, and source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
