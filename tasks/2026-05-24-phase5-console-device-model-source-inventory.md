# Phase 5 Console Device Model Source Inventory

Task: `phase5-console-device-model-source-inventory-20260524`

## Goal

Prepare the first Phase 5 console-device-model source inventory after the Phase 4 closeout checkpoint explicitly allowed Phase 5 planning.

## Inventory Summary

- `target::console::_print`, `print!`, and `println!` are the current kernel-facing formatting surface.
- QEMU writes through a polling PL011 backend at `0x0900_0000` after `qemu_virt::init()` initializes the UART.
- Pi 5 writes through the firmware-preserved UART10 path at `0x10_7d00_1000` with TX-ready polling, 32-bit data-register writes, and posted-write flushing.
- Static/hex/decimal helper writes remain important for panic/OOM, exception, DTB, memory, allocator, and other early reports.
- Phase 4 timer and scheduler boot images are diagnostic validation surfaces, not runtime console interfaces.

## Ownership Boundary

Early logging remains polling-only and target-owned for boot and crash visibility. Runtime console ownership begins with a named output write core that can back normal kernel diagnostics while preserving the public `print!` / `println!` surface.

The runtime console must leave room for future descriptor-backed `stdout` / `stderr`, but it must not implement descriptor tables, stdin, TTY line discipline, userspace, shell behavior, filesystems, networking, SSH, or blocking I/O in this slice.

## Next Task

`phase5-runtime-console-write-core-20260524` is the next bounded Phase 5.1 implementation task. It may add a runtime console write facade backed by the existing polling PL011 paths and route kernel printing through it if the accepted early serial contracts are preserved.

## Validation

- fmt/lint/typecheck: `git diff --check` passed.
- fmt/lint/typecheck: `git diff --cached --check` passed.
- static inspection: `mdbook` was unavailable in the container, so the mdBook build was not run.
- No Rust tests, QEMU smoke, Pi 5 image build, archive review, or hardware run was required because this was a docs, task-record, and state inventory only.
