# Static Evidence Inspection

Task id: phase11-known-good-serial-cursor-completeness-closeout-20260606

## Inputs Inspected

- tasks/2026-06-06-phase11-serial-cursor-saturation-repair-core.md.
- tasks/evidence/2026-06-06-phase11-serial-cursor-saturation-repair-core/static-evidence-inspection.md.
- tasks/evidence/2026-06-06-phase11-serial-cursor-saturation-repair-core/evidence-map.json.
- tasks/2026-06-06-phase11-known-good-serial-cursor-completeness-pi5.md.
- tasks/evidence/2026-06-06-phase11-known-good-serial-cursor-completeness-pi5/evidence-map.json.
- tasks/evidence/2026-06-06-phase11-known-good-serial-cursor-completeness-pi5/known-good-run/capture-invariant-summary.json.
- docs/src/project/lab-controller.md.
- docs/src/project/phase11-rp1-pcie-map-contract.md.

## Findings

- The repair core decisively identified the repeated zero-byte serial windows
  as a saturated cursor/capture failure class: the FR-read candidate,
  known-good control, and candidate rerun all observed from cursor 4194304.
- The helper repair is repository-side and bounded: saturated saved cursors use
  direct /serial/read with
  observe_contract=deadline-loop-direct-read-after-saturated-cursor, while
  non-saturated cursor observations keep the observe path.
- The known-good proof used the restored accepted tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10,
  effective_kernel=kernel_2712.img, and expected 104,136-byte
  da591740/kernel_2712.img fetch.
- Direct /serial/read from saturated cursor 4194304 captured 6,347 fresh bytes
  over 26 seconds, including firmware NETWORK output and the downstream Talos
  marker rpi5-production-timer-preemption: PASS.
- Stable same-cursor TFTP evidence before restore retained 13 events and two
  served da591740/kernel_2712.img fetches with 104,136 bytes.
- Restore evidence returned the boot tree to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  hardwareTestLock release.
- The proof accepts serial cursor/capture completeness only. It does not accept
  RP1 UART0 FR-read mapped/read-value, unmapped/trap, firmware-state,
  pre-MMIO reachability, GPIO, interrupts, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe, Milestone 11.2, or a phase
  transition.

## Closeout Classification

serial-cursor-capture-completeness-accepted

The repaired proof path is sufficient for a future explicitly queued RP1 UART0
FR-read rerun to avoid the prior cursor-4194304 saturation failure class. The
future task must still retain candidate identity, fresh serial and TFTP cursors,
stable pre-restore TFTP evidence, direct-read serial output if saturated,
restore proof, and exact RP1 classification evidence before accepting any RP1
behavior.
