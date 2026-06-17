# Phase 10 Pi 5 Generated-Root Milestone 10.3 Closeout

Task id: phase10-pi5-generated-root-milestone-10-3-closeout-20260617

Status: accepted

Classification:
phase10-milestone-10-3-closed-generated-root-consumption-command-input-paused

Evidence level: static/task evidence inspection, accepted generated-root
transport task records, accepted Pi 5 firmware-initramfs consumption evidence,
blocked command-input closeout evidence, task-owned JSON evidence, docs build,
and diff checks. No hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, runtime implementation, storage work, networking,
SSH, Phase 11/12 expansion, or phase transition was performed by this closeout.

## Goal

Reconcile the Milestone 10.3 generated-root transport boundary after the
command-input follow-up. Record whether the milestone is accepted, paused, or
blocked without treating the paused command-input surface as feature progress.

## Outcome

Milestone 10.3 is closed at the generated-root transport and consumption
boundary:

- local/QEMU no-kernel-rebuild generated-root transport is accepted: one kernel
  image consumed two different external generated-root artifacts and observed
  different generated file and executable behavior;
- Pi 5 firmware-initramfs generated-root consumption is accepted: the Pi 5 boot
  proof preserved the firmware-loaded initramfs_2712 artifact through early
  memory setup and installed it as source=firmware-initramfs
  reason=valid-artifact;
- Pi 5 shell-visible generated-root command input is not accepted and is
  explicitly paused at the command 0 prelude blocker recorded by the direct-read
  closeout.

The paused command-input blocker does not reopen the generated-root transport
acceptance boundary. It is retained as a future supervisor-planned control
surface problem: after command 0 pre-write freshness and a successful
/serial/write of rootinfo, the direct-read window did not retain rootinfo,
source evidence, or dispatch command=0 status=handled responses=1.

No next implementation task is mechanically available in the queue after this
closeout. Supervisor planning is required to select the next feature-led task.
This closeout does not authorize persistence, writable storage, networking,
SSH, Phase 11/12 expansion, or any phase transition.

## Findings

- fixed: reconciled the accepted local/QEMU generated-root no-rebuild
  transport, Pi 5 firmware-initramfs consumption, and command-input follow-up
  into one Milestone 10.3 boundary.
- fixed: recorded that command-input failure is a paused control-surface
  blocker, not a generated-root transport blocker.
- deferred: future command-input work remains behind supervisor planning around
  the command 0 prelude invariant.
- deferred: writable persistence, SD/USB/block storage, broader filesystem
  mutation, networking, SSH, Phase 11/12 expansion, and phase transition remain
  outside this milestone closeout.
- rejected: treating prompt visibility, /serial/write byte acceptance,
  command-loop readiness, or direct-read readiness as shell-visible
  generated-root command-input proof.
- rejected: selecting a storage, networking, SSH, Phase 11/12, or
  phase-transition follow-up from this closeout without an explicit queued
  supervisor task.
- not-an-issue: no hardware lock, boot publication, Pi 5 rerun, or runtime code
  change was required because this task is a static milestone closeout over
  committed evidence.

## Evidence

- Local/QEMU no-rebuild transport closeout:
  tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-closeout.md.
- Pi 5 boot-transport contract:
  docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md.
- Pi 5 firmware-initramfs reservation closeout:
  tasks/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-closeout.md.
- Direct-read command-input closeout:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-closeout.md.
- Direct-read closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-direct-read-closeout/classification.json.
- Milestone closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-milestone-10-3-closeout/classification.json.
- Milestone closeout evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-milestone-10-3-closeout/evidence-map.json.

## Accepted Boundary

Accepted:

- local/QEMU no-kernel-rebuild generated-root transport for different external
  generated-root artifacts with one kernel image;
- Pi 5 firmware-loaded initramfs_2712 delivery of the accepted generated-root
  artifact format;
- Pi 5 preservation of the firmware initramfs bytes through early memory setup
  by excluding the FDT /chosen initrd range from the early usable-memory
  candidate;
- Pi 5 generated-root installation from source=firmware-initramfs
  reason=valid-artifact;
- deterministic compiled-fallback behavior for missing, invalid, malformed, or
  oversized generated-root artifact bytes.

Paused:

- Pi 5 shell-visible generated-root command input at the command 0 prelude
  invariant.

Not accepted:

- generated-root command input on Pi 5;
- writable persistence;
- SD/USB/block storage;
- broader filesystem mutation;
- networking, sockets, or SSH;
- Phase 11/12 feature expansion from this evidence;
- phase transition.

## Acceptance Check

- Milestone 10.3 accepted/paused/blocked boundary is explicit and
  evidence-backed: satisfied. Generated-root transport and Pi 5 consumption are
  accepted; command input is paused at command 0.
- Deferred risks and rejected claims are documented: satisfied.
- Any selected next task follows the feature-led roadmap and is dependency-gated
  by accepted task ids/evidence: satisfied by selecting no implementation task
  because none is explicitly queued; supervisor planning is required.
- Phase transition, storage, networking, SSH, and Phase 12 resumption remain
  gated behind explicit closeout/checkpoint evidence: satisfied.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before any next implementation task. The next
task must be explicitly queued and feature-led; this worker closeout does not
create or promote storage, networking, SSH, Phase 11/12 expansion, command-input
retry, or phase-transition work.
