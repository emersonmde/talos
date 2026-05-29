# Phase 7 Syscall Routing Closeout Checkpoint

Status: accepted checkpoint for
phase7-syscall-routing-closeout-checkpoint-20260529.

## Scope

This checkpoint reconciles the accepted Phase 7.3 syscall ABI contract,
target-independent dispatch core, production trap-routing contract, QEMU
syscall smoke implementation, retained evidence, deferred surfaces, and next
bounded task. It does not add Rust or assembly behavior, rerun QEMU, publish a
Pi 5 boot archive, acquire the hardware lock, observe physical serial output,
add descriptor I/O, byte copy-in/copy-out, pointer-taking syscalls, process
loading, VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

## Accepted Frontier

The accepted syscall ABI contract is
phase7-syscall-abi-contract-20260529, committed at
380994e6003c048c4b88497e52c327c18ca3dffd. It fixes the first stable syscall
shape: lower-AArch64 svc #0, syscall number in x8, scalar arguments in x0
through x5, x0 as the sole return register, talos_nop = 0, and unknown syscall
= -ENOSYS.

The accepted target-independent dispatch core is
phase7-syscall-dispatch-core-20260529, committed at
734160cee68e69c02c0aea124ba185ea7e36bdc3. It implements the pure dispatch
vocabulary and unit-tested return/error encoding without production exception
routing, QEMU, or hardware claims.

The accepted production trap-routing contract is
phase7-syscall-trap-routing-contract-20260529, committed at
10aa4423db70b80a134edc31dbb4c7c34a9f7554. It limits production routing to
lower-AArch64 synchronous SVC with immediate 0, captures x8 and x0 through x5,
mutates only saved x0 for the dispatch result, preserves ELR/SPSR, and keeps
diagnostic marker 0x7a10 proof-only.

The accepted QEMU production syscall smoke implementation is
phase7-qemu-syscall-smoke-core-20260529, committed at
3abaf63ec11830137df15f0e3947161cad11688c. Retained QEMU/substitute evidence
is stored at:

~~~text
tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log
tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-el0-trap-smoke.log
~~~

The accepted production syscall smoke log contains the required QEMU-only
classification and PASS lines:

~~~text
qemu-syscall-smoke: final participants=2 expected=2 errors=0 classification=qemu-syscall-smoke-complete
qemu-syscall-smoke: PASS
~~~

It also contains the required production syscall routing observations:

~~~text
qemu-syscall-smoke: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 args=[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000000
qemu-syscall-smoke: user-observed case=talos_nop x0=0x0000000000000000 ok=true
qemu-syscall-smoke: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
qemu-syscall-smoke: user-observed case=unknown x0=0xffffffffffffffda ok=true
qemu-syscall-smoke: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
~~~

Together, these accepted tasks establish only the first QEMU/substitute
production syscall routing path: a fixed built-in lower-EL payload can issue
stable svc #0, route through the production lower-AArch64 synchronous
exception path, dispatch talos_nop and unknown syscall number 17 through the
accepted target-independent core, observe the returned x0 values in lower EL,
and keep diagnostic marker 0x7a10 quarantined as proof-owned completion
vocabulary.

## Preserved Boundaries

The accepted frontier is intentionally QEMU-only and scalar-only. It does not
accept:

- Pi 5 production syscall hardware proof, archive publishing, power-cycle,
  serial observation, or hardware-lock acquisition;
- descriptor read/write/close/dup through syscall entry, descriptor-backed
  stdio, runtime-console or TTY descriptor routing, blocking I/O, readiness,
  pipes, sockets, or device objects;
- byte copy-in/copy-out, pointer-taking syscalls, partial copies, restart
  semantics, signals, resumable user faults, process-fatal fault policy, or
  per-thread errno storage;
- process loading, ELF parsing, argv/envp setup, PID allocation, exit/wait,
  credentials, sessions, controlling TTY, VFS, filesystem behavior, local
  shell, networking, or SSH;
- RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, demand paging,
  copy-on-write, shared memory, user DMA buffers, or memory mapped files.

The diagnostic SVC marker 0x7a10 remains proof vocabulary. It is not a stable
syscall immediate, syscall number, ABI version, compatibility mode, or
production success path.

## Next Boundary

The next recommended bounded task is
phase7-pi5-syscall-proof-plan-20260529. It should be documentation-only and
define the serialized physical proof before any hardware action. The plan
should name candidate archive identity, fresh serial cursor, TFTP delta,
known-good control, candidate rerun rules after inconclusive evidence,
restoration requirements, retained log paths, hardwareTestLock ownership, and
the exact physical PASS/classification lines expected from a later Pi 5
production syscall proof.

If the supervisor chooses to defer hardware, the next safe alternatives are a
copy-in/copy-out helper contract or a descriptor syscall contract. Those are
not mechanically objective from this checkpoint alone because they decide the
next Phase 7.3 direction rather than close out accepted evidence.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: retained QEMU syscall smoke evidence was reviewed from
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log.
- static inspection: retained QEMU diagnostic EL0 smoke evidence was reviewed
  from
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-el0-trap-smoke.log.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- hardware: no Pi 5 hardware run, archive publication, hardware-lock
  acquisition, power cycle, serial observation, or physical syscall routing
  claim was made.
