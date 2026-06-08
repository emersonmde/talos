# Phase 11 RP1 Clock Write-Effect Discriminator Control Pi 5

Task id: phase11-rp1-clock-write-effect-discriminator-control-pi5-20260607

Status: accepted

Classification: no-mmio-clock-adc-window-coherence-control-visible

## Goal

Run the paired no-MMIO/no-RP1/no-GIC ADC clock-window coherence control
candidate on Pi 5 before any real read-only ADC clock-window coherence proof.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Published only the accepted no-MMIO/no-RP1/no-GIC control archive:
  target/talos-rpi5-rp1-clock-adc-window-coherence-no-mmio-control-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after the first powered
  candidate run was rejected by capture/staging evidence: candidate identity,
  fresh serial/TFTP evidence, known-good control, and unchanged candidate rerun.

## Non-Goals

No real ADC clock-window coherence candidate, RP1 clock/reset read or write,
GPIO/RIO/pad access, event generation, interrupt enablement or delivery, GIC
IAR/EOIR acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
phase transition, or RP1 clock/reset hardware behavior acceptance.

## Classification

Accepted as no-mmio-clock-adc-window-coherence-control-visible.

The accepted candidate rerun selected boot tree
326db32f8082eb83f24df752d81611b77a2a270ff539a7af27adb91b0ef89412 with
effective kernel_2712.img and a 47,360-byte da591740/kernel_2712.img. The
v2 identity join passed with no rejection reasons: pre-power serial drain was
empty, stable pre-restore TFTP retained two served 47,360-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
serial capture retained 52 occurrences of
TALOS: rp1-clock-adc-window-coherence-control.

The retained control output classification remains
no-mmio-clock-adc-window-coherence-control-visible. This accepts only the
no-MMIO/no-RP1/no-GIC output shape and capture path for the queued real ADC
clock-window coherence proof. Real RP1 clock/reset behavior, GPIO ownership,
event generation, interrupt delivery, GIC acknowledgement, ISR/handler
ownership, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
behavior, Milestone 11.3, and phase transition remain unaccepted.

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
  it had one control marker after a saturated direct-read cursor but was
  rejected by non-empty pre-power serial drain evidence and missing same-run
  candidate TFTP fetches.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate run; it passed the v2 identity join, retained two
  served 104,136-byte known-good kernel fetches, and reached its PASS marker.
- fixed: reran the same no-MMIO/no-RP1/no-GIC control candidate after the
  known-good control; the rerun passed the v2 identity join and retained
  repeated ADC clock-window coherence control output.
- deferred: the real ADC clock-window coherence proof remains queued and must
  pass its own hardware lock, identity join, restore, and classification gates.
- not-an-issue: no RP1 clock/reset behavior is inferred from a no-MMIO
  simulated control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-control-pi5/control-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-control-pi5/control-rerun-after-manual-restore/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-control-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,360-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 52 occurrences of
  TALOS: rp1-clock-adc-window-coherence-control were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-clock-adc-window-coherence-control-visible. The queued real
ADC clock-window coherence proof is mechanically unblocked on a future worker
wake if hardwareTestLock remains unlocked/restored and supervisorIntervention
remains inactive.
