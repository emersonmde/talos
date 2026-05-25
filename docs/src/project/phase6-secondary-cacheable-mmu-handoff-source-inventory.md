# Phase 6 Secondary Cacheable MMU Handoff Source Inventory

Status: accepted as the source inventory and contract for the secondary-core
cacheable EL2 stage-1 handoff required before the Pi 5 SMP lock hardware proof
can resume.

This checkpoint is documentation-only. It does not implement the handoff,
publish a boot archive, acquire the hardware lock, power-cycle the Pi 5, change
the generic SpinLock contract, migrate scheduler state, add shared run queues,
add IPIs or cross-core wakeups, enter EL0, add descriptors, filesystem
behavior, networking, SSH, shell behavior, RP1/PCIe, or DMA driver policy.

## Blocked Proof Linkage

The current Pi 5 SMP lock proof is blocked by
pi5-smp-lock-cache-coherence-invalid-mixed-cache-mmu-regime, not by accepted
lock semantics alone. The decisive discriminator in
tasks/evidence/2026-05-24-pi5-smp-lock-cache-coherence-proof/summary.md shows:

- the boot CPU reached the proof with SCTLR_EL2=0x0000000030c51835, meaning
  stage-1 translation, instruction cache, and data cache are enabled;
- secondary cores 1, 2, and 3 reached the proof before first lock attempt with
  SCTLR_EL2=0x0000000030c50830, meaning stage-1 translation and instruction
  cache are enabled but data cache is not;
- all secondaries were still at diag-phase=before-lock-attempt,
  diag-attempts=0, and diag-releases=0, so the observed hardware setup is
  invalid before generic SpinLock contention begins.

The accepted QEMU lock contention smoke remains valid QEMU/substitute evidence.
The accepted Pi 5 secondary controlled workload also remains valid, but it used
explicit per-core clean/invalidate publication and did not contend on generic
shared lock state.

## Source Inventory

Boot CPU memory/cache path:

- src/boot/rpi5.rs plans the low-memory translation layout, populates the early
  tables, then enables EL2 stage-1 translation, instruction cache, and data
  cache before running Phase 6 Pi 5 diagnostics.
- src/memory_map/translation.rs defines the accepted early map: low memory
  0x0..0x40000000 as inner-shareable normal memory and BCM2712 local MMIO
  0x107c000000..0x1080000000 as Device-nGnRE.
- src/memory_map/translation.rs also defines MAIR_EL2=0x4ff,
  TCR_EL2=0x53510, TTBR0_EL2 pointing at the root table, SCTLR_EL2.M, and
  separate SCTLR_EL2.I and SCTLR_EL2.C enable plans.
- src/arch/aarch64/mod.rs writes MAIR_EL2, TCR_EL2, and TTBR0_EL2, invalidates
  EL2 translations with tlbi alle2, sets SCTLR_EL2.M, then enables instruction
  and data caches with explicit dsb sy / isb ordering.

Secondary entry path:

- src/arch/aarch64/boot.S defines the current secondary trampoline. It receives
  the logical CPU id in x0, selects a reserved 4 KiB secondary stack, calls the
  Rust secondary entry, and parks on return.
- The trampoline does not currently install the boot CPU's EL2 translation
  registers, invalidate translations, or enable SCTLR_EL2.C for secondaries.
- src/smp.rs owns per-core lifecycle, identity, stack, and workload-progress
  records. It uses acquire/release atomics plus explicit dc cvac and dc ivac
  diagnostic publication where accepted Pi 5 proofs require cache-visible
  handoff.
- src/target/rpi5.rs owns the Pi 5 PSCI SMC CPU_ON flow for affinities 0x100,
  0x200, and 0x300, and currently runs the lock proof only after boot CPU
  cache/MMU enablement.

Accepted architecture documentation:

- docs/src/architecture/memory.md records the accepted boot CPU stage-1 map,
  register plan, and cache-enable sequence. The data-cache-enabled line reports
  SCTLR_EL2=0x30c51835 on the accepted normal Pi 5 path.
- docs/src/project/phase6-smp-safe-primitives-source-inventory.md explicitly
  keeps cache maintenance outside the generic SpinLock API. That remains
  correct: the lock should not hide MMU/cache handoff or early-boot cache
  maintenance policy.

## Handoff Contract

Before any Pi 5 hardware proof may claim generic shared lock contention on
secondary cores, each participating secondary must enter an equivalent
cacheable EL2 stage-1 regime to the boot CPU for the memory it will touch.

The required invariant is:

- MAIR_EL2 uses the accepted attr index 0 normal WBWA and attr index 1
  Device-nGnRE values;
- TCR_EL2 uses the accepted 48-bit VA/PA, 4 KiB granule, inner-shareable,
  write-back/write-allocate walk-cacheability plan;
- TTBR0_EL2 points at the same populated early root table used by the boot CPU,
  or at an equivalent table with the same attributes for the shared lock,
  per-core state, stacks, kernel text/data, and UART/GIC MMIO windows;
- EL2 translations are invalidated after installing translation registers and
  before relying on the new regime;
- SCTLR_EL2.M, SCTLR_EL2.I, and SCTLR_EL2.C are set before a secondary touches
  generic shared atomic or SpinLock state;
- the transition is ordered with dsb sy / isb around register writes, cache/TLB
  invalidation, and visible state publication.

The first implementation should reuse the existing boot CPU map and register
plan rather than invent a second memory policy. If the implementation cannot
share the exact root table, it must justify the narrower equivalent and prove
that all shared lock/progress/stack/code/data addresses use the same normal
cacheable attributes on all participating cores.

## Hazards

- A boot CPU with SCTLR_EL2.C=1 and a secondary with SCTLR_EL2.C=0 is not a
  valid proof environment for generic cached shared atomics.
- Plain acquire/release atomics do not repair a mixed cacheability regime.
- Explicit clean/invalidate is still valid for diagnostic publication, but it
  is not the generic lock contract and should not be hidden inside SpinLock.
- The handoff must not enable broad high-memory, lower-EL, DMA, or RP1/PCIe
  policy. The accepted low-memory and BCM2712 local-MMIO map remains the
  near-term boundary.

## Follow-Up Tasks

Implementation task: phase6-secondary-cacheable-mmu-handoff-core-20260524.

- Add the smallest architecture/target surface needed for secondaries to
  install or inherit the accepted EL2 stage-1 register plan and enable
  instruction/data caches before generic shared-state access.
- Keep it boot-time and Pi 5/QEMU diagnostic-scoped.
- Add local tests or static checks for register-plan identity where practical.
- Do not change SpinLock, scheduler migration, shared run queues, IPIs,
  userspace, descriptors, filesystem, networking, SSH, shell behavior,
  RP1/PCIe, or DMA policy.

Hardware proof task: phase6-secondary-cacheable-mmu-handoff-pi5-proof-20260524.

- After the core task is accepted, run a serialized Pi 5 proof under
  hardwareTestLock.
- Capture archive/kernel digests, TFTP fetch evidence, serial output showing
  secondary SCTLR_EL2/cacheable-MMU state before lock attempt, classification,
  and restore proof.
- Only after that proof is accepted may the blocked
  phase6-pi5-smp-lock-cache-coherence-proof-20260524 resume.

## Validation

- static inspection: git status --short before edits showed existing
  unaccepted Pi 5 lock-proof diagnostic changes; this task did not modify or
  revert them.
- static inspection: inspected src/boot/rpi5.rs, src/arch/aarch64,
  src/memory_map/translation.rs, src/smp.rs, src/target/rpi5.rs,
  docs/src/architecture/memory.md, Phase 6 project docs, and the accepted SMP
  lock proof evidence summary.
- fmt/lint/typecheck: git diff --check passed after documentation edits.
- static inspection: mdbook was unavailable in the container, so mdBook build
  was not run.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.
