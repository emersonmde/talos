# Phase 8 QEMU Program Loader Smoke Plan

Status: accepted as the documentation-only Milestone 8.3 QEMU/substitute
program-loader smoke plan after the accepted
[Phase 8 Program Loader Format Contract](phase8-program-loader-format-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition, loader
core, process creation, argv/envp setup, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

The purpose of this plan is to make the next loader implementation task
mechanical: add the target-independent executable image validator for the
accepted narrow static ELF64/AArch64 subset, then add one QEMU/substitute
smoke that proves success and deterministic rejection cases before any Pi 5
proof is considered.

## Smoke Invariant

The first QEMU/substitute program-loader smoke must demonstrate one bounded
invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_program_loader_smoke.
2. The scenario consumes immutable regular-file bytes from the accepted
   read-only initramfs/VFS fixture boundary. It must not use descriptor-backed
   production open/read syscalls, writable storage, host filesystem state,
   TFTP archives, firmware-provided initramfs envelopes, or Pi 5 hardware
   artifacts.
3. The fixture identity line records a stable fixture name and deterministic
   digest for the exact executable bytes and expected image plan.
4. The primary success fixture is a minimal static ELF64 little-endian
   AArch64 ET_EXEC image named /bin/init. It has no PT_INTERP, no PT_DYNAMIC,
   no relocation requirement, at least one PT_LOAD UserText segment, one
   PT_LOAD UserData/BSS segment, and an entry point inside loaded executable
   text.
5. The validator checks the ELF magic, class, data encoding, version, type,
   machine, ELF header size, program-header entry size, program-header range,
   and at least one loadable segment before any image-plan success line.
6. The validator emits an image plan with source path, source digest,
   entry-point address, ordered segment count, text/data permission
   classification, file-copy ranges, zero-fill ranges, and total planned
   memory footprint.
7. The success case proves that R+X text maps to UserText, R+W data/BSS maps
   to UserData, W+X is absent, zero-fill is explicit, and the entry point is
   AArch64-aligned inside UserText.
8. Negative cases prove deterministic rejection for malformed magic,
   unsupported dynamic interpreter, W+X segment permissions, segment outside
   the accepted user range, overlapping PT_LOAD ranges, entry outside
   executable text, and file-range overflow.
9. Failure cases must prove that no partial image install, process object,
   scheduler task, descriptor table, initial stack, or lower-EL frame is
   observable.
10. The smoke prints final classification and PASS only after the success
    image-plan observations and all negative classifications have been
    recorded.

If implementation work needs a different scenario name, fixture path,
executable format, dynamic-linking policy, user-address window, success
segment layout, required negative cases, retained evidence path, or required
PASS/classification vocabulary, it must stop for supervisor planning instead
of accepting a changed smoke.

## Fixture Identity

The accepted fixture name is:

    phase8-program-loader-elf64-aarch64-v1

The success fixture should replace the current data-only /bin/init bytes only
inside a later accepted implementation task. The fixture should remain small
enough to inspect as bytes or as a generated constant, and it must be
deterministic across host, QEMU, and documentation builds.

The implementation may build the fixture from checked-in bytes, a small
checked-in generator, or no_std test constants. It must print the digest
algorithm and digest value in the retained log. The recommended digest input
is the exact ELF byte stream plus a stable textual image-plan manifest.

Required success semantics:

| Field | Required value |
| --- | --- |
| path | /bin/init |
| format | elf64-aarch64-static-et-exec |
| text segment | R+X, not W, UserText |
| data/BSS segment | R+W, not X, UserData |
| entry | canonical user VA inside UserText |
| zero-fill | explicit nonzero BSS range |
| dynamic features | absent |
| output object | image plan only |

The fixture must not claim that /bin/init can be launched. Process-owned page
tables, user frame allocation, initial stack, argv/envp, descriptor
inheritance, scheduler handoff, exec/spawn/wait, and shell behavior remain
outside this smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-program-loader-smoke: final participants=8 expected=8 errors=0 classification=qemu-program-loader-smoke-complete
    qemu-program-loader-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-program-loader-smoke: start
    qemu-program-loader-smoke: fixture name=phase8-program-loader-elf64-aarch64-v1 path=/bin/init digest-algorithm=stable-elf-manifest digest=0x<hex>
    qemu-program-loader-smoke: success format=elf64-aarch64-static-et-exec type=ET_EXEC machine=EM_AARCH64 phdrs=2 loadable=2 dynamic=false relocations=false ok=true
    qemu-program-loader-smoke: segment index=0 kind=UserText flags=R-X file-bytes=0x<hex> mem-bytes=0x<hex> zero-fill=0x0 wx=false ok=true
    qemu-program-loader-smoke: segment index=1 kind=UserData flags=RW- file-bytes=0x<hex> mem-bytes=0x<hex> zero-fill=0x<hex> wx=false ok=true
    qemu-program-loader-smoke: entry va=0x<hex> in-user=true in-text=true aligned=true ok=true
    qemu-program-loader-smoke: image-plan source=/bin/init output=image-plan-only process-created=false stack-built=false descriptors-installed=false ok=true
    qemu-program-loader-smoke: error case=bad-magic errno=-ENOEXEC partial-install=false ok=true
    qemu-program-loader-smoke: error case=dynamic-interpreter errno=-ENOTSUP partial-install=false ok=true
    qemu-program-loader-smoke: error case=wx-segment errno=-EACCES partial-install=false ok=true
    qemu-program-loader-smoke: error case=out-of-user-range errno=-EACCES partial-install=false ok=true
    qemu-program-loader-smoke: error case=overlap errno=-EACCES partial-install=false ok=true
    qemu-program-loader-smoke: error case=bad-entry errno=-ENOEXEC partial-install=false ok=true
    qemu-program-loader-smoke: error case=file-range-overflow errno=-ENOEXEC partial-install=false ok=true

The implementation may print additional header, program-header, alignment,
range, digest, or memory-budget fields, but these required lines must stay
stable enough for the script gate. The hex values are intentionally specified
as fields rather than fixed constants because the implementation task owns the
exact fixture bytes and bounded address window.

## Failure Classification

The smoke must distinguish loader-contract failures from scenario wiring
failures:

- Contract failure: the validator accepts a rejected ELF identity, class,
  dynamic feature, segment range, W+X permission, overlap, malformed file
  range, bad entry, or zero-fill condition, or it emits an image plan that
  contradicts the accepted contract.
- Scenario wiring failure: the scenario cannot select
  qemu_program_loader_smoke, cannot obtain immutable /bin/init fixture bytes
  through the accepted read-only initramfs/VFS boundary, cannot retain a fresh
  log, cannot print the fixture identity line, or cannot drive the planned
  success and negative observations in order.
- Regression failure: an accepted user-memory, read-only initramfs/VFS,
  descriptor/read, or lower-EL/syscall gate required by this plan fails after
  implementation changes touch shared owners.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_program_loader_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains qemu-program-loader-smoke: start before looking
   for PASS.
4. Confirm the fixture identity line appears before the success image-plan
   observations.
5. Confirm success image-plan lines appear before the negative errno lines.
6. Confirm every negative case reports partial-install=false.
7. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
8. Rerun the smoke script once after cleaning only stale QEMU/substitute
   output artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Regression Gates

The implementation task must retain:

- The QEMU/substitute program-loader smoke log named above.
- The command used to build and run qemu_program_loader_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- scripts/qemu-program-loader-smoke.sh or an accepted replacement script that
  retains the required log and greps the required lines.
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
process launch, executable userland, shell, networking, or SSH support is
claimed.

## Source Owners For Later Implementation

The later target-independent loader core task may touch only these owners
unless it records a narrow reason:

- A small program-loader module for ELF64/AArch64 header parsing, program
  header validation, image-plan construction, segment permission
  classification, zero-fill planning, entry validation, and deterministic
  loader errors.
- src/initramfs.rs only for replacing or adding deterministic /bin/init
  fixture bytes under the accepted fixture identity, or for exposing immutable
  regular-file bytes through the accepted read-only fixture boundary.
- src/posix.rs only for reusing PosixError, accepted user-range and
  user-permission vocabulary, or narrowly exposed helper APIs needed by the
  loader contract.
- Focused unit tests for accepted success, malformed ELF identity, dynamic
  interpreter rejection, W+X rejection, range/overlap rejection, entry
  rejection, file-range overflow, and zero-fill planning.
- Documentation and the task record needed to report evidence.

The later QEMU/substitute smoke task may also touch:

- build.rs and src/main.rs for boot-scenario routing.
- src/target/qemu_virt.rs for scenario orchestration, fixture reporting,
  required output, and final classification.
- scripts/qemu-program-loader-smoke.sh for retained evidence.

Existing process table, scheduler process-install, argv/envp, user-stack,
descriptor inheritance across exec, shell, Pi 5, RP1/PCIe, UART interrupt,
DMA/cache-driver, network, and SSH owners remain out of scope for this smoke
frontier.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 hardware proof, archive publishing, power-cycle, serial observe, TFTP
  fixture delivery, and hardware-lock acquisition.
- Process address-space installation, user frame allocation, page-table
  mutation, teardown, lower-EL launch of the loaded image, initial user stack,
  argv/envp, auxiliary vectors, TLS, libc startup, and shell behavior.
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
phase8-program-loader-core-20260530 if the supervisor has queued it with
explicit scope, acceptance criteria, validation gates, documentation
requirements, and evidence requirements.

Its goal should be to implement only the accepted loader validator and image
plan for the narrow static ELF64/AArch64 fixture, deterministic negative
cases, and focused unit tests needed by this smoke plan. After that core is
accepted, the next QEMU/substitute evidence task should add only the
qemu_program_loader_smoke scenario or substitute script, required
PASS/classification output, retained log, and regression gates described here.

Neither task may add Pi 5 hardware proof, boot archive publication, process
creation, lower-EL launch of a loaded image, argv/envp setup, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: used the accepted program-loader format
  contract, program-loader source inventory, read-only initramfs/VFS smoke
  plan pattern, roadmap, and ADR index.
- static documentation diff: added this smoke plan, linked it from SUMMARY,
  updated roadmap Phase 8 status, updated the decision log, and added the task
  record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, Pi 5 hardware runs, archive publication, and
  hardwareTestLock acquisition were not required because this task changes only
  Markdown documentation and durable worker state.
