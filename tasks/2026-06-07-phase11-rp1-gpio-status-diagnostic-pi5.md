# Phase 11 RP1 GPIO Status Diagnostic Pi 5

Task id: phase11-rp1-gpio-status-diagnostic-pi5-20260607

Status: accepted

## Goal

Run the first serialized Pi 5 proof for the accepted RP1 GPIO14 STATUS
diagnostic after the no-MMIO control output shape was proven capturable.

## Scope

- Acquired hardwareTestLock for the serialized real diagnostic hardware proof.
- Checked and published only the accepted real diagnostic archive:
  target/talos-rpi5-rp1-gpio14-status-read-core.tar.gz.
- Retained candidate identity, publication identity, serial/TFTP cursors,
  serial captures, stable same-cursor TFTP evidence when available, final
  pre-restore identity, restore evidence, and triage records.
- Performed the standard inconclusive-run triage before any code changes:
  candidate identity, fresh serial cursor, TFTP delta, known-good control, and
  candidate rerun.

## Non-Goals

No uncontracted GPIO/pin-control writes, clock/reset programming, interrupt
enable/handling beyond the accepted read-only diagnostic, DMA/cache work,
storage, generated-root, networking, SSH, broader PCIe enumeration, Milestone
11.3, or phase transition.

## Classification

Accepted as capture-staging-blocked.

The real diagnostic archive SHA-256 matched the accepted core evidence:
7bc21b39a5d0150221a244701285d733c8faef4e153085a49a34b5069c1fecea. Lab
publication selected boot tree cb7827b07a3822370fc610dfd18a8ab580cea31a47c4559e41a242975976f83a
with effective kernel_2712.img and a 46,336-byte da591740/kernel_2712.img.

The first selected-tree rerun captured 483 occurrences of
TALOS: gpio14-status-result, but the v2 identity join rejected decisive
classification because pre-power serial drain was not empty, TFTP fetches were
104,136-byte known-good fetches rather than the 46,336-byte selected candidate,
and final pre-restore identity did not remain tied to the candidate.

The required triage then ran a known-good control and a candidate rerun. Both
clean-drain direct runs saw firmware-only serial and no stable TFTP events from
their saved cursors. The final candidate rerun kept the selected candidate
staged through final pre-restore identity, then restored the pre-run tree, but
it had no candidate-tied TFTP fetch and no diagnostic marker. This blocks
accepting RP1 GPIO14 STATUS behavior; it does not justify a code change by
itself.

Accepted claims remain limited to capture-staging-blocked evidence for this
diagnostic proof attempt. Real RP1 GPIO14 STATUS behavior, interrupt delivery,
clock/reset programming, GPIO ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe behavior, and phase transition remain unaccepted.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around the serialized Pi 5
  diagnostic proof.
- fixed: retained static archive identity for the accepted real diagnostic
  archive, including archive SHA-256 and selected kernel size.
- fixed: retained the initial publication/preflight mismatch caused by using
  the local archive tree hash as a lab-selected tree hash; restored before any
  power-cycle and reran using the lab-selected tree identity.
- fixed: retained the selected-tree rerun as capture-staging-blocked evidence;
  it showed diagnostic marker text, but failed the v2 identity join because
  serial drain and TFTP/final identity were not tied to the selected candidate.
- fixed: ran the required known-good control after the inconclusive candidate
  evidence; it had empty pre-power serial drain but only firmware serial and no
  stable TFTP events from the saved cursor.
- fixed: reran the selected candidate after the known-good control; it had empty
  pre-power serial drain and retained final selected-tree identity before
  restore, but had no diagnostic marker and no candidate TFTP events.
- deferred: supervisor closeout must decide the next qualitatively different
  diagnostic or capture/staging repair; this task does not authorize a same
  shaped rerun or feature expansion.
- not-an-issue: marker text from a rejected identity join is evidence, not
  accepted GPIO14 STATUS behavior.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-pi5/classification.json.
- Initial lab tree preflight mismatch:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-pi5/diagnostic-run/.
- Selected-tree candidate run rejected by identity join:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-pi5/diagnostic-rerun-selected-tree/.
- Known-good triage control:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-pi5/known-good-control-direct-run/.
- Candidate rerun after known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-pi5/diagnostic-rerun-after-kg/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: capture-staging-blocked.
- pi5-capture-transaction-v2 identity join: rejected the marker-visible run
  because selected-candidate identity was not strong enough.
- stable same-cursor TFTP evidence before restore: retained; final candidate
  rerun had stable zero-event TFTP from the saved cursor.
- fresh serial hardware output: retained; decisive candidate rerun saw
  firmware-only output with no diagnostic marker.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- git diff --check: passed.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as capture-staging-blocked with committed blocker evidence. The queued
phase11-rp1-irq-clock-gpio-diagnostic-closeout-20260607 closeout is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored.
