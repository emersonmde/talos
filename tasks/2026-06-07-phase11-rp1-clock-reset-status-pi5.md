# Phase 11 RP1 Clock/Reset Status Pi 5

Task id: phase11-rp1-clock-reset-status-pi5-20260607

Status: accepted

## Goal

Run the accepted real read-only RP1 clock manager status diagnostic on Pi 5
after the paired no-MMIO/no-RP1/no-GIC control proof, retaining decisive
identity-joined evidence or a blocker.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 hardware work.
- Published only the accepted real RP1 clock manager status archive:
  target/talos-rpi5-rp1-clock-manager-status-read-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial capture, stable pre-restore TFTP evidence, final
  pre-restore identity, restore evidence, and v2 identity-join records.
- Performed the standard inconclusive-run triage after the first candidate run
  was rejected by non-empty pre-power serial-drain evidence: candidate
  identity, fresh serial/TFTP evidence, known-good/control run, and candidate
  rerun without code changes.
- Restored the lab to the original pre-run boot tree after the accepted rerun.

## Non-Goals

No RP1 clock/reset ownership or writes, GPIO ownership, GPIO event generation,
interrupt delivery, GIC acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
phase transition, or runtime behavior beyond the selected read-only RP1 clock
manager status boundary.

## Classification

Accepted as rp1-clock-manager-status-visible.

The accepted candidate rerun selected boot tree
3e64059ed440eaf48f096d8e2e4113609dbfe9f78444955003547515439c3704 with
effective kernel_2712.img and a 47,280-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: pre-power serial drain reached
empty on the rerun, stable pre-restore TFTP retained two served 47,280-byte
candidate fetches, final pre-restore identity still matched the selected tree,
and serial capture retained 320 occurrences of
TALOS: rp1-clock-manager-status-result.

The retained result markers report
classification=rp1-clock-manager-status-visible, pll-sys-lock=true,
clk-sys-enabled=true, and clk-uart-enabled=true. This accepts only the
read-only RP1 clock manager status snapshot selected by the source contract. It
does not accept RP1 clock/reset ownership, clock/reset writes, GPIO ownership,
GPIO event generation, interrupt delivery, GIC acknowledgement, ISR/handler
ownership, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
behavior, Milestone 11.3, or phase transition.

The capture helper restored the rerun snapshot, which was taken after
publication; the worker then restored the original pre-run snapshot
pre-clock-reset-status-real-184321 and retained a final lab status showing the
lab returned to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  diagnostic work.
- fixed: retained static archive identity for the accepted real clock manager
  status archive, including archive SHA-256, kernel SHA-256, marker string, and
  forbidden control string absence.
- fixed: retained the first candidate run as capture-staging-blocked evidence;
  it had candidate-tied fetches and real result markers but was rejected by
  non-empty pre-power serial-drain evidence.
- fixed: ran the required known-good/control capture after the inconclusive
  first candidate run, then reran the same real candidate without code changes.
- fixed: accepted the identity-joined candidate rerun as the bounded read-only
  RP1 clock manager status snapshot.
- fixed: restored the lab to the original pre-run boot tree after the accepted
  rerun because the rerun helper snapshot was taken after candidate
  publication.
- deferred: any future clock/reset write or ownership contract requires
  supervisor planning after closeout.
- not-an-issue: the accepted result is read-only status evidence, not
  clock/reset ownership or write capability evidence.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/real-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/real-run/.
- Known-good/control run:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/known-good-control-run/.
- Final original restore proof:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/final-post-restore-original-status.json.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,280-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 320 occurrences of
  TALOS: rp1-clock-manager-status-result were retained.
- inconclusive-run triage: completed; first candidate run was rejected by
  serial-drain freshness, known-good/control was retained, and the candidate
  rerun passed identity join.
- final original restore proof: passed; the lab returned to the original
  pre-run tree after the accepted rerun.
- git diff --check: passed.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as rp1-clock-manager-status-visible. The queued clock/reset/status
closeout task is mechanically unblocked on a future worker wake if
hardwareTestLock remains unlocked/restored and supervisorIntervention remains
inactive.
