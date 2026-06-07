# Phase 11 RP1 Interrupt-Routing No-MMIO Control Pi 5

Task id: phase11-rp1-interrupt-routing-no-mmio-control-pi5-20260607

Status: accepted

## Goal

Run the paired no-MMIO/no-enable control candidate on Pi 5 to prove the
output/capture/identity path before any real interrupt-routing diagnostic
proof.

## Scope

- Acquired hardwareTestLock for the serialized Pi 5 control run.
- Checked the accepted local/static control archive before publication:
  target/talos-rpi5-rp1-interrupt-routing-no-mmio-control-core.tar.gz.
- Published only the accepted no-MMIO/no-enable interrupt-routing control
  archive.
- Retained publication identity, fresh serial/TFTP cursors, serial capture,
  stable pre-restore TFTP evidence, final pre-restore identity, restore
  evidence, and v2 identity-join records.
- Performed standard inconclusive-run triage after the first powered candidate
  run was rejected by non-empty pre-power serial drain evidence: candidate
  identity, fresh serial/TFTP evidence, known-good control, and candidate
  rerun.

## Non-Goals

No real RP1 interrupt-routing diagnostic candidate, RP1 interrupt/GPIO/pads/RIO
or clock/reset writes, MSI-X enable/IACK writes, PCIe config or MSI writes, GIC
MMIO access, interrupt enablement/delivery, GPIO ownership, pin-control writes,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, phase transition, or real interrupt-routing behavior
acceptance.

## Classification

Accepted as no-mmio-interrupt-routing-control-visible.

The accepted candidate rerun selected boot tree
c4d59ab46368e4f79f59b10543d54cf6b2198e86f57b7a2e0bfdf8c2313dc1ae with
effective kernel_2712.img and a 46,520-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: pre-power serial drain was
empty, stable pre-restore TFTP retained two served 46,520-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
capture retained 990 occurrences of TALOS: rp1-interrupt-routing-control.

The retained control output classification remains simulated/control. This
accepts only the no-MMIO/no-enable output-shape and capture path for the queued
real interrupt-routing diagnostic proof. Real RP1 MSIX_CFG read behavior, GPIO
ownership, pin-control behavior, pad writes, interrupt enablement/delivery,
MSI-X enable/IACK writes, PCIe MSI delivery, GIC delivery, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
behavior, Milestone 11.3, and phase transition remain unaccepted.

The capture helper restored its pre-run snapshot after the accepted rerun. The
worker also restored the original pre-task boot snapshot, returning the lab to
tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO/no-enable
  control archive, including archive SHA-256, marker string, and forbidden
  real diagnostic string absence.
- fixed: retained the first unpowered attempt as preflight-staging-publication
  mismatch evidence; the local evidence script used a non-matching tar member
  path and recorded expected_fetch_bytes=0, so capture exited before power.
- fixed: reran the selected candidate with corrected expected fetch identity;
  it retained candidate fetches and 990 control markers but was rejected by
  non-empty pre-power serial drain evidence.
- fixed: ran the required known-good control after the rejected candidate run;
  it retained the production timer PASS marker and preserved the serial-drain
  rejection evidence.
- fixed: reran the selected no-MMIO/no-enable control candidate after the
  known-good control; the rerun passed the v2 identity join and retained
  repeated interrupt-routing control output.
- fixed: performed an explicit final restore to the original pre-task boot
  tree after the accepted rerun.
- deferred: the real RP1 interrupt-routing diagnostic proof remains queued and
  must pass its own hardware lock, identity join, and classification gates.
- not-an-issue: no RP1 interrupt-routing behavior is inferred from a no-MMIO
  simulated control.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-no-mmio-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-no-mmio-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-no-mmio-control-pi5/control-rerun-after-kg/.
- Rejected powered candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-no-mmio-control-pi5/control-rerun-after-preflight-fix/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-no-mmio-control-pi5/known-good-control-run/.
- Preflight mismatch:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-no-mmio-control-pi5/control-run/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 46,520-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 990 occurrences of
  TALOS: rp1-interrupt-routing-control were retained.
- known-good control and candidate rerun after rejected evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run tree.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-interrupt-routing-control-visible. The queued real RP1
interrupt-routing diagnostic proof is mechanically unblocked on a future worker
wake if hardwareTestLock remains unlocked/restored and supervisorIntervention
remains inactive.
