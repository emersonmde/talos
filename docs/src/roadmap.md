# Roadmap

Talos is planned as a long-running Raspberry Pi 5 operating-system program, not as a single bring-up spike. The roadmap is organized around user-visible capabilities and validation gates. Each phase should leave the repository in a buildable, documented state.

The near-term strategy is dual-target:

- talos-aarch64-virt: a fast QEMU virt target for architecture work, tests, and CI.
- talos-rpi5-bcm2712: the physical Raspberry Pi 5 target, validated through the lab controller and serial console.

QEMU does not currently model the Raspberry Pi 5, BCM2712, or RP1. The physical Pi 5 lab is therefore the authority for board behavior. QEMU is still valuable for generic AArch64 boot, exceptions, MMU, scheduler, and pure subsystem tests.

The Pi 5 boot path should follow the normal firmware contract first. The EEPROM bootloader loads the kernel image directly, prefers kernel_2712.img, falls back to kernel8.img, and passes the physical device-tree address in x0 according to the arm64 boot ABI. Talos should implement that handoff before considering any custom boot path.

## Current Status

Talos has moved past first-light and Phase 3 memory/runtime closeout into
source-backed Phase 4 interrupt and timer planning.

Completed:

- Talos project directory created separately from Daedalus.
- mdBook documentation skeleton created.
- Lab controller documented and reachable from OpenClaw at http://talos-lab-api:8080.
- TFTP boot archive publishing and PoE control API are documented.
- Minimal Rust no_std AArch64 kernel skeleton created for QEMU virt.
- Pi 5 target definition and target boundary stubs created.
- Early target service descriptors added for boot info, UART kind, timer kind,
  interrupt-controller kind, MMIO map, and device tree pointer.
- Pi 5 kernel image and boot-tree staging scripts added for local archive
  preparation.
- Physical Pi 5 first-light reached Talos code.
- Readable Talos-origin serial output is available through the lab controller.
- Exception and panic diagnostics report useful AArch64 state.
- The Pi 5 boot path parses firmware handoff state and DTB memory metadata.
- Early EL2 stage-1 translation, instruction cache, and data cache have booted
  on hardware while preserving serial output.
- A no-free bootstrap allocator and narrow Rust alloc-crate diagnostics for
  Box, Vec, String, and alloc-backed formatting have hardware evidence.
- Phase 3 has an accepted closeout checkpoint for the current memory, MMU, and
  kernel-runtime boundary. The checkpoint recommends planning Phase 4 next while
  preserving explicit deferrals for high memory, DMA/cache ownership, lower-EL
  userspace, SMP, filesystem/userland, and networking.
- Phase 4 has a source-backed interrupt/timer inventory naming the first QEMU
  virt and Pi 5 GICv2 plus ARM generic-timer targets.
- QEMU virt has a focused EL2 timer-interrupt smoke: with virtualization
  enabled, CNTHP_*_EL2 raises PPI 10 / INTID 26 through GICv2, the current-EL
  IRQ frame path acknowledges and EOIs it, and execution returns to a bounded
  post-IRQ workload.
- Pi 5 has a focused EL2 timer-interrupt smoke using the same CNTHP_*_EL2 /
  PPI 10 / INTID 26 shape through GIC-400. Serialized lab evidence shows the
  candidate image was fetched, the IRQ handler acknowledged and EOI'd INTID 26,
  and execution returned to a bounded post-IRQ workload.
- The Phase 4 timer-smoke checkpoint reconciles the accepted QEMU and Pi 5
  evidence, and monotonic tick accounting now reprograms the EL2 physical
  timer for four periodic ticks on QEMU and Pi 5 before reporting outside the
  IRQ path.
- Phase 4.1/4.2 has a pre-scheduler closeout checkpoint covering the accepted
  interrupt-controller, EL2 physical timer, periodic tick, and single-core
  interrupt-mask/restore boundary. Milestone 4.3 may start with a bounded
  scheduler-shape task that checks task/process terminology against the early
  POSIX note before committing scheduler structs.
- Phase 4.3 scheduler shape is accepted as a single-core, kernel-thread-first
  boundary. The next bounded implementation step is scheduler structs and a
  runnable queue, without context switching, preemption time slicing, SMP,
  userspace, file descriptors, console/TTY, filesystem, networking, or SSH.
- Phase 4.3 now has the first scheduler data structures: scheduler-local task
  IDs, kernel-thread state, per-task kernel stack and context placeholders, an
  optional future process-owner hook, a fixed single-core runnable queue, and
  unit tests for the queue/state invariants. Context switching, sleep queues,
  preemption, SMP, userspace, descriptors, console/TTY, filesystem, networking,
  and SSH remain deferred.
- Phase 4.3 has a documented EL2 cooperative context-switch contract for
  single-core kernel threads. The first QEMU context-switch smoke is accepted:
  two kernel-thread contexts with separate stacks make bounded progress through
  the AArch64 save/restore primitive, and the implementation reports switch,
  current-task, and runnable-task state outside the switch hot path.
- Phase 4.3 voluntary-yield dispatch is accepted in QEMU. The single-core
  scheduler can requeue a running task, select the next runnable task, count
  voluntary yields and dispatch switches, and cross the cooperative switch
  boundary while keeping the short scheduler mutation window IRQ-masked.
  Timer-driven preemption and async exception-frame switching remain deferred.
- The Phase 4.3 preemption-entry policy checkpoint is accepted. The next bounded
  task may attempt a QEMU-only timer-preemption smoke that preserves
  acknowledge/reprogram/EOI ordering, keeps scheduler switching and diagnostics
  out of the IRQ hot path, and remains single-core EL2 kernel-thread only.
- Phase 4.3 QEMU timer-preemption smoke is accepted. EL2 timer ticks now record
  bounded preemption requests in the IRQ hot path, then kernel-thread code
  performs scheduler dispatch and context switching outside IRQ context. Two
  QEMU kernel threads make progress from timer-driven preemption with zero
  voluntary-yield dispatches.
- Phase 4.3 Pi 5 timer-preemption hardware proof is accepted. The physical Pi
  5 fetched the 103,152-byte candidate kernel over TFTP, reached the EL2
  timer-preemption smoke, and reported task1=3, task2=3, ticks=6, requests=6,
  handled=6, timer-preemptions=6, dispatch-switches=6, voluntary-yields=0,
  INTID 26, unexpected=0, and PASS before the pre-run boot snapshot was
  restored.
- Phase 4.3 scheduler/preemption contract consolidation is accepted. The
  production boundary is the single-core scheduler data model, short
  IRQ-masked scheduler mutation windows, and an IRQ hot path limited to
  acknowledge/classify/tick/request/reprogram/EOI. The QEMU and Pi 5
  timer-preemption boot images remain validation surfaces, not supported
  kernel interfaces.
- Phase 4 closeout is accepted. The checkpoint reconciles the accepted QEMU and
  Pi 5 interrupt/timer/preemption evidence, names remaining deferrals and
  risks, and allows Phase 5 planning to start with a bounded local console
  device-model source inventory.
- Phase 5 console device-model source inventory is accepted. The current early
  logging surfaces are inventoried, the early/runtime console ownership
  boundary is documented, and descriptor/TTY compatibility constraints are
  named without implementing descriptor tables, input, userspace, filesystem,
  networking, SSH, or shell behavior.
- Phase 5 runtime console write core and write-result contract are accepted.
  Normal kernel output now routes through the named
  `runtime_console::write_default_console_output` boundary while preserving
  `print!` / `println!` and the existing target-owned polling PL011 backends.
  Pi 5 normal serial output is intended to be preserved through the existing
  firmware-preserved UART10 backend.
- Phase 5 default console identity is accepted. The output-side runtime console
  is named `runtime-console0`; later `stdout` and `stderr` descriptors
  should attach to that console through descriptor-owned handles instead of
  calling target backends directly.
- Phase 5 console input-source inventory is accepted. QEMU PL011 polling RX is
  the recommended first input implementation proof; Pi 5 input should follow
  only with serialized hardware evidence, preferably starting from the accepted
  UART10 console path before revisiting RP1 UART0 risk.
- Phase 5.1 console model checkpoint is accepted. The console model is
  output-capable and input-planned: normal diagnostics route through
  runtime-console0, target modules own QEMU/Pi 5 PL011 backend selection,
  and Milestone 5.2 may start with a documentation-only TTY/stdio shape task.
- Phase 5.2 TTY/stdio shape is accepted as a design boundary. Raw mode,
  canonical-lite line assembly, newline/backspace/echo/control-character
  policy, and descriptor-facing stdin/stdout/stderr shape are documented.
- Phase 5.2 QEMU polling TTY RX, the shared line-discipline core, the internal
  console input result contract, and the Pi 5 UART10 polling RX proof are
  accepted. QEMU and Pi 5 both use the same injected byte sequence through the
  runtime-console/TTY boundary and report deterministic echo, line, truncation,
  and control-event evidence without adding descriptors, syscalls, userspace,
  shell behavior, UART interrupts, networking, SSH, or scheduler blocking I/O.
- The Phase 5.2 TTY/stdio closeout checkpoint is accepted. The next
  supervisor-planned slice should be a Milestone 5.3 local kernel diagnostic
  command-channel source inventory, not an implementation shortcut around the
  accepted runtime-console and TTY boundaries.
- Phase 5.3 local diagnostic command-channel source inventory is accepted. The
  command channel must consume completed TTY lines, write bounded responses
  through runtime-console0, classify existing diagnostics before exposing them,
  and remain separate from descriptor/syscall/POSIX shell semantics.
- Phase 5.3 diagnostic command-channel contract is accepted. The
  target-independent parser/dispatcher consumes complete TTY lines, bounds
  command and argument tokens, exposes deterministic help/list/status responses,
  reports unknown and malformed commands, and keeps the response sink attached
  to runtime-console0 without adding a shell, descriptor table, syscall ABI,
  filesystem command execution, networking, SSH, SMP, UART interrupts, or
  scheduler blocking I/O.
- Phase 5.3 QEMU diagnostic command-channel smoke is accepted. The QEMU serial
  transcript proves `help`, `list`, deterministic unknown-command handling,
  and `status` through the accepted polling TTY line path and
  runtime-console0 response sink without adding Pi 5 hardware behavior,
  descriptors, syscalls, userspace shell behavior, filesystem-backed commands,
  networking, SSH, SMP, UART interrupts, or scheduler blocking I/O.
- Phase 5.3 Pi 5 diagnostic command-channel proof is accepted. The serialized
  UART10 hardware transcript proves the same `help`, `list`, `bogus`, and
  `status` command sequence through canonical-lite TTY input and
  runtime-console0 responses, with TFTP evidence tying the output to the
  staged candidate image.
- The Phase 5.3 diagnostic command-channel closeout checkpoint is accepted.
  Milestone 5.3 now has reconciled source inventory, parser/dispatcher
  contract, QEMU smoke, and Pi 5 UART10 hardware proof evidence. The accepted
  command channel remains kernel-owned and diagnostic-only; descriptor tables,
  syscalls, userspace shell behavior, filesystem-backed commands, networking,
  SSH, SMP, UART interrupts, RP1 UART0, and scheduler blocking I/O remain
  deferred.
- Phase 6.1 secondary-core bring-up source inventory and contract is accepted.
  PSCI with the firmware/DTB SMC conduit is the default bring-up path;
  spin-table and custom mailbox approaches remain fallback research. Before
  scheduler work, each secondary core must prove MPIDR/logical identity,
  exclusive stack ownership, per-core state registration, and controlled
  handoff.
- Phase 6.1 QEMU secondary-core discriminator is accepted. Under QEMU virt with
  EL2 virtualization enabled, PSCI `CPU_ON` through SMC starts secondary CPUs
  1, 2, and 3; each reports distinct MPIDR affinity, runs on its reserved
  stack, reaches `handoff-ready`, and parks without claiming Pi 5 hardware
  behavior.
- Phase 6.1 Pi 5 PSCI secondary-core alive proof is accepted. Serialized
  hardware evidence shows the Pi fetched the 90,784-byte candidate image and
  cores 1, 2, and 3 reported MPIDR affinities `0x100`, `0x200`, and
  `0x300`, distinct owned stack slots, `handoff-ready` state, and
  `pi5-psci-smc-secondary-cores-alive` before the pre-run boot snapshot was
  restored.
- Phase 6.1 controlled secondary-core workload is accepted. QEMU and serialized
  Pi 5 hardware evidence show secondary cores 1, 2, and 3 reach
  `workload-complete` with `progress=64 target=64 ok=true` through the
  accepted PSCI/trampoline/stack boundary while the production scheduler
  remains single-core.
- The Phase 6.1 secondary-core bring-up closeout checkpoint is accepted.
  Milestone 6.1 now has reconciled source inventory, QEMU discriminator,
  per-core state/stacks, Pi 5 PSCI alive proof, and controlled secondary-core
  workload evidence. SMP-safe primitives, scheduler migration, shared run
  queues, cross-core wakeups, userspace, descriptors, filesystem, networking,
  SSH, shell behavior, UART interrupts, RP1/PCIe, and DMA/cache policy remain
  deferred.
- Phase 6.2 SMP-safe primitives source inventory and contract is accepted. It
  separates local IRQ masking, SMP mutual exclusion, memory ordering, and cache
  maintenance; carries forward the accepted Pi 5 cache-maintenance lesson; and
  names `phase6-spinlock-barrier-core-20260524` as the first bounded
  implementation task before scheduler migration or shared run queues.
- Phase 6.2 spinlock/barrier core is accepted. `src/smp_sync.rs` provides a
  narrow `SpinLock<T>`, RAII guard, AArch64 IRQ-save lock composition, and a
  named `dmb ish` full-barrier boundary without wiring scheduler migration,
  shared run queues, cross-core wakeups, or cache maintenance into the lock.
- Phase 6.2 QEMU SMP lock contention smoke is accepted. QEMU virt with
  `-smp 4` starts secondary cores 1, 2, and 3 through the accepted PSCI
  trampoline path; each core contends on the shared `SpinLock<T>` for 64
  iterations, and the transcript reports `counter=192 expected=192`,
  `participants=3`, `errors=0`, and
  `qemu-smp-lock-contention-complete`. This remains QEMU/substitute evidence;
  the separate hardware-locked Pi 5 proof below closes the physical
  cache/coherence claim.
- Phase 6.2 Pi 5 SMP lock cache/coherence proof is accepted. Serialized Pi 5
  hardware evidence shows the boot CPU and logical cores 1, 2, and 3 in the
  accepted cacheable-MMU regime before generic lock access; each secondary
  reports stable identity and `ok=true`; the final invariant reports
  `counter=192 expected=192 participants=3 errors=0`,
  `mixed-cache-mmu=false`,
  `classification=pi5-smp-lock-cache-coherence-complete`, and `PASS`.
- The Phase 6.2 SMP-safe primitives closeout checkpoint is accepted. Milestone
  6.2 now has reconciled source inventory, generic `SpinLock<T>` and barrier
  implementation, QEMU SMP lock contention evidence, serialized Pi 5 physical
  cache/coherence proof, and proof-scaffolding cleanup. Scheduler migration,
  shared run queues, cross-core wakeups, IPIs, userspace, descriptors,
  filesystem, networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and
  DMA/cache-coherent driver policy remain deferred.
- Phase 6.3 scheduler migration readiness, per-core scheduler state, and QEMU
  per-core scheduler ownership evidence are accepted. The scheduler now has a
  CPU-local ownership data boundary and QEMU substitute evidence that logical
  CPUs 0 through 3 can publish distinct local scheduler ownership snapshots
  while secondary production dispatch, shared run queues, task migration,
  cross-core wakeups, and IPIs remain deferred.
- Phase 6.3 cross-core wakeup/IPI source inventory is accepted. The selected
  path was raw SGI delivery first: a QEMU-only SGI/IPI smoke for target-list
  mapping, acknowledgement/EOI, and per-core counters before any scheduler
  wakeup implementation, followed by a serialized Pi 5 proof before SGIs are
  accepted for physical scheduler wakeups.
- Phase 6.3 raw SGI delivery is accepted on both QEMU and Pi 5. The QEMU proof
  shows SGI INTID 1 target-list delivery to logical CPUs 1, 2, and 3; the
  serialized Pi 5 proof shows the physical GIC-400 path delivering and EOI'ing
  SGI INTID 1 on logical CPUs 1, 2, and 3. These are raw interrupt-delivery
  proofs, not scheduler wakeup or remote enqueue implementations.
- Phase 6.3 remote wakeup ownership source inventory is accepted. The selected
  first model is a bounded per-target remote wake-request list: a remote sender
  may publish a bounded request and signal with SGI INTID 1, while the target
  CPU owns request consumption and any later local scheduler effect.
- The senior-review maintainability remediation checkpoint is accepted: stale
  Pi 5 probe/proof surfaces were removed, validation hygiene was restored, the
  Pi 5 boot pipeline is split into named phases, and cross-module tests now
  live in owning modules.

Blocked or pending:

- The next explicit worker task should stay within Phase 6.3 and prove the
  accepted remote wake-request model under QEMU before any Pi 5
  scheduler-facing wakeup proof. The recommended bounded slice is
  `phase6-qemu-remote-wakeup-request-smoke-20260525`. Multi-core scheduler
  migration, shared run queues, task migration, production remote wakeups,
  userspace, descriptors, filesystem, networking, SSH, shell behavior, UART
  interrupts, RP1/PCIe, and DMA/cache-coherent driver policy remain deferred
  until an explicit durable task is queued.
- The roadmap order below now prioritizes a local Unix-like OS before network
  shell access. Ethernet and SSH should reuse the local process, stdio, TTY,
  filesystem, and syscall mechanisms rather than define them.

## Roadmap Principles

- Use Rust for kernel code, with small AArch64 assembly stubs where the hardware requires it.
- Use established Rust kernel development practices where they fit: pinned nightly toolchain, explicit custom targets, no_std, build-std, small unsafe boundaries, narrow target abstractions, and QEMU-backed smoke tests for generic architecture work.
- Keep POSIX direction visible from the start: processes, file descriptors, pipes, paths, sockets, exit/wait, and exec-style program loading should shape interfaces even before compatibility is complete.
- Prefer local OS capability before remote access: serial/local TTY, stdio,
  user processes, ramfs/initramfs, VFS, libc, and a local shell come before
  Ethernet and SSH on the critical path.
- Reuse proven libraries where they shorten the path without hiding kernel
  responsibilities. smoltcp is preferred for TCP/IP evaluation over
  hand-rolling TCP; Rust uutils is preferred for core utilities once the Rust
  userspace target is viable.
- Treat self-hosting as a long-term north star, not a committed roadmap phase.
  Native compilers such as GCC, LLVM, or rustc require a mature userspace,
  filesystem, process model, libc/Rust std target, linker, storage, memory
  reclamation, and developer tooling.
- Treat Pi 5 hardware facts as evidence, not assumptions. Device tree, Linux drivers, Raspberry Pi firmware docs, Circle/RPi bare-metal examples, serial logs, and lab results should be cited in task notes.
- Keep board-specific code behind clear target boundaries. The QEMU virt target and Pi 5 target should share architecture code where possible, but not pretend to have the same devices.
- Prefer small, inspectable milestones with a boot/test gate over broad subsystem rewrites.
- Every milestone must update docs, ADRs, or task records when it changes architecture or hardware understanding.

## Phase 0: Planning, Sources, and Lab Loop

Goal: make the development system trustworthy before kernel implementation accelerates.

Milestone 0.1: Source Map

- Build a curated source index for Pi 5, BCM2712, RP1, ARMv8-A, QEMU, Linux, Raspberry Pi firmware, Circle/RPi bare-metal, and Daedalus references.
- Record which sources are authoritative and which are advisory.
- Identify missing datasheets or areas that require Linux-source archaeology.

Acceptance criteria:

- project/reference-notes.md lists primary source URLs and known gaps.
- Open hardware questions are tracked as future research tasks.

Milestone 0.2: Lab Controller Readiness

- Verify health, status, boot files, boot archive upload, power cycle, rollback, and serial endpoints.
- Keep network-controller credentials outside OpenClaw; use only the lab API.
- Establish a boot-attempt record format with archive digest, power-cycle time, serial tail, and result classification.

Acceptance criteria:

- A known-good Raspberry Pi OS boot archive can be published, power-cycled, and observed.
- Serial output is available through the API after the physical cable is installed.
- Failed boots can be rolled back without manual SD-card intervention.

Milestone 0.3: Initial ADRs

- Decide target split: QEMU virt plus physical Pi 5.
- Decide Rust toolchain and repository layout.
- Decide boot image contract and lab automation contract.
- Add an early POSIX/process shape note before scheduler task structures harden.

Acceptance criteria:

- ADRs exist for the target strategy, boot/lab loop, and Rust toolchain.
- The early POSIX/process shape note exists and is referenced before implementing scheduler task structs.

## Phase 1: Rust Kernel Skeleton and Fast Test Target

Goal: create a minimal Rust kernel that builds reproducibly and runs under QEMU virt.

Milestone 1.1: Toolchain and Image Build

- Add a custom AArch64 target JSON, Cargo config, linker script, build script, and image conversion step.
- Reuse Daedalus patterns where they still apply: build-std, alloc, panic-strategy abort, redzone disabled, explicit linker memory layout, and assembly build integration.
- Produce artifacts for both talos-aarch64-virt and talos-rpi5-bcm2712 even if the Pi 5 artifact is initially a stub.
- Pin the Rust nightly with rust-toolchain.toml and document the exact build and test commands.
- Decide target-feature policy, relocation model, inline assembly policy, compiler_builtins memory intrinsic handling, and no-unwind guarantees.

Acceptance criteria:

- cargo build produces a kernel artifact.
- The artifact layout documents load address, stack, BSS, heap reservation, and exception-vector alignment.
- Formatting and basic lint gates exist.
- Toolchain drift is detectable through CI or an explicit local check.
- Linker map or equivalent layout output can be inspected when early boot fails.

Milestone 1.2: QEMU Boot Smoke Test

- Boot on QEMU virt with a simple serial console message.
- Add a custom bare-metal test harness modeled after Daedalus, including success/failure exit through QEMU.
- Keep hardware-only behavior out of unit tests; expose it as diagnostics once real hardware exists.

Acceptance criteria:

- cargo test or an equivalent runner boots QEMU and exits with pass/fail status.
- Panic output reaches the QEMU serial console.
- Pure Rust modules can define no_std test cases.

Milestone 1.3: Early Architecture Boundaries

- Define target abstractions for boot info, UART, timer, interrupt controller, MMIO map, and device tree access.
- Keep the interfaces narrow enough to avoid overengineering before hardware facts are known.

Acceptance criteria:

- QEMU virt implements enough target operations for boot and test output. [done: QEMU test gate]
- Pi 5 target has explicit stubs or early implementations with documented unknowns. [done: build gate, pending hardware evidence]

## Phase 2: Raspberry Pi 5 First Light

Goal: boot Talos on physical Pi 5 and get reliable serial output.

Milestone 2.1: Firmware Handoff and Firmware-Preserved Serial

- Build a Pi 5 boot tree that satisfies the lab controller archive contract: config.txt, cmdline.txt, bcm2712-rpi-5-b.dtb, and kernel_2712.img or kernel8.img.
- Prefer kernel_2712.img for the Pi 5 artifact; keep kernel8.img fallback behavior documented only as firmware compatibility.
- Configure AArch64 entry, stack, BSS clearing, panic path, and serial output.
- Implement the arm64 boot ABI: x0 contains the physical DTB address, interrupts are masked, the MMU is off, and non-secure EL2 is preferred while EL1 is allowed.
- Start by using serial state preserved by firmware. Do not assume Talos owns UART clocks, GPIO muxing, or RP1 reset behavior yet.
- Check config.txt serial settings, baud rate, DTB aliases, chosen stdout-path, and whether enable_rp1_uart=1 is required for the attached cable path.

Acceptance criteria:

- The lab can publish the Talos boot archive. [local staging tool exists; publish not yet run]
- Power cycle reaches Talos code on the Pi 5.
- Serial output includes a version string, exception level, core ID, and panic path.
- A failed boot can be rolled back.

Milestone 2.2: Boot-State and UART Ownership Discovery

- Preserve and inspect the firmware-provided device tree from x0 enough to confirm memory and chosen boot arguments.
- Record actual firmware handoff state: exception level, MMU/cache state, DTB address if provided, core startup behavior, and UART clock assumptions.
- Compare observations against Linux device tree and Raspberry Pi documentation.
- Split firmware-preserved serial from Talos-owned UART initialization. The firmware console serial10 maps to BCM2712 uarta; the 40-pin header UART is RP1 UART0 and can be firmware-initialized with enable_rp1_uart=1.
- Verify serial still works after cache and MMU transitions.

Acceptance criteria:

- Architecture docs describe the actual Pi 5 handoff observed in the lab.
- UART path and ownership assumptions are documented before any UART driver is treated as stable.
- Any mismatch with assumptions becomes an ADR or tracked task.

Milestone 2.3: Exception Vectors and Panic Diagnostics

- Install AArch64 exception vectors.
- Dump ESR, FAR, ELR, SPSR, and general registers on synchronous exceptions.
- Add a deliberate exception diagnostic.

Acceptance criteria:

- A deliberate fault produces a readable serial dump.
- The dump includes enough state to debug early MMU and driver faults.

## Phase 3: Memory, MMU, and Kernel Runtime

Goal: build the foundations for safe Rust allocation, virtual memory, and later userspace.

Status: accepted for the current closeout boundary. See
[Phase 3 Closeout Checkpoint](project/phase3-closeout-checkpoint.md) for the
accepted capabilities, commit references, deferred work, and Phase 4
recommendation.

Milestone 3.1: Physical Memory Map

- Determine usable DRAM and reserved regions from device tree and firmware observations.
- Define kernel image, stack, heap, boot info, and early allocator regions.
- Avoid hardcoding a single RAM size.

Acceptance criteria:

- Boot log reports memory regions.
- Early allocator avoids kernel image, stack, DTB, and reserved firmware regions.

Milestone 3.2: Page Tables and MMU

- Implement early identity mappings for kernel memory and required MMIO.
- Map normal memory cacheable and MMIO as device memory.
- Keep translation setup compatible with SMP and future EL0 isolation.

Acceptance criteria:

- Pi 5 boots with MMU enabled.
- Serial still works after MMU enable.
- A page-fault diagnostic is available.

Milestone 3.3: Kernel Heap and Core Runtime

- Add a simple allocator first, then evolve toward a free-capable allocator when needed.
- Enable Rust alloc for Box, Vec, String, and collections.
- Keep allocation failure behavior explicit.

Acceptance criteria:

- Allocation tests pass under QEMU.
- Pi 5 diagnostic confirms heap allocation and panic-on-OOM behavior.

## Phase 4: Interrupts, Timers, and Preemption

Goal: move from cooperative boot code to timer-driven kernel scheduling.

Status: accepted for the current closeout boundary. See
[Phase 4 Closeout Checkpoint](project/phase4-closeout-checkpoint.md) for the
accepted capabilities, commit references, deferred work, risks, and Phase 5
planning recommendation.

Milestone 4.1: Interrupt Controller

- Identify the Pi 5 interrupt controller topology from device tree and Linux references. Current evidence points to GIC-400 / GICv2, with distributor and CPU interfaces in the 0x10_7fff9000 region.
- Bring up enough GIC support for architectural timer and UART interrupts.
- Keep QEMU virt and Pi 5 interrupt-controller setup target-specific.

Acceptance criteria:

- Timer interrupt fires on QEMU virt.
- Timer interrupt fires on Pi 5.
- IRQ entry/exit preserves register state.

Milestone 4.2: Monotonic Time and Preemption Tick

- Implement monotonic time based on the ARM generic timer first. The BCM2835-compatible 1 MHz system timer at 0x10_7c003000 is a secondary board timer path, not the first scheduler clock.
- Add scheduler tick accounting.
- Make interrupt masking and critical sections explicit.

Acceptance criteria:

- Serial diagnostics show periodic ticks without polling.
- Tick handling remains stable under simple workload loops.

Milestone 4.3: Kernel Threads and Scheduler

- Define the scheduler shape against the early POSIX note before committing
  structs. [done: single-core kernel-thread-first boundary]
- Implement kernel task structures and a single-core runnable queue before
  context switch, sleeping, yielding, or preemptive time slicing.
- Start with one core; keep data structures ready for SMP.
- Check task/process terminology and lifetime assumptions against the early POSIX shape note before committing scheduler structs. [done: scheduler shape note]

Acceptance criteria:

- Multiple kernel threads make progress under preemption.
- A diagnostic shows task state and context-switch counts.

## Phase 5: Local Console, TTY, and Kernel Diagnostics

Goal: make Talos locally operable over serial before adding network access.

Milestone 5.1: Console Device Model

- Split early boot logging from a runtime console device.
- Preserve the proven firmware-preserved UART path while defining the ownership
  boundary for later Talos-owned serial drivers.
- Route console reads and writes through interfaces that can become file
  descriptors and TTY devices.

Acceptance criteria:

- Kernel diagnostics can write through a runtime console abstraction.
- The early boot logger and runtime console ownership rules are documented.
- Console paths do not depend on ad hoc shell-only code.

Milestone 5.2: TTY and Stdio Shape

- Define the first TTY line discipline: raw/canonical input policy, newline
  handling, backspace, echo, and control-character behavior.
- Model stdin, stdout, and stderr as descriptor-capable streams even before
  full userspace exists.
- Keep the design compatible with later PTY/SSH sessions.

Acceptance criteria:

- A local serial TTY diagnostic can accept input and echo/process lines.
- Stdio streams can be represented by the same descriptor model planned for
  user processes.
- TTY behavior and known POSIX gaps are documented.

Milestone 5.3: Local Kernel Diagnostic Command Channel

- Add a constrained local diagnostic command channel over the serial TTY.
- Keep commands clearly kernel-owned until EL0 programs and a real shell exist.
- Prefer diagnostics that exercise scheduler, memory, filesystem, and process
  state without becoming permanent shell architecture.

Acceptance criteria:

- A user at the serial console can run bounded kernel diagnostic commands.
- Diagnostic commands are separated from the later user shell design.
- The command channel remains usable while scheduler/timer work is active.

## Phase 6: SMP and Multi-Core Scheduling

Goal: use all Pi 5 CPU cores with correct synchronization and preemptive scheduling.

Status: Milestone 6.1 is accepted through the secondary-core bring-up closeout
checkpoint. Milestone 6.2 has an accepted SMP-safe primitive source inventory,
contract, first spinlock/barrier core, QEMU SMP contention smoke, and physical
Pi 5 lock cache/coherence proof. Milestone 6.3 has accepted the first
scheduler-migration slice: CPU-local scheduler ownership, QEMU per-core
ownership evidence, and cross-core wakeup/IPI source inventory. See
[Phase 6 Secondary-Core Bring-Up Closeout Checkpoint](project/phase6-secondary-core-bringup-closeout-checkpoint.md)
and
[Phase 6 Secondary-Core Bring-Up Source Inventory](project/phase6-secondary-core-bringup-source-inventory.md),
plus
[Phase 6 SMP-Safe Primitives Source Inventory](project/phase6-smp-safe-primitives-source-inventory.md)
and
[Phase 6 Scheduler Migration Slice Checkpoint](project/phase6-scheduler-migration-slice-checkpoint.md).

Milestone 6.1: Secondary Core Bring-Up

- Observe firmware core startup behavior.
- Use PSCI as the primary secondary-core bring-up path; Raspberry Pi Linux device tree advertises PSCI 1.0 with SMC and cpu_on 0xc4000003.
- Treat spin-table or custom mailbox bring-up as fallback research, not the default plan.
- Add per-core stacks, per-core state, and CPU-local data.

Acceptance criteria:

- All four Cortex-A76 cores report alive through serial diagnostics.
- Secondary cores can run a controlled kernel-thread workload.

Milestone 6.2: SMP-Safe Primitives

- Implement spin locks, interrupt-safe locks, atomics policy, memory barriers, and per-core critical-section rules.
- Review any inherited Daedalus synchronization assumptions before reuse.

Acceptance criteria:

- Stress diagnostics run shared counters and queues across cores.
- Lock misuse and interrupt-context constraints are documented.

Milestone 6.3: Multi-Core Preemptive Scheduler

- Support per-core run queues or a global scheduler with clear tradeoffs.
- Add load balancing only after correctness is established.
- Keep task migration visible in diagnostics.

Acceptance criteria:

- Multiple CPU-bound tasks run across all cores.
- Preemption continues to work under cross-core wakeups.

## Phase 7: POSIX Contract, EL0, Syscalls, and File Descriptors

Goal: introduce Unix-like execution boundaries without attempting full POSIX yet.

Milestone 7.1: POSIX Contract Baseline

- Define the first Talos error model and errno mapping.
- Define path normalization, absolute and relative paths, root, current working directory, and namespace assumptions.
- Define initial descriptor operations: open, read, write, close, dup, pipe, and descriptor inheritance.
- Define process lifetime concepts: spawn or exec, exit status, wait, parent/child relationship, and signal deferrals.
- Define the early loader ABI and argument/environment story.

Acceptance criteria:

- A POSIX-baseline design note exists before VFS or process code grows around convenient shortcuts.
- Host-side tests cover path normalization and descriptor-table edge cases.

Milestone 7.2: EL0 Trap Path and User Address Spaces

- Split kernel and user mappings.
- Add user stacks, trap return, copy-in/copy-out helpers, and fault handling.
- Validate exception return and bad user pointers before stabilizing the syscall ABI.

Acceptance criteria:

- A simple user-mode payload runs and traps back to the kernel.
- Invalid user memory access traps without corrupting the kernel.
- Negative tests cover bad pointers and invalid trap state.

Milestone 7.3: Syscall ABI

- Add an SVC-based syscall path from lower exception level.
- Define stable error handling and numeric syscall IDs.

Acceptance criteria:

- A minimal syscall test exercises return values, invalid calls, and fault handling.

Milestone 7.4: File Descriptor Table

- Implement per-process descriptor tables.
- Model standard input, output, error, pipes, devices, and later sockets through one interface.

Acceptance criteria:

- A test process can read/write through descriptor-backed console streams.
- Descriptor lifetime and close semantics are documented.

## Phase 8: Filesystem and Program Loading

Goal: make Talos able to run more than built-in commands.

Milestone 8.1: Initramfs or Ramfs

- Add an embedded or TFTP-loaded initramfs for early files.
- Implement path lookup, file metadata, and read-only file contents.

Acceptance criteria:

- A diagnostic command or test process can list and read files from the initial filesystem.

Milestone 8.2: VFS

- Add VFS nodes for regular files, directories, devices, and pipes.
- Keep interfaces compatible with future persistent filesystems.

Acceptance criteria:

- Common file operations route through the VFS, not ad hoc shell logic.

Milestone 8.3: Program Loader

- Choose an executable format for early user programs.
- Load a program from initramfs, map it into a process, and pass arguments.

Acceptance criteria:

- A separate user program can be launched and waited on.

## Phase 9: Libc, Rust Std, and Portable Userland

Goal: make existing user programs portable to Talos instead of hand-writing a
complete command suite.

Milestone 9.1: Libc Strategy

- Define the Talos userspace ABI: startup, crt objects, errno, environment,
  arguments, TLS expectations, allocator hooks, and syscall wrappers.
- Evaluate a small libc path first: Talos-native libc shim, relibc, newlib, or
  musl when the syscall surface is mature enough.
- Treat glibc as a later compatibility target, not the first libc goal. It
  assumes a broad Linux-like environment and is too heavy for the first
  userspace porting layer.

Acceptance criteria:

- An ADR chooses the first libc strategy and records why glibc is deferred or
  rejected for the initial port.
- Simple C programs can call libc wrappers for write, read, open, close, exit,
  malloc/free, and basic path operations.
- Host-side and QEMU tests cover syscall-wrapper error behavior.

Milestone 9.2: Rust Userspace Target and Std Subset

- Define a Talos Rust userspace target distinct from the kernel target.
- Bring up enough Rust runtime support for no_std user programs first, then a
  constrained std subset when libc, allocation, filesystem, time, and descriptor
  behavior are ready.
- Keep proc-macros, build scripts, dynamic loading, and native compilation out
  of scope for this milestone.

Acceptance criteria:

- A cross-compiled Rust user program runs on Talos and uses arguments, stdio,
  heap allocation, and file reads.
- The supported and unsupported Rust std APIs are documented.
- Cargo target configuration for Talos userspace exists.

Milestone 9.3: Core Utilities Port

- Prefer Rust uutils/coreutils once the Rust userspace target is viable.
- Keep toybox, busybox, or GNU coreutils as fallback/reference ports if they
  expose missing POSIX semantics more clearly.
- Start with a small command set: cat, echo, true, false, ls, pwd, cp, mv, rm,
  mkdir, and sh-compatible process launching where practical.

Acceptance criteria:

- A cross-compiled utility suite can be packaged into initramfs/ramfs.
- Basic utilities run as separate user programs through the normal process,
  descriptor, and filesystem paths.
- Porting gaps become tracked syscall/libc/VFS tasks instead of local hacks.

## Phase 10: Local Shell and Developer UX

Goal: make Talos useful from a local console before depending on Ethernet.

Milestone 10.1: Local Shell

- Implement or port a small shell that runs as a user program.
- Use the normal process, descriptor, TTY, filesystem, and program-loader
  mechanisms.
- Support built-ins only where they reflect normal shell behavior, not kernel
  shortcuts.

Acceptance criteria:

- A user can interact through the serial TTY, run commands, inspect files, and
  launch separate user programs.
- Shell I/O uses stdin/stdout/stderr descriptors.
- Shell limitations and POSIX gaps are documented.

Milestone 10.2: Pipelines and Process Control

- Add pipes, redirection, exit status, wait, and basic job/process accounting.
- Keep signals minimal at first but avoid designs that make POSIX signals
  impossible later.

Acceptance criteria:

- The shell can run simple pipelines and report exit statuses.
- Multiple user programs can make progress while the shell remains responsive.
- Descriptor inheritance and close-on-exec behavior are tested.

Milestone 10.3: Persistent or Larger Local Storage

- Evaluate SD, USB mass storage, generated image roots, and TFTP-loaded
  initramfs expansion for a practical development filesystem.
- Add a persistent filesystem path only after VFS and block/storage ownership
  rules are clear.

Acceptance criteria:

- Talos can load a nontrivial userland image without rebuilding the kernel for
  every user program change.
- Documentation explains the chosen local storage path and remaining risks.

## Phase 11: RP1, PCIe, DMA, and Hardware Substrate

Goal: understand the Pi 5 I/O substrate before relying on RP1 devices for
networking, GPIO, storage, or broader hardware support.

Milestone 11.1: RP1 and PCIe Mapping

- Determine whether firmware leaves RP1 configured and usable for early
  bare-metal access.
- Map the BCM2712 PCIe2 window, RP1 BAR/peripheral ranges, and address
  translations from device tree.
- Decide how much PCIe enumeration Talos needs for built-in RP1 versus external
  PCIe devices.

Acceptance criteria:

- A hardware note documents CPU physical addresses for initial RP1 access.
- A diagnostic can read a stable RP1 register or otherwise prove RP1 mapping
  assumptions.
- Known limitations around firmware-initialized state are recorded.

Milestone 11.2: RP1 Interrupts, Clocks, and GPIO

- Trace RP1 interrupt delivery into the BCM2712/GIC path.
- Identify clock/reset dependencies needed before Talos-owned RP1 drivers.
- Add a narrow GPIO or status-LED diagnostic only after mapping and interrupt
  assumptions are understood.

Acceptance criteria:

- RP1 interrupt routing is documented with source references.
- A minimal RP1 diagnostic works or the blocker is captured with serial
  evidence.

Milestone 11.3: DMA, IOMMU, and Cache Maintenance

- Determine RP1 DMA addressability, dma-ranges, IOMMU behavior, and
  cache-coherency requirements.
- Define kernel APIs for cache clean/invalidate and DMA-safe buffers before
  Ethernet or block drivers use DMA.

Acceptance criteria:

- DMA buffer ownership and cache-maintenance rules are documented.
- A small DMA or driver-adjacent diagnostic exists before networking depends on
  DMA.

## Phase 12: Networking and SSH Development Access

Goal: reach Talos over the network and make the system usable without serial.

Milestone 12.1: RP1 Ethernet Research Spike

- Study RP1 Ethernet as exposed by Linux device tree: rp1_eth is compatible with raspberrypi,rp1-gem and cdns,macb, behind RP1 PCIe address space.
- Decide whether to implement the Cadence GEM path directly, reuse a no_std driver if viable, or stage networking through a simpler transport first.
- Capture RP1 PCIe, RP1 interrupt routing, clocks, DMA, IOMMU, PHY reset, and cache-coherency implications. RP1 is not a simple fixed MMIO block from the CPU's point of view.

Acceptance criteria:

- A design note or ADR records the chosen Ethernet path.
- Unknown hardware behavior has diagnostic tasks.

Milestone 12.2: Network Device Abstraction

- Reuse the Daedalus idea of a small NetworkDevice trait, but revise it for Talos needs.
- Keep packet parsing and protocol logic testable without hardware.

Acceptance criteria:

- Ethernet, ARP, and IP parsing tests run in QEMU or host-side unit tests.
- Driver-specific code is isolated from protocol code.

Milestone 12.3: IP Stack

- Prefer smoltcp for no_std TCP/IP evaluation rather than hand-rolling TCP
  unless a concrete Talos constraint rules it out.
- Implement packet buffers, ARP, IPv4, ICMP, UDP/TCP, and socket integration.

Acceptance criteria:

- Talos responds to ping on the lab network.
- Talos can establish a TCP connection or accept one through a test service.

Milestone 12.4: Socket Integration

- Integrate sockets with the existing descriptor table, scheduler, blocking I/O,
  poll/wakeup model, and process lifetime rules.
- Add network diagnostics as user programs where possible, not kernel-only
  command paths.

Acceptance criteria:

- User programs can open sockets through the normal syscall/libc path.
- A network diagnostic program can accept or initiate a TCP connection.
- Blocking network I/O does not stall unrelated tasks.

Milestone 12.5: Entropy, Crypto, and SSH Strategy

- Bring up a kernel entropy source suitable for SSH host keys and session crypto.
- Evaluate porting an existing SSH server before writing one. OpenSSH is the
  compatibility target, but a smaller Rust SSH server may be a better first
  user-space port if it fits Talos libc/std and crypto constraints sooner.
- Define host key provisioning, authorized key storage, authentication policy, time requirements, heap-pressure expectations, and failure modes.

Acceptance criteria:

- ADR records the SSH implementation strategy.
- Entropy and key-management diagnostics exist before accepting SSH connections.

Milestone 12.6: SSH and Shell

- Implement or port the chosen SSH server and connect it to the existing local
  shell, PTY/TTY, descriptor, process, and filesystem model.
- Use SSH as the preferred path for user-space development and testing once it
  is reliable. Kernel changes may still use TFTP and lab power control, but
  user programs should not require serial-only workflows.

Acceptance criteria:

- User can connect remotely and run a shell.
- Multiple programs or commands can make progress concurrently.
- User-space programs can be copied, launched, and tested over SSH without using
  serial as the primary interaction channel.

## Phase 13: Toward a Useful Unix-Like System

Goal: grow from a local and remote shell into a practical small OS.

Milestones:

- Process spawning and wait/exit status.
- Permissions and user model sufficient for local experimentation.
- More complete POSIX compatibility review.
- Package/update workflow for user-space programs.
- Broader utility and service ports.
- Native build tools may be explored incrementally, but self-hosting GCC, LLVM,
  or rustc remains a north-star objective outside the committed roadmap.

Acceptance criteria:

- The shell can run separate programs, pipe output, inspect files, and operate
  locally or over SSH.
- Documentation explains how each major subsystem works and what POSIX gaps remain.

## Rolling Documentation Requirements

Each milestone should update at least one of:

- roadmap status
- task record
- architecture doc
- hardware note
- ADR
- lab runbook

Source-backed findings should cite URLs or local file references. Serial logs and boot attempts should be saved when they influence design decisions.
