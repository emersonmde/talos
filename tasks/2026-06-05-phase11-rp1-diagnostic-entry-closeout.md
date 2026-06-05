# Phase 11 RP1 Diagnostic Entry Closeout

Task: `phase11-rp1-diagnostic-entry-closeout-20260605`

Status: accepted with hardware blocker; RP1 mapped/read-value remains
unaccepted.

## Goal

Reconcile the revised RP1 UART0 FR diagnostic entry proof after the source
handoff review and serialized Pi 5 run. This closeout records what was
accepted, what remains blocked, and the exact boundary for supervisor planning
without broadening into GPIO ownership, interrupts, DMA/cache policy,
networking, SSH, storage, Milestone 11.2, or a revised diagnostic shape.

## Outcome

The source-level handoff task accepted a revised diagnostic candidate that
prints `rpi5-rp1-uart0-fr-read: start` and
`rpi5-rp1-uart0-fr-read: pre-mmio-read` before the single RP1 UART0 flag
register read. The serialized Pi 5 proof then fetched that exact candidate
from TFTP, but serial output still did not reach Talos Rust entry, the
pre-MMIO marker, the diagnostic classification path, or PASS. The known-good
control on the restored accepted boot tree reached `TALOS: kernel_main` and
PASS, so the accepted closeout boundary is:

`blocked-pre-entry-or-handoff-after-candidate-fetch`

This is evidence that publication, TFTP serving, restore, and the serial
control path worked. It is not evidence that RP1 UART0 FR is mapped,
unmapped, faulting, or firmware-state dependent.

## Evidence Map

- source/handoff task:
  `tasks/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core.md`
  at commit `2c3530064f51b92f28900a63cd7911fc29de3477`.
- source comparison:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core/source-script-image-comparison.md`.
- revised candidate identity:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core/candidate-identity.txt`.
- hardware proof/blocker:
  `tasks/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof.md` at commit
  `d1aff42567e95708aa6fa346a7fe39d7d0eb0632`.
- proof summary:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof/proof-summary.txt`.
- TFTP proof:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof/candidate-rerun-tftp-delta-followup-pre-restore.json`.
- serial blocker:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof/candidate-rerun-serial-observe-final-pre-restore.json`.
- known-good control:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof/known-good-serial-observe-followup.json`.
- restore evidence:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof/final-restore.json`.
- closeout inspection:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-closeout/static-evidence-inspection.md`.

## Findings

- fixed: the revised candidate added a pre-MMIO serial discriminator before
  the single RP1 UART0 FR volatile read, preserving the narrow one-read
  contract.
- removed: raw assembly entry-provenance marker routing stayed quarantined
  because prior accepted evidence showed that path can break prompt-capable
  Pi 5 controls.
- not-an-issue: the candidate archive and boot tree were produced with the
  accepted Pi 5 image/boot-tree pattern and carried the expected static
  strings, section/header shape, and identity.
- not-an-issue: the serialized proof fetched the selected
  87,480-byte `da591740/kernel_2712.img` twice before restore.
- not-an-issue: the known-good control restored boot tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`,
  fetched the 104,136-byte control kernel, reached `TALOS: kernel_main`,
  and retained PASS output.
- deferred: the revised candidate still did not emit Talos entry,
  pre-MMIO, diagnostic classification, or PASS output after TFTP fetch, so a
  future source/handoff investigation must be supervisor-planned.
- deferred: no RP1 mapped/read-value, unmapped, trap, interrupt, GPIO,
  DMA/cache, networking, SSH, storage, or Milestone 11.2 claim is accepted.

## Validation

- static evidence inspection: passed; see
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-closeout/static-evidence-inspection.md`.
- diff hygiene: `git diff --check` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed with the
  existing large search-index warning.
- staged diff hygiene: `git diff --cached --check` passed before commit.

## Next Action

Request supervisor planning for the next bounded Phase 11 slice. The worker
must not infer source-level pre-entry/handoff changes, a revised RP1
diagnostic shape, Milestone 11.2, networking, SSH, GPIO ownership,
interrupts, DMA/cache policy, storage, generated-root work, or broader PCIe
work from this closeout.
