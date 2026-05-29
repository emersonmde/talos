# Phase 7 Pi 5 Pointer-Copy Proof Closeout Checkpoint

Status: accepted checkpoint for
phase7-pi5-pointer-copy-proof-closeout-checkpoint-20260529.

## Scope

This checkpoint reconciles the accepted pointer-taking syscall contract, QEMU
pointer-copy smoke plan and core, serialized Pi 5 pointer-copy proof, retained
evidence, restored hardware state, deferred surfaces, and next bounded task. It
does not add Rust or assembly behavior, rerun QEMU, publish a Pi 5 boot
archive, acquire the hardware lock, observe physical serial output, add
descriptor-backed I/O, runtime console or TTY integration, process loading,
VFS/filesystem behavior, path copying, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, demand paging,
copy-on-write, signal/restart semantics, lower-EL fault-table recovery, or a
stable public talos_copy_probe claim.

## Accepted Frontier

The accepted pointer-taking syscall contract is
phase7-pointer-taking-syscall-contract-20260529, committed at
ddefb045443010a3de0dd89a046454df93f192c2. It defines proof-only
talos_copy_probe as lower-AArch64 svc #0 with x8 = 0x7001, x0 as user pointer,
x1 as length, x2 as expected byte, x3 as replacement byte, and x4/x5 reserved
as zero. The contract keeps x8 = 0x7001 as -ENOSYS outside explicitly named
proof scenarios.

The accepted QEMU pointer-copy smoke plan is
phase7-qemu-pointer-copy-smoke-plan-20260529, committed at
75414541efb936f467ce57e270b1701edcba9b3d. It defines the QEMU/substitute
success-copy, guard-range -EFAULT, unknown-syscall -ENOSYS, and diagnostic
marker quarantine invariant.

The accepted QEMU pointer-copy smoke core is
phase7-qemu-pointer-copy-smoke-core-20260529, committed at
10c23e00e04173fa9b8af987273b047d2dd4e2e3. Retained QEMU/substitute evidence
is stored at:

~~~text
tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log
~~~

The accepted QEMU/substitute proof lines include:

~~~text
qemu-pointer-copy-smoke: final participants=3 expected=3 errors=0 classification=qemu-pointer-copy-smoke-complete
qemu-pointer-copy-smoke: PASS
~~~

The accepted Pi 5 pointer-copy proof plan is
phase7-pi5-pointer-copy-proof-plan-20260529, committed at
a5a1b9856f057a456bdcdb52eeaa523fab5c7adb. It translates the QEMU/substitute
boundary into a serialized hardware proof with candidate identity, fresh
serial/TFTP evidence, hardwareTestLock ownership, inconclusive-run triage,
restoration proof, classification, and PASS requirements.

The accepted Pi 5 pointer-copy proof is
phase7-pi5-pointer-copy-proof-20260529. The candidate implementation source
commit is f67595b892125a8d03f5190103b6af886d3c1ffd, and the
acceptance/evidence commit is af0a3590b904be6d5b95ecc884da27bb48cff718.
Retained physical evidence is stored under:

~~~text
tasks/evidence/2026-05-29-pi5-pointer-copy-proof/
~~~

The accepted Pi 5 local3 rerun proof lines are stored at:

~~~text
tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-proof-lines.txt
~~~

The key accepted physical lines are:

~~~text
rpi5-pointer-copy-proof: syscall case=copy_probe_success vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=[x0=0x0000000000110000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000010
rpi5-pointer-copy-proof: user-observed case=copy_probe_success x0=0x0000000000000010 data=0xa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5 ok=true
rpi5-pointer-copy-proof: syscall case=copy_probe_efault vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 args=[x0=0x00000000001e0000 x1=0x0000000000000010 x2=0x000000000000002a x3=0x00000000000000a5 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0xfffffffffffffff2 expected=-EFAULT
rpi5-pointer-copy-proof: user-observed case=copy_probe_efault x0=0xfffffffffffffff2 ok=true
rpi5-pointer-copy-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-pointer-copy-proof: user-observed case=unknown x0=0xffffffffffffffda ok=true
rpi5-pointer-copy-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
rpi5-pointer-copy-proof: final participants=3 expected=3 errors=0 classification=pi5-pointer-copy-proof-complete
rpi5-pointer-copy-proof: PASS
~~~

Together, these accepted tasks establish only the first physical proof-only
pointer-copy syscall boundary: a fixed built-in lower-EL payload on Pi 5 can
issue stable svc #0 with x8 = 0x7001 in the focused proof scenario, route
through the production lower-AArch64 synchronous exception path, invoke the
accepted copy_from_user and copy_to_user helpers, return x0 = 16 for the
16-byte 0x2a-to-0xa5 success case, return -EFAULT for the guard-range case,
return -ENOSYS for unknown syscall number 17, and keep diagnostic marker
0x7a10 quarantined as proof-owned completion vocabulary.

## Hardware State And Evidence

The hardware lock was owned only by
phase7-pi5-pointer-copy-proof-20260529 for the serialized Pi 5 proof:

- acquired: 2026-05-29T09:40:08Z.
- released: 2026-05-29T09:45:56Z.
- classification: pi5-pointer-copy-proof-complete.
- restored: true.

Candidate identity and fetch evidence are retained in the task record and
evidence tree:

- candidate source commit:
  f67595b892125a8d03f5190103b6af886d3c1ffd.
- candidate archive:
  target/talos-rpi5-pointer-copy-proof-boot.tar.gz.
- archive SHA256:
  195e196bb785292847da7e98f32ef4e15b08caa7d2bdd850a1240682a1c68dd9.
- candidate kernel SHA256:
  99890a3520fc1351c00250551409974cba82b802a47b40b18ac683234c1fa23b.
- candidate kernel size: 106408 bytes.
- TFTP proof:
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-tftp-delta-before-restore.json.

The first candidate run was inconclusive: TFTP served the candidate image, but
the serial observe windows contained only firmware/network-boot output. No
code changed after that run. The accepted triage recorded candidate identity,
fresh serial and TFTP cursors, a passing production-timer known-good control,
and an unchanged candidate rerun with complete physical evidence.

Restore proof is retained at:

~~~text
tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-post-restore-status.json
~~~

The pre-run and post-restore boot-tree hash was:

~~~text
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
~~~

## Preserved Boundaries

The accepted frontier is intentionally proof-scenario-only. It does not
accept:

- descriptor read/write/close/dup through syscall entry, descriptor-backed
  stdio, runtime-console or TTY descriptor routing, blocking I/O, readiness,
  pipes, sockets, device objects, or stable POSIX descriptor claims;
- stable public API status for talos_copy_probe, path copying, filesystem
  buffers, descriptor-backed copies, process-owned address spaces, partial
  copies, restart semantics, signals, resumable user faults, process-fatal
  lower-EL data-abort recovery, or per-thread errno storage;
- process loading, ELF parsing, argv/envp setup, PID allocation, exit/wait,
  credentials, sessions, controlling TTY, VFS, filesystem behavior, local
  shell, networking, or SSH;
- RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, demand paging,
  copy-on-write, shared memory, user DMA buffers, memory mapped files, or
  lower-EL fault-table recovery.

The diagnostic SVC marker 0x7a10 remains proof vocabulary. It is not a stable
syscall immediate, syscall number, ABI version, compatibility mode, copy-probe
operation, or production success path.

## Next Boundary

The next recommended bounded task is
phase7-descriptor-syscall-source-inventory-20260529. It should remain
documentation-only and inventory the accepted descriptor table, syscall
dispatch, copy helper, runtime console/TTY, process/task ownership, and
return/error surfaces before any descriptor read/write/close/dup contract or
implementation.

Descriptor syscall contracts and implementations remain blocked until that
inventory names one exact bounded descriptor syscall slice and records no
supervisor or Matthew decision blocker. The immediate next task is not
descriptor I/O implementation, process loading, filesystem behavior, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver work.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: retained QEMU pointer-copy evidence was reviewed from
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log.
- static inspection: retained Pi 5 local3 proof lines were reviewed from
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- static inspection: retained TFTP and restore evidence were reviewed from
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout checkpoint.
