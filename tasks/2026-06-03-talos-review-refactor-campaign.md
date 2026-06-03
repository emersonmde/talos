# Talos Review/Refactor Campaign

Status: accepted through closeout

Supervisor state scheduled this campaign before further POSIX feature
expansion. The campaign reviewed Talos as a senior software engineer, fixed the
issues found within each subsystem task, and avoided building new features on
top of code that was already showing architectural strain.

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

## Accepted Subsystem Reviews

- `talos-review-entry-boot-targets-20260603`: entry, boot, target routing,
  Raspberry Pi 5/QEMU proof harnesses, and target-specific cfg boundaries.
  Accepted at `c2918ca2c0476167a60cf877a278a4b5c1cb0bb0`.
- `talos-review-memory-mmu-allocator-20260603`: memory map, page frames,
  translation/MMU setup, allocator, and MMIO boundaries. Accepted at
  `afd32e86e707d8c46f7e5506094add28998df700`.
- `talos-review-device-tree-boot-reports-20260603`: DTB parsing, chosen and
  memory nodes, Raspberry Pi 5 boot reports, and reusable reporting
  boundaries. Accepted at `595703920c3c5b5e317b5d8f77f4bf9e10032a98`.
- `talos-review-console-tty-command-stdio-20260603`: PL011, runtime console,
  TTY, diagnostic command channel, local command loop, stdio, and early
  formatting. Accepted at `bc6a884aff31a82386d4c08afbc446e196a9e95b`.
- `talos-review-scheduler-smp-sync-20260603`: scheduler, SMP, synchronization,
  timer/preemption ownership, run queues, and cross-core wake behavior.
  Accepted at `b548b8e6361a7625591375caf82dbb088e6a4c60`.
- `talos-review-posix-syscall-descriptors-20260603`: POSIX baseline, syscall
  ABI/routing, descriptor tables, descriptor I/O, copy helpers, and error
  surfaces. Accepted at `372c3cf31b710cbe6276fa8dced567fef4cfaafa`.
- `talos-review-vfs-loader-userspace-20260603`: initramfs/VFS, program loader,
  process install/address-space, page-table materialization, user stack, and
  launch preparation. Accepted at `b8ccec78b10de93951f50491e1ca42da21d9a664`.
- `talos-review-docs-scripts-evidence-hygiene-20260603`: scripts, docs, task
  records, retained evidence, generated artifacts, and roadmap/frontier
  honesty. Accepted at `f2b2886e1110911c8c265f47926cf85e51253f23`.

## Full Review Cycles

After subsystem reviews, the supervisor scheduled two complete whole-repo
review cycles:

- `talos-full-review-cycle-1-20260603`: full-system review after subsystem
  fixes, including cross-subsystem coupling, stale abstractions, docs/tests,
  and remaining dead code. Accepted at
  `4d1b395c359d8762c990fa440e28010e07eaee7f`.
- `talos-full-review-cycle-2-20260603`: second full-system review after cycle
  1 changes settle, focused on regressions introduced by refactors, missed
  dead code, validation gaps, and any remaining fake-feature surfaces.
  Accepted at `c9c317bd65004f3016b9399629b61df583696821`.

The campaign ended with `talos-review-campaign-closeout-20260603`, which
reconciled findings, fixes, validation, remaining risks, and whether the
descriptor-backed VFS/open/read feature chain can resume.

## Post-Closeout Frontier

The POSIX-backed feature chain can resume after this campaign, starting with:

- descriptor-backed read-only initramfs/VFS file I/O;
- open/read syscall surface;
- program loader input from the VFS-backed `/bin/init`;
- smallest real initial userspace `/bin/init` launch;
- shell behavior backed by VFS, descriptors, syscalls, and userspace.
