# Phase 6 Pi 5 Load-Balancing Proof

## Task

- Title: Phase 6 Pi 5 load-balancing proof
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: serialized physical Pi 5 proof for the accepted load-balancing core

## Goal

Carry the accepted QEMU load-balancing invariant to serialized Raspberry Pi 5
hardware evidence.

## Acceptance Criteria

- Pi 5 evidence proves the same named load-balancing invariant as QEMU through
  the implemented core.
- Evidence includes candidate identity, fresh serial cursor, TFTP fetch/delta,
  final classification/PASS, and restore status.
- Any inconclusive run is triaged before code changes.
- Accepted hardware proof is committed before closeout starts.

## Context

The target-independent load-balancing core and the focused QEMU substitute
proof were accepted in tasks/2026-05-27-phase6-load-balancing-core.md and
tasks/2026-05-27-phase6-qemu-load-balancing-smoke.md. This task adds only a
focused Pi 5 boot scenario and staging helper for the same deterministic
front-runnable selection and SharedRunQueue handoff invariant.

## Work Performed

- Added the rpi5_load_balancing_proof boot scenario.
- Added scripts/rpi5-load-balancing-image.sh and
  scripts/rpi5-load-balancing-boot-tree.sh.
- Added target::rpi5::run_load_balancing_proof().
- The diagnostic runs on the boot CPU, creates source owner 0 and destination
  owner 1, and exercises LoadBalancingPolicy::plan_front_runnable,
  LoadBalancingPolicy::publish_front_runnable, and
  SharedRunQueue::consume_for_destination.
- Updated scheduler architecture, roadmap status, and decision log entries for
  the accepted Pi 5 proof.

## Evidence

Evidence directory:
tasks/evidence/2026-05-27-pi5-load-balancing-proof/.

- hardware lock: local1 acquired at 2026-05-27T15:03:26Z.
- candidate identity: archive target/talos-rpi5-load-balancing-boot.tar.gz
  SHA256 e7d4c80740bac203e9516e68baef29e9d197a8e760d233301cb209605a38d119;
  kernel SHA256 ceb75685864c32ed3d5a028c877d6a1d911892d4cbf14b36536d266206d7fecd;
  kernel size 95,128 bytes.
- image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-load-balancing-boot.tar.gz passed.
- lab-controller API: pre-run snapshot
  pre-load-balancing-20260527T150622Z, publish, power cycle, TFTP delta,
  serial observe, restore, and post-restore status were captured under local1.
- TFTP evidence: local1 TFTP delta from cursor 3707089 records fresh
  da591740/kernel_2712.img fetches from 10.42.1.4 / 88:a2:9e:ae:c8:7f.
  Candidate size is tied through publish/status and archive/kernel digests
  because the API event byte field is computed from the current boot tree when
  queried.
- serial hardware boot/output: cursor 2193432 reached
  classification=pi5-load-balancing-complete and rpi5-load-balancing: PASS.
- restore proof: restore-exit.txt is 0 and post-restore-status.json restored
  the pre-run boot tree.

## Validation

- git status --short before edits: clean during worker promotion.
- hardwareTestLock acquisition evidence: recorded in supervisor state and
  local1/acquired-at.txt.
- candidate identity check: archive/kernel digests and post-publish status
  captured.
- fresh serial cursor check: local1 started at serial cursor 2193432.
- TFTP delta/fetch evidence: local1 TFTP cursor 3707089 -> 3708440 records
  fresh Pi 5 boot fetches.
- serial hardware boot/output: local1 reached
  classification=pi5-load-balancing-complete and PASS.
- restore proof: pre-run snapshot restored successfully.

Final repository validation:

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed.
- QEMU/substitute: scripts/qemu-shared-runqueue-migration-smoke.sh passed.
- QEMU/substitute: scripts/qemu-load-balancing-smoke.sh passed.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Result

Accepted as serialized Pi 5 hardware evidence for the load-balancing core. The
proof shows the same deterministic invariant as the QEMU smoke: source owner 0
selects task 109 for destination owner 1, plan generation matches registered
generation, the source-local queue changes 1 -> 0, shared queue changes 1 -> 0,
destination queue front becomes task 109, metadata owner changes to 1, metadata
generation advances to 2, and PASS is emitted.

This does not accept autonomous work stealing, running-task migration, remote
reschedule, multi-core preemption, userspace, descriptors, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.
