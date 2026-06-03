# Talos Review/Refactor Campaign

Status: scheduled

Supervisor state schedules this campaign before further POSIX feature
expansion. The goal is to review Talos as a senior software engineer, fix all
issues found within each subsystem task, and avoid building new features on top
of code that is already showing architectural strain.

## Review Standard

Each subsystem review is implementation work, not a read-only audit.

Required behavior for every review task:

- inspect the subsystem for correctness risks, architectural coupling, dead or
  unused code, unclear APIs, brittle cfg/proof code, oversized modules, naming
  problems, validation gaps, and stale documentation;
- fix all issues found that are in scope, big or small;
- make broad refactors when the code structure justifies them;
- remove unused or dead code instead of preserving it as historical clutter;
- record every finding with disposition: fixed, removed, deferred, or
  not-an-issue;
- commit accepted work before the next subsystem review starts.

## Scheduled Subsystem Reviews

- `talos-review-entry-boot-targets-20260603`: entry, boot, target routing,
  Raspberry Pi 5/QEMU proof harnesses, and target-specific cfg boundaries.
- `talos-review-memory-mmu-allocator-20260603`: memory map, page frames,
  translation/MMU setup, allocator, and MMIO boundaries.
- `talos-review-device-tree-boot-reports-20260603`: DTB parsing, chosen and
  memory nodes, Raspberry Pi 5 boot reports, and reusable reporting
  boundaries.
- `talos-review-console-tty-command-stdio-20260603`: PL011, runtime console,
  TTY, diagnostic command channel, local command loop, stdio, and early
  formatting.
- `talos-review-scheduler-smp-sync-20260603`: scheduler, SMP, synchronization,
  timer/preemption ownership, run queues, and cross-core wake behavior.
- `talos-review-posix-syscall-descriptors-20260603`: POSIX baseline, syscall
  ABI/routing, descriptor tables, descriptor I/O, copy helpers, and error
  surfaces.
- `talos-review-vfs-loader-userspace-20260603`: initramfs/VFS, program loader,
  process install/address-space, page-table materialization, user stack, and
  launch preparation.
- `talos-review-docs-scripts-evidence-hygiene-20260603`: scripts, docs, task
  records, retained evidence, generated artifacts, and roadmap/frontier
  honesty.

## Full Review Cycles

After subsystem reviews, the supervisor schedules at least two complete
whole-repo review cycles:

- `talos-full-review-cycle-1-20260603`: full-system review after subsystem
  fixes, including cross-subsystem coupling, stale abstractions, docs/tests,
  and remaining dead code.
- `talos-full-review-cycle-2-20260603`: second full-system review after cycle
  1 changes settle, focused on regressions introduced by refactors, missed
  dead code, validation gaps, and any remaining fake-feature surfaces.

The campaign ends with `talos-review-campaign-closeout-20260603`, which
reconciles findings, fixes, validation, remaining risks, and whether the
descriptor-backed VFS/open/read feature chain can resume.

## Deferred Until Closeout

The POSIX-backed feature chain remains scheduled but is intentionally behind
this campaign:

- descriptor-backed read-only initramfs/VFS file I/O;
- open/read syscall surface;
- program loader input from the VFS-backed `/bin/init`;
- smallest real initial userspace `/bin/init` launch;
- shell behavior backed by VFS, descriptors, syscalls, and userspace.
