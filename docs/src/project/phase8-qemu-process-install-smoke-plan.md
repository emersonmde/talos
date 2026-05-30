# Phase 8 QEMU Process Install Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute
process-install smoke plan after the accepted
[Phase 8 Process Install Contract](phase8-process-install-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition,
process-install core implementation, process address-space mutation, user-frame
allocation, page-table installation, lower-EL launch, argv/envp setup,
process creation, exec/spawn/wait, shell behavior, descriptor-backed filesystem
syscall, writable filesystem, persistent storage, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

The purpose of this plan is to make the next metadata-only implementation task
mechanical: derive a ProcessImageInstallPlan from an already validated
ProgramImagePlan, then retain one QEMU/substitute smoke log proving success and
deterministic rejection observations without installing the image into a
process address space.

## Smoke Invariant

The first QEMU/substitute process-install smoke must demonstrate one bounded
invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_process_install_smoke.
2. The scenario consumes the accepted ProgramImagePlan for immutable /bin/init
   fixture bytes through the accepted program-loader boundary. It must not use
   descriptor-backed production open/read syscalls, writable storage, host
   filesystem state, TFTP archives, firmware-provided initramfs envelopes, or
   Pi 5 hardware artifacts.
3. The fixture identity line records the stable loader fixture name
   phase8-program-loader-elf64-aarch64-v1, /bin/init source path, loader source
   digest, and the process-install metadata boundary.
4. The success path derives one ProcessImageInstallPlan with entry preserved
   from the ProgramImagePlan, total rounded footprint preserved, and ordered
   page records for all UserText and UserData ranges.
5. Each page record preserves exact permissions: UserText is readable and
   executable but not writable; UserData is readable and writable but not
   executable. No page record may merge incompatible segment permissions or
   widen the ProgramImagePlan permissions.
6. Each page record reports clipped file-copy ranges and zero-fill ranges
   relative to the page. BSS bytes and rounded page tails must be explicit
   zero-fill records, not implicit behavior.
7. The success path reports the later action sequence as allocate, copy, zero,
   then map, but it does not allocate a frame, copy bytes into physical memory,
   create page tables, publish mappings, build a lower-EL frame, create a
   scheduler task, or make /bin/init runnable.
8. Negative cases prove deterministic rejection for a malformed
   ProgramImagePlan-equivalent request, overlapping rounded pages, permission
   widening, entry outside UserText after plan derivation, and memory-budget
   overflow.
9. Failure cases must prove no partial install: no ProcessImageInstallPlan,
   process object, frame lease, page-table mapping, descriptor mutation,
   lower-EL frame, scheduler task, or runnable state is observable.
10. The smoke prints final classification and PASS only after the success
    install-plan observations and all negative classifications have been
    recorded.

If implementation work needs a different scenario name, fixture identity,
evidence path, boundary type, negative-case matrix, PASS/classification
vocabulary, or hardware involvement, it must stop for supervisor planning
instead of accepting a changed smoke.

## Fixture And Boundary Identity

The accepted loader fixture identity remains:

    phase8-program-loader-elf64-aarch64-v1

The accepted process-install boundary identity for this smoke is:

    phase8-process-install-plan-v1

The smoke must derive the install plan from the accepted ProgramImagePlan
produced for /bin/init. The retained log must print the source digest already
owned by ProgramImagePlan and may also print an install-plan digest over a
stable textual manifest of ordered page records. The install-plan digest is
diagnostic evidence only; it is not a filesystem, exec, or ABI promise.

Required success semantics:

| Field | Required value |
| --- | --- |
| source path | /bin/init |
| loader fixture | phase8-program-loader-elf64-aarch64-v1 |
| install boundary | metadata-only ProcessImageInstallPlan |
| text page policy | UserText, R-X, copy file bytes, zero rounded tails if any |
| data page policy | UserData, RW-, copy file bytes, zero BSS and rounded tails |
| entry | preserved from ProgramImagePlan and inside UserText |
| later action order | allocate, copy, zero, map |
| output object | install plan only |
| physical side effects | none |

The fixture must not claim that /bin/init can be launched. Process-owned page
tables, user frame allocation, physical byte copy, initial stack, argv/envp,
descriptor inheritance, scheduler handoff, exec/spawn/wait, and shell behavior
remain outside this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-process-install-smoke: final participants=7 expected=7 errors=0 classification=qemu-process-install-smoke-complete
    qemu-process-install-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-process-install-smoke: start
    qemu-process-install-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init source-digest=0x<hex> install-boundary=phase8-process-install-plan-v1
    qemu-process-install-smoke: success output=ProcessImageInstallPlan metadata-only=true entry=0x<hex> entry-preserved=true footprint=0x<hex> pages=<decimal> ok=true
    qemu-process-install-smoke: page index=0 kind=UserText flags=R-X copy-offset=0x<hex> copy-len=0x<hex> zero-offset=0x<hex> zero-len=0x<hex> action-order=allocate,copy,zero,map permission-widened=false ok=true
    qemu-process-install-smoke: page index=1 kind=UserData flags=RW- copy-offset=0x<hex> copy-len=0x<hex> zero-offset=0x<hex> zero-len=0x<hex> action-order=allocate,copy,zero,map permission-widened=false ok=true
    qemu-process-install-smoke: side-effects frames-allocated=0 mappings-installed=0 process-created=false descriptors-mutated=false lower-el-frame=false runnable=false ok=true
    qemu-process-install-smoke: error case=bad-plan-invariant errno=-EINVAL partial-install=false ok=true
    qemu-process-install-smoke: error case=overlap errno=-EACCES partial-install=false ok=true
    qemu-process-install-smoke: error case=permission-widening errno=-EACCES partial-install=false ok=true
    qemu-process-install-smoke: error case=bad-entry errno=-ENOEXEC partial-install=false ok=true
    qemu-process-install-smoke: error case=budget-overflow errno=-ENOMEM partial-install=false ok=true

The implementation may print additional page records when the accepted
ProgramImagePlan spans more than two pages, and may print additional digest,
budget, clipping, source-segment, or manifest fields. The required line shapes
must stay stable enough for the script gate. Hex values and page counts are
field placeholders because the later implementation task owns the exact
install-plan representation.

## Failure Classification

The smoke must distinguish process-install contract failures from scenario
wiring failures:

- Contract failure: the metadata core widens permissions, reorders pages
  nondeterministically, loses entry or footprint identity, computes incorrect
  copy/zero-fill clipping, accepts malformed or overlapping input, reports the
  wrong errno, or leaves any partial install object after rejection.
- Scenario wiring failure: the scenario cannot select
  qemu_process_install_smoke, cannot obtain the accepted ProgramImagePlan,
  cannot retain a fresh log, cannot print the fixture identity line, or cannot
  drive the planned success and negative observations in order.
- Regression failure: an accepted program-loader, read-only initramfs/VFS,
  user-memory, descriptor/read, or lower-EL/syscall gate required by this plan
  fails after implementation changes touch shared owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_process_install_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains qemu-process-install-smoke: start before looking
   for PASS.
4. Confirm the fixture identity line appears before install-plan observations.
5. Confirm success page-record lines appear before the negative errno lines.
6. Confirm every negative case reports partial-install=false.
7. Confirm side-effect counters remain zero/false.
8. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
9. Rerun the smoke script once after cleaning only stale QEMU/substitute output
   artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Regression Gates

The implementation task must retain:

- The QEMU/substitute process-install smoke log named above.
- The command used to build and run qemu_process_install_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- A planned qemu-process-install-smoke script, or an accepted replacement
  script, that retains the required log and greps the required lines.
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
physical page allocation, page-table mutation, lower-EL launch, executable
userland, shell, networking, or SSH support is claimed.

## Source Owners For Later Implementation

The later target-independent process-install core task may touch only these
owners unless it records a narrow reason:

- A small process-install module, or a narrow program-loader-adjacent module,
  for ProcessImageInstallPlan, page records, deterministic error mapping,
  copy/zero-fill clipping, manifest/digest reporting, and no-partial-install
  tests.
- src/program_loader.rs only for reusing ProgramImagePlan and
  PlannedUserSegment accessors, or for narrowly exposing helper APIs required
  by the accepted contract.
- src/posix.rs only for reusing PosixError, user-range, null-guard, and
  UserMappingPermissions vocabulary already accepted by Phase 7 and the
  loader.
- Focused unit tests for accepted success, malformed input, overlap,
  permission widening, bad entry, budget overflow, and no-partial-install
  behavior.
- Documentation and the task record needed to report evidence.

The later QEMU/substitute smoke task may also touch:

- build.rs and src/main.rs for boot-scenario routing.
- src/target/qemu_virt.rs for scenario orchestration, fixture reporting,
  required output, and final classification.
- scripts/qemu-process-install-smoke.sh for retained evidence.

Existing lower-EL launch, process table, scheduler handoff, argv/envp,
user-stack, descriptor inheritance across exec, shell, Pi 5, RP1/PCIe, UART
interrupt, DMA/cache-driver, network, and SSH owners remain out of scope for
this smoke frontier.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 hardware proof, archive publishing, power-cycle, serial observe, TFTP
  fixture delivery, and hardware-lock acquisition.
- Physical process address-space installation, user frame allocation,
  page-table mutation, teardown, lower-EL launch of the loaded image, initial
  user stack, argv/envp, auxiliary vectors, TLS, libc startup, and shell
  behavior.
- Process creation, exec/spawn/wait, PID allocation, parent/child ownership,
  exit status, signals, credentials, close-on-exec enforcement, current
  working directory, process root, descriptor inheritance, and open-file
  description final release.
- Descriptor-backed filesystem syscalls, directory iteration, readdir/getdents,
  seek syscalls, writable filesystems, persistent storage, block devices,
  symlinks, device nodes, pipes, sockets, mmap, demand paging, copy-on-write,
  shared memory, user DMA buffers, Rust std filesystem support, networking,
  and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Next Mechanically Derivable Task

The next target-independent implementation task should be
phase8-process-install-core-20260530 if the supervisor has queued it with
explicit scope, acceptance criteria, validation gates, documentation
requirements, and evidence requirements.

Its goal should be to implement only the metadata-only
ProcessImageInstallPlan boundary, deterministic rejection cases, and focused
unit tests needed by the contract and this smoke plan. After that core is
accepted, the next QEMU/substitute evidence task should add only the
qemu_process_install_smoke scenario or substitute script, required
PASS/classification output, retained log, and regression gates described here.

Neither task may add Pi 5 hardware proof, boot archive publication, physical
page allocation, page-table mutation, lower-EL launch, argv/envp, process
creation, shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
or DMA/cache-driver policy.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted process-install
  contract, process-install source inventory, program-loader smoke-plan
  pattern, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this plan.
