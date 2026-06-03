# Phase 8 Initial Userspace Process Launch

Task ID: phase8-initial-userspace-process-launch-20260603

Status: accepted

## Scope

- Use the accepted VFS-backed program loader, process install,
  address-space, page-table materialization, initial process launch, and
  initial user-stack prerequisites for `/bin/init`.
- Implement the smallest QEMU/substitute launch boundary that enters
  `/bin/init` as EL0 code and records a real userspace execution signal.
- Keep scheduler publication, process lifecycle, broad startup ABI, shell
  behavior, Pi 5 hardware proof, writable filesystems, networking, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy out of scope.

## Outcome

- Changed the immutable `/bin/init` ELF fixture text from a `ret` placeholder
  to `svc #0x7a10`, preserving the accepted ELF64/AArch64 image shape while
  making the file executable as a deterministic first userspace signal.
- Added `qemu_initial_userspace_process_launch_smoke`, which builds the
  accepted loader/install/address-space/materialization/launch/stack chain,
  copies `/bin/init` LOAD segment bytes from the initramfs file into EL0
  mappings, maps the accepted initial user stack range, enters EL0 at the
  loader-proven `/bin/init` entry point, and handles the lower-AArch64 SVC.
- Added `scripts/qemu-initial-userspace-process-launch-smoke.sh` and retained
  the QEMU transcript under
  `tasks/evidence/2026-06-03-qemu-initial-userspace-process-launch/`.

## Evidence

- Source evidence: `src/initramfs.rs` now gives `/bin/init` a minimal SVC text
  payload; `src/target/qemu_virt.rs`, `src/main.rs`,
  `src/arch/aarch64/exceptions.rs`, and `build.rs` add the QEMU/substitute
  launch scenario.
- QEMU/substitute:
  `tasks/evidence/2026-06-03-qemu-initial-userspace-process-launch/qemu-initial-userspace-process-launch-smoke.log`
  records `/bin/init` fixture identity, accepted launch/stack boundaries,
  EL0 mappings, lower-AArch64 sync from `elr=0x0000000000010104`,
  `sp=0x0000800000000000`, `marker=0x7a10`, final
  `qemu-initial-userspace-process-launch-smoke-complete`, and PASS.
- Regression QEMU/substitute:
  `tasks/evidence/2026-06-03-qemu-program-loader-from-vfs-file/qemu-program-loader-from-vfs-smoke.log`
  was rerun after the executable init payload change.
- Unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- Typecheck:
  `TALOS_BOOT_SCENARIO=qemu_initial_userspace_process_launch_smoke cargo -Zjson-target-spec check --quiet`
  passed.
- Formatting/static inspection: `cargo fmt --all -- --check`,
  `git diff --check`, and `git diff --cached --check` passed.
- Documentation: `/home/node/.cargo/bin/mdbook build` passed.

## Deferred Surfaces

This accepts a first real userspace execution signal only. Process table/PID
lifecycle, scheduler runnable publication, context switching between process
tasks, argv/envp/auxv/TLS, descriptor inheritance, filesystem syscalls from a
loaded user program, shell behavior backed by userspace, Pi 5 hardware proof,
writable filesystems, networking, RP1/PCIe, UART interrupts, and DMA/cache
policy remain future explicit tasks.

## Commit

Recorded in durable supervisor state after acceptance.
