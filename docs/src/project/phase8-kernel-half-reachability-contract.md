# Phase 8 Kernel-Half Reachability Contract

Status: accepted documentation-only contract for
phase8-kernel-half-reachability-contract-20260531.

This contract follows the accepted
[Phase 8 Kernel-Half Reachability Source Inventory](phase8-kernel-half-reachability-source-inventory.md).
It selects the first kernel-half reachability boundary after the accepted
live address-space activation preflight. It adds no Rust behavior, assembly
behavior, QEMU execution, Pi 5 hardware run, boot archive publication,
hardware-lock acquisition, live TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1,
SCTLR_EL1, ASID, TLBI, DSB, or ISB mutation, lower-EL ERET, scheduler
runnable publication, process lifecycle, shell behavior, descriptor-backed
filesystem syscalls, writable filesystem, persistent storage, networking,
SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Selected Boundary

The next implementation boundary should be a target-independent
KernelHalfReachabilityPlan. It consumes the accepted loader, install,
address-space, materialization, launch, stack, and activation-preflight
lineage, then produces an inspectable kernel-half policy record only. It does
not build a live TTBR1_EL1 root, copy descriptors into an active root, write
architectural translation registers, invalidate TLB state, publish a
scheduler task, or ERET to lower EL.

Accepted inputs:

- ProgramImagePlan from src/program_loader.rs, including fixture identity
  phase8-program-loader-elf64-aarch64-v1, source digest, entry point, and
  segment permission records.
- ProcessImageInstallPlan from src/process_install.rs, including copied and
  zeroed page records and lower_el_launch_blocked=true.
- ProcessAddressSpace from src/process_address_space.rs, including owner,
  publication state, rollback state, teardown state, and root/table/user-frame
  lease vocabulary.
- ProcessPageTableMaterialization from
  src/process_page_table_materialization.rs, including boundary identity
  phase8-process-page-table-materialization-v1, descriptor records,
  activation_blocked=true, and the previous
  kernel_mapping_policy=activation-blocked-no-kernel-half state.
- InitialProcessLaunchPlan from src/initial_process_launch.rs and
  InitialUserStackPlan from src/initial_user_stack.rs, including model-only
  lower-EL launch and stack readiness.
- LiveAddressSpaceActivationPlan from src/live_address_space_activation.rs,
  including boundary identity phase8-live-address-space-activation-plan-v1,
  TTBR0 root provenance, blocked live-register state, and kernel
  reachability checklist.
- Linker, allocator, page-frame, translation, exception-vector, UART/MMIO,
  runtime console, scheduler, panic/fault reporting, and AArch64 register
  vocabulary identified by the source inventory.

Accepted output:

- one KernelHalfReachabilityPlan whose boundary identity should be
  phase8-kernel-half-reachability-plan-v1;
- selected policy identity
  preflight-ttbr1-shared-kernel-root-reachability-v1;
- copied lineage for image, install, address-space, materialization, launch,
  stack, and activation inputs;
- a shared-kernel-root intent that keeps user descriptors in TTBR0_EL1 and
  reserves TTBR1_EL1 for privileged kernel reachability, without writing
  either register or building a live root;
- required reachability entries for kernel text, rodata, data, bss, vectors,
  active kernel stack, heap/page-frame allocator state, UART/MMIO diagnostics,
  scheduler code/data, panic/fault reporting, and the exception-vector path;
- descriptor-policy intent for privileged executable text/vectors,
  privileged non-executable data/stack/heap/scheduler state, and device
  MMIO diagnostics with no EL0 access;
- TCR_EL1, MAIR_EL1, TTBR0_EL1, TTBR1_EL1, ASID, TLB, and barrier
  compatibility observations that remain record-only;
- a replacement blocker vocabulary that moves activation from
  blocked-no-accepted-kernel-half-map to
  blocked-no-kernel-half-descriptor-image and
  blocked-no-live-register-sequence; and
- side-effect counters proving no descriptor image was made live, no
  TTBR/TCR/MAIR/SCTLR write occurred, no ASID was allocated, no TLBI or
  activation barrier ran, no lower-EL ERET occurred, and no scheduler or
  process-table state was published.

This selected policy deliberately chooses the normal user/kernel split as the
future direction: TTBR0_EL1 owns process-user mappings and TTBR1_EL1 owns a
shared privileged kernel root. The first implementation is still preflight
only. It accepts policy identity and deterministic blockers, not live
translation-register mutation or a hardware-facing descriptor image.

## Kernel-Half Invariant

Before any later task may mutate live translation state, these facts must be
true and inspectable:

- the accepted current kernel translation regime remains current;
- kernel text, vectors, and read-only data have privileged-only reachability
  intent and cannot be executed or read by EL0;
- kernel data, bss, active stack, heap, allocator metadata, scheduler state,
  and panic/fault state have privileged-only data reachability intent and
  cannot be accessed by EL0;
- UART/MMIO diagnostics have device-attribute reachability intent and remain
  privileged-only;
- VBAR_EL1 and exception vectors are reachable without relying on user
  mappings;
- fault reporting remains kernel-owned and can reject an activation preflight
  without scheduler publication or lower-EL execution;
- TTBR0_EL1 root provenance still points only at the accepted materialized
  process root lease;
- TTBR1_EL1 shared-kernel-root provenance is policy-selected but descriptor
  image construction remains blocked;
- TCR_EL1 and MAIR_EL1 compatibility are recorded but not installed;
- ASID allocation, live TLBI, DSB, and ISB activation sequencing remain
  blocked; and
- no scheduler task, process table entry, descriptor table inheritance, or
  lower-EL saved frame is publishable as runnable state.

After a later contract accepts descriptor-image construction, the same
invariant must be rechecked with concrete descriptor digests and root/table
ownership. After an even later contract accepts live mutation, it must be
rechecked against physical instruction fetch, data access, stack use,
exception entry, UART/MMIO diagnostics, ASID/TLB scope, and barrier ordering.

## Policy Fields

The plan must expose stable field names so a later QEMU/substitute smoke can
retain evidence without interpreting private implementation details:

| Field | Required value for this boundary |
| --- | --- |
| boundary identity | phase8-kernel-half-reachability-plan-v1 |
| selected policy | preflight-ttbr1-shared-kernel-root-reachability-v1 |
| user root intent | TTBR0_EL1 materialized process root provenance only |
| kernel root intent | TTBR1_EL1 shared privileged kernel root, descriptor image blocked |
| kernel image ranges | linker-provenanced text/rodata/data/bss/vectors |
| stack policy | active kernel stack privileged-only reachable |
| heap/page-frame policy | allocator and page-frame metadata privileged-only reachable |
| diagnostics policy | UART/MMIO device reachability privileged-only |
| scheduler policy | scheduler code/data reachable, runnable publication blocked |
| fault-reporting policy | VBAR/vector/panic path required before live activation |
| TCR_EL1 state | split compatibility-record-only |
| MAIR_EL1 state | normal/device attribute compatibility-record-only |
| ASID state | blocked-no-asid-allocation |
| TLB state | blocked-no-live-tlbi |
| barriers | planned-only-no-live-dsb-isb |
| live register mutation | blocked-no-live-register-sequence |
| descriptor image | blocked-no-kernel-half-descriptor-image |

The plan may let LiveAddressSpaceActivationPlan consume a policy-selected
kernel reachability record, but only as model evidence. It must not claim a
live kernel-half map, lower-EL launch, scheduler publication, or Pi 5 proof.

## Deterministic Errors And Blockers

Errors must be deterministic and leave no visible partial reachability plan.

| Condition | Required result |
| --- | --- |
| Missing input, wrong boundary identity, destroyed input, unpublished address space, or mismatched copied identity | EINVAL |
| Loader, install, materialization, launch, stack, or activation lineage disagreement | ENOEXEC |
| Kernel range overlaps user/device/null-guard space, lacks linker provenance, wraps, is unaligned where alignment is required, or requests EL0 access | EACCES |
| Device/MMIO diagnostic reachability lacks device-attribute intent or is requested as normal memory | EACCES |
| Missing required VBAR/vector/stack/heap/scheduler/UART/fault-reporting prerequisite | ENOSYS |
| Missing deterministic plan capacity | ENOMEM |
| Caller asks for kernel-half descriptor image construction, TTBR0_EL1/TTBR1_EL1 write, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, live TLBI, activation DSB/ISB, lower-EL ERET, scheduler publication, process-table mutation, Pi 5 proof, boot archive publication, or hardware-lock use | ENOSYS |

The plan should preserve explicit blocker vocabulary:

- blocked-no-kernel-half-descriptor-image;
- blocked-no-live-register-sequence;
- blocked-no-asid-allocation;
- blocked-no-live-tlbi;
- blocked-no-lower-el-eret;
- blocked-no-runnable-publication;
- blocked-no-process-lifecycle;
- blocked-no-startup-abi-expansion;
- blocked-no-filesystem-syscalls; and
- blocked-no-pi5-hardware-proof.

These blockers are contract facts, not implementation failures.

## No-Partial Behavior

Plan construction is all-or-nothing. No KernelHalfReachabilityPlan is returned
until all input identities, required kernel ranges, diagnostic mappings,
fault-reporting prerequisites, compatibility observations, and side-effect
counters are consistent.

On failure:

1. no KernelHalfReachabilityPlan is published;
2. ProcessPageTableMaterialization remains activation_blocked=true;
3. LiveAddressSpaceActivationPlan remains model-only and live-register
   blocked;
4. no TTBR0_EL1, TTBR1_EL1, TCR_EL1, MAIR_EL1, or SCTLR_EL1 intent becomes
   live;
5. no kernel-half descriptor image is created or installed;
6. no ASID, TLB, DSB, or ISB side effect is recorded as performed;
7. no lower-EL ERET or SP_EL0/ELR_EL1/SPSR_EL1 architectural write occurs;
8. no scheduler, process-table, descriptor-table, or filesystem state is
   mutated; and
9. teardown remains owned by the already accepted address-space,
   materialization, launch, stack, and activation records.

Teardown of a published plan is idempotent and releases only plan-local model
ownership. It must not release or scrub underlying process roots, user frames,
kernel allocator state, stack memory, UART/MMIO state, scheduler state, or
loader input pages.

## Smoke And Implementation Gates

The mechanically next documentation-only task should be
phase8-qemu-kernel-half-reachability-smoke-plan-20260531, if queued
dependencies remain satisfied.

That smoke plan should define QEMU/substitute evidence for this selected
preflight boundary:

- scenario identity qemu_kernel_half_reachability_smoke;
- retained evidence path under
  tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/;
- boundary identity phase8-kernel-half-reachability-plan-v1;
- selected policy preflight-ttbr1-shared-kernel-root-reachability-v1;
- classification qemu-kernel-half-reachability-smoke-complete and PASS
  vocabulary;
- success observations for copied accepted input lineage, selected TTBR1
  shared-kernel-root policy, required kernel text/data/stack/heap/vector/
  UART/MMIO/scheduler/fault-reporting entries, compatibility-only register
  state, deterministic blockers, idempotent teardown, and zero live side
  effects;
- deterministic rejection observations for identity mismatch, missing kernel
  range, missing diagnostic/fault-reporting prerequisite, forbidden EL0 access,
  bad device attribute intent, live-register request, descriptor-image request,
  scheduler publication request, lower-EL launch request, and resource
  exhaustion; and
- conditional regression gates for loader, process-install, address-space,
  materialization, launch, stack, and live-activation smokes when shared
  owners are touched.

The later implementation task must at minimum run cargo fmt, the Rust test
suite, the accepted QEMU/substitute smoke if a script exists by then,
git diff --check, mdbook build, and git diff --cached --check before commit.
It must record when a QEMU/substitute smoke is not yet applicable. Pi 5
hardware evidence, boot archive publication, lower-EL user execution, and
scheduler runnable publication remain blocked until separately planned and
accepted.

## Deferred Surfaces

This contract explicitly defers:

- kernel-half descriptor-image construction and any live TTBR1_EL1 root;
- live TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLBI, DSB,
  and ISB mutation;
- live activation fault recovery under a mutated translation regime;
- lower-EL ERET, SP_EL0/ELR_EL1/SPSR_EL1 architectural writes, trap return,
  and loaded /bin/init execution;
- scheduler runnable publication, current-process state, process table/PID
  allocation, parent/child lifecycle, exit, wait, exec, spawn, signals, and
  credentials;
- argv/envp/auxv/TLS expansion, libc startup, dynamic stack growth,
  guard-fault recovery, copy-on-write, and demand paging;
- descriptor inheritance, close-on-exec, cwd/root, descriptor-backed
  filesystem syscalls, writable filesystem state, and shell behavior;
- Pi 5 hardware proof, boot archive publication, hardwareTestLock
  acquisition, TFTP/serial evidence, and physical serial claims; and
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Reviewed Inputs

- docs/src/project/phase8-kernel-half-reachability-source-inventory.md
- docs/src/project/phase8-live-address-space-activation-closeout-checkpoint.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-initial-process-launch-contract.md
- docs/src/project/phase8-initial-user-stack-contract.md
- src/live_address_space_activation.rs
- src/process_page_table_materialization.rs
- src/memory_map/layout.rs
- src/memory_map/page_frames.rs
- src/memory_map/translation.rs
- src/arch/aarch64/exceptions.rs
- src/arch/aarch64/vectors.S
- src/mmio.rs
- src/pl011.rs
- src/runtime_console.rs
- src/scheduler.rs
- docs/src/architecture/memory.md
- docs/src/architecture/lower-el-userspace.md
- docs/src/architecture/exceptions.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted kernel-half
  reachability source inventory, live address-space activation docs, adjacent
  Phase 8 contracts, source owners for translation/vector/UART/MMIO/
  scheduler/fault-reporting boundaries, architecture notes, roadmap, SUMMARY,
  and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this contract.
