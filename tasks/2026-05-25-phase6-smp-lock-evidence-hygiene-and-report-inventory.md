# Phase 6 SMP Lock Evidence Hygiene and Report Inventory

Task: `phase6-smp-lock-evidence-hygiene-and-report-inventory-20260525`

Status: accepted.

## Scope

This task reconciles the paused Pi 5 SMP lock proof after the accepted
secondary cacheable-MMU handoff proof. It does not change Rust behavior, rerun
hardware, or accept the generic lock/coherence proof.

## Failed Invariant

The post-handoff Pi 5 run proved that the secondary cores entered the same
cacheable EL2 stage-1 regime as the boot CPU before touching the lock
diagnostic, but the final lock proof still failed the per-core identity/report
invariant.

Evidence:

- Handoff proof evidence:
  `tasks/evidence/2026-05-25-pi5-secondary-cacheable-mmu-handoff-proof/`.
- Key serial extract:
  `serial-key-lines.txt`.
- Summary:
  `summary.md`.

Observed facts:

- Boot CPU: `boot-sctlr-el2=0x0000000030c51835`,
  `boot-cacheable-mmu=true`.
- Handoff plan: `mair-el2=0x00000000000004ff`,
  `tcr-el2=0x0000000000053510`,
  `ttbr0-el2=0x000000002f000000`,
  `sctlr-el2=0x0000000030c51835`,
  `cacheable-mmu=true`.
- Logical cores 1, 2, and 3 reached `workload-complete` with
  `diag-progress=64`, `diag-attempts=64`, `diag-timeouts=0`,
  `diag-releases=64`, and
  `diag-sctlr-el2=0x0000000030c51835`.
- Final shared lock state was visible:
  `counter=192 expected=192 participants=3 diag-participants=3 errors=0`,
  `lock-available=true`, `generic-state-visible=true`, and
  `mixed-cache-mmu=false`.
- Final per-core identity reports for logical cores 1 and 2 were zeroed:
  `context=0 mpidr=0x0000000000000000 affinity=0x0 sp=0x0000000000000000`,
  even though both had `lock-count=64`, `progress=64`, and complete
  diagnostic counters.
- Logical core 3 reported the expected identity and stack fields:
  `context=3`, `mpidr=0x0000000081000300`, `affinity=0x300`, and a
  stack pointer inside the logical-3 slot.

Classification:
`pi5-smp-lock-cache-coherence-invariant-failed`.

## Evidence Hygiene

Tracked accepted evidence before this task:

- `tasks/evidence/2026-05-24-pi5-smp-lock-cache-coherence-proof/summary.md`.
- The accepted handoff proof task and evidence under
  `tasks/evidence/2026-05-25-pi5-secondary-cacheable-mmu-handoff-proof/`.

Kept and committed as durable lock-proof evidence:

- A/B known-good control evidence:
  `abcontrol-secondary-workload-*`,
  `ab-discriminator-result.json`.
- Entry-discriminator evidence:
  `ablock-entrydisc-*`, `entrydisc*`.
- Full lock proof and lock/cache-regime evidence:
  `lockproof-clean-*`, `lockproof-diag1-*`,
  `lockproof-diag2-*`, `lockproof-immediate-*`.
- Earlier fetched/no-entry rerun evidence:
  `rerun*`, plus the unprefixed first-run files.
- The updated lock-proof `summary.md`.

Removed from the Talos worktree and quarantined outside the repository:

- 20 accidental template-literal files named `${name}-*`.
- Quarantine location:
  `/opt/strider/openclaw/current/workspace/trash/talos-20260525-smp-lock-template-literals/`.

Rationale: the template-literal files are script placeholder output and do not
identify a hardware run. The prefixed run files preserve the actual serial,
TFTP, publish, restore, and classification evidence.

## Source Inventory

Writer path:

- `src/target/rpi5.rs::talos_rpi5_secondary_entry` maps MPIDR affinity to a
  logical CPU, calls `PerCoreState::enter(context, mpidr, affinity)`, records
  stack readiness, marks the core registered, performs the cacheable-MMU
  handoff, marks handoff ready, and then calls
  `run_smp_lock_contention_secondary`.
- `run_smp_lock_contention_secondary` marks workload running, records
  diagnostic phase/progress, acquires the shared `SpinLock`, increments the
  shared counter and per-core lock count, records progress, and marks workload
  complete.
- `src/smp.rs::PerCoreState` stores identity fields and workload progress in
  per-core atomics. The writer cleans those cache lines to PoC after state
  transitions and progress updates.

Reader path:

- `run_smp_lock_cache_coherence_proof` starts CPU_ON for logical cores 1-3,
  waits for `CoreLifecycle::WorkloadComplete`, then invalidates and snapshots
  each `SECONDARY_CORE_STATES[logical_cpu]`.
- The final report requires lifecycle complete, matching context, MPIDR
  affinity mapping, stack ownership, workload progress 64, and lock count 64.

Reset path:

- `run_smp_lock_cache_coherence_proof` resets
  `SECONDARY_CORE_STATES`, `SMP_LOCK_CONTENTION_STATE`, and
  `SMP_LOCK_DIAGNOSTIC_*` before publishing the secondary cacheable-MMU
  handoff plan and issuing CPU_ON.
- There is no intentional reset after CPU_ON in the final report path.

Ownership finding:

- The shared lock state and diagnostic arrays reached a coherent final state
  for all three secondaries after handoff. The remaining failure is narrower:
  `PerCoreState` identity fields for logical cores 1 and 2 were no longer
  visible to the boot CPU at final report time, while their lifecycle/progress
  and the separate lock diagnostic state were visible.
- The smallest likely source surface is the publication/maintenance of
  `PerCoreState` identity fields during secondary entry and later workload
  completion, not the generic `SpinLock<T>` contract and not scheduler work.

Concrete unknown:

- Static inspection does not prove whether logical cores 1 and 2 lost identity
  fields because their early `enter`/stack writes were never cleaned after
  the cacheable-MMU handoff, were overwritten by a later reset-like path, or
  were read from stale cache lines while lifecycle/progress came from later
  cleaned stores.

## Next Recommendation

Promote `phase6-smp-lock-report-invariant-core-20260525` next. Keep the
target surface specific:

- Add a narrow report-invariant fix/discriminator around
  `PerCoreState` publication after cacheable-MMU handoff.
- Prefer an implementation that republishes or re-cleans identity fields after
  the secondary cacheable-MMU handoff and before lock work, then covers the
  expectation with a small unit/static test where possible.
- Do not alter the generic `SpinLock<T>` contract unless the follow-up proves
  the lock itself corrupted protected data.
- Do not broaden into scheduler migration, shared run queues, cross-core
  wakeups, IPIs, userspace, descriptors, filesystem, networking, SSH, shell,
  RP1/PCIe, or DMA policy.

## Validation

- static inspection: inspected `src/target/rpi5.rs`, `src/smp.rs`, and
  `src/smp_sync.rs` for writer, reader, reset, and publication ownership.
- static inspection: inspected the accepted handoff proof and paused lock
  proof evidence summaries.
- static inspection: `git status --short` was inspected before evidence
  hygiene; the worktree contained only untracked lock-proof evidence.
- evidence hygiene: quarantined 20 accidental `${name}-*` placeholder files
  outside the repository and preserved real run evidence.
- evidence hygiene: added a repository attribute so raw hardware serial
  `.txt` captures preserve firmware CRLF and spacing without failing
  whitespace checks.
- whitespace inspection: `git diff --check` passed.
- whitespace inspection: `git diff --cached --check` passed.
- static inspection: `mdbook build` was not run because `mdbook` is
  unavailable in the container.
- hardware: no hardware commands were run for this task.

## Acceptance

Accepted as an evidence hygiene and report-invariant inventory only. The Pi 5
SMP lock cache/coherence proof remains unaccepted until the next bounded task
fixes or discriminates the `PerCoreState` identity publication failure and a
separate serialized Pi 5 proof passes or is decisively closed as a blocker.
