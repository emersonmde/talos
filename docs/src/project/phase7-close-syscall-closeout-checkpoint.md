# Phase 7 Close Syscall Closeout Checkpoint

Status: accepted as the documentation-only Milestone 7.4 close syscall
closeout checkpoint. This checkpoint adds no Rust behavior, assembly behavior,
QEMU rerun, Pi 5 hardware run, boot archive publication, hardwareTestLock
acquisition, dup/read syscall behavior, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, object finalization, or full POSIX descriptor
readiness claim.

## Accepted Inputs

| Task | Commit | Evidence level |
| --- | --- | --- |
| phase7-descriptor-lifetime-close-source-inventory-20260529 | 0de2bf2be47986da3220d9fb3edea534448822b8 | static documentation/source inspection |
| phase7-descriptor-lifetime-close-contract-20260529 | 4ff46a6f68bf8349ba0b974d610a8ceb3d92ccd1 | static documentation/source inspection |
| phase7-descriptor-close-core-20260529 | 1e8cdd6fcb4bd16cbb04febd56529b66b0579182 | fmt/unit tests/static inspection |
| phase7-descriptor-close-core-closeout-checkpoint-20260529 | c537670fa9879257db403f260b4a3797f9fd829a | static documentation inspection |
| phase7-close-dup-read-syscall-source-inventory-20260529 | 8e17c1d0be80f860ef83bc02a01035dacd78d439 | static documentation/source inspection |
| phase7-close-syscall-contract-20260529 | 687ef5c04e745853230d61ef64845ec90ddb337c | static documentation/source inspection |
| phase7-close-syscall-core-20260529 | ab8915b9696a046b367830e9f5acfd632ee98788 | fmt/unit tests/QEMU regression gates |
| phase7-qemu-close-syscall-smoke-plan-20260529 | cfe3098d559ea21cd69d411f03e456064b265ee7 | static documentation/source inspection |
| phase7-qemu-close-syscall-smoke-core-20260529 | 3be4e1a76e1a065a846f1ebb226bc3e8554c2acf | QEMU/substitute serial evidence |

## Retained Evidence

- QEMU close syscall smoke:
  tasks/evidence/2026-05-29-qemu-close-syscall-smoke-core/qemu-close-syscall-smoke.log.
- QEMU descriptor-write regression:
  scripts/qemu-descriptor-write-smoke.sh passed during the accepted smoke core.
- QEMU scalar syscall regression:
  scripts/qemu-syscall-smoke.sh passed during the accepted smoke core.
- Target-independent close syscall unit tests:
  cargo -Zjson-target-spec test passed with 231 no_std tests during the
  accepted close syscall core and QEMU close smoke core.
- Formatting and static inspection:
  cargo fmt --all -- --check, git diff --check, git diff --cached --check,
  and mdbook build passed during the accepted QEMU close smoke core.

The retained QEMU/substitute log reports:

~~~text
qemu-close-syscall-smoke: final participants=11 expected=11 errors=0 classification=qemu-close-syscall-smoke-complete
qemu-close-syscall-smoke: PASS
~~~

## Accepted Capability

Talos has accepted only this bounded close syscall capability:

1. Stable svc #0 with x8 = 2 selects talos_close through the accepted
   lower-AArch64 syscall route.
2. x0 carries the descriptor number, x1 through x5 must be zero, and x0
   returns 0 or negative errno.
3. talos_close resolves the current ProcessOwnerId through
   ProcessDescriptorStore and applies
   ProcessDescriptorStore::close_current_descriptor().
4. Closing fd 1 and fd 2 clears only the selected descriptor slot for the
   current owner.
5. Later talos_write on a closed descriptor returns -EBADF before any
   runtime-console0 side effect.
6. fd 2 remains writable after fd 1 closes and after a failed reserved-register
   close attempt.
7. Repeated close and invalid-descriptor close return -EBADF with the table
   unchanged.
8. Nonzero reserved close arguments return -EINVAL with the table unchanged.
9. talos_nop, unknown-syscall -ENOSYS, proof-only talos_copy_probe
   quarantine, and diagnostic marker 0x7a10 quarantine remain intact.

The evidence level for the lower-EL close syscall path is QEMU/substitute
serial output. The target-independent helper and syscall core also have no_std
unit-test evidence. No physical Pi 5 close syscall claim is accepted by this
checkpoint.

## Deferred Surfaces

The following surfaces remain blocked until later explicit tasks accept their
contracts and gates:

- Pi 5 physical close proof, boot archive publication, power-cycle, serial
  observation, hardwareTestLock acquisition, and restoration evidence.
- dup and read syscalls, stdin/read object model, descriptor allocation policy,
  close-on-exec, descriptor inheritance beyond the accepted table-local
  behavior, open-file-description reference counting, and object finalization.
- process loading, VFS/filesystem behavior, filesystem-backed data, pipes,
  sockets, TTY blocking/readiness, local shell, networking, SSH, and portable
  userland.
- partial I/O, EOF, nonblocking mode, wait queues, signals, restart semantics,
  per-thread errno storage, resumable lower-EL user faults, demand paging,
  copy-on-write, shared memory, mmap, and lower-EL fault-table recovery.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Residual Risks

- The accepted lower-EL close evidence is QEMU/substitute only. It does not
  prove Pi 5 physical close syscall behavior.
- The accepted descriptor table is ProcessOwnerId-backed, but it is still a
  focused kernel-owned test fixture rather than a loaded user process with
  process lifetime, exit teardown, or exec inheritance.
- Closing a descriptor currently clears the table slot; final object release
  and open-file-description reference counting remain unaccepted.
- read and dup need separate contracts because they introduce copy-out,
  allocation, EOF, blocking/readiness, and lifetime policy beyond close.

## Recommended Next Task

The next bounded Milestone 7.4 task should be a documentation-only Pi 5 close
syscall proof plan, for example
phase7-pi5-close-syscall-proof-plan-20260529.

That plan should translate the accepted QEMU/substitute close invariant into a
serialized physical proof with hardwareTestLock ownership, candidate archive
identity, fresh TFTP and serial evidence, inconclusive-run triage, restoration
requirements, exact close/write/classification/PASS lines, and an explicit
statement that dup/read, process loading, filesystem, shell, networking, SSH,
object finalization, and full POSIX descriptor readiness remain blocked.

The worker should not promote a Pi 5 close run, dup/read implementation, VFS
work, process loading, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver work, or a phase transition without an explicit
queued task.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: reviewed the accepted close source inventory,
  contract, core task record, QEMU smoke plan, QEMU close smoke task record,
  retained QEMU log path, validation gates, and deferred surfaces.
- static documentation diff: added this closeout, linked it from SUMMARY,
  updated roadmap current status, updated the Phase 7 milestone summary,
  updated the decision log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker
  state.
