# Phase 7 Pi 5 Close Syscall Proof Closeout Checkpoint

Status: accepted as the documentation-only Milestone 7.4 Pi 5 close syscall
proof closeout checkpoint. This checkpoint adds no Rust behavior, assembly
behavior, QEMU rerun, Pi 5 hardware rerun, boot archive publication,
hardwareTestLock acquisition, dup/read syscall behavior, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, object finalization, or full
POSIX descriptor readiness claim.

## Accepted Inputs

| Task | Commit | Evidence level |
| --- | --- | --- |
| phase7-close-dup-read-syscall-source-inventory-20260529 | 8e17c1d0be80f860ef83bc02a01035dacd78d439 | static documentation/source inspection |
| phase7-close-syscall-contract-20260529 | 687ef5c04e745853230d61ef64845ec90ddb337c | static documentation/source inspection |
| phase7-close-syscall-core-20260529 | ab8915b9696a046b367830e9f5acfd632ee98788 | fmt/unit tests/QEMU regression gates |
| phase7-qemu-close-syscall-smoke-plan-20260529 | cfe3098d559ea21cd69d411f03e456064b265ee7 | static documentation/source inspection |
| phase7-qemu-close-syscall-smoke-core-20260529 | 3be4e1a76e1a065a846f1ebb226bc3e8554c2acf | QEMU/substitute serial evidence |
| phase7-close-syscall-closeout-checkpoint-20260529 | 626f688b230b20cb4a4e1b156cb8c1bb425107e1 | static documentation inspection |
| phase7-pi5-close-syscall-proof-plan-20260529 | 12426dec0d266aa578777ff32dd7e26f4e02d17b | static documentation inspection |
| phase7-pi5-close-syscall-proof-20260529 | 586041eae89944df10e0fa8f3ee7d5fda6a5f6d4 | serial hardware boot/output |

## Retained Evidence

- QEMU close syscall smoke:
  tasks/evidence/2026-05-29-qemu-close-syscall-smoke-core/qemu-close-syscall-smoke.log.
- Pi 5 close syscall proof:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/proof-lines.txt.
- Pi 5 candidate identity:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/source-identity.txt
  records candidate source commit
  4a8be90847c06499bacc08572aef2953a01e52da.
- Pi 5 archive and image identity:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/digests.txt
  records archive SHA256
  5296c1f238f99c84818ecf841ea4956c2330518265a06951a2a54d4471d4712c
  and kernel image SHA256
  2faef603f6a6d5d89857c2ceaee1f0c851021570ffdcd40ba39b1d8df3129657.
- Pi 5 archive review:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/archive-review.txt
  records kernel_size=114792 and a passing focused archive inspection.
- Pi 5 TFTP and restore evidence:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/tftp-delta-before-restore.json
  and
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/post-snapshot-restore-status.json.
- Hardware-lock timeline:
  hardwareTestLock was acquired at 2026-05-29T20:12:00.630Z for the local16
  through local19 serialized triage and proof, then released at
  2026-05-29T20:29:28.954Z after restoring the prior accepted boot tree.

The retained QEMU/substitute log reports:

~~~text
qemu-close-syscall-smoke: final participants=11 expected=11 errors=0 classification=qemu-close-syscall-smoke-complete
qemu-close-syscall-smoke: PASS
~~~

The retained Pi 5 hardware proof reports:

~~~text
rpi5-close-syscall-proof: final participants=11 expected=11 errors=0 classification=pi5-close-syscall-proof-complete
rpi5-close-syscall-proof: PASS
~~~

## Accepted Capability

Talos has accepted only this bounded physical close syscall capability:

1. Stable svc #0 with x8 = 2 selects talos_close through the accepted
   lower-AArch64 syscall route on Raspberry Pi 5.
2. x0 carries the descriptor number, x1 through x5 must be zero, and x0
   returns 0 or negative errno.
3. talos_close resolves the current ProcessOwnerId through
   ProcessDescriptorStore and applies
   ProcessDescriptorStore::close_current_descriptor().
4. Closing fd 1 and fd 2 clears only the selected descriptor slot for the
   current owner on physical Pi 5 hardware.
5. Later talos_write on a closed descriptor returns -EBADF before any
   runtime-console0 side effect.
6. fd 2 remains writable after fd 1 closes and after a failed reserved-register
   close attempt.
7. Repeated close and invalid-descriptor close return -EBADF with the table
   unchanged.
8. Nonzero reserved close arguments return -EINVAL with the table unchanged.
9. talos_nop, unknown-syscall -ENOSYS, proof-only talos_copy_probe quarantine,
   and diagnostic marker 0x7a10 quarantine remain intact.
10. The initialized ProcessDescriptorStore static is cleaned to PoC before the
    EL2-to-EL1/EL0 proof handoff so the lower-EL Pi 5 handler sees the
    inherited-stdio owner table.

This is a physical proof of talos_close behavior for the focused built-in
rpi5_close_syscall_proof scenario. It is not process loading, filesystem-backed
descriptor I/O, object finalization, or full POSIX descriptor readiness.

## Deferred Surfaces

The following surfaces remain blocked until later explicit tasks accept their
contracts and gates:

- dup and read syscalls, stdin/read object model, descriptor allocation policy,
  close-on-exec, descriptor inheritance beyond the accepted table-local
  behavior, open-file-description reference counting, and object finalization.
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
- Closing a descriptor currently clears the table slot; final object release
  and open-file-description reference counting remain unaccepted.
- dup and read need separate contracts because they introduce allocation,
  copy-out, EOF, blocking/readiness, and lifetime policy beyond close.
- The ProcessDescriptorStore PoC cleaning is proof-bounded. A broader cache and
  DMA policy must remain owned by later RP1/PCIe/DMA/cache-driver work.

## Recommended Next Task

The next bounded Milestone 7.4 task should be the already queued
documentation-only dup syscall contract,
phase7-dup-syscall-contract-20260529.

That contract should define talos_dup descriptor allocation, source descriptor
validation, current-owner lookup, reuse ordering, reference/lifetime
vocabulary, deterministic errno mapping, and the exact surfaces still blocked
after close. The worker should not start read, process loading, VFS/filesystem,
shell, networking, SSH, object finalization, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver work, or a phase transition without an explicit
queued task.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: reviewed the accepted close source inventory,
  contract, core, QEMU close smoke plan/core, close closeout, Pi 5 proof plan,
  Pi 5 proof task record, retained QEMU log, retained local19 hardware proof,
  candidate/archive identity, hardware-lock timeline, restore proof, and
  deferred surfaces.
- static documentation diff: added this closeout, linked it from SUMMARY,
  updated roadmap current status, updated the decision log, and added the task
  record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker state.
