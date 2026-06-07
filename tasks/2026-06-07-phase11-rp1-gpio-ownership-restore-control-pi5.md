# Task: Phase 11 RP1 GPIO Ownership/Restore Control Pi 5

Task ID: phase11-rp1-gpio-ownership-restore-control-pi5-20260607

Status: accepted

Evidence level: static archive identity inspection, lab-controller API, serial
hardware boot/output, stable same-cursor TFTP evidence,
pi5-capture-transaction-v2 identity join, restore proof

## Goal

Run the paired no-MMIO/no-RP1/no-GIC control candidate on Pi 5 to prove the
output/capture/identity path before any real GPIO ownership/restore hardware
proof.

## Scope

- Acquired the hardware lock only after the ownership/restore core task was
  accepted and committed.
- Preflighted candidate identity through lab status, effective kernel, selected
  boot tree, and expected kernel_2712.img size.
- Published only the accepted no-MMIO/no-RP1/no-GIC control archive:
  target/talos-rpi5-rp1-gpio14-ownership-route-preflight-no-mmio-control-core.tar.gz.
- Captured fresh serial and stable same-cursor TFTP evidence before restore.
- Restored the prior accepted boot tree before releasing the hardware lock.
- Applied the standard triage sequence after initial serial freshness rejection:
  candidate identity, fresh serial/TFTP evidence, known-good control, and
  candidate rerun.

## Non-Goals

No real GPIO ownership/restore candidate, RP1/GPIO/RIO/pads/clock-reset/MSI-X/
PCIe/MIP/GIC MMIO access, interrupt enablement or delivery, GIC IAR/EOIR
acknowledgement, ISR installation, GPIO event generation, broad GPIO ownership,
pin-control writes, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, or phase transition.

## Findings

- fixed: retained static archive review for the accepted control archive before
  publication. The archive SHA256 is
  5977bfdd8880a7eebe5a7d31c1db8cde10bea65994c9da3d14c41b1913dba170, the
  kernel_2712.img SHA256 is
  c406268f9c5b5257bd3671d9502b1328910d56352e720d46ae6d5cf34e6964e7, and the
  kernel size is 48368 bytes.
- fixed: first candidate run retained 556 control markers and two matching
  48368-byte candidate TFTP fetches, but was rejected as capture-staging-blocked
  because pre-power serial drain was non-empty.
- fixed: known-good timer control retained its PASS marker and two matching
  TFTP fetches, but was also rejected as capture-staging-blocked because
  pre-power serial drain was non-empty.
- fixed: candidate rerun passed the repaired v2 identity join with no rejection
  reasons, 556 control markers, two matching 48368-byte candidate TFTP fetches,
  final selected-tree identity still staged, and post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: real GPIO ownership/restore hardware proof, GPIO event generation,
  interrupt pending/delivery, GIC acknowledgement, ISR/handler ownership, broad
  GPIO ownership, pin-control/pad/RIO writes, clock/reset programming,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe
  enumeration, Milestone 11.3, and phase transition.

No findings were removed or classified as not-an-issue in this task.

## Classification

Accepted classification:
no-mmio-gpio-ownership-restore-control-visible.

Accepted claims are limited to the control output/capture path. This proves the
no-MMIO/no-RP1/no-GIC control candidate can be staged, fetched, observed,
identity-joined, and restored on Pi 5 hardware. It does not accept any real RP1
GPIO ownership/restore behavior.

## Evidence

- Classification: tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-control-pi5/classification.json
- Evidence map: tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-control-pi5/evidence-map.json
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-control-pi5/control-run/run-result-summary.json
- Known-good control run:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-control-pi5/known-good-control-run/run-result-summary.json
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-control-pi5/control-rerun-after-kg/run-result-summary.json
- Lock release:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-control-pi5/lock-release.json

## Validation

- Static archive identity check: passed.
- Lab API status preflight and post-restore records: passed.
- Fresh serial capture summary: passed on accepted candidate rerun.
- Stable same-cursor TFTP evidence before restore: passed on accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on accepted candidate rerun.
- Restore proof: passed.
- mdbook build: not run; no docs/src files were touched.

## Result

Accepted. The serialized Pi 5 no-MMIO/no-RP1/no-GIC GPIO ownership/restore
control proof is visible and identity-joined. Hardware lock was released after
restore.

## Follow-Up

Promote phase11-rp1-gpio-ownership-restore-pi5-20260607 on a future worker
wake if hardwareTestLock remains unlocked/restored and supervisorIntervention
remains inactive. The next task owns the real GPIO ownership/restore diagnostic
proof; this control task does not accept real GPIO behavior.
