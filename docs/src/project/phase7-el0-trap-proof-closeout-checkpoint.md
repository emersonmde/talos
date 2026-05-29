# Phase 7 EL0 Trap Proof Closeout Checkpoint

Status: accepted checkpoint for
phase7-el0-trap-proof-closeout-checkpoint-20260529.

## Scope

This checkpoint reconciles the accepted QEMU EL0 trap smoke proof, the
serialized Pi 5 EL0 trap proof, retained evidence, deferred surfaces, and the
next bounded Phase 7 task. It does not add Rust or assembly behavior, rerun
QEMU, publish a Pi 5 boot archive, acquire the hardware lock, observe serial
hardware, define a syscall ABI, add process loading, descriptor I/O,
VFS/filesystem, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Accepted Proofs

The accepted QEMU implementation task is
phase7-qemu-el0-trap-smoke-core-20260528, committed at
6bb55d65f6df66235edcf2abce4014ff8fc18a2b. Retained QEMU/substitute evidence is
stored at:

~~~text
tasks/evidence/2026-05-28-qemu-el0-trap-smoke-core/qemu-el0-trap-smoke.txt
~~~

The accepted Pi 5 hardware proof task is
phase7-pi5-el0-trap-proof-20260528, committed at
8605e17dd21648eeaddf9a1c9b4aa932e8d9bf8b. Retained physical evidence is stored
under:

~~~text
tasks/evidence/2026-05-28-pi5-el0-trap-proof/local62-clean-final-lower-el0-trap/
~~~

The accepted Pi 5 proof-lines file contains the source-backed translation
feature report, descriptor shape report, EL1 handoff markers, regular
VBAR_EL1 lower-AArch64 synchronous trap, final classification, and PASS line.
The key accepted physical lines are:

~~~text
rpi5-el0-trap-proof: trap vector=lower-aarch64-sync esr=0x0000000054007a10 far=0x0000000000000000 elr=0x0000000000100004 sp=0x0000000000200000 spsr=0x00000000000003c0 marker=0x7a10
rpi5-el0-trap-proof: final participants=1 expected=1 errors=0 classification=pi5-el0-trap-proof-complete
rpi5-el0-trap-proof: PASS
~~~

The corresponding accepted QEMU/substitute lines are:

~~~text
qemu-el0-trap-smoke: trap vector=lower-aarch64-sync esr=0x0000000054007a10 far=0x0000000000000000 elr=0x0000000000100004 sp=0x0000000000200000 spsr=0x00000000000003c0 marker=0x7a10
qemu-el0-trap-smoke: final participants=1 expected=1 errors=0 classification=qemu-el0-trap-smoke-complete
qemu-el0-trap-smoke: PASS
~~~

Together, these accepted proofs establish the bounded lower-EL trap path for
the diagnostic built-in payload: Talos can validate the fixed user frame,
enter EL0t, execute diagnostic SVC marker 0x7a10, take the lower-AArch64
synchronous exception through regular vectors, preserve the saved user state,
classify completion, and report PASS on both QEMU/substitute and physical Pi 5
hardware.

## Preserved Boundaries

The accepted proof frontier is intentionally narrow. It does not accept:

- a general SVC/syscall ABI, syscall number table, errno return convention,
  restart convention, or per-thread errno storage;
- process loading, ELF parsing, argument/environment setup, process exit/wait,
  signals, or resumable user faults;
- descriptor I/O, runtime-console or TTY descriptor routing, VFS/filesystem
  behavior, pipes, sockets, local shell, networking, or SSH;
- RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, or broader
  Pi 5 device-driver behavior.

The diagnostic marker 0x7a10 remains proof vocabulary, not stable syscall
ABI behavior. Copy-in/copy-out remains limited to the already accepted
target-independent user-memory permission vocabulary until a later contract
defines the runtime interface and validation gates.

## Next Boundary

The next recommended bounded task is
phase7-syscall-abi-source-inventory-20260529. It should remain
documentation-only and map the existing source owners and gaps for SVC
exception decoding, syscall number and argument registers, return/error
convention, user-copy preconditions, descriptor-table interaction, and
process/task ownership before any syscall implementation starts.

Implementation work should wait for a separately accepted syscall ABI contract.
The immediate next task is not syscall dispatch core, process loading,
descriptor I/O, filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver work.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: retained QEMU evidence was reviewed from
  tasks/evidence/2026-05-28-qemu-el0-trap-smoke-core/qemu-el0-trap-smoke.txt.
- static inspection: retained Pi 5 local62 proof lines were reviewed from
  tasks/evidence/2026-05-28-pi5-el0-trap-proof/local62-clean-final-lower-el0-trap/proof-lines.txt.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout checkpoint.
