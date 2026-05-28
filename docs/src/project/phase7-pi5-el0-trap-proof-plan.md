# Phase 7 Pi 5 EL0 Trap Proof Plan

Status: accepted candidate plan for phase7-pi5-el0-trap-proof-plan-20260528.

## Scope

This plan defines the serialized Raspberry Pi 5 proof that may carry the
accepted QEMU EL0 trap smoke invariant to physical hardware. It does not change
Rust or assembly code, publish a boot archive, power-cycle hardware, observe
serial output, acquire hardwareTestLock, define a syscall ABI, add process
loading, descriptor I/O, VFS/filesystem behavior, shell behavior, networking,
SSH, RP1/PCIe policy, UART interrupt ownership, or DMA/cache-driver policy.

The future hardware task may touch only the source and lab surfaces needed to
stage and run the physical proof:

- build.rs for the new rpi5_el0_trap_proof boot-scenario cfg.
- src/boot/rpi5.rs and src/target/rpi5.rs for the focused Pi 5 scenario
  dispatch and proof routine.
- src/arch/aarch64/exceptions.rs only if the Pi 5 lower-EL trap path needs the
  same bounded diagnostic exception routing used by the QEMU smoke.
- scripts/rpi5-el0-trap-proof-image.sh and
  scripts/rpi5-el0-trap-proof-boot-tree.sh for focused image and archive
  staging.
- tasks/2026-05-28-phase7-pi5-el0-trap-proof.md and a retained evidence
  directory under tasks/evidence/.

Any broader source ownership requires supervisor planning before execution.

## Physical Invariant

The future proof must demonstrate one physical invariant on serialized
Raspberry Pi 5 hardware:

1. Talos builds a focused rpi5_el0_trap_proof boot scenario from the accepted
   QEMU EL0 trap smoke implementation boundary.
2. The kernel constructs the fixed UserText, UserStack, and UserGuard ranges
   from the Phase 7.2 contract, validates the selected user ELR, SP, SPSR, and
   mappings, and rejects the guard range through the accepted user-memory
   validation primitive before ERET.
3. A built-in diagnostic EL0 payload executes only SVC marker 0x7a10. The
   marker is diagnostic evidence, not a syscall ABI.
4. The lower-AArch64 synchronous trap path saves vector, ESR, FAR, ELR, SP,
   SPSR, and marker fields without routing through same-EL diagnostics.
5. The proof reports the planned physical completion classification and PASS
   line, then stops or returns only through the focused proof harness.

The expected physical serial evidence must include these lines, with hex fields
matching the captured physical run:

```text
rpi5-el0-trap-proof: start
rpi5-el0-trap-proof: validated elr=0x0000000000100000 sp=0x0000000000200000 spsr=0x00000000000003c0 guard-blocked=true
rpi5-el0-trap-proof: trap vector=lower-aarch64-sync esr=0x0000000054007a10 far=<hex> elr=0x0000000000100004 sp=0x0000000000200000 spsr=0x00000000000003c0 marker=0x7a10
rpi5-el0-trap-proof: final participants=1 expected=1 errors=0 classification=pi5-el0-trap-proof-complete
rpi5-el0-trap-proof: PASS
```

The accepted PASS requires both the saved-state trap line and the final
classification/PASS lines. Firmware output, target init output, or marker-only
printing is not enough.

## Serialized Run Requirements

The future proof task must acquire hardwareTestLock before archive publishing,
power-cycle, serial observe, candidate boot, TFTP inspection, or restoration.
The lock owner and taskId must be phase7-pi5-el0-trap-proof-20260528, and the
lock must be released after completion, failure, or pause.

Before publishing the candidate, the future task must retain:

- git commit and git status --short for candidate identity.
- focused image path, kernel SHA256, kernel size, archive path, archive
  SHA256, and scripts/rpi5-archive-review.sh output.
- pre-run boot tree snapshot or restore handle.
- fresh serial cursor captured before the candidate run.
- TFTP cursor captured before the candidate run.

After publishing and power-cycling, the future task must retain:

- TFTP delta showing a fresh da591740/kernel_2712.img fetch for the candidate.
- serial evidence from the fresh cursor, using repeated observe windows if one
  window reaches only firmware or early bootloader output.
- final classification and PASS, or a classified failure/inconclusive result.
- restoration record and post-restore status.

The accepted proof may not depend on old serial scrollback or a stale TFTP
event. Candidate identity must be tied to both archive/kernel digests and the
fresh TFTP fetch.

## Inconclusive-Run Triage

If the first candidate hardware run is inconclusive, no code changes are
allowed until this triage sequence is recorded:

1. Candidate identity: commit, git status, archive SHA256, kernel SHA256,
   kernel size, and archive-review output.
2. Fresh serial cursor: prove the observe window starts after the candidate
   publish/power-cycle point.
3. TFTP delta: prove the Pi fetched the candidate kernel_2712.img after the
   candidate publish.
4. Known-good control: restore and run an accepted known-good Pi 5 proof or
   baseline to verify lab health, unless the failure is already a clear Talos
   proof failure with complete candidate fetch and serial evidence.
5. Candidate rerun: republish or rerun the same candidate with fresh serial and
   TFTP cursors before changing code.

Only after that sequence may the worker classify the result as a code issue
and change implementation. Failed hardware boots are evidence, not incidents.
The lab should be restored to the pre-run tree unless restoration itself is the
classified failure.

## Evidence Boundary

A passing Pi 5 run would accept only the physical lower-EL diagnostic trap
invariant listed above. It would not accept a general SVC/syscall ABI, syscall
dispatch table, numeric errno return convention, restart convention,
per-thread errno storage, process loading, ELF parsing, argument/environment
setup, process exit/wait, signals, resumable user faults, copy-in/copy-out
implementation, descriptor I/O, VFS/filesystem behavior, stdio TTY
integration, local shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
or DMA/cache-driver policy.

QEMU/substitute evidence remains useful for regression comparison, but only
serial hardware boot/output with candidate identity, fresh TFTP evidence,
hardwareTestLock ownership, and restoration proof can establish the Pi 5
claim.

## Validation

- static inspection: git status --short before edits must be clean or record
  unrelated changes.
- static inspection: git diff --check must pass.
- documentation: mdbook build must pass for this plan.
- hardware: no Pi 5 hardware run, archive publication, power-cycle, serial
  observe, hardware-lock acquisition, or physical lower-EL claim is made by
  this plan.

## Next Boundary

The next mechanically derivable task is
phase7-pi5-el0-trap-proof-20260528, provided hardwareTestLock is unlocked. It
must acquire the lock before any hardware action and must run exactly the
serialized proof described here.
