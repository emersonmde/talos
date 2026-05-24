# Phase 5 Console Device Model Source Inventory

Status: accepted as the Phase 5.1 source inventory and planning boundary. The next task may implement a runtime console write core only; TTY, descriptors, input, userspace, filesystem, networking, SSH, and shell behavior remain deferred.

## Scope

This task inventories the current early logging and target console surfaces after the accepted Phase 4 closeout. It defines the boundary between early boot logging and a future runtime console device. It does not change kernel code, normal boot output, boot archives, hardware state, scheduler/timer contracts, or Phase 4 diagnostic surfaces.

## Source Inventory

Current source surfaces:

- `src/target/mod.rs`: owns the `print!` / `println!` macros, `target::console::_print`, and the early helper functions for static, hex, and decimal output.
- `src/pl011.rs`: owns the polling PL011 backend, TX-ready waits, posted-write flushing, newline translation, and byte versus word data-register writes.
- `src/target/qemu_virt.rs`: owns QEMU's PL011 backend at `0x0900_0000` and initializes it through `console().init_early()`.
- `src/target/rpi5.rs`: owns the Pi 5 UART10 backend at `0x10_7d00_1000`, preserves firmware/BL31 UART programming, and retains RP1 UART0 MMIO/pin metadata without depending on RP1 UART0 for accepted visible console output.
- `src/boot/rpi5.rs` and `src/boot/rpi5_reports.rs`: own the normal Pi 5 early boot output contract and the bounded static/formatter-backed memory, DTB, allocator, translation, cache, and diagnostic reports.
- `src/main.rs`: owns QEMU boot banners, QEMU smoke PASS output, panic/OOM output, and the no_std test harness output.
- `docs/src/architecture/early-serial.md`: records the accepted UART10 bring-up evidence, formatter pacing policy, retired probes, and current early serial risks.
- `docs/src/project/early-posix-shape.md`: records the descriptor, process, and stdio direction that the console work must leave room for.

Current behavior:

- QEMU uses a reinitialized PL011 path and normal formatted output.
- Pi 5 uses firmware-preserved UART10, polling, 32-bit data-register writes, and posted-write flushing.
- Early helper functions remain available for panic, exception, OOM, and bring-up reports that need small allocation-free output.
- Phase 4 timer/scheduler diagnostics print counters after IRQ return or from diagnostic control flow; they are validation surfaces, not runtime console interfaces.

## Early Versus Runtime Ownership

Early logging remains polling-only and target-owned for boot and crash visibility. It is allowed to exist before the allocator, scheduler, descriptor table, input stack, and runtime device model. It owns boot-progress visibility and crash-path visibility.

Runtime console ownership starts when normal kernel code can address a named console device or facade instead of directly creating a target console backend. The first runtime console should preserve the existing `print!` / `println!` surface while moving ownership toward a device model that later descriptor and TTY work can use.

The first runtime console device must be output-only. It should not imply blocking I/O, stdin, line discipline, shell commands, descriptor allocation, process-local handles, or userspace syscalls.

## Descriptor And TTY Constraints

The console boundary must remain compatible with the early POSIX shape:

- `stdout` and `stderr` should later be descriptors backed by the runtime console write operation.
- `stdin` needs a real input path and must wait for explicit keyboard/UART receive or equivalent input work.
- TTY line discipline, echo, canonical mode, PTYs, terminal signals, and local shell behavior are later phases.
- Blocking reads and writes require scheduler sleep/wakeup support first.
- Descriptor errors should eventually translate to stable syscall errors; the Phase 5.1 write core may keep simpler internal errors.

## Next Implementation Task

Queue `phase5-runtime-console-write-core-20260524` next.

Goal: add the smallest runtime console write core that can own kernel console output while preserving the current target PL011 behavior.

Allowed:

- add a runtime console module or facade for output writes;
- keep QEMU and Pi 5 backend differences behind target-owned PL011 backends;
- preserve `print!` / `println!` as the public kernel formatting surface;
- add unit tests for write routing or backend selection where practical;
- run QEMU and Pi 5 image gates to prove the refactor does not break existing output build paths.

Not allowed:

- UART interrupts, input, TTY line discipline, descriptor tables, userspace, syscalls, filesystems, networking, SSH, shell behavior, sleeping/blocking I/O, scheduler policy changes, or Phase 4 timer/preemption redesign.

## Validation

- fmt/lint/typecheck: `git diff --check` passed.
- fmt/lint/typecheck: `git diff --cached --check` passed.
- static inspection: `mdbook` was unavailable in the container, so the mdBook build was not run.
- Rust tests, QEMU smokes, Pi 5 image builds, archives, and hardware were not required because this task changed only documentation, task records, and durable state.
