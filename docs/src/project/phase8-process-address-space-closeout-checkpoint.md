# Phase 8 Process Address-Space Closeout Checkpoint

Status: accepted documentation-only closeout for
phase8-process-address-space-closeout-checkpoint-20260530.

## Accepted Frontier

This checkpoint reconciles the accepted Phase 8 Milestone 8.3 process
address-space slice:

- process address-space source inventory:
  59928e0c929263d087dc37dab847fffdbf635a90;
- process address-space contract:
  84f5ef11f5e8afcb4c5b6196866e212ea17396a2;
- QEMU/substitute process address-space smoke plan:
  48e6cb99869b46f7efaeba74dea7e17a7ebdd076;
- target-independent process address-space core:
  06a5f4ed8e426afd01b77382c070a76d572d7c12; and
- QEMU/substitute process address-space smoke core:
  572faf034b90656c119682498a663cb258c780a5.

The accepted capability is narrow: a validated ProgramImagePlan for immutable
/bin/init can be converted into a ProcessImageInstallPlan and then installed
into a target-independent ProcessAddressSpace model. The model records one
process address-space identity, one owner label, one model page-table root
token, table-page lease records, one zero-before-copy user-frame lease per
installed page, ordered UserText/UserData mapping records, copy/zero byte
accounting, publication state, deterministic rollback, and idempotent teardown.

This is not hardware address-space installation. It does not allocate real
physical frames, install AArch64 descriptors, switch TTBR0_EL1 or TTBR1_EL1,
choose TCR/MAIR/SCTLR/ASID/TLB policy, construct a lower-EL exception frame,
build argv/envp, create a runnable process, mutate descriptors, or perform
filesystem syscalls.

## Retained Evidence

The retained QEMU/substitute evidence is:

    tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log

That log records the accepted fixture, install boundary, and address-space
boundary:

    qemu-process-address-space-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x3892eed223900c65 install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1
    qemu-process-address-space-smoke: success output=ProcessAddressSpace published=true id=0x83000001 owner=0x83001001 root-token=0x1 table-leases=1 user-frame-leases=3 mappings=3 ok=true
    qemu-process-address-space-smoke: side-effects root-leased=true table-leases=1 user-frame-leases=3 mappings-installed=3 copied-bytes=0x8 zeroed-bytes=0x2ff8 scheduler-owner=false descriptors-mutated=false lower-el-frame=false runnable=false ok=true
    qemu-process-address-space-smoke: teardown phase=first mappings-released=3 user-frame-releases=3 table-lease-releases=1 root-released=true already-destroyed=false ok=true
    qemu-process-address-space-smoke: teardown phase=second mappings-released=0 user-frame-releases=0 table-lease-releases=0 root-released=false already-destroyed=true ok=true
    qemu-process-address-space-smoke: final participants=8 expected=8 errors=0 classification=qemu-process-address-space-smoke-complete
    qemu-process-address-space-smoke: PASS

The deterministic no-partial-install and no-leak rejection evidence is:

    qemu-process-address-space-smoke: error case=bad-install-plan errno=-EINVAL partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=null-guard-or-kernel-split errno=-EACCES partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=overlap errno=-EACCES partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=permission-widening errno=-EACCES partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=lease-exhaustion errno=-ENOMEM partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=copy-zero-model-failure errno=-EINVAL partial-install=false leaked-leases=false ok=true

The accepted core and smoke tasks also record cargo -Zjson-target-spec test
passing with 285 no_std tests, cargo fmt passing, the process address-space
smoke passing, the conditional process-install smoke passing, git diff --check
passing, mdbook build passing, and staged whitespace inspection passing before
commit.

## Deferred Surfaces

This closeout accepts no new Rust or assembly behavior. It performs no QEMU
rerun and no Pi 5 hardware run.

Still deferred:

- real physical frame allocation, page-table page allocation, descriptor
  construction, page-table leaf installation, TTBR/TCR/MAIR/SCTLR policy,
  ASID allocation, TLB maintenance, and barrier sequencing;
- lower-EL launch of /bin/init, initial exception frame, SPSR/PSTATE choice,
  ERET, launch-time trap classification, and runnable task state;
- initial user stack, guard page, argv/envp, auxiliary vectors, TLS, libc
  startup, exec/spawn/wait, process table, PID allocation, parent/child
  relation, wait/exit state, signals, and credentials;
- descriptor inheritance, close-on-exec, descriptor-backed filesystem
  syscalls, current/root directory handling, and open-file-description
  lifetime;
- writable filesystem, persistent storage, shell behavior, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy; and
- Pi 5 hardware proof, boot archive publication, TFTP evidence, power-cycle,
  serial observation, or hardwareTestLock acquisition.

## Next Planning State

No explicit queued follow-up task remains after this closeout. Supervisor
planning is required before the worker may choose the next Phase 8.3 direction.
The likely frontier is a new bounded source inventory or contract for real
process launch prerequisites, such as hardware page-table materialization or
lower-EL launch setup, but this checkpoint does not create that task.

Until the supervisor queues explicit scope, non-goals, dependencies,
acceptance criteria, validation gates, documents, and evidence, the worker must
not promote implementation for hardware page tables, lower-EL launch,
argv/envp, exec/spawn/wait, shell, filesystem syscalls, hardware proof,
networking, or SSH.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/evidence review: inspected the accepted process
  address-space source inventory, contract, QEMU/substitute smoke plan,
  process address-space core task record, QEMU/substitute process
  address-space smoke task record, retained smoke evidence, roadmap, SUMMARY,
  and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout.
