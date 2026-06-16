# Phase 10 Pi 5 Generated-Root Firmware Initramfs Reservation Source Contract

Task id: phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract-20260616

Status: accepted

Classification:
pi5-generated-root-firmware-initramfs-reservation-source-contract-selected

Evidence level: static/source/task evidence inspection, task-owned JSON
evidence, docs build, and diff checks. No runtime code change, Pi 5 hardware
run, boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, TFTP/serial capture, networking, SSH, persistence,
SD/USB/block-driver work, or phase transition was performed.

## Goal

Select the smallest source-backed implementation boundary that can unblock Pi 5
generated-root consumption of the firmware-loaded 'initramfs_2712' artifact.

## Context

The accepted Pi 5 generated-root boot transport delivered the candidate
artifact through Raspberry Pi firmware:

- the archive shape with root and 'da591740/' copies of 'initramfs_2712' is
  accepted;
- the hardware proof retained TFTP evidence for
  'da591740/initramfs_2712' at 662 bytes;
- serial evidence reported FDT '/chosen' firmware initramfs bounds
  '0x2efff000..0x2efff296';
- Talos still installed the compiled fallback because early page-frame seed,
  bootstrap reservation, and translation-table setup reused that same low-tail
  physical range before generated-root installation.

## Selected Boundary

Selected implementation boundary:
'pi5-generated-root-firmware-initramfs-reserve-by-memory-plan-exclusion-v1'.

The next implementation task should preserve the firmware initramfs bytes by
excluding the FDT '/chosen' 'linux,initrd-start..linux,initrd-end' range from
the Pi 5 early usable-memory candidate before the bootstrap page reservation,
translation-table layout, bootstrap allocator, and cache transition can use
that memory.

The core implementation is intentionally narrow:

- read and retain the firmware initramfs range during the Pi 5 DTB phase before
  'plan_boot_memory';
- thread that optional range into the low-tail memory-layout planner as an
  additional reserved/excluded range;
- make the planner advance 'candidate_start' past the initramfs range when it
  overlaps the low-bank tail, using the same page-aligned low-tail policy as
  existing DTB, FDT reservation, and '/reserved-memory' exclusions;
- keep 'install_firmware_initramfs_generated_root' after memory setup, but now
  with the accepted invariant that early bootstrap allocation did not consume
  the firmware bytes;
- preserve the existing all-or-nothing generated-root parser and compiled
  fallback behavior for missing, malformed, oversize, or unsupported artifact
  bytes.

This is reservation, not a copy-first design. Copying the range before the
memory plan would require choosing a destination before allocator ownership is
established or adding a static maximum-size buffer; both are broader than the
known blocker.

## Source Owners

- 'src/device_tree/chosen.rs': already parses 'linux,initrd-start' and
  'linux,initrd-end' into 'FdtInitrdRange'.
- 'src/boot/rpi5.rs': owns Pi 5 DTB phase ordering, memory planning, bootstrap
  allocator initialization, cache transition, and the
  'rpi5_generated_root_boot_transport' installer.
- 'src/memory_map/layout.rs': owns the conservative low-tail candidate policy
  and should receive the optional extra exclusion/range.
- 'src/memory_map/page_frames.rs' and 'src/memory_map/translation.rs': should
  remain consumers of the selected candidate/reservation; they do not need to
  know why the candidate starts later.
- 'src/initramfs.rs': owns artifact parsing, generated-root selection, and
  compiled fallback behavior; this task does not change the artifact format.
- 'scripts/rpi5-generated-root-boot-transport-*.sh': remain the candidate
  archive/image/review owners unless the implementation changes expected serial
  proof strings.

## Memory Invariants

- If FDT '/chosen' initrd bounds are present and non-empty, the early usable
  candidate must not include any page that intersects the rounded firmware
  initramfs range.
- Existing exclusions for kernel image, DTB blob, FDT reservation entries, and
  '/reserved-memory' ranges remain in force.
- The low-tail policy remains conservative; this task does not accept high
  memory, a complete physical-memory map, DMA-safe allocation, or generalized
  reservation accounting.
- If the excluded range leaves no valid low-tail candidate, the boot-memory
  plan should fail through existing unavailable paths rather than allocate over
  firmware-owned bytes.
- Missing or invalid initrd bounds must not make Pi 5 generated-root
  consumption succeed; the installer should continue reporting compiled
  fallback reasons.

## Failure And Rollback Behavior

- Parser fallback remains all-or-nothing: malformed artifact bytes never
  partially merge into the VFS.
- Hardware proof remains a later serialized task. This source contract does not
  publish a new boot tree or mutate lab state.
- A failing local/static implementation should record the first unsatisfied
  invariant rather than moving directly to Pi 5 hardware.
- A future Pi 5 proof must still follow the standard triage order after an
  inconclusive run: candidate identity, fresh serial cursor, TFTP delta,
  known-good control when needed, then candidate rerun.

## Findings

- fixed: selected a concrete source boundary that preserves the firmware
  initramfs by excluding it from early memory ownership before bootstrap and
  translation-table allocation.
- fixed: tied the implementation to existing source owners and the accepted
  low-tail memory policy rather than inventing a second boot-time storage path.
- deferred: implementation, local tests, candidate archive rebuild/static
  review, and Pi 5 proof remain separate dependency-gated tasks.
- rejected: treating the accepted TFTP fetch alone as Pi 5 generated-root
  consumption proof.
- rejected: copy-first remediation, static maximum-size buffers, SD/USB/block
  persistence, networking, SSH, high-memory ownership, DMA-safe allocation, and
  phase transition claims from this source/static task.
- not-an-issue: no hardware lock or Pi 5 inconclusive-run triage was needed for
  this static source contract.

## Evidence

- Prior Pi 5 boot transport contract:
  'tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-contract.md'.
- Accepted candidate archive core:
  'tasks/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core.md'.
- Source-backed Pi 5 proof blocker:
  'tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-proof.md'.
- Prior closeout:
  'tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-closeout.md'.
- Current project contract:
  'docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md'.
- Source owners inspected:
  'src/device_tree/chosen.rs', 'src/boot/rpi5.rs',
  'src/memory_map/layout.rs', 'src/memory_map/page_frames.rs',
  'src/memory_map/translation.rs', and 'src/initramfs.rs'.
- Classification JSON:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract/classification.json'.
- Evidence map:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-source-contract/evidence-map.json'.

## Acceptance Check

- Contract names one selected implementation boundary or precise blocker:
  satisfied with
  'pi5-generated-root-firmware-initramfs-reserve-by-memory-plan-exclusion-v1'.
- Selected boundary preserves accepted generated-root transport goals:
  satisfied; Pi 5 must consume firmware-loaded FDT '/chosen' initrd bytes, not
  the compiled fallback.
- Contract identifies exact source owners, memory invariants, rollback/failure
  behavior, and rejected claims: satisfied.
- Implementation follow-up selected:
  'phase10-pi5-generated-root-firmware-initramfs-reservation-core-20260616'.
- Pi 5 proof, boot publication, hardware action, networking, SSH, persistence,
  SD/USB/block drivers, and phase transition remain rejected: satisfied.

## Validation

- static/source/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

After this source contract is accepted and committed, promote
'phase10-pi5-generated-root-firmware-initramfs-reservation-core-20260616' if
dependencies remain satisfied. That task may implement only the accepted
reservation-by-memory-plan-exclusion boundary and local/static validation; it
must not publish a boot archive, acquire hardwareTestLock, power-cycle the Pi
5, or claim Pi 5 generated-root consumption.
