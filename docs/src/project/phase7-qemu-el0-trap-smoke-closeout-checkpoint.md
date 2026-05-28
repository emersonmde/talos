# Phase 7 QEMU EL0 Trap Smoke Closeout Checkpoint

Status: accepted candidate checkpoint for
phase7-qemu-el0-trap-smoke-closeout-checkpoint-20260528.

## Scope

This checkpoint reconciles the accepted QEMU EL0 trap smoke implementation,
retained evidence, deferred surfaces, and next lower-EL proof boundary. It does
not add Rust or assembly behavior, rerun QEMU, publish a Pi 5 boot archive,
power-cycle hardware, acquire the hardware lock, define a syscall ABI, add
descriptor I/O, VFS/filesystem, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Accepted QEMU Evidence

The accepted implementation task is
phase7-qemu-el0-trap-smoke-core-20260528, committed at
6bb55d65f6df66235edcf2abce4014ff8fc18a2b.

Retained QEMU serial evidence:

```text
tasks/evidence/2026-05-28-qemu-el0-trap-smoke-core/qemu-el0-trap-smoke.txt
```

The log contains the required saved-state line:

```text
qemu-el0-trap-smoke: trap vector=lower-aarch64-sync esr=0x0000000054007a10 far=0x0000000000000000 elr=0x0000000000100004 sp=0x0000000000200000 spsr=0x00000000000003c0 marker=0x7a10
```

It also contains the accepted final classification and PASS lines:

```text
qemu-el0-trap-smoke: final participants=1 expected=1 errors=0 classification=qemu-el0-trap-smoke-complete
qemu-el0-trap-smoke: PASS
```

The task record states that the selected payload ELR is
0x0000000000100000, inside fixed UserText
0x0000000000100000..0x0000000000101000. The observed trap ELR is
0x0000000000100004 because the SVC has executed. The selected user SP is
0x0000000000200000, the top of fixed UserStack
0x00000000001f0000..0x0000000000200000. The guard range
0x00000000001e0000..0x00000000001f0000 is rejected by the accepted
user-memory validation primitive.

The evidence level is QEMU/substitute only. It proves that the QEMU-only
scenario can construct the fixed mappings, validate the user frame, enter EL0t,
take the diagnostic lower-AArch64 SVC trap, save the user state, and report the
planned completion classification. It does not prove physical Raspberry Pi 5
lower-EL behavior.

## Preserved Boundaries

The accepted proof keeps these surfaces deferred:

- Pi 5 lower-EL proof, archive publishing, power-cycle, serial observe, and
  hardware-lock acquisition.
- General SVC/syscall ABI, syscall table, numeric errno return convention,
  restart convention, and per-thread errno storage.
- Process loading, ELF parsing, argument/environment setup, process exit/wait,
  signals, and resumable user faults.
- Copy-in/copy-out implementation beyond the accepted target-independent
  permission vocabulary.
- Descriptor I/O, VFS, filesystem behavior, stdio TTY integration, local shell,
  networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

The raw ESR line in the retained log remains useful diagnostic detail:
raw-esr=0x0000000056007a10 preserves QEMU's architectural IL bit. The accepted
esr= field follows the planned EC/ISS classifier, 0x0000000054007a10.

## Next Boundary

The evidence supports Pi 5 proof planning next. That task should be
phase7-pi5-el0-trap-proof-plan-20260528. It should stay documentation-only and
define the serialized physical proof before any hardware action. The plan must
name candidate archive identity, fresh serial cursor, TFTP delta, known-good
control requirements, candidate rerun rules after inconclusive evidence,
restoration, retained logs, hardwareTestLock ownership, and the exact
PASS/classification lines expected from the later physical run.

If future review finds the QEMU evidence path missing or internally
contradictory, the next task should be a narrower QEMU remediation instead of
Pi 5 planning. No such contradiction is present in the retained evidence above.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: retained QEMU serial evidence was reviewed from
  tasks/evidence/2026-05-28-qemu-el0-trap-smoke-core/qemu-el0-trap-smoke.txt.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- hardware: no Pi 5 hardware run, archive publication, power-cycle, serial
  observe, hardware-lock acquisition, or physical lower-EL claim was made.
