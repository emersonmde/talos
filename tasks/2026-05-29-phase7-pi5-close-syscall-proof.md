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
- c03d690: recorded local13/local14 triage evidence and added a temporary
  post-dispatch trace in the active close syscall proof handler to distinguish
  a non-returning descriptor dispatch from per-case logging/state update
  failure.
- pending: after local17 reproduced the physical -EBADF return from the
  unchanged c03d690 candidate, added a temporary ProcessDescriptorStore and
  talos_close state trace around dispatch and close to distinguish missing
  current-owner state from descriptor-table close behavior.
- pending: local18 showed the EL1 close handler sees owner-present=false before
  dispatch, while EL2 initialization had already created the inherited-stdio
  owner. The next candidate cleans the ProcessDescriptorStore static to PoC
  before EL2-to-EL1/EL0 entry.

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
- fmt/lint: cargo fmt --all -- --check passed after the post-dispatch trace.
- unit tests: cargo -Zjson-target-spec test passed after the post-dispatch
  trace.
- QEMU/substitute: scripts/qemu-close-syscall-smoke.sh passed after the
  post-dispatch trace.

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
- local13-known-good-control: with the pre-run snapshot restored, power-cycled
  the accepted 104136-byte production-timer control tree. Fresh TFTP evidence
  shows da591740/kernel_2712.img served at 104136 bytes, and retained serial
  captured the complete production-timer final classification and PASS lines.
  This is the accepted known-good control for the local11/local14 triage.
- local14-unchanged-candidate-rerun: rebuilt and published the unchanged
  corrected 772ef82-equivalent 114792-byte close proof candidate. Fresh TFTP
  evidence shows da591740/kernel_2712.img served at 114792 bytes. Retained
  serial reached rpi5-close-syscall-proof start, validation, pre-eret, and the
  lower-AArch64 handler-entry line for close_stdout, but did not produce the
  dispatch-return, per-case syscall, final classification, or PASS lines. This
  completes the post-local11 triage and narrows the next discriminator to the
  close proof dispatch path after handler entry.
- local15-post-dispatch-candidate: committed the post-dispatch trace at
  c03d690, rebuilt and published the 114792-byte candidate, and reran the
  serialized Pi 5 proof. Fresh TFTP evidence shows da591740/kernel_2712.img
  served at 114792 bytes. Retained serial again reached start, validation,
  pre-eret, lower-AArch64 handler-entry for close_stdout, and the new
  dispatch-return line. The dispatch returned x0=0xfffffffffffffff7 (-EBADF)
  for close_stdout instead of 0, then produced no per-case syscall line, final
  classification, or PASS. This moves the physical failure from exception
  routing into the process-descriptor dispatch/store state used by
  close_stdout.
- local16-known-good-control: refreshed the post-local15 inconclusive-run
  triage without source changes. The restored production-timer control fetched
  the accepted 104136-byte da591740/kernel_2712.img and retained complete
  pi5-production-timer-preemption final classification and PASS lines.
- local17-unchanged-post-dispatch-candidate-rerun: republished the unchanged
  c03d690-equivalent 114792-byte candidate from the retained local15 archive.
  Fresh TFTP evidence shows da591740/kernel_2712.img served at 114792 bytes,
  and retained serial again reached close proof start/validation/pre-eret,
  lower-AArch64 handler entry, and dispatch-return with x0 =
  0xfffffffffffffff7 (-EBADF) for close_stdout. No per-case syscall, final
  classification, or PASS line appeared. The boot tree was restored by the
  pre-pi5-close-syscall-proof-local1-20260529 snapshot.
- local18-store-state-discriminator: built and published the eb2af12
  descriptor-store/talos_close trace candidate. Fresh TFTP evidence shows
  da591740/kernel_2712.img served at 114792 bytes. Retained serial reached the
  lower-AArch64 close handler and printed store-before-dispatch owner=1
  owner-present=false table-error=EBADF, followed by talos-close-entry/result
  owner-present=false and dispatch-return -EBADF. The boot tree was restored by
  the pre-pi5-close-syscall-proof-local1-20260529 snapshot. This narrows the
  failure to visibility of the initialized ProcessDescriptorStore static at the
  EL1 handler boundary, before talos_close mutates the table.

## Restore Proof

The pre-run boot tree hash was
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. Post-restore
status after local11/local12 named-snapshot restore matched that hash.

## Next Action

Do not change source after the local15 inconclusive candidate until the
inconclusive-run triage sequence is refreshed for c03d690: retain candidate
identity, fresh serial cursor, TFTP delta, a clean known-good production-timer
control, and an unchanged c03d690 candidate rerun. If that unchanged rerun still
returns -EBADF for close_stdout, the next source discriminator should
instrument the physical ProcessDescriptorStore/current-owner table state before
dispatch_process_descriptor and inside talos_close.

That triage is now refreshed by local16/local17. Continue with the bounded
ProcessDescriptorStore/talos_close state discriminator and do not mark the proof
accepted unless the required close syscall PASS/final lines are retained.

The discriminator completed in local18 and points to pre-dispatch owner-table
visibility. Continue with a minimal cache-clean fix for the initialized
ProcessDescriptorStore static, retaining the temporary traces until a passing
physical proof proves they are no longer needed.
