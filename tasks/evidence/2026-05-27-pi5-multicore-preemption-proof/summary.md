# Pi 5 Multi-Core Preemption Proof Evidence Summary

Task: tasks/2026-05-27-phase6-pi5-multicore-preemption-proof.md.

## Attempts

- Prior candidate, 2026-05-28T12:25Z: archive and kernel were staged at
  103,144 bytes, TFTP fetched da591740/kernel_2712.img, but serial showed only
  Raspberry Pi firmware/RP1 boot lines before reset. The required inconclusive
  triage was completed: candidate identity, fresh serial cursor, TFTP evidence,
  known-good load-balancing control, candidate rerun, and restore to the
  accepted load-balancing tree.
- Accepted candidate, local2-candidate-handoff-fix: after comparing against
  the accepted secondary service-loop proof, the worker added the missing
  rpi5_multicore_preemption_proof cfg to the secondary cacheable-MMU handoff
  guard and reran the serialized Pi 5 proof.

## Accepted Candidate

- Archive: target/talos-rpi5-multicore-preemption-boot.tar.gz.
- Archive SHA256:
  93d6231019e94a46635a938009e96ca2668fcba1971ce2316bbe753c0df1f235.
- Kernel SHA256:
  3e01bc68871cdbe5a00755ef03b482ef67c77d83066561f008c6d1121718686a.
- Kernel size: 103,144 bytes.
- Archive review: passed with kernel_size=103144, header_image_size=103144,
  text_offset=0, flags=12, loader_diagnostic=false, file_count=19.
- Serial cursor: 2250254.
- TFTP cursor: 1048576 to 2097152.
- Classification: pi5-multicore-preemption-complete.
- Restore tree hash:
  6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef.

## Key Serial Facts

- Human-readable Talos boot output reached asm_start, rust_entry, kernel_main,
  and talos: boot start.
- The Pi 5 proof published a secondary cacheable-MMU handoff plan with
  cacheable-mmu=true.
- Logical CPUs 1, 2, and 3 all reached workload-complete and reported
  lock-progress=1, errors=0, ok=true.
- Final line:
  participants=3 expected=3 errors=0 classification=pi5-multicore-preemption-complete.
- PASS line: rpi5-multicore-preemption: PASS.

## Retained Files

Evidence files are under
tasks/evidence/2026-05-27-pi5-multicore-preemption-proof/local2-candidate-handoff-fix/.
The most useful files are archive-review.txt, archive-sha256.txt,
kernel-sha256.txt, serial-key-lines.txt, serial-combined.txt,
tftp-delta-before-restore.json, pre-snapshot.json, restore-pre-snapshot.json,
post-restore-status.json, and classification.txt.
