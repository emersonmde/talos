# Phase 11 RP1 UART0 FR-Read Delayed-Marker Pi 5 Discriminator

Task id: phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator-20260606

Status: completed

## Goal

Run one serialized Pi 5 discriminator for the accepted delayed-marker RP1
UART0 FR candidate and classify the volatile-load boundary without broadening
Phase 11.

## Scope

- Acquired the hardware lock before candidate publication.
- Published only the archive accepted by
  phase11-rp1-uart0-fr-read-delayed-marker-core-20260606.
- Captured candidate identity, lab status, publication response, fresh serial
  cursor, stable pre-restore TFTP evidence, repaired saturated-cursor direct
  serial evidence, restore evidence, and lock-release-ready state.
- Because the first candidate run fetched the candidate but did not show the
  final pre-load marker, retained the required inconclusive-run triage:
  candidate identity, fresh serial cursor, TFTP delta, known-good control, and
  candidate rerun before any code change or follow-up task.

## Non-Goals

No source changes, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, phase transition, address
constant change, UART0 configuration, or workaround stack.

## Classification

candidate-fetch-without-final-preload-marker.

The accepted candidate archive SHA-256 was
`90452242f872eb085c9fe7963c02ad67556694326daebd7d199caf4ed5f597f4`. The
published candidate tree was
`e9cd5c4a9571cab464ee76c046a7c4a2f42ba9cf75bb91f55de931dba16a3e2a` with a
46,152-byte `da591740/kernel_2712.img` and effective kernel
`kernel_2712.img`.

The first candidate run retained stable pre-restore TFTP evidence with 13
events, including two candidate `da591740/kernel_2712.img` fetches. Direct
serial read from saturated cursor `4194304` retained 5,039 bytes and visible
firmware NETWORK output, but did not show `TALOS: fr-delayed-preload-loop`,
the final pre-load marker, post-load output, mapped/read-value classification,
or trap/panic output.

The required triage retained a known-good control and candidate rerun. The
known-good control retained stable TFTP evidence and 6,575 direct-read serial
bytes with firmware NETWORK output, but no `TALOS: kernel_main`. The
candidate rerun republished the same accepted candidate shape, retained stable
TFTP evidence with two candidate kernel fetches, and captured 4,479 direct-read
serial bytes with firmware NETWORK output but no delayed-marker, final
pre-load marker, mapped/read-value, or trap/panic output.

This classification accepts only candidate publication/fetch evidence and the
absence of the final pre-load marker in the retained serial windows. It does
not accept RP1 mapped/read-value behavior, trap/unmapped behavior,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, or a phase transition.

## Findings And Disposition

- fixed: the pre-hardware archive SHA-256 matched the accepted core candidate.
- fixed: publication identity recorded effective kernel `kernel_2712.img`,
  the selected staged tree, and the expected 46,152-byte
  `da591740/kernel_2712.img`.
- fixed: the first candidate run retained stable same-cursor TFTP evidence
  before restore with two selected candidate kernel fetches.
- fixed: repaired saturated-cursor direct serial capture retained firmware
  reboot output for the first candidate run, known-good control, and candidate
  rerun.
- fixed: inconclusive-run triage was completed before any code change or
  follow-up rerun beyond the required triage sequence.
- fixed: the lab was restored to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
  before hardware-lock release.
- removed: no RP1 mapped/read-value or trap/unmapped claim is made because no
  final pre-load marker, post-load output, or trap output was visible.
- deferred: the next non-repetitive discriminator belongs in the closeout; do
  not run another same-shaped FR-read candidate rerun from this task.
- not-an-issue: `GET /` is not the deployed lab identity endpoint; retained
  proof identity uses `GET /status` and `GET /boot/files`.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/evidence-map.json`.
- Classification:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/classification.json`.
- First candidate run:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/candidate-run/`.
- Known-good control:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/known-good-control-run/`.
- Candidate rerun:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/candidate-rerun/`.
- Validation summary:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-pi5-discriminator/validation-summary.txt`.

## Validation

- static archive identity check: passed against accepted core SHA-256.
- lab-controller API hardware run: completed with blocker classification
  `candidate-fetch-without-final-preload-marker`.
- stable same-cursor TFTP evidence: passed for the first candidate run and
  candidate rerun.
- serial hardware boot/output: completed; repaired saturated-cursor direct-read
  captured firmware output but no final pre-load marker.
- known-good control and candidate rerun: completed as required by the
  inconclusive marker evidence.
- restore proof: passed; post-restore tree hash was
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- git diff --check: passed.
- mdbook build: passed because docs/src changed.
- git diff --cached --check before commit: passed.

## Result

Completed with blocker evidence as candidate-fetch-without-final-preload-marker.
The queued closeout may reconcile the source/static and Pi 5 evidence before
any further Phase 11 expansion.
