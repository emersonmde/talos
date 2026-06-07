# Phase 11 RP1 Interrupt-Routing Diagnostic Pi 5

Task id: phase11-rp1-interrupt-routing-diagnostic-pi5-20260607

Status: accepted

## Goal

Run the accepted real interrupt-routing diagnostic candidate on Pi 5 after the
paired no-MMIO/no-enable control proof, retaining decisive identity-joined
evidence or a blocker.

## Scope

- Acquired hardwareTestLock for the serialized real diagnostic run.
- Checked the accepted local/static diagnostic archive before publication:
  target/talos-rpi5-rp1-interrupt-routing-msix-cfg-read-core.tar.gz.
- Published only the accepted real interrupt-routing diagnostic archive.
- Retained publication identity, fresh serial/TFTP cursors, serial capture,
  stable pre-restore TFTP evidence, final pre-restore identity, restore
  evidence, and v2 identity-join records.
- Performed standard inconclusive-run triage after the first powered candidate
  run was rejected by non-empty pre-power serial drain evidence: candidate
  identity, fresh serial/TFTP evidence, known-good control, and candidate
  rerun.

## Non-Goals

No operation outside the accepted source-contract diagnostic boundary, GPIO
ownership, pin-control writes, pad writes, clock/reset programming, broad
interrupt handler ownership, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe enumeration, Milestone 11.3, phase transition, or acceptance
of interrupt delivery.

## Classification

Accepted as routing-msix-cfg-visible.

The accepted candidate rerun selected boot tree
`63800845c9837b3d57153051583b269070b028412bcd57ea9c55a5f9e56a2304` with
effective `kernel_2712.img` and a 46,648-byte
`da591740/kernel_2712.img`. The v2 identity join passed with no rejection
reasons: pre-power serial drain was empty, stable pre-restore TFTP retained two
served 46,648-byte candidate fetches, final pre-restore identity still matched
the selected tree, and the capture retained 970 occurrences of
`TALOS: rp1-interrupt-routing-result`.

The retained diagnostic output reported contract
`phase11-rp1-interrupt-routing-source-contract-v1`, target
`rp1-io-bank0-msix-cfg-read`, hwirq 0, predicted MSI-X vector 0, predicted
GIC SPI 128 / INTID 160, address `0x1f00108008`, width 32, raw
`0xdeaddead`, enable=true, test=false, iack=true, iack-en=true, and
classification=routing-msix-cfg-visible.

This accepts only the selected read-only/no-enable MSIX_CFG(0) diagnostic
boundary and its Pi 5 visibility. It does not accept interrupt delivery,
handler ownership, GPIO ownership, pin-control behavior, pad writes,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe behavior, Milestone 11.3, or a phase transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5
  diagnostic work.
- fixed: retained static archive identity for the accepted real diagnostic
  archive, including archive SHA-256, kernel size, and result marker strings.
- fixed: retained the first powered diagnostic run as capture-staging-blocked
  evidence; it had two 46,648-byte candidate fetches and 969 result markers but
  failed the v2 identity join because pre-power serial drain was not empty.
- fixed: ran the required known-good control after the rejected diagnostic run;
  it retained the production timer PASS marker and preserved the serial-drain
  rejection evidence.
- fixed: reran the selected real diagnostic after the known-good control; the
  rerun passed the v2 identity join and retained repeated interrupt-routing
  result output.
- fixed: restored the lab to the pre-run boot tree after each capture bundle.
- deferred: interpreting the returned MSIX_CFG value as real interrupt
  delivery, GPIO ownership, or clock/reset ownership remains future work.
- not-an-issue: the raw value `0xdeaddead` is accepted only as the observed
  read result for this diagnostic boundary, not as proof that interrupts are
  enabled or delivered.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-pi5/diagnostic-rerun-after-kg/.
- Rejected first diagnostic run:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-pi5/diagnostic-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-pi5/known-good-control-run/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 46,648-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 970 occurrences of
  `TALOS: rp1-interrupt-routing-result` were retained.
- known-good control and candidate rerun after rejected evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run tree.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as routing-msix-cfg-visible. The queued interrupt-routing closeout is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored and supervisorIntervention remains inactive.
