# Phase 7 Descriptor-Write Closeout Checkpoint

Status: accepted as the documentation-only Phase 7.3 closeout for the first
QEMU/substitute descriptor-write syscall smoke. This checkpoint adds no Rust or
assembly behavior, QEMU rerun, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, stdin/read, close, dup, process loading,
VFS/filesystem behavior, path copying, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, demand paging,
copy-on-write, signal/restart semantics, lower-EL fault-table recovery, or
phase transition.

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
- Retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.

The smoke core also retained scalar syscall and pointer-copy regression
evidence through scripts/qemu-syscall-smoke.sh and
scripts/qemu-pointer-copy-smoke.sh, plus target-independent unit-test coverage
for the descriptor-write core.

## Accepted Capability

Talos has accepted only this bounded descriptor-write capability:

1. In the qemu_descriptor_write_smoke QEMU/substitute scenario, lower-AArch64
   stable svc #0 routes through the saved-frame production syscall path.
2. x8 = 1 dispatches to talos_write with fd/user pointer/length in x0/x1/x2
   and reserved zero x3 through x5.
3. fd 1 and fd 2 are the inherited stdout/stderr descriptors backed by
   runtime-console0; each accepted write copies 18 bytes from the substitute
   UserData mapping with copy_from_user() before producing console side
   effects.
4. fd 0 and fd 99 return -EBADF without changing the runtime-console capture.
5. The guard range returns -EFAULT without changing the runtime-console
   capture.
6. Nonzero reserved x3 returns -EINVAL without changing the runtime-console
   capture.
7. talos_nop and unknown-syscall regression behavior remains intact.
8. x8 = 0x7001 remains proof-only and returns -ENOSYS in this descriptor-write
   scenario, and diagnostic marker 0x7a10 remains outside stable syscall
   dispatch.

The retained log reports:

~~~text
qemu-descriptor-write-smoke: final participants=8 expected=8 errors=0 classification=qemu-descriptor-write-smoke-complete
qemu-descriptor-write-smoke: PASS
~~~

The retained stdout/stderr observation lines are:

~~~text
qemu-descriptor-write-smoke: runtime-console case=write_stdout device=runtime-console0 bytes=18 hex=74616c6f732d7374646f75742d71656d750a ok=true
qemu-descriptor-write-smoke: runtime-console case=write_stderr device=runtime-console0 bytes=18 hex=74616c6f732d7374646572722d71656d750a ok=true
~~~

This evidence level is QEMU/substitute only. It does not prove Pi 5 physical
descriptor-write behavior, stdin/read behavior, close/dup behavior,
process-owned descriptor tables, process-owned address spaces,
filesystem-backed data, path copying, program loading, shell behavior,
networking, or SSH support.

## Deferred Surfaces

The following surfaces remain blocked until later explicit tasks accept their
contracts and gates:

- Pi 5 descriptor-write hardware proof, boot archive publishing, power-cycle,
  serial observation, and hardwareTestLock acquisition.
- stdin/read, close, dup, blocking/readiness, pipes, sockets, device objects,
  process-owned descriptor tables, process-owned address spaces, VFS/filesystem
  behavior, path copying, argv/envp loading, program loading, shell behavior,
  networking, and SSH.
- Stable full POSIX descriptor semantics beyond the fd 1/fd 2
  runtime-console0 write slice.
- Demand paging, copy-on-write, shared memory, user DMA buffers, mmap,
  lower-EL fault-table recovery, signal/restart semantics, and per-thread
  errno storage.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Residual Risks

- The accepted write evidence uses QEMU substitute UserData storage and
  inherited stdio descriptors, not a process-owned address space or live
  process-owned descriptor table.
- Pi 5 behavior for descriptor writes remains unproven. The earlier Pi 5
  syscall and pointer-copy proofs do not establish runtime-console0
  descriptor writes on hardware.
- runtime-console0 is the only accepted backing object for talos_write. The
  TTY, device, pipe, socket, filesystem, and blocking/readiness models still
  need separate contracts.
- Lower-EL data abort recovery remains intentionally unimplemented; this smoke
  observes recoverable EFAULT through helper validation, not through a
  lower-EL fault-table path.

## Next Bounded Direction

The next bounded direction should be
phase7-pi5-descriptor-write-proof-plan-20260529 before any Pi 5
descriptor-write hardware action. That plan should translate the accepted
QEMU/substitute invariant into a serialized physical proof with
hardwareTestLock ownership, candidate archive identity, fresh TFTP and serial
evidence, inconclusive-run triage, restoration requirements, exact
runtime-console/classification/PASS lines, and an explicit statement that
stdin/read, close, dup, process loading, filesystem, shell, networking, and SSH
remain blocked.

The worker should not promote a Pi 5 descriptor-write run, broader descriptor
implementation, process loading, filesystem behavior, shell, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver work without an
explicit queued task.

## Validation

- static inspection: git status --short before edits showed a pre-existing
  docs/src/roadmap.md working-tree edit that was preserved.
- static inspection: retained QEMU descriptor-write evidence was reviewed from
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.
- static documentation diff: added this closeout, linked it from SUMMARY,
  updated roadmap current status, updated the Phase 7 milestone summary,
  updated the decision log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker state.
