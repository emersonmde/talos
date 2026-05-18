# Testing Strategy

Talos uses layered testing because no single environment can validate the whole system.

## Test Environments

### Host Tests

Use host-side tests for pure Rust logic that does not depend on privileged CPU state or MMIO:

- parsers
- packet formats
- path handling
- descriptor-table logic
- scheduler data-structure invariants
- filesystem structures
- errno/error mapping
- packet parsing and serialization

These tests should be fast and deterministic.

### QEMU virt

Use QEMU virt for generic AArch64 kernel behavior:

- image boot sanity
- serial console path
- panic path
- exception vectors
- EL transitions
- MMU/page-table basics
- ARM generic timer
- scheduler and multicore logic within QEMU limits

Local evidence:

- System QEMU is 7.2.22 and supports Raspberry Pi machines only through raspi3b.
- Workspace QEMU 9.2.0 supports raspi4b but not raspi5, bcm2712, or RP1.
- Workspace QEMU supports -cpu cortex-a76, which is useful with -M virt.

### QEMU raspi4b

Use QEMU raspi4b only for limited comparison experiments. It is not a Pi 5 emulator.

QEMU's Raspberry Pi documentation lists raspi4b but its missing devices include the Raspberry Pi 4 PCIe root port and GENET Ethernet controller. It does not list Raspberry Pi 5, BCM2712, or RP1 support.

### Physical Pi 5 Lab

Use the physical lab for every Pi 5 claim:

- firmware direct-loading of kernel_2712.img
- arm64 boot ABI handoff through x0 DTB pointer
- firmware and EEPROM boot behavior
- TFTP boot archive behavior
- BCM2712 MMIO and interrupts
- RP1 peripherals
- PSCI secondary-core bring-up
- PCIe/RP1 bridge behavior
- DMA, IOMMU, and cache-coherency behavior
- multicore startup realism
- network hardware
- power-cycle recovery

The physical lab is the source of truth for the talos-rpi5-bcm2712 target.

## Test Policy

- Do not write ignored tests for hardware that QEMU cannot run. Prefer pure tests plus physical diagnostics.
- Every hardware milestone needs a diagnostic command or serial-observable result.
- Every boot attempt that changes direction should record the archive digest, power-cycle time, serial result, and classification.
- Only one physical Pi 5 test may run at a time. The lab board is a shared serial hardware resource, so hardware runs must use a durable test lock or queue.
- Code must pass review before it is sent to the physical Pi 5. Hardware time should be spent on plausible candidates, not unreviewed work.
- Hardware results must be reviewed after the run. Serial logs, boot classification, and lab-controller metadata are part of the task evidence, and the implementation may need another coding iteration before acceptance.
- A task is accepted only when its stated acceptance criteria pass at the required validation level and the evidence is recorded.
- Treat flaky timing tests as bugs in the test design until proven otherwise.
- Keep QEMU tests deterministic where possible.
- Add property-style or fuzz-style host tests for path normalization, packet parsing, and descriptor-table lifetime rules once those modules exist.
- Add negative QEMU tests for bad syscalls, deliberate faults, allocator exhaustion, interrupt masking, and bad user pointers.
- Keep persistent lab boot-attempt artifacts for regressions: boot archive digest, timestamps, serial tail, and classification.

## Hardware Test Flow

Physical Pi 5 testing follows a controlled acceptance loop:

1. Define acceptance criteria for the task before requesting hardware time.
2. Complete implementation and local validation at the smallest meaningful non-hardware level.
3. Run a focused code review and resolve blocking findings.
4. Acquire the hardware test lock, publish the boot artifact, power-cycle the board, and capture serial/lab-controller evidence.
5. Release the hardware test lock even if the test fails.
6. Review the hardware evidence against the acceptance criteria.
7. Iterate if the evidence shows a code, assumptions, or test-design issue.
8. Accept the task only after the post-hardware review finds the criteria satisfied at the required validation level.

Failed hardware tests are useful evidence, not automatic blockers. Record the failure mode, classify it, and decide whether the next step is another implementation iteration, a better diagnostic, a lab-controller fix, or a hold for Matthew input.

## Validation Gates

Minimal gates by phase:

- Toolchain: build artifact exists, linker layout inspected.
- QEMU boot: serial hello, panic path, pass/fail exit.
- Pi 5 first light: lab publish, power cycle, serial version line.
- Boot ABI: serial report confirms DTB pointer, exception level, MMU/cache state, and chosen UART path.
- MMU: serial works after MMU enable, fault diagnostic works.
- Interrupts: timer IRQ observed and counted.
- Scheduler: multiple tasks make progress under preemption.
- SMP: all four cores execute controlled work.
- RP1 substrate: stable RP1 register access, interrupt routing understood, DMA/cache rules documented.
- Userspace: invalid user memory traps cleanly.
- POSIX baseline: path, errno, descriptor, exec/wait/exit semantics have tests or design notes before VFS expansion.
- Networking: ping or equivalent packet-level diagnostic, then TCP connection.
- Remote shell: non-SSH TCP shell works before SSH.
- SSH prerequisites: entropy, key provisioning, crypto strategy, and auth policy are validated.
- SSH: remote login reaches shell.

## Daedalus Lessons

Daedalus provides useful patterns:

- custom target JSON and Cargo build-std
- explicit linker script
- assembly build integration
- QEMU runner and pass/fail exit
- custom no_std test harness
- testing policy: pure logic in tests, real hardware through diagnostics
- small network-device abstraction

Talos should not blindly copy:

- incomplete Daedalus HTTP client code
- stale networking docs
- global-state-heavy tests
- exact network queue constants without review
- Pi 4-specific MMIO, GIC, GENET, or boot assumptions
