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
- 772ef82: moved that temporary handler-entry trace from the descriptor-write
  proof handler to the active close syscall proof handler.

## Local Evidence

- fmt/lint: cargo fmt --all -- --check passed before local1.
- unit tests: cargo -Zjson-target-spec test passed, 231 tests, before local1.
- QEMU/substitute: scripts/qemu-close-syscall-smoke.sh passed before local1.
- image/archive inspection: scripts/rpi5-archive-review.sh passed for local1,
  local3, and local4 candidates.
- fmt/lint: cargo fmt --all -- --check passed after the handler-entry trace
  correction.
- unit tests: cargo -Zjson-target-spec test passed after the handler-entry
  trace correction, 231 tests, with the QEMU 9.2.0 tool path exported.

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
- local5-handler-entry-rerun: unchanged handler-entry candidate for commit
  d17b247 published and fetched, but the serial observe request used an
  oversized max_bytes value that the lab controller rejected. The run is
  retained as agent-observation failure evidence, not kernel proof evidence.
  The boot tree was restored.
- local6-handler-entry-rerun: unchanged handler-entry candidate for commit
  d17b247 published, but serial observe again used an invalid request body for
  the controller limit. The run is retained as agent-observation failure
  evidence and did not produce proof lines. The boot tree was restored.
- local7-handler-entry-rerun: unchanged handler-entry candidate for commit
  d17b247 published, fetched, and reached the close proof pre-eret line twice,
  but still produced no syscall or handler-entry lines. Inspection then found
  the temporary handler-entry trace had been inserted in the descriptor-write
  proof handler instead of the close proof handler. The boot tree was
  restored.
- local8-close-handler-entry-candidate: corrected handler-entry candidate for
  commit 772ef82 published, fetched, and retained serial evidence, but the
  collection overlapped with the earlier long-running observe/restore shell
  flow and captured only partial start/validated lines before restore. It is
  not accepted; use a clean single-controller rerun of 772ef82 next.
- local9-clean-rerun: attempted the corrected 772ef82 archive with
  controller-accepted serial observe requests. The earlier local9 shell flow
  was still running while follow-up inspection started, so its restore and
  observe windows overlapped later work. Retained TFTP evidence does show the
  114792-byte candidate kernel was served, but the serial evidence is mixed
  with restored production-timer output and is not accepted.
- local10-clean-rerun: attempted another rerun before the local9 shell session
  had fully exited. Its retained serial and TFTP evidence are contaminated by
  the overlapping local9 restore path; it is retained only as agent-flow
  failure evidence and is not accepted. The lab was restored by named snapshot
  afterward.
- local11-clean-rerun: ran after all previous shell sessions had exited, using
  the corrected 772ef82-equivalent source and the same 114792-byte archive.
  Fresh TFTP evidence shows da591740/kernel_2712.img served at 114792 bytes,
  but serial retained only firmware/network-boot output plus NUL padding and
  no TALOS, close-proof, handler-entry, syscall, classification, or PASS lines.
  The lab was restored by named snapshot to the pre-run tree hash.
- local12-known-good-control: with the pre-run snapshot restored, power-cycled
  the accepted 104136-byte production-timer control tree. Fresh TFTP evidence
  shows the 104136-byte kernel was served, and serial reached a partial
  production-timer final-classification line, but the retained capture missed
  the complete classification suffix and PASS line. Treat this as an
  inconclusive control, not an accepted control proof.

## Restore Proof

The pre-run boot tree hash was
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. Post-restore
status after local11/local12 named-snapshot restore matched that hash.

## Next Action

Do not change source after the local11 inconclusive candidate run until the
triage sequence is complete. Reacquire hardwareTestLock for a clean known-good
control rerun that captures the complete production-timer classification and
PASS lines, then rerun the unchanged corrected 772ef82-equivalent candidate
only after that control is accepted. Keep using a named snapshot restore to
avoid rollback toggling between candidate and control trees.
