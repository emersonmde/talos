# Cleanup Baseline Pi 5 Hardware Validation Evidence

Task: talos-cleanup-baseline-pi5-hardware-validation-20260527

Status: accepted

## Candidate

- Commit: 515a94b30e326844c1596d2ef7d9b093a357f1f0
- Archive: target/talos-rpi5-cleanup-baseline-load-balancing-boot.tar.gz
- Archive SHA256:
  2273a90146469638c687c70a7b383bb517a1c2b1aa31d1c54d562ba2fb3594b5
- Kernel SHA256:
  901bf299a080fdb7ece60281b6cea237e98fdc44129e0f1da0ed81370b06e98c
- Kernel size: 95,064 bytes
- Archive review: passed

## Run Notes

- local1: cursor-valid serial from cursor 2204785 reached
  `classification=pi5-load-balancing-complete` and
  `rpi5-load-balancing: PASS`.
- local3: TFTP delta before restore captured candidate
  `da591740/kernel_2712.img` fetches at 95,064 bytes.
- local4: candidate rerun captured candidate TFTP fetches at 95,064 bytes, but
  serial observe from cursor 2217194 contained only NUL/newline.
- local5-control: restored the local3 pre-run 82,045-byte boot tree and
  captured fresh control TFTP fetches, but serial observe from cursor 2217196
  contained only NUL/newline.
- local6-control: clean known-good control using the previously accepted
  `target/talos-rpi5-load-balancing-boot.tar.gz` archive passed from serial
  cursor 2217200. TFTP served `da591740/kernel_2712.img` at 95,128 bytes.
- local7-candidate: cleanup-baseline candidate rerun passed from serial cursor
  2230226. TFTP served `da591740/kernel_2712.img` at 95,064 bytes.
- final restore: restored snapshot
  `pre-cleanup-baseline-local6control-20260528T012920Z`; final lab status
  reports the 82,045-byte pre-run boot tree.

## Decisive Serial Excerpt

```text
rpi5-load-balancing: start task-capacity=1 queue-capacity=1 boot-mpidr=0x0000000081000000 boot-affinity=0x0 boot-logical=Some(0)
rpi5-load-balancing: report source-owner=0 destination-owner=1 task=109 task-state=runnable registered-generation=1 plan-generation=1 publish-reserved-state=migration-reserved publish-queued-state=shared-queued consume-queued-state=shared-queued consume-destination-state=destination-enqueued source-queue-before=1 source-queue-after-publish=0 shared-len-after-publish=1 shared-len-after-consume=0 destination-queue-len=1 destination-front=109 metadata-owner-after-consume=1 metadata-generation-after-consume=2 selected-front=true source-removed=true destination-enqueued=true metadata-migrated=true errors=0 ok=true
rpi5-load-balancing: final participants=1 expected=1 errors=0 classification=pi5-load-balancing-complete
rpi5-load-balancing: PASS
```

The same PASS/classification sequence is present in local1, local6-control, and
local7-candidate. The decisive acceptance rerun is local7-candidate.

## Validation Levels

- image/archive inspection: archive review passed
- lab-controller API: publish, power-cycle, TFTP, serial, and restore/control
  records captured
- serial hardware boot/output: local1 captured cursor-valid Talos PASS output;
  local6-control and local7-candidate also captured cursor-valid Talos PASS
  output after the inconclusive local4/local5 runs
- hardware triage: candidate identity, clean known-good control, fresh serial
  cursors, TFTP deltas, candidate rerun, and final restore evidence captured
