# Phase 8 QEMU Process Address-Space Smoke Core Task

Task: phase8-qemu-process-address-space-smoke-core-20260530

Status: accepted

## Scope

Implemented and retained QEMU/substitute evidence for the accepted
ProcessAddressSpace model boundary selected by
docs/src/project/phase8-qemu-process-address-space-smoke-plan.md.

The smoke derives the accepted ProgramImagePlan and ProcessImageInstallPlan
for immutable /bin/init, installs a target-independent ProcessAddressSpace
model, reports explicit root/table/user-frame leases, ordered UserText/UserData
mappings, copy/zero accounting, no scheduler/descriptor/lower-EL/runnable side
effects, idempotent teardown, and deterministic no-partial-install rejection
observations.

## Deferred Surfaces

No Pi 5 hardware behavior, boot archive publication, TFTP archive identity,
hardware page-table mutation, TTBR/TCR switch, lower-EL launch, argv/envp,
exec/spawn/wait, descriptor-backed filesystem syscalls, writable filesystem,
persistent storage, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy is claimed by this task.

## Evidence

- retained QEMU/substitute log:
  tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log.
- command used to build/run the smoke:
  scripts/qemu-process-address-space-smoke.sh.
- fixture identity: phase8-program-loader-elf64-aarch64-v1.
- process-install boundary identity: phase8-process-install-plan-v1.
- process address-space boundary identity:
  phase8-process-address-space-model-v1.
- exact classification:
  qemu-process-address-space-smoke: final participants=8 expected=8 errors=0 classification=qemu-process-address-space-smoke-complete.
- exact PASS line: qemu-process-address-space-smoke: PASS.
- success observations: source digest 0x3892eed223900c65, published
  ProcessAddressSpace id 0x83000001, owner 0x83001001, one model root token,
  one table lease, three user-frame leases, three mappings, UserText R-X and
  UserData RW- records, zero-before-copy=true, copied bytes 0x8, zeroed bytes
  0x2ff8, and no scheduler owner, descriptor mutation, lower-EL frame, or
  runnable state.
- teardown observations: first teardown released three mappings, three user
  frames, one table lease, and the root token; second teardown reported
  already-destroyed without double release.
- deterministic rejection observations:
  - bad-install-plan -> -EINVAL, partial-install=false, leaked-leases=false.
  - null-guard-or-kernel-split -> -EACCES, partial-install=false,
    leaked-leases=false.
  - overlap -> -EACCES, partial-install=false, leaked-leases=false.
  - permission-widening -> -EACCES, partial-install=false, leaked-leases=false.
  - lease-exhaustion -> -ENOMEM, partial-install=false, leaked-leases=false.
  - copy-zero-model-failure -> -EINVAL, partial-install=false,
    leaked-leases=false.
- unit tests: cargo -Zjson-target-spec test passed; 285 no_std tests passed,
  including the process-address-space success, rollback/no-partial-install,
  permission preservation, null-guard/user-kernel split, lease ownership, and
  idempotent teardown tests.
- formatting: cargo fmt --all -- --check passed.
- smoke gate: scripts/qemu-process-address-space-smoke.sh passed.
- conditional regression gate: scripts/qemu-process-install-smoke.sh passed
  because the new smoke shares the process-install fixture plan and QEMU
  scenario owner file. Program-loader, readonly-initramfs/VFS,
  lower-EL/syscall, descriptor, read/stdin, and pointer-copy smokes were not
  run because their generation logic, syscall dispatch, descriptor tables,
  user-copy helpers, lower-EL routing, and diagnostic output owners were not
  changed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed; no docs/src files were changed by this
  task.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as the Milestone 8.3 QEMU/substitute process address-space smoke
core. It proves only the target-independent ProcessAddressSpace model evidence
vocabulary. Physical page-table installation, TTBR/TCR switching, lower-EL
launch, argv/envp, exec/spawn/wait, shell, descriptor-backed filesystem
syscalls, Pi 5 hardware proof, writable filesystem, networking, SSH, RP1/PCIe,
UART interrupt ownership, and DMA/cache-driver policy remain blocked until
later explicit tasks accept their contracts and gates.

Commit: recorded in durable supervisor state after acceptance.
