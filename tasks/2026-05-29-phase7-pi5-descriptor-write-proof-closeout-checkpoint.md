# Phase 7 Pi 5 Descriptor-Write Proof Closeout Checkpoint

Task: phase7-pi5-descriptor-write-proof-closeout-checkpoint-20260529
Status: accepted
Date: 2026-05-29

## Scope

This documentation-only checkpoint reconciles the accepted descriptor-write
contract, QEMU/substitute evidence, serialized Pi 5 physical proof,
hardware-lock timeline, restoration proof, residual risks, and deferred
surfaces before any Milestone 7.3 closeout or Milestone 7.4 source inventory.

It did not add Rust or assembly behavior, rerun QEMU, rerun Pi 5 hardware,
publish a boot archive, acquire hardwareTestLock, add stdin/read, close, dup,
process loading, VFS/filesystem behavior, shell behavior, networking, SSH,
RP1/PCIe work, UART interrupt ownership, DMA/cache-driver policy, process-owned
descriptor tables, or a phase transition.

## Accepted Evidence

- static inspection: git status --short before edits was clean.
- accepted QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.
- accepted Pi 5 evidence:
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- accepted physical proof commit:
  f2762a9015053e6cd6cf60e54dd4d92789fddc3d.
- implementation commit:
  83b17d5695c3bd69ae39cd3cc1e74bf7d5fcd168.
- retained local3 kernel/archive digests:
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-digests.txt.
- retained TFTP proof:
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-tftp-delta-before-restore.json.
- retained restore proof:
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-post-restore-status.json.
- retained classification lines:
  rpi5-descriptor-write-proof: final participants=8 expected=8 errors=0
  classification=pi5-descriptor-write-proof-complete;
  rpi5-descriptor-write-proof: PASS.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Capability

The accepted capability is only the physical Pi 5 descriptor-backed
stdout/stderr write proof. The retained local3 serial evidence shows stable
lower-AArch64 svc #0 routes talos_write x8 = 1 through the production syscall
path; fd 1 and fd 2 write 18-byte UserData buffers through copy_from_user(),
inherited stdio descriptors, and runtime-console0; fd 0/fd 99 return -EBADF;
guard-range writes return -EFAULT; nonzero reserved x3 returns -EINVAL;
talos_nop and unknown-syscall regressions remain intact; x8 = 0x7001 remains
quarantined as -ENOSYS; and diagnostic marker 0x7a10 remains outside stable
syscall dispatch.

## Hardware Lock And Restore

hardwareTestLock was acquired by
phase7-pi5-descriptor-write-proof-20260529 before local1 candidate
publication/power action and released after the local3 unchanged candidate
rerun and boot-tree restore. The first candidate was inconclusive despite a
fresh TFTP fetch; the required triage recorded candidate identity, fresh
serial/TFTP cursors, a passing production-timer known-good control, and an
unchanged candidate rerun before acceptance. The restored boot tree hash
matches the pre-run hash:

~~~text
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
~~~

## Deferred Work

stdin/read, close, dup, process loading, process-owned descriptor tables,
process-owned address spaces, VFS/filesystem behavior, path copying,
argv/envp loading, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, blocking/readiness, signals, restart
semantics, lower-EL fault-table recovery, and full POSIX descriptor claims
remain blocked until later explicit tasks.

## Next Action

phase7-syscall-abi-dispatch-closeout-checkpoint-20260529 is mechanically
unblocked for the next worker wake if durable state, the working tree, and
hardwareTestLock remain compatible. That task should stay documentation-only
and close out Milestone 7.3 before any Milestone 7.4 source inventory.
