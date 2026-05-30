# Phase 7 Pi 5 Read And Stdin Proof Closeout Checkpoint

Task: phase7-pi5-read-stdin-proof-closeout-checkpoint-20260530
Status: accepted

## Scope

This documentation-only checkpoint reconciles the accepted read/stdin
inventory, contract, core, QEMU/substitute smoke, Pi 5 proof plan, physical
Pi 5 proof, hardware-lock timeline, restore proof, validation gates, deferred
surfaces, and residual risks before Milestone 7.4 file descriptor table
closeout.

It does not add Rust or assembly behavior, rerun QEMU, run Pi 5 hardware,
publish an archive, acquire hardwareTestLock, add runtime-console0/TTY or
hardware stdin, add process loading, filesystem behavior, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, object finalization,
dup2/fcntl, signals, wait queues, nonblocking I/O, or DMA/cache-driver policy.

## Evidence Reviewed

- QEMU/substitute read/stdin retained log:
  tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log.
- Pi 5 read/stdin retained proof lines:
  tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/proof-lines.txt.
- Pi 5 local5 TFTP candidate fetch:
  tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/tftp-delta-before-restore.json.
- Pi 5 local5 restore proof:
  tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/post-snapshot-restore-status.json.
- Accepted physical proof task record:
  tasks/2026-05-30-phase7-pi5-read-stdin-proof.md.

## Accepted Closeout

The closeout document is
docs/src/project/phase7-pi5-read-stdin-proof-closeout-checkpoint.md.

It records the accepted frontier as fixed-proof-stdin talos_read only:
inherited fd 0 can be duplicated to fd 3, both descriptors read from the fixed
proof buffer, copy-out faults and invalid/reserved/error cases preserve cursor
and user-memory state, EOF returns zero, scalar syscall regressions remain
intact, talos_copy_probe remains quarantined as -ENOSYS, and diagnostic marker
0x7a10 remains proof-only.

The retained physical proof is tied to implementation commit
fd2be8ea42ddf88dd4cff120439ab1d3df51bce1, archive SHA-256
5f91281b2dcdfb1bca6fddd6dde6c3f0b39d89f4a4274a5bf91127d8ba833983, candidate
kernel digest 1b7417340d4b0dc44e741683464900500667929c2089b4c1ea88dc050f06d014,
a 114816-byte da591740/kernel_2712.img TFTP fetch, retained serial
classification=pi5-read-stdin-proof-complete, rpi5-read-stdin-proof: PASS, and
restore proof for the prior accepted 104136-byte boot tree.

## Deferred Surfaces

Still blocked: runtime-console0/TTY/hardware stdin, pipes, sockets, regular
files, filesystem-backed reads, blocking/readiness/nonblocking I/O,
poll/select, wait queues, signals/restart behavior, process loading, ELF,
argv/envp, PID allocation, exit/wait, credentials, sessions, controlling TTY,
VFS/filesystem behavior, local shell, networking, SSH, descriptor inheritance
across exec, close-on-exec, dup2/fcntl, object finalizers, full POSIX
descriptor readiness, demand paging, recoverable lower-EL data-abort copy
tables, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, user DMA
buffers, and memory-mapped files.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: retained QEMU/Pi 5 proof evidence and restore/TFTP files
  reviewed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

No Rust tests, QEMU rerun, archive publication, hardware lock acquisition, or
Pi 5 run was required because this task changes only Markdown documentation and
durable worker state.

## Next Task

The next mechanically derivable task is the already queued
phase7-file-descriptor-table-closeout-checkpoint-20260530. It should reconcile
the Milestone 7.4 descriptor-table slice without claiming a phase transition.
