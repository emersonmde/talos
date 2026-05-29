# Phase 7 Dup Syscall Closeout Checkpoint

Status: accepted as the documentation-only Milestone 7.4 QEMU/substitute dup
syscall closeout checkpoint. This checkpoint adds no Rust behavior, assembly
behavior, QEMU rerun, Pi 5 hardware run, boot archive publication,
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

## Retained Evidence

- QEMU dup syscall smoke:
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.
- QEMU descriptor-write regression:
  scripts/qemu-descriptor-write-smoke.sh passed during the accepted smoke core.
- QEMU close syscall regression:
  scripts/qemu-close-syscall-smoke.sh passed during the accepted smoke core.
- Target-independent dup syscall unit tests:
  cargo -Zjson-target-spec test passed with 239 no_std tests during the
  accepted dup syscall core and QEMU dup smoke core.
- Formatting and static inspection:
  cargo fmt --all -- --check, git diff --check, git diff --cached --check,
  and mdbook build passed during the accepted QEMU dup smoke core.

The retained QEMU/substitute log reports:

~~~text
qemu-dup-syscall-smoke: final participants=14 expected=14 errors=0 classification=qemu-dup-syscall-smoke-complete
qemu-dup-syscall-smoke: PASS
~~~

## Accepted Capability

Talos has accepted only this bounded dup syscall capability:

1. Stable svc #0 with x8 = 3 selects talos_dup through the accepted
   lower-AArch64 syscall route.
2. x0 carries the source descriptor, x1 through x5 must be zero, and x0
   returns the new descriptor number or negative errno.
3. talos_dup resolves the current ProcessOwnerId through
   ProcessDescriptorStore and applies
   ProcessDescriptorStore::dup_current_descriptor().
4. Duplicating fd 1 in the four-slot inherited-stdio table returns fd 3, the
   lowest free descriptor.
5. Duplicating into a full table returns -EMFILE without table mutation.
6. Nonzero reserved dup arguments return -EINVAL without table mutation.
7. Writes through both source fd 1 and duplicate fd 3 reach runtime-console0
   through copied StdioOutput descriptor entries.
8. close(fd 1) clears only the source descriptor; fd 3 remains writable until
   it is closed independently.
9. Closed source/duplicate descriptors and dup(closed fd 1) return -EBADF
   without console side effects or table mutation.
10. talos_nop, unknown-syscall -ENOSYS, proof-only talos_copy_probe
    quarantine, and diagnostic marker 0x7a10 quarantine remain intact.

The evidence level for the lower-EL dup syscall path is QEMU/substitute serial
output. The target-independent syscall core also has no_std unit-test evidence.
No physical Pi 5 dup syscall claim is accepted by this checkpoint.

## Deferred Surfaces

The following surfaces remain blocked until later explicit tasks accept their
contracts and gates:

- Pi 5 physical dup proof, boot archive publication, power-cycle, serial
  observation, hardwareTestLock acquisition, and restoration evidence.
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
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Residual Risks

- The accepted lower-EL dup evidence is QEMU/substitute only. It does not
  prove Pi 5 physical dup syscall behavior.
- The accepted descriptor table is ProcessOwnerId-backed, but it is still a
  focused kernel-owned test fixture rather than a loaded user process with
  process lifetime, exit teardown, or exec inheritance.
- Duplicated descriptors copy table entries. Open-file-description reference
  counting and final object release remain unaccepted.
- read needs a separate contract because it introduces copy-out, EOF,
  blocking/readiness, and stdin object policy beyond write/close/dup.

## Recommended Next Task

The next bounded Milestone 7.4 task should be the already queued
documentation-only Pi 5 dup syscall proof plan,
phase7-pi5-dup-syscall-proof-plan-20260529.

That plan should translate the accepted QEMU/substitute dup invariant into a
serialized physical proof with hardwareTestLock ownership, candidate archive
identity, fresh TFTP and serial evidence, inconclusive-run triage, restoration
requirements, exact dup/write/close/error/classification/PASS lines, and an
explicit statement that read, process loading, filesystem, shell, networking,
SSH, object finalization, dup2/fcntl, and full POSIX descriptor readiness
remain blocked.

The worker should not promote a Pi 5 dup run, read implementation, VFS work,
process loading, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver work, or a phase transition without an explicit queued task.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: reviewed the accepted dup contract, core task
  record, QEMU smoke plan, QEMU dup smoke task record, retained QEMU log path,
  validation gates, and deferred surfaces.
- static documentation diff: added this closeout, linked it from SUMMARY,
  updated roadmap current status and milestone summary, updated the decision
  log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker
  state.
