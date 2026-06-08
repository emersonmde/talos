# Phase 11 RP1 Clock Sentinel Address Discriminator Control Pi 5

Task id: phase11-rp1-clock-sentinel-address-discriminator-control-pi5-20260608

Status: accepted

Classification: no-mmio-sysinfo-clock-sentinel-control-visible

## Goal

Run the paired no-MMIO/no-RP1/no-GIC SYSINFO clock-sentinel control candidate
on Pi 5 before any real SYSINFO-vs-clock-window-sentinel hardware proof.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Published only the accepted no-MMIO/no-RP1/no-GIC control archive:
  target/talos-rpi5-rp1-sysinfo-clock-sentinel-no-mmio-control-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after the first powered
  candidate run was rejected by capture evidence: candidate identity, fresh
  serial/TFTP evidence, known-good control, and unchanged candidate rerun.

## Non-Goals

No real RP1 SYSINFO or clock-window read behavior, RP1 clock/reset writes,
GPIO/RIO/pad access, event generation, interrupt enablement or delivery, GIC
IAR/EOIR acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
phase transition, or RP1 clock/reset hardware behavior acceptance.

## Classification

Accepted as no-mmio-sysinfo-clock-sentinel-control-visible.

The accepted candidate rerun selected boot tree
499b836e2dfbd94d9301dfcb90d9625cd90e6e7507ba8070413ce8b36c5c551e with
effective kernel_2712.img and a 47,288-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
47,288-byte candidate fetches, final pre-restore identity still matched the
selected tree, and the serial capture retained 60 occurrences of
TALOS: rp1-sysinfo-clock-sentinel-control.

The retained control output classification remains
no-mmio-sysinfo-clock-sentinel-control-visible. This accepts only the
no-MMIO/no-RP1/no-GIC output shape and capture path for the queued real
SYSINFO-vs-clock-window-sentinel proof. Real RP1 SYSINFO or clock-window
hardware behavior, broad clock/reset ownership, GPIO ownership, event
generation, interrupt delivery, GIC acknowledgement, ISR/handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe behavior,
Milestone 11.3, and phase transition remain unaccepted.

The capture helper restored the candidate snapshot after the accepted rerun,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO/no-RP1/no-GIC
  control archive, including archive SHA-256, kernel SHA-256, marker string,
  report-shape strings, and forbidden real/RP1/GIC/MMIO strings absence.
- fixed: retained the first candidate run as capture-staging-blocked evidence;
  it had 60 control marker occurrences, coherent selected-tree/TFTP evidence,
  and restore proof, but was rejected by non-empty pre-power serial drain.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate run; it retained two served 104,136-byte known-good
  kernel fetches and reached its PASS marker, but was also rejected by
  non-empty pre-power serial drain.
- fixed: reran the same no-MMIO/no-RP1/no-GIC control candidate after the
  known-good control; the rerun passed the v2 identity join and retained
  repeated SYSINFO clock-sentinel control output.
- deferred: the real SYSINFO-vs-clock-window-sentinel proof remains queued and
  must pass its own hardware lock, identity join, restore, and classification
  gates.
- not-an-issue: no RP1 SYSINFO, clock-window, or clock/reset behavior is
  inferred from a no-MMIO simulated control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-control-pi5/control-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-control-pi5/control-run/.
- Known-good control:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-control-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,288-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 60 occurrences of
  TALOS: rp1-sysinfo-clock-sentinel-control were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-sysinfo-clock-sentinel-control-visible. The queued real
SYSINFO-vs-clock-window-sentinel proof is mechanically unblocked on a future
worker wake if hardwareTestLock remains unlocked/restored and
supervisorIntervention remains inactive.
