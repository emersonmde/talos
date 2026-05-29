# Phase 7 Close Syscall Closeout Checkpoint

Task: phase7-close-syscall-closeout-checkpoint-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Close out the accepted close syscall contract, target-independent core, and
QEMU/substitute smoke evidence before any Pi 5 close proof or dup/read work.

## Scope

- Added docs/src/project/phase7-close-syscall-closeout-checkpoint.md.
- Linked the closeout from docs/src/SUMMARY.md.
- Updated docs/src/roadmap.md and docs/src/decisions/README.md with the
  accepted QEMU/substitute close syscall frontier and blocked surfaces.
- Preserved the retained QEMU evidence path:
  tasks/evidence/2026-05-29-qemu-close-syscall-smoke-core/qemu-close-syscall-smoke.log.

## Non-Goals

- No Rust or assembly behavior changes.
- No QEMU rerun, Pi 5 hardware run, boot archive publication, or
  hardwareTestLock acquisition.
- No dup/read syscall behavior, process loading, VFS/filesystem, shell,
  networking, SSH, object finalization, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, or full POSIX descriptor readiness claim.

## Accepted Evidence Matrix

| Task | Commit | Evidence level |
| --- | --- | --- |
| phase7-close-dup-read-syscall-source-inventory-20260529 | 8e17c1d0be80f860ef83bc02a01035dacd78d439 | static documentation/source inspection |
| phase7-close-syscall-contract-20260529 | 687ef5c04e745853230d61ef64845ec90ddb337c | static documentation/source inspection |
| phase7-close-syscall-core-20260529 | ab8915b9696a046b367830e9f5acfd632ee98788 | fmt/unit tests/QEMU regression gates |
| phase7-qemu-close-syscall-smoke-plan-20260529 | cfe3098d559ea21cd69d411f03e456064b265ee7 | static documentation/source inspection |
| phase7-qemu-close-syscall-smoke-core-20260529 | 3be4e1a76e1a065a846f1ebb226bc3e8554c2acf | QEMU/substitute serial evidence |

## Accepted Capability

The accepted close syscall capability is limited to stable talos_close x8 = 2
through the current ProcessOwnerId-backed ProcessDescriptorStore. The retained
QEMU/substitute smoke proves close success for fd 1 and fd 2, -EBADF for
closed/repeated/bad descriptors, -EINVAL for reserved-register violations,
no runtime-console0 side effects after closing, unaffected fd 2 behavior after
fd 1 closes, talos_nop and unknown-syscall regressions, copy-probe quarantine,
and diagnostic marker quarantine.

## Deferred Work

Pi 5 physical close proof, dup/read syscalls, process loading, VFS/filesystem,
stdin/read object model, shell, networking, SSH, open-file-description
reference counting, object finalization, blocking/readiness, signals, restart
semantics, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and
full POSIX descriptor readiness remain blocked.

## Recommended Next Task

The next bounded Milestone 7.4 task should be a documentation-only Pi 5 close
syscall proof plan, for example
phase7-pi5-close-syscall-proof-plan-20260529. That task should not acquire
hardwareTestLock or run hardware; it should define the later serialized
physical proof before any Pi 5 close action.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: reviewed accepted close source inventory, contract,
  core task record, QEMU smoke plan, QEMU close smoke task record, retained
  QEMU evidence path, validation gates, and deferred surfaces.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker
  state.

## Result

Accepted as the documentation-only close syscall closeout checkpoint.
