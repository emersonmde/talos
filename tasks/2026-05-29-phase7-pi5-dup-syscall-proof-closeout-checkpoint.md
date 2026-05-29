# Phase 7 Pi 5 Dup Syscall Proof Closeout Checkpoint

Task: phase7-pi5-dup-syscall-proof-closeout-checkpoint-20260529
Status: accepted

## Scope

This documentation-only checkpoint reconciles the accepted dup contract/core,
QEMU dup smoke evidence, Pi 5 dup hardware proof evidence, hardware-lock
timeline, restore proof, residual risks, and deferred surfaces. It does not
change Rust or assembly behavior, rerun QEMU, rerun Pi 5 hardware, publish a
boot archive, acquire hardwareTestLock, implement read, add process loading,
add VFS/filesystem behavior, add shell behavior, add networking or SSH, add
object finalization, add dup2/fcntl, or claim full POSIX descriptor readiness.

## Changes

- Added
  docs/src/project/phase7-pi5-dup-syscall-proof-closeout-checkpoint.md.
- Linked the checkpoint from docs/src/SUMMARY.md.
- Updated docs/src/roadmap.md with the accepted Pi 5 dup closeout frontier and
  blocked surfaces.
- Updated docs/src/decisions/README.md with the closeout decision.
- Updated durable worker state after acceptance.

## Evidence

- Closeout document:
  docs/src/project/phase7-pi5-dup-syscall-proof-closeout-checkpoint.md.
- Retained QEMU/substitute dup evidence:
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.
- Retained Pi 5 dup proof evidence:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/proof-lines.txt.
- Accepted known-good control:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local7-known-good-control-rerun/proof-lines.txt.
- Candidate identity:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/source-commit.txt
  records 2d8e5f9de177c4b4040bcbdc826f1efbf715674f.
- Archive identity:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/digests.txt
  records archive SHA256
  7f1bf15f49245d0590fba24d89ec50094ee579855a6448416aa28abdc4ae0bfd
  and kernel SHA256
  73a15d22c4082ceeac49bb0e5159d241038d4f39edc62a1f56e6b6c3ba1d941c.
- TFTP evidence:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/tftp-delta-before-restore.json
  records da591740/kernel_2712.img served at 114792 bytes.
- Restore proof:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/post-snapshot-restore-status.json
  records restored tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- Hardware lock:
  acquired at 2026-05-29T23:10:42.269Z and released at
  2026-05-29T23:23:40.798Z after restore.

## Accepted Frontier

The accepted physical frontier is talos_dup x8 = 3 on Raspberry Pi 5 for the
focused rpi5_dup_syscall_proof scenario: fd 1 duplicates to fd 3, fd 2 dup in
the full table returns -EMFILE, reserved dup arguments return -EINVAL, writes
through source and duplicate stdout descriptors reach runtime-console0,
close(fd 1) preserves fd 3, closing fd 3 clears only the duplicate,
closed-descriptor and dup(closed fd 1) cases return -EBADF, talos_nop and
unknown-syscall regressions remain intact, talos_copy_probe remains
quarantined, diagnostic marker 0x7a10 remains proof-only, and the retained
physical log reports classification=pi5-dup-syscall-proof-complete plus PASS.

## Deferred Work

Read/stdin behavior, process loading, VFS/filesystem behavior, shell,
networking, SSH, object finalization, open-file-description reference
counting, dup2/fcntl, RP1/PCIe, UART interrupt ownership, DMA/cache-driver
policy, and full POSIX descriptor readiness remain blocked.

## Recommended Next Task

The next bounded Milestone 7.4 task should be a documentation-only read/stdin
source inventory, phase7-read-stdin-source-inventory-20260529, if the
supervisor queues it explicitly. No next task is promoted by this checkpoint
because no explicit queued read/stdin task currently exists in durable state.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added the closeout doc, SUMMARY link, roadmap
  update, decision-log entry, and this task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker state.

## Result

Accepted.
