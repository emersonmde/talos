# Post-Review Final Pi 5 Hardware Remediation Evidence

Task: talos-post-review-final-pi5-hardware-remediation-20260527

Status: accepted

## Candidate

- Commit: 2b7b69b5fc9f8aab4b6c311cb2c4be392542a882
- Archive: target/talos-rpi5-post-review-final-load-balancing-boot.tar.gz
- Archive SHA256:
  8b18b92377718bff6dd573597efd8d135303fc8e71ad80ae809477edee441db2
- Kernel SHA256:
  a67a65d756d47b86950d9c7f38112b9a4bd46b1d1b1bda4c0277c81016deb7cf
- Kernel size: 95,080 bytes
- Archive review: passed

## Gate Rationale

Senior-engineer review pass 1 changed the panic guard and documentation; pass 2
added only the independent second-pass review task record. The retained Pi 5
load-balancing proof was selected because it is the newest active Phase 6.3
physical scheduler/SMP proof surface and exercises boot, DTB parsing, memory,
cache/MMU, scheduler, and serial output.

## Hardware Run

- Snapshot: pre-post-review-final-local1-20260528T090208Z
- Serial cursor: 2242164
- Serial observe: cursor 2242164 -> 2248830, 6,666 bytes
- TFTP cursor: 3720599
- TFTP result: fresh da591740/kernel_2712.img serves at 95,080 bytes
- Classification: pi5-load-balancing-complete
- PASS line: rpi5-load-balancing: PASS
- Restore: restored the pre-run snapshot; post-restore status reports ok=true
  and effective_kernel=kernel_2712.img.

## Failed Attempts and Fixes

None. The first candidate run passed. No known-good control or remediation fix
was required.

## Decisive Serial Excerpt

    rpi5-load-balancing: start task-capacity=1 queue-capacity=1 boot-mpidr=0x0000000081000000 boot-affinity=0x0 boot-logical=Some(0)
    rpi5-load-balancing: report source-owner=0 destination-owner=1 task=109 task-state=runnable registered-generation=1 plan-generation=1 publish-reserved-state=migration-reserved publish-queued-state=shared-queued consume-queued-state=shared-queued consume-destination-state=destination-enqueued source-queue-before=1 source-queue-after-publish=0 shared-len-after-publish=1 shared-len-after-consume=0 destination-queue-len=1 destination-front=109 metadata-owner-after-consume=1 metadata-generation-after-consume=2 selected-front=true source-removed=true destination-enqueued=true metadata-migrated=true errors=0 ok=true
    rpi5-load-balancing: final participants=1 expected=1 errors=0 classification=pi5-load-balancing-complete
    rpi5-load-balancing: PASS

## Validation Levels

- static inspection: pass-1 and pass-2 changed-file review recorded
- image/archive inspection: archive review passed
- lab-controller API: publish, power-cycle, TFTP, serial, and restore records
  captured
- serial hardware boot/output: cursor-valid Talos PASS output captured
- documentation/whitespace: git diff --check passed
- documentation: mdbook build passed
