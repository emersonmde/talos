# Talos POSIX, Syscall, and Descriptor Review

Task: talos-review-posix-syscall-descriptors-20260603
Status: accepted

## Scope

Reviewed the target-independent POSIX baseline, syscall ABI/return encoding,
user-copy helpers, descriptor table/store behavior, fixed proof-stdin read
path, read-only initramfs descriptor-facing file read path, AArch64 syscall
trap helpers, and accepted Phase 7 syscall/copy/descriptor QEMU/Pi 5 proof
surfaces.

## Findings

- Fixed: SyscallReturn::error only encoded a small accepted errno subset and
  collapsed all other PosixError variants to -ENOSYS. That would make upcoming
  VFS/open/read paths report ENOENT, EISDIR, ENOTDIR, ENAMETOOLONG, ENOMEM,
  and similar real errors as "syscall not implemented". The syscall boundary
  now maps every current PosixError variant to a stable errno number and tests
  prove VFS path errors do not collapse to -ENOSYS.
- Fixed: UserRange::new rejected zero-length requests whose pointer was in the
  null guard or exactly one-past the user address-space limit. Zero-length
  copy requests should not dereference the user pointer. The copy helpers now
  accept zero-length operations without requiring a valid mapped address while
  preserving EFAULT for nonempty null-guard, kernel-space, wraparound, and
  oversized ranges.
- Removed: src/target/rpi5.rs carried a cfg-gated qemu_pointer_copy_smoke
  finisher that referenced QEMU-only pointer-copy counters and QEMU semihosting
  exit helpers. Building the QEMU pointer-copy smoke exposed it as stale
  cross-target proof plumbing. The misplaced block was removed; the Pi 5
  rpi5_pointer_copy_proof finisher remains.
- Fixed: Default cargo check emitted dead-code warnings for AArch64 syscall
  trap helpers that are deliberately used by cfg-gated QEMU/Pi 5 syscall proof
  scenarios and no_std unit tests. The helpers now carry scoped dead-code
  allowances so the default build stays warning-clean without deleting accepted
  proof hooks.
- Not an issue: DescriptorEntry still copies object references on dup rather
  than owning open-file-description refcounts. That is the accepted Phase 7
  frontier; real open-file-description sharing is introduced by the later
  descriptor-backed VFS/open/read tasks.
- Not an issue: FixedStdin remains proof input only. It is retained to protect
  accepted talos_read fd0/fd3 evidence until descriptor-backed regular-file
  reads replace it as the feature path.
- Not an issue: read-only initramfs descriptor reads already use a separate
  ReadOnlyFileDescriptions table and preserve offset/user-memory behavior on
  copy faults. The next task can build on this path without adding new feature
  scope in this review.

## Changes

- src/syscall.rs now has a complete errno table for the current PosixError
  vocabulary and tests for path/VFS error encoding.
- src/posix.rs now treats zero-length user-copy requests as no-op range
  validations and adds regression coverage for copy_from_user/copy_to_user.
- src/target/rpi5.rs no longer contains the misplaced QEMU pointer-copy smoke
  finisher.
- src/arch/aarch64/exceptions.rs marks cfg-gated syscall trap helper surfaces
  as intentionally retained when not used by the default build.
- docs/src/project/early-posix-shape.md records the current syscall errno
  mapping rule for the POSIX direction.

No VFS/open/read feature implementation, program loading, userspace execution,
hardware claim, shell behavior expansion, networking, RP1/PCIe, UART interrupt
ownership, or DMA/cache policy was added.

## Validation

- Static inspection: reviewed src/posix.rs, src/syscall.rs, src/initramfs.rs,
  src/arch/aarch64/exceptions.rs, src/target/qemu_virt.rs,
  src/target/rpi5.rs, relevant syscall/descriptor smoke scripts, Phase 7 docs,
  and prior task records with rg/sed.
- Dead-code/proof inspection: rg reviewed syscall, descriptor, copy, proof,
  cfg, and diagnostic surfaces. The stale rpi5 qemu_pointer_copy_smoke finisher
  was removed; retained proof-only surfaces are explicit QEMU/Pi 5 cfg gates.
- fmt: cargo fmt --all passed; cargo fmt --all -- --check passed.
- fmt/lint/typecheck: cargo -Zjson-target-spec check passed after scoped
  warning cleanup.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 363 no_std
  tests, including new errno and zero-length copy regressions.
- QEMU/substitute smoke: ./scripts/qemu-syscall-smoke.sh passed with
  qemu-syscall-smoke: PASS.
- QEMU/substitute smoke: ./scripts/qemu-pointer-copy-smoke.sh passed with
  qemu-pointer-copy-smoke: PASS after stale rpi5 proof cleanup.
- QEMU/substitute smoke: ./scripts/qemu-descriptor-write-smoke.sh passed with
  qemu-descriptor-write-smoke: PASS.
- QEMU/substitute smoke: ./scripts/qemu-process-descriptor-stdio-smoke.sh
  passed with qemu-process-descriptor-stdio-smoke: PASS.
- QEMU/substitute smoke: ./scripts/qemu-close-syscall-smoke.sh passed with
  qemu-close-syscall-smoke: PASS.
- QEMU/substitute smoke: ./scripts/qemu-dup-syscall-smoke.sh passed with
  qemu-dup-syscall-smoke: PASS.
- QEMU/substitute smoke: ./scripts/qemu-read-stdin-smoke.sh passed with
  qemu-read-stdin-smoke: PASS.
- QEMU/substitute smoke rerun: ./scripts/qemu-syscall-smoke.sh passed again
  after the final exception-helper warning cleanup.
- docs validation: /home/node/.cargo/bin/mdbook build passed after adding
  this task record and doc update.
- diff hygiene: git diff --check passed; git diff --cached --check pending
  before commit.
- hardwareTestLock remained unlocked/restored and unused; no hardware run was
  performed.

## Remaining Risks

- Descriptor duplication still copies descriptor entries instead of managing
  shared open-file-description lifetimes. This is acceptable for inherited
  stdio and fixed proof-stdin but must change for real VFS-backed file objects.
- FixedStdin is still a proof-only read source. The next descriptor-backed
  initramfs/VFS task should avoid extending this shim and use the regular-file
  object path instead.
- The syscall errno table now covers the current PosixError vocabulary. Future
  PosixError variants must add explicit syscall encoding tests at the same
  time they are introduced.

Review snapshot commit: 331345bf56a76557ec55cba1bf2c0f45af9aeefd.
Final acceptance commit is recorded in durable state.
