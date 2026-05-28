# Phase 6 Pi 5 Multi-Core Preemption Proof

## Task

- Title: Phase 6 Pi 5 multi-core preemption proof
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: serialized physical Pi 5 proof for the accepted multi-core preemption core

## Goal

Carry the accepted QEMU multi-core preemption invariant to serialized Raspberry
Pi 5 hardware evidence.

## Acceptance Criteria

- Serialized Pi 5 hardware reaches a PASS/classification line for the same
  named invariant as the accepted QEMU proof.
- Serial evidence includes human-readable Talos kernel output and decisive
  proof lines.
- TFTP, archive/kernel digest, candidate identity, participant count, and
  restore evidence are recorded.
- Every failed or inconclusive hardware attempt has a recorded disposition.

## Context

The target-independent multi-core preemption core and focused QEMU substitute
proof were accepted in tasks/2026-05-27-phase6-multicore-preemption-core.md and
tasks/2026-05-27-phase6-qemu-multicore-preemption-smoke.md. This task adds only
the serialized Pi 5 boot scenario and staging helpers needed to exercise the
same owner-local preemption invariant on physical hardware.

## Work Performed

- Added the rpi5_multicore_preemption_proof boot scenario.
- Added scripts/rpi5-multicore-preemption-image.sh and
  scripts/rpi5-multicore-preemption-boot-tree.sh.
- Added target::rpi5::run_multicore_preemption_proof().
- Reused the accepted Pi 5 secondary service-loop PSCI, stack, per-core state,
  and secondary cacheable-MMU handoff path.
- Fixed the Pi 5 secondary entry cfg so rpi5_multicore_preemption_proof enters
  the same secondary cacheable-MMU handoff as the accepted service-loop proof
  before running the diagnostic.

## Evidence

Evidence directory:
tasks/evidence/2026-05-27-pi5-multicore-preemption-proof/.

- hardware lock: local2-candidate-handoff-fix acquired at 2026-05-28T12:39:01Z.
- candidate identity: based on commit 28887a50d5753f1304d0259fea61ec5d4e94772e
  plus this task's uncommitted Pi 5 proof and handoff-path edits before final
  acceptance commit.
- prior candidate disposition: the first candidate archive was staged and
  fetched at 103,144 bytes, but two serialized Pi 5 candidate runs produced
  only Raspberry Pi firmware/RP1 boot output before reset. The required
  inconclusive-run triage was completed before code changes: candidate identity,
  fresh serial cursor, TFTP evidence, known-good load-balancing control, and a
  candidate rerun. The lab was restored to the accepted load-balancing tree.
- candidate fix disposition: comparison against the accepted secondary
  scheduler service-loop proof found that the new scenario published a
  secondary cacheable-MMU handoff plan, but secondary entry did not enter that
  handoff for rpi5_multicore_preemption_proof. The fix added only that cfg to
  the existing handoff guard.
- accepted candidate identity: archive
  target/talos-rpi5-multicore-preemption-boot.tar.gz SHA256
  93d6231019e94a46635a938009e96ca2668fcba1971ce2316bbe753c0df1f235; kernel
  SHA256 3e01bc68871cdbe5a00755ef03b482ef67c77d83066561f008c6d1121718686a;
  kernel size 103,144 bytes.
- image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-multicore-preemption-boot.tar.gz passed with
  kernel_size=103144, header_image_size=103144, and flags=12.
- lab-controller API: health, pre-run status, named snapshot
  pre-multicore-preemption-20260528T1238Z, publish, boot files, power cycle,
  TFTP delta, serial observe, restore, post-restore status, and post-restore
  power-cycle records were captured under local2-candidate-handoff-fix.
- TFTP evidence: local2-candidate-handoff-fix TFTP delta records fresh
  da591740/kernel_2712.img fetches at 103,144 bytes from 10.42.1.4 /
  88:a2:9e:ae:c8:7f before restore.
- serial hardware boot/output: cursor 2250254 reached human-readable Talos
  output, report lines for logical CPUs 1, 2, and 3, participants=3,
  errors=0, classification=pi5-multicore-preemption-complete, and
  rpi5-multicore-preemption: PASS.
- restore proof: restore-exit.txt is 0 and post-restore-status.json restored
  tree_hash=6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef
  with effective_kernel=kernel_2712.img.

## Validation

- git status --short before edits: carried existing paused Pi 5 proof edits
  from the prior worker wake.
- candidate identity: git rev-parse HEAD and git show -s --format recorded in
  local2-candidate-handoff-fix/head.txt.
- fmt/lint: cargo fmt --all -- --check passed after the handoff-path fix.
- unit tests: cargo -Zjson-target-spec test passed with 153 no_std tests.
- QEMU/substitute: scripts/qemu-multicore-preemption-smoke.sh passed after the
  handoff-path fix.
- focused retained Pi 5 proof image script:
  scripts/rpi5-multicore-preemption-image.sh passed.
- image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-multicore-preemption-boot.tar.gz passed.
- serialized Pi 5 hardware boot/output: local2-candidate-handoff-fix reached
  classification=pi5-multicore-preemption-complete and PASS.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- restore proof: named pre-run snapshot restored and post-restore status
  confirmed the accepted load-balancing tree hash.

## Result

Accepted as serialized Pi 5 hardware evidence for the multi-core preemption
core. The proof shows logical CPUs 1, 2, and 3 each record only local pending
timer-preemption state, coalesce duplicate local records, reject cross-owner
records, leave scheduler state unchanged during the record-only step, and
service the pending request through owner-local normal scheduler control flow.

This does not accept direct IRQ/IPI-context scheduling, autonomous work
stealing, running-task migration, remote current-task switching, general remote
reschedule, userspace, descriptors, filesystem, networking, SSH, shell
behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.
