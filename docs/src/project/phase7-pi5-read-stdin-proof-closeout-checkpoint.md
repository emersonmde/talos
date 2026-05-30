# Phase 7 Pi 5 Read And Stdin Proof Closeout Checkpoint

Status: accepted checkpoint for
phase7-pi5-read-stdin-proof-closeout-checkpoint-20260530.

## Scope

This checkpoint reconciles the accepted read/stdin source inventory, contract,
target-independent core, QEMU/substitute smoke, serialized Pi 5 proof plan,
physical Pi 5 proof, retained evidence, restored hardware state, deferred
surfaces, and next bounded Milestone 7.4 task. It does not add Rust or
assembly behavior, rerun QEMU, publish a Pi 5 boot archive, acquire the
hardware lock, observe physical serial output, add runtime-console0/TTY or
hardware stdin, add process loading, VFS/filesystem behavior, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, object finalization,
dup2/fcntl, signals, wait queues, nonblocking I/O, or DMA/cache-driver policy.

## Accepted Frontier

The accepted read/stdin contract is phase7-read-stdin-contract-20260529. It
defines stable talos_read as syscall number 4, with x0 as fd, x1 as the user
destination pointer, x2 as count, x3 through x5 reserved zero, and x0 as byte
count, zero EOF, or negative errno.

The accepted target-independent read/stdin core is
phase7-read-stdin-core-20260529. It implements the FixedStdin proof buffer,
ProcessDescriptorStore-backed fd lookup, fd 0 and duplicated stdin reads,
short-read and EOF behavior, copy_to_user EFAULT ordering, reserved-register
EINVAL, descriptor EBADF cases, and scalar/write/close/dup/copy-probe
regressions in no_std tests.

The accepted QEMU/substitute read/stdin smoke is
phase7-qemu-read-stdin-smoke-core-20260529. Retained evidence is stored at:

~~~text
tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log
~~~

That evidence proves qemu_read_stdin_smoke through lower-AArch64 svc #0 with
fd 0 duplication to fd 3, fixed proof stdin bytes talos-stdin-qemu\n,
copy-out guard EFAULT without cursor mutation, reserved EINVAL, fd/error
EBADF, fd 0 read success, duplicated-stdin bounded short read, EOF,
talos_nop and unknown-syscall regressions, talos_copy_probe quarantine,
diagnostic-marker quarantine, classification=qemu-read-stdin-smoke-complete,
and qemu-read-stdin-smoke: PASS.

The accepted Pi 5 read/stdin proof plan is
phase7-pi5-read-stdin-proof-plan-20260530. It defined the serialized physical
boundary before any hardware action, including candidate identity, fresh serial
and TFTP evidence, inconclusive-run triage, restore proof, hardwareTestLock
ownership, and exact rpi5_read_stdin_proof PASS/classification requirements.

The accepted physical Pi 5 read/stdin proof is
phase7-pi5-read-stdin-proof-20260530. The implementation source commit is
fd2be8ea42ddf88dd4cff120439ab1d3df51bce1, and the acceptance/evidence commit
is c76a1518e7de6d5f989f7d0bc646df5524adcd60. Retained physical evidence is
stored under:

~~~text
tasks/evidence/2026-05-30-pi5-read-stdin-proof/
~~~

The accepted local5 proof lines are stored at:

~~~text
tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/proof-lines.txt
~~~

The key accepted physical lines are:

~~~text
rpi5-read-stdin-proof: syscall case=dup_stdin vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=3 args=[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000003 lowest-free=true source-open=true
rpi5-read-stdin-proof: syscall case=read_guard vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff2 expected=-EFAULT fixed-stdin-cursor=0 user-unchanged=true
rpi5-read-stdin-proof: syscall case=read_reserved vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xffffffffffffffea expected=-EINVAL fixed-stdin-cursor=0 user-unchanged=true
rpi5-read-stdin-proof: syscall case=read_fd1 vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff7 expected=-EBADF fixed-stdin-cursor=0 user-unchanged=true
rpi5-read-stdin-proof: syscall case=read_badfd vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0xfffffffffffffff7 expected=-EBADF fixed-stdin-cursor=0 user-unchanged=true
rpi5-read-stdin-proof: syscall case=read_stdin_first vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 args=[x0=0x0000000000000000 x1=0x0000000000110080 x2=0x0000000000000005 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x0000000000000005 fixed-stdin-cursor=5
rpi5-read-stdin-proof: user-buffer case=read_stdin_first addr=0x0000000000110080 bytes=5 hex=74616c6f73 ok=true
rpi5-read-stdin-proof: syscall case=read_stdin_duplicate_remaining vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 args=[x0=0x0000000000000003 x1=0x00000000001100a0 x2=0x0000000000000020 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] descriptor-owner=0x0000000000000001 return-x0=0x000000000000000c fixed-stdin-cursor=17 short-read=true
rpi5-read-stdin-proof: user-buffer case=read_stdin_duplicate_remaining addr=0x00000000001100a0 bytes=12 hex=2d737464696e2d727069350a ok=true
rpi5-read-stdin-proof: syscall case=read_stdin_eof vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=4 return-x0=0x0000000000000000 fixed-stdin-cursor=17 user-unchanged=true eof=true
rpi5-read-stdin-proof: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 return-x0=0x0000000000000000
rpi5-read-stdin-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-read-stdin-proof: syscall case=copy_probe_quarantine vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0x0000000000007001 return-x0=0xffffffffffffffda expected=-ENOSYS dispatched=false
rpi5-read-stdin-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
rpi5-read-stdin-proof: final participants=11 expected=11 errors=0 classification=pi5-read-stdin-proof-complete
rpi5-read-stdin-proof: PASS
~~~

Together, these accepted tasks establish the bounded physical read/stdin
frontier: a fixed built-in lower-EL payload on Raspberry Pi 5 can duplicate
inherited fd 0, read fixed proof stdin through fd 0 and the duplicate fd 3,
preserve cursor and user memory on EFAULT/EINVAL/EBADF cases, report bounded
EOF, preserve scalar syscall regressions, and keep proof-only diagnostic
surfaces quarantined.

## Hardware State And Evidence

The hardware lock was owned only by phase7-pi5-read-stdin-proof-20260530 for
the serialized Pi 5 proof:

- acquired: 2026-05-30T02:20:56Z for the first candidate run, then
  2026-05-30T02:55:05Z for corrected unchanged-candidate rerun attempts.
- released: 2026-05-30T03:03:23Z.
- classification: pi5-read-stdin-proof-complete in retained local5 serial
  evidence.
- restored: true.

Candidate identity and fetch evidence are retained in the task record and
evidence tree:

- candidate source commit:
  fd2be8ea42ddf88dd4cff120439ab1d3df51bce1.
- candidate archive:
  target/talos-rpi5-read-stdin-proof-local5-boot.tar.gz.
- archive SHA256:
  5f91281b2dcdfb1bca6fddd6dde6c3f0b39d89f4a4274a5bf91127d8ba833983.
- candidate kernel SHA256:
  1b7417340d4b0dc44e741683464900500667929c2089b4c1ea88dc050f06d014.
- candidate kernel size: 114816 bytes.
- TFTP proof:
  tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/tftp-delta-before-restore.json.

The first candidate run was inconclusive because the serial transcript did not
reach rpi5-read-stdin-proof markers. A known-good production-timer control
then passed. No source changes were made during the triage sequence. Later
inspection found local3/local6 retained TFTP evidence only proved restored
104136-byte control fetches, so the accepted evidence comes from local5: the
unchanged fd2be8e candidate TFTP fetch at 114816 bytes plus complete serial
PASS/classification output.

Restore proof is retained at:

~~~text
tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/post-snapshot-restore-status.json
~~~

The restored prior accepted boot tree reported 104136-byte kernel_2712.img and
kernel8.img entries.

## Preserved Boundaries

The accepted frontier is intentionally fixed-proof-stdin-only. It does not
accept:

- runtime-console0-backed stdin, TTY raw/canonical input, hardware UART input,
  pipes, sockets, regular files, directories, or filesystem-backed reads;
- blocking, readiness, nonblocking flags, poll/select, wait queues, signal
  restart, Ctrl-C/Ctrl-D terminal behavior, foreground process groups, or
  terminal sessions;
- process loading, ELF parsing, argv/envp setup, PID allocation, exit/wait,
  credentials, sessions, controlling TTY, VFS, filesystem behavior, local
  shell, networking, or SSH;
- descriptor inheritance across exec, close-on-exec, dup2/fcntl, open-file
  description finalizers, object teardown, or stable full POSIX descriptor
  readiness;
- per-thread errno storage, demand paging, recoverable lower-EL data-abort
  copy tables, partial user copies on EFAULT, or process-fatal user-fault
  policy;
- RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, user DMA
  buffers, memory-mapped files, or broader cache maintenance policy.

The talos_copy_probe x8 = 0x7001 path remains proof-only and is not dispatched
by rpi5_read_stdin_proof. The diagnostic SVC marker 0x7a10 remains proof
vocabulary. It is not a stable syscall immediate, syscall number, ABI version,
compatibility mode, descriptor operation, stdin source selector, or production
success path.

## Next Boundary

The next mechanically derivable task is the already queued
phase7-file-descriptor-table-closeout-checkpoint-20260530.

That task should reconcile the full Milestone 7.4 descriptor-table slice across
descriptor ownership, stdout/stderr write, close, dup, fixed-stdin read,
QEMU/substitute evidence, Pi 5 physical proofs, retained logs, residual risks,
and deferred surfaces. It should not claim a phase transition, filesystem,
program loading, shell, networking, SSH, runtime-console0/TTY/hardware stdin,
object finalization, RP1/PCIe, UART interrupt ownership, DMA/cache-driver
policy, dup2/fcntl, signals, wait queues, nonblocking I/O, or full POSIX
descriptor readiness unless a later explicit task accepts those contracts and
gates.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: retained QEMU read/stdin evidence was reviewed from
  tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log.
- static inspection: retained Pi 5 local5 proof lines were reviewed from
  tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/proof-lines.txt.
- static inspection: retained TFTP and restore evidence were reviewed from
  tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout checkpoint.
