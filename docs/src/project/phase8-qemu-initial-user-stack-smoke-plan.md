# Phase 8 QEMU Initial User Stack Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute
initial user stack smoke plan after the accepted
[Phase 8 Initial User Stack Contract](phase8-initial-user-stack-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition, initial
user stack implementation, TTBR activation, lower-EL ERET, scheduler runnable
publication, process lifecycle, shell behavior, descriptor-backed filesystem
syscalls, writable filesystem, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

The purpose of this plan is to make the next implementation task mechanical:
construct an inspectable InitialUserStackPlan from the accepted loader,
install, address-space, materialization, and launch records, then retain one
QEMU/substitute smoke log proving success, deterministic rejection,
no-partial-stack, no-partial-launch, and zero live-launch side effects without
claiming a runnable lower-EL process.

## Smoke Invariant

The first QEMU/substitute initial-user-stack smoke must demonstrate one
bounded invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_initial_user_stack_smoke.
2. The scenario consumes the accepted ProgramImagePlan,
   ProcessImageInstallPlan, ProcessAddressSpace,
   ProcessPageTableMaterialization, and InitialProcessLaunchPlan records for
   immutable /bin/init.
3. The fixture identity line records loader identity
   phase8-program-loader-elf64-aarch64-v1, process-install boundary
   phase8-process-install-plan-v1, address-space boundary
   phase8-process-address-space-model-v1, materialization boundary
   phase8-process-page-table-materialization-v1, launch boundary
   phase8-initial-process-launch-plan-v1, and stack boundary
   phase8-initial-user-stack-plan-v1.
4. The success path creates exactly one InitialUserStackPlan with copied
   image, install, address-space, materialization, and launch identities.
5. The stack layout reports stack_top=0x0000_8000_0000_0000,
   initial_sp=0x0000_8000_0000_0000, usable range
   [0x0000_7fff_ffff_c000, 0x0000_8000_0000_0000), guard range
   [0x0000_7fff_ffff_b000, 0x0000_7fff_ffff_c000), four usable pages, one
   guard page, LOADER_PAGE_SIZE=0x1000, and 16-byte SP alignment.
6. Every usable page is USER_DATA, stack-owned, zeroed_before_copy=true,
   copied_bytes=0, and zeroed_bytes=0x1000; aggregate copied_bytes=0 and
   zeroed_bytes=0x4000.
7. The guard page has no frame lease and no descriptor.
8. The startup payload state is minimal-empty-argc0 with argc=0, argv=NULL,
   envp=NULL, auxv blocked, TLS blocked, and no copied startup bytes.
9. Launch-plan integration changes only model state:
   user_sp_state=model-only-initial-user-stack-ready and saved-frame SP_EL0
   intent equals the stack initial SP.
10. Activation remains blocked-no-ttbr-activation and side-effect records show
    no TTBR/TCR/MAIR/SCTLR writes, no ASID allocation, no live TLB
    invalidation, no lower-EL ERET, no scheduler publication, no process-table
    mutation, and no descriptor-table mutation.
11. Negative cases prove deterministic rejection for identity mismatch, stack
    range fault, image overlap, executable-stack permission request, exhausted
    stack-page capacity, already-stack-ready launch input, and unsupported
    live-launch request.
12. Failure cases must prove no partial stack and no partial launch: no stack
    plan after validation failure, all partial stack leases released, no
    scheduler publication, no process-table mutation, no descriptor-table
    mutation, no register write, no TTBR/TLB mutation, and no lower-EL
    runnable state.
13. The smoke prints final classification and PASS only after success,
    rejection, teardown, no-partial-stack, no-partial-launch, launch
    integration, and side-effect observations have been recorded.

If implementation work needs a different scenario name, evidence path,
boundary identity, PASS/classification vocabulary, hardware involvement, live
TTBR activation, broad argv/envp/auxv/TLS ABI, scheduler publication,
lower-EL launch semantics, or process lifecycle behavior, it must stop for
supervisor planning instead of accepting a changed smoke.

## Fixture And Boundary Identity

The accepted loader fixture identity remains:

    phase8-program-loader-elf64-aarch64-v1

The accepted process-install boundary remains:

    phase8-process-install-plan-v1

The accepted process address-space boundary remains:

    phase8-process-address-space-model-v1

The accepted non-activating materialization boundary remains:

    phase8-process-page-table-materialization-v1

The accepted launch-plan boundary remains:

    phase8-initial-process-launch-plan-v1

The accepted stack-plan boundary for this smoke is:

    phase8-initial-user-stack-plan-v1

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
| stack top | 0x0000_8000_0000_0000 |
| initial SP | 0x0000_8000_0000_0000, 16-byte aligned |
| usable stack range | [0x0000_7fff_ffff_c000, 0x0000_8000_0000_0000) |
| guard range | [0x0000_7fff_ffff_b000, 0x0000_7fff_ffff_c000) |
| usable pages | 4 USER_DATA stack-owned pages |
| guard pages | 1 unmapped page with no lease and no descriptor |
| copied bytes | 0 |
| zeroed bytes | 0x4000 |
| startup payload | minimal-empty-argc0 |
| launch binding | model-only stack ready, activation still blocked |
| hardware side effects | none |

The fixture must not claim that /bin/init can run. Live TTBR0_EL1/TTBR1_EL1
programming, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, TLB
invalidation, lower-EL ERET, scheduler handoff, process lifecycle,
descriptor-backed filesystem syscalls, broad argv/envp/auxv/TLS layout, shell
behavior, and Pi 5 hardware proof remain outside this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-initial-user-stack-smoke: final participants=13 expected=13 errors=0 classification=qemu-initial-user-stack-smoke-complete
    qemu-initial-user-stack-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-initial-user-stack-smoke: start
    qemu-initial-user-stack-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1 stack-boundary=phase8-initial-user-stack-plan-v1
    qemu-initial-user-stack-smoke: success output=InitialUserStackPlan published=true stack-top=0x0000800000000000 initial-sp=0x0000800000000000 sp-aligned-16=true ok=true
    qemu-initial-user-stack-smoke: layout usable-start=0x00007fffffffc000 usable-end=0x0000800000000000 guard-start=0x00007fffffffb000 guard-end=0x00007fffffffc000 page-size=0x1000 usable-pages=4 guard-pages=1 ok=true
    qemu-initial-user-stack-smoke: ownership usable-user-data=true stack-owned=true guard-has-frame=false guard-has-descriptor=false total-copied-bytes=0 total-zeroed-bytes=0x4000 ok=true
    qemu-initial-user-stack-smoke: startup argc=0 argv=null envp=null auxv=blocked-pending-startup-abi tls=blocked-pending-startup-abi copied-startup-bytes=0 ok=true
    qemu-initial-user-stack-smoke: launch-binding user-sp-state=model-only-initial-user-stack-ready saved-frame-sp-el0=0x0000800000000000 activation-state=blocked-no-ttbr-activation no-partial-launch=true ok=true
    qemu-initial-user-stack-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false asid-allocated=false tlb-mutated=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true
    qemu-initial-user-stack-smoke: teardown stack-leases-released=true image-leases-untouched=true idempotent=true ok=true
    qemu-initial-user-stack-smoke: error case=identity-mismatch errno=-EINVAL partial-stack=false partial-launch=false ok=true
    qemu-initial-user-stack-smoke: error case=range-fault errno=-EFAULT partial-stack=false partial-launch=false ok=true
    qemu-initial-user-stack-smoke: error case=image-overlap errno=-EACCES partial-stack=false partial-launch=false ok=true
    qemu-initial-user-stack-smoke: error case=executable-stack errno=-EACCES partial-stack=false partial-launch=false ok=true
    qemu-initial-user-stack-smoke: error case=capacity-exhausted errno=-ENOMEM partial-stack=false partial-launch=false leases-released=true ok=true
    qemu-initial-user-stack-smoke: error case=already-stack-ready errno=-EINVAL partial-stack=false partial-launch=false ok=true
    qemu-initial-user-stack-smoke: error case=live-launch-request errno=-ENOSYS partial-stack=false partial-launch=false runnable-published=false ok=true

The implementation may print additional digest, lease-token, page, descriptor,
blocked prerequisite, or diagnostic fields. The required line shapes must stay
stable enough for the script gate. Hex values and lease identifiers are field
placeholders because the later implementation task owns the exact model
representation.

## Failure Classification

The smoke must distinguish stack-contract failures from scenario wiring
failures:

- Contract failure: the stack constructor accepts mismatched identities,
  produces an unaligned SP, maps the guard page, allocates executable stack
  permissions, overlaps image pages, copies startup bytes, fails to release
  partial stack leases, mutates launch state beyond model-only stack readiness,
  writes any live translation or register state, publishes scheduler or
  process-table state, returns the wrong errno, or leaves partial stack or
  runnable state visible after rejection.
- Scenario wiring failure: the scenario cannot select
  qemu_initial_user_stack_smoke, cannot obtain the accepted
  ProgramImagePlan/ProcessImageInstallPlan/ProcessAddressSpace/
  ProcessPageTableMaterialization/InitialProcessLaunchPlan chain, cannot
  retain a fresh log, cannot print the fixture identity line, or cannot drive
  success and negative observations in order.
- Regression failure: an accepted initial-process-launch, materialization,
  address-space, install, loader, read-only initramfs/VFS, user-memory,
  descriptor/read, or lower-EL/syscall gate required by this plan fails after
  implementation changes touch shared owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_initial_user_stack_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains qemu-initial-user-stack-smoke: start before
   looking for PASS.
4. Confirm the fixture identity line appears before stack observations.
5. Confirm layout, ownership, startup, and launch-binding observations appear
   before negative errno lines.
6. Confirm every negative case reports partial-stack=false and
   partial-launch=false.
7. Confirm the side-effect line reports no register, TTBR/TLB, scheduler,
   process-table, descriptor-table, or lower-EL mutation.
8. Confirm teardown releases stack leases without touching image leases.
9. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
10. Rerun the smoke script once after cleaning only stale QEMU/substitute
    output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Regression Gates

The implementation task must retain:

- The QEMU/substitute initial user stack smoke log named above.
- The command used to build and run qemu_initial_user_stack_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- A planned qemu-initial-user-stack-smoke script, or an accepted replacement
  script, that retains the required log and greps the required lines.
- scripts/qemu-initial-process-launch-smoke.sh if implementation changes
  InitialProcessLaunchPlan generation, launch diagnostics, saved-frame intent,
  side-effect accounting, or boot-scenario output owners used by that smoke.
- scripts/qemu-process-page-table-materialization-smoke.sh if implementation
  changes ProcessPageTableMaterialization generation, descriptor output,
  activation-blocked output, teardown behavior, or diagnostic output owners
  used by that smoke.
- scripts/qemu-process-address-space-smoke.sh if implementation changes
  ProcessAddressSpace model generation, lease accounting, teardown behavior,
  address-space diagnostics, or boot-scenario output owners used by that smoke.
- scripts/qemu-process-install-smoke.sh if implementation changes
  ProcessImageInstallPlan generation, process-install diagnostics, or
  boot-scenario output owners used by that smoke.
- scripts/qemu-program-loader-smoke.sh if implementation changes
  ProgramImagePlan generation, /bin/init fixture bytes, loader diagnostics, or
  boot-scenario output owners used by that smoke.
- Existing lower-EL/syscall, descriptor, read/stdin, and pointer-copy smokes
  only if implementation touches shared syscall dispatch, descriptor tables,
  user-copy helpers, lower-EL routing, boot-scenario routing, or diagnostic
  output owners used by those smokes.
- git diff --check.
- mdbook build if docs are touched.

The allowed evidence level for this smoke is QEMU/substitute or
target-independent inspection only. It must not claim physical Pi 5 behavior,
boot-archive behavior, live TTBR activation, or runnable lower-EL process
behavior.

## Next Task

The mechanically next implementation task should be
phase8-initial-user-stack-core-20260530, if queued dependencies remain
satisfied.

That implementation task should add only the accepted InitialUserStackPlan
boundary and focused tests needed to produce the smoke observations above.
Live address-space activation, lower-EL ERET, scheduler runnable publication,
process lifecycle, broad argv/envp/auxv/TLS ABI, descriptor-backed filesystem
syscalls, shell behavior, Pi 5 hardware proof, networking, and SSH remain
blocked until later explicit tasks accept their contracts and gates.

## Reviewed Inputs

- docs/src/project/phase8-initial-user-stack-contract.md
- docs/src/project/phase8-initial-user-stack-source-inventory.md
- docs/src/project/phase8-initial-process-launch-contract.md
- docs/src/project/phase8-qemu-initial-process-launch-smoke-plan.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-process-address-space-contract.md
- docs/src/project/phase8-process-install-contract.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- docs/src/SUMMARY.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted initial user
  stack contract and source inventory, initial process launch contract and
  smoke-plan pattern, adjacent Phase 8 contracts, roadmap, SUMMARY, and ADR
  index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this smoke plan.
