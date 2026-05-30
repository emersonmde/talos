# Phase 7 Final Frontier Source Inventory

Task: phase7-final-frontier-source-inventory-20260530
Status: accepted

## Scope

This documentation-only inventory reconciles accepted Phase 7.1 through Phase
7.4 work across POSIX baseline contracts, EL0/address-space proof, syscall ABI
and copy-helper boundaries, descriptor-backed write/close/dup/read frontiers,
QEMU/substitute evidence, serialized Pi 5 evidence, hardware-lock/restore
records, deferred surfaces, and residual risks.

It does not add implementation, run QEMU, run Pi 5 hardware, publish a boot
archive, acquire hardwareTestLock, or claim a Phase 8 transition.

## Evidence Reviewed

- Phase 7.1 closeout:
  docs/src/project/phase7-posix-baseline-closeout-checkpoint.md.
- Phase 7.2 closeout and Pi 5 lower-EL proof:
  docs/src/project/phase7-el0-trap-proof-closeout-checkpoint.md and
  tasks/evidence/2026-05-28-pi5-el0-trap-proof/local62-clean-final-lower-el0-trap/proof-lines.txt.
- Phase 7.3 syscall ABI/dispatch closeout:
  docs/src/project/phase7-syscall-abi-dispatch-closeout-checkpoint.md.
- Phase 7.4 file descriptor table closeout:
  docs/src/project/phase7-file-descriptor-table-closeout-checkpoint.md.
- Final inventory document:
  docs/src/project/phase7-final-frontier-source-inventory.md.

## Accepted Inventory

The inventory maps the accepted Phase 7 frontier by slice:

- Phase 7.1: target-independent POSIX baseline, path/error model, and
  descriptor-table vocabulary.
- Phase 7.2: lower-EL trap/address-space proof with QEMU/substitute and Pi 5
  lower-AArch64 SVC evidence.
- Phase 7.3: stable svc #0 syscall ABI, production routing, copy helpers,
  pointer-copy proof plumbing, and descriptor-backed stdout/stderr writes.
- Phase 7.4: ProcessOwnerId-backed inherited stdio, descriptor lifetime/close,
  talos_close, talos_dup, and fixed-proof-stdin talos_read through fd 0/fd 3.

No remaining bounded Phase 7 implementation or evidence task is identified as
mechanically required before the final Phase 7 closeout checkpoint.

## Deferred Surfaces

Still blocked: runtime-console0/TTY/hardware stdin, pipes, sockets, regular
files, directories, VFS/filesystem behavior, process loading, ELF, argv/envp,
PID allocation, exit/wait, credentials, sessions, controlling TTY, descriptor
inheritance across exec, close-on-exec, dup2/fcntl, object finalization, local
shell, libc/Rust std stdio, portable userland, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, blocking/readiness,
nonblocking I/O, poll/select, wait queues, signals, demand paging,
recoverable lower-EL data-abort recovery, mmap, shared memory, copy-on-write,
and user DMA buffers.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: accepted Phase 7 closeouts, retained QEMU evidence,
  retained Pi 5 evidence, hardware-lock/restore summaries, and deferred
  surfaces reviewed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

No Rust tests, QEMU run, archive publication, hardware lock acquisition, or Pi
5 run was required because this task changes only Markdown documentation and
durable worker state.

## Next Task

The next mechanically unblocked queued task is
phase7-final-closeout-checkpoint-20260530. It should use this accepted
inventory to decide whether Phase 7 closes for the bounded frontier and whether
to recommend the first Phase 8 filesystem/program-loading source inventory.
