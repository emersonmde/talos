# Phase 7 QEMU EL0 Trap Smoke Core

Status: accepted candidate evidence captured for
phase7-qemu-el0-trap-smoke-core-20260528.

## Scope

This task implements the accepted QEMU-only built-in EL0 payload trap smoke.
It does not add a Pi 5 hardware run, archive publishing, hardware-lock use,
general syscall ABI, syscall dispatch table, process loading, descriptor I/O,
VFS/filesystem, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Touched Files

- build.rs: registers TALOS_BOOT_SCENARIO=qemu_el0_trap_smoke.
- src/main.rs: routes the QEMU scenario to the bounded smoke.
- src/arch/aarch64/vectors.S and src/arch/aarch64/mod.rs: add the EL2 to EL1h
  to EL0t handoff trampoline.
- src/arch/aarch64/exceptions.rs: allows the QEMU scenario to classify the
  lower-AArch64 synchronous trap.
- src/target/qemu_virt.rs: owns the fixed payload bytes, temporary QEMU-only
  translation tables, EL1/EL0 register setup, user-memory validation, saved
  state reporting, and PASS/classification output.
- scripts/qemu-el0-trap-smoke.sh: builds, runs, retains, and greps the QEMU
  serial evidence.
- tasks/2026-05-28-phase7-qemu-el0-trap-smoke-core.md and
  tasks/evidence/2026-05-28-qemu-el0-trap-smoke-core/qemu-el0-trap-smoke.txt:
  task record and retained serial evidence.

## Invariant

The scenario validates and maps:

- UserText: 0x0000_0000_0010_0000..0x0000_0000_0010_1000, readable and
  executable, not writable.
- UserStack: 0x0000_0000_001f_0000..0x0000_0000_0020_0000, readable and
  writable, not executable.
- UserGuard: 0x0000_0000_001e_0000..0x0000_0000_001f_0000, rejected by the
  accepted user-memory validation primitive.

The selected payload ELR is 0x0000000000100000, which is inside UserText. The
observed trap ELR is 0x0000000000100004 because AArch64 reports ELR after the
executed SVC instruction. The selected user SP is 0x0000000000200000, the top
of UserStack; the validated writable probe covers
0x00000000001ffff0..0x0000000000200000.

The QEMU handoff uses an EL2 to EL1h trampoline, configures TTBR0_EL1 for the
fixed user mappings, then enters EL0t. The diagnostic payload executes only
SVC marker 0x7a10. The lower-AArch64 synchronous vector records the saved
state and exits through the QEMU harness. The printed esr= field follows the
accepted plan's EC/ISS value, 0x0000000054007a10. The retained log also prints
raw-esr=0x0000000056007a10 to preserve QEMU's architectural IL bit.

## QEMU Evidence

Retained serial log:
tasks/evidence/2026-05-28-qemu-el0-trap-smoke-core/qemu-el0-trap-smoke.txt

Command:

    scripts/qemu-el0-trap-smoke.sh

Required lines:

    qemu-el0-trap-smoke: trap vector=lower-aarch64-sync esr=0x0000000054007a10 far=0x0000000000000000 elr=0x0000000000100004 sp=0x0000000000200000 spsr=0x00000000000003c0 marker=0x7a10
    qemu-el0-trap-smoke: final participants=1 expected=1 errors=0 classification=qemu-el0-trap-smoke-complete
    qemu-el0-trap-smoke: PASS

The evidence level is QEMU/substitute only. No Pi 5 hardware behavior,
physical lower-EL behavior, archive publication, power-cycle, serial observe,
or hardware lock is claimed.

## Validation

- static inspection: git status --short before edits was clean.
- unit tests: cargo -Zjson-target-spec test passed.
- QEMU/substitute: scripts/qemu-el0-trap-smoke.sh passed and retained the
  serial log above.
- formatting: cargo fmt --all -- --check passed.
- static inspection: git diff --check passed.
- documentation: docs/src was not touched, so mdbook build was not required by
  this task's conditional gate.
- Pending final gate before acceptance: commit hash recording.
