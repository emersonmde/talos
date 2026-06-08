# Phase 11 RP1 Clock Write-Effect Discriminator Pi 5

Task id: phase11-rp1-clock-write-effect-discriminator-pi5-20260607

Status: accepted

Classification: rp1-clock-adc-window-readback-sentinel

## Goal

Run the accepted real RP1 ADC clock-window coherence discriminator on Pi 5
after the paired no-MMIO/no-RP1/no-GIC control proof, accepting only the
bounded result or a precise blocker.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 real diagnostic work.
- Published only the accepted real archive:
  target/talos-rpi5-rp1-clock-adc-window-coherence-read-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after the first capture was not
  acceptable: retained candidate identity and TFTP/serial evidence, ran a
  known-good production-timer control, and reran the unchanged real candidate.

## Non-Goals

No uncontracted clock/reset writes, reset-controller writes, GPIO/RIO/pad
writes, event generation, interrupt enablement or delivery, GIC IAR/EOIR
acknowledgement, ISR installation, broad clock/reset driver ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Classification

Accepted as rp1-clock-adc-window-readback-sentinel.

The accepted rerun selected boot tree
f93e47c1d5b68dd243c795d3323cc04249c0b62cda22c3ccb003593c56232902 with
effective kernel_2712.img and a 48,056-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: pre-power serial drain was
empty, stable pre-restore TFTP retained two served 48,056-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
serial capture retained 52 occurrences of
TALOS: rp1-clock-adc-window-coherence-result.

The retained diagnostic output reported the accepted read-only operation
sequence:

- CLK_SYS_CTRL at 0x1f00018014 raw=0xdeaddead.
- CLK_UART_CTRL at 0x1f00018054 raw=0xdeaddead.
- CLK_ADC_CTRL first read at 0x1f00018144 raw=0xdeaddead.
- local ordering barrier.
- CLK_ADC_CTRL second read at 0x1f00018144 raw=0xdeaddead.
- CLK_ADC_DIV_INT at 0x1f00018148 raw=0xdeaddead.
- CLK_ADC_SEL at 0x1f00018150 raw=0xdeaddead.

Decoded fields were clk-sys-enable=true, clk-uart-enable=true,
adc-ctrl-stable=true, adc-window-all-equal=true,
adc-window-all-deaddead=true, adc-sel-zero=false, adc-sel-one-hot=false, and
adc-sel-multi-bit=true. The retained enable-toggle context remained
pre_raw=0xdeaddead, transition_raw=0xdeadd6ad, post_raw=0xdeaddead,
restore_raw=0xdeaddead, restore-eq-pre=true.

This accepts only the selected read-only ADC clock-window coherence result,
identity-joined hardware output, and restore evidence. It does not accept
successful non-idempotent clock ownership, broad RP1 clock/reset ownership, any
new RP1 clock/reset write, GPIO ownership, event generation, interrupt
delivery, GIC acknowledgement, ISR/handler ownership, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe behavior, Milestone 11.3, or
phase transition.

The capture helper restored the candidate snapshot after the accepted rerun,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  diagnostic work.
- fixed: retained static archive identity for the accepted real archive,
  including archive SHA-256, kernel SHA-256, marker string, and report-shape
  checks.
- fixed: retained the first real run as capture-staging-blocked evidence; it
  had real-result serial output and matching candidate TFTP fetches, but the
  repeated capture attempt polluted preflight/selected-tree identity, so it was
  not accepted.
- fixed: ran the required known-good production-timer control after the
  inconclusive first capture; it passed the v2 identity join with two
  104,136-byte TFTP fetches and the PASS marker.
- fixed: reran the unchanged real candidate after the known-good control; the
  rerun passed the v2 identity join and retained repeated ADC clock-window
  coherence result output.
- deferred: successful non-idempotent clock ownership, broad RP1 clock/reset
  ownership, reset-controller ownership, GPIO ownership retries, event
  generation, interrupt delivery, handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
  and phase transition.
- not-an-issue: the observed 0xdeaddead values are retained only as the raw
  sentinel values for this selected proof and blocker; no broader clock/reset
  state interpretation is inferred from them.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-pi5/real-rerun-after-kg/.
- First candidate capture:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-pi5/real-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 48,056-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 52 occurrences of
  TALOS: rp1-clock-adc-window-coherence-result were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.

## Result

Accepted as rp1-clock-adc-window-readback-sentinel. The queued closeout task is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored and supervisorIntervention remains inactive.
