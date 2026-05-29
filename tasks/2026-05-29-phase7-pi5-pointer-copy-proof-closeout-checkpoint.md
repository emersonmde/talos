# Phase 7 Pi 5 Pointer-Copy Proof Closeout Checkpoint

Task: phase7-pi5-pointer-copy-proof-closeout-checkpoint-20260529
Status: accepted

## Scope

This documentation-only task reconciles the accepted QEMU/substitute
pointer-copy evidence, serialized Pi 5 pointer-copy proof evidence,
implementation and acceptance commits, retained validation gates, proof-only
status, deferred surfaces, and next bounded descriptor-planning direction.

It did not change Rust or assembly behavior, rerun QEMU, publish a Pi 5 boot
archive, acquire hardwareTestLock, observe physical serial output, add
descriptor I/O, runtime console/TTY integration, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, demand paging, copy-on-write,
signal/restart semantics, lower-EL fault-table recovery, or a stable public
talos_copy_probe claim.

## Accepted Inputs

- Pointer-taking syscall contract:
  ddefb045443010a3de0dd89a046454df93f192c2.
- QEMU pointer-copy smoke plan:
  75414541efb936f467ce57e270b1701edcba9b3d.
- QEMU pointer-copy smoke core:
  10c23e00e04173fa9b8af987273b047d2dd4e2e3.
- QEMU pointer-copy closeout:
  a30883bc5b4458850fe369b4558c27dc97736258.
- Pi 5 pointer-copy proof plan:
  a5a1b9856f057a456bdcdb52eeaa523fab5c7adb.
- Pi 5 pointer-copy implementation commit:
  f67595b892125a8d03f5190103b6af886d3c1ffd.
- Pi 5 pointer-copy acceptance commit:
  af0a3590b904be6d5b95ecc884da27bb48cff718.

## Evidence Reviewed

- static inspection: retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log.
- serial hardware boot/output: retained Pi 5 proof lines:
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- lab-controller API: retained Pi 5 TFTP proof:
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-tftp-delta-before-restore.json.
- lab-controller API: retained Pi 5 restore proof:
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-post-restore-status.json.

## Accepted Capability

The accepted capability is only the physical proof-only pointer-copy boundary.
In the focused rpi5_pointer_copy_proof scenario, lower-AArch64 stable svc #0
with x8 = 0x7001 routes through production syscall dispatch on Pi 5, invokes
copy_from_user and copy_to_user against the fixed UserData backing storage,
returns x0 = 16 for the 16-byte 0x2a-to-0xa5 success case, returns -EFAULT for
the guard-range case, preserves unknown-syscall -ENOSYS behavior, and keeps
diagnostic marker 0x7a10 outside syscall dispatch.

The accepted physical evidence is:

~~~text
tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-proof-lines.txt
~~~

with classification=pi5-pointer-copy-proof-complete and
rpi5-pointer-copy-proof: PASS.

## Deferred Work

Descriptor read/write/close/dup syscalls, descriptor-backed stdio,
runtime-console or TTY descriptor routing, process-owned address spaces,
partial copies, restart semantics, signals, resumable user faults, lower-EL
fault-table recovery, per-thread errno storage, process loading, VFS/filesystem
behavior, path copying, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, stable POSIX descriptor claims,
and stable public talos_copy_probe status remain blocked until later explicit
tasks accept their contracts and gates.

## Next Action

phase7-descriptor-syscall-source-inventory-20260529 is mechanically unblocked
for the next worker wake if durable state and the working tree remain
compatible. That task should inventory descriptor table operations, syscall
argument extraction, copy helper use, runtime console/TTY boundaries,
return/error encoding, and ownership gaps before any descriptor syscall
contract or implementation.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added the closeout document, linked it from
  SUMMARY, updated roadmap current status and Phase 7.3 accepted progress,
  updated the decision log, and added this task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker state.
