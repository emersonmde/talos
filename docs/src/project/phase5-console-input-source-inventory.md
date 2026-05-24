# Phase 5 Console Input Source Inventory

Status: accepted as the Phase 5.1 local input source inventory and first-input recommendation.

## Scope

This task inspected the existing QEMU and Raspberry Pi 5 console source surfaces for the smallest future local input path. It changed documentation only. It did not add UART RX reads, UART interrupts, input buffers, descriptor tables, TTY line discipline, userspace, syscalls, filesystems, networking, SSH, shell behavior, scheduler sleep/wakeup behavior, boot archives, or hardware runs.

## Source Inventory

Current input-relevant source surfaces:

- `src/pl011.rs`: names PL011 data, flag, interrupt-mask, and interrupt-clear offsets, but only exposes TX initialization, TX-ready polling, data writes, posted-write flushing, and `core::fmt::Write`.
- `src/target/qemu_virt.rs`: owns the QEMU PL011 UART0 at `0x0900_0000`, initializes it with `console().init_early()`, and reports `UartKind::Pl011`.
- `src/target/rpi5.rs`: owns accepted output through firmware-preserved BCM2712 UART10 at `0x10_7d00_1000`, keeps RP1 UART0 MMIO and GPIO metadata, and reports `UartKind::FirmwarePreserved`.
- `src/target/mod.rs`: routes normal formatted output through `runtime_console::write_default_console_output` and keeps early helper output separate.
- `src/runtime_console.rs`: owns the named output-side `runtime-console0` facade and write-result contract, but has no input-side trait or read outcome yet.
- `docs/src/project/lab-controller.md`: documents serial read/write/observe endpoints needed for future Pi 5 input evidence.

## Recommendation

The first input implementation task should be QEMU-only polling PL011 RX. It should add the smallest receive operation needed to check RX readiness and read a byte or short injected line from QEMU's existing PL011 UART0, then report the observed input through kernel diagnostics.

That task should remain a diagnostic proof. It should not introduce TTY state, echo, canonical mode, descriptor allocation, POSIX `read`, task blocking, scheduler wakeups, UART interrupts, shell commands, userspace, filesystems, networking, or SSH.

Pi 5 input should follow only after the QEMU polling shape is accepted. The preferred first Pi 5 input candidate is UART10 because it is already the accepted `runtime-console0` output backend. RP1 UART0 should stay deferred until a task explicitly owns RP1/PCIe and pinmux risk rather than mixing that risk into the first console input proof.

## Pi 5 Hardware Evidence Requirements

Any Pi 5 input claim must be serialized with `hardwareTestLock` and record:

- candidate boot archive name, archive SHA256, kernel image SHA256, and staged TFTP tree or upload identifier;
- pre-run TFTP cursor and post-run TFTP delta showing the candidate image was fetched;
- lab-controller serial capture showing the input diagnostic started;
- lab-controller serial write or equivalent captured injected bytes;
- serial output proving the kernel observed the exact injected bytes or reporting a bounded RX-empty/timeout classification;
- restore or post-run state note if the diagnostic archive replaced a known-good boot tree.

Failed boots, missing RX bytes, or timeouts are evidence. They should be classified by TFTP fetch status, ordinary output survival, injected-byte proof, and observed RX result.

## TTY And Stdio Constraints

`stdout` and `stderr` should continue to attach later to `runtime-console0` through descriptor-owned output handles. `stdin` should attach to the input side of a console object only after an input backend exists. Until descriptor lifetime, blocking semantics, scheduler sleep/wakeup, and syscall errno mapping exist, input diagnostics must not present themselves as POSIX `read`.

Line editing, echo, canonical mode, signals, PTYs, terminal window state, and shell behavior belong to later TTY and local shell milestones.

## Evidence

- Static inspection: `Pl011` currently has no RX API; it only writes bytes and formatted strings.
- Static inspection: QEMU has the simplest accepted PL011 UART at `0x0900_0000` and can support a local polling RX diagnostic before any hardware claim.
- Static inspection: Pi 5 UART10 is the accepted output backend for `runtime-console0`; RP1 UART0 remains metadata/pin setup and carries extra RP1 ownership risk for input.
- Validation: `git status --short` was clean before edits. `git diff --check` passed after edits. `mdbook` was unavailable in the container, so `mdbook build` was not run.
- Rust gates: `cargo fmt --all -- --check` and `cargo -Zjson-target-spec test` were not required because this task changed only documentation.
