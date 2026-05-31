# Phase 8 QEMU Live Descriptor-Image Installation Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute
live descriptor-image installation smoke plan after the accepted
[Phase 8 Live Descriptor-Image Installation Contract](phase8-live-descriptor-image-installation-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition, live
TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLBI, DSB, or ISB
mutation, lower-EL ERET, scheduler runnable publication, process lifecycle,
shell behavior, descriptor-backed filesystem syscalls, writable filesystem,
persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The purpose of this plan is to make the next implementation task mechanical:
construct one inspectable, model-level descriptor-image installation binding
between the accepted KernelHalfDescriptorImage and
LiveAddressSpaceActivationPlan, then retain one QEMU/substitute smoke log
proving installation-ready binding records, deterministic rejection,
no-partial-install rollback, idempotent teardown, and zero live translation
side effects.

## Smoke Invariant

The first QEMU/substitute live descriptor-image installation smoke must
demonstrate one bounded invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_live_descriptor_image_installation_smoke.
2. The scenario consumes the accepted ProgramImagePlan,
   ProcessImageInstallPlan, ProcessAddressSpace,
   ProcessPageTableMaterialization, InitialProcessLaunchPlan,
   InitialUserStackPlan, LiveAddressSpaceActivationPlan,
   KernelHalfReachabilityPlan, and KernelHalfDescriptorImage records for
   immutable /bin/init.
3. The fixture identity line records loader identity
   phase8-program-loader-elf64-aarch64-v1, process-install boundary
   phase8-process-install-plan-v1, address-space boundary
   phase8-process-address-space-model-v1, materialization boundary
   phase8-process-page-table-materialization-v1, launch boundary
   phase8-initial-process-launch-plan-v1, stack boundary
   phase8-initial-user-stack-plan-v1, activation boundary
   phase8-live-address-space-activation-plan-v1, reachability boundary
   phase8-kernel-half-reachability-plan-v1, descriptor-image boundary
   phase8-kernel-half-descriptor-image-v1, installation boundary
   phase8-live-descriptor-image-installation-v1, and installation policy
   model-installed-ttbr1-descriptor-image-below-live-registers-v1.
4. The success path creates exactly one model-level installation-ready
   activation binding with copied accepted input lineage, copied TTBR0
   materialized-root provenance, copied TTBR1 descriptor-image root
   provenance, preserved kernel-half coverage and permissions, rollback/
   teardown state, and no live architectural installation.
5. The binding records descriptor-image input state as published=true,
   installed=false, descriptor_image_installed=false, and ttbr1-written=false
   before the model binding is created.
6. The binding records TTBR0_EL1 and TTBR1_EL1 as provenance-only and must
   report no TTBR0_EL1 or TTBR1_EL1 write.
7. Kernel text, rodata, data, bss, vectors, active stack, heap, page-frame
   metadata, UART/MMIO diagnostics, scheduler code/data, runtime console, and
   panic/fault reporting are preserved from the accepted descriptor image.
8. Descriptor attributes preserve privileged executable text/vectors,
   privileged read-only rodata, privileged non-executable writable data,
   privileged device MMIO diagnostics, EL0 access denied, W+X denied, and
   device-vs-normal memory attributes separated.
9. TCR_EL1 and MAIR_EL1 observations remain compatibility-record-only, and
   SCTLR_EL1, ASID, TLB, DSB, and ISB observations remain blocked from live
   mutation.
10. Side-effect records show no TTBR/TCR/MAIR/SCTLR writes, no active-root
    descriptor copy, no descriptor-table publication, no ASID allocation, no
    live TLB invalidation, no live DSB/ISB activation sequence, no lower-EL
    ERET, no scheduler publication, no process-table mutation, no filesystem
    syscall behavior, and no hardware action.
11. Negative cases prove deterministic rejection for missing or stale inputs,
    destroyed inputs, identity mismatch, lineage mismatch, already-installed
    input, forbidden EL0 kernel access, missing diagnostic reachability,
    resource exhaustion, and live register request.
12. Failure cases must prove no partial installation: no visible installed
    binding after validation failure, no installed bit set on the descriptor
    image, no live register intent becomes live, and no scheduler, process,
    descriptor-table, filesystem, QEMU hardware, or Pi 5 hardware state
    changes.
13. Teardown observations prove that a published model binding clears only
    installation-record-local state, preserves input descriptor-image and
    activation ownership, and reports already-destroyed on a second teardown.
14. The smoke prints final classification and PASS only after success,
    rejection, teardown, no-partial, and zero-side-effect observations have
    been recorded.

If implementation work needs a different scenario name, evidence path,
boundary identity, policy identity, PASS/classification vocabulary, live
TTBR/TCR/MAIR/SCTLR mutation, active-root descriptor copy, ASID/TLB side
effect, lower-EL launch semantics, scheduler publication, process lifecycle
behavior, shell behavior, filesystem syscall expansion, boot archive
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

The accepted live descriptor-image installation boundary for this smoke is:

    phase8-live-descriptor-image-installation-v1

The accepted installation policy for this smoke is:

    model-installed-ttbr1-descriptor-image-below-live-registers-v1

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
| reachability boundary | phase8-kernel-half-reachability-plan-v1 |
| descriptor-image boundary | phase8-kernel-half-descriptor-image-v1 |
| installation boundary | phase8-live-descriptor-image-installation-v1 |
| installation policy | model-installed-ttbr1-descriptor-image-below-live-registers-v1 |
| TTBR0_EL1 intent | materialized process root provenance only; no write |
| TTBR1_EL1 intent | installed binding to descriptor-image root provenance only; no write |
| installation state | installation-ready-activation-binding |
| kernel coverage | text, rodata, data, bss, vectors, active stack, heap, page frames, UART/MMIO, scheduler, runtime console, panic/fault reporting |
| descriptor attributes | privileged-only normal/device policy, EL0 denied, W+X denied |
| root/table ownership | input descriptor image and activation plan preserved |
| activation | model binding only; no live activation |
| hardware side effects | none |

The fixture must not claim that /bin/init can run and must not claim that a
kernel-half image is live. Live TTBR0_EL1/TTBR1_EL1 programming,
TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, TLB invalidation,
DSB/ISB live activation sequencing, lower-EL ERET, scheduler handoff, process
lifecycle, descriptor-backed filesystem syscalls, shell behavior, and Pi 5
hardware proof remain outside this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-live-descriptor-image-installation-smoke: final participants=15 expected=15 errors=0 classification=qemu-live-descriptor-image-installation-smoke-complete
    qemu-live-descriptor-image-installation-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-live-descriptor-image-installation-smoke: start
    qemu-live-descriptor-image-installation-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-boundary=phase8-live-address-space-activation-plan-v1 reachability-boundary=phase8-kernel-half-reachability-plan-v1 descriptor-image-boundary=phase8-kernel-half-descriptor-image-v1 installation-boundary=phase8-live-descriptor-image-installation-v1 installation-policy=model-installed-ttbr1-descriptor-image-below-live-registers-v1
    qemu-live-descriptor-image-installation-smoke: success output=KernelHalfDescriptorImageInstallation published=true copied-identities=true installation-boundary=phase8-live-descriptor-image-installation-v1 installation-policy=model-installed-ttbr1-descriptor-image-below-live-registers-v1 ok=true
    qemu-live-descriptor-image-installation-smoke: input-state descriptor-published=true descriptor-installed=false descriptor-image-installed=false ttbr1-written=false activation-published=true activation-model-only=true ok=true
    qemu-live-descriptor-image-installation-smoke: ttbr-provenance ttbr0-root=materialized-process-root-provenance ttbr0-written=false ttbr1-root=descriptor-image-kernel-root-provenance ttbr1-written=false active-root-copied=false ok=true
    qemu-live-descriptor-image-installation-smoke: coverage kernel-text=true rodata=true data=true bss=true vectors=true active-stack=true heap=true page-frames=true uart-mmio-diagnostics=true scheduler-code-data=true runtime-console=true panic-fault-reporting=true ok=true
    qemu-live-descriptor-image-installation-smoke: permissions text-exec=privileged-only rodata-write=false data-exec=false device-normal-memory=false el0-kernel-access=false wx-normal-memory=false ok=true
    qemu-live-descriptor-image-installation-smoke: installation-state previous=non-installed-descriptor-image next=installation-ready-activation-binding live-register-state=blocked-no-live-register-sequence ok=true
    qemu-live-descriptor-image-installation-smoke: compatibility tcr-state=compatibility-record-only mair-state=compatibility-record-only sctlr-state=mutation-blocked ok=true
    qemu-live-descriptor-image-installation-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb lower-el-eret=false scheduler-publication=false filesystem-syscalls=false ok=true
    qemu-live-descriptor-image-installation-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false active-root-copied=false descriptor-table-published=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false filesystem-mutated=false hardware-action=false ok=true
    qemu-live-descriptor-image-installation-smoke: teardown phase=first installation-cleared=true descriptor-input-owned=true activation-input-owned=true live-state-mutated=false already-destroyed=false ok=true
    qemu-live-descriptor-image-installation-smoke: teardown phase=second installation-cleared=false descriptor-input-owned=true activation-input-owned=true already-destroyed=true ok=true
    qemu-live-descriptor-image-installation-smoke: error case=missing-input errno=-EINVAL partial-installation=false descriptor-installed=false live-state-mutated=false ok=true
    qemu-live-descriptor-image-installation-smoke: error case=destroyed-input errno=-EINVAL partial-installation=false descriptor-installed=false live-state-mutated=false ok=true
    qemu-live-descriptor-image-installation-smoke: error case=identity-mismatch errno=-EINVAL partial-installation=false descriptor-installed=false live-state-mutated=false ok=true
    qemu-live-descriptor-image-installation-smoke: error case=lineage-mismatch errno=-ENOEXEC partial-installation=false descriptor-installed=false live-state-mutated=false ok=true
    qemu-live-descriptor-image-installation-smoke: error case=already-installed-input errno=-EBUSY partial-installation=false descriptor-installed=true live-state-mutated=false ok=true
    qemu-live-descriptor-image-installation-smoke: error case=forbidden-el0-access errno=-EACCES partial-installation=false descriptor-installed=false live-state-mutated=false ok=true
    qemu-live-descriptor-image-installation-smoke: error case=diagnostic-reachability-loss errno=-EACCES partial-installation=false descriptor-installed=false live-state-mutated=false ok=true
    qemu-live-descriptor-image-installation-smoke: error case=resource-exhaustion errno=-ENOMEM partial-installation=false descriptor-installed=false live-state-mutated=false ok=true
    qemu-live-descriptor-image-installation-smoke: error case=live-register-request errno=-ENOSYS partial-installation=false descriptor-installed=false live-state-mutated=false ok=true

The implementation may print additional descriptor indices, table topology,
range digests, lease identifiers, rollback records, or manifest fields. The
required line shapes must stay stable enough for the script gate. Hex values,
lease identifiers, counts, and descriptor totals are field placeholders
because the later implementation task owns the exact model representation.

## Failure Classification

The smoke must distinguish installation contract failures from scenario
wiring failures:

- Contract failure: the installer loses accepted lineage, accepts stale input,
  marks a descriptor image installed on failure, widens kernel permissions,
  permits EL0 kernel access, loses UART/MMIO or vector diagnostic
  reachability, changes TCR/MAIR/SCTLR/ASID/TLB/barrier state from blocked to
  live, copies descriptors into an active root, publishes descriptor-table or
  scheduler/process/filesystem state, reports the wrong errno, leaks a partial
  installation binding after rejection, or double-releases during teardown.
- Scenario wiring failure: the scenario cannot select
  qemu_live_descriptor_image_installation_smoke, cannot obtain the accepted
  input lineage through KernelHalfDescriptorImage and
  LiveAddressSpaceActivationPlan, cannot retain a fresh log, cannot print the
  fixture identity line, or cannot drive success, negative, no-partial,
  teardown, and zero-side-effect observations in order.
- Regression failure: an accepted loader, process-install, address-space,
  materialization, launch, stack, live-activation, kernel-half reachability,
  kernel-half descriptor-image, or related Phase 8 gate required by this plan
  fails after implementation changes touch shared owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_live_descriptor_image_installation_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains
   qemu-live-descriptor-image-installation-smoke: start before looking for
   PASS.
4. Confirm the fixture identity line appears before installation observations.
5. Confirm input-state, TTBR provenance, coverage, permission, installation,
   side-effect, and teardown lines appear before negative errno lines.
6. Confirm every negative case reports partial-installation=false and
   live-state-mutated=false.
7. Confirm teardown lines show first-release and second already-destroyed
   behavior.
8. Confirm the side-effect line reports ttbr-mutated=false,
   tcr-mutated=false, mair-mutated=false, sctlr-mutated=false,
   active-root-copied=false, descriptor-table-published=false,
   scheduler-published=false, process-table-mutated=false,
   filesystem-mutated=false, and hardware-action=false.
9. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
10. Rerun the smoke script once after cleaning only stale QEMU/substitute
    output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Conditional Regression Gates

The smoke-core task must run the accepted qemu-live-descriptor-image
installation smoke. It must also run adjacent QEMU/substitute smokes when
implementation changes touch shared owners:

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
phase8-live-descriptor-image-installation-core-20260531. That task may
implement only the target-independent, model-level installation binding
selected here and by the contract. It must not acquire hardwareTestLock,
publish a boot archive, run Pi 5 hardware, install a live kernel-half
descriptor image, mutate live translation registers, copy descriptors into an
active root, allocate ASIDs, mutate live TLB or barrier state, launch lower
EL, publish scheduler runnable state, expand process lifecycle semantics, add
shell behavior, or accept descriptor-backed filesystem syscalls.

## Reviewed Inputs

- docs/src/project/phase8-live-descriptor-image-installation-contract.md
- docs/src/project/phase8-live-descriptor-image-installation-source-inventory.md
- docs/src/project/phase8-kernel-half-descriptor-image-contract.md
- docs/src/project/phase8-qemu-kernel-half-descriptor-image-smoke-plan.md
- docs/src/project/phase8-kernel-half-descriptor-image-closeout-checkpoint.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md
- docs/src/project/phase8-kernel-half-reachability-contract.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean except durable
  supervisor state promotion outside the Talos repo.
- static documentation review: inspected the accepted live descriptor-image
  installation contract/source inventory, adjacent Phase 8 contracts and
  smoke-plan patterns, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this plan.
