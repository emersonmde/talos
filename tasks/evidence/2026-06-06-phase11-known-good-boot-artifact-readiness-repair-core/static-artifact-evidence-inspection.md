# Static Artifact/Evidence Inspection

Task id: phase11-known-good-boot-artifact-readiness-repair-core-20260606

Level: static source/artifact/evidence inspection + lab-controller API read +
archive static review.

## Inputs Inspected

- Accepted lineage map:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-lineage-and-cursor-repair/lineage-map.json`.
- Latest direct-cursor blocker:
  `tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-pi5-recheck/observed-summary.json`.
- Current lab status read:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/lab-status-read.json`.
- Current lab boot files read:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/boot-files-read.json`.
- Local production-timer archive review:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/production-timer-archive-review.log`.
- Local artifact and marker inspection:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/local-artifact-inspection.log`.
- Direct-cursor versus prior successful serial comparison:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/serial-comparison.log`.

## Observations

- Restored/current lab boot identity still reports tree hash
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`,
  `configured_kernel=kernel_2712.img`, and
  `effective_kernel=kernel_2712.img`.
- The selected prefixed kernel path remains
  `da591740/kernel_2712.img` at 104,136 bytes, matching the accepted
  direct-cursor fetch evidence and the prior same-tree readiness evidence.
- The local production-timer boot archive passes static review: 19 files,
  both root and serial-prefixed mirrors, `kernel_2712.img` selected in
  config, matching `kernel8.img`, ARM64 Image header size equal to file size,
  `text_offset=0`, flags `12`, and `ARMd` header magic.
- Local marker inspection confirms the production-timer image contains
  `TALOS: asm_start` and `rpi5-production-timer-preemption: PASS`
  strings. The same image family has prior hardware PASS evidence on the
  restored tree.
- Latest direct-cursor serial stopped after Raspberry Pi firmware/RP1 output
  and did not include `TALOS: kernel_main` or
  `rpi5-production-timer-preemption: PASS`.
- Prior accepted same-tree serial, after the same RP1 firmware output class,
  reached `TALOS: kernel_main` and
  `rpi5-production-timer-preemption: PASS`.

## Classification

No actionable source/artifact/staging defect was found by this no-hardware
inspection. The latest blocker remains a runtime/lab discriminator: the
known-good kernel was fetched, but that run did not reach Talos runtime
readiness markers on serial.

The next smallest useful discriminator is supervisor-planned serialized
known-good runtime rerun or lab firmware/serial observation discrimination.
RP1 candidate/source work remains blocked until valid known-good Talos
readiness is accepted again.
