# Phase 6 Cross-Core Wakeup Closeout Checkpoint

Status: accepted as the Phase 6.3 cross-core wakeup closeout checkpoint.

This checkpoint reconciles raw SGI/IPI delivery, the remote wake-request
ownership model, the first QEMU remote wake-request implementation proof,
retained gates, remaining risks, and the next bounded recommendation before
Talos starts any broader scheduler migration, shared run queues, task
migration, Phase 7, filesystem, networking, SSH, or shell work.

## Accepted Work

- Cross-core wakeup/IPI source inventory: commit `e92ff9d`; checkpoint
  `docs/src/project/phase6-cross-core-wakeup-ipi-source-inventory.md`; task
  record
  `tasks/2026-05-25-phase6-cross-core-wakeup-ipi-source-inventory.md`.
- QEMU raw SGI/IPI delivery smoke: commit `9983ca8`; task record
  `tasks/2026-05-25-phase6-qemu-cross-core-ipi-delivery-smoke.md`;
  transcript `target/qemu-cross-core-ipi-delivery-smoke.log`.
- Pi 5 raw SGI/IPI delivery proof: commit `29ce350`; task record
  `tasks/2026-05-25-phase6-pi5-cross-core-ipi-delivery-proof.md`;
  evidence summary
  `tasks/evidence/2026-05-25-pi5-cross-core-ipi-delivery-proof/summary.md`.
- Remote wake-request ownership inventory: commit `5dd1310`; checkpoint
  `docs/src/project/phase6-remote-wakeup-ownership-source-inventory.md`;
  task record
  `tasks/2026-05-25-phase6-remote-wakeup-ownership-source-inventory.md`.
- QEMU remote wake-request smoke: commit `5ac7eff`; task record
  `tasks/2026-05-25-phase6-qemu-remote-wakeup-request-smoke.md`;
  transcript `target/qemu-remote-wakeup-request-smoke.log`.

## Evidence Reconciliation

The raw IPI split is complete for the current proof level. QEMU virt proved
SGI INTID 1 delivery from logical CPU 0 to logical CPUs 1, 2, and 3 with
target-list bits `0x02`, `0x04`, and `0x08`. Each target acknowledged and
EOI'd one SGI, reported one receive count, and the diagnostic ended with
`classification=qemu-cross-core-ipi-delivery-complete`.

Serialized Pi 5 hardware evidence now proves the physical GIC-400 path for the
same raw SGI class. After the Pi 5 exception dispatcher included the raw IPI
proof handler, the accepted run served archive
`a6c5cb6999784e8f8c61a07765d39e9549c19c0ae37a54267c738b116a521a79` and a
97,016-byte kernel with SHA256
`44792c6681d0e67df08abeaebd18f2408680940ead47e2cf1e0b44f5b3956837`.
Cursor-valid serial showed `cpuif-poll=active-spin`, SGIR `0x01000001`,
receivers 1, 2, and 3 each at `receive-count=1 eoi-count=1 intid=1`,
`participants=3 expected=3 errors=0`, and
`classification=pi5-cross-core-ipi-delivery-complete`. Earlier failed or
inconclusive hardware runs remain useful failure evidence, not accepted
delivery proof.

The remote wake-request ownership inventory selected a bounded per-target
request list. A remote sender may publish or coalesce a wake request for a
scheduler-local `TaskId`, then signal the target with SGI INTID 1. The target
CPU owns request consumption and any future local scheduler effect. Direct
remote mutation of another CPU's `RunnableQueue` remains forbidden.

The QEMU remote wake-request smoke proves that first scheduler-facing model at
substitute level. CPU 0 published requests for logical CPUs 1, 2, and 3,
coalesced a duplicate target-1 request, sent SGI INTID 1 to each target, and
each target observed one SGI, EOI'd it, consumed its own request, drained to
queue length 0, rejected cross-owner local scheduler mutation, and reported
zero errors with `classification=qemu-remote-wakeup-request-complete`.

## Retained Gates

Retained regression gates for cross-core wakeup work:

- `cargo fmt --all -- --check` and `cargo -Zjson-target-spec test` for
  scheduler, GICv2, SMP, and remote wake-request invariants;
- `scripts/qemu-smoke.sh` for broad QEMU boot coverage;
- `scripts/qemu-cross-core-ipi-delivery-smoke.sh` for raw SGI delivery;
- `scripts/qemu-remote-wakeup-request-smoke.sh` for the scheduler-facing
  QEMU remote wake-request model;
- `scripts/qemu-per-core-scheduler-ownership-smoke.sh` when changes touch
  per-core scheduler ownership;
- `scripts/rpi5-image.sh`, focused Pi 5 boot-tree scripts, and
  `scripts/rpi5-archive-review.sh` before any serialized Pi 5 proof;
- hardwareTestLock, TFTP/archive evidence, cursor-valid serial, artifact
  digests, classification, and restore proof for any physical Pi 5 claim.

This checkpoint itself changed only Markdown documentation and durable state,
so Rust fmt/tests and hardware runs were not required for acceptance.

## Deferred Work

The following remain explicitly deferred to later supervisor-planned tasks:

- Pi 5 scheduler-facing remote wake-request proof;
- shared run queues, global task lookup, remote enqueue queues, task
  migration, load balancing, work stealing, and remote reschedule;
- production secondary-core scheduler dispatch and multi-core preemption;
- sleep queues, wait queues, blocking I/O readiness, runtime-console
  concurrency, UART interrupts, and descriptor-facing TTY behavior;
- userspace, EL0, syscalls, descriptor tables, file descriptors, user/kernel
  copy policy, process address spaces, and process lifetime rules;
- filesystem behavior, program loading, libc/Rust std support, portable
  userland, and local shell behavior;
- RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH,
  Ethernet, and shell access.

## Remaining Risks

The QEMU remote wake-request proof is substitute evidence. It proves the
bounded request model and target-owned consumption semantics under QEMU, but
it does not prove physical Pi 5 scheduler-facing wake request publication,
target-side consumption, duplicate semantics, or cross-owner rejection.

Raw Pi 5 SGI delivery is accepted, but it is not enough by itself to accept a
Pi 5 scheduler wakeup path. The physical scheduler-facing proof must still tie
candidate fetch, SGI signaling, wake-request state, target-side observation,
target-owned consumption, and local queue ownership rejection to one
cursor-valid run.

The accepted `SpinLock<T>` and remote wake-request queue do not create a
shared scheduler topology. Any future shared metadata still needs a named
owner, protecting lock, memory-ordering rule, bounded capacity, and IRQ-context
policy. IPI context remains limited to acknowledge, classify, record bounded
state, EOI, and return.

## Next Recommendation

The next bounded task should be
`phase6-pi5-remote-wakeup-request-proof-20260525`.

That task should carry only the accepted QEMU remote wake-request model to Pi 5
hardware under hardwareTestLock. It should prove candidate fetch, SGI INTID 1
signaling, target-side IPI observation/EOI, target-owned request consumption,
duplicate coalescing, cross-owner runnable-queue mutation rejection, artifact
digests, classification, and restore evidence. It should not add shared run
queues, task migration, production secondary scheduler dispatch, multi-core
preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe, or
DMA behavior.

No broader scheduler migration or later roadmap work should start until the
supervisor creates or promotes an explicit durable task.

## Validation

- static inspection: `git status --short` was clean before checkpoint edits.
- static review: inspected accepted raw IPI and remote wake task records,
  architecture docs, roadmap, decision log, QEMU transcript, and Pi 5 evidence
  summary.
- whitespace inspection: `git diff --check` passed after checkpoint edits.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests and hardware runs were not required because this checkpoint
  changes only Markdown documentation and durable task state.
