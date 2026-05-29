# Phase 7 Pi 5 Descriptor-Write Proof Closeout Checkpoint

Status: accepted as the documentation-only closeout for the serialized Phase
7.3 Raspberry Pi 5 descriptor-write proof. This checkpoint adds no Rust or
assembly behavior, QEMU rerun, Pi 5 hardware rerun, boot archive publication,
hardwareTestLock acquisition, stdin/read, close, dup, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, or phase transition.

## Accepted Inputs

- Descriptor syscall source inventory:
  96dda33fbca64ed71c6d8ea76d21e4fd030463c4.
- Descriptor syscall contract:
  23429329540dfa87ebc13a5086829173400791ea.
- QEMU descriptor-write smoke plan:
  dd338a284f8c9ba47c36b0735ade498664ff439f.
- Descriptor-write core:
  e462f45ff98fe5196900c2c5ce8783a997349568.
- QEMU descriptor-write smoke core:
  26c36ffaada05e4ba598144c44f49210534b233a.
- QEMU descriptor-write closeout:
  d00b1939ed49266b107d5d130a64e6851a5f628a.
- Pi 5 descriptor-write proof plan:
  194a9de74603be601fc9b89b324efb4886e9e4fb.
- Pi 5 descriptor-write proof implementation:
  83b17d5695c3bd69ae39cd3cc1e74bf7d5fcd168.
- Pi 5 descriptor-write proof acceptance:
  f2762a9015053e6cd6cf60e54dd4d92789fddc3d.
- Retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.
- Retained Pi 5 proof evidence:
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt.

## Accepted Capability

The accepted physical capability is only this descriptor-backed stdout/stderr
write boundary on serialized Raspberry Pi 5 hardware:

1. A focused rpi5_descriptor_write_proof image enters lower AArch64 and
   reaches the production stable svc #0 routing path.
2. talos_write x8 = 1 copies 18 bytes from lower-EL UserData and writes fd 1
   stdout and fd 2 stderr through inherited stdio descriptors backed by
   runtime-console0.
3. fd 0 and fd 99 return -EBADF without additional runtime-console bytes.
4. A guard-range user pointer returns -EFAULT without additional
   runtime-console bytes.
5. Nonzero reserved x3 returns -EINVAL without additional runtime-console
   bytes.
6. talos_nop and unknown-syscall behavior remains intact.
7. x8 = 0x7001 remains quarantined outside the accepted copy-probe proof
   scenarios and returns -ENOSYS here.
8. Diagnostic marker 0x7a10 remains proof-owned vocabulary outside stable
   syscall dispatch.

The retained Pi 5 proof reports:

~~~text
rpi5-descriptor-write-proof: final participants=8 expected=8 errors=0 classification=pi5-descriptor-write-proof-complete
rpi5-descriptor-write-proof: PASS
~~~

The retained physical stdout/stderr observations are:

~~~text
rpi5-descriptor-write-proof: runtime-console case=write_stdout device=runtime-console0 bytes=18 hex=74616c6f732d7374646f75742d727069350a ok=true
rpi5-descriptor-write-proof: runtime-console case=write_stderr device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d727069350a ok=true
~~~

This accepts physical descriptor-write behavior for the proof-owned inherited
runtime-console0 stdio slice only. It does not accept stdin/read, close, dup,
pipes, sockets, process-owned descriptor tables, process-owned address spaces,
blocking/readiness, filesystem-backed data, path copying, program loading,
shell behavior, networking, SSH, full POSIX descriptor readiness, or a general
userland ABI.

## Hardware Timeline And Restore Proof

- hardwareTestLock owner/taskId:
  phase7-pi5-descriptor-write-proof-20260529.
- local1 candidate: fresh TFTP served
  da591740/kernel_2712.img at 108136 bytes, but retained serial windows were
  null/blank. The result was classified inconclusive and no code changes
  followed.
- local2 known-good control: restored accepted production-timer tree reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS from fresh serial/TFTP cursors.
- local3 unchanged candidate rerun: fresh TFTP served the same
  108136-byte descriptor-write kernel and retained serial output reached
  classification=pi5-descriptor-write-proof-complete plus
  rpi5-descriptor-write-proof: PASS.
- Lock release: recorded at 2026-05-29T12:34:55Z after local3 evidence and
  restore.
- Restore snapshot: pre-pi5-descriptor-write-proof-local1-20260529.
- Pre-run boot-tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- Post-restore boot-tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Retained restore evidence:
tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-post-restore-status.json.

## Deferred Surfaces

The following surfaces remain blocked until later explicit tasks accept their
contracts and gates:

- stdin/read, close, dup, pipe, socket, blocking/readiness, and descriptor
  lifetime semantics.
- process-owned descriptor tables, process-owned address spaces,
  argv/envp setup, program loading, PID allocation, exit/wait, credentials,
  sessions, controlling TTY, VFS/filesystem behavior, local shell, networking,
  and SSH.
- partial writes, per-thread errno storage, restart semantics, signals,
  resumable user faults, demand paging, copy-on-write, shared memory, mmap,
  and lower-EL fault-table recovery.
- RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and non-proof
  runtime-console device ownership.

## Residual Risks

- The accepted proof uses a focused built-in lower-EL payload and inherited
  runtime-console0 descriptors, not a process-owned descriptor table or loaded
  user program.
- fd 1/fd 2 write success is physical, but only for the proof-owned
  runtime-console0 backing object and 18-byte buffers.
- EFAULT is produced by explicit helper validation, not by lower-EL data abort
  recovery or a resumable user-fault table.
- The first hardware run was inconclusive; the accepted result depends on the
  recorded same-candidate triage, known-good control, and unchanged rerun.

## Next Bounded Direction

The next bounded task should be
phase7-syscall-abi-dispatch-closeout-checkpoint-20260529 before any Milestone
7.4 source inventory or broader descriptor work. That checkpoint should close
out Milestone 7.3 by reconciling accepted scalar syscall routing,
copy-in/copy-out, proof-only pointer-copy, stable descriptor-write QEMU/Pi 5
evidence, diagnostic-surface quarantine, blocked surfaces, and a precise
recommendation for Milestone 7.4.

The worker should not promote Milestone 7.4 source inventory, process-owned
descriptor tables, stdin/read, close, dup, process loading, filesystem, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver work
without an explicit queued task whose dependencies and acceptance gates are
already satisfied.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: retained Pi 5 descriptor-write proof evidence was
  reviewed from
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- static inspection: retained QEMU descriptor-write smoke evidence remains
  referenced from
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.
- static documentation diff: added this closeout, linked it from SUMMARY,
  updated roadmap current status and Phase 7.3 milestone summary, updated the
  decision log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker
  state.
