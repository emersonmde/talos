# Phase 11 RP1 GIC-Visible Route Diagnostic Pi 5

Task id: phase11-rp1-gic-visible-route-diagnostic-pi5-20260607

Status: accepted

## Goal

Run the accepted real GIC-visible route diagnostic candidate on Pi 5 after the
paired control proof, retaining decisive identity-joined evidence or a blocker.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 real diagnostic work.
- Checked the accepted local/static real archive before publication:
  target/talos-rpi5-rp1-gic-visible-route-status-read-core.tar.gz.
- Published only the accepted real GIC-visible route diagnostic archive.
- Retained publication identity, fresh serial/TFTP cursors, serial capture,
  stable pre-restore TFTP evidence, final pre-restore identity, restore
  evidence, and v2 identity-join records.
- Performed standard inconclusive-run triage after the first powered candidate
  run was rejected by serial-drain and missing-TFTP-fetch evidence: candidate
  identity, fresh serial/TFTP evidence, known-good control, and candidate
  rerun.

## Non-Goals

No operation outside the source-contract diagnostic boundary, GIC enable
writes, IAR/EOIR acknowledgement, interrupt unmasking, ISR installation, RP1
writes, MSI-X enable/IACK writes, GPIO ownership, unplanned pin-control or pad
writes, unplanned clock/reset programming, broad interrupt handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Classification

Accepted as gic-route-status-visible.

The accepted candidate rerun selected boot tree
8ef75b3125c21d7025cff539f5004d7f6911af057c5523ce1610be46deecbbe4 with
effective kernel_2712.img and a 47,816-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: pre-power serial drain was
empty, stable pre-restore TFTP retained two served 47,816-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
capture retained 209 occurrences of TALOS: rp1-gic-route-status-result.

The visible result reported contract
phase11-rp1-gic-visible-route-source-contract-v1, target
rp1-io-bank0-gic-route-status-read, hwirq 0, predicted MSI-X vector 0,
predicted GIC SPI 128 / INTID 160, GICD base 0x107fff9000, GICC base
0x107fffa000, bank 5, bit mask 0x1, GICD_ISENABLER5 raw 0x0,
GICD_ISPENDR5 raw 0x0, GICD_ISACTIVER5 raw 0x0, INTID 160 enabled=false,
pending=false, active=false, GICC_HPPIR raw 0x3ff, hppir-intid 1023,
hppir-spurious=true, hppir-target-match=false, and
classification=gic-route-status-visible.

This accepts only the selected read-only/no-ack GIC-visible status boundary.
It does not accept interrupt pending generation, interrupt delivery, IAR/EOIR
acknowledgement, ISR or handler ownership, GPIO ownership, pin-control
behavior, GIC/RP1/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock/reset writes,
DMA/cache behavior, storage or generated-root behavior, networking, SSH,
broader PCIe behavior, Milestone 11.3, or phase transition.

The capture helper restored its pre-run snapshot after each powered run,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  diagnostic work.
- fixed: retained static archive identity for the accepted real diagnostic
  archive, including archive SHA-256, kernel SHA-256, marker string, and
  expected kernel size.
- fixed: retained the first candidate run as inconclusive evidence; it showed
  visible result output but was rejected by non-empty pre-power serial drain
  and missing same-cursor TFTP fetch evidence.
- fixed: ran the required known-good control after the inconclusive first
  candidate run; it passed the v2 identity join with two 104,136-byte
  production-timer candidate fetches.
- fixed: reran the selected real diagnostic after the known-good control; the
  rerun passed the v2 identity join and retained repeated GIC-visible route
  status output.
- deferred: interrupt delivery, IAR/EOIR acknowledgement, ISR/handler
  ownership, GPIO ownership, clock/reset programming, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
  and phase transition.
- not-an-issue: HPPIR returning 1023/spurious with enable/pending/active bits
  clear is still a valid read-only/no-ack GIC-visible status snapshot; it is
  not proof of delivered interrupts.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-pi5/diagnostic-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-pi5/diagnostic-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-pi5/known-good-control-run/.
- Operator staging-mismatch record:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-pi5/diagnostic-run-staging-mismatch-operator/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,816-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 209 occurrences of
  TALOS: rp1-gic-route-status-result were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as gic-route-status-visible. The accepted boundary is a read-only
GICv2 status snapshot for the source-predicted RP1 IO_BANK0 route to GIC SPI
128 / INTID 160. It does not accept interrupt delivery or handler ownership.
