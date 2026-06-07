# Phase 11 RP1 Clock ADC Enable Toggle Pi 5

Task id: phase11-rp1-clock-adc-enable-toggle-pi5-20260607

Status: accepted

Classification: rp1-clock-adc-ctrl-enable-toggle-mismatch-restored

## Goal

Run the accepted real reversible CLK_ADC_CTRL enable-bit transition diagnostic
on Pi 5 as the smallest non-idempotent clock-manager ownership proof for the
selected target.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 real diagnostic work.
- Checked the accepted local/static real archive before publication:
  target/talos-rpi5-rp1-clock-adc-ctrl-enable-toggle-core.tar.gz.
- Published only the accepted real CLK_ADC_CTRL enable-bit transition archive.
- Retained candidate identity, fresh serial/TFTP cursors, serial capture, stable
  pre-restore TFTP evidence, final pre-restore identity, restore evidence, and
  pi5-capture-transaction-v2 identity join.
- Performed standard inconclusive-run triage after the first powered candidate
  run was rejected by serial/TFTP freshness evidence: retained candidate
  identity, fresh serial/TFTP evidence, known-good control evidence, and
  candidate rerun evidence.

## Non-Goals

No uncontracted clock/reset writes, reset-controller writes, GPIO/RIO/pad
writes, GPIO ownership, event generation, interrupt enablement or delivery,
GIC acknowledgement, ISR installation, broad clock/reset driver ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Classification

Accepted as rp1-clock-adc-ctrl-enable-toggle-mismatch-restored.

The accepted candidate rerun selected boot tree
7024bb54a9446c681d4a8b9c80372fe52a4d4f93b7939f299a8eb2d7199a697a with
effective kernel_2712.img and a 47,512-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: stable pre-restore TFTP retained
two served 47,512-byte candidate fetches, final pre-restore identity still
matched the selected tree, and the capture retained 78 occurrences of
TALOS: rp1-clock-adc-ctrl-enable-toggle-result.

The retained diagnostic output reported:

- register: CLK_ADC_CTRL at 0x1f00018144, width 32.
- transition mask: 0x800.
- pre-raw: 0xdeaddead, pre-enable=true, pre-auxsrc=0x15, pre-source=0x1.
- transition-raw: 0xdeadd6ad.
- post-raw: 0xdeaddead, post-enable=true, pre/post auxsrc=0x15,
  post-source=0x1.
- restore-raw: 0xdeaddead, restore-enable=true, restore-auxsrc=0x15,
  restore-source=0x1.
- one-bit-transition=true, post-enable-flipped=false,
  post-delta-is-transition-mask=false, and restore-eq-pre=true.
- retained GPIO14/GPIO16 blocker context remains fsel13.
- terminal classification:
  rp1-clock-adc-ctrl-enable-toggle-mismatch-restored.

This accepts only the selected CLK_ADC_CTRL enable-bit transition attempt,
identity-joined hardware output, and restore evidence. It does not accept a
successful non-idempotent enable-bit transition because the post-read value did
not reflect the requested 0x800 bit change. It also does not accept broad RP1
clock/reset ownership, reset-controller writes, GPIO ownership, event
generation, interrupt delivery, GIC acknowledgement, ISR/handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe behavior,
Milestone 11.3, or phase transition.

The capture helper restored the candidate snapshot after the accepted rerun,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  diagnostic work.
- fixed: retained static archive identity for the accepted real archive,
  including archive SHA-256, kernel SHA-256, marker string, and report-shape
  checks.
- fixed: retained the first candidate run as capture-staging-blocked evidence;
  it was rejected by non-empty pre-power serial drain, saturated direct-read
  freshness rules, and missing candidate TFTP fetch evidence.
- fixed: ran the required known-good control after the inconclusive first
  candidate run; it passed the v2 identity join with the production timer PASS
  marker and two 104,136-byte TFTP fetches.
- fixed: reran the selected real candidate after the known-good control; the
  rerun passed the v2 identity join and retained repeated CLK_ADC_CTRL
  enable-toggle output.
- deferred: a successful non-idempotent CLK_ADC_CTRL transition, broad RP1
  clock/reset ownership, reset-controller ownership, GPIO ownership retries,
  event generation, interrupt delivery, handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
  and phase transition.
- not-an-issue: the observed 0xdeaddead value is retained only as the raw value
  for this selected proof and blocker; no broader clock/reset state
  interpretation is inferred from it.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-pi5/real-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-pi5/real-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-pi5/known-good-control-run/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,512-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 78 occurrences of
  TALOS: rp1-clock-adc-ctrl-enable-toggle-result were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as rp1-clock-adc-ctrl-enable-toggle-mismatch-restored. The queued
closeout task is mechanically unblocked on a future worker wake if
hardwareTestLock remains unlocked/restored and supervisorIntervention remains
inactive.
