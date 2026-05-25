# Phase 6 Remote Wakeup Scheduler Integration Closeout

Status: accepted as the Phase 6.3 remote wakeup scheduler-integration
closeout checkpoint.

This checkpoint reconciles the accepted raw SGI/IPI, remote wake-request, and
target-owned local runnable evidence before Talos starts production secondary
scheduler dispatch, broader scheduler migration, multi-core preemption, Phase
7, filesystem, networking, SSH, or shell work.

## Accepted Work

- Cross-core wakeup closeout checkpoint: commit ed777c3; checkpoint
  docs/src/project/phase6-cross-core-wakeup-closeout-checkpoint.md; task
  record tasks/2026-05-25-phase6-cross-core-wakeup-closeout-checkpoint.md.
- Pi 5 remote wake-request proof: commit 256717b; task record
  tasks/2026-05-25-phase6-pi5-remote-wakeup-request-proof.md; evidence
  summary tasks/evidence/2026-05-25-pi5-remote-wakeup-request-proof/summary.md.
- Target-owned wake-consumption contract: commit 0bec99f; checkpoint
  docs/src/project/phase6-target-owned-wake-consumption-contract.md; task
  record tasks/2026-05-25-phase6-target-owned-wake-consumption-contract.md.
- QEMU remote wake to local runnable smoke: commit fdafa22; task record
  tasks/2026-05-25-phase6-qemu-remote-wake-to-local-runnable-smoke.md;
  transcript target/qemu-remote-wake-to-local-runnable-smoke.log.
- Pi 5 remote wake to local runnable proof: commit 13c2208; task record
  tasks/2026-05-25-phase6-pi5-remote-wake-to-local-runnable-proof.md;
  evidence summary
  tasks/evidence/2026-05-25-pi5-remote-wake-to-local-runnable-proof/summary.md.

## Evidence Reconciliation

Raw SGI delivery is accepted on QEMU and Pi 5. That evidence proves INTID 1
can be sent from CPU 0 to logical CPUs 1, 2, and 3, observed by the target,
and EOIed without accepting any scheduler mutation in the IPI hot path.

The remote wake-request model is accepted at QEMU substitute and Pi 5 hardware
levels. A remote CPU may publish or coalesce a bounded request for a
scheduler-local TaskId into the target CPU RemoteWakeQueue, then send SGI
INTID 1. The target CPU owns request observation, consumption, and any later
local scheduler effect. Direct mutation of another CPU RunnableQueue remains
rejected.

The target-owned wake-consumption contract and QEMU proof added the next local
scheduler boundary: after the target drains its own request queue outside IPI
context, only that target may transition a matching local diagnostic task from
Blocked to Runnable. The proof rejected wrong-owner calls, mismatched task IDs,
duplicate local enqueue, drained queues incorrectly left non-empty, and
production secondary dispatch.

Serialized Pi 5 evidence now carries that invariant to hardware. The accepted
local2 run used archive SHA256
acf72b3b52416ac8e41178c7bf328d4f075981c5800f937cb016c9cecb8226b2, kernel
SHA256 01e04b23addf8876d58d0d6f332d9b8d923a9f814bcf2a72c68cfb5f421ffae6,
and kernel size 103,040 bytes. Cursor-valid serial showed request publication
for logical CPUs 1, 2, and 3, duplicate request coalescing for target 1, SGI
sends, target-side receive/EOI/accounting, consumed diagnostic tasks
201/202/203, drained request queues, cross-owner rejection, duplicate local
enqueue rejection, local Blocked -> Runnable transitions on each target,
production dispatch deferred, and
classification=pi5-remote-wake-to-local-runnable-complete with PASS.

The first local1 hardware attempt remains classified as early firmware-only
current serial; it is not accepted proof. Both local1 and local2 restore
evidence recorded restore-exit.txt as 0.

## Retained Gates

Retained regression gates for this slice:

- cargo fmt --all -- --check and cargo -Zjson-target-spec test for scheduler,
  remote wake queue, per-core scheduler ownership, GICv2, and SMP invariants;
- scripts/qemu-smoke.sh for broad QEMU boot coverage;
- scripts/qemu-cross-core-ipi-delivery-smoke.sh for raw SGI delivery;
- scripts/qemu-remote-wakeup-request-smoke.sh for bounded request publication,
  coalescing, target consumption, and cross-owner rejection;
- scripts/qemu-remote-wake-to-local-runnable-smoke.sh for the target-owned
  local Blocked -> Runnable transition proof;
- scripts/qemu-per-core-scheduler-ownership-smoke.sh when scheduler ownership
  code changes;
- scripts/rpi5-image.sh, focused Pi 5 boot-tree scripts, and
  scripts/rpi5-archive-review.sh before any Pi 5-targeted proof;
- hardwareTestLock, TFTP/archive evidence, cursor-valid serial, artifact
  digests, classification, and restore proof for any physical Pi 5 claim.

The focused remote wake and Pi 5 boot-tree scripts are retained as named
validation gates. No temporary diagnostic surface needs quarantine in this
checkpoint.

## Deferred Work

The following remain explicitly deferred to later supervisor-planned tasks:

- production secondary-core scheduler dispatch;
- shared run queues, global task lookup, remote enqueue queues, task
  migration, load balancing, work stealing, and remote reschedule;
- multi-core preemption, per-core timer ownership for secondary production
  scheduling, sleep queues, wait queues, and blocking I/O readiness;
- runtime-console concurrency, UART interrupts, descriptor-facing TTY behavior,
  userspace, EL0, syscalls, descriptor tables, file descriptors, user/kernel
  copy policy, process address spaces, and process lifetime rules;
- filesystem behavior, program loading, libc/Rust std support, portable
  userland, and local shell behavior;
- RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH,
  Ethernet, and shell access.

## Remaining Risks

The accepted remote wake path is still diagnostic. It proves a target-owned
local runnable transition for bounded diagnostic tasks, not production task
selection or secondary-core dispatch through the real scheduler loop.

The topology remains CPU-local. There is no shared run queue, global task
lookup, migration policy, load balancer, wake-list ownership beyond the
bounded RemoteWakeQueue, or remote enqueue authority.

IPI context remains constrained to acknowledge/classify/record/EOI and return.
It must not allocate, format, print to serial, poll UART input, dispatch
diagnostic commands, block, sleep, take long locks, walk arbitrary scheduler
queues, migrate tasks, or cross the context-switch boundary.

## Readiness Decision

Talos is ready for one bounded production secondary scheduler dispatch source
inventory and contract task. That task should reconcile the accepted per-core
scheduler ownership, secondary-core workload, SMP lock/cache-coherence, raw
SGI, remote wake-request, and target-owned local runnable evidence before any
implementation lets secondary cores run production scheduler dispatch.

Talos is not ready for multi-core preemption planning, shared run queues, task
migration, load balancing, Phase 7, filesystem, networking, SSH, or shell work.

## Next Recommendation

The next bounded task should be
phase6-production-secondary-scheduler-dispatch-source-inventory-20260525.

That task should be documentation/source-inventory only. It should define the
ownership and validation boundary for letting secondary cores leave diagnostic
dispatch and run production scheduler work, including how target-local runnable
queues, context-switch boundaries, timer/preemption state, IPI observation,
remote wake drains, console/output ownership, and failure diagnostics remain
bounded. It should not implement production secondary dispatch.

No broader scheduler migration or later roadmap work should start until the
supervisor creates or promotes an explicit durable task.

## Validation

- static inspection: git status --short was clean before checkpoint edits.
- static review: inspected accepted remote wake task records, architecture
  docs, roadmap, decision log, QEMU transcripts, and Pi 5 evidence summaries.
- whitespace inspection: git diff --check passed after checkpoint edits.
- static inspection: mdbook was unavailable in the container, so mdBook build
  was not run.
- Rust fmt/tests and hardware runs were not required because this checkpoint
  changes only Markdown documentation and durable task state.
