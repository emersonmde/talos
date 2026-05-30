# Phase 8 Process Install Core Task

Task: phase8-process-install-core-20260530

Status: accepted

## Scope

Implemented the target-independent metadata-only ProcessImageInstallPlan
boundary accepted by phase8-process-install-contract-20260530. The core derives
ordered page install records from a validated ProgramImagePlan, preserving
fixture identity, source digest, entry point, total rounded footprint,
UserText/UserData permissions, clipped file-copy ranges, explicit zero-fill
ranges, and the future action order allocate/copy/zero/map.

Non-goals honored: no QEMU process-install smoke, Pi 5 hardware run, boot
archive publication, hardwareTestLock acquisition, frame allocation, physical
byte copy, page-table mutation, process object, descriptor mutation, lower-EL
frame, runnable task, argv/envp construction, exec/spawn/wait, shell,
descriptor-backed filesystem syscall, writable filesystem, persistent storage,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Evidence

- static inspection: git status --short before edits was clean.
- implementation paths: src/process_install.rs, src/program_loader.rs, and
  src/main.rs.
- documentation paths: docs/src/roadmap.md, docs/src/decisions/README.md, and
  this task record.
- fixture identity: phase8-program-loader-elf64-aarch64-v1.
- install boundary identity: phase8-process-install-plan-v1.
- success unit test:
  talos::process_install::tests::derives_metadata_only_install_plan_from_fixture
  asserts source path /bin/init, source digest 0x3892eed223900c65, preserved
  entry 0x10100, footprint 0x3000, three ordered page records, R-X UserText,
  RW- UserData, clipped copy ranges, explicit zero-fill ranges, zero side
  effects, lower-EL launch blocked, and allocate/copy/zero/map action
  metadata.
- deterministic negative tests:
  - talos::process_install::tests::rejects_missing_segment_slot_without_partial_install
  - talos::process_install::tests::rejects_overlapping_pages_without_partial_install
  - talos::process_install::tests::rejects_permission_widening_without_partial_install
  - talos::process_install::tests::rejects_bad_entry_without_partial_install
  - talos::process_install::tests::rejects_budget_overflow_without_partial_install
  - talos::process_install::tests::rejects_source_range_overflow_without_partial_install
- failure behavior: every rejection returns Err before a ProcessImageInstallPlan
  is produced; this metadata-only boundary has no frame lease, mapping,
  process object, descriptor mutation, lower-EL frame, or runnable task side
  effect to unwind.
- unit tests: cargo -Zjson-target-spec test passed with QEMU 9.2.0 on PATH;
  279 no_std tests passed.
- formatting: cargo fmt --all -- --check passed.
- conditional QEMU regression: scripts/qemu-program-loader-smoke.sh passed
  because src/program_loader.rs was touched for test-only malformed-plan
  constructors; retained output reported
  classification=qemu-program-loader-smoke-complete and
  qemu-program-loader-smoke: PASS.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as the Milestone 8.3 metadata-only process-install core. It proves the
first install-plan data model and deterministic no-partial-install rejection
behavior only. QEMU/substitute process-install smoke evidence, process-owned
address-space mutation, frame allocation, page-table installation, lower-EL
launch, argv/envp construction, process creation, exec/spawn/wait, shell,
descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked until later explicit tasks accept their
contracts and gates.

Commit: recorded in durable supervisor state after acceptance.
