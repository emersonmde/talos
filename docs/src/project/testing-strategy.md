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
- QEMU smoke scripts and the Cargo test runner resolve `qemu-system-aarch64`
  through `scripts/qemu-tool.sh`. Use
  `QEMU_SYSTEM_AARCH64=/path/to/qemu-system-aarch64` when the intended
  workspace QEMU binary is not on `PATH`.

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

- The feature is the primary test. Prefer direct evidence of the intended
  behavior over isolated proof scenarios. If the task is about a shell, the
  acceptance evidence should show an interactive shell command. If the task is
  about TTY stdin, the evidence should show typed serial input reaching the path
  that future programs will use.
- Do not write ignored tests for hardware that QEMU cannot run. Prefer pure tests plus physical feature validation.
- Every hardware milestone needs a feature-level serial-observable result. Use a
  diagnostic-only command only when the real feature path is blocked and the
  command answers a named blocker.
- Every boot attempt that changes direction should record the archive digest, power-cycle time, serial result, and classification.
- Keep accepted evidence reviewable through task records and summaries. Raw lab
  captures belong in Git only when compact, decisive, or retained by an explicit
  evidence policy; see [Evidence Retention Policy](evidence-retention-policy.md).
- Keep one-off diagnostic flags, proof scripts, and serial markers on a named
  lifecycle. A diagnostic is either a temporary discriminator for a currently
  blocked feature, a retained regression gate for already-working feature
  behavior, promoted into ordinary product behavior or tests, quarantined for a
  bounded follow-up, or retired after its accepted evidence is summarized; see
  [Diagnostic Surface Policy](diagnostic-surface-policy.md).
- Only one physical Pi 5 test may run at a time. The lab board is a shared serial hardware resource, so hardware runs must use a durable test lock or queue.
- Code must pass review before it is sent to the physical Pi 5. Hardware time should be spent on plausible candidates, not unreviewed work.
- Hardware results must be reviewed after the run. Serial logs, boot classification, and lab-controller metadata are part of the task evidence, and the implementation may need another coding iteration before acceptance.
- A task is accepted only when its stated acceptance criteria pass at the required validation level and the evidence is recorded. For feature tasks, acceptance cannot be based solely on a diagnostic unless the task explicitly exists to resolve a blocker discovered while implementing the feature.
- Treat flaky timing tests as bugs in the test design until proven otherwise.
- Keep QEMU tests deterministic where possible.
- Add property-style or fuzz-style host tests for path normalization, packet parsing, and descriptor-table lifetime rules once those modules exist.
- Add negative QEMU tests for bad syscalls, deliberate faults, allocator exhaustion, interrupt masking, and bad user pointers.
- Keep persistent lab boot-attempt artifacts for regressions: boot archive digest, timestamps, serial tail, and classification.

## Hardware Test Flow

Physical Pi 5 testing follows a controlled acceptance loop:

1. Define feature-level acceptance criteria for the task before requesting hardware time.
2. Complete implementation and local validation at the smallest meaningful non-hardware level.
3. Run a focused code review and resolve blocking findings.
4. Acquire the hardware test lock, publish the boot artifact, power-cycle the board, and capture serial/lab-controller evidence.
5. Release the hardware test lock even if the test fails.
6. Review the hardware evidence against the acceptance criteria.
7. Iterate if the evidence shows a code, assumptions, or test-design issue.
8. Accept the task only after the post-hardware review finds the criteria satisfied at the required validation level.

Failed hardware tests are useful evidence, not automatic blockers. Record the failure mode, classify it, and first decide whether the next step is another feature implementation iteration. Add a diagnostic only when the failure has a specific unknown that blocks feature iteration.

## Validation Gates

Minimal gates by phase:

- Toolchain: build artifact exists, linker layout inspected.
- QEMU boot: serial hello, panic path, pass/fail exit.
- Pi 5 first light: lab publish, power cycle, serial version line.
- Boot ABI: serial report confirms DTB pointer, exception level, MMU/cache state, and chosen UART path.
- MMU: the kernel continues the intended boot or user/process path after MMU
  enable; fault diagnostics are secondary evidence only when debugging a fault
  path.
- Interrupts: timer IRQ observed and counted.
- Scheduler: multiple tasks make progress under preemption.
- SMP: all four cores execute controlled work.
- RP1 substrate: stable RP1 register access, interrupt routing understood, DMA/cache rules documented.
- Userspace: invalid user memory traps cleanly.
- POSIX baseline: path, errno, descriptor, exec/wait/exit semantics are
  exercised through small programs or shell-visible behavior as soon as that is
  feasible.
- Networking: first useful packet exchange, then TCP connection; packet-level
  diagnostics are temporary unless retained as regression gates.
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
