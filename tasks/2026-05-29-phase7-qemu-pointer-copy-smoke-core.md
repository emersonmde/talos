# Phase 7 QEMU Pointer-Copy Smoke Core

Task: phase7-qemu-pointer-copy-smoke-core-20260529
Status: accepted

## Scope

This implementation task added only the QEMU/substitute pointer-copy smoke for
the proof-only talos_copy_probe boundary defined by the accepted contract and
smoke plan. It did not add Pi 5 hardware proof, hardwareTestLock acquisition,
boot archive publication, descriptor read/write syscalls, runtime console or
TTY integration, process loading, VFS/filesystem behavior, path copying, shell
behavior, networking, SSH, RP1/PCIe work, UART interrupt ownership,
DMA/cache-driver policy, demand paging, copy-on-write, signal/restart
semantics, or lower-EL fault-table recovery.

## Changed Files

- build.rs: registered TALOS_BOOT_SCENARIO=qemu_pointer_copy_smoke and reused
  the existing syscall-return assembly path for recoverable lower-EL SVC
  handling.
- src/main.rs: routed the new QEMU boot scenario and excluded it from the
  default QEMU timer smoke fallback.
- src/arch/aarch64/exceptions.rs: allowed the QEMU pointer-copy smoke to use
  the recoverable syscall exception handler shape.
- src/syscall.rs: added proof-only talos_copy_probe helper dispatch under the
  QEMU pointer-copy smoke/unit-test configuration while preserving x8 = 0x7001
  as -ENOSYS for normal syscall dispatch.
- src/target/qemu_virt.rs: added the qemu_pointer_copy_smoke payload, UserData
  backing storage mapping, success/EFAULT/unknown observations, diagnostic
  marker quarantine, and final classification/PASS output.
- scripts/qemu-pointer-copy-smoke.sh: added the focused QEMU gate and retained
  log copy.
- docs/src/roadmap.md and docs/src/decisions/README.md: recorded the accepted
  QEMU/substitute pointer-copy frontier and deferred surfaces.

## Accepted Evidence

- static inspection: git status --short before edits was clean.
- unit tests: cargo -Zjson-target-spec test passed with 208 no_std tests.
- QEMU/substitute smoke: scripts/qemu-pointer-copy-smoke.sh passed.
- retained QEMU evidence:
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log.
- retained classification lines:
  qemu-pointer-copy-smoke: final participants=3 expected=3 errors=0
  classification=qemu-pointer-copy-smoke-complete; qemu-pointer-copy-smoke:
  PASS.
- QEMU/substitute regression: scripts/qemu-syscall-smoke.sh passed.
- QEMU/substitute regression: scripts/qemu-el0-trap-smoke.sh passed.
- formatting: cargo fmt --all -- --check passed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Capability

The accepted capability is only QEMU/substitute evidence that lower-EL stable
svc #0 can route a proof-only x8 = 0x7001 talos_copy_probe request through the
saved-frame syscall path, invoke the accepted copy_from_user and copy_to_user
helpers against explicit UserData mapping/backing storage, return x0 = 16 for
the 16-byte 0x2a-to-0xa5 success case, return -EFAULT for the guard-range
case, preserve unknown-syscall -ENOSYS behavior, and keep diagnostic marker
0x7a10 outside syscall dispatch.

## Deferred Work

Pi 5 pointer-copy hardware proof, descriptor read/write/close/dup syscalls,
runtime console or TTY-backed stdio, process loading, process-owned address
spaces, VFS/filesystem behavior, path copying, argv/envp loading, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, demand paging, copy-on-write, signal/restart
semantics, and lower-EL fault-table recovery remain blocked until later
explicit tasks.

## Next Action

phase7-pointer-copy-closeout-checkpoint-20260529 is mechanically unblocked for
the next worker wake if durable state and the working tree remain compatible.
That task should reconcile the accepted contract, plan, core implementation,
retained QEMU evidence, regression gates, and deferred surfaces before any Pi 5
pointer-copy proof or descriptor I/O planning.
