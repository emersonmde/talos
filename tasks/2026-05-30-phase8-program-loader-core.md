# Phase 8 Program Loader Core Task

Task: phase8-program-loader-core-20260530

Status: accepted

## Scope

Implemented the target-independent image-plan-only program loader for the
accepted narrow static ELF64/AArch64 executable subset. The implementation
parses immutable /bin/init bytes from the read-only initramfs/VFS regular-file
boundary, validates ELF identity and program headers, classifies loadable
segments, records file-copy and zero-fill ranges, checks entry-point placement,
and reports deterministic loader errors.

Non-goals honored: no QEMU program-loader smoke scenario, Pi 5 hardware run,
boot archive publication, hardwareTestLock acquisition, process address-space
installation, page-table mutation, frame allocation, lower-EL launch, user
stack, argv/envp, scheduler handoff, exec/spawn/wait, shell, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Evidence

- static inspection: git status --short before edits was clean.
- implementation paths: src/program_loader.rs, src/initramfs.rs,
  src/target/qemu_virt.rs, and src/main.rs.
- fixture identity: phase8-program-loader-elf64-aarch64-v1.
- fixture image: /bin/init immutable ELF64 little-endian AArch64 ET_EXEC bytes,
  length 516, stable digest 0x3892eed223900c65.
- image-plan evidence: unit test
  talos::program_loader::tests::phase8_init_fixture_produces_image_plan_only
  asserts path=/bin/init, identity, digest, entry=0x10100, two ordered loadable
  segments, UserText R-X file range 0x100..0x104, UserData RW- file range
  0x200..0x204, explicit BSS zero-fill 0x20204..0x21204, and total rounded
  memory footprint 0x3000.
- deterministic negative tests:
  - talos::program_loader::tests::rejects_bad_magic_before_plan
  - talos::program_loader::tests::rejects_unsupported_type_and_machine
  - talos::program_loader::tests::rejects_dynamic_interpreter_program_header
  - talos::program_loader::tests::rejects_malformed_program_header_range
  - talos::program_loader::tests::rejects_writable_executable_segment
  - talos::program_loader::tests::rejects_out_of_user_range_segment
  - talos::program_loader::tests::rejects_rounded_segment_overlap
  - talos::program_loader::tests::rejects_bad_entry_outside_executable_text
  - talos::program_loader::tests::rejects_file_range_overflow_without_partial_plan
- failure behavior: the loader returns Result<ProgramImagePlan,
  ProgramLoaderError>; all required negative cases return errors before any
  process object, scheduler task, user stack, lower-EL frame, descriptor table,
  process-owned mapping, or installation side effect exists.
- unit tests: cargo -Zjson-target-spec test passed with QEMU 9.2.0 on PATH;
  272 no_std tests passed.
- conditional regression gate: scripts/qemu-readonly-initramfs-vfs-smoke.sh
  was run because /bin/init fixture bytes and the stable VFS manifest digest
  changed; it passed with classification
  qemu-readonly-initramfs-vfs-smoke-complete and
  qemu-readonly-initramfs-vfs-smoke: PASS.
- formatting: cargo fmt --all -- --check passed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as the target-independent Milestone 8.3 program-loader core. The
loader validates the accepted static ELF64/AArch64 /bin/init fixture and
required negative cases, but it remains image-plan-only. QEMU/substitute
program-loader smoke evidence, process address-space installation, lower-EL
launch, argv/envp stack construction, exec/spawn/wait, shell behavior,
descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain blocked until later explicit
tasks accept their contracts and gates.

Commit: recorded in durable supervisor state after acceptance.
