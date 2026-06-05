# Phase 11 RP1 Diagnostic Entry-Control Closeout

Task: `phase11-rp1-diagnostic-entry-control-closeout-20260605`

Status: accepted with staging/capture blocker; entry-control reachability and
RP1 mapped/read-value remain unaccepted.

## Goal

Reconcile the source/local entry-control discriminator and the serialized Pi 5
proof. This closeout records what was accepted, what remains blocked, and the
exact boundary for supervisor planning without broadening into GPIO ownership,
interrupts, DMA/cache policy, networking, SSH, storage, generated-root work,
Milestone 11.2, or a revised diagnostic shape.

## Outcome

The source-level task accepted a focused `rpi5_rp1_entry_control` candidate
that emits a unique marker/PASS path immediately after the normal Pi 5
`rust_entry` early-phase line and stops before BootInfo parsing,
`target::init`, normal RP1 GPIO/pin flushes, boot reports, memory planning,
or the RP1 UART0 FR read diagnostic path.

The serialized Pi 5 proof published that exact candidate archive, but the
first candidate run, known-good control, and candidate rerun all produced empty
fresh TFTP deltas. Serial output in all three attempts reached Raspberry Pi
firmware output through `Boot mode: NETWORK`, but did not reach
`TALOS: kernel_main`, the entry-control marker/PASS, known-good PASS, or any
RP1 diagnostic classification. The accepted closeout boundary is therefore:

`staging-or-capture-blocker`

This is not evidence that the candidate kernel was fetched, that Rust entry or
entry-control is reachable, or that RP1 UART0 FR is mapped, unmapped, faulting,
or firmware-state dependent.

## Evidence Map

- source/local task:
  `tasks/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core.md`
  at commit `4460e80e211cf73e2bd7f2b45a8a8b59cf75ac77`.
- source comparison:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/source-script-image-comparison.md`.
- entry-control candidate identity:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/candidate-identity.txt`.
- marker review:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/marker-review.txt`.
- hardware proof/blocker:
  `tasks/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof.md`
  at commit `ad6f929aa88b319dc31c0a4d7c4b921ea055d20f`.
- proof summary:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/proof-summary.txt`.
- candidate identity:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/candidate-identity.txt`.
- TFTP blocker evidence:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/candidate-tftp-delta-followup-pre-restore.json`,
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/known-good-tftp-delta-followup.json`,
  and
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/candidate-rerun-tftp-delta-followup-pre-restore.json`.
- serial blocker/control evidence:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/candidate-serial-observe-followup.json`,
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/known-good-serial-observe-followup.json`,
  and
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/candidate-rerun-serial-observe-final-pre-restore.json`.
- restore evidence:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/final-restore.json`
  and
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-pi5-proof/lab-status-after-restore.json`.
- closeout inspection:
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-closeout/static-evidence-inspection.md`.

## Findings

- fixed: the entry-control candidate created a unique pre-BootInfo,
  no-RP1-MMIO marker/PASS path for handoff discrimination.
- fixed: the candidate and proof records preserve artifact identity across
  source/static review and publication.
- removed: quarantined raw assembly entry-provenance marker routing stayed
  absent.
- not-an-issue: the candidate image/header shape matches the accepted Pi 5
  arm64 Image contract, with `text_offset=0`, `header_image_size=51808`,
  `flags=12`, and `magic=ARMd`.
- not-an-issue: the proof restored the accepted boot tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
  and released `hardwareTestLock`.
- deferred: the hardware proof produced empty fresh TFTP deltas for candidate,
  known-good control, and candidate rerun attempts, so candidate fetch and
  handoff reachability remain unaccepted.
- deferred: no RP1 mapped/read-value, unmapped, trap, firmware-state, GPIO,
  interrupt, DMA/cache, networking, SSH, storage, generated-root, broader
  PCIe, or Milestone 11.2 claim is accepted.

## Validation

- static evidence inspection: passed; see
  `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-closeout/static-evidence-inspection.md`.
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
