# Phase 6 Pi 5 Secondary Scheduler Service Loop Proof

Task ID: phase6-pi5-secondary-scheduler-service-loop-proof-20260526
Status: accepted

## Goal

Run the serialized Pi 5 physical proof for the accepted secondary scheduler
service loop after QEMU substitute evidence is accepted.

## Scope

- Added the focused Pi 5 proof flag
  TALOS_RPI5_SECONDARY_SCHEDULER_SERVICE_LOOP_PROOF.
- Added focused Pi 5 image and boot-tree scripts for the hardware proof.
- Proved logical CPUs 1, 2, and 3 running one owner-local service-loop cycle
  after accepted secondary cacheable-MMU handoff.
- Preserved the diagnostic boundary: no shared run queues, remote enqueue
  queues, task migration, load balancing, work stealing, remote reschedule, or
  multi-core preemption.

## Evidence

Evidence directory:
tasks/evidence/2026-05-26-pi5-secondary-scheduler-service-loop-proof/.

- Static/code: src/target/rpi5.rs adds a focused secondary scheduler
  service-loop proof path mirroring the accepted QEMU invariant; build.rs,
  src/arch/aarch64/boot.S, src/boot/rpi5.rs, and scripts/ add the narrow
  diagnostic flag and archive helpers.
- Image/archive inspection:
  scripts/rpi5-secondary-scheduler-service-loop-boot-tree.sh and
  scripts/rpi5-archive-review.sh
  target/talos-rpi5-secondary-scheduler-service-loop-boot.tar.gz passed.
  Archive SHA256:
  56fb95ec7ff4092fa384a83f9af1705a0ec11a023a1e216f4563f9d18d6f24b3.
  Kernel SHA256:
  a9228747b7102024efa933e3d7acf6ed5ee800354fac5721a13115ab34c6184d.
  Kernel size: 102,824 bytes.
- Hardware local1: cursor-valid serial showed Talos entry, the
  rpi5-secondary-scheduler-service-loop start line, secondary cacheable-MMU
  handoff plan, PSCI CPU_ON for logical CPUs 1/2/3, per-core reports with
  role=secondary-production-diagnostic, remote wake drain, local dispatch,
  no-work metadata refresh, cross-owner rejection, deferred-role rejection,
  preserved local queue, lock-progress=1, errors=0, and ok=true.
- Hardware local1 final line reported participants=3 expected=3 errors=0
  state-lock-available=true metadata-lock-available=true final-metadata-len=3
  final-metadata-generation=9 and
  classification=pi5-secondary-scheduler-service-loop-complete; PASS followed.
- TFTP proof: local1 TFTP delta records da591740/kernel_2712.img fetches from
  10.42.1.4 with bytes=102824 before restore.
- Restore proof: local1 restore-exit.txt is 0 and post-restore status/serial
  were captured after restoring the pre-run snapshot.

## Validation

- git status --short before edits: clean.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed, 134 no_std tests.
- QEMU/substitute baseline: scripts/qemu-smoke.sh passed with
  talos: qemu smoke PASS.
- focused QEMU/substitute: scripts/qemu-secondary-scheduler-service-loop-smoke.sh
  passed with classification qemu-secondary-scheduler-service-loop-complete.
- image/archive inspection:
  scripts/rpi5-archive-review.sh
  target/talos-rpi5-secondary-scheduler-service-loop-boot.tar.gz passed.
- serial hardware boot/output:
  classification=pi5-secondary-scheduler-service-loop-complete.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Result

Accepted. The Pi 5 evidence proves the accepted secondary scheduler
service-loop invariant on physical secondary cores, including candidate
identity, TFTP fetch, boot execution, owner-local service-loop behavior,
classification, PASS output, and restore evidence.
