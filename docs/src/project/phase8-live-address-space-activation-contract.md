# Phase 8 Live Address-Space Activation Contract

Status: accepted documentation-only contract for
phase8-live-address-space-activation-contract-20260530.

This contract follows the accepted
[Phase 8 Live Address-Space Activation Source Inventory](phase8-live-address-space-activation-source-inventory.md).
It selects the first target-independent activation-preflight boundary after
accepted loader, install, address-space, materialization, initial process
launch, and initial user stack records. It adds no Rust behavior, assembly
behavior, QEMU execution, Pi 5 hardware run, boot archive publication,
hardware-lock acquisition, TTBR/TCR/MAIR/SCTLR write, ASID allocation, live
TLB invalidation, lower-EL ERET, scheduler runnable publication, process
lifecycle, shell behavior, descriptor-backed filesystem syscalls, writable
filesystem, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Selected Boundary

The next implementation boundary should be a LiveAddressSpaceActivationPlan
record. It consumes the accepted materialization, launch, and stack records
and produces an inspectable activation preflight only. It does not make the
materialized page-table root current in hardware, write architectural
translation registers, invalidate live TLB state, publish a scheduler task,
or ERET to lower EL.

Accepted inputs:

- ProgramImagePlan from src/program_loader.rs, including fixture identity
  phase8-program-loader-elf64-aarch64-v1, source path, digest, entry point,
  and segment permissions.
- ProcessImageInstallPlan from src/process_install.rs, including boundary
  identity phase8-process-install-plan-v1, copied/zeroed page records, and
  lower_el_launch_blocked=true.
- ProcessAddressSpace from src/process_address_space.rs, including boundary
  identity phase8-process-address-space-model-v1, owner identity, mapping
  order, publication state, rollback state, and teardown state.
- ProcessPageTableMaterialization from
  src/process_page_table_materialization.rs, including boundary identity
  phase8-process-page-table-materialization-v1, root/table/user-frame leases,
  descriptor records, activation_blocked=true, and
  kernel_mapping_policy=activation-blocked-no-kernel-half.
- InitialProcessLaunchPlan from src/initial_process_launch.rs, including
  boundary identity phase8-initial-process-launch-plan-v1, validated entry
  provenance, saved-frame intent, and activation_state=blocked-no-ttbr-activation.
- InitialUserStackPlan from src/initial_user_stack.rs, including boundary
  identity phase8-initial-user-stack-plan-v1, initial SP, guard/usable ranges,
  startup metadata, and model-only launch-plan stack binding.
- POSIX user-range and error vocabulary from src/posix.rs, plus AArch64
  translation, register, and exception-vector vocabulary from
  src/memory_map/translation.rs and src/arch/aarch64/.

Accepted output:

- one LiveAddressSpaceActivationPlan whose boundary identity should be
  phase8-live-address-space-activation-plan-v1;
- copied identities for image, install, address-space, materialization,
  launch, and stack inputs;
- selected activation policy identity
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1;
- activation root provenance tying TTBR0_EL1 intent to the materialized root
  lease without writing TTBR0_EL1;
- an explicit TTBR1_EL1/kernel-half policy state of
  blocked-no-accepted-kernel-half-map;
- TCR_EL1 and MAIR_EL1 compatibility observations for the accepted descriptor
  image, normal-memory user descriptors, and device/MMIO diagnostics;
- ASID and TLB preflight fields that remain blocked until allocation and TLBI
  sequencing are accepted;
- kernel reachability checklist for VBAR_EL1, exception vectors, active
  kernel stack, kernel text/data, allocator state, UART/MMIO diagnostics,
  scheduler code/data, and panic/fault reporting;
- launch binding that may change only model activation state from
  blocked-no-ttbr-activation to model-only-activation-preflight-ready; and
- side-effect counters proving no TTBR/TCR/MAIR/SCTLR write, no ASID
  allocation, no TLBI, no DSB/ISB for live activation, no lower-EL ERET, no
  scheduler publication, no process-table mutation, and no descriptor-table
  mutation.

This selected boundary is activation readiness, not live execution. It exists
so the next QEMU/substitute smoke can prove that Talos can assemble and reject
the activation preflight deterministically before any task attempts a real
translation-register mutation.

## Activation Invariant

Before any eventual TTBR/TCR/MAIR/SCTLR mutation, all of these facts must hold:

- the currently accepted kernel translation regime remains current;
- the materialized process root and table pages are owned descriptor images,
  not live CPU roots;
- materialization.activation_blocked=true remains true;
- InitialProcessLaunchPlan.activation_state is blocked-no-ttbr-activation
  until the activation plan is accepted;
- InitialUserStackPlan binds a model-only stack and does not write SP_EL0;
- no live ASID has been allocated and no live TLB entry has been invalidated;
- VBAR_EL1, exception vectors, active kernel stack, UART/MMIO diagnostics,
  scheduler code/data, and panic/fault reporting remain reachable through the
  existing kernel regime; and
- no scheduler task, process table entry, descriptor table inheritance, or
  lower-EL saved frame is publishable as runnable state.

After a later contract accepts live mutation, the same invariant must be
rechecked with hardware-facing facts:

- TTBR0_EL1 root identity must match the accepted materialized root, owner,
  descriptor digest, and process address-space identity.
- TTBR1_EL1 or another kernel-half policy must preserve privileged-only
  reachability for kernel text, data, stack, heap, VBAR_EL1, exception
  vectors, scheduler state, UART/MMIO diagnostics, and panic/fault reporting.
- TCR_EL1 must match the selected split, granule, VA range, shareability,
  cacheability, ASID size, and TTBR0/TTBR1 enable/disable policy.
- MAIR_EL1 must define the normal-memory attribute used by user descriptors
  and the device attribute required for diagnostic MMIO.
- SCTLR_EL1 mutation must be sequenced so instruction fetch, data access,
  stack use, and exception entry remain valid after translation is enabled.
- ASID ownership and TLBI scope must prevent stale user translations from a
  prior address space from being observed.
- DSB and ISB ordering must bracket future TTBR/TCR/MAIR/SCTLR/TLBI mutations
  according to the accepted architecture sequence.
- Fault reporting must not depend on user mappings that may be broken; failed
  activation must still be observable through kernel-owned vectors and UART or
  panic diagnostics.

The first implementation records these expectations as preflight facts. It
must return ENOSYS for requests to perform the live register sequence.

## Policy Fields

The activation plan must expose stable field names so a later smoke can retain
evidence without interpreting private implementation details:

| Field | Required value for this boundary |
| --- | --- |
| boundary identity | phase8-live-address-space-activation-plan-v1 |
| selected policy | preflight-split-user-ttbr0-kernel-reachability-blocked-v1 |
| TTBR0_EL1 intent | materialized process root lease only; no write |
| TTBR1_EL1 policy | blocked-no-accepted-kernel-half-map |
| TCR_EL1 state | compatibility-record-only |
| MAIR_EL1 state | compatibility-record-only |
| SCTLR_EL1 state | mutation-blocked |
| ASID state | blocked-no-asid-allocation |
| TLB state | blocked-no-live-tlbi |
| barriers | planned-only-no-live-dsb-isb |
| kernel reachability | checklist-required-not-yet-proven-live |
| fault reporting | kernel-owned-vector-diagnostics-required |
| launch binding | model-only-activation-preflight-ready |
| runnable publication | blocked-no-runnable-publication |

The selected policy deliberately keeps the first implementation below a live
TTBR switch. A later supervisor-planned contract may replace this policy with
an accepted kernel-half mapping and hardware-facing register sequence, but the
worker must not infer that transition from this document.

## Deterministic Errors And Blockers

Errors must be deterministic and leave no visible partial activation state.

| Condition | Required result |
| --- | --- |
| Missing input, wrong boundary identity, destroyed input, unpublished address space, or mismatched copied identity | EINVAL |
| Entry, stack, descriptor, or permission disagreement between accepted launch, stack, and materialization records | ENOEXEC |
| User range wraps, enters null guard, crosses user-space end, points at kernel/device space, or would require EL0 access to kernel/MMIO mappings | EACCES |
| Missing plan-record capacity or deterministic fixture resource exhaustion | ENOMEM |
| Caller asks for TTBR0_EL1/TTBR1_EL1 write, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, live TLBI, lower-EL ERET, scheduler publication, process-table mutation, descriptor-table inheritance, Pi 5 proof, boot archive publication, or hardware-lock use | ENOSYS |

The activation plan should also preserve explicit blocker vocabulary:

- blocked-no-accepted-kernel-half-map;
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

## No-Partial-Activation Behavior

Plan construction is all-or-nothing. No published activation plan is returned
until all input identities, descriptor records, stack ranges, launch
preconditions, kernel-reachability prerequisites, and side-effect counters are
consistent.

On failure:

1. no LiveAddressSpaceActivationPlan is published;
2. ProcessPageTableMaterialization remains activation_blocked=true;
3. InitialProcessLaunchPlan remains blocked-no-ttbr-activation;
4. InitialUserStackPlan remains model-only stack-ready;
5. no TTBR/TCR/MAIR/SCTLR register intent becomes live;
6. no ASID, TLB, DSB, or ISB side effect is recorded as performed;
7. no lower-EL ERET or SP_EL0/ELR_EL1/SPSR_EL1 architectural write occurs;
8. no scheduler, process-table, descriptor-table, or filesystem state is
   mutated; and
9. teardown remains owned by the already accepted address-space,
   materialization, launch, and stack records.

Teardown of a published activation plan is idempotent and releases only
plan-local model ownership. It must not release or scrub the underlying
materialized user frames, stack frames, address-space records, or loader input
pages.

## Smoke And Implementation Gates

The mechanically next documentation-only task should be
phase8-qemu-live-address-space-activation-smoke-plan-20260530, if queued
dependencies remain satisfied.

That smoke plan should define QEMU/substitute evidence for this selected
activation-preflight boundary:

- scenario identity qemu_live_address_space_activation_smoke;
- retained evidence path under
  tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/;
- boundary identity phase8-live-address-space-activation-plan-v1;
- selected policy
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1;
- classification qemu-live-address-space-activation-smoke-complete and PASS
  vocabulary;
- success observations for copied input identities, TTBR0 root provenance,
  blocked TTBR1/kernel-half policy, TCR/MAIR/SCTLR compatibility records,
  ASID/TLB/barrier blocked states, kernel reachability checklist,
  fault-reporting prerequisites, model-only activation binding, teardown, and
  zero live side effects;
- deterministic rejection observations for identity mismatch, entry/stack/
  descriptor disagreement, forbidden kernel/device/user-range requests,
  missing kernel-reachability prerequisites, live-register request, scheduler
  publication request, lower-EL launch request, and resource exhaustion; and
- conditional regression gates for loader, process-install, address-space,
  materialization, launch, and stack smokes when shared owners are touched.

The later implementation task must at minimum run cargo fmt, the Rust test
suite, the accepted QEMU/substitute smoke if a script exists by then,
git diff --check, mdbook build, and git diff --cached --check before commit.
It must record when a QEMU/substitute smoke is not yet applicable. Pi 5
hardware evidence, boot archive publication, lower-EL user execution, and
scheduler runnable publication remain blocked until separately planned and
accepted.

## Deferred Surfaces

This contract explicitly defers:

- live TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLBI, DSB,
  and ISB mutation;
- accepted kernel-half mapping, live kernel reachability proof, and live
  activation fault recovery;
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

- docs/src/project/phase8-live-address-space-activation-source-inventory.md
- docs/src/project/phase8-initial-user-stack-closeout-checkpoint.md
- docs/src/project/phase8-initial-user-stack-contract.md
- docs/src/project/phase8-initial-process-launch-contract.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/architecture/lower-el-userspace.md
- src/program_loader.rs
- src/process_install.rs
- src/process_address_space.rs
- src/process_page_table_materialization.rs
- src/initial_process_launch.rs
- src/initial_user_stack.rs
- src/posix.rs
- src/memory_map/translation.rs
- src/arch/aarch64/mod.rs
- src/arch/aarch64/exceptions.rs
- src/scheduler.rs
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted live
  address-space activation source inventory, initial user stack and launch
  contracts, process page-table materialization contract, lower-EL architecture
  note, source owners for loader/install/address-space/materialization/
  launch/stack/POSIX/translation/exception/scheduler boundaries, roadmap,
  SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this contract.
