# Phase 7 Pi 5 Dup Syscall Proof Closeout Checkpoint

Status: accepted as the documentation-only Milestone 7.4 Pi 5 dup syscall
proof closeout checkpoint. This checkpoint adds no Rust behavior, assembly
behavior, QEMU rerun, Pi 5 hardware rerun, boot archive publication,
hardwareTestLock acquisition, read syscall behavior, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, object finalization, dup2/fcntl,
or full POSIX descriptor readiness claim.

## Accepted Inputs

| Task | Commit | Evidence level |
| --- | --- | --- |
| phase7-dup-syscall-contract-20260529 | 041ca2f449afc9bd7889497720702b4f4f849bc3 | static documentation/source inspection |
| phase7-dup-syscall-core-20260529 | 2c30e4446f6611edb2bea1b75f226a6e919bf310 | fmt/unit tests/QEMU regression gates |
| phase7-qemu-dup-syscall-smoke-plan-20260529 | 37401fb7d9ff4924acd8a9ed072db1ec3441b261 | static documentation/source inspection |
| phase7-qemu-dup-syscall-smoke-core-20260529 | 5cce637bab95b227f5a98aba99b9104d2a017751 | QEMU/substitute serial evidence |
| phase7-dup-syscall-closeout-checkpoint-20260529 | 84e2306b7e0d1397e3ea002d71c32f6d57556595 | static documentation inspection |
| phase7-pi5-dup-syscall-proof-plan-20260529 | 332853301d62a0a6283236dcfb997b941c8e4add | static documentation inspection |
| phase7-pi5-dup-syscall-proof-20260529 | e4f52f5 | serial hardware boot/output |

## Retained Evidence

- QEMU dup syscall smoke:
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.
- Pi 5 dup syscall proof:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/proof-lines.txt.
- Accepted Pi 5 known-good control:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local7-known-good-control-rerun/proof-lines.txt.
- Pi 5 candidate source commit:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/source-commit.txt
  records candidate source commit
  2d8e5f9de177c4b4040bcbdc826f1efbf715674f.
- Pi 5 archive and image identity:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/digests.txt
  records archive SHA256
  7f1bf15f49245d0590fba24d89ec50094ee579855a6448416aa28abdc4ae0bfd
  and kernel image SHA256
  73a15d22c4082ceeac49bb0e5159d241038d4f39edc62a1f56e6b6c3ba1d941c.
- Pi 5 archive review:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/archive-review.txt
  records kernel_size=114792 and a passing focused archive inspection.
- Pi 5 TFTP and restore evidence:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/tftp-delta-before-restore.json
  and
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/post-snapshot-restore-status.json.
- Hardware-lock timeline:
  hardwareTestLock was acquired at 2026-05-29T23:10:42.269Z for the final
  local7 known-good control rerun and local8 unchanged-candidate rerun, then
  released at 2026-05-29T23:23:40.798Z after restoring the prior accepted boot
  tree.

The retained QEMU/substitute log reports:

~~~text
qemu-dup-syscall-smoke: final participants=14 expected=14 errors=0 classification=qemu-dup-syscall-smoke-complete
qemu-dup-syscall-smoke: PASS
~~~

The retained Pi 5 hardware proof reports:

~~~text
rpi5-dup-syscall-proof: final participants=14 expected=14 errors=0 classification=pi5-dup-syscall-proof-complete
rpi5-dup-syscall-proof: PASS
~~~

## Accepted Capability

Talos has accepted only this bounded physical dup syscall capability:

1. Stable svc #0 with x8 = 3 selects talos_dup through the accepted
   lower-AArch64 syscall route on Raspberry Pi 5.
2. x0 carries the source descriptor, x1 through x5 must be zero, and x0
   returns the new descriptor number or negative errno.
3. talos_dup resolves the current ProcessOwnerId through
   ProcessDescriptorStore and applies
   ProcessDescriptorStore::dup_current_descriptor().
4. Duplicating fd 1 in the four-slot inherited-stdio table returns fd 3, the
   lowest free descriptor, and leaves fd 1 occupied.
5. Duplicating fd 2 after fd 3 is occupied returns -EMFILE without table
   mutation.
6. Nonzero reserved dup arguments return -EINVAL without table mutation.
7. Writes through both source fd 1 and duplicate fd 3 reach runtime-console0
   through copied StdioOutput descriptor entries.
8. close(fd 1) clears only the source descriptor; fd 3 remains writable until
   it is closed independently.
9. Closing fd 3 clears only the duplicate descriptor.
10. Closed source/duplicate descriptors and dup(closed fd 1) return -EBADF
    without console side effects or table mutation.
11. talos_nop, unknown-syscall -ENOSYS, proof-only talos_copy_probe
    quarantine, and diagnostic marker 0x7a10 quarantine remain intact.
12. The initialized ProcessDescriptorStore static is cleaned to PoC before the
    EL2-to-EL1/EL0 proof handoff so the lower-EL Pi 5 handler sees the
    inherited-stdio owner table.

This is a physical proof of talos_dup behavior for the focused built-in
rpi5_dup_syscall_proof scenario. It is not read/stdin behavior, process
loading, filesystem-backed descriptor I/O, object finalization, dup2/fcntl, or
full POSIX descriptor readiness.

## Deferred Surfaces

The following surfaces remain blocked until later explicit tasks accept their
contracts and gates:

- read syscall behavior, stdin/read object model, descriptor allocation beyond
  lowest-free dup, close-on-exec, descriptor inheritance beyond the accepted
  table-local behavior, open-file-description reference counting, object
  finalization, dup2/fcntl, and full POSIX descriptor readiness.
- process loading, VFS/filesystem behavior, filesystem-backed data, pipes,
  sockets, TTY blocking/readiness, local shell, networking, SSH, and portable
  userland.
- partial I/O, EOF, nonblocking mode, wait queues, signals, restart semantics,
  per-thread errno storage, resumable lower-EL user faults, demand paging,
  copy-on-write, shared memory, mmap, and lower-EL fault-table recovery.
- broader cache-maintenance or DMA/cache-driver policy beyond the focused
  ProcessDescriptorStore PoC cleaning required by the accepted Pi 5 proof.
- RP1/PCIe and UART interrupt ownership.

## Residual Risks

- The accepted Pi 5 proof uses a focused built-in payload and a kernel-owned
  current ProcessOwnerId. Loaded user processes, process teardown, exec
  inheritance, and real per-process lifetime policy remain unaccepted.
- Duplicated descriptors currently copy table entries. Open-file-description
  reference counting and final object release remain unaccepted.
- read needs a separate source inventory and contract because it introduces
  copy-out, EOF, blocking/readiness, stdin object policy, and user-visible data
  transfer semantics beyond write/close/dup.
- The ProcessDescriptorStore PoC cleaning is proof-bounded. A broader cache and
  DMA policy must remain owned by later RP1/PCIe/DMA/cache-driver work.

## Recommended Next Task

The next bounded Milestone 7.4 task should be a documentation-only read/stdin
source inventory, phase7-read-stdin-source-inventory-20260529, if the
supervisor queues it with explicit scope, dependencies, acceptance criteria,
validation gates, docs, and evidence requirements.

That inventory should reconcile existing stdin descriptor vocabulary,
copy-out helpers, user-memory permission checks, ProcessDescriptorStore lookup,
runtime-console/stdin object gaps, EOF and readiness policy gaps, retained
write/close/dup evidence, and the exact surfaces still blocked before any read
contract or implementation. The worker must not invent that task, start read
implementation, process loading, VFS/filesystem, shell, networking, SSH,
object finalization, RP1/PCIe, UART interrupt ownership, DMA/cache-driver
work, or a phase transition without an explicit queued task.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: reviewed the accepted dup contract, dup core, QEMU
  dup smoke plan/core, QEMU dup closeout, Pi 5 dup proof plan, Pi 5 proof task
  record, retained QEMU log, retained local7 known-good control, retained
  local8 hardware proof, candidate/archive identity, hardware-lock timeline,
  restore proof, and deferred surfaces.
- static documentation diff: added this closeout, linked it from SUMMARY,
  updated roadmap current status, updated the decision log, and added the task
  record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker state.
