# Phase 11 RP1 Clock/Reset Dependency Control Pi 5

Task id: phase11-rp1-clock-reset-dependency-control-pi5-20260609

Status: accepted

Classification: no-mmio-clock-reset-dependency-control-visible

## Goal

Prove the paired no-MMIO clock/reset dependency control output shape is visible
on Pi 5 before any real read-only clock/reset preflight.

## Scope

- Acquired the hardware lock for this task only.
- Rebuilt and static-reviewed the clock/reset dependency no-MMIO control
  archive with a fresh task-owned capture nonce.
- Published the control archive and captured selected candidate identity, fresh
  serial cursor/drain evidence, stable TFTP delta, final selected-tree identity,
  marker-visible no-MMIO output, replay checkers, and restore proof.
- Accepted only the control output-shape proof and capture-chain evidence; no
  RP1, clock/reset, GPIO, GIC, PCIe, DMA, or other hardware behavior is
  accepted from this control.

## Non-Goals

No RP1/clock/reset/GPIO/GIC/PCIe hardware behavior, clock/reset write, GPIO
function change, GPIO/RIO/pad/INTE/CTRL write, interrupt unmasking or delivery,
endpoint config retry, bridge setup write, DMA/cache, networking, SSH,
Milestone 11.3, or phase transition is accepted.

## Findings And Disposition

- fixed: the control archive includes the task nonce
  clockresetctl20260609T064728Z-1bb93ad1 in the no-MMIO control marker.
- fixed: static archive review passed for the nonce-bearing control archive and
  found no forbidden clock/reset dependency MMIO strings.
- fixed: selected candidate identity before power matched tree
  3f48e70435914a0ca3deb160c517a32205643c3fbd9547d407387895ae417aba,
  effective kernel kernel_2712.img, and expected
  da591740/kernel_2712.img size 48,640 bytes.
- fixed: TFTP evidence retained two matching served fetches for
  da591740/kernel_2712.img, each with 48,640 bytes.
- fixed: run-unique serial visibility passed; the nonce token was absent before
  power and appeared 30 times after power.
- fixed: final pre-restore identity still pointed at the selected tree, and
  restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: boot-staging identity checker classified the retained bundle as
  boot-staging-identity-ready.
- not-an-issue: no docs/src update was required because the accepted frontier
  and control contract did not change.

No findings were removed or deferred.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-control-pi5/classification.json.
- Static archive review:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-control-pi5/static-archive-review.txt.
- Capture bundle:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-control-pi5/control-run/.
- Run-unique checker:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-control-pi5/run-unique-check.json.
- Boot-staging identity checker:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-control-pi5/boot-staging-identity-check.json.
- Restore proof:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-control-pi5/control-run/restore-snapshot.json and
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-control-pi5/control-run/post-restore-status.json.

## Validation

- image/archive inspection: static review passed for the nonce-bearing no-MMIO
  control archive.
- lab-controller API: snapshot, archive publish, power-cycle, serial, TFTP,
  final identity, and restore records were captured.
- serial hardware boot/output: passed under the run-unique discriminator;
  required nonce token was absent before power and present after power.
- TFTP evidence: passed; stable same-cursor pre-restore delta retained two
  matching 48,640-byte candidate kernel fetches.
- final pre-restore identity: passed; selected tree remained staged before
  restore.
- restore proof: passed; post-restore tree hash matched the pre-run baseline.
- run-unique checker: passed; classification capture-transaction-run-unique-ready.
- boot-staging identity checker: passed; classification boot-staging-identity-ready.
- jq empty on classification, evidence map, checker, and retained JSON
  artifacts: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed before commit.

No docs/src files were touched, so mdbook is not required.

## Result

Accepted as no-mmio-clock-reset-dependency-control-visible. This unblocks the
queued real clock/reset dependency read-only Pi 5 preflight task, subject to the
hardware lock remaining unlocked/restored.

## Next Action

Next queued task phase11-rp1-clock-reset-dependency-pi5-20260609 is
mechanically unblocked on the next worker wake if hardwareTestLock remains
unlocked/restored.
