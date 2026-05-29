# Phase 7 Pi 5 Close Syscall Proof

Task: phase7-pi5-close-syscall-proof-20260529
Status: in progress

## Scope

This task is carrying the accepted QEMU/substitute talos_close invariant to
serialized Raspberry Pi 5 hardware. The current implementation adds only the
focused rpi5_close_syscall_proof boot scenario, image and boot-tree helpers,
ProcessDescriptorStore-backed close/write observations, and retained lab
evidence for the physical proof attempt.

It does not add dup/read syscalls, process loading, VFS/filesystem behavior,
shell behavior, networking, SSH, object finalization, RP1/PCIe work, UART
interrupt ownership, or DMA/cache-driver policy.

## Implementation Commits

- 1e6fcc2: added the focused Pi 5 close syscall proof scenario and staging
  helpers.
- e4a048b: mapped the proof UserData page for the close syscall scenario.
- 993f290: added a temporary handler-entry trace for the next discriminator.

## Local Evidence

- fmt/lint: cargo fmt --all -- --check passed before local1.
- unit tests: cargo -Zjson-target-spec test passed, 231 tests, before local1.
- QEMU/substitute: scripts/qemu-close-syscall-smoke.sh passed before local1.
- image/archive inspection: scripts/rpi5-archive-review.sh passed for local1,
  local3, and local4 candidates.

## Hardware Evidence

Evidence directory: tasks/evidence/2026-05-29-pi5-close-syscall-proof/.

- local1-candidate: fresh serial reached rpi5-close-syscall-proof start,
  validation, and pre-eret lines three times, then rebooted without any
  required lower-AArch64 syscall, classification, or PASS lines. The boot tree
  was restored to the pre-run hash.
- local2-known-good-control: attempted a restored-tree control run. Serial
  showed firmware/network-boot output, but no accepted Talos PASS line and no
  TFTP delta from the fresh cursor; treated as inconclusive control evidence,
  not an accepted proof.
- local3-candidate-rerun: unchanged candidate for commit e4a048b again reached
  start, validation, and pre-eret lines three times, then rebooted before the
  lower-AArch64 syscall handler lines. The boot tree was restored.
- local4-handler-entry-candidate: candidate for commit 993f290 produced only
  firmware/network-boot output in the retained window and no TFTP delta from
  the fresh cursor. The boot tree was restored.

## Restore Proof

The pre-run boot tree hash was
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. Post-restore
status after the hardware attempts matched that hash.

## Next Action

Continue bounded proof debugging from the retained local1/local3 evidence:
the candidate reaches EL1 pre-eret and then reboots before any close syscall
handler line. Reacquire hardwareTestLock only for the next serialized
candidate/control action.
