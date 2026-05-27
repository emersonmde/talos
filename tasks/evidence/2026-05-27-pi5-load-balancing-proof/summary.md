# Phase 6 Pi 5 Load-Balancing Proof Evidence

Task: phase6-pi5-load-balancing-proof-20260527

Status: accepted

Evidence directory: tasks/evidence/2026-05-27-pi5-load-balancing-proof/local1/

## Candidate

- Archive: target/talos-rpi5-load-balancing-boot.tar.gz
- Archive SHA256:
  e7d4c80740bac203e9516e68baef29e9d197a8e760d233301cb209605a38d119
- Kernel SHA256:
  ceb75685864c32ed3d5a028c877d6a1d911892d4cbf14b36536d266206d7fecd
- Kernel size: 95,128 bytes
- Archive review: passed

## Hardware Run

- Hardware lock acquired: 2026-05-27T15:03:26Z
- Pre-run snapshot: pre-load-balancing-20260527T150622Z
- Serial cursor: 2193432
- TFTP cursor: 3707089 -> 3708440
- Classification: pi5-load-balancing-complete
- Result line: rpi5-load-balancing: PASS
- Restore: restore-exit.txt is 0 and post-restore status was captured

## Decisive Serial Excerpt

text:
rpi5-load-balancing: start task-capacity=1 queue-capacity=1 boot-mpidr=0x0000000081000000 boot-affinity=0x0 boot-logical=Some(0)
rpi5-load-balancing: report source-owner=0 destination-owner=1 task=109 task-state=runnable registered-generation=1 plan-generation=1 publish-reserved-state=migration-reserved publish-queued-state=shared-queued consume-queued-state=shared-queued consume-destination-state=destination-enqueued source-queue-before=1 source-queue-after-publish=0 shared-len-after-publish=1 shared-len-after-consume=0 destination-queue-len=1 destination-front=109 metadata-owner-after-consume=1 metadata-generation-after-consume=2 selected-front=true source-removed=true destination-enqueued=true metadata-migrated=true errors=0 ok=true
rpi5-load-balancing: final participants=1 expected=1 errors=0 classification=pi5-load-balancing-complete
rpi5-load-balancing: PASS

## Validation Levels

- image/archive inspection: scripts/rpi5-archive-review.sh passed
- lab-controller API: snapshot, publish, power-cycle, status, serial, TFTP,
  and restore records captured
- serial hardware boot/output: cursor-valid PASS transcript captured
- fmt/lint/typecheck: cargo fmt --all -- --check passed
- unit tests: cargo -Zjson-target-spec test passed
- QEMU/substitute: shared run-queue migration and load-balancing smoke gates
  passed
- whitespace inspection: git diff --check passed
- documentation: mdbook build passed
