# Phase 7 Pi 5 Dup Syscall Proof

Task: phase7-pi5-dup-syscall-proof-20260529
Status: accepted

## Scope

This task carries the accepted QEMU/substitute talos_dup invariant to
serialized Raspberry Pi 5 hardware. The implementation adds only the focused
rpi5_dup_syscall_proof boot scenario, image and boot-tree helpers,
ProcessDescriptorStore-backed dup/write/close observations, and retained lab
evidence for the physical proof.

It does not add read syscall behavior, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, object finalization, dup2/fcntl,
RP1/PCIe work, UART interrupt ownership, DMA/cache-driver policy, or full
POSIX descriptor readiness.

## Implementation Commits

- 64e5ba4: added the focused Pi 5 dup syscall proof scenario and staging
  helpers.
- 2d8e5f9: cleaned the initialized ProcessDescriptorStore static to PoC before
  the EL2-to-EL1/EL0 proof handoff, matching the accepted close proof fix.

## Local Evidence

- fmt/lint: cargo fmt --all -- --check passed before hardware.
- unit tests: cargo -Zjson-target-spec test passed before hardware.
- QEMU/substitute: scripts/qemu-dup-syscall-smoke.sh passed before hardware.
- QEMU/substitute regressions: scripts/qemu-descriptor-write-smoke.sh and
  scripts/qemu-close-syscall-smoke.sh passed before hardware.
- image/archive inspection: scripts/rpi5-archive-review.sh passed for the
  retained local8 candidate archive.
- final validation: cargo fmt --all -- --check, cargo -Zjson-target-spec test
  (239 no_std tests), scripts/qemu-dup-syscall-smoke.sh,
  scripts/qemu-descriptor-write-smoke.sh, scripts/qemu-close-syscall-smoke.sh,
  git diff --check, and mdbook build passed after the accepted local8 proof.
  Logs are retained under
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/validation/.

## Hardware Evidence

Evidence directory:
tasks/evidence/2026-05-29-pi5-dup-syscall-proof/.

- local1-candidate: initial 64e5ba4 candidate was inconclusive. It retained
  candidate identity, archive/kernel digests, fresh serial/TFTP cursors, TFTP
  fetch evidence, and restore proof, but did not retain the required final
  classification or PASS.
- local2-known-good-control: restored the prior accepted production-timer boot
  tree and retained complete classification=pi5-production-timer-preemption-complete
  plus PASS, proving lab health for the local1/local3 triage.
- local3-unchanged-candidate-rerun: reran the unchanged 64e5ba4 candidate from
  fresh serial/TFTP cursors. It reached start, validation, current descriptor
  lookup, and pre-ERET lines, but did not produce syscall, final
  classification, or PASS lines.
- local4-store-clean-candidate: after the descriptor-store cache-clean fix in
  2d8e5f9, the candidate fetched the 114792-byte kernel and reached several
  dup/write/close observations, but stopped after a truncated
  write_duplicate_after_duplicate_close line. It is retained as inconclusive
  candidate evidence.
- local5-known-good-control-after-local4: restored the prior accepted
  production-timer boot tree and fetched the 104136-byte kernel, but retained
  only partial production-timer output without the complete final
  classification and PASS. It is inconclusive control evidence.
- local6-known-good-control-rerun: reran the restored production-timer control
  with fresh serial/TFTP cursors. Fresh TFTP evidence showed
  da591740/kernel_2712.img served at 104136 bytes, and serial reached logical
  CPU reports, but again missed the complete final classification and PASS. It
  is inconclusive control evidence.
- local7-known-good-control-rerun: reran the restored production-timer control
  with fresh serial/TFTP cursors. Fresh TFTP evidence showed the accepted
  104136-byte kernel was served, and retained serial captured complete
  classification=pi5-production-timer-preemption-complete plus PASS. This is
  the accepted known-good control for the local4/local8 triage.
- local8-unchanged-candidate-rerun: rebuilt and published the unchanged
  2d8e5f9 candidate after the accepted local7 control. Archive SHA256 was
  7f1bf15f49245d0590fba24d89ec50094ee579855a6448416aa28abdc4ae0bfd;
  kernel SHA256 was
  73a15d22c4082ceeac49bb0e5159d241038d4f39edc62a1f56e6b6c3ba1d941c;
  kernel size was 114792 bytes. Fresh TFTP evidence showed
  da591740/kernel_2712.img served at 114792 bytes. Retained serial proved
  current-owner lookup, dup(fd 1) returning fd 3, full-table -EMFILE,
  reserved-register -EINVAL, writes through source and duplicate stdout,
  close(fd 1) preserving fd 3, duplicate close behavior, closed-descriptor
  -EBADF, talos_nop success, unknown-syscall -ENOSYS, copy-probe quarantine,
  final participants=14 expected=14 errors=0
  classification=pi5-dup-syscall-proof-complete, and PASS.

## Restore Proof

The prior accepted boot tree snapshot is
pre-pi5-dup-syscall-proof-local1-20260529. Post-restore status after local8
reported tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and
kernel_2712.img size 104136 bytes.

## Accepted Evidence

- serialized Pi 5 hardware boot/output:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/proof-lines.txt.
- retained TFTP proof:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/tftp-delta-before-restore.json.
- accepted known-good control:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local7-known-good-control-rerun/proof-lines.txt.
- restore proof:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/post-snapshot-restore-status.json.

## Result

Accepted. local8 is the retained final physical Pi 5 evidence for this task.
Promote the already queued dup syscall closeout checkpoint next; do not start
read/stdin or broader descriptor work before that checkpoint reconciles the
QEMU/Pi 5 frontier and deferred surfaces.
