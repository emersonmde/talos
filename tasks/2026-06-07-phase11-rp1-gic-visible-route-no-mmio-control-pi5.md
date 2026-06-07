# Phase 11 RP1 GIC-Visible Route No-MMIO Control Pi 5

Task id: phase11-rp1-gic-visible-route-no-mmio-control-pi5-20260607

Status: accepted

## Goal

Run the paired no-MMIO/no-GIC/no-RP1 control candidate on Pi 5 to prove the
output/capture/identity path before any real GIC-visible route status
diagnostic proof.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Checked the accepted local/static control archive before publication:
  target/talos-rpi5-rp1-gic-visible-route-no-mmio-control-core.tar.gz.
- Published only the accepted no-MMIO/no-GIC/no-RP1 GIC-visible route control
  archive.
- Retained publication identity, fresh serial/TFTP cursors, serial capture,
  stable pre-restore TFTP evidence, final pre-restore identity, restore
  evidence, and v2 identity-join records.
- Performed standard inconclusive-run triage after the first powered candidate
  run was rejected by non-empty pre-power serial drain evidence: candidate
  identity, fresh serial/TFTP evidence, known-good control, and candidate
  rerun.

## Non-Goals

No real GIC-visible route diagnostic candidate, GICD/GICC/RP1/MSI-X/PCIe
config/MIP/GPIO/pads/RIO/clock/reset MMIO access, GIC enable writes, IAR/EOIR
acknowledgement, interrupt unmasking, ISR installation, RP1 writes, GPIO
ownership, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, phase transition,
or real GIC-visible route behavior acceptance.

## Classification

Accepted as no-mmio-gic-visible-route-control-visible.

The accepted candidate rerun selected boot tree
4dddd8f21e47f01dcae0fb73ab336c8f2486536dea225543b05103ab30d888b8 with
effective kernel_2712.img and a 47,040-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: pre-power serial drain was
empty, stable pre-restore TFTP retained two served 47,040-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
capture retained 507 occurrences of TALOS: rp1-gic-route-status-control.

The retained control output classification remains simulated/control. This
accepts only the no-MMIO/no-GIC/no-RP1 output-shape and capture path for the
queued real GIC-visible route diagnostic proof. Real GIC status behavior,
interrupt pending state, interrupt delivery, IAR/EOIR acknowledgement,
ISR/handler ownership, GPIO ownership, pin-control behavior, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
behavior, Milestone 11.3, and phase transition remain unaccepted.

The capture helper restored its pre-run snapshot after the accepted rerun,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO/no-GIC/no-RP1
  control archive, including archive SHA-256, kernel SHA-256, marker string,
  and forbidden real diagnostic string absence.
- fixed: retained the first candidate run as capture-staging-blocked evidence;
  it had candidate-tied fetches and 506 control markers but was rejected by
  non-empty pre-power serial drain evidence.
- fixed: ran the required known-good control after the inconclusive first
  candidate run; it retained the production timer PASS marker and preserved
  the serial-drain rejection evidence.
- fixed: reran the selected no-MMIO/no-GIC/no-RP1 control candidate after the
  known-good control; the rerun passed the v2 identity join and retained
  repeated GIC-visible route control output.
- deferred: the real GIC-visible route diagnostic proof remains queued and
  must pass its own hardware lock, identity join, and classification gates.
- not-an-issue: no GIC/RP1 route behavior is inferred from a no-MMIO simulated
  control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-no-mmio-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-no-mmio-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-no-mmio-control-pi5/control-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-no-mmio-control-pi5/control-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-no-mmio-control-pi5/known-good-control-run/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,040-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 507 occurrences of
  TALOS: rp1-gic-route-status-control were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-gic-visible-route-control-visible. The queued real
GIC-visible route diagnostic proof is mechanically unblocked on a future worker
wake if hardwareTestLock remains unlocked/restored and supervisorIntervention
remains inactive.
