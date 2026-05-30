# Phase 8 QEMU Process Page-Table Materialization Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute
process page-table materialization smoke plan after the accepted
[Phase 8 Process Page-Table Materialization Contract](phase8-process-page-table-materialization-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition,
materialization core implementation, TTBR activation, ASID/TLB policy change,
lower-EL launch, argv/envp setup, process lifecycle, shell behavior,
descriptor-backed filesystem syscall, writable filesystem, persistent storage,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

The purpose of this plan is to make the next implementation task mechanical:
materialize the accepted ProcessAddressSpace model into a non-activating
AArch64 descriptor image and owned user-frame byte images, then retain one
QEMU/substitute smoke log proving success, deterministic rejection, rollback,
no-leak, and teardown observations without claiming runnable user state.

## Smoke Invariant

The first QEMU/substitute materialization smoke must demonstrate one bounded
invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_process_page_table_materialization_smoke.
2. The scenario consumes the accepted ProgramImagePlan,
   ProcessImageInstallPlan, and ProcessAddressSpace model for immutable
   /bin/init. It must not use descriptor-backed production open/read syscalls,
   writable storage, host filesystem state, TFTP archives, firmware initramfs
   envelopes, live CPU translation registers, or Pi 5 hardware artifacts.
3. The fixture identity line records the stable loader fixture name
   phase8-program-loader-elf64-aarch64-v1, process-install boundary
   phase8-process-install-plan-v1, address-space boundary
   phase8-process-address-space-model-v1, and materialization boundary
   phase8-process-page-table-materialization-v1.
4. The success path creates exactly one published
   ProcessPageTableMaterialization record with one owned root descriptor image,
   the minimum owned table-page descriptor images needed for the accepted user
   mappings, one owned user-frame byte image per accepted UserFrameLease, and
   ordered descriptor records tying each ProcessUserMapping to a user-frame
   physical address.
5. Every user frame is zeroed before copy, reports copied bytes, zeroed bytes,
   source page ordinal, virtual page, physical-frame identity, and
   scrub-required release state.
6. UserText descriptors preserve R-X and EL0 executable policy: normal memory,
   inner shareable, AF set, AP=EL0 read-only, PXN set, UXN clear, and W^X
   true.
7. UserData descriptors preserve RW- and non-executable policy: normal memory,
   inner shareable, AF set, AP=EL0 read-write, PXN set, UXN set, and W^X true.
8. The success path reports activation_blocked=true and
   kernel_mapping_policy=activation-blocked-no-kernel-half. It must not write
   TTBR0_EL1, TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID state, live TLB
   state, scheduler state, or lower-EL frames.
9. Negative cases prove deterministic rejection for address-space/model
   mismatch, null-guard or kernel/device range, permission widening,
   resource exhaustion, unsupported table topology, copy/zero mismatch, and
   activation requests.
10. Failure cases must prove no partial materialization: no visible published
    record, unreleased root/table/user-frame lease, descriptor slot, copied
    frame image, scheduler publication, live translation-register mutation, or
    lower-EL runnable state is observable.
11. Teardown observations prove that a published record clears descriptors,
    releases table pages, scrubs or marks user frames, releases the root image,
    and that a second teardown reports already-destroyed without double
    release.
12. The smoke prints final classification and PASS only after success,
    rejection, rollback/no-leak, and teardown observations have been recorded.

If implementation work needs a different scenario name, fixture identity,
evidence path, boundary type, negative-case matrix, PASS/classification
vocabulary, hardware involvement, live TTBR activation, or lower-EL launch
semantics, it must stop for supervisor planning instead of accepting a changed
smoke.

## Fixture And Boundary Identity

The accepted loader fixture identity remains:

    phase8-program-loader-elf64-aarch64-v1

The accepted process-install boundary remains:

    phase8-process-install-plan-v1

The accepted process address-space boundary remains:

    phase8-process-address-space-model-v1

The accepted materialization boundary for this smoke is:

    phase8-process-page-table-materialization-v1

The smoke must derive the materialization record from the accepted
ProcessAddressSpace model for /bin/init. The retained log must print the source
digest owned by ProgramImagePlan and may also print a materialization manifest
digest over stable textual identity, lease, descriptor, frame, rollback, and
teardown records. The materialization digest is diagnostic evidence only; it
is not a filesystem, exec, ABI, TTBR, ASID, TLB, hardware, or launch promise.

Required success semantics:

| Field | Required value |
| --- | --- |
| source path | /bin/init |
| loader fixture | phase8-program-loader-elf64-aarch64-v1 |
| install boundary | phase8-process-install-plan-v1 |
| address-space boundary | phase8-process-address-space-model-v1 |
| materialization boundary | phase8-process-page-table-materialization-v1 |
| root page | one owned zeroed descriptor-image page |
| table pages | minimum owned zeroed descriptor-image pages needed by accepted user mappings |
| user frames | one zero-before-copy frame image per accepted user frame lease |
| descriptors | ordered records for accepted UserText/UserData mappings |
| kernel mapping policy | activation-blocked-no-kernel-half |
| activation | activation_blocked=true |
| hardware side effects | none |

The fixture must not claim that /bin/init can be launched. Live TTBR/TCR/MAIR
programming, ASID allocation, TLB invalidation, lower-EL frames, initial user
stack, argv/envp, descriptor inheritance, scheduler handoff, exec/spawn/wait,
and shell behavior remain outside this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-process-page-table-materialization-smoke: final participants=12 expected=12 errors=0 classification=qemu-process-page-table-materialization-smoke-complete
    qemu-process-page-table-materialization-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-process-page-table-materialization-smoke: start
    qemu-process-page-table-materialization-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1 materialization-boundary=phase8-process-page-table-materialization-v1
    qemu-process-page-table-materialization-smoke: success output=ProcessPageTableMaterialization published=true id=0x<hex> root-pages=1 table-pages=<decimal> user-frames=<decimal> descriptors=<decimal> activation-blocked=true kernel-mapping-policy=activation-blocked-no-kernel-half ok=true
    qemu-process-page-table-materialization-smoke: frame index=0 kind=UserText virtual-page=0x<hex> physical-frame=0x<hex> copy-bytes=0x<hex> zero-bytes=0x<hex> zero-before-copy=true source-page=0 scrub-required=true ok=true
    qemu-process-page-table-materialization-smoke: frame index=1 kind=UserData virtual-page=0x<hex> physical-frame=0x<hex> copy-bytes=0x<hex> zero-bytes=0x<hex> zero-before-copy=true source-page=1 scrub-required=true ok=true
    qemu-process-page-table-materialization-smoke: descriptor index=0 kind=UserText flags=R-X ap=EL0_RO pxn=true uxn=false attr=normal-inner-shareable af=true wx=false ok=true
    qemu-process-page-table-materialization-smoke: descriptor index=1 kind=UserData flags=RW- ap=EL0_RW pxn=true uxn=true attr=normal-inner-shareable af=true wx=false ok=true
    qemu-process-page-table-materialization-smoke: side-effects root-pages-leased=1 table-pages-leased=<decimal> user-frames-leased=<decimal> descriptors-installed=<decimal> copied-bytes=0x<hex> zeroed-bytes=0x<hex> ttbr-mutated=false tlb-mutated=false scheduler-published=false lower-el-frame=false runnable=false ok=true
    qemu-process-page-table-materialization-smoke: teardown phase=first descriptors-cleared=<decimal> table-pages-released=<decimal> user-frames-released=<decimal> root-released=true already-destroyed=false ok=true
    qemu-process-page-table-materialization-smoke: teardown phase=second descriptors-cleared=0 table-pages-released=0 user-frames-released=0 root-released=false already-destroyed=true ok=true
    qemu-process-page-table-materialization-smoke: error case=bad-address-space errno=-EINVAL partial-materialization=false leaked-leases=false ok=true
    qemu-process-page-table-materialization-smoke: error case=forbidden-range errno=-EACCES partial-materialization=false leaked-leases=false ok=true
    qemu-process-page-table-materialization-smoke: error case=permission-widening errno=-EACCES partial-materialization=false leaked-leases=false ok=true
    qemu-process-page-table-materialization-smoke: error case=resource-exhaustion errno=-ENOMEM partial-materialization=false leaked-leases=false ok=true
    qemu-process-page-table-materialization-smoke: error case=unsupported-topology errno=-ENOTSUP partial-materialization=false leaked-leases=false ok=true
    qemu-process-page-table-materialization-smoke: error case=copy-zero-mismatch errno=-EINVAL partial-materialization=false leaked-leases=false ok=true
    qemu-process-page-table-materialization-smoke: error case=activation-request errno=-ENOSYS partial-materialization=false leaked-leases=false ok=true

The implementation may print additional frame, descriptor, digest,
intermediate-table, rollback, or manifest fields. The required line shapes
must stay stable enough for the script gate. Hex values, counts, and mapping
totals are field placeholders because the later implementation task owns the
exact model representation.

## Failure Classification

The smoke must distinguish materialization contract failures from scenario
wiring failures:

- Contract failure: the materializer widens permissions, accepts forbidden
  ranges, creates executable data or writable text, publishes before all
  resources and descriptors are complete, loses copy/zero accounting, reports
  the wrong errno, leaks any lease after rejection, leaves a partial record
  visible, mutates live TTBR/TLB/scheduler state, or double-releases during
  teardown.
- Scenario wiring failure: the scenario cannot select
  qemu_process_page_table_materialization_smoke, cannot obtain the accepted
  ProgramImagePlan/ProcessImageInstallPlan/ProcessAddressSpace chain, cannot
  retain a fresh log, cannot print the fixture identity line, or cannot drive
  success, negative, rollback/no-leak, and teardown observations in order.
- Regression failure: an accepted process-address-space, process-install,
  program-loader, read-only initramfs/VFS, user-memory, descriptor/read, or
  lower-EL/syscall gate required by this plan fails after implementation
  changes touch shared owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_process_page_table_materialization_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains
   qemu-process-page-table-materialization-smoke: start before looking for
   PASS.
4. Confirm the fixture identity line appears before materialization
   observations.
5. Confirm success frame, descriptor, and side-effect lines appear before
   negative errno lines.
6. Confirm every negative case reports partial-materialization=false and
   leaked-leases=false.
7. Confirm teardown lines show first-release and second already-destroyed
   behavior.
8. Confirm the side-effect line reports ttbr-mutated=false,
   tlb-mutated=false, scheduler-published=false, lower-el-frame=false, and
   runnable=false.
9. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
10. Rerun the smoke script once after cleaning only stale QEMU/substitute
    output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Regression Gates

The implementation task must retain:

- The QEMU/substitute process page-table materialization smoke log named
  above.
- The command used to build and run
  qemu_process_page_table_materialization_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- A planned qemu-process-page-table-materialization-smoke script, or an
  accepted replacement script, that retains the required log and greps the
  required lines.
- scripts/qemu-process-address-space-smoke.sh if implementation changes
  ProcessAddressSpace model generation, lease accounting, teardown behavior,
  address-space diagnostics, or boot-scenario output owners used by that smoke.
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
- mdbook build when docs are touched.

The evidence level is QEMU/substitute only. It must explicitly state that no
Pi 5 hardware behavior, boot archive publication, TFTP archive identity, live
TTBR activation, ASID/TLB mutation, lower-EL launch, executable userland,
shell, networking, or SSH support is claimed.

## Source Owners For Later Implementation

The later materialization core task may touch only these owners unless it
records a narrow reason:

- A small page-table materialization module, or a narrow
  process-address-space-adjacent module, for
  ProcessPageTableMaterialization identity, owned root/table/user-frame image
  records, descriptor records, deterministic error mapping, rollback,
  teardown, digest reporting, and no-partial-materialization tests.
- src/process_address_space.rs only for reusing ProcessAddressSpace,
  ProcessUserMapping, and lease accessors, or for narrowly exposing helper
  APIs required by the accepted contract.
- src/process_install.rs only for reusing ProcessImageInstallPlan and page
  install records already accepted by the install frontier.
- src/program_loader.rs only if the smoke needs stable fixture identity or
  source-digest reporting already accepted by the loader frontier.
- src/posix.rs only for reusing PosixError, user-range, null-guard, and
  UserMappingPermissions vocabulary already accepted by Phase 7 and the
  loader.
- src/memory_map/translation.rs only for descriptor constants/helpers needed
  to construct an owned descriptor image, not for EL2 bootstrap table
  population or live MMU register programming.
- Focused unit tests for accepted success, forbidden range, permission
  widening, resource exhaustion, unsupported topology, copy/zero mismatch,
  activation request rejection, rollback, and idempotent teardown behavior.
- Documentation and the task record needed to report evidence.

The later QEMU/substitute smoke task may also touch:

- build.rs and src/main.rs for boot-scenario routing.
- src/target/qemu_virt.rs for scenario orchestration, fixture reporting,
  required output, and final classification.
- scripts/qemu-process-page-table-materialization-smoke.sh for retained
  evidence.

Existing live TTBR/TCR/MAIR/SCTLR programming, ASID lifecycle, TLB
invalidation, lower-EL launch, process table, scheduler handoff, argv/envp,
user stack, descriptor inheritance across exec, shell, Pi 5, RP1/PCIe, UART
interrupt, DMA/cache-driver, network, and SSH owners remain out of scope for
this smoke frontier.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 hardware proof, archive publishing, power-cycle, serial observe, TFTP
  fixture delivery, and hardware-lock acquisition.
- Live TTBR0_EL1/TTBR1_EL1 activation, ASID allocation/reuse, TCR/MAIR
  compatibility commitment, live TLB invalidation, barrier sequencing, and
  address-space activation during context switch.
- Lower-EL launch of the loaded image, initial user stack, argv/envp,
  auxiliary vectors, TLS, libc startup, exec/spawn/wait, and shell behavior.
- Process creation, PID allocation, parent/child ownership, exit status,
  signals, credentials, close-on-exec enforcement, current working directory,
  process root, descriptor inheritance, and open-file-description final
  release.
- Descriptor-backed filesystem syscalls, directory iteration, readdir/getdents,
  seek syscalls, writable filesystems, persistent storage, block devices,
  symlinks, device nodes, pipes, sockets, mmap, demand paging, copy-on-write,
  shared memory, user DMA buffers, Rust std filesystem support, networking,
  and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Next Mechanically Derivable Task

The next implementation task should be
phase8-process-page-table-materialization-core-20260530 if the supervisor has
queued it with explicit scope, acceptance criteria, validation gates,
documentation requirements, and evidence requirements.

Its goal should be to implement only the non-activating descriptor-image and
user-frame materialization boundary selected by the accepted contract and this
smoke plan. If that queued task is missing, blocked, ambiguous, or requires a
broader activation decision, the worker must request supervisor planning
instead of broadening scope.

## Reviewed Inputs

- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-process-page-table-materialization-source-inventory.md
- docs/src/project/phase8-qemu-process-address-space-smoke-plan.md
- docs/src/project/phase8-qemu-process-install-smoke-plan.md
- tasks/2026-05-30-phase8-process-page-table-materialization-contract.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted materialization
  contract and source inventory, process address-space and process-install
  smoke-plan patterns, roadmap, SUMMARY, ADR index, and accepted contract task
  record.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this smoke plan.
