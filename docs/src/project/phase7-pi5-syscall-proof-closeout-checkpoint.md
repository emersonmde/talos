# Phase 7 Pi 5 Syscall Proof Closeout Checkpoint

Status: accepted checkpoint for
phase7-pi5-syscall-proof-closeout-checkpoint-20260529.

## Scope

This checkpoint reconciles the accepted syscall ABI contract, target-independent
dispatch core, production trap-routing contract, QEMU syscall smoke, serialized
Pi 5 syscall proof, retained evidence, restored hardware state, deferred
surfaces, and next bounded task. It does not add Rust or assembly behavior,
rerun QEMU, publish a Pi 5 boot archive, acquire the hardware lock, observe
physical serial output, add descriptor I/O, byte copy-in/copy-out,
pointer-taking syscalls, process loading, VFS/filesystem behavior, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

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
vocabulary and unit-tested return/error encoding.

The accepted production trap-routing contract is
phase7-syscall-trap-routing-contract-20260529, committed at
10aa4423db70b80a134edc31dbb4c7c34a9f7554. It limits production routing to
lower-AArch64 synchronous SVC with immediate 0, captures x8 and x0 through x5,
mutates only saved x0 for the dispatch result, preserves ELR/SPSR, and keeps
diagnostic marker 0x7a10 proof-only.

The accepted QEMU production syscall smoke implementation is
phase7-qemu-syscall-smoke-core-20260529, committed at
3abaf63ec11830137df15f0e3947161cad11688c. Retained QEMU/substitute evidence is
stored at:

~~~text
tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log
tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-el0-trap-smoke.log
~~~

The accepted Pi 5 syscall proof is
phase7-pi5-syscall-proof-20260529. The candidate implementation source commit
is 9d702d7e1a9ca8f3e1ab71da5f25297a8f34410c, and the acceptance/evidence
commit is 63ee22e4c1d01e772b0f530835355bf7ef3d7d80. Retained physical
evidence is stored under:

~~~text
tasks/evidence/2026-05-29-pi5-syscall-proof/
~~~

The accepted Pi 5 local3 rerun proof lines are stored at:

~~~text
tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-proof-lines.txt
~~~

The key accepted physical lines are:

~~~text
rpi5-syscall-proof: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 args=[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000000
rpi5-syscall-proof: user-observed case=talos_nop x0=0x0000000000000000 ok=true
rpi5-syscall-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-syscall-proof: user-observed case=unknown x0=0xffffffffffffffda ok=true
rpi5-syscall-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
rpi5-syscall-proof: final participants=2 expected=2 errors=0 classification=pi5-syscall-proof-complete
rpi5-syscall-proof: PASS
~~~

Together, these accepted tasks establish the first physical production syscall
routing boundary: a fixed built-in lower-EL payload on Pi 5 can issue stable
svc #0, route through the production lower-AArch64 synchronous exception path,
dispatch talos_nop and unknown syscall number 17 through the accepted
target-independent core, observe the returned x0 values in lower EL, and keep
diagnostic marker 0x7a10 quarantined as proof-owned completion vocabulary.

## Hardware State And Evidence

The hardware lock was owned only by
phase7-pi5-syscall-proof-20260529 for the serialized Pi 5 proof:

- acquired: 2026-05-29T05:16:00Z.
- released: 2026-05-29T05:28:00Z.
- classification: pi5-syscall-proof-complete.
- restored: true.

Candidate identity and fetch evidence are retained in the task record and
evidence tree:

- candidate source commit:
  9d702d7e1a9ca8f3e1ab71da5f25297a8f34410c.
- candidate archive:
  target/talos-rpi5-syscall-proof-boot.tar.gz.
- archive SHA256:
  d864125d2c99a16ce28641a3fccb0aacabdcc305030f1462ec09dcec5d61e073.
- candidate kernel SHA256:
  6c0d4c040e0d10e4bc8ec9400e4596278daed2615b55ebd2665c10fc77ac8a6f.
- candidate kernel size: 101408 bytes.
- TFTP proof:
  tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-tftp-delta-before-restore.json.

The first candidate run was inconclusive: TFTP served the candidate image, but
the serial observe windows contained only firmware/network-boot output. No
code changed after that run. The accepted triage then recorded candidate
identity, fresh serial and TFTP cursors, a passing production-timer known-good
control, and an unchanged candidate rerun with complete physical evidence.

Restore proof is retained at:

~~~text
tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-post-restore-status.json
~~~

The pre-run and post-restore boot-tree hash was:

~~~text
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
~~~

## Preserved Boundaries

The accepted frontier is intentionally scalar-only and proof-scenario-only. It
does not accept:

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
phase7-copyin-copyout-helper-contract-20260529. It should remain
documentation-only and define byte-range validation, recoverable EFAULT
mapping, null/kernel-range/unmapped/permission/wraparound failures,
partial-copy policy, and process-fatal versus recoverable fault policy before
any pointer-taking syscall or descriptor I/O implementation.

Descriptor syscall work should remain blocked until the copy helper contract
either accepts a usable helper boundary or records why descriptor work should
be planned first. The immediate next task is not descriptor I/O,
read/write/close/dup implementation, process loading, filesystem behavior,
shell, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver work.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: retained QEMU syscall smoke evidence was reviewed from
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log.
- static inspection: retained QEMU diagnostic EL0 smoke evidence was reviewed
  from
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-el0-trap-smoke.log.
- static inspection: retained Pi 5 local3 proof lines were reviewed from
  tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- static inspection: retained TFTP and restore evidence were reviewed from
  tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout checkpoint.
