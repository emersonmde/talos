# Phase 11 RP1 Final-Preload-Marker Hold Pi 5 Discriminator

Task id: phase11-rp1-final-preload-marker-hold-pi5-discriminator-20260606

Status: accepted

## Goal

Run one serialized Pi 5 discriminator for the accepted final-preload-marker
hold candidate and classify marker visibility without touching RP1 MMIO.

## Scope

- Acquired the hardware lock before candidate publication.
- Published only the archive accepted by
  phase11-rp1-final-preload-marker-hold-core-20260606.
- Captured candidate identity, lab status, publication response, fresh serial
  cursor, stable pre-restore TFTP evidence, direct-read serial evidence,
  restore evidence, and lock-release-ready state.
- Restored the pre-run boot snapshot before hardware-lock release.

## Non-Goals

No source changes, RP1 UART0 FR volatile load, address-constant change, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, phase transition, or workaround stack.

## Classification

final-preload-hold-marker-visible.

The accepted candidate archive SHA-256 was
`07af64b86908f36c63d368589d79c76aebd492a81906a39586a2c5902d8b9287`. The
published candidate tree was
`101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47` with a
45,816-byte `da591740/kernel_2712.img` and effective kernel
`kernel_2712.img`.

Stable same-cursor TFTP evidence retained 13 events, including two served
45,816-byte `da591740/kernel_2712.img` candidate fetches. The serial cursor
was already saturated at `4194304`; direct `/serial/read` retained 57,040
bytes with 1,628 occurrences of the unique hold marker:
`TALOS: fr-final-preload-hold-loop`.

The direct-read window did not retain the earlier final pre-load marker, so the
accepted classification is specifically hold-marker visibility from the
selected no-RP1-MMIO candidate. It does not accept RP1 mapped/read-value
behavior, RP1 unmapped/trap behavior, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or a phase transition.

## Findings And Disposition

- fixed: the pre-hardware archive SHA-256 matched the accepted core candidate.
- fixed: publication identity recorded effective kernel `kernel_2712.img`,
  the selected staged tree, and the expected 45,816-byte
  `da591740/kernel_2712.img`.
- fixed: stable same-cursor TFTP evidence before restore retained two selected
  candidate kernel fetches.
- fixed: direct serial read from the saturated cursor retained visible unique
  hold-marker output from the selected no-RP1-MMIO candidate.
- fixed: the lab was restored to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
  before hardware-lock release.
- removed: no RP1 mapped/read-value, no-return/trap, or firmware-state claim
  is made from hold-marker visibility.
- deferred: returning to an actual RP1 UART0 FR read needs closeout planning
  and a new non-repetitive bounded discriminator.
- not-an-issue: the initial observe helper returned HTTP 22 with an empty file;
  the task-required repaired direct-read path retained the decisive serial
  marker evidence.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator/evidence-map.json`.
- Classification:
  `tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator/classification.json`.
- Candidate run:
  `tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator/candidate-run/`.
- Validation summary:
  `tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-pi5-discriminator/validation-summary.txt`.

## Validation

- static archive identity check: passed against accepted core SHA-256.
- lab-controller API hardware run: accepted with classification
  `final-preload-hold-marker-visible`.
- stable same-cursor TFTP evidence: passed.
- serial hardware boot/output: direct-read captured the unique hold marker.
- restore proof: passed; post-restore tree hash was
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as final-preload-hold-marker-visible. The queued closeout may
reconcile the source/static and Pi 5 marker-visibility evidence before any
return to an actual RP1 FR-read hardware discriminator.
