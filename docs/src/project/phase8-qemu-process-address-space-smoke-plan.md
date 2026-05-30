# Phase 8 QEMU Process Address-Space Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute
process address-space smoke plan after the accepted
[Phase 8 Process Address-Space Contract](phase8-process-address-space-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition,
process-address-space core implementation, AArch64 descriptor construction,
TTBR/TCR switching, lower-EL launch, argv/envp setup, process creation,
exec/spawn/wait, shell behavior, descriptor-backed filesystem syscalls,
writable filesystem, persistent storage, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

The purpose of this plan is to make the next implementation task mechanical:
install the accepted ProcessImageInstallPlan into one target-independent
ProcessAddressSpace model with explicit leases, mappings, rollback evidence,
and idempotent teardown, then retain one QEMU/substitute smoke log proving the
success and rejection vocabulary without claiming hardware page-table support.

## Smoke Invariant

The first QEMU/substitute process address-space smoke must demonstrate one
bounded invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_process_address_space_smoke.
2. The scenario consumes the accepted ProgramImagePlan and
   ProcessImageInstallPlan for immutable /bin/init. It must not use
   descriptor-backed production open/read syscalls, writable storage, host
   filesystem state, TFTP archives, firmware initramfs envelopes, or Pi 5
   hardware artifacts.
3. The fixture identity line records the stable loader fixture name
   phase8-program-loader-elf64-aarch64-v1, install-plan boundary
   phase8-process-install-plan-v1, and address-space boundary
   phase8-process-address-space-model-v1.
4. The success path creates exactly one published ProcessAddressSpace model
   with a stable identity, owner label, one model page-table root token,
   table-page lease records, user-frame lease records, ordered mappings,
   copy/zero byte accounting, and teardown status.
5. UserText mappings preserve R-X permissions and UserData mappings preserve
   RW- permissions. No mapping may widen permissions, cross the null guard,
   cross the canonical user limit, enter kernel/device ranges, or overlap an
   earlier mapping.
6. Every installed user page receives one zero-before-copy user-frame lease.
   The smoke reports copied bytes, zeroed bytes, source page ordinal, and
   release status for every lease.
7. The success path publishes the address-space record only after all leases,
   copy/zero accounting, and mappings are complete.
8. Negative cases prove deterministic rejection for malformed install-plan
   input, null-guard or user/kernel split violation, overlapping mapping,
   permission widening, root/table/user-frame lease exhaustion, and copy/zero
   representation failure.
9. Failure cases must prove no partial install: no visible address-space
   record, unreleased root token, table-page lease, user-frame lease, mapping
   record, scheduler owner, descriptor mutation, lower-EL frame, or runnable
   state is observable.
10. Teardown observations prove that published records release mappings,
    user-frame leases, table-page leases, and the root token in deterministic
    order, and that a second teardown reports already-destroyed without
    double release.
11. The smoke prints final classification and PASS only after the success,
    rejection, no-leak, and teardown observations have been recorded.

If implementation work needs a different scenario name, fixture identity,
evidence path, boundary type, negative-case matrix, PASS/classification
vocabulary, hardware involvement, or lower-EL launch semantics, it must stop
for supervisor planning instead of accepting a changed smoke.

## Fixture And Boundary Identity

The accepted loader fixture identity remains:

    phase8-program-loader-elf64-aarch64-v1

The accepted process-install boundary identity remains:

    phase8-process-install-plan-v1

The accepted process address-space boundary identity for this smoke is:

    phase8-process-address-space-model-v1

The smoke must derive the address-space model from the accepted
ProcessImageInstallPlan for /bin/init. The retained log must print the source
digest owned by ProgramImagePlan and may also print an address-space manifest
digest over stable textual identity, lease, mapping, and teardown records. The
address-space digest is diagnostic evidence only; it is not a filesystem,
exec, ABI, page-table, or hardware promise.

Required success semantics:

| Field | Required value |
| --- | --- |
| source path | /bin/init |
| loader fixture | phase8-program-loader-elf64-aarch64-v1 |
| install boundary | phase8-process-install-plan-v1 |
| address-space boundary | target-independent ProcessAddressSpace model |
| root token | one model page-table root token |
| user frames | one zero-before-copy lease per installed page |
| mappings | ordered from ProcessImageInstallPlan page records |
| text page policy | UserText, R-X, copy file bytes, zero rounded tails if any |
| data page policy | UserData, RW-, copy file bytes, zero BSS and rounded tails |
| output object | published ProcessAddressSpace record |
| hardware side effects | none |

The fixture must not claim that /bin/init can be launched. Real AArch64
descriptors, TTBR/TCR/ASID/TLB policy, lower-EL frames, initial stack,
argv/envp, descriptor inheritance, scheduler handoff, exec/spawn/wait, and
shell behavior remain outside this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-process-address-space-smoke: final participants=8 expected=8 errors=0 classification=qemu-process-address-space-smoke-complete
    qemu-process-address-space-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-process-address-space-smoke: start
    qemu-process-address-space-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1 address-space-boundary=phase8-process-address-space-model-v1
    qemu-process-address-space-smoke: success output=ProcessAddressSpace published=true id=0x<hex> owner=0x<hex> root-token=0x<hex> table-leases=<decimal> user-frame-leases=<decimal> mappings=<decimal> ok=true
    qemu-process-address-space-smoke: mapping index=0 kind=UserText flags=R-X copy-bytes=0x<hex> zero-bytes=0x<hex> zero-before-copy=true source-page=0 permission-widened=false ok=true
    qemu-process-address-space-smoke: mapping index=1 kind=UserData flags=RW- copy-bytes=0x<hex> zero-bytes=0x<hex> zero-before-copy=true source-page=1 permission-widened=false ok=true
    qemu-process-address-space-smoke: side-effects root-leased=true table-leases=<decimal> user-frame-leases=<decimal> mappings-installed=<decimal> copied-bytes=0x<hex> zeroed-bytes=0x<hex> scheduler-owner=false descriptors-mutated=false lower-el-frame=false runnable=false ok=true
    qemu-process-address-space-smoke: teardown phase=first mappings-released=<decimal> user-frame-releases=<decimal> table-lease-releases=<decimal> root-released=true already-destroyed=false ok=true
    qemu-process-address-space-smoke: teardown phase=second mappings-released=0 user-frame-releases=0 table-lease-releases=0 root-released=false already-destroyed=true ok=true
    qemu-process-address-space-smoke: error case=bad-install-plan errno=-EINVAL partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=null-guard-or-kernel-split errno=-EACCES partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=overlap errno=-EACCES partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=permission-widening errno=-EACCES partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=lease-exhaustion errno=-ENOMEM partial-install=false leaked-leases=false ok=true
    qemu-process-address-space-smoke: error case=copy-zero-model-failure errno=-EINVAL partial-install=false leaked-leases=false ok=true

The implementation may print additional mapping records when the accepted
ProcessImageInstallPlan spans more than two pages, and may print additional
digest, lease, rollback, teardown, or manifest fields. The required line
shapes must stay stable enough for the script gate. Hex values, counts, and
mapping totals are field placeholders because the later implementation task
owns the exact model representation.

## Failure Classification

The smoke must distinguish address-space contract failures from scenario
wiring failures:

- Contract failure: the model widens permissions, reorders mappings
  nondeterministically, publishes before all leases and mappings are complete,
  loses copy/zero accounting, accepts malformed input, reports the wrong
  errno, leaks any lease after rejection, leaves a partial record visible, or
  double-releases during teardown.
- Scenario wiring failure: the scenario cannot select
  qemu_process_address_space_smoke, cannot obtain the accepted
  ProcessImageInstallPlan, cannot retain a fresh log, cannot print the
  fixture identity line, or cannot drive success, negative, no-leak, and
  teardown observations in order.
- Regression failure: an accepted process-install, program-loader,
  read-only initramfs/VFS, user-memory, descriptor/read, or lower-EL/syscall
  gate required by this plan fails after implementation changes touch shared
  owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_process_address_space_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains qemu-process-address-space-smoke: start before
   looking for PASS.
4. Confirm the fixture identity line appears before address-space
   observations.
5. Confirm success mapping and side-effect lines appear before negative errno
   lines.
6. Confirm every negative case reports partial-install=false and
   leaked-leases=false.
7. Confirm teardown lines show first-release and second already-destroyed
   behavior.
8. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
9. Rerun the smoke script once after cleaning only stale QEMU/substitute output
   artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Regression Gates

The implementation task must retain:

- The QEMU/substitute process address-space smoke log named above.
- The command used to build and run qemu_process_address_space_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- A planned qemu-process-address-space-smoke script, or an accepted
  replacement script, that retains the required log and greps the required
  lines.
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
Pi 5 hardware behavior, boot archive publication, TFTP archive identity,
hardware page-table mutation, TTBR/TCR switch, lower-EL launch, executable
userland, shell, networking, or SSH support is claimed.

## Source Owners For Later Implementation

The later target-independent process address-space core task may touch only
these owners unless it records a narrow reason:

- A small process-address-space module for ProcessAddressSpace identity,
  owner labels, root/table lease records, user-frame leases, ordered mappings,
  copy/zero accounting, rollback, teardown, deterministic errors, and
  no-partial-install tests.
- src/process_install.rs only for reusing ProcessImageInstallPlan and
  ProcessImagePageInstallRecord accessors, or for narrowly exposing helper
  APIs required by the accepted contract.
- src/program_loader.rs only if the smoke needs stable fixture identity or
  source-digest reporting already accepted by the loader frontier.
- src/posix.rs only for reusing PosixError, user-range, null-guard, and
  UserMappingPermissions vocabulary already accepted by Phase 7 and the
  loader.
- Focused unit tests for accepted success, malformed input, split violations,
  overlap, permission widening, lease exhaustion, copy/zero model failure,
  rollback, and idempotent teardown behavior.
- Documentation and the task record needed to report evidence.

The later QEMU/substitute smoke task may also touch:

- build.rs and src/main.rs for boot-scenario routing.
- src/target/qemu_virt.rs for scenario orchestration, fixture reporting,
  required output, and final classification.
- scripts/qemu-process-address-space-smoke.sh for retained evidence.

Existing AArch64 page-table descriptor installation, TTBR/TCR switching,
lower-EL launch, process table, scheduler handoff, argv/envp, user stack,
descriptor inheritance across exec, shell, Pi 5, RP1/PCIe, UART interrupt,
DMA/cache-driver, network, and SSH owners remain out of scope for this smoke
frontier.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 hardware proof, archive publishing, power-cycle, serial observe, TFTP
  fixture delivery, and hardware-lock acquisition.
- Hardware page-table descriptor construction, TTBR0_EL1/TTBR1_EL1 switching,
  ASID/TLB policy, barrier sequencing, and address-space activation.
- Lower-EL launch of the loaded image, initial user stack, argv/envp,
  auxiliary vectors, TLS, libc startup, exec/spawn/wait, shell behavior,
  descriptor-backed filesystem syscalls, writable storage, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Next Task

The next bounded task should be
phase8-process-address-space-core-20260530 only because supervisor planning
has already queued it with explicit scope, dependencies, validation gates, and
deferred surfaces. If that task's dependencies are no longer satisfied or the
implementation boundary is not mechanically objective, the worker must request
supervisor planning instead of broadening scope.
