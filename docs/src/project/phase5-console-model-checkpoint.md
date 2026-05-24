# Phase 5 Console Model Checkpoint

Status: accepted as the Phase 5.1 console device-model closeout before Milestone 5.2 TTY/stdio design work.

## Scope

This checkpoint reconciles the accepted Phase 5.1 console work:

- 34a3108 accepted the console device-model source inventory.
- bde4079 added the output-only runtime console write core.
- 21b0847 added the internal console write-result contract.
- 0580166 named the default runtime console identity as runtime-console0.
- 2925fa6 accepted the local input-source inventory and first-input recommendation.

The checkpoint changes documentation and durable task state only. It does not add kernel code, boot image changes, hardware publish, power-cycle, hardware tests, descriptor implementation, input implementation, TTY implementation, shell behavior, networking, or scheduler changes.

## Accepted Console Model

The Phase 5.1 console model is output-capable and input-planned.

Normal kernel formatted output enters through print! and println!, then routes through target::console::_print into runtime_console::write_default_console_output. The runtime console layer owns the named default console identity, runtime-console0, and returns the internal ConsoleWriteOutcome contract for complete-write accounting and backend failure reporting.

QEMU and Raspberry Pi 5 target code still own hardware backend selection. QEMU backs runtime-console0 with the PL011 UART0 at 0x0900_0000. Pi 5 backs it with the firmware-preserved BCM2712 UART10 path at 0x10_7d00_1000. Pi 5 early helper output remains separate for boot, panic, OOM, exception, DTB, and memory diagnostics that must keep narrow early-output behavior.

There is no accepted receive path yet. src/pl011.rs remains TX-only in behavior, and runtime console has no input trait, line discipline, descriptor read path, scheduler wakeup path, or shell command channel.

## Milestone 5.2 Decision

Milestone 5.2 may start with a documentation-only TTY/stdio shape task.

The first Milestone 5.2 task should be phase5-tty-stdio-shape-doc-20260524. It must define the initial raw/canonical mode expectations, newline/backspace/echo/control-character policy, and stdin/stdout/stderr descriptor shape before any UART RX, line discipline, descriptor table, syscall, userspace, or blocking I/O implementation begins.

Implementation is not authorized by this checkpoint. The first input implementation remains a later QEMU-only polling PL011 RX diagnostic, and it should only start after the TTY/stdio shape is accepted or the supervisor explicitly queues it as a prerequisite.

## Deferred Work And Risks

Deferred work remains explicit:

- UART RX implementation and polling input diagnostics.
- UART interrupts and input buffering.
- TTY line discipline, canonical mode, echo, signals, PTYs, and terminal window state.
- Descriptor tables, descriptor lifetime, syscall ABI, userspace, and POSIX errno mapping.
- Scheduler blocking I/O, sleep/wakeup queues, and read/write wakeup integration.
- Filesystems, program loading, local shell, networking, SSH, SMP, DMA/RP1/PCIe ownership, and lower-EL process state.
- Pi 5 input proof, which must serialize with hardwareTestLock and capture archive digest, TFTP evidence, serial injected bytes, observed RX result or timeout classification, and restore state.

The main residual risk is that output-side UART evidence can be mistaken for input readiness. Future input tasks must keep QEMU polling RX and Pi 5 hardware RX evidence separate.

## Validation

- static inspection: accepted Phase 5.1 docs and task records were reconciled against docs/src/architecture/console.md.
- fmt/lint/typecheck: git status --short was clean before edits.
- fmt/lint/typecheck: git diff --check passed for documentation changes.
- static inspection: mdbook was unavailable in the container, so mdbook build was not run.
- Rust gates were not required because this checkpoint changed only Markdown documentation and durable task state.
