# Phase 7 Final Frontier Source Inventory

Status: accepted source inventory for
phase7-final-frontier-source-inventory-20260530.

## Scope

This documentation-only inventory reconciles the accepted Phase 7.1 through
Phase 7.4 frontier before the final closeout checkpoint. It maps accepted
POSIX baseline, EL0/address-space, syscall/copy boundary, and descriptor-table
work to commit and evidence anchors, records residual risks, and recommends
the next checkpoint task.

It adds no Rust or assembly behavior, runs no QEMU scenario, performs no
Raspberry Pi 5 hardware action, publishes no boot archive, and acquires no
hardwareTestLock. It does not claim a Phase 8 transition or accept
filesystem/program-loading, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, full POSIX readiness, or any deferred
runtime surface.

## Accepted Phase 7 Slice Map

| Slice | Accepted boundary | Commit/evidence anchors | Evidence level |
| --- | --- | --- | --- |
| Phase 7.1 POSIX baseline | Errno-style PosixError names, lexical path parsing, descriptor-table vocabulary, and the fixed-capacity target-independent descriptor core. | phase7-posix-baseline-closeout-checkpoint-20260528 at b6480823a6de9900f281e25ae8c201f49305666d. | static documentation/source inspection; fmt/unit tests for target-independent helpers. |
| Phase 7.2 EL0/address space | Lower-AArch64 trap entry/return vocabulary, user/kernel split, user-memory permission model, QEMU EL0 trap smoke, and serialized Pi 5 lower-EL trap proof. | phase7-el0-trap-proof-closeout-checkpoint-20260529 at ccc9c8ba2fcc8cc441c5c2b38d85bf0a87e278f8; Pi 5 evidence at tasks/evidence/2026-05-28-pi5-el0-trap-proof/local62-clean-final-lower-el0-trap/proof-lines.txt. | QEMU/substitute serial evidence; serialized Pi 5 hardware boot/output; static documentation/source inspection. |
| Phase 7.3 syscall ABI and dispatch | Stable svc #0 syscall ABI, x8 syscall numbers, x0-through-x5 scalar arguments, negative-errno returns, production syscall routing, copy-in/copy-out helpers, proof-only pointer-copy, and descriptor-backed stdout/stderr writes. | phase7-syscall-abi-dispatch-closeout-checkpoint-20260529 at d2e26c385880e82fc15d40682ab5fc25e05d2994; syscall evidence at tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log and tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-proof-lines.txt; pointer-copy evidence at tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log and tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-proof-lines.txt; descriptor-write evidence at tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log and tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt. | target-independent unit tests; QEMU/substitute serial evidence; serialized Pi 5 hardware boot/output; static documentation/source inspection. |
| Phase 7.4 file descriptor table | ProcessOwnerId-backed inherited stdio, process descriptor store lookup, descriptor lifetime/close semantics, stable talos_close, talos_dup, and fixed-proof-stdin talos_read through fd 0/fd 3. | phase7-file-descriptor-table-closeout-checkpoint-20260530 at cf748c75ea84aec1975436a0a50d377fee4a9fbf; retained evidence at tasks/evidence/2026-05-29-qemu-process-descriptor-stdio-smoke-core/qemu-process-descriptor-stdio-smoke.log, tasks/evidence/2026-05-29-qemu-close-syscall-smoke-core/qemu-close-syscall-smoke.log, tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/proof-lines.txt, tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log, tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/proof-lines.txt, tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log, and tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/proof-lines.txt. | target-independent unit tests; QEMU/substitute serial evidence; serialized Pi 5 hardware boot/output; static evidence/documentation inspection. |

## Accepted Frontier

The accepted Phase 7 frontier is bounded to local OS capability required before
filesystem/program-loading planning:

1. Target-independent POSIX error, path, and descriptor-table vocabulary exists
   for the first process/descriptor contracts.
2. A built-in lower-EL payload can enter EL0/lower AArch64, trap synchronously
   through SVC, return to the kernel, and report QEMU and Pi 5 PASS evidence.
3. Stable svc #0 dispatch handles scalar arguments and negative-errno returns
   for talos_nop and unknown syscall behavior.
4. Whole-range copy_from_user and copy_to_user helpers validate before side
   effects and provide recoverable EFAULT mapping for the accepted syscall
   helpers.
5. Descriptor-backed stdout/stderr writes through runtime-console0 are proven
   through QEMU/substitute and serialized Pi 5 evidence.
6. Process-owned inherited stdio tables, close, dup, and fixed-proof-stdin read
   are accepted only in the focused target-independent, QEMU/substitute, and
   Pi 5 proof frontiers recorded by the closeouts above.

## Deferred Surfaces

The following remain outside the accepted Phase 7 frontier:

- runtime-console0-backed stdin, TTY raw/canonical input, hardware UART input,
  pipes, sockets, regular files, directories, device nodes, filesystem-backed
  reads/writes, and VFS/path lookup;
- process loading, ELF parsing, argv/envp setup, PID allocation, exit/wait,
  credentials, sessions, controlling TTY, descriptor inheritance across exec,
  close-on-exec, dup2/fcntl, open-file-description reference counting, object
  finalization, and object teardown;
- local shell, libc/Rust std stdio, portable userland, filesystem/program
  loading, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy;
- blocking/readiness, nonblocking flags, poll/select, wait queues,
  signal/restart behavior, per-thread errno storage, demand paging, resumable
  lower-EL data-abort copy recovery, mmap, shared memory, copy-on-write, and
  user DMA buffers.

## Residual Risks

- All lower-EL and descriptor evidence uses focused built-in proof payloads,
  not loaded user programs.
- The accepted stdin source is fixed proof data, not runtime-console0, TTY, or
  hardware input.
- Descriptor objects are enough for inherited stdio and focused close/dup/read
  behavior, but open-file-description lifetime and final release remain
  unaccepted.
- EFAULT evidence is based on explicit helper validation before side effects,
  not resumable lower-EL data-abort recovery.
- The Pi 5 evidence depended on serialized hardwareTestLock discipline,
  candidate/archive/TFTP identity, known-good controls after inconclusive
  runs, unchanged-candidate reruns, and restore proof. Later physical claims
  should preserve that standard.

## Recommendation

No bounded Phase 7 implementation or evidence task remains mechanically
required before a final closeout checkpoint. The next task should be
phase7-final-closeout-checkpoint-20260530, using this inventory to decide
whether Phase 7 closes for the accepted frontier and whether it can recommend
the first Phase 8 filesystem/program-loading source inventory.

This inventory does not itself set a Phase 8 transition flag and does not
promote any Phase 8 task.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: reviewed the accepted Phase 7 closeout documents,
  retained QEMU/substitute evidence anchors, retained Pi 5 evidence anchors,
  hardware-lock/restore summaries, and deferred-surface lists.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this inventory.
