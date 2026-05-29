# Phase 7 Pi 5 Syscall Proof Plan

Status: accepted candidate plan for phase7-pi5-syscall-proof-plan-20260529.

## Scope

This plan defines the serialized Raspberry Pi 5 proof that may carry the
accepted QEMU production syscall routing invariant to physical hardware. It
does not change Rust or assembly code, publish a boot archive, power-cycle
hardware, observe serial output, acquire hardwareTestLock, add descriptor I/O,
byte copy-in/copy-out, pointer-taking syscalls, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, RP1/PCIe policy, UART interrupt
ownership, or DMA/cache-driver policy.

The future hardware task may touch only the source and lab surfaces needed to
stage and run the physical proof:

- build.rs for a focused rpi5_syscall_proof boot-scenario cfg.
- src/boot/rpi5.rs and src/target/rpi5.rs for focused Pi 5 scenario dispatch,
  proof orchestration, and physical serial output.
- src/arch/aarch64/exceptions.rs only if the Pi 5 lower-AArch64 synchronous SVC
  path needs the same bounded production routing already accepted by the QEMU
  syscall smoke.
- src/syscall.rs only for using the accepted stable constants and dispatch API,
  not for adding a broader syscall namespace.
- scripts/rpi5-syscall-proof-image.sh,
  scripts/rpi5-syscall-proof-boot-tree.sh, and a focused static/archive review
  helper if the implementation needs one.
- tasks/2026-05-29-phase7-pi5-syscall-proof.md and a retained evidence
  directory under tasks/evidence/.

Any broader source ownership requires supervisor planning before execution.

## Physical Invariant

The future proof must demonstrate one physical invariant on serialized
Raspberry Pi 5 hardware:

1. Talos builds a focused rpi5_syscall_proof boot scenario from the accepted
   QEMU syscall smoke boundary.
2. The kernel constructs the fixed UserText, UserStack, and UserGuard ranges
   from the accepted EL0/address-space contract and validates user ELR, user
   SP, SPSR/PSTATE, and mappings before ERET.
3. The lower-EL payload enters lower AArch64 and performs a stable production
   svc #0 with x8 = 0, expecting x0 = 0 after return.
4. After observing that return in lower EL, the payload performs a second
   stable production svc #0 with x8 = 17, expecting x0 =
   0xffffffffffffffda, the two's-complement encoding of -ENOSYS.
5. After both production syscall return observations, the payload may use the
   existing diagnostic SVC marker 0x7a10 as proof-only completion vocabulary.
   That marker must be printed as non-production and not dispatched as a
   syscall.
6. The proof reports the planned physical completion classification and PASS
   line only after both production syscall return observations and the
   diagnostic-marker quarantine check have been recorded.

The expected ESR for both production syscalls is 0x0000000054000000
(EC=0x15, AArch64 SVC, ISS=0). The expected ESR for the optional completion
marker is 0x0000000054007a10. FAR_ELx is expected to be zero for these SVC
paths unless the physical run reports a defined architectural value; any
nonzero FAR must be printed and justified in the task evidence.

## Required Output

The future hardware task must retain physical serial evidence with these exact
PASS/classification lines:

~~~text
rpi5-syscall-proof: final participants=2 expected=2 errors=0 classification=pi5-syscall-proof-complete
rpi5-syscall-proof: PASS
~~~

The retained serial log must also include these exact field names so evidence
review can distinguish production syscall dispatch from diagnostic proof:

~~~text
rpi5-syscall-proof: start
rpi5-syscall-proof: validated elr=0x0000000000100000 sp=0x0000000000200000 spsr=0x00000000000003c0 guard-blocked=true
rpi5-syscall-proof: syscall case=talos_nop vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=0 args=[x0=0x0000000000000000 x1=0x0000000000000000 x2=0x0000000000000000 x3=0x0000000000000000 x4=0x0000000000000000 x5=0x0000000000000000] return-x0=0x0000000000000000
rpi5-syscall-proof: user-observed case=talos_nop x0=0x0000000000000000 ok=true
rpi5-syscall-proof: syscall case=unknown vector=lower-aarch64-sync esr=0x0000000054000000 svc=0x0000 number=17 return-x0=0xffffffffffffffda expected=-ENOSYS
rpi5-syscall-proof: user-observed case=unknown x0=0xffffffffffffffda ok=true
rpi5-syscall-proof: diagnostic-marker marker=0x7a10 stable-syscall=false dispatched=false
~~~

The accepted PASS requires both user-observed production return lines, both
kernel syscall case lines, the diagnostic-marker quarantine line, the final
classification line, and PASS. Firmware output, target init output, old QEMU
evidence, or marker-only printing is not enough.

## Diagnostic Marker Quarantine

SVC immediate 0x7a10 remains proof vocabulary. It is not a stable syscall
immediate, syscall number, ABI version, compatibility mode, production
dispatch selector, or production success path.

The future proof may use the marker only after the stable svc #0 talos_nop and
unknown-syscall return observations have already been recorded in lower EL.
The final PASS must fail if the marker appears before the two production return
observations, if the marker is dispatched through the syscall core, or if any
production SVC uses an immediate other than 0.

## Serialized Run Requirements

The future proof task must acquire hardwareTestLock before archive publishing,
power-cycle, serial observe, candidate boot, TFTP inspection, or restoration.
The lock owner and taskId must be phase7-pi5-syscall-proof-20260529, and the
lock must be released after completion, failure, or pause.

Before publishing the candidate, the future task must retain:

- candidate source commit and git status --short.
- focused image path, kernel SHA256, kernel size, archive path, archive SHA256,
  and scripts/rpi5-archive-review.sh output or an accepted focused replacement.
- pre-run boot-tree snapshot or restore handle.
- fresh serial cursor captured before the candidate run.
- TFTP cursor captured before the candidate run.

After publishing and power-cycling, the future task must retain:

- TFTP delta showing a fresh da591740/kernel_2712.img fetch for the candidate.
- serial evidence from the fresh cursor, using repeated observe windows if one
  window reaches only firmware or early bootloader output.
- final classification and PASS, or a classified failure/inconclusive result.
- restoration record and post-restore status for the prior accepted boot tree.

The accepted proof may not depend on old serial scrollback or a stale TFTP
event. Candidate identity must be tied to both archive/kernel digests and the
fresh TFTP fetch.

## Inconclusive-Run Triage

If any candidate hardware run is inconclusive, no code changes are allowed
until this triage sequence is recorded:

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

Only after that sequence may the worker classify the result as a code issue and
change implementation. Failed hardware boots are evidence, not incidents. The
lab should be restored to the pre-run tree unless restoration itself is the
classified failure.

## Evidence Boundary

A passing Pi 5 run would accept only the physical production syscall routing
invariant listed above: stable lower-AArch64 svc #0 reaches the production
syscall dispatch core on Pi 5, talos_nop returns x0 = 0 to lower EL, unknown
syscall number 17 returns x0 = -ENOSYS to lower EL, and diagnostic marker
0x7a10 stays quarantined outside production dispatch.

It would not accept descriptor read/write/close/dup, byte copy-in/copy-out,
pointer-taking syscalls, partial copies, restart semantics, signals, resumable
user faults, per-thread errno storage, process loading, ELF parsing, argv/envp
setup, PID allocation, exit/wait, credentials, sessions, controlling TTY, VFS,
filesystem behavior, local shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

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
  observe, hardware-lock acquisition, or physical production syscall routing
  claim is made by this plan.

## Next Boundary

The next mechanically derivable task is phase7-pi5-syscall-proof-20260529,
provided this plan is accepted and hardwareTestLock is unlocked. It must acquire
the lock before any hardware action and must run exactly the serialized proof
described here.
