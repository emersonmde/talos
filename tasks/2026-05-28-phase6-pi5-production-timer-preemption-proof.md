# Phase 6 Pi 5 Production Timer Preemption Proof

## Task

- Title: Phase 6 Pi 5 production timer/preemption proof
- Owner: worker
- Date: 2026-05-28
- Milestone: Phase 6.3, Production Scheduler Runtime Integration
- Scope: serialized physical Pi 5 proof for the accepted production timer/preemption invariant

## Goal

Carry the accepted QEMU production timer/preemption invariant to serialized
Raspberry Pi 5 hardware evidence.

## Current Status

Accepted. The Pi 5 lab reached the production timer/preemption PASS line after
switching from one-shot serial observe windows to a multi-observe capture loop,
then restored to the pre-run boot tree.

## Work Performed

- Added the rpi5_production_timer_preemption_proof boot scenario.
- Added scripts/rpi5-production-timer-preemption-image.sh and
  scripts/rpi5-production-timer-preemption-boot-tree.sh.
- Added target::rpi5::run_production_timer_preemption_proof(), using the
  target-owned production timer IRQ recording adapter and owner-local
  ProductionSchedulerRuntime::service_pending_preemption() path.

## Evidence So Far

Evidence directory:
tasks/evidence/2026-05-28-pi5-production-timer-preemption-proof/.

- local1-candidate: candidate archive fetched da591740/kernel_2712.img at
  104,136 bytes, but the serial capture only reached Raspberry Pi firmware/RP1
  output before restore.
- local1-control-restored: restored known-good control tree
  6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef
  initially had a short observe window, but later retained serial showed the
  restored multi-core preemption control still reaches
  classification=pi5-multicore-preemption-complete and PASS.
- local2-candidate-rerun and local3-candidate-settled: candidate reruns
  fetched the 104,136-byte kernel, but did not produce Talos proof lines before
  restore.
- local4-candidate-tail: tail-based capture after a settled candidate run
  still showed only firmware/bootloader output for the candidate, no
  rpi5-production-timer-preemption lines.
- local5-padded-control: a multi-core preemption control padded to 104,136
  bytes and with a matching arm64 Image header reached PASS, so the current
  candidate failure is not explained by the larger image size alone.
- local6-static-comparison-rerun: static/image comparison showed the rebuilt
  production image and the accepted multi-core image share the same primary
  early entry path; a one-shot rerun again returned only early firmware before
  the capture window ended.
- local7-correct-cursor-rerun: corrected TFTP cursor handling but still used a
  one-shot serial observe, so it remained inconclusive and was restored.
- local8-multi-observe-rerun: captured serial in repeated observe windows from
  the fresh cursor. The candidate fetched da591740/kernel_2712.img at 104,136
  bytes, reached rpi5-production-timer-preemption reports for logical CPUs 1,
  2, and 3, and printed
  classification=pi5-production-timer-preemption-complete plus PASS. The lab
  was restored to tree hash
  6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef.

## Validation

- candidate identity: commit 7bbe7a54d29eaacd86cad8a1881921268758cc2a
  plus uncommitted Pi 5 proof edits.
- image/archive inspection:
  scripts/rpi5-archive-review.sh target/talos-rpi5-production-timer-preemption-boot.tar.gz
  passed; accepted candidate archive SHA256
  739810c8480893e1878967dd0409f2705e71481453fc08038e9aacffdebcc11e;
  kernel SHA256
  fdf8858d0740c0d7bf4fc0df884d4052d8309fd9c020ba65e5df1472198e7dfa;
  kernel size 104,136 bytes.
- fmt/lint: cargo fmt --all -- --check passed after the latest Pi 5 edits.
- unit tests: cargo -Zjson-target-spec test passed with 156 no_std tests
  after the latest Pi 5 edits.
- focused QEMU/substitute: scripts/qemu-production-timer-preemption-smoke.sh
  passed after the Pi 5 proof code was added; the post-edit output is captured
  in qemu-production-timer-preemption-after-pi5-edits.txt.
- static inspection: git diff --check passed after the latest Pi 5 edits.
- serialized Pi 5 hardware boot/output:
  local8-multi-observe-rerun/serial-combined.txt records
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS with participants=3 expected=3
  errors=0.
- TFTP evidence:
  local8-multi-observe-rerun/tftp-delta-before-restore.json records fresh
  104,136-byte da591740/kernel_2712.img serves.
- restore proof: local8-multi-observe-rerun/final-status.json reports restored
  tree hash
  6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef.

## Result

Accepted as serialized Pi 5 hardware evidence for the production
timer/preemption integration. The accepted run demonstrates the target-owned
production timer IRQ recording adapter records only local pending preemption
state, coalesces duplicate records, rejects cross-owner records, leaves
scheduler state unchanged during the IRQ-record step, and services the pending
request through owner-local normal scheduler control flow.

This does not accept direct IRQ/IPI-context scheduling, remote current-task
switching, running-task migration, autonomous work stealing, userspace,
descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.
