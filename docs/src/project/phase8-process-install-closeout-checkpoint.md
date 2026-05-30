# Phase 8 Process Install Closeout Checkpoint

Status: accepted documentation-only closeout for
phase8-process-install-closeout-checkpoint-20260530.

## Accepted Frontier

This checkpoint reconciles the accepted Phase 8 Milestone 8.3 process-install
slice:

- process-install source inventory:
  97be8d926033b5394d72dd607e6a5187181cfdfe;
- process-install contract:
  099bb712f37d20d718a4b65ed115592229e4d6bc;
- QEMU/substitute process-install smoke plan:
  a0974d53875b6a373d676434d570c1b6360c58db;
- metadata-only process-install core:
  49a54d91ef7920f74c97ca403a5075ce5f8d84a1; and
- QEMU/substitute process-install smoke core:
  f2363aea4fcd373bec1ab3121f2758eb4a96d18a.

The accepted capability is narrow: a validated ProgramImagePlan for immutable
/bin/init can be converted into a target-independent
ProcessImageInstallPlan. The install plan preserves fixture identity,
source digest, entry point, rounded footprint, ordered UserText/UserData page
records, exact R-X/RW- permissions, clipped file-copy ranges, explicit
zero-fill ranges, and future action metadata for allocate, copy, zero, then
map.

The boundary remains metadata-only. It does not allocate physical frames, copy
bytes into process-owned memory, install page-table leaves, publish mappings,
create a process object, mutate descriptors, construct a lower-EL frame, build
argv/envp, create a scheduler task, or make /bin/init runnable.

## Retained Evidence

The retained QEMU/substitute evidence is:

    tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log

That log records the accepted fixture and process-install boundary:

    qemu-process-install-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x3892eed223900c65 install-boundary=phase8-process-install-plan-v1
    qemu-process-install-smoke: success output=ProcessImageInstallPlan metadata-only=true entry=0x10100 entry-preserved=true footprint=0x3000 pages=3 ok=true
    qemu-process-install-smoke: side-effects frames-allocated=0 mappings-installed=0 process-created=false descriptors-mutated=false lower-el-frame=false runnable=false ok=true
    qemu-process-install-smoke: final participants=7 expected=7 errors=0 classification=qemu-process-install-smoke-complete
    qemu-process-install-smoke: PASS

The deterministic no-partial-install rejection evidence is:

    qemu-process-install-smoke: error case=bad-plan-invariant errno=-EINVAL partial-install=false ok=true
    qemu-process-install-smoke: error case=overlap errno=-EACCES partial-install=false ok=true
    qemu-process-install-smoke: error case=permission-widening errno=-EACCES partial-install=false ok=true
    qemu-process-install-smoke: error case=bad-entry errno=-ENOEXEC partial-install=false ok=true
    qemu-process-install-smoke: error case=budget-overflow errno=-ENOMEM partial-install=false ok=true

The accepted core task also records cargo -Zjson-target-spec test passing with
279 no_std tests, cargo fmt passing, the conditional program-loader smoke
passing, git diff --check passing, mdbook build passing, and staged whitespace
inspection passing before commit.

## Deferred Surfaces

This closeout accepts no new Rust or assembly behavior. It performs no QEMU
rerun and no Pi 5 hardware run.

Still deferred:

- physical process address-space installation and ownership;
- user-frame allocation, physical byte copy, page-table mutation, teardown,
  TTBR/TCR switching, and mapping rollback proof;
- lower-EL launch of the loaded image, initial user frame, SPSR/PSTATE
  choice, ERET, launch-time trap classification, and runnable task state;
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
The likely frontier is a new bounded contract or source inventory for a
process-owned address-space installation mutator, but that is not accepted as
a worker-created task by this checkpoint.

Until the supervisor queues explicit scope, non-goals, dependencies,
acceptance criteria, validation gates, documents, and evidence, the worker must
not promote implementation for frame allocation, page-table installation,
lower-EL launch, argv/envp, exec/spawn/wait, shell, filesystem syscalls,
hardware proof, networking, or SSH.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/evidence review: inspected the accepted
  process-install contract, QEMU/substitute smoke plan, process-install core
  task record, QEMU/substitute process-install smoke task record, retained
  smoke evidence, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout.
