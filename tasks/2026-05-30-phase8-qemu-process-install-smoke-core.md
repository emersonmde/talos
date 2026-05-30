# Phase 8 QEMU Process Install Smoke Core Task

Task: phase8-qemu-process-install-smoke-core-20260530

Status: accepted

## Scope

Implemented and retained the QEMU/substitute smoke evidence required by
phase8-qemu-process-install-smoke-plan-20260530. The smoke derives the accepted
metadata-only ProcessImageInstallPlan from the immutable /bin/init
ProgramImagePlan fixture, reports ordered UserText/UserData page records, checks
exact R-X/RW- permission preservation, prints explicit file-copy and zero-fill
ranges, proves zero physical side effects, and reports deterministic
no-partial-install rejection observations.

Non-goals honored: no Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, physical frame allocation, page-table mutation,
process object creation, descriptor mutation, lower-EL frame, runnable task,
lower-EL launch, argv/envp construction, exec/spawn/wait, shell,
descriptor-backed filesystem syscalls, writable filesystem, persistent storage,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Evidence

- retained QEMU/substitute log:
  tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log.
- command used to build/run the smoke:
  scripts/qemu-process-install-smoke.sh.
- fixture identity: phase8-program-loader-elf64-aarch64-v1.
- install boundary identity: phase8-process-install-plan-v1.
- exact classification:
  qemu-process-install-smoke: final participants=7 expected=7 errors=0 classification=qemu-process-install-smoke-complete.
- exact PASS line: qemu-process-install-smoke: PASS.
- success observations: source digest 0x3892eed223900c65, entry 0x10100,
  footprint 0x3000, three install-plan pages, UserText R-X and UserData RW-
  records, allocate/copy/zero/map action order, permission-widened=false, and
  zero frame/mapping/process/descriptor/lower-EL/runnable side effects.
- deterministic rejection observations:
  - bad-plan-invariant -> -EINVAL, partial-install=false.
  - overlap -> -EACCES, partial-install=false.
  - permission-widening -> -EACCES, partial-install=false.
  - bad-entry -> -ENOEXEC, partial-install=false.
  - budget-overflow -> -ENOMEM, partial-install=false.
- unit tests: cargo -Zjson-target-spec test passed; 279 no_std tests passed,
  including the process-install success and deterministic rejection tests.
- formatting: cargo fmt --all -- --check passed.
- smoke gate: scripts/qemu-process-install-smoke.sh passed.
- conditional regression gates: scripts/qemu-program-loader-smoke.sh passed
  because src/program_loader.rs was touched to expose existing unchecked
  fixture-plan constructors to the QEMU process-install smoke cfg. Existing
  lower-EL/syscall, descriptor, read/stdin, and pointer-copy smokes were not
  run because shared syscall dispatch, descriptor tables, user-copy helpers,
  lower-EL routing, boot-scenario routing used by those smokes, and their
  diagnostic output owners were not touched.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before commit.

## Result

Accepted as the Milestone 8.3 QEMU/substitute process-install smoke core. It
proves only the metadata-only install-plan frontier and retained evidence
vocabulary. Physical process address-space installation, user frame allocation,
page-table mutation, teardown, lower-EL launch, argv/envp, exec/spawn/wait,
shell, descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked until later explicit tasks accept their
contracts and gates.

Commit: recorded in durable supervisor state after acceptance.
