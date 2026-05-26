# Pi 5 Secondary Scheduler Service Loop Proof Evidence

Task ID: phase6-pi5-secondary-scheduler-service-loop-proof-20260526

Status: accepted

## local1

- Archive: target/talos-rpi5-secondary-scheduler-service-loop-boot.tar.gz.
- Archive SHA256:
  56fb95ec7ff4092fa384a83f9af1705a0ec11a023a1e216f4563f9d18d6f24b3.
- Kernel SHA256:
  a9228747b7102024efa933e3d7acf6ed5ee800354fac5721a13115ab34c6184d.
- Kernel size: 102,824 bytes.
- Pre-run snapshot:
  pre-secondary-scheduler-service-loop-20260526T1516Z.
- TFTP proof:
  tasks/evidence/2026-05-26-pi5-secondary-scheduler-service-loop-proof/local1/tftp-delta-before-restore.json
  records served da591740/kernel_2712.img requests from 10.42.1.4 with
  bytes=102824.
- Serial proof:
  tasks/evidence/2026-05-26-pi5-secondary-scheduler-service-loop-proof/local1/serial-combined.txt
  is cursor-valid from the pre-power-cycle cursor.
- Classification:
  pi5-secondary-scheduler-service-loop-complete.
- Restore proof:
  restore-exit.txt is 0, restore-pre-snapshot.json reports ok=true, and
  post-restore-status.json reports tree hash
  6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef.

## Decisive Serial Lines

```text
TALOS: asm_start
rpi5-secondary-scheduler-service-loop: start conduit=smc cores=4 task-capacity=1 boot-mpidr=0x0000000081000000 boot-affinity=0x0 boot-logical=Some(0) boot-sctlr-el2=0x0000000030c51835 boot-cacheable-mmu=true
rpi5-secondary-scheduler-service-loop: secondary-cacheable-mmu-handoff-plan mair-el2=0x00000000000004ff tcr-el2=0x0000000000053510 ttbr0-el2=0x000000002f000000 sctlr-el2=0x0000000030c51835 cacheable-mmu=true
rpi5-secondary-scheduler-service-loop: report logical=1 state=workload-complete context=1 mpidr=0x0000000081000100 affinity=0x100 mapped=Some(1) owner=1 role=secondary-production-diagnostic task=201 task-state=running current=201 queue-len=0 front=0 remote-wake=201 dispatch=201 no-work-did-work=false metadata-len=1 metadata-generation=3 observed-remote-wake=true pending-timer-preemption=false dispatch-requested=true cross-owner-rejected=true deferred-role-rejected=true local-queue-preserved=true lock-progress=1 errors=0 ok=true
rpi5-secondary-scheduler-service-loop: report logical=2 state=workload-complete context=2 mpidr=0x0000000081000200 affinity=0x200 mapped=Some(2) owner=2 role=secondary-production-diagnostic task=301 task-state=running current=301 queue-len=0 front=0 remote-wake=301 dispatch=301 no-work-did-work=false metadata-len=1 metadata-generation=3 observed-remote-wake=true pending-timer-preemption=false dispatch-requested=true cross-owner-rejected=true deferred-role-rejected=true local-queue-preserved=true lock-progress=1 errors=0 ok=true
rpi5-secondary-scheduler-service-loop: report logical=3 state=workload-complete context=3 mpidr=0x0000000081000300 affinity=0x300 mapped=Some(3) owner=3 role=secondary-production-diagnostic task=401 task-state=running current=401 queue-len=0 front=0 remote-wake=401 dispatch=401 no-work-did-work=false metadata-len=1 metadata-generation=3 observed-remote-wake=true pending-timer-preemption=false dispatch-requested=true cross-owner-rejected=true deferred-role-rejected=true local-queue-preserved=true lock-progress=1 errors=0 ok=true
rpi5-secondary-scheduler-service-loop: final participants=3 expected=3 errors=0 state-lock-available=true metadata-lock-available=true final-metadata-len=3 final-metadata-generation=9 wait-remaining=199997610 classification=pi5-secondary-scheduler-service-loop-complete
rpi5-secondary-scheduler-service-loop: PASS
```
