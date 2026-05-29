# Phase 7 Pi 5 Close Syscall Proof Closeout Checkpoint

Task: phase7-pi5-close-syscall-proof-closeout-checkpoint-20260529
Status: accepted

## Scope

This documentation-only checkpoint reconciles the accepted close syscall
contract/core, QEMU close smoke, serialized Pi 5 close syscall proof,
hardware-lock timeline, restore proof, deferred surfaces, and next bounded
Milestone 7.4 task.

It does not add Rust or assembly behavior, rerun QEMU, rerun Pi 5 hardware,
publish a boot archive, acquire hardwareTestLock, implement dup/read, add
process loading, add VFS/filesystem behavior, add shell behavior, add
networking or SSH, change object finalization, or change RP1/PCIe,
UART-interrupt, or DMA/cache-driver policy.

## Closeout Document

- docs/src/project/phase7-pi5-close-syscall-proof-closeout-checkpoint.md.

## Evidence Reviewed

- QEMU close syscall smoke:
  tasks/evidence/2026-05-29-qemu-close-syscall-smoke-core/qemu-close-syscall-smoke.log.
- Pi 5 close syscall proof:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/proof-lines.txt.
- Candidate identity:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/source-identity.txt.
- Archive and kernel digests:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/digests.txt.
- Archive inspection:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/archive-review.txt.
- TFTP and restore proof:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/tftp-delta-before-restore.json
  and
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/post-snapshot-restore-status.json.

## Accepted Capability

The checkpoint accepts only the physical talos_close proof for the focused
rpi5_close_syscall_proof scenario: close fd 1/fd 2, write-after-close -EBADF,
reserved-argument -EINVAL no-mutation, repeated/invalid close -EBADF,
talos_nop, unknown-syscall -ENOSYS, copy-probe quarantine, diagnostic-marker
quarantine, final classification=pi5-close-syscall-proof-complete, and PASS.

Dup/read, process loading, VFS/filesystem, stdin/read object policy, shell,
networking, SSH, object finalization, broader cache/DMA policy, and full POSIX
descriptor readiness remain blocked.

## Next Action

Promote phase7-dup-syscall-contract-20260529 on the next worker wake if
hardwareTestLock remains unlocked/restored and no supervisor intervention is
active.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added the closeout document and task record,
  linked the closeout from SUMMARY, and updated roadmap and decisions.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
this checkpoint changes only Markdown documentation and durable worker state.
