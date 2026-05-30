# Phase 8 QEMU Program Loader Smoke Core Task

Task: phase8-qemu-program-loader-smoke-core-20260530

Status: accepted

## Scope

Implemented the QEMU/substitute program-loader smoke evidence for the accepted
image-plan-only loader core. The task added the qemu_program_loader_smoke boot
scenario, script gate, retained log path, and scenario reporting for the
accepted /bin/init fixture success case and required deterministic negative
cases.

Non-goals honored: no Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, process address-space installation, lower-EL
launch of the loaded image, initial user stack, argv/envp, descriptor
inheritance across exec, exec/spawn/wait, shell, descriptor-backed filesystem
syscalls, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Evidence

- static inspection: git status --short before edits was clean.
- implementation paths: build.rs, src/main.rs, src/target/qemu_virt.rs, and
  scripts/qemu-program-loader-smoke.sh.
- retained QEMU/substitute log:
  tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log.
- smoke command: scripts/qemu-program-loader-smoke.sh.
- PASS/classification evidence:
  qemu-program-loader-smoke: final participants=8 expected=8 errors=0 classification=qemu-program-loader-smoke-complete
  and qemu-program-loader-smoke: PASS.
- fixture evidence: phase8-program-loader-elf64-aarch64-v1 path=/bin/init
  digest-algorithm=stable-elf-manifest digest=0x3892eed223900c65.
- image-plan evidence: UserText R-X file-bytes=0x4, UserData RW-
  file-bytes=0x4 mem-bytes=0x1004 zero-fill=0x1000, entry 0x10100 inside
  text, and image-plan-only process-created=false stack-built=false
  descriptors-installed=false.
- negative evidence: bad-magic -ENOEXEC, dynamic-interpreter -ENOTSUP,
  wx-segment -EACCES, out-of-user-range -EACCES, overlap -EACCES, bad-entry
  -ENOEXEC, and file-range-overflow -ENOEXEC, each with partial-install=false.
- unit tests: cargo -Zjson-target-spec test passed.
- conditional regression gate: scripts/qemu-readonly-initramfs-vfs-smoke.sh
  passed because boot-scenario routing and shared QEMU diagnostic output owners
  were touched.
- formatting: cargo fmt --all -- --check passed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as the Milestone 8.3 QEMU/substitute program-loader smoke core. The
retained evidence proves only the image-plan loader frontier and deterministic
rejections. It does not claim Pi 5 hardware behavior, process launch,
process-owned mappings, user stack construction, exec/spawn/wait, shell,
descriptor-backed filesystem syscalls, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

Commit: recorded in durable supervisor state after acceptance.
