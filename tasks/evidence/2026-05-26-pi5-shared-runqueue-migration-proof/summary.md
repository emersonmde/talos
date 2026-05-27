# Pi 5 Shared Run-Queue Migration Proof Evidence

Task ID: phase6-pi5-shared-runqueue-migration-proof-20260526

Status: accepted

## local1

- Archive: target/talos-rpi5-shared-runqueue-migration-boot.tar.gz.
- Archive SHA256:
  4d5c8e2666d64ddcc5df7b49c8d3a541b01634800917616cbdb88404a54630d5.
- Kernel SHA256:
  98a9cb87bcb89c38b19a097a05695a136aaf6b0eb911ec03c3b0c17eeab6a394.
- Kernel size: 102,952 bytes.
- Pre-run snapshot:
  pre-shared-runqueue-migration-20260527T0308Z.
- TFTP proof:
  tasks/evidence/2026-05-26-pi5-shared-runqueue-migration-proof/local1/tftp-delta-before-restore.json
  records served da591740/kernel_2712.img requests from 10.42.1.4 with
  bytes=102952 before restore.
- Serial proof:
  tasks/evidence/2026-05-26-pi5-shared-runqueue-migration-proof/local1/serial-combined.txt
  is cursor-valid from the pre-power-cycle cursor.
- Classification:
  pi5-shared-runqueue-migration-complete.
- Restore proof:
  restore-exit.txt is 0, restore-pre-snapshot.json reports ok=true, and
  post-restore-status.json reports tree hash
  6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef.

## Decisive Serial Lines

The serial stream includes firmware network boot chatter and some concurrent
secondary-core UART interleaving. The decisive cursor-valid Talos lines are:

~~~text
rpi5-shared-runqueue-migration: report logical=3 source-owner=3 source-role=secondary-production-diagnostic destination-owner=0 destination-role=boot-production task=407 task-state=runnable registered-generation=1 publish-reserved-state=migration-reserved publish-queued-state=shared-queued consume-queued-state=shared-queued consume-destination-state=destination-enqueued source-queue-before=1 source-queue-after-publish=0 shared-len-after-publish=1 shared-len-after-consume=0 destination-queue-len=1 destination-front=407 metadata-owner-after-consume=0 metadata-generation-after-consume=2 source-removed=true destination-enqueued=true metadata-migrated=true errors=0 ok=true
rpi5-shared-runqueue-migration: final participants=4 expected=4 errors=0 lock-available=true wait-remaining=199997163 classification=pi5-shared-runqueue-migration-complete
rpi5-shared-runqueue-migration: PASS
~~~

The final line is the acceptance discriminator: all four physical-core
participants completed the implemented shared run-queue/migration invariant,
with no reported errors and the expected classification.
