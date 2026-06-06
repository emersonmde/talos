# Phase 11 RP1 UART0 FR Read Hold-Control Pi 5 Discriminator

Task id: phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator-20260606

Status: accepted

## Goal

Run one serialized Pi 5 discriminator for the accepted hold-control RP1 UART0
FR-read candidate and classify the hardware boundary without broadening Phase
11 scope.

## Scope

- Acquired the hardware lock before candidate publication.
- Checked the accepted core archive SHA-256 before publication.
- Published only the accepted hold-control RP1 UART0 FR-read candidate archive.
- Captured lab status, candidate publication identity, serial cursor evidence,
  TFTP cursor evidence, pre-restore TFTP evidence, restore evidence, and triage
  attempts.
- Restored the pre-run boot tree before hardware-lock release.

## Non-Goals

No source changes, GPIO/pin configuration, UART programming, RP1
clocks/resets, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe enumeration, Milestone 11.2, phase transition, or workaround
stack.

## Classification

capture-staging-blocked.

The accepted core archive SHA-256 matched
`e9ab45b6dd15e4e80395302a116fb8aa751d699c5b679e5b9cee22077059a9b2` before
publication. Candidate publication reported lab tree
`ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0`,
effective kernel `kernel_2712.img`, and the expected 46,320-byte
`da591740/kernel_2712.img`.

The hardware capture could not tie selected-candidate TFTP fetch evidence,
fresh serial cursor evidence, and observed serial bytes strongly enough for a
more specific RP1 classification. The main candidate run retained 222,783
direct-read serial bytes and 5,582 `TALOS: fr-hold-control-post-read-loop`
occurrences, but its stable same-cursor TFTP delta retained 13 restored-tree
events with two 104,136-byte `da591740/kernel_2712.img` fetches and zero
46,320-byte selected-candidate fetches. Because that serial evidence was not
candidate-tied, it does not accept mapped/read-value behavior.

Required triage was attempted. The known-good control produced stable
same-cursor TFTP with zero events and no useful serial tie. The candidate rerun
again published the selected 46,320-byte kernel, but same-cursor TFTP returned
zero events and the serial observe file remained empty. The lab was restored to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` before
hardware-lock release.

Accepted claims are limited to capture-staging-blocked. RP1 UART0 FR
mapped/read-value behavior, bus-fault/trap behavior,
pre-read-control-visible-without-read-result, candidate-fetch-without-control-
marker, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, and phase
transition remain unaccepted.

## Findings And Disposition

- fixed: acquired hardwareTestLock before publication and restored the lab
  before release.
- fixed: checked the selected archive SHA-256 against accepted core evidence
  before publication.
- fixed: retained candidate publication identity showing effective
  `kernel_2712.img` and the expected 46,320-byte selected kernel.
- fixed: retained stable same-cursor TFTP evidence before restore for the main
  candidate run.
- fixed: performed required triage after the main run could not tie serial
  evidence to selected-candidate TFTP fetch evidence.
- deferred: a decisive RP1 FR-read hardware classification still needs a
  capture path that ties selected-candidate fetch, serial cursor, and marker or
  trap output in the same run.
- removed: no mapped/read-value or trap claim is made from post-read-loop
  serial text that lacked selected-candidate TFTP fetch evidence.
- not-an-issue: the first no-power preflight rejected repo boot-tree identity
  as a strict lab `tree_hash`; prior accepted publications also recorded a
  different lab-selected tree hash, so the rerun recorded the lab-selected tree
  as publication identity instead.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/evidence-map.json`.
- Classification:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/classification.json`.
- Main candidate run:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/candidate-run/`.
- Known-good control:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/known-good-control-run/`.
- Candidate rerun:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/candidate-rerun/`.
- Preflight no-power mismatch record:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/preflight-tree-hash-mismatch-no-power/`.

## Validation

- static archive identity check: passed against accepted core SHA-256.
- lab-controller API hardware run: completed with capture-staging-blocked.
- stable same-cursor TFTP evidence: retained, but not candidate-tied in the
  decisive run.
- fresh serial capture: retained, but not candidate-tied in the decisive run.
- known-good control and candidate rerun: attempted because candidate
  fetch/control-marker evidence was inconclusive.
- restore proof: passed; post-restore tree hash was
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as capture-staging-blocked. The queued closeout should reconcile this
blocker evidence and name the smallest non-repetitive next discriminator; no
Phase 11 feature expansion is accepted by this task.
