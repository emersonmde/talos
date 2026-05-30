# Phase 8 QEMU Live Address-Space Activation Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute live
address-space activation smoke plan after the accepted
[Phase 8 Live Address-Space Activation Contract](phase8-live-address-space-activation-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition, live
TTBR/TCR/MAIR/SCTLR mutation, ASID allocation, live TLB invalidation,
lower-EL ERET, scheduler runnable publication, process lifecycle, shell
behavior, descriptor-backed filesystem syscalls, writable filesystem,
persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The purpose of this plan is to make the next implementation task mechanical:
construct an inspectable LiveAddressSpaceActivationPlan from the accepted
loader, install, address-space, materialization, initial process launch, and
initial user stack records, then retain one QEMU/substitute smoke log proving
activation-preflight success, deterministic rejection, no-partial-activation,
no-runnable-publication, teardown, and zero live translation side effects.

## Smoke Invariant

The first QEMU/substitute live-address-space activation smoke must demonstrate
one bounded invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_live_address_space_activation_smoke.
2. The scenario consumes the accepted ProgramImagePlan,
   ProcessImageInstallPlan, ProcessAddressSpace,
   ProcessPageTableMaterialization, InitialProcessLaunchPlan, and
   InitialUserStackPlan records for immutable /bin/init.
3. The fixture identity line records loader identity
   phase8-program-loader-elf64-aarch64-v1, process-install boundary
   phase8-process-install-plan-v1, address-space boundary
   phase8-process-address-space-model-v1, materialization boundary
   phase8-process-page-table-materialization-v1, launch boundary
   phase8-initial-process-launch-plan-v1, stack boundary
   phase8-initial-user-stack-plan-v1, activation boundary
   phase8-live-address-space-activation-plan-v1, and activation policy
   preflight-split-user-ttbr0-kernel-reachability-blocked-v1.
4. The success path creates exactly one LiveAddressSpaceActivationPlan with
   copied image, install, address-space, materialization, launch, and stack
   identities.
5. The plan records TTBR0_EL1 intent as the materialized process root lease
   only and must report no TTBR0_EL1 write.
6. The plan records TTBR1_EL1/kernel-half policy as
   blocked-no-accepted-kernel-half-map.
7. TCR_EL1 and MAIR_EL1 observations are compatibility-record-only, and
   SCTLR_EL1 state is mutation-blocked.
8. ASID, TLB, DSB, and ISB observations remain blocked-no-asid-allocation,
   blocked-no-live-tlbi, and planned-only-no-live-dsb-isb.
9. The kernel-reachability checklist records VBAR_EL1, exception vectors,
   active kernel stack, kernel text/data, allocator state, UART/MMIO
   diagnostics, scheduler code/data, and panic/fault reporting as required
   prerequisites before live activation.
10. Launch binding changes only model state:
    blocked-no-ttbr-activation becomes
    model-only-activation-preflight-ready.
11. Side-effect records show no TTBR/TCR/MAIR/SCTLR writes, no ASID
    allocation, no live TLB invalidation, no live DSB/ISB activation
    sequence, no lower-EL ERET, no scheduler publication, no process-table
    mutation, and no descriptor-table mutation.
12. Negative cases prove deterministic rejection for identity mismatch,
    entry/stack/descriptor disagreement, forbidden kernel/device/user-range
    requests, missing kernel-reachability prerequisites, live-register
    requests, scheduler publication requests, lower-EL launch requests, and
    deterministic resource exhaustion.
13. Failure cases must prove no partial activation: no activation plan after
    validation failure, materialization remains activation_blocked=true,
    launch remains blocked-no-ttbr-activation, stack remains model-only,
    no architectural register intent becomes live, no scheduler or process
    state is mutated, and no descriptor-table or filesystem state changes.
14. Teardown observations prove that a published preflight record releases
    only plan-local model ownership and leaves the accepted materialization,
    launch, stack, loader, and install records owned by their existing
    teardown paths.
15. The smoke prints final classification and PASS only after success,
    rejection, teardown, no-partial-activation, no-runnable-publication, and
    zero-side-effect observations have been recorded.

If implementation work needs a different scenario name, evidence path,
boundary identity, activation policy, PASS/classification vocabulary, hardware
involvement, live TTBR/TCR/MAIR/SCTLR mutation, ASID/TLB side effect,
lower-EL launch semantics, scheduler publication, or process lifecycle
behavior, it must stop for supervisor planning instead of accepting a changed
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

The accepted activation preflight boundary for this smoke is:

    phase8-live-address-space-activation-plan-v1

The accepted activation policy for this smoke is:

    preflight-split-user-ttbr0-kernel-reachability-blocked-v1

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
| activation policy | preflight-split-user-ttbr0-kernel-reachability-blocked-v1 |
| TTBR0_EL1 intent | materialized process root lease only; no write |
| TTBR1_EL1 policy | blocked-no-accepted-kernel-half-map |
| TCR_EL1 state | compatibility-record-only |
| MAIR_EL1 state | compatibility-record-only |
| SCTLR_EL1 state | mutation-blocked |
| ASID state | blocked-no-asid-allocation |
| TLB state | blocked-no-live-tlbi |
| barriers | planned-only-no-live-dsb-isb |
| launch binding | model-only-activation-preflight-ready |
| runnable publication | blocked-no-runnable-publication |
| hardware side effects | none |

The fixture must not claim that /bin/init can run. Live TTBR0_EL1/TTBR1_EL1
programming, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, TLB
invalidation, DSB/ISB live activation sequencing, lower-EL ERET, scheduler
handoff, process lifecycle, descriptor-backed filesystem syscalls, shell
behavior, and Pi 5 hardware proof remain outside this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-live-address-space-activation-smoke: final participants=15 expected=15 errors=0 classification=qemu-live-address-space-activation-smoke-complete
    qemu-live-address-space-activation-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-live-address-space-activation-smoke: start
    qemu-live-address-space-activation-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1 activation-boundary=phase8-live-address-space-activation-plan-v1 activation-policy=preflight-split-user-ttbr0-kernel-reachability-blocked-v1
    qemu-live-address-space-activation-smoke: success output=LiveAddressSpaceActivationPlan published=true copied-identities=true activation-boundary=phase8-live-address-space-activation-plan-v1 activation-policy=preflight-split-user-ttbr0-kernel-reachability-blocked-v1 ok=true
    qemu-live-address-space-activation-smoke: ttbr-provenance ttbr0-root=materialized-process-root-lease ttbr0-written=false ttbr1-policy=blocked-no-accepted-kernel-half-map ok=true
    qemu-live-address-space-activation-smoke: compatibility tcr-state=compatibility-record-only mair-state=compatibility-record-only sctlr-state=mutation-blocked ok=true
    qemu-live-address-space-activation-smoke: blocked-states asid=blocked-no-asid-allocation tlb=blocked-no-live-tlbi barriers=planned-only-no-live-dsb-isb live-register-sequence=blocked-no-live-register-sequence ok=true
    qemu-live-address-space-activation-smoke: kernel-reachability vbar=true vectors=true active-stack=true kernel-text-data=true allocator=true uart-mmio-diagnostics=true scheduler-code-data=true panic-fault-reporting=true ok=true
    qemu-live-address-space-activation-smoke: launch-binding previous=blocked-no-ttbr-activation next=model-only-activation-preflight-ready lower-el-eret=false scheduler-published=false ok=true
    qemu-live-address-space-activation-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false asid-allocated=false tlb-mutated=false live-dsb-isb=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true
    qemu-live-address-space-activation-smoke: teardown plan-local-released=true materialization-owned=true launch-owned=true stack-owned=true image-owned=true idempotent=true ok=true
    qemu-live-address-space-activation-smoke: error case=identity-mismatch errno=-EINVAL partial-activation=false runnable-published=false ok=true
    qemu-live-address-space-activation-smoke: error case=entry-stack-descriptor-disagreement errno=-ENOEXEC partial-activation=false runnable-published=false ok=true
    qemu-live-address-space-activation-smoke: error case=forbidden-range errno=-EACCES partial-activation=false runnable-published=false ok=true
    qemu-live-address-space-activation-smoke: error case=missing-kernel-reachability errno=-EINVAL partial-activation=false runnable-published=false ok=true
    qemu-live-address-space-activation-smoke: error case=live-register-request errno=-ENOSYS partial-activation=false runnable-published=false ok=true
    qemu-live-address-space-activation-smoke: error case=scheduler-publication-request errno=-ENOSYS partial-activation=false runnable-published=false ok=true
    qemu-live-address-space-activation-smoke: error case=lower-el-launch-request errno=-ENOSYS partial-activation=false runnable-published=false ok=true
    qemu-live-address-space-activation-smoke: error case=resource-exhaustion errno=-ENOMEM partial-activation=false runnable-published=false ok=true

The implementation may print additional descriptor digests, lease tokens,
preflight checklist details, teardown records, or diagnostic fields. The
required line shapes must stay stable enough for the script gate. Hex values
and lease identifiers are field placeholders because the later implementation
task owns the exact model representation.

## Failure Classification

The smoke must distinguish activation contract failures from scenario wiring
failures:

- Contract failure: the activation preflight accepts mismatched identities,
  loses TTBR0 root provenance, treats the blocked TTBR1/kernel-half policy as
  accepted, reports TCR/MAIR/SCTLR as live rather than compatibility-only,
  allocates ASID state, mutates TLB/barrier state, skips kernel-reachability
  prerequisites, publishes scheduler/process/descriptor state, reports the
  wrong errno, or leaves partial activation/runnable state visible after
  rejection.
- Scenario wiring failure: the scenario cannot select
  qemu_live_address_space_activation_smoke, cannot obtain the accepted
  ProgramImagePlan/ProcessImageInstallPlan/ProcessAddressSpace/
  ProcessPageTableMaterialization/InitialProcessLaunchPlan/InitialUserStackPlan
  chain, cannot retain a fresh log, cannot print the fixture identity line, or
  cannot drive success, negative, no-partial, and teardown observations in
  order.
- Regression failure: an accepted loader, process-install, address-space,
  materialization, launch, stack, read-only initramfs/VFS, user-memory,
  descriptor/read, or lower-EL/syscall gate required by this plan fails after
  implementation changes touch shared owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_live_address_space_activation_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains qemu-live-address-space-activation-smoke: start
   before looking for PASS.
4. Confirm the fixture identity line appears before activation observations.
5. Confirm success, TTBR provenance, compatibility, blocked-state,
   reachability, launch-binding, side-effect, and teardown lines appear before
   negative errno lines.
6. Confirm every negative case reports partial-activation=false and
   runnable-published=false.
7. Confirm the side-effect line reports no TTBR/TCR/MAIR/SCTLR, ASID, TLB,
   live DSB/ISB, lower-EL, scheduler, process-table, or descriptor-table
   mutation.
8. Confirm teardown releases only plan-local ownership and leaves accepted
   materialization, launch, stack, and image ownership intact.
9. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
10. Rerun the smoke script once after cleaning only stale QEMU/substitute
    output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Regression Gates

The implementation task must retain:

- The QEMU/substitute live address-space activation smoke log named above.
- The command used to build and run
  qemu_live_address_space_activation_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- A planned qemu-live-address-space-activation-smoke script, or an accepted
  replacement script, that retains the required log and greps the required
  lines.
- scripts/qemu-initial-user-stack-smoke.sh if implementation changes
  InitialUserStackPlan generation, stack diagnostics, launch binding, teardown,
  or boot-scenario output owners used by that smoke.
- scripts/qemu-initial-process-launch-smoke.sh if implementation changes
  InitialProcessLaunchPlan generation, saved-frame intent, activation-state
  vocabulary, or boot-scenario output owners used by that smoke.
- scripts/qemu-process-page-table-materialization-smoke.sh if implementation
  changes ProcessPageTableMaterialization records, root/table/user-frame
  lease accounting, descriptor diagnostics, teardown, or activation-blocked
  output owners used by that smoke.
- scripts/qemu-process-address-space-smoke.sh if implementation changes
  ProcessAddressSpace model generation, mapping order, publication state,
  rollback state, or teardown behavior.
- scripts/qemu-process-install-smoke.sh if implementation changes
  ProcessImageInstallPlan generation, process-install diagnostics, or
  boot-scenario output owners used by that smoke.
- scripts/qemu-program-loader-smoke.sh if implementation changes
  ProgramImagePlan generation, /bin/init fixture bytes, loader diagnostics, or
  boot-scenario output owners used by that smoke.
- scripts/qemu-readonly-initramfs-vfs-smoke.sh if implementation changes the
  read-only initramfs/VFS fixture, lookup, regular-file read helpers,
  descriptor-facing fixture reads, or diagnostic output owners used by that
  smoke.
- Existing lower-EL/syscall, descriptor, read/stdin, and pointer-copy smokes
  only if implementation touches shared syscall dispatch, descriptor tables,
  user-copy helpers, lower-EL routing, boot-scenario routing, or diagnostic
  output owners used by those smokes.
- git diff --check.
- mdbook build.
- git diff --cached --check before commit.

If no qemu_live_address_space_activation_smoke script exists when the core
implementation is ready, the core task must record the accepted smoke as not
yet applicable and stop short of accepting QEMU/substitute evidence. The
separate smoke-core task owns the retained run evidence.

## Next Bounded Task

The next mechanically bounded implementation task is
phase8-live-address-space-activation-core-20260530 if this plan remains
accepted and committed, the accepted contract still selects
phase8-live-address-space-activation-plan-v1, and hardwareTestLock remains
unlocked/restored.

That task may implement only the inspectable preflight boundary and focused
tests needed to produce a LiveAddressSpaceActivationPlan. It must not perform
live translation-register mutation, lower-EL ERET, scheduler runnable
publication, process lifecycle, Pi 5 proof, shell behavior, filesystem syscall
expansion, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Reviewed Inputs

- docs/src/project/phase8-live-address-space-activation-contract.md
- docs/src/project/phase8-live-address-space-activation-source-inventory.md
- docs/src/project/phase8-initial-user-stack-contract.md
- docs/src/project/phase8-qemu-initial-user-stack-smoke-plan.md
- docs/src/project/phase8-initial-process-launch-contract.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-qemu-process-page-table-materialization-smoke-plan.md
- docs/src/project/phase8-process-address-space-contract.md
- docs/src/project/phase8-process-install-contract.md
- docs/src/project/phase8-program-loader-format-contract.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted live
  address-space activation contract and source inventory, adjacent initial
  stack, initial launch, materialization, address-space, install, and loader
  contracts, existing QEMU smoke-plan patterns, roadmap, SUMMARY, and ADR
  index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this plan.
