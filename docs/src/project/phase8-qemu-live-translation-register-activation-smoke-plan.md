# Phase 8 QEMU Live Translation-Register Activation Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute live
translation-register activation smoke plan after the accepted
[Phase 8 Live Translation-Register Activation Contract](phase8-live-translation-register-activation-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition, live
TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, active-root
descriptor copy, ASID allocation, live TLB invalidation, activation DSB/ISB,
lower-EL ERET, scheduler runnable publication, process lifecycle, shell
behavior, descriptor-backed filesystem syscalls, writable filesystem,
persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The purpose of this plan is to make the next implementation task mechanical:
construct one inspectable model-level LiveTranslationRegisterActivation
record from the accepted KernelHalfDescriptorImageInstallation, then retain
one QEMU/substitute smoke log proving activation-commit intent, deterministic
rejection, no-partial-activation, rollback/teardown, preserved kernel
diagnostic reachability, and zero live translation side effects.

## Smoke Invariant

The first QEMU/substitute live translation-register activation smoke must
demonstrate one bounded invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_live_translation_register_activation_smoke.
2. The scenario consumes the accepted ProgramImagePlan,
   ProcessImageInstallPlan, ProcessAddressSpace,
   ProcessPageTableMaterialization, InitialProcessLaunchPlan,
   InitialUserStackPlan, LiveAddressSpaceActivationPlan,
   KernelHalfReachabilityPlan, KernelHalfDescriptorImage, and
   KernelHalfDescriptorImageInstallation records for immutable /bin/init.
3. The fixture identity line records loader identity
   phase8-program-loader-elf64-aarch64-v1, process-install boundary
   phase8-process-install-plan-v1, address-space boundary
   phase8-process-address-space-model-v1, materialization boundary
   phase8-process-page-table-materialization-v1, launch boundary
   phase8-initial-process-launch-plan-v1, stack boundary
   phase8-initial-user-stack-plan-v1, activation-plan boundary
   phase8-live-address-space-activation-plan-v1, reachability boundary
   phase8-kernel-half-reachability-plan-v1, descriptor-image boundary
   phase8-kernel-half-descriptor-image-v1, installation boundary
   phase8-live-descriptor-image-installation-v1, activation boundary
   phase8-live-translation-register-activation-v1, and activation policy
   model-ttbr0-ttbr1-activation-commit-below-live-registers-v1.
4. The success path creates exactly one model-level activation-commit intent
   with copied accepted input lineage, copied TTBR0 materialized-root
   provenance, copied TTBR1 descriptor-image kernel-root provenance, preserved
   TCR/MAIR compatibility records, blocked SCTLR/ASID/TLB/barrier states, and
   no live architectural activation.
5. The installation input state is published and not destroyed, remains below
   live register ownership, records descriptor-image-installed=false,
   ttbr0-written=false, ttbr1-written=false, sctlr-mutated=false, and
   active-root-copied=false before the model activation intent is created.
6. TTBR0_EL1 and TTBR1_EL1 observations are provenance-only and must report
   no register write.
7. TCR_EL1 and MAIR_EL1 observations remain compatibility-record-only.
8. SCTLR_EL1, ASID, TLB, DSB, and ISB observations remain mutation-blocked,
   blocked-no-asid-allocation, blocked-no-live-tlbi, and
   planned-only-no-live-dsb-isb.
9. Kernel text, rodata, data, bss, vectors, active stack, heap, allocator
   metadata, UART/MMIO diagnostics, scheduler code/data, runtime console, and
   panic/fault reporting remain reachable through kernel-owned prerequisites.
10. Side-effect records show no TTBR/TCR/MAIR/SCTLR writes, no active-root
    descriptor copy, no ASID allocation, no live TLB invalidation, no live
    DSB/ISB activation sequence, no lower-EL ERET, no scheduler publication,
    no process-table mutation, no descriptor-table publication, no filesystem
    syscall behavior, and no hardware action.
11. Negative cases prove deterministic rejection for missing or destroyed
    inputs, identity mismatch, lineage mismatch, stale root provenance,
    already-consumed installation, forbidden EL0 kernel access, diagnostic
    reachability loss, live-register request, active-root-copy request,
    lower-EL launch request, scheduler publication request, filesystem
    request, and resource exhaustion.
12. Failure cases must prove no partial activation: no visible activation
    intent after validation failure, no consumed bit set on the installation,
    no architectural register intent becomes live, and no scheduler, process,
    descriptor-table, filesystem, QEMU hardware, or Pi 5 hardware state
    changes.
13. Teardown observations prove that a published model activation clears only
    activation-record-local state, preserves input descriptor-image,
    installation, activation-plan, and materialized-root ownership, and
    reports already-destroyed on a second teardown.
14. The smoke prints final classification and PASS only after success,
    rejection, rollback/teardown, no-partial-activation, reachability, and
    zero-side-effect observations have been recorded.

If implementation work needs a different scenario name, evidence path,
boundary identity, policy identity, PASS/classification vocabulary, live
TTBR/TCR/MAIR/SCTLR mutation, active-root descriptor copy, ASID/TLB side
effect, barrier execution, lower-EL launch semantics, scheduler publication,
process lifecycle behavior, filesystem syscall expansion, boot archive
publication, or Pi 5 hardware involvement, it must stop for supervisor
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

The accepted kernel-half reachability boundary remains:

    phase8-kernel-half-reachability-plan-v1

The accepted kernel-half descriptor-image boundary remains:

    phase8-kernel-half-descriptor-image-v1

The accepted live descriptor-image installation boundary remains:

    phase8-live-descriptor-image-installation-v1

The accepted live translation-register activation boundary for this smoke is:

    phase8-live-translation-register-activation-v1

The accepted activation policy for this smoke is:

    model-ttbr0-ttbr1-activation-commit-below-live-registers-v1

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
| activation-plan boundary | phase8-live-address-space-activation-plan-v1 |
| reachability boundary | phase8-kernel-half-reachability-plan-v1 |
| descriptor-image boundary | phase8-kernel-half-descriptor-image-v1 |
| installation boundary | phase8-live-descriptor-image-installation-v1 |
| activation boundary | phase8-live-translation-register-activation-v1 |
| activation policy | model-ttbr0-ttbr1-activation-commit-below-live-registers-v1 |
| TTBR0_EL1 intent | materialized process root provenance only; no write |
| TTBR1_EL1 intent | descriptor-image kernel root provenance only; no write |
| TCR_EL1 state | compatibility-record-only |
| MAIR_EL1 state | compatibility-record-only |
| SCTLR_EL1 state | mutation-blocked |
| ASID state | blocked-no-asid-allocation |
| TLB state | blocked-no-live-tlbi |
| barriers | planned-only-no-live-dsb-isb |
| active root | no-active-root-copy-or-mutation |
| fault reporting | kernel-owned-vector-diagnostics-preserved |
| activation state | model-only-activation-commit-intent |
| lower-EL launch | blocked-no-lower-el-eret |
| runnable publication | blocked-no-runnable-publication |
| hardware side effects | none |

The fixture must not claim that /bin/init can run and must not claim that a
translation-register sequence has executed. Live TTBR0_EL1/TTBR1_EL1
programming, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, TLB
invalidation, DSB/ISB live activation sequencing, active-root descriptor
copy, lower-EL ERET, scheduler handoff, process lifecycle, descriptor-backed
filesystem syscalls, shell behavior, and Pi 5 hardware proof remain outside
this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-31-qemu-live-translation-register-activation-smoke-core/qemu-live-translation-register-activation-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-live-translation-register-activation-smoke: final participants=17 expected=17 errors=0 classification=qemu-live-translation-register-activation-smoke-complete
    qemu-live-translation-register-activation-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-live-translation-register-activation-smoke: start
    qemu-live-translation-register-activation-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-plan-boundary=phase8-live-address-space-activation-plan-v1 reachability-boundary=phase8-kernel-half-reachability-plan-v1 descriptor-image-boundary=phase8-kernel-half-descriptor-image-v1 installation-boundary=phase8-live-descriptor-image-installation-v1 activation-boundary=phase8-live-translation-register-activation-v1 activation-policy=model-ttbr0-ttbr1-activation-commit-below-live-registers-v1
    qemu-live-translation-register-activation-smoke: success output=LiveTranslationRegisterActivation published=true copied-identities=true activation-boundary=phase8-live-translation-register-activation-v1 activation-policy=model-ttbr0-ttbr1-activation-commit-below-live-registers-v1 ok=true
    qemu-live-translation-register-activation-smoke: input-state installation-published=true installation-destroyed=false below-live-registers=true descriptor-image-installed=false ttbr0-written=false ttbr1-written=false sctlr-mutated=false active-root-copied=false ok=true
    qemu-live-translation-register-activation-smoke: ttbr-provenance ttbr0-root=materialized-process-root-provenance ttbr0-written=false ttbr1-root=descriptor-image-kernel-root-provenance ttbr1-written=false active-root-copied=false ok=true
    qemu-live-translation-register-activation-smoke: compatibility tcr-state=compatibility-record-only mair-state=compatibility-record-only sctlr-state=mutation-blocked ok=true
    qemu-live-translation-register-activation-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb live-register-sequence=blocked-no-live-register-sequence ok=true
    qemu-live-translation-register-activation-smoke: kernel-reachability vbar=true vectors=true active-stack=true kernel-text=true rodata=true data=true bss=true heap=true allocator=true uart-mmio-diagnostics=true scheduler-code-data=true runtime-console=true panic-fault-reporting=true ok=true
    qemu-live-translation-register-activation-smoke: activation-state previous=installation-ready-activation-binding next=model-only-activation-commit-intent lower-el-eret=false scheduler-published=false ok=true
    qemu-live-translation-register-activation-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false active-root-copied=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-published=false filesystem-mutated=false hardware-action=false ok=true
    qemu-live-translation-register-activation-smoke: teardown phase=first activation-cleared=true installation-input-owned=true descriptor-input-owned=true activation-plan-owned=true materialized-root-owned=true live-state-mutated=false already-destroyed=false ok=true
    qemu-live-translation-register-activation-smoke: teardown phase=second activation-cleared=false installation-input-owned=true descriptor-input-owned=true activation-plan-owned=true materialized-root-owned=true already-destroyed=true ok=true
    qemu-live-translation-register-activation-smoke: error case=missing-input errno=-EINVAL partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=destroyed-input errno=-EINVAL partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=identity-mismatch errno=-EINVAL partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=lineage-mismatch errno=-ENOEXEC partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=stale-root-provenance errno=-EBUSY partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=already-consumed-installation errno=-EBUSY partial-activation=false consumed=true live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=forbidden-el0-kernel-access errno=-EACCES partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=diagnostic-reachability-loss errno=-EACCES partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=live-register-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=active-root-copy-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=lower-el-launch-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=scheduler-publication-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=filesystem-request errno=-ENOSYS partial-activation=false consumed=false live-state-mutated=false ok=true
    qemu-live-translation-register-activation-smoke: error case=resource-exhaustion errno=-ENOMEM partial-activation=false consumed=false live-state-mutated=false ok=true

The implementation may print additional descriptor digests, lease tokens,
activation identifiers, copied compatibility fields, teardown records, or
diagnostic fields. The required line shapes must stay stable enough for the
script gate. Hex values, lease identifiers, counts, and descriptor totals are
field placeholders because the later implementation task owns the exact model
representation.

## Failure Classification

The smoke must distinguish activation contract failures from scenario wiring
failures:

- Contract failure: the activation model loses accepted lineage, accepts stale
  or destroyed input, consumes installation input on failure, widens kernel
  permissions, permits EL0 kernel access to kernel-half mappings, loses
  UART/MMIO or vector diagnostic reachability, reports TCR/MAIR/SCTLR as live
  rather than compatibility-only/blocked, allocates ASID state, mutates
  TLB/barrier state, copies descriptors into an active root, publishes
  scheduler/process/descriptor-table/filesystem state, reports the wrong
  errno, leaks a partial activation intent after rejection, or double-releases
  input ownership during teardown.
- Scenario wiring failure: the scenario cannot select
  qemu_live_translation_register_activation_smoke, cannot obtain the accepted
  KernelHalfDescriptorImageInstallation and copied Phase 8 lineage, cannot
  retain a fresh log, cannot print the fixture identity line, or cannot drive
  success, negative, no-partial, teardown, reachability, and zero-side-effect
  observations in order.
- Regression failure: an accepted loader, process-install, address-space,
  materialization, launch, stack, live-activation, kernel-half reachability,
  kernel-half descriptor-image, live descriptor-image installation, or related
  Phase 8 gate required by this plan fails after implementation changes touch
  shared owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_live_translation_register_activation_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains
   qemu-live-translation-register-activation-smoke: start before looking for
   PASS.
4. Confirm the fixture identity line appears before activation observations.
5. Confirm success, input-state, TTBR provenance, compatibility,
   blocked-state, reachability, activation-state, side-effect, and teardown
   lines appear before negative errno lines.
6. Confirm every negative case reports partial-activation=false and
   live-state-mutated=false.
7. Confirm the side-effect line reports no TTBR/TCR/MAIR/SCTLR, active-root,
   ASID, TLB, live DSB/ISB, lower-EL, scheduler, process-table,
   descriptor-table, filesystem, or hardware mutation.
8. Confirm teardown releases only activation-record-local ownership and leaves
   accepted installation, descriptor-image, activation-plan, and
   materialized-root ownership intact.
9. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
10. Rerun the smoke script once after cleaning only stale QEMU/substitute
    output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Conditional Regression Gates

The smoke-core task must run the accepted qemu-live-translation-register
activation smoke. It must also run adjacent QEMU/substitute smokes when
implementation changes touch shared owners:

- live descriptor-image installation smoke if
  src/live_descriptor_image_installation.rs or installation vocabulary
  changes;
- kernel-half descriptor-image smoke if src/kernel_half_descriptor_image.rs or
  its smoke fixture changes;
- live-address-space-activation smoke if src/live_address_space_activation.rs
  or activation-preflight vocabulary changes;
- kernel-half reachability smoke if src/kernel_half_reachability.rs changes;
- process-page-table-materialization smoke if
  src/process_page_table_materialization.rs or page-table descriptor modeling
  changes;
- initial-user-stack smoke if src/initial_user_stack.rs changes;
- initial-process-launch smoke if src/initial_process_launch.rs changes;
- process-address-space smoke if src/process_address_space.rs changes;
- process-install smoke if src/process_install.rs changes; and
- program-loader smoke if src/program_loader.rs or loader fixture code
  changes.

If none of those owners change, the smoke-core task should record those
regressions as not applicable with the untouched-path rationale. The normal
implementation task still must run cargo fmt, cargo -Zjson-target-spec test,
git diff --check, mdbook build if docs are touched, and git diff --cached
--check before commit.

## Next Task

If this plan is accepted and committed, the next mechanically bounded task is
phase8-live-translation-register-activation-core-20260531. That task may
implement only the target-independent, model-level activation-commit intent
selected here and by the contract. It must not acquire hardwareTestLock,
publish a boot archive, run Pi 5 hardware, execute a live architectural
translation-register sequence, copy descriptors into an active root, allocate
ASIDs, mutate live TLB or barrier state, launch lower EL, publish scheduler
runnable state, expand process lifecycle semantics, add shell behavior, or
accept descriptor-backed filesystem syscalls.

## Reviewed Inputs

- docs/src/project/phase8-live-translation-register-activation-contract.md
- docs/src/project/phase8-live-translation-register-activation-source-inventory.md
- docs/src/project/phase8-live-descriptor-image-installation-contract.md
- docs/src/project/phase8-qemu-live-descriptor-image-installation-smoke-plan.md
- docs/src/project/phase8-live-descriptor-image-installation-closeout-checkpoint.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md
- docs/src/project/phase8-kernel-half-descriptor-image-contract.md
- docs/src/project/phase8-qemu-kernel-half-descriptor-image-smoke-plan.md
- docs/src/project/phase8-kernel-half-reachability-contract.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean except durable
  supervisor state promotion outside the Talos repo.
- static documentation review: inspected the accepted live
  translation-register activation contract/source inventory, adjacent Phase 8
  contracts and smoke-plan patterns, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this plan.
