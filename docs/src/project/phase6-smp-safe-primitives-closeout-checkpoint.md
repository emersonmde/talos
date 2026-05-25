# Phase 6 SMP-Safe Primitives Closeout Checkpoint

Status: accepted as the Milestone 6.2 SMP-safe primitives closeout.

This checkpoint reconciles the accepted Phase 6.2 primitive contract,
implementation, QEMU substitute evidence, serialized Pi 5 hardware proof, and
cleanup before Talos starts scheduler migration, shared run queues, cross-core
wakeups, IPIs, EL0, syscalls, descriptors, filesystem, networking, SSH, shell
behavior, UART interrupts, RP1/PCIe, or DMA/cache-coherent driver policy.

## Accepted Work

- Source inventory and contract: commit `6067f64`; checkpoint
  `docs/src/project/phase6-smp-safe-primitives-source-inventory.md`; task
  record `tasks/2026-05-24-phase6-smp-safe-primitives-source-inventory.md`.
- Spinlock/barrier core: commit `4290c36`; task record
  `tasks/2026-05-24-phase6-spinlock-barrier-core.md`.
- QEMU SMP lock contention smoke: commit `895448b`; task record
  `tasks/2026-05-24-phase6-qemu-smp-lock-contention-smoke.md`.
- Secondary cacheable-MMU handoff source inventory: commit `dddb27e`;
  checkpoint
  `docs/src/project/phase6-secondary-cacheable-mmu-handoff-source-inventory.md`;
  task record
  `tasks/2026-05-25-phase6-secondary-cacheable-mmu-handoff-source-inventory.md`.
- Secondary cacheable-MMU handoff core: commit `a45cf92`; task record
  `tasks/2026-05-25-phase6-secondary-cacheable-mmu-handoff-core.md`.
- Pi 5 secondary cacheable-MMU handoff proof: commit `79937bc`; task record
  `tasks/2026-05-25-phase6-secondary-cacheable-mmu-handoff-pi5-proof.md`;
  evidence directory
  `tasks/evidence/2026-05-25-pi5-secondary-cacheable-mmu-handoff-proof/`.
- SMP lock evidence hygiene and report inventory: commit `d8a2087`; task
  record
  `tasks/2026-05-25-phase6-smp-lock-evidence-hygiene-and-report-inventory.md`;
  historical evidence summary
  `tasks/evidence/2026-05-24-pi5-smp-lock-cache-coherence-proof/summary.md`.
- SMP lock report-invariant core: commit `85f53c8`; task record
  `tasks/2026-05-25-phase6-smp-lock-report-invariant-core.md`.
- Pi 5 SMP lock cache/coherence final proof: commit `0a3b50f`; task record
  `tasks/2026-05-25-phase6-pi5-smp-lock-cache-coherence-final-proof.md`;
  evidence directory
  `tasks/evidence/2026-05-25-pi5-smp-lock-cache-coherence-final-proof/`.
- SMP lock proof scaffolding quarantine: commit `9a80fa8`; task record
  `tasks/2026-05-25-phase6-smp-lock-proof-scaffolding-quarantine.md`.

## Accepted Boundary

Milestone 6.2 accepts the first generic SMP mutual-exclusion primitive:
`SpinLock<T>`, `SpinLockGuard`, AArch64 IRQ-save composition through
`lock_irqsave()`, and `smp_full_barrier()` as a named `dmb ish` boundary.
The lock uses acquire ordering on successful acquisition and release ordering
on unlock. The IRQ-save composition keeps the accepted order explicit: save and
mask local IRQ state, acquire the SMP lock, release the lock, then restore the
saved IRQ state.

The generic `SpinLock<T>` now has physical Pi 5 proof. The accepted final
hardware run shows the boot CPU and logical cores 1, 2, and 3 in the same
cacheable EL2 stage-1 regime before shared lock access. Each secondary
contended on the generic lock for 64 iterations, reported stable identity and
`ok=true`, and the final invariant reported `counter=192 expected=192`,
`participants=3`, `diag-participants=3`, `errors=0`,
`mixed-cache-mmu=false`, and
`classification=pi5-smp-lock-cache-coherence-complete`.

The production scheduler remains single-core. Phase 6.2 does not make
`src/scheduler.rs`, runtime-console output, descriptor state, userspace state,
or any device driver SMP-safe.

## Evidence Reconciliation

The source inventory separated local IRQ masking, SMP mutual exclusion, memory
ordering, and explicit cache maintenance. That separation held through the
implementation and hardware proof: the generic lock does not hide cache
maintenance, and early secondary handoff state still uses a named cacheable-MMU
handoff boundary before lock contention begins.

The QEMU SMP lock contention smoke is accepted as substitute evidence. Under
QEMU virt with `-smp 4`, secondaries 1, 2, and 3 start through the accepted
PSCI trampoline path, contend on the shared `SpinLock<T>` for 64 iterations
each, and report `counter=192 expected=192 participants=3 errors=0` with
`classification=qemu-smp-lock-contention-complete`.

The first Pi 5 lock investigation produced useful rejected classifications
rather than acceptance. It proved that candidate staging, TFTP fetch, and
serial cursor semantics had to be separated, then identified a real blocker:
secondaries were entering the lock diagnostic in a non-cacheable/no-MMU regime
while the boot CPU was cacheable. The subsequent handoff task accepted only the
secondary cacheable-MMU gate, not the lock proof, because the same run still
had a report-invariant failure.

The final Pi 5 lock proof is accepted as serialized hardware evidence. The
archive SHA256 is
`73041969803f1153a4277d0f56700df08022451a486cd7088ceabe654e953910`, the
kernel SHA256 is
`e28596b5f259775c4c239c3e18b57e3d61d24ff453aa3c762c879e38075f7278`, and the
kernel size is 96,824 bytes. The Pi 5 fetched the staged boot tree, emitted the
candidate-only serial transcript, passed the invariant above, and the pre-run
snapshot `pre-phase6-pi5-smp-lock-final-proof-20260525T033151Z` was restored.

## Retained and Retired Surfaces

Retained as regression gates:

- `cargo fmt --all -- --check` and `cargo -Zjson-target-spec test` for
  primitive and no_std invariant coverage;
- `scripts/qemu-smoke.sh` for the broad QEMU boot smoke;
- `scripts/qemu-smp-lock-contention-smoke.sh` for substitute SMP lock
  contention;
- `scripts/rpi5-smp-lock-cache-coherence-image.sh` and
  `scripts/rpi5-smp-lock-cache-coherence-boot-tree.sh` as the explicit Pi 5
  proof image path when a future supervisor task needs serialized hardware
  regression evidence;
- `scripts/rpi5-archive-review.sh` for Pi 5 archive/header inspection.

Retained as kernel surfaces:

- `src/smp_sync.rs` for `SpinLock<T>`, guard ownership, IRQ-save
  composition, and `smp_full_barrier()`;
- the secondary cacheable-MMU handoff in the Pi 5 diagnostic path, because the
  Pi 5 hardware proof depends on secondaries joining the boot CPU's cacheable
  EL2 stage-1 regime before generic shared lock state is touched.

Retired or kept historical only:

- the temporary entry-discriminator build flag, assembly/Rust markers, and
  entry-discriminator scripts removed by the scaffolding quarantine task;
- stale and rejected Pi 5 lock-proof evidence files except as historical
  evidence explaining the final accepted discriminator sequence.

## Deferred Work

The following remain explicitly deferred to later supervisor-planned tasks:

- scheduler migration and any use of `SpinLock<T>` inside the production
  scheduler;
- shared run queues, per-core run queues, task migration, load balancing, and
  scheduler fairness across cores;
- cross-core wakeups, IPIs, cross-core preemption, and per-core timer ownership
  for preemption;
- userspace, EL0, syscalls, descriptor tables, file descriptors, and user/kernel
  copy policy;
- filesystem behavior, program loading, libc/Rust std support, portable
  userland, and local shell behavior;
- runtime-console concurrency, UART interrupts, blocking I/O, and
  descriptor-facing TTY behavior beyond the accepted Phase 5 diagnostic path;
- RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH,
  and Ethernet.

## Remaining Risks

The first accepted lock is intentionally narrow. It proves mutual exclusion and
ordering for a bounded counter/report diagnostic, not scheduler correctness,
long-running fairness, lock ordering across subsystems, deadlock policy,
interrupt-time contention policy beyond the IRQ-save composition, or safe
printing/allocation/blocking while locks are held.

The Pi 5 proof depends on secondaries joining the cacheable-MMU regime before
generic shared state is used. Future work that introduces DMA, uncached memory,
device memory, RP1/PCIe, or mixed memory attributes must define a separate
cache and coherency policy rather than treating `SpinLock<T>` as a cache
maintenance primitive.

## Next Recommendation

Milestone 6.2 is closed for the current SMP-safe primitive boundary. The next
supervisor-planned slice should stay in Phase 6 and start with a bounded source
inventory for scheduler migration readiness, covering shared scheduler state,
run-queue ownership, lock placement, cross-core wakeup/IPI requirements, and
per-core timer/preemption interactions before any implementation begins.

The worker must not start that slice until the supervisor creates or promotes
an explicit durable task.

## Validation

- static inspection: `git status --short` was clean before checkpoint edits.
- static review: inspected the Phase 6.2 task records, accepted hardware
  evidence summaries, `docs/src/roadmap.md`,
  `docs/src/decisions/README.md`, `docs/src/architecture/scheduler.md`,
  `docs/src/project/phase6-smp-safe-primitives-source-inventory.md`, and
  `docs/src/project/phase6-secondary-cacheable-mmu-handoff-source-inventory.md`.
- whitespace inspection: `git diff --check` passed after checkpoint edits.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests were not required because this checkpoint changes only
  Markdown documentation and durable task state.
