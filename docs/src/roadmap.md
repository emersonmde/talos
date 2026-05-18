# Roadmap

Talos is planned as a long-running Raspberry Pi 5 operating-system program, not as a single bring-up spike. The roadmap is organized around user-visible capabilities and validation gates. Each phase should leave the repository in a buildable, documented state.

The near-term strategy is dual-target:

- talos-aarch64-virt: a fast QEMU virt target for architecture work, tests, and CI.
- talos-rpi5-bcm2712: the physical Raspberry Pi 5 target, validated through the lab controller and serial console.

QEMU does not currently model the Raspberry Pi 5, BCM2712, or RP1. The physical Pi 5 lab is therefore the authority for board behavior. QEMU is still valuable for generic AArch64 boot, exceptions, MMU, scheduler, and pure subsystem tests.

The Pi 5 boot path should follow the normal firmware contract first. The EEPROM bootloader loads the kernel image directly, prefers kernel_2712.img, falls back to kernel8.img, and passes the physical device-tree address in x0 according to the arm64 boot ABI. Talos should implement that handoff before considering any custom boot path.

## Current Status

Planning, lab setup, and the first Rust no_std skeleton are in progress.

Completed:

- Talos project directory created separately from Daedalus.
- mdBook documentation skeleton created.
- Lab controller documented and reachable from OpenClaw at http://talos-lab-api:8080.
- TFTP boot archive publishing and PoE control API are documented.
- Minimal Rust no_std AArch64 kernel skeleton created for QEMU virt.
- Pi 5 target definition and target boundary stubs created.
- Early target service descriptors added for boot info, UART kind, timer kind,
  interrupt-controller kind, MMIO map, and device tree pointer.

Blocked or pending:

- Physical serial cable installation. Until serial is configured, Talos cannot complete autonomous boot-result classification.
- Physical Pi 5 first-light implementation is pending serial/lab feedback.
- Pi 5 service descriptors are build-checked stubs until serial hardware
  evidence confirms the real firmware handoff and UART path.

## Roadmap Principles

- Use Rust for kernel code, with small AArch64 assembly stubs where the hardware requires it.
- Use established Rust kernel development practices where they fit: pinned nightly toolchain, explicit custom targets, no_std, build-std, small unsafe boundaries, narrow target abstractions, and QEMU-backed smoke tests for generic architecture work.
- Keep POSIX direction visible from the start: processes, file descriptors, pipes, paths, sockets, exit/wait, and exec-style program loading should shape interfaces even before compatibility is complete.
- Treat Pi 5 hardware facts as evidence, not assumptions. Device tree, Linux drivers, U-Boot, Raspberry Pi firmware docs, serial logs, and lab results should be cited in task notes.
- Keep board-specific code behind clear target boundaries. The QEMU virt target and Pi 5 target should share architecture code where possible, but not pretend to have the same devices.
- Prefer small, inspectable milestones with a boot/test gate over broad subsystem rewrites.
- Every milestone must update docs, ADRs, or task records when it changes architecture or hardware understanding.

## Phase 0: Planning, Sources, and Lab Loop

Goal: make the development system trustworthy before kernel implementation accelerates.

Milestone 0.1: Source Map

- Build a curated source index for Pi 5, BCM2712, RP1, ARMv8-A, QEMU, Linux, U-Boot, and Daedalus references.
- Record which sources are authoritative and which are advisory.
- Identify missing datasheets or areas that require Linux-source archaeology.

Acceptance criteria:

- project/reference-notes.md lists primary source URLs and known gaps.
- Open hardware questions are tracked as future research tasks.

Milestone 0.2: Lab Controller Readiness

- Verify health, status, boot files, boot archive upload, power cycle, rollback, and serial endpoints.
- Keep UniFi credentials outside OpenClaw; use only the lab API.
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

- The lab can publish the Talos boot archive.
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

- Implement kernel task structures, runnable queues, context switch, sleeping, yielding, and preemptive time slicing.
- Start with one core; keep data structures ready for SMP.
- Check task/process terminology and lifetime assumptions against the early POSIX shape note before committing scheduler structs.

Acceptance criteria:

- Multiple kernel threads make progress under preemption.
- A diagnostic shows task state and context-switch counts.

## Phase 5: RP1, PCIe, DMA, and Hardware Substrate

Goal: understand the Pi 5 I/O substrate before relying on RP1 devices for networking, GPIO, storage, or shell access.

Milestone 5.1: RP1 and PCIe Mapping

- Determine whether firmware leaves RP1 configured and usable for early bare-metal access.
- Map the BCM2712 PCIe2 window, RP1 BAR/peripheral ranges, and address translations from device tree.
- Decide how much PCIe enumeration Talos needs for built-in RP1 versus external PCIe devices.

Acceptance criteria:

- A hardware note documents CPU physical addresses for initial RP1 access.
- A diagnostic can read a stable RP1 register or otherwise prove RP1 mapping assumptions.
- Known limitations around firmware-initialized state are recorded.

Milestone 5.2: RP1 Interrupts, Clocks, and GPIO

- Trace RP1 interrupt delivery into the BCM2712/GIC path.
- Identify clock/reset dependencies needed before Talos-owned RP1 drivers.
- Add a narrow GPIO or status-LED diagnostic only after mapping and interrupt assumptions are understood.

Acceptance criteria:

- RP1 interrupt routing is documented with source references.
- A minimal RP1 diagnostic works or the blocker is captured with serial evidence.

Milestone 5.3: DMA, IOMMU, and Cache Maintenance

- Determine RP1 DMA addressability, dma-ranges, IOMMU behavior, and cache-coherency requirements.
- Define kernel APIs for cache clean/invalidate and DMA-safe buffers before Ethernet or block drivers use DMA.

Acceptance criteria:

- DMA buffer ownership and cache-maintenance rules are documented.
- A small DMA or driver-adjacent diagnostic exists before networking depends on DMA.

## Phase 6: SMP and Multi-Core Scheduling

Goal: use all Pi 5 CPU cores with correct synchronization and preemptive scheduling.

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

## Phase 9: Networking and Remote Shell

Goal: reach Talos over the network and make the system usable without serial.

Milestone 9.1: RP1 Ethernet Research Spike

- Study RP1 Ethernet as exposed by Linux device tree: rp1_eth is compatible with raspberrypi,rp1-gem and cdns,macb, behind RP1 PCIe address space.
- Decide whether to implement the Cadence GEM path directly, reuse a no_std driver if viable, or stage networking through a simpler transport first.
- Capture RP1 PCIe, RP1 interrupt routing, clocks, DMA, IOMMU, PHY reset, and cache-coherency implications. RP1 is not a simple fixed MMIO block from the CPU's point of view.

Acceptance criteria:

- A design note or ADR records the chosen Ethernet path.
- Unknown hardware behavior has diagnostic tasks.

Milestone 9.2: Network Device Abstraction

- Reuse the Daedalus idea of a small NetworkDevice trait, but revise it for Talos needs.
- Keep packet parsing and protocol logic testable without hardware.

Acceptance criteria:

- Ethernet, ARP, and IP parsing tests run in QEMU or host-side unit tests.
- Driver-specific code is isolated from protocol code.

Milestone 9.3: IP Stack

- Evaluate smoltcp for no_std TCP/IP rather than hand-rolling TCP.
- Implement packet buffers, ARP, IPv4, ICMP, UDP/TCP, and socket integration.

Acceptance criteria:

- Talos responds to ping on the lab network.
- Talos can establish a TCP connection or accept one through a test service.

Milestone 9.4: Non-SSH Remote Shell Gate

- Build a constrained TCP shell or debug command channel before SSH.
- Exercise blocking socket semantics, descriptor integration, process interaction, and shell I/O without crypto complexity.
- Treat this as an intermediate gate, not the final remote-access goal.

Acceptance criteria:

- A remote TCP client can run diagnostic shell commands.
- Multiple tasks continue making progress while a shell session is active.
- The limitations versus SSH are documented.

Milestone 9.5: Entropy, Crypto, and SSH Feasibility

- Bring up a kernel entropy source suitable for SSH host keys and session crypto.
- Evaluate no_std-compatible crypto and SSH crates, or document why a custom subset is required.
- Define host key provisioning, authorized key storage, authentication policy, time requirements, heap-pressure expectations, and failure modes.

Acceptance criteria:

- ADR records the SSH implementation strategy.
- Entropy and key-management diagnostics exist before accepting SSH connections.

Milestone 9.6: SSH and Shell

- Implement the chosen SSH path and connect it to the shell and descriptor model.

Acceptance criteria:

- User can connect remotely and run a shell.
- Multiple programs or commands can make progress concurrently.

## Phase 10: Toward a Useful Unix-Like System

Goal: grow from remote shell to a practical small OS.

Milestones:

- Pipes and shell pipelines.
- Process spawning and wait/exit status.
- Persistent filesystem path, likely after evaluating SD, USB mass storage, NFS root, or generated image roots.
- Basic command-line programs.
- Permissions and user model sufficient for local experimentation.
- More complete POSIX compatibility review.

Acceptance criteria:

- The shell can run separate programs, pipe output, inspect files, and operate over SSH.
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
