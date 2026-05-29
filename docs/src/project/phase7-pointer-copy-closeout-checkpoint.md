# Phase 7 Pointer-Copy Closeout Checkpoint

Status: accepted as the documentation-only Phase 7.3 closeout for the first
QEMU/substitute pointer-copy syscall smoke. This checkpoint adds no Rust or
assembly behavior, QEMU rerun, Pi 5 hardware run, archive publication,
hardwareTestLock acquisition, descriptor I/O, runtime console or TTY
integration, process loading, VFS/filesystem behavior, path copying, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, demand paging, copy-on-write, signal/restart
semantics, or lower-EL fault-table recovery.

## Accepted Inputs

- Pointer-taking syscall contract:
  ddefb045443010a3de0dd89a046454df93f192c2.
- QEMU pointer-copy smoke plan:
  75414541efb936f467ce57e270b1701edcba9b3d.
- QEMU pointer-copy smoke core:
  10c23e00e04173fa9b8af987273b047d2dd4e2e3.
- Retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log.

The core implementation also retained scalar syscall and diagnostic EL0 trap
regression evidence through scripts/qemu-syscall-smoke.sh and
scripts/qemu-el0-trap-smoke.sh, plus target-independent unit-test coverage for
the copy helper boundary.

## Accepted Capability

Talos has accepted only this bounded capability:

1. In the qemu_pointer_copy_smoke QEMU/substitute scenario, lower-AArch64
   stable svc #0 routes through the saved-frame production syscall path.
2. x8 = 0x7001 dispatches to the proof-only talos_copy_probe syscall only in
   that scenario; outside the scenario it remains an unknown syscall and
   returns -ENOSYS.
3. The proof syscall obtains x0 through x5 from the saved lower-EL frame,
   invokes the accepted copy_from_user and copy_to_user helpers against the
   explicit substitute UserData mapping and backing storage, returns x0 = 16
   for the 16-byte 0x2a-to-0xa5 success case, and returns -EFAULT for the
   guard-range case.
4. The smoke preserves the unknown-syscall -ENOSYS regression and keeps
   diagnostic marker 0x7a10 outside syscall dispatch.

The retained log reports:

    qemu-pointer-copy-smoke: final participants=3 expected=3 errors=0 classification=qemu-pointer-copy-smoke-complete
    qemu-pointer-copy-smoke: PASS

This evidence level is QEMU/substitute only. It does not prove Pi 5 physical
pointer-copy behavior, descriptor-backed I/O, process-owned address spaces,
filesystem-backed data, path copying, program loading, shell behavior,
networking, or SSH support.

## Deferred Surfaces

The following surfaces remain blocked until later explicit tasks accept their
contracts and gates:

- Pi 5 pointer-copy hardware proof, boot archive publishing, power-cycle,
  serial observation, and hardwareTestLock acquisition.
- Descriptor read/write/close/dup syscalls, runtime console or TTY-backed
  stdio, process loading, process-owned address spaces, VFS/filesystem
  behavior, path copying, argv/envp loading, shell behavior, networking, and
  SSH.
- Stable public POSIX API status for talos_copy_probe or any descriptor-backed
  copy operation.
- Demand paging, copy-on-write, shared memory, user DMA buffers, mmap,
  lower-EL fault-table recovery, signal/restart semantics, and per-thread
  errno storage.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Residual Risks

- The accepted pointer-copy evidence uses a QEMU substitute user mapping and
  in-kernel backing storage, not a process-owned address space or
  descriptor-backed buffer.
- Pi 5 behavior for the pointer-copy syscall remains unproven. The earlier Pi
  5 syscall proof accepted only scalar talos_nop and unknown-syscall routing.
- talos_copy_probe is proof-only vocabulary. A later stable descriptor or
  read/write syscall must receive its own contract, validation gates, and
  evidence.
- Lower-EL data abort recovery remains intentionally unimplemented; this
  smoke observes recoverable EFAULT through helper validation, not through a
  lower-EL fault-table path.

## Next Bounded Direction

The next bounded direction should be a supervisor-planned
phase7-pi5-pointer-copy-proof-plan-20260529 task before any Pi 5 hardware
action. That plan should translate the accepted QEMU/substitute invariant into
a serialized physical proof with hardwareTestLock ownership, candidate archive
identity, fresh TFTP and serial evidence, inconclusive-run triage, restoration
requirements, exact classification/PASS lines, and an explicit statement that
descriptor I/O, process loading, filesystem, shell, networking, and SSH remain
blocked.

If supervisor planning chooses to defer Pi 5 pointer-copy proof, the next
alternative is a descriptor syscall contract. The worker should not promote
descriptor I/O or a Pi 5 run without an explicit queued task.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added this closeout, linked it from SUMMARY,
  updated roadmap current status, updated the Phase 7 milestone summary,
  updated the decision log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker state.
