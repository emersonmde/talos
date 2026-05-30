# Phase 8 QEMU Initial Process Launch Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute
initial process launch smoke plan after the accepted
[Phase 8 Initial Process Launch Contract](phase8-initial-process-launch-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition, initial
process launch implementation, TTBR activation, lower-EL ERET, initial user
stack implementation, argv/envp/auxv/TLS setup, process lifecycle, scheduler
runnable publication, shell behavior, descriptor-backed filesystem syscalls,
writable filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The purpose of this plan is to make the next implementation task mechanical:
construct an inspectable InitialProcessLaunchPlan for immutable /bin/init from
the accepted loader, install, address-space, and non-activating materialization
records, then retain one QEMU/substitute smoke log proving success,
deterministic rejection, no-partial-launch, and no-runnable-publication
observations without claiming a runnable lower-EL process.

## Smoke Invariant

The first QEMU/substitute initial-process-launch smoke must demonstrate one
bounded invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_initial_process_launch_smoke.
2. The scenario consumes the accepted ProgramImagePlan,
   ProcessImageInstallPlan, ProcessAddressSpace, and
   ProcessPageTableMaterialization records for immutable /bin/init. It must
   not use descriptor-backed production filesystem syscalls, writable storage,
   TFTP archives, firmware initramfs envelopes, live CPU translation
   registers, scheduler runnable queues, process table state, or Pi 5
   hardware artifacts.
3. The fixture identity line records the stable loader fixture name
   phase8-program-loader-elf64-aarch64-v1, process-install boundary
   phase8-process-install-plan-v1, address-space boundary
   phase8-process-address-space-model-v1, materialization boundary
   phase8-process-page-table-materialization-v1, and launch-plan boundary
   phase8-initial-process-launch-plan-v1.
4. The success path creates exactly one InitialProcessLaunchPlan with copied
   image, install, address-space, and materialization identities.
5. The plan entry_pc is copied from ProgramImagePlan only after the install
   plan preserves the same entry, the ProcessAddressSpace contains a UserText
   mapping covering that entry, and the materialization record contains an
   EL0-executable UserText descriptor for the same page.
6. The plan reports user_sp_state=blocked-missing-initial-user-stack and
   activation_state=blocked-no-ttbr-activation.
7. The saved-frame intent names ELR, SP_EL0, SPSR, x0 through x5, DAIF, and
   address-space token state without writing architectural registers.
8. The side-effect record reports no TTBR/TCR/MAIR/SCTLR writes, no ASID
   allocation, no live TLB invalidation, no lower-EL ERET, no scheduler
   publication, no process-table mutation, and no descriptor-table mutation.
9. Commit-to-runnable or lower-EL launch requests return ENOSYS and report
   no-partial-launch=true plus no-runnable-publication=true.
10. Negative cases prove deterministic rejection for mismatched fixture,
    entry, mapping, descriptor, destroyed input, bad user range, activation
    request, stack-required launch request, and scheduler publication request.
11. Failure cases must prove no partial launch: no visible launch plan after
    validation failure, no scheduler publication, no process-table mutation,
    no descriptor-table mutation, no register write, no TTBR/TLB mutation, and
    no lower-EL runnable state.
12. The smoke prints final classification and PASS only after success,
    rejection, no-partial-launch, and no-runnable-publication observations have
    been recorded.

If implementation work needs a different scenario name, evidence path,
boundary identity, PASS/classification vocabulary, hardware involvement, live
TTBR activation, initial stack construction, scheduler publication, lower-EL
launch semantics, or process lifecycle behavior, it must stop for supervisor
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

The accepted launch-plan boundary for this smoke is:

    phase8-initial-process-launch-plan-v1

The smoke must derive the launch plan from the accepted /bin/init fixture
chain. The retained log must print the source digest owned by ProgramImagePlan
and may also print a launch-plan digest over stable textual identity, entry,
blocked stack, blocked activation, saved-frame intent, side-effect, and error
records. The digest is diagnostic evidence only; it is not a filesystem,
exec, ABI, TTBR, ASID, TLB, scheduler, hardware, or shell promise.

Required success semantics:

| Field | Required value |
| --- | --- |
| source path | /bin/init |
| loader fixture | phase8-program-loader-elf64-aarch64-v1 |
| install boundary | phase8-process-install-plan-v1 |
| address-space boundary | phase8-process-address-space-model-v1 |
| materialization boundary | phase8-process-page-table-materialization-v1 |
| launch boundary | phase8-initial-process-launch-plan-v1 |
| entry | copied from validated ProgramImagePlan lineage |
| stack | blocked-missing-initial-user-stack |
| activation | blocked-no-ttbr-activation |
| saved frame | intent only, no architectural writes |
| scheduler publication | blocked |
| hardware side effects | none |

The fixture must not claim that /bin/init can be launched. Live
TTBR0_EL1/TTBR1_EL1 programming, TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID
allocation, TLB invalidation, lower-EL ERET, initial user stack mapping,
argv/envp/auxv/TLS layout, descriptor inheritance, scheduler handoff,
exec/spawn/wait, shell behavior, and Pi 5 hardware proof remain outside this
smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-initial-process-launch-smoke: final participants=11 expected=11 errors=0 classification=qemu-initial-process-launch-smoke-complete
    qemu-initial-process-launch-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-initial-process-launch-smoke: start
    qemu-initial-process-launch-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1 launch-boundary=phase8-initial-process-launch-plan-v1
    qemu-initial-process-launch-smoke: success output=InitialProcessLaunchPlan published=true entry=0x<hex> user-sp-state=blocked-missing-initial-user-stack activation-state=blocked-no-ttbr-activation ok=true
    qemu-initial-process-launch-smoke: entry provenance image=true install=true address-space-user-text=true materialization-user-text-descriptor=true el0-executable=true ok=true
    qemu-initial-process-launch-smoke: saved-frame-intent elr=entry-pc sp-el0=blocked-missing-initial-user-stack spsr=blocked-pending-lower-el-pstate-policy x0-x5=blocked-pending-startup-abi daif=blocked-pending-interrupt-mask-policy address-space-token=model-only ok=true
    qemu-initial-process-launch-smoke: side-effects ttbr-mutated=false tcr-mutated=false mair-mutated=false sctlr-mutated=false asid-allocated=false tlb-mutated=false lower-el-eret=false scheduler-published=false process-table-mutated=false descriptor-table-mutated=false ok=true
    qemu-initial-process-launch-smoke: commit-request target=runnable errno=-ENOSYS no-partial-launch=true no-runnable-publication=true ok=true
    qemu-initial-process-launch-smoke: error case=identity-mismatch errno=-EINVAL partial-launch=false runnable-published=false ok=true
    qemu-initial-process-launch-smoke: error case=entry-mismatch errno=-ENOEXEC partial-launch=false runnable-published=false ok=true
    qemu-initial-process-launch-smoke: error case=missing-user-text-descriptor errno=-ENOEXEC partial-launch=false runnable-published=false ok=true
    qemu-initial-process-launch-smoke: error case=forbidden-entry-range errno=-EACCES partial-launch=false runnable-published=false ok=true
    qemu-initial-process-launch-smoke: error case=destroyed-input errno=-EINVAL partial-launch=false runnable-published=false ok=true
    qemu-initial-process-launch-smoke: error case=activation-request errno=-ENOSYS partial-launch=false runnable-published=false ok=true
    qemu-initial-process-launch-smoke: error case=stack-required-launch errno=-ENOSYS partial-launch=false runnable-published=false ok=true
    qemu-initial-process-launch-smoke: error case=scheduler-publication-request errno=-ENOSYS partial-launch=false runnable-published=false ok=true

The implementation may print additional digest, mapping, descriptor, blocked
prerequisite, or diagnostic fields. The required line shapes must stay stable
enough for the script gate. Hex values and counts are field placeholders
because the later implementation task owns the exact model representation.

## Failure Classification

The smoke must distinguish launch-contract failures from scenario wiring
failures:

- Contract failure: the launcher accepts mismatched identities, accepts an
  entry without UserText coverage or EL0-executable descriptor provenance,
  fabricates an initial stack, reports activation as unblocked, writes any
  register or live translation state, publishes scheduler or process-table
  state, returns the wrong errno, or leaves partial launch/runnable state
  visible after rejection.
- Scenario wiring failure: the scenario cannot select
  qemu_initial_process_launch_smoke, cannot obtain the accepted
  ProgramImagePlan/ProcessImageInstallPlan/ProcessAddressSpace/
  ProcessPageTableMaterialization chain, cannot retain a fresh log, cannot
  print the fixture identity line, or cannot drive success and negative
  observations in order.
- Regression failure: an accepted program-loader, process-install,
  process-address-space, process-page-table-materialization, read-only
  initramfs/VFS, user-memory, descriptor/read, or lower-EL/syscall gate
  required by this plan fails after implementation changes touch shared
  owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_initial_process_launch_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains qemu-initial-process-launch-smoke: start before
   looking for PASS.
4. Confirm the fixture identity line appears before launch-plan observations.
5. Confirm entry provenance and saved-frame-intent observations appear before
   negative errno lines.
6. Confirm every negative case reports partial-launch=false and
   runnable-published=false.
7. Confirm the side-effect line reports no register, TTBR/TLB, scheduler,
   process-table, descriptor-table, or lower-EL mutation.
8. Confirm the commit-request line reports errno=-ENOSYS,
   no-partial-launch=true, and no-runnable-publication=true.
9. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
10. Rerun the smoke script once after cleaning only stale QEMU/substitute
    output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Regression Gates

The implementation task must retain:

- The QEMU/substitute initial process launch smoke log named above.
- The command used to build and run qemu_initial_process_launch_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- A planned qemu-initial-process-launch-smoke script, or an accepted
  replacement script, that retains the required log and greps the required
  lines.
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

The allowed evidence level for this smoke is QEMU/substitute only. It must not
claim physical Pi 5 behavior, boot-archive behavior, or runnable lower-EL
process behavior.

## Next Task

The mechanically next implementation task should be
phase8-initial-process-launch-core-20260530, if queued dependencies remain
satisfied.

That implementation task should add only the accepted
InitialProcessLaunchPlan boundary and focused tests needed to produce the
smoke observations above. Initial user stack implementation, live address-space
activation, lower-EL ERET, scheduler runnable publication, process lifecycle,
filesystem syscalls, shell behavior, Pi 5 hardware proof, networking, and SSH
remain blocked until later explicit tasks accept their contracts and gates.

## Reviewed Inputs

- docs/src/project/phase8-initial-process-launch-contract.md
- docs/src/project/phase8-initial-process-launch-source-inventory.md
- docs/src/project/phase8-qemu-process-page-table-materialization-smoke-plan.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-process-address-space-contract.md
- docs/src/project/phase8-process-install-contract.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- docs/src/SUMMARY.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted initial process
  launch contract and source inventory, the QEMU process page-table
  materialization smoke-plan pattern, adjacent Phase 8 contracts, roadmap,
  SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this smoke plan.
