# Phase 10 Pi 5 Generated-Root Firmware Initramfs Reservation Closeout

Task id: phase10-pi5-generated-root-firmware-initramfs-reservation-closeout-20260616

Status: accepted

Classification:
pi5-generated-root-firmware-initramfs-consumption-accepted-command-input-deferred

Evidence level: static/task evidence inspection, accepted Pi 5
serial/TFTP/restore evidence, task-owned JSON evidence, docs build, and diff
checks. No runtime code change, Pi 5 hardware run, boot archive publication,
lab mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial capture,
networking, SSH, persistence, SD/USB/block-driver work, or phase transition
was performed by this closeout.

## Goal

Reconcile the accepted source contract, local/static implementation, and
serialized Pi 5 proof for the firmware-initramfs reservation boundary. Freeze
the accepted generated-root capability and keep remaining risks explicitly
dependency-gated.

## Outcome

The Pi 5 generated-root firmware-initramfs transport is accepted for the
read-only generated-root artifact boundary:

- source contract selected
  'pi5-generated-root-firmware-initramfs-reserve-by-memory-plan-exclusion-v1';
- core implementation excludes the page-rounded FDT '/chosen'
  'linux,initrd-start..linux,initrd-end' range from the Pi 5 early
  usable-memory candidate before bootstrap reservation, translation-table
  placement, allocator initialization, and cache transition can claim it;
- serialized Pi 5 proof retained selected-tree identity, served the expected
  208984-byte 'da591740/kernel_2712.img' twice in the same power cycle, and
  reported 'source=firmware-initramfs reason=valid-artifact' for the
  '0x2efff000..0x2efff296' firmware initramfs range;
- post-run restore returned the lab boot tree to
  'a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10'.

This accepts Pi 5 consumption of the firmware-loaded generated-root artifact.
It does not accept scripted interactive command injection for this scenario:
the accepted proof reached the command-loop surface but the scripted input
arrived empty and the scenario reported an incomplete command-loop proof. That
is a separate control-surface limitation, not a generated-root preservation
blocker.

## Findings

- fixed: reconciled the source contract, implementation, and Pi 5 proof into
  an accepted Pi 5 firmware-initramfs generated-root consumption boundary.
- fixed: replaced the prior Milestone 10.3 hardware blocker, where the
  firmware initramfs range overlapped early memory setup, with accepted proof
  that memory-plan exclusion preserves the bytes through generated-root
  installation.
- fixed: retained all-or-nothing generated-root parser and compiled-fallback
  behavior for missing, invalid, malformed, oversize, or unsupported artifact
  bytes.
- deferred: scripted Pi 5 command injection for this generated-root scenario
  remains unaccepted because the proof captured empty input.
- deferred: writable persistence, SD/USB/block storage, broader filesystem
  mutation, networking, SSH, Phase 11/12 expansion, and phase transition still
  require explicit future tasks.
- rejected: treating Pi 5 generated-root consumption as proof of persistence,
  storage-driver support, networking, SSH, or a phase transition.
- not-an-issue: no hardware lock, Pi 5 inconclusive-run triage, or new
  hardware run was required for this static closeout because the accepted proof
  already retained decisive serial, TFTP, identity, and restore evidence.

## Evidence

- Source contract:
  'tasks/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract.md'.
- Source contract classification:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract/classification.json'.
- Core implementation:
  'tasks/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-core.md'.
- Core classification:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-core/classification.json'.
- Pi 5 proof:
  'tasks/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof.md'.
- Pi 5 proof classification:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof/classification.json'.
- Pi 5 proof evidence map:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof/evidence-map.json'.
- Closeout classification:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-closeout/classification.json'.
- Closeout evidence map:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-closeout/evidence-map.json'.

## Accepted Boundary

Accepted:

- Pi 5 firmware-loaded 'initramfs_2712' delivery of the existing
  'talos-generated-root-v1' artifact.
- Preservation of the FDT '/chosen' firmware initramfs bytes through early Pi
  5 memory setup by excluding the page-rounded range from the early
  usable-memory candidate.
- Generated-root installation from 'source=firmware-initramfs
  reason=valid-artifact' on Pi 5 hardware.
- Retained compiled-fallback behavior for invalid or missing firmware
  initramfs evidence.

Not accepted:

- scripted command injection success in the Pi 5 proof scenario;
- writable persistence;
- SD/USB/block storage;
- broader filesystem mutation;
- networking, sockets, or SSH;
- Phase 11/12 feature expansion from this evidence;
- phase transition.

## Acceptance Check

- Closeout classification matches retained proof/blocker evidence: satisfied
  with
  'pi5-generated-root-firmware-initramfs-consumption-accepted-command-input-deferred'.
- Accepted generated-root capability, deferred risks, and rejected claims are
  explicit: satisfied.
- Next milestone step selected only through explicit dependency-gated
  planning: satisfied; this closeout selects no follow-up implementation task
  and requests supervisor planning.
- If Pi 5 consumption remained blocked, planningNeeded would be set or a
  precise blocker recorded: not applicable because consumption is accepted.
- Task record and task-owned JSON record findings with disposition:
  satisfied.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before any next task. This closeout does not
promote command-input work, writable persistence, SD/USB/block storage,
networking, SSH, Phase 11/12 work, or a phase transition. The next explicit
task should be dependency-gated around the roadmap priority rather than
inferred from this accepted hardware transport proof.
