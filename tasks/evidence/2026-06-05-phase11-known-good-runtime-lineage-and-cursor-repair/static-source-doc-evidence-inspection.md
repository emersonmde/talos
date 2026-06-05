# Static Source/Doc/Evidence Inspection

Task id: phase11-known-good-runtime-lineage-and-cursor-repair-20260605

- fixed: `lab-status-before.json` and `boot-files-before.json` for the latest
  known-good runtime discriminator both identify restored tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`,
  `configured_kernel=kernel_2712.img`, `effective_kernel=kernel_2712.img`, and
  a 104,136-byte `da591740/kernel_2712.img`.
- fixed: the retained fresh cursor file contains `4095602`; stable replay from
  that cursor returned 13 events and two 104,136-byte kernel fetches.
- fixed: prior same-tree control evidence in the Phase 11 entry proof reached
  `TALOS: kernel_main` and `rpi5-production-timer-preemption: PASS`, so the
  restored tree is still expected to reach the accepted markers.
- fixed: the helper path now rejects missing or blank cursor data before a
  direct-cursor TFTP delta can be recorded.
- deferred: the latest bounded serial readiness window retained only firmware
  output and did not contain Talos readiness markers; runtime readiness remains
  a hardware recheck question.
- not-an-issue: no `docs/src` proof-rule update is required because the
  accepted readiness markers and stable TFTP semantics are unchanged.
