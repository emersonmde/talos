# Phase 6 Secondary-Core Bring-Up Source Inventory

Status: accepted as the Phase 6.1 source inventory and bring-up contract before
any SMP implementation.

This checkpoint defines the first secondary-core bring-up contract for Talos. It
does not add code, publish a boot archive, power-cycle hardware, acquire the
hardware lock, or start scheduler migration, load balancing, EL0, syscalls,
filesystem, networking, SSH, descriptors, shell behavior, or SMP-safe lock work.

## Source Inventory

Repository sources:

- The roadmap names Phase 6.1 as secondary-core bring-up and says PSCI is the
  primary path. Spin-table and custom mailbox bring-up remain fallback research.
- `docs/src/project/reference-notes.md` records Raspberry Pi Linux device-tree
  evidence that the Pi 5 advertises PSCI 1.0 with SMC and `cpu_on`
  `0xc4000003`.
- `src/arch/aarch64/boot.S` preserves the firmware-provided `x0` DTB pointer
  across BSS clearing and stack setup, then calls `rust_entry`.
- `src/boot/mod.rs` currently records `BootInfo.primary_core = 0`; Talos does
  not yet read `MPIDR_EL1` or map affinity values to CPU identities.
- `src/arch/aarch64/mod.rs` can read `CurrentEL`, and current Pi 5 memory and
  cache setup is EL2-only.
- `src/target/mod.rs`, `src/target/qemu_virt.rs`, and `src/target/rpi5.rs`
  keep target services board-specific while sharing AArch64 code.

Primary and advisory external sources:

- Raspberry Pi Linux `bcm2712.dtsi` on `rpi-6.12.y` describes four
  `arm,cortex-a76` CPU nodes with `enable-method = "psci"`. The CPU node
  `reg` values are `0x000`, `0x100`, `0x200`, and `0x300`, which are the
  first MPIDR affinity values Talos should expect on Pi 5.
- The same DTSI has a `/psci` node with `method = "smc"` and compatible values
  `arm,psci-1.0` and `arm,psci-0.2`.
- The same DTSI keeps the accepted Phase 4 interrupt fact intact: BCM2712 uses
  GIC-400/GICv2, and the GIC node targets four CPUs.
- A QEMU 9.2.0 `virt,gic-version=2,virtualization=on` DTB generated with
  `-cpu cortex-a76 -smp 4` contains `psci`, `arm,psci-1.0`,
  `arm,psci-0.2`, `cpu@0` through `cpu@3`, `enable-method`, and `cpu_on`
  strings. `dtc` was unavailable in this container, so this checkpoint treats
  that as static source inspection, not decoded DTB proof.
- The ARM PSCI specification remains the authority for PSCI calling convention
  and return-code semantics. Talos should use the SMC conduit on Pi 5 unless
  later source or hardware evidence contradicts the firmware DTB.

## Bring-Up Contract

PSCI is the default Phase 6.1 mechanism. The boot CPU must call PSCI `CPU_ON`
for secondary CPUs using the target CPU MPIDR-derived affinity value, the
physical entry address of a narrow secondary entry trampoline, and a context
argument. Spin-table, VideoCore mailbox, or custom mailbox paths are only
fallback research if PSCI source evidence or hardware evidence fails.

Before a secondary core can be counted alive, it must prove all of these
invariants:

- core identity: the core reads `MPIDR_EL1`, masks implementation-reserved
  bits, maps the affinity value to a Talos logical CPU index, and reports both
  values;
- stack ownership: the core runs on a stack reserved for exactly that logical
  CPU, separate from the boot stack and other secondary stacks;
- per-core state registration: the core publishes a bounded state transition
  such as `parked -> entered -> stack-ready -> registered -> handoff-ready`;
- controlled handoff: the core parks in a `wfe` loop or runs a named
  supervisor-planned kernel-thread workload, not arbitrary scheduler work.

The first implementation should keep secondary-core diagnostics boot-only and
serial-bounded. It may print compact lines through the existing runtime console
after the boot CPU has initialized the console path, or use a deliberately
serialized diagnostic buffer if concurrent printing would confuse evidence.

## Evidence Levels

QEMU substitute evidence:

- The QEMU task may use `virt,gic-version=2,virtualization=on`, `-cpu
  cortex-a76`, and `-smp 4` to prove that Talos can identify multiple CPU
  nodes/source facts and keep the secondary-core contract separated from Pi 5
  hardware claims.
- QEMU evidence is not Pi 5 hardware proof. It can check target-independent
  parsing, MPIDR mapping helpers, state-machine tests, and archive-free smoke
  behavior.
- Accepted QEMU discriminator result: `scripts/qemu-secondary-core-discriminator.sh`
  builds `TALOS_QEMU_SECONDARY_CORE_DISCRIMINATOR=1` and runs QEMU virt with
  EL2 virtualization, GICv2, Cortex-A76, and four CPUs. PSCI `CPU_ON` through
  SMC returns success for logical CPUs 1, 2, and 3. Each secondary reaches
  `handoff-ready`, reports MPIDR affinities `0x1`, `0x2`, and `0x3`, runs on
  its reserved 4 KiB stack slot, and parks. This proves the QEMU/substitute
  discriminator only; it does not weaken the Pi 5 hardware proof requirements.
- Runtime discriminator note: an exploratory HVC `CPU_ON` call from the EL2
  diagnostic path raised a current-SPx synchronous exception with ESR
  `0x5a000000` under QEMU. The accepted QEMU proof therefore uses SMC for this
  EL2 boot model while preserving the Pi 5 firmware/DTB SMC contract.
- Implemented per-core ownership boundary: `src/smp.rs` now defines four
  possible cores, 4 KiB secondary kernel stack slots, a bounded lifecycle
  `parked -> entered -> stack-ready -> registered -> handoff-ready`, per-core
  atomic identity/state records, stack-slot validation, and the Pi 5 MPIDR
  affinity map `0x000`, `0x100`, `0x200`, `0x300`. The QEMU discriminator uses
  this shared state/stack boundary behind its focused diagnostic gate. This is
  not scheduler migration, SMP-safe locking, cross-core wakeup, or Pi 5 hardware
  proof.

Pi 5 hardware evidence:

- Hardware proof must serialize with `hardwareTestLock`.
- The proof must capture the candidate archive path and SHA256, kernel SHA256
  and size, pre-run boot tree snapshot, TFTP delta showing the candidate image
  was served, serial transcript, classification, restore evidence, and
  post-hardware review.
- Acceptance requires all four Cortex-A76 cores to report alive with distinct
  MPIDR/logical CPU identities, distinct stack ownership, per-core registration,
  and a controlled handoff result.
- First hardware attempt for
  `phase6-pi5-psci-secondary-core-alive-proof-20260524` built and served the
  focused PSCI candidate archive, but serial capture after two candidate
  power-cycles contained only a NUL/newline and no Talos PSCI alive transcript.
  This is classified as `serial-capture-or-pre-entry-output-ambiguity`, not as
  accepted Pi 5 PSCI proof.
- Follow-up pre-entry/early-serial discriminator evidence for the same task
  proved the Pi fetched and entered the candidate image: serial showed
  `asm_start`, `asm_pre_rust_entry`, `rust_entry`, normal Talos boot output,
  and PSCI `CPU_ON` result 0 for target affinities `0x100`, `0x200`, and
  `0x300`. The secondary records still remained `parked` with no owned
  stack/state, so this is classified as
  `pi5-psci-started-but-state-or-stack-incomplete`, not accepted Pi 5 PSCI
  proof.
- A secondary-entry discriminator then added fixed UART10 markers at the first
  secondary trampoline instruction and immediately before the Rust secondary
  entry. Corrected hardware evidence again proved candidate TFTP fetch,
  `rust_entry`, and PSCI `CPU_ON` result 0 for `0x100`, `0x200`, and `0x300`,
  but neither secondary-entry marker appeared. The current classification is
  `pi5-psci-accepted-secondary-entry-not-observed`, so Pi 5 secondary-core
  alive proof remains unaccepted pending a PSCI-entry-address or PSCI-state
  discriminator.
- A follow-up PSCI state discriminator added `PSCI_VERSION`, `PSCI_FEATURES`,
  and `AFFINITY_INFO` calls around the same CPU_ON path. The corrected
  hardware run proved the Pi fetched the 91,000-byte candidate image from TFTP,
  but serial did not show BL31, `TALOS: asm_start`, `TALOS: rust_entry`, or
  PSCI state-discriminator lines for that boot. The current classification is
  `pi5-state-discriminator-candidate-fetched-no-bl31-or-asm-entry`; this is
  not accepted Pi 5 PSCI proof and needs local image/disassembly review or a
  smaller state discriminator before another hardware acceptance attempt.
- Local image/disassembly review found the state-discriminator image header,
  early markers, and entry symbols structurally sound. A smaller discriminator
  with explicit SMC clobbers and only a post-`CPU_ON` `AFFINITY_INFO` probe was
  then fetched repeatedly at 90,416 bytes, but current-run serial still did not
  show BL31-to-Talos entry or the new affinity-state lines. The current
  classification is
  `pi5-minstate-discriminator-candidate-fetched-no-current-entry`; the original
  CPU_ON-success/no-secondary-entry result remains unexplained and unaccepted.
- A comparison rerun proved the last Talos-entry image could still reach
  secondary trampoline markers, and a Rust-entry marker run proved each
  secondary reached `talos_rpi5_secondary_entry` and emitted a state-published
  marker while the primary still saw parked zero records. The accepted fix
  keeps the diagnostic's per-core state cache-visible by cleaning secondary
  updates to the point of coherency and invalidating the primary view before
  snapshots. The final Pi 5 hardware run served the 90,784-byte candidate
  before restore and serial proved cores 1-3 with MPIDR affinities `0x100`,
  `0x200`, and `0x300`, owned stack slots, `handoff-ready` state, and
  classification `pi5-psci-smc-secondary-cores-alive`. The Pi 5 PSCI
  secondary-core alive proof is accepted for boot-time alive/park behavior.

## Deferred Work

This contract does not accept SMP-safe primitives, scheduler migration,
cross-core wakeups, load balancing, inter-processor interrupts, concurrent
console writes, userspace, descriptors, syscalls, filesystem, networking, SSH,
RP1/PCIe/DMA work, or a local shell.

## Next Worker Task

The accepted QEMU discriminator task is
`phase6-qemu-secondary-core-bringup-discriminator-20260524`. The per-core
state/stacks boundary is accepted in
`phase6-per-core-state-and-stacks-20260524` and recorded in
`tasks/2026-05-24-phase6-per-core-state-and-stacks.md`. The accepted Pi 5
hardware proof is
`phase6-pi5-psci-secondary-core-alive-proof-20260524`. The next queued Phase
6.1 task remains separate from SMP-safe locking and scheduler migration unless
the durable supervisor task explicitly broadens that scope.

## Validation

- static inspection: `git status --short` was clean before documentation edits.
- static inspection: repository docs, source files, generated QEMU DTB strings,
  and Raspberry Pi Linux `bcm2712.dtsi` were inspected.
- fmt/lint/typecheck: `git diff --check` passed after documentation edits.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests were not required because this checkpoint changes only
  Markdown documentation and durable task state.
