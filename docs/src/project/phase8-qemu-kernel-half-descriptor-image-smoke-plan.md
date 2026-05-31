# Phase 8 QEMU Kernel-Half Descriptor-Image Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute
kernel-half descriptor-image smoke plan after the accepted
[Phase 8 Kernel-Half Descriptor-Image Contract](phase8-kernel-half-descriptor-image-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition, live
TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLBI, DSB, or ISB
mutation, lower-EL ERET, scheduler runnable publication, process lifecycle,
shell behavior, descriptor-backed filesystem syscalls, writable filesystem,
persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The purpose of this plan is to make the next implementation task mechanical:
construct one inspectable, non-installed KernelHalfDescriptorImage from the
accepted KernelHalfReachabilityPlan and copied Phase 8 input lineage, then
retain one QEMU/substitute smoke log proving descriptor-image construction
records, deterministic rejection, no-partial rollback, idempotent teardown,
and zero live translation side effects.

## Smoke Invariant

The first QEMU/substitute kernel-half descriptor-image smoke must demonstrate
one bounded invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_kernel_half_descriptor_image_smoke.
2. The scenario consumes the accepted ProgramImagePlan,
   ProcessImageInstallPlan, ProcessAddressSpace,
   ProcessPageTableMaterialization, InitialProcessLaunchPlan,
   InitialUserStackPlan, LiveAddressSpaceActivationPlan, and
   KernelHalfReachabilityPlan records for immutable /bin/init.
3. The fixture identity line records loader identity
   phase8-program-loader-elf64-aarch64-v1, process-install boundary
   phase8-process-install-plan-v1, address-space boundary
   phase8-process-address-space-model-v1, materialization boundary
   phase8-process-page-table-materialization-v1, launch boundary
   phase8-initial-process-launch-plan-v1, stack boundary
   phase8-initial-user-stack-plan-v1, activation boundary
   phase8-live-address-space-activation-plan-v1, reachability boundary
   phase8-kernel-half-reachability-plan-v1, descriptor-image boundary
   phase8-kernel-half-descriptor-image-v1, and descriptor-image policy
   ttbr1-shared-privileged-kernel-root-descriptor-image-v1.
4. The success path creates exactly one non-installed
   KernelHalfDescriptorImage with copied accepted input lineage, a
   model-owned TTBR1 kernel-root image intent, root/table lease records,
   descriptor records, coverage records, and rollback/teardown state.
5. The image records TTBR0_EL1 intent as provenance-only from the accepted
   ProcessPageTableMaterialization and must report no TTBR0_EL1 write.
6. The image records TTBR1_EL1 intent as a shared privileged kernel-root image
   intent and must report no TTBR1_EL1 write or descriptor installation into
   a live root.
7. Kernel text, rodata, data, bss, vectors, active stack, heap, page-frame
   metadata, UART/MMIO diagnostics, scheduler code/data, runtime console, and
   panic/fault reporting are all required coverage entries.
8. Descriptor attributes preserve privileged executable text/vectors,
   privileged read-only rodata, privileged non-executable writable data,
   privileged device MMIO diagnostics, EL0 access denied, W+X denied, and
   device-vs-normal memory attributes separated.
9. TCR_EL1 and MAIR_EL1 observations are compatibility-record-only.
10. SCTLR_EL1, ASID, TLB, DSB, and ISB observations remain blocked from live
    mutation.
11. Side-effect records show no TTBR/TCR/MAIR/SCTLR writes, no kernel-half
    descriptor image installed, no ASID allocation, no live TLB invalidation,
    no live DSB/ISB activation sequence, no lower-EL ERET, no scheduler
    publication, no process-table mutation, and no descriptor-table mutation.
12. Negative cases prove deterministic rejection for bad reachability input,
    lineage mismatch, missing kernel coverage, forbidden EL0 access, writable
    text, executable data, bad device attribute intent, overlapping range,
    resource exhaustion, unsupported topology, and live activation request.
13. Failure cases must prove no partial descriptor image: no visible
    KernelHalfDescriptorImage after validation failure, no leaked root/table
    lease, no installed descriptor, no live register intent becomes live, and
    no scheduler, process, descriptor-table, or filesystem state changes.
14. Teardown observations prove that a published model image clears descriptor
    records, releases model-owned root/table leases, marks the image
    unpublished, preserves accepted input ownership, and reports
    already-destroyed on a second teardown.
15. The smoke prints final classification and PASS only after success,
    rejection, teardown, no-partial, and zero-side-effect observations have
    been recorded.

If implementation work needs a different scenario name, evidence path,
boundary identity, policy identity, PASS/classification vocabulary, live
descriptor-image installation, hardware involvement, live TTBR/TCR/MAIR/SCTLR
mutation, ASID/TLB side effect, lower-EL launch semantics, scheduler
publication, process lifecycle behavior, shell behavior, or filesystem syscall
expansion, it must stop for supervisor planning instead of accepting a changed
smoke.

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

The accepted kernel-half descriptor-image boundary for this smoke is:

    phase8-kernel-half-descriptor-image-v1

The accepted kernel-half descriptor-image policy for this smoke is:

    ttbr1-shared-privileged-kernel-root-descriptor-image-v1

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
| descriptor-image policy | ttbr1-shared-privileged-kernel-root-descriptor-image-v1 |
| TTBR0_EL1 intent | materialized process root provenance only; no write |
| TTBR1_EL1 intent | non-installed shared privileged kernel-root descriptor image; no write |
| kernel coverage | text, rodata, data, bss, vectors, active stack, heap, page frames, UART/MMIO, scheduler, runtime console, panic/fault reporting |
| descriptor attributes | privileged-only normal/device policy, EL0 denied, W+X denied |
| root/table ownership | model-owned leases, no live-table borrowing |
| activation | descriptor-image-ready record only; no live activation |
| hardware side effects | none |

The fixture must not claim that /bin/init can run and must not claim that a
kernel-half image is live. Live TTBR0_EL1/TTBR1_EL1 programming,
TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, TLB invalidation,
DSB/ISB live activation sequencing, lower-EL ERET, scheduler handoff, process
lifecycle, descriptor-backed filesystem syscalls, shell behavior, and Pi 5
hardware proof remain outside this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-kernel-half-descriptor-image-smoke: final participants=17 expected=17 errors=0 classification=qemu-kernel-half-descriptor-image-smoke-complete
    qemu-kernel-half-descriptor-image-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-kernel-half-descriptor-image-smoke: start
    qemu-kernel-half-descriptor-image-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-boundary=phase8-live-address-space-activation-plan-v1 reachability-boundary=phase8-kernel-half-reachability-plan-v1 descriptor-image-boundary=phase8-kernel-half-descriptor-image-v1 descriptor-image-policy=ttbr1-shared-privileged-kernel-root-descriptor-image-v1
    qemu-kernel-half-descriptor-image-smoke: success output=KernelHalfDescriptorImage published=true installed=false copied-identities=true descriptor-image-boundary=phase8-kernel-half-descriptor-image-v1 descriptor-image-policy=ttbr1-shared-privileged-kernel-root-descriptor-image-v1 ok=true
    qemu-kernel-half-descriptor-image-smoke: root-policy ttbr0-root=materialized-process-root-provenance ttbr0-written=false ttbr1-root=owned-kernel-root-image ttbr1-written=false descriptor-image-installed=false ok=true
    qemu-kernel-half-descriptor-image-smoke: coverage kernel-text=true rodata=true data=true bss=true vectors=true active-stack=true heap=true page-frames=true uart-mmio-diagnostics=true scheduler-code-data=true runtime-console=true panic-fault-reporting=true ok=true
    qemu-kernel-half-descriptor-image-smoke: permissions text-exec=privileged-only rodata-write=false data-exec=false device-normal-memory=false el0-kernel-access=false wx-normal-memory=false ok=true
    qemu-kernel-half-descriptor-image-smoke: attributes normal-memory=inner-shareable device-memory=device-nGnRE af=true user-access=denied exact-coverage=true ok=true
    qemu-kernel-half-descriptor-image-smoke: ownership root-lease=model-owned table-leases=model-owned live-table-borrowed=false input-records-owned=true rollback-ready=true ok=true
    qemu-kernel-half-descriptor-image-smoke: compatibility tcr-state=split-compatibility-record-only mair-state=normal-device-compatibility-record-only sctlr-state=mutation-blocked ok=true
    qemu-kernel-half-descriptor-image-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb live-register-sequence=blocked-no-live-register-sequence lower-el-eret=false scheduler-publication=false ok=true
    qemu-kernel-half-descriptor-image-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false descriptor-image-installed=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true
    qemu-kernel-half-descriptor-image-smoke: teardown phase=first descriptors-cleared=true root-released=true tables-released=true published=false input-records-owned=true already-destroyed=false ok=true
    qemu-kernel-half-descriptor-image-smoke: teardown phase=second descriptors-cleared=false root-released=false tables-released=false published=false already-destroyed=true ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=bad-reachability-plan errno=-EINVAL partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=lineage-mismatch errno=-EINVAL partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=missing-kernel-coverage errno=-EINVAL partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=forbidden-el0-access errno=-EACCES partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=writable-text errno=-EACCES partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=executable-data errno=-EACCES partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=bad-device-attribute-intent errno=-EACCES partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=overlapping-range errno=-EINVAL partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=resource-exhaustion errno=-ENOMEM partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=unsupported-topology errno=-ENOTSUP partial-image=false leaked-leases=false ok=true
    qemu-kernel-half-descriptor-image-smoke: error case=live-activation-request errno=-ENOSYS partial-image=false leaked-leases=false ok=true

The implementation may print additional descriptor indices, table topology,
range digests, lease identifiers, rollback records, or manifest fields. The
required line shapes must stay stable enough for the script gate. Hex values,
lease identifiers, counts, and descriptor totals are field placeholders
because the later implementation task owns the exact model representation.

## Failure Classification

The smoke must distinguish descriptor-image contract failures from scenario
wiring failures:

- Contract failure: the builder loses accepted lineage, omits required kernel
  coverage, borrows a live table, widens permissions, permits EL0 kernel
  access, creates writable text or executable data, maps device MMIO as normal
  memory, publishes before every record is complete, reports the wrong errno,
  leaks a root/table lease after rejection, leaves a partial image visible,
  mutates live TTBR/TCR/MAIR/SCTLR/TLB/barrier/scheduler/process state, or
  double-releases during teardown.
- Scenario wiring failure: the scenario cannot select
  qemu_kernel_half_descriptor_image_smoke, cannot obtain the accepted input
  lineage through KernelHalfReachabilityPlan, cannot retain a fresh log,
  cannot print the fixture identity line, or cannot drive success, negative,
  no-partial, teardown, and zero-side-effect observations in order.
- Regression failure: an accepted loader, process-install, address-space,
  materialization, launch, stack, live-activation, kernel-half reachability,
  or related Phase 8 gate required by this plan fails after implementation
  changes touch shared owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_kernel_half_descriptor_image_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains
   qemu-kernel-half-descriptor-image-smoke: start before looking for PASS.
4. Confirm the fixture identity line appears before descriptor-image
   observations.
5. Confirm success coverage, permission, ownership, side-effect, and teardown
   lines appear before negative errno lines.
6. Confirm every negative case reports partial-image=false and
   leaked-leases=false.
7. Confirm teardown lines show first-release and second already-destroyed
   behavior.
8. Confirm the side-effect line reports ttbr-mutated=false,
   tcr-mutated=false, mair-mutated=false, sctlr-mutated=false,
   descriptor-image-installed=false, tlb-mutated=false,
   scheduler-published=false, process-table-mutated=false, and
   descriptor-table-mutated=false.
9. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
10. Rerun the smoke script once after cleaning only stale QEMU/substitute
    output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Conditional Regression Gates

The smoke-core task must run the accepted qemu-kernel-half-descriptor-image
smoke. It must also run adjacent QEMU/substitute smokes when implementation
changes touch shared owners:

- kernel-half reachability smoke if src/kernel_half_reachability.rs or its
  smoke fixture changes;
- live-address-space-activation smoke if src/live_address_space_activation.rs
  or activation-preflight vocabulary changes;
- initial-user-stack smoke if src/initial_user_stack.rs changes;
- initial-process-launch smoke if src/initial_process_launch.rs changes;
- process-page-table-materialization smoke if
  src/process_page_table_materialization.rs or page-table descriptor modeling
  changes;
- process-address-space smoke if src/process_address_space.rs changes;
- process-install smoke if src/process_install.rs changes; and
- program-loader smoke if src/program_loader.rs or loader fixture code
  changes.

If none of those owners change, the smoke-core task should record those
regressions as not applicable with the untouched-path rationale. The normal
implementation task still must run cargo fmt, cargo -Zjson-target-spec test,
git diff --check, mdbook build, and git diff --cached --check before commit.

## Next Task

If this plan is accepted and committed, the next mechanically bounded task is
phase8-kernel-half-descriptor-image-core-20260531. That task may implement
only the target-independent, non-installed descriptor-image construction
boundary selected here and by the contract. It must not acquire
hardwareTestLock, publish a boot archive, run Pi 5 hardware, install a live
kernel-half descriptor image, mutate live translation registers, allocate
ASIDs, mutate live TLB or barrier state, launch lower EL, publish scheduler
runnable state, expand process lifecycle semantics, add shell behavior, or
accept descriptor-backed filesystem syscalls.

## Reviewed Inputs

- docs/src/project/phase8-kernel-half-descriptor-image-contract.md
- docs/src/project/phase8-kernel-half-descriptor-image-source-inventory.md
- docs/src/project/phase8-kernel-half-reachability-contract.md
- docs/src/project/phase8-qemu-kernel-half-reachability-smoke-plan.md
- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-qemu-process-page-table-materialization-smoke-plan.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation review: inspected the accepted descriptor-image
  contract and source inventory, adjacent Phase 8 contracts, existing
  QEMU/substitute smoke-plan patterns, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this plan.
