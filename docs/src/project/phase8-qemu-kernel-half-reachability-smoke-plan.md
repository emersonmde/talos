# Phase 8 QEMU Kernel-Half Reachability Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute
kernel-half reachability smoke plan after the accepted
[Phase 8 Kernel-Half Reachability Contract](phase8-kernel-half-reachability-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition, live
TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLBI, DSB, or ISB
mutation, lower-EL ERET, scheduler runnable publication, process lifecycle,
shell behavior, descriptor-backed filesystem syscalls, writable filesystem,
persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The purpose of this plan is to make the next implementation task mechanical:
construct an inspectable KernelHalfReachabilityPlan from the accepted loader,
install, address-space, materialization, launch, stack, and activation
preflight records, then retain one QEMU/substitute smoke log proving
kernel-half policy selection, required kernel reachability entries,
deterministic blockers, no-partial construction, idempotent teardown, and zero
live translation side effects.

## Smoke Invariant

The first QEMU/substitute kernel-half reachability smoke must demonstrate one
bounded invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_kernel_half_reachability_smoke.
2. The scenario consumes the accepted ProgramImagePlan,
   ProcessImageInstallPlan, ProcessAddressSpace,
   ProcessPageTableMaterialization, InitialProcessLaunchPlan,
   InitialUserStackPlan, and LiveAddressSpaceActivationPlan records for
   immutable /bin/init.
3. The fixture identity line records loader identity
   phase8-program-loader-elf64-aarch64-v1, process-install boundary
   phase8-process-install-plan-v1, address-space boundary
   phase8-process-address-space-model-v1, materialization boundary
   phase8-process-page-table-materialization-v1, launch boundary
   phase8-initial-process-launch-plan-v1, stack boundary
   phase8-initial-user-stack-plan-v1, activation boundary
   phase8-live-address-space-activation-plan-v1, kernel-half boundary
   phase8-kernel-half-reachability-plan-v1, and kernel-half policy
   preflight-ttbr1-shared-kernel-root-reachability-v1.
4. The success path creates exactly one KernelHalfReachabilityPlan with copied
   accepted input lineage.
5. The plan records TTBR0_EL1 intent as the materialized process root lease
   only and must report no TTBR0_EL1 write.
6. The plan records TTBR1_EL1 intent as a shared privileged kernel-root policy
   only and must report descriptor image construction as blocked.
7. Kernel text, rodata, data, bss, vectors, active stack, heap/page-frame
   allocator metadata, UART/MMIO diagnostics, scheduler code/data, and
   panic/fault reporting are all required reachability entries.
8. TCR_EL1 and MAIR_EL1 observations are compatibility-record-only.
9. SCTLR_EL1, ASID, TLB, DSB, and ISB observations remain blocked from live
   mutation.
10. Side-effect records show no TTBR/TCR/MAIR/SCTLR writes, no kernel-half
    descriptor image installed, no ASID allocation, no live TLB invalidation,
    no live DSB/ISB activation sequence, no lower-EL ERET, no scheduler
    publication, no process-table mutation, and no descriptor-table mutation.
11. Negative cases prove deterministic rejection for identity mismatch,
    missing kernel range, missing diagnostic/fault-reporting prerequisite,
    forbidden EL0 access, bad device attribute intent, live-register request,
    descriptor-image request, scheduler publication request, lower-EL launch
    request, and deterministic resource exhaustion.
12. Failure cases must prove no partial reachability plan: no
    KernelHalfReachabilityPlan after validation failure, materialization
    remains activation_blocked=true, live activation remains model-only,
    no architectural register intent becomes live, no descriptor image is
    installed, and no scheduler, process, descriptor-table, or filesystem
    state changes.
13. Teardown observations prove that a published preflight record releases
    only plan-local model ownership and leaves the accepted input records owned
    by their existing teardown paths.
14. The smoke prints final classification and PASS only after success,
    rejection, teardown, no-partial, and zero-side-effect observations have
    been recorded.

If implementation work needs a different scenario name, evidence path,
boundary identity, policy identity, PASS/classification vocabulary, live
descriptor-image construction, hardware involvement, live TTBR/TCR/MAIR/SCTLR
mutation, ASID/TLB side effect, lower-EL launch semantics, scheduler
publication, or process lifecycle behavior, it must stop for supervisor
planning instead of accepting a changed smoke.

## Fixture And Boundary Identity

The accepted loader fixture identity remains:

    phase8-program-loader-elf64-aarch64-v1

The accepted process-install boundary remains:

    phase8-process-install-plan-v1

The accepted process address-space boundary remains:

    phase8-process-address-space-model-v1

The accepted non-activating materialization boundary remains:

    phase8-process-page-table-materialization-v1

The accepted initial launch boundary remains:

    phase8-initial-process-launch-plan-v1

The accepted initial user stack boundary remains:

    phase8-initial-user-stack-plan-v1

The accepted activation preflight boundary remains:

    phase8-live-address-space-activation-plan-v1

The accepted kernel-half reachability boundary for this smoke is:

    phase8-kernel-half-reachability-plan-v1

The accepted kernel-half policy for this smoke is:

    preflight-ttbr1-shared-kernel-root-reachability-v1

Required success semantics:

| Field | Required value |
| --- | --- |
| source path | /bin/init |
| loader fixture | phase8-program-loader-elf64-aarch64-v1 |
| install boundary | phase8-process-install-plan-v1 |
| address-space boundary | phase8-process-address-space-model-v1 |
| materialization boundary | phase8-process-page-table-materialization-v1 |
| launch boundary | phase8-initial-process-launch-plan-v1 |
| stack boundary | phase8-initial-user-stack-plan-v1 |
| activation boundary | phase8-live-address-space-activation-plan-v1 |
| kernel-half boundary | phase8-kernel-half-reachability-plan-v1 |
| kernel-half policy | preflight-ttbr1-shared-kernel-root-reachability-v1 |
| TTBR0_EL1 intent | materialized process root lease only; no write |
| TTBR1_EL1 intent | shared privileged kernel-root policy; descriptor image blocked |
| kernel reachability | text, rodata, data, bss, vectors, active stack, heap, page frames, UART/MMIO, scheduler, panic/fault reporting |
| TCR_EL1 state | split compatibility-record-only |
| MAIR_EL1 state | normal/device compatibility-record-only |
| SCTLR_EL1 state | mutation-blocked |
| ASID state | blocked-no-asid-allocation |
| TLB state | blocked-no-live-tlbi |
| barriers | planned-only-no-live-dsb-isb |
| descriptor image | blocked-no-kernel-half-descriptor-image |
| live register mutation | blocked-no-live-register-sequence |
| runnable publication | blocked-no-runnable-publication |
| hardware side effects | none |

The fixture must not claim that /bin/init can run. Live TTBR0_EL1/TTBR1_EL1
programming, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, TLB
invalidation, DSB/ISB live activation sequencing, lower-EL ERET, scheduler
handoff, process lifecycle, descriptor-backed filesystem syscalls, shell
behavior, and Pi 5 hardware proof remain outside this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-kernel-half-reachability-smoke: final participants=16 expected=16 errors=0 classification=qemu-kernel-half-reachability-smoke-complete
    qemu-kernel-half-reachability-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-kernel-half-reachability-smoke: start
    qemu-kernel-half-reachability-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-boundary=phase8-live-address-space-activation-plan-v1 kernel-half-boundary=phase8-kernel-half-reachability-plan-v1 kernel-half-policy=preflight-ttbr1-shared-kernel-root-reachability-v1
    qemu-kernel-half-reachability-smoke: success output=KernelHalfReachabilityPlan published=true copied-identities=true kernel-half-boundary=phase8-kernel-half-reachability-plan-v1 kernel-half-policy=preflight-ttbr1-shared-kernel-root-reachability-v1 ok=true
    qemu-kernel-half-reachability-smoke: root-policy ttbr0-root=materialized-process-root-lease ttbr0-written=false ttbr1-policy=shared-privileged-kernel-root ttbr1-written=false descriptor-image=blocked-no-kernel-half-descriptor-image ok=true
    qemu-kernel-half-reachability-smoke: reachability kernel-text=true rodata=true data=true bss=true vectors=true active-stack=true heap=true page-frames=true uart-mmio-diagnostics=true scheduler-code-data=true panic-fault-reporting=true ok=true
    qemu-kernel-half-reachability-smoke: permissions text-exec=privileged-only data-exec=false device-normal-memory=false el0-kernel-access=false ok=true
    qemu-kernel-half-reachability-smoke: compatibility tcr-state=split-compatibility-record-only mair-state=normal-device-compatibility-record-only sctlr-state=mutation-blocked ok=true
    qemu-kernel-half-reachability-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb live-register-sequence=blocked-no-live-register-sequence ok=true
    qemu-kernel-half-reachability-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false descriptor-image-installed=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true
    qemu-kernel-half-reachability-smoke: teardown plan-local-released=true input-records-owned=true descriptor-image-installed=false idempotent=true ok=true
    qemu-kernel-half-reachability-smoke: error case=identity-mismatch errno=-EINVAL partial-plan=false ok=true
    qemu-kernel-half-reachability-smoke: error case=missing-kernel-range errno=-EACCES partial-plan=false ok=true
    qemu-kernel-half-reachability-smoke: error case=missing-diagnostic-fault-reporting errno=-ENOSYS partial-plan=false ok=true
    qemu-kernel-half-reachability-smoke: error case=forbidden-el0-access errno=-EACCES partial-plan=false ok=true
    qemu-kernel-half-reachability-smoke: error case=bad-device-attribute-intent errno=-EACCES partial-plan=false ok=true
    qemu-kernel-half-reachability-smoke: error case=live-register-request errno=-ENOSYS partial-plan=false ok=true
    qemu-kernel-half-reachability-smoke: error case=descriptor-image-request errno=-ENOSYS partial-plan=false ok=true
    qemu-kernel-half-reachability-smoke: error case=scheduler-publication-request errno=-ENOSYS partial-plan=false ok=true
    qemu-kernel-half-reachability-smoke: error case=lower-el-launch-request errno=-ENOSYS partial-plan=false ok=true
    qemu-kernel-half-reachability-smoke: error case=resource-exhaustion errno=-ENOMEM partial-plan=false ok=true

The implementation may print additional range digests, descriptor-intent
records, lease tokens, teardown records, or diagnostic fields. The required
line shapes must stay stable enough for the script gate. Hex values and lease
identifiers are field placeholders because the later implementation task owns
the exact model representation.

## Failure Classification

The smoke must distinguish kernel-half contract failures from scenario wiring
failures:

- Contract failure: the plan accepts mismatched identities, loses TTBR0 root
  provenance, treats the blocked TTBR1 descriptor image as live, permits EL0
  access to kernel ranges, accepts bad device attributes, reports TCR/MAIR/
  SCTLR as live rather than compatibility-only or blocked, allocates ASID
  state, mutates TLB/barrier state, skips required kernel reachability
  entries, publishes scheduler/process/descriptor state, reports the wrong
  errno, or leaves partial plan state visible after rejection.
- Scenario wiring failure: the scenario cannot select
  qemu_kernel_half_reachability_smoke, cannot obtain the accepted input
  lineage through LiveAddressSpaceActivationPlan, cannot retain a fresh log,
  cannot print the fixture identity line, or cannot drive success, negative,
  no-partial, teardown, and zero-side-effect observations in order.
- Regression failure: an accepted loader, process-install, address-space,
  materialization, launch, stack, live-activation, or related Phase 8 gate
  required by this plan fails after implementation changes touch shared
  owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_kernel_half_reachability_smoke.
2. Confirm the retained log path is fresh for the run.
3. Confirm the fixture identity and copied lineage line printed before the
   first success or rejection observation.
4. Confirm the scenario did not attempt live register mutation, lower-EL
   launch, scheduler publication, or hardware lock acquisition.

## Conditional Regression Gates

The smoke-core task must run the accepted qemu-kernel-half-reachability smoke.
It must also run adjacent QEMU/substitute smokes when implementation changes
touch shared owners:

- program-loader smoke if src/program_loader.rs or loader fixture code changes;
- process-install smoke if src/process_install.rs changes;
- process-address-space smoke if src/process_address_space.rs changes;
- process-page-table-materialization smoke if
  src/process_page_table_materialization.rs or page-table descriptor modeling
  changes;
- initial-process-launch smoke if src/initial_process_launch.rs changes;
- initial-user-stack smoke if src/initial_user_stack.rs changes; and
- live-address-space-activation smoke if src/live_address_space_activation.rs
  or activation-preflight vocabulary changes.

If none of those owners change, the smoke-core task should record those
regressions as not applicable with the untouched-path rationale. The normal
implementation task still must run cargo fmt, cargo -Zjson-target-spec test,
git diff --check, mdbook build, and git diff --cached --check before commit.

## Next Task

If this plan is accepted and committed, the next mechanically bounded task is
phase8-kernel-half-reachability-core-20260531. That task may implement only the
target-independent preflight boundary selected here and by the contract. It
must not acquire hardwareTestLock, publish a boot archive, run Pi 5 hardware,
construct or install a live kernel-half descriptor image, mutate live
translation registers, launch lower EL, publish scheduler runnable state,
expand process lifecycle semantics, add shell behavior, or accept
descriptor-backed filesystem syscalls.

## Reviewed Inputs

- docs/src/project/phase8-kernel-half-reachability-contract.md
- docs/src/project/phase8-kernel-half-reachability-source-inventory.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-initial-process-launch-contract.md
- docs/src/project/phase8-initial-user-stack-contract.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation review: inspected the accepted kernel-half reachability
  contract and source inventory, adjacent Phase 8 contracts, existing
  QEMU/substitute smoke-plan pattern, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this plan.
