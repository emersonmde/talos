# Phase 7 Close Syscall Core

Task: phase7-close-syscall-core-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Wire the accepted close syscall contract through target-independent syscall
dispatch and the current process-owned descriptor table without adding dup,
read, QEMU close proof, or Pi 5 hardware proof behavior.

## Scope

- Added `TALOS_CLOSE_SYSCALL` as stable syscall number 2 and
  `SyscallNumber::TalosClose`.
- Added `dispatch_process_descriptor()` as the context-bearing syscall helper
  for process-owned descriptor operations. `talos_write` now has a reusable
  path that first resolves `ProcessDescriptorStore::current_descriptor_table()`
  before runtime-console output, and `talos_close` routes through
  `ProcessDescriptorStore::close_current_descriptor()`.
- Preserved scalar dispatch behavior for context-requiring syscalls:
  `talos_write` and `talos_close` return `-ENOTSUP` without descriptor-store
  context, while unknown syscalls continue to return `-ENOSYS`.
- Kept the existing proof-only copy-probe quarantine unchanged.
- Updated QEMU and Pi 5 syscall smoke/proof match arms only for exhaustiveness
  so context-requiring syscalls remain unexpected in those scenarios.

## Non-Goals

- No dup/read syscall implementation, stdin/read object model, process
  loading, VFS/filesystem, shell, networking, SSH, Pi 5 hardware run, boot
  archive publication, hardwareTestLock acquisition, object finalization, or
  full POSIX descriptor readiness claim.
- No lower-AArch64 QEMU close smoke proof. That remains the next separate
  evidence task.

## Changed Files

- `src/syscall.rs`
- `src/target/qemu_virt.rs`
- `src/target/rpi5.rs`
- `docs/src/decisions/README.md`
- `docs/src/roadmap.md`
- `tasks/2026-05-29-phase7-close-syscall-core.md`

## Evidence

- Accepted contract dependency:
  `687ef5c04e745853230d61ef64845ec90ddb337c`.
- Focused unit-test coverage:
  `descriptor_close_number_requires_context_in_scalar_dispatch`,
  `talos_close_stdout_blocks_later_process_descriptor_write`,
  `talos_close_stderr_uses_the_same_table_local_rule`,
  `talos_close_failures_are_deterministic_and_do_not_mutate_on_einval`, and
  `talos_close_preserves_duplicate_descriptor_lifetime`.
- Formatting:
  `cargo fmt --all -- --check` passed.
- Unit tests:
  `cargo -Zjson-target-spec test` passed with 231 no_std tests.
- QEMU/substitute regression:
  `scripts/qemu-syscall-smoke.sh` passed with
  `classification=qemu-syscall-smoke-complete` and `PASS`.
- QEMU/substitute descriptor-write regression:
  `scripts/qemu-descriptor-write-smoke.sh` passed with
  `classification=qemu-descriptor-write-smoke-complete` and `PASS`.
- Static inspection:
  `git diff --check` passed.
- Documentation:
  `mdbook build` passed.

## Deferred Work

QEMU close syscall smoke, Pi 5 physical close proof, dup/read syscalls,
stdin/read object model, descriptor duplication ABI, process loading,
fork/spawn/exec, close-on-exec application, process exit teardown,
open-file-description reference counting, object finalizers, VFS/filesystem,
regular files, directories, pipes, sockets, TTY blocking/readiness, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor readiness remain blocked.

## Result

Accepted as the target-independent close syscall core. The next bounded task
should be `phase7-qemu-close-syscall-smoke-plan-20260529`, scoped to a
documentation-only QEMU close smoke plan before any QEMU close implementation
or Pi 5 physical proof work.
