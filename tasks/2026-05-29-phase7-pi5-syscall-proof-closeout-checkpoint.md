# Phase 7 Pi 5 Syscall Proof Closeout Checkpoint

Task: phase7-pi5-syscall-proof-closeout-checkpoint-20260529
Status: accepted

## Scope

This documentation checkpoint reconciles the accepted syscall ABI contract,
target-independent dispatch core, production trap-routing contract, QEMU
syscall smoke evidence, serialized Pi 5 syscall proof evidence, hardware-lock
timeline, restore proof, deferred surfaces, and next bounded task. It did not
add Rust or assembly behavior, rerun QEMU, run Pi 5 hardware, publish boot
archives, acquire hardwareTestLock, add descriptor I/O, byte copy-in/copy-out,
pointer-taking syscalls, process loading, VFS/filesystem behavior, shell
behavior, networking, SSH, RP1/PCIe work, UART interrupt ownership, or
DMA/cache-driver policy.

## Accepted Evidence

- syscall ABI contract commit:
  380994e6003c048c4b88497e52c327c18ca3dffd.
- target-independent syscall dispatch core commit:
  734160cee68e69c02c0aea124ba185ea7e36bdc3.
- production syscall trap-routing contract commit:
  10aa4423db70b80a134edc31dbb4c7c34a9f7554.
- QEMU syscall smoke core commit:
  3abaf63ec11830137df15f0e3947161cad11688c.
- Pi 5 syscall proof candidate implementation commit:
  9d702d7e1a9ca8f3e1ab71da5f25297a8f34410c.
- Pi 5 syscall proof acceptance/evidence commit:
  63ee22e4c1d01e772b0f530835355bf7ef3d7d80.
- retained QEMU syscall smoke log:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log.
- retained QEMU diagnostic preservation log:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-el0-trap-smoke.log.
- retained Pi 5 proof lines:
  tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- retained Pi 5 TFTP proof:
  tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-tftp-delta-before-restore.json.
- retained restore proof:
  tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-post-restore-status.json.

Accepted Pi 5 physical classification:

~~~text
rpi5-syscall-proof: final participants=2 expected=2 errors=0 classification=pi5-syscall-proof-complete
rpi5-syscall-proof: PASS
~~~

## Hardware Lock And Restore

- hardwareTestLock owner task id: phase7-pi5-syscall-proof-20260529.
- acquired: 2026-05-29T05:16:00Z.
- released: 2026-05-29T05:28:00Z.
- classification: pi5-syscall-proof-complete.
- restored: true.
- pre-run tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- post-restore tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The first candidate run was inconclusive, so the accepted proof task completed
the required same-candidate triage before acceptance: candidate identity, fresh
serial cursor, TFTP delta, passing known-good production-timer control, and an
unchanged candidate rerun.

## Deferred Work

Descriptor read/write/close/dup, byte copy-in/copy-out, pointer-taking
syscalls, partial copies, restart semantics, signals, resumable user faults,
per-thread errno storage, process loading, ELF parsing, argv/envp setup, PID
allocation, exit/wait, credentials, sessions, controlling TTY, VFS/filesystem
behavior, local shell, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked until later explicit tasks.

## Next Task Recommendation

Recommend phase7-copyin-copyout-helper-contract-20260529 as the next bounded
documentation-only task. It should define byte-range validation, recoverable
EFAULT mapping, null/kernel-range/unmapped/permission/wraparound failures,
partial-copy policy, and process-fatal versus recoverable fault policy before
any pointer-taking syscall or descriptor I/O implementation.

## Evidence

- static inspection: git status --short before edits was clean.
- static inspection: retained QEMU syscall and diagnostic smoke logs were
  reviewed.
- static inspection: retained Pi 5 proof lines, TFTP evidence, and restore
  evidence were reviewed.
- static documentation diff: added
  docs/src/project/phase7-pi5-syscall-proof-closeout-checkpoint.md, linked it
  from docs/src/SUMMARY.md, updated docs/src/roadmap.md, updated
  docs/src/decisions/README.md, and added this task record.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
