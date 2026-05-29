# Phase 7 Pi 5 EL0 Trap Proof

## Task

- Title: Phase 7 Pi 5 EL0 trap proof
- Owner: worker
- Date: 2026-05-28
- Milestone: Phase 7.2, EL0 Trap Path and User Address Spaces
- Scope: serialized physical Pi 5 proof for the accepted EL0 diagnostic trap invariant

## Current Status

In progress. The focused Pi 5 boot scenario, image helper, and boot-tree helper
are implemented. QEMU/substitute and local build gates pass, and the Pi 5 lab
has repeatedly reached the planned rpi5-el0-trap-proof validated line with a
fresh candidate fetch. Same-candidate triage after local12 completed with a
fresh accepted production-timer control and an unchanged b3aadd08/5dcd4b71
rerun. Staging EL1 entry with SCTLR_EL1.M clear moved the hardware boundary:
Pi 5 now reaches the EL1 assembly marker and Rust EL1-entry callback, proving
the EL2-to-EL1 ERET target executes. The local19 triage sequence was completed
with a fresh production-timer control and unchanged candidate rerun before the
next code change. A follow-up candidate added EL1 assembly markers after
SCTLR_EL1.M enable and immediately before EL0 ERET; physical evidence still
does not reach the post-MMU marker. Same-candidate triage after local22
completed with a fresh production-timer PASS and unchanged candidate rerun.
Moving the BCM2712 MMIO mapping to the EL1 translation-table slot and then
trying SCTLR_EL1.M without enabling EL1 caches did not move the physical
boundary: Pi 5 still reaches the EL1 assembly marker and Rust EL1-entry
callback, then stops before the post-MMU marker, lower-AArch64 trap saved-state
line, classification=pi5-el0-trap-proof-complete, or rpi5-el0-trap-proof:
PASS. The supervisor-required local28 triage is complete: local29 proved lab
health with the accepted production-timer control, and local30 reran the
unchanged local28 candidate from a fresh serial cursor with fresh 97,133-byte
candidate TFTP fetches and reproduced the same no-post-MMU-marker boundary. No
physical PASS is accepted yet.

The static checker and minimal identity-map/fault-vector discriminator are now
implemented. Local gates pass, and the latest retained local54/local56 hardware
candidate identity is archive SHA256 722a09c9..., kernel SHA256 3cb610af...,
kernel size 96,065 bytes. The static checker proves the discriminator handoff,
EL1 proof vector, regular vectors, callback, stack, and UART10 index, and the
latest variant maps the linked kernel-through-stack range through 4 KiB page
descriptors instead of a 2 MiB executable block. Pi 5 local56 reran the
unchanged page-descriptor candidate and reached start, validated, pre-ERET,
identity-entered-el1, the Rust entered-el1 callback, pre-M=1 PAR lines,
identity-after-sctlr-msr, and identity-after-sctlr-dsb. It still did not emit
the literal-free post-ISB marker, el1-fault-vector, lower-AArch64 trap,
classification, or PASS. A fresh local55 production-timer control reached
classification=pi5-production-timer-preemption-complete and PASS, proving lab
health after the page-descriptor discriminator. No physical EL0 trap PASS is
accepted yet.

## Work Performed

- Added the rpi5_el0_trap_proof boot scenario.
- Added target::rpi5::run_el0_trap_proof() and the focused lower-EL
  diagnostic exception handler.
- Added scripts/rpi5-el0-trap-proof-image.sh and
  scripts/rpi5-el0-trap-proof-boot-tree.sh.
- Mirrored the QEMU proof's fixed UserText, UserStack, UserGuard, diagnostic
  SVC marker 0x7a10, saved-state reporting, and PASS/classification contract.
- Added focused Pi 5 pre-ERET register reporting for HCR_EL2, SCTLR_EL1,
  TCR_EL1, TTBR0_EL1, VBAR_EL1, ELR_EL1, and SPSR_EL1.
- Added a Pi 5 proof-only EL1-entry callback in the EL1-then-EL0 handoff path
  so the next hardware run can distinguish EL2-to-EL1 ERET failure from later
  EL1-to-EL0 failure.
- Added a Pi 5 proof-only assembly UART marker immediately after the EL2-to-EL1
  ERET target to test whether execution reaches EL1 before any Rust callback.
- Staged EL1 entry with SCTLR_EL1.M clear, then enables EL1 translation in the
  EL1 handoff path before the EL0 ERET.
- Corrected the proof page-table placement for the BCM2712 MMIO table from the
  L1 slot to the low L2 slot used by UART10's VA.
- Added Pi 5 proof-only EL1 assembly UART markers after SCTLR_EL1.M enable and
  immediately before EL0 ERET to isolate whether the remaining reset happens at
  EL1 translation enable or the EL0 transition.
- Corrected the BCM2712 MMIO mapping to the EL1 L1 index for the
  0x10_7c00_0000 VA range used after enabling EL1 translation.
- Tried an EL1 handoff variant that enables SCTLR_EL1.M without enabling
  SCTLR_EL1.C or SCTLR_EL1.I in the same step.
- Added scripts/rpi5-el0-trap-proof-static-check.sh to inspect the focused
  ELF and prove the minimal discriminator's handoff, EL1 vector, regular
  vectors, callback, stack, and UART10 index before archive publication.
- Added a proof-only EL1 identity-map discriminator that maps the kernel
  execution/stack block and BCM2712 UART window, installs a formatter-free EL1
  fault vector at VBAR_EL1, enters EL1 with SCTLR_EL1.M clear, then enables
  SCTLR_EL1.M before any EL0 transition.
- Replaced the proof EL1 executable identity block with 4 KiB page descriptors
  for the linked kernel-through-stack range to test descriptor legality at the
  ISB boundary without changing the EL0/syscall contract.

## Evidence

Evidence directory: tasks/evidence/2026-05-28-pi5-el0-trap-proof/.

- local1-candidate: candidate archive df95103d..., kernel SHA256
  a4038dc1..., kernel size 96,664 bytes. TFTP served
  da591740/kernel_2712.img at 96,664 bytes. Serial reached start and
  validated lines only. Result: inconclusive.
- local2-known-good-control: restored control tree
  6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef.
  Serial tail retained classification=pi5-production-timer-preemption-complete
  and rpi5-production-timer-preemption: PASS, proving lab health.
- local3-candidate-rerun: unchanged candidate rerun after the known-good
  control. Serial from a fresh cursor again reached the validated line only.
- local4-candidate-hcr-rw: after setting HCR_EL2.RW, local gates passed and
  the candidate again reached only the validated line. The lab was restored.
- local5-known-good-control-after-hcr-rw: restored control tree again reached
  classification=pi5-production-timer-preemption-complete and PASS.
- local6-candidate-hcr-rw-rerun: unchanged rerun after that control again
  reached only the validated line.
- local7-candidate-sctlr-res1: after setting EL1 SCTLR RES1 bits and clearing
  WXN, local gates passed and the candidate still reached only the validated
  line. The lab was restored.
- local8-candidate-pre-eret-report: candidate archive a298516f..., kernel
  SHA256 22a5e3b4..., kernel size 96,824 bytes. Serial reached the pre-ERET
  line:
  HCR_EL2=0x80000000, SCTLR_EL1=0x30d01805, TCR_EL1=0x500003510,
  TTBR0_EL1=0x241000, VBAR_EL1=0x200800, ELR_EL1=0x100000, and
  SPSR_EL1=0x3c0. No lower-EL trap/PASS followed.
- local9-known-good-control-after-pre-eret: restored tree
  6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef
  fetched 82,045-byte kernel_2712.img and reached Talos early boot after a
  fresh power-cycle, but no production-timer PASS line was captured in the
  retained observe window.
- local10-candidate-pre-eret-rerun: unchanged pre-ERET-report candidate rerun
  again reached the pre-ERET line only, then the lab was restored.
- local11-candidate-el1-entry-report: first candidate with the EL1-entry
  callback was restored too early after only firmware serial output and no
  candidate TFTP delta, so it is not used as a Talos behavior classification.
- local12-candidate-el1-entry-rerun: candidate archive b3aadd08..., kernel
  SHA256 5dcd4b71..., kernel size 97,016 bytes. Fresh serial/TFTP evidence
  reached start, validated, and pre-ERET lines, but did not emit the
  entered-el1 callback, lower-EL trap, final classification, or PASS. The lab
  was restored.
- local13c-production-timer-control-accepted-archive: published the accepted
  production-timer control archive 739810c8..., kernel SHA256 fdf8858d...,
  kernel size 104,136 bytes. Fresh serial reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health before the
  unchanged candidate rerun.
- local14c-candidate-el1-entry-same-candidate-rerun: reran unchanged archive
  b3aadd08... / kernel 5dcd4b71... after the fresh control. Fresh serial and
  TFTP evidence again reached start, validated, and pre-ERET lines but did not
  emit entered-el1, lower-EL trap, final classification, or PASS. The lab was
  restored.
- local15-candidate-el1-asm-marker: after adding an assembly UART marker at the
  EL2-to-EL1 ERET target, local gates passed and the candidate archive was
  8ec2ed3a..., kernel SHA256 852dcff0..., kernel size 97,055 bytes. Fresh
  serial again reached start, validated, and pre-ERET lines but did not emit the
  entered-el1-asm marker, entered-el1 callback, lower-EL trap, final
  classification, or PASS. The lab was restored.
- local16b-candidate-staged-el1-rerun: after staging EL1 entry with
  SCTLR_EL1.M clear, candidate archive 01d03733..., kernel SHA256 283bdac9...,
  kernel size 97,055 bytes reached start, validated, pre-ERET with
  SCTLR_EL1=0x30d00800, entered-el1-asm, and entered-el1 callback. It did not
  emit the lower-AArch64 trap, final classification, or PASS. The lab was
  restored.
- local17-production-timer-control-after-staged-el1: the accepted
  production-timer control archive 739810c8... reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS after the staged-EL1 inconclusive
  candidate, proving lab health.
- local18-candidate-staged-el1-same-candidate-rerun: unchanged 01d03733... /
  283bdac9... candidate rerun after the fresh control again reached
  entered-el1-asm and entered-el1 callback but did not emit lower-AArch64 trap,
  final classification, or PASS. The lab was restored.
- local19-candidate-el1-mmio-l2-fix: after correcting the BCM2712 MMIO table
  level, local gates passed and the candidate archive was 21ba91a9..., kernel
  SHA256 35053d61..., kernel size 97,055 bytes. Fresh serial again reached
  entered-el1-asm and entered-el1 callback but did not emit lower-AArch64 trap,
  final classification, or PASS. The lab was restored. The retained
  post-restore TFTP query has stale 82,045-byte labels because the endpoint
  computes bytes from the current restored tree; candidate archive identity and
  publish status retain the 97,055-byte candidate identity.
- local20-production-timer-control-after-mmio-l2-fix: fresh control after
  local19 reached classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS from serial cursor 2464588, proving lab
  health before the unchanged candidate rerun.
- local21-candidate-el1-mmio-l2-same-candidate-rerun: unchanged 21ba91a9... /
  35053d61... candidate rerun after the fresh control again reached start,
  validated, pre-ERET, entered-el1-asm, and entered-el1 callback but did not
  emit lower-AArch64 trap, final classification, or PASS. The lab was restored.
- local22-candidate-el1-mmu-markers: after adding post-SCTLR and pre-EL0-ERET
  assembly markers, local gates passed and the candidate archive was
  927b3e55..., kernel SHA256 27569410..., kernel size 97,133 bytes. Fresh TFTP
  evidence before restore recorded da591740/kernel_2712.img served at 97,133
  bytes. Serial reached entered-el1-asm and entered-el1 callback but did not
  emit el1-mmu-enabled, before-el0-eret, lower-AArch64 trap, final
  classification, or PASS. The lab was restored.
- local23-production-timer-control-after-el1-mmu-markers: fresh accepted
  production-timer control archive 739810c8... reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS from serial cursor 2510526, proving lab
  health before the unchanged local22 candidate rerun.
- local24-candidate-el1-mmu-markers-same-candidate-rerun: unchanged
  927b3e55... / 27569410... candidate rerun after the fresh control again
  reached start, validated, pre-ERET, entered-el1-asm, and entered-el1 callback
  but did not emit el1-mmu-enabled, before-el0-eret, lower-AArch64 trap, final
  classification, or PASS. The lab was restored.
- local25-candidate-mmio-l1-fix: after correcting the BCM2712 MMIO mapping to
  the EL1 L1 index, local gates passed and the candidate archive was
  feb7a1a3..., kernel SHA256 78f9c60f..., kernel size 97,133 bytes. Fresh
  serial again reached entered-el1-asm and entered-el1 callback but did not
  emit el1-mmu-enabled, before-el0-eret, lower-AArch64 trap, final
  classification, or PASS. The lab was restored.
- local26-production-timer-control-after-mmio-l1-fix: fresh accepted
  production-timer control archive 739810c8... reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS from serial cursor 2546666, proving lab
  health before the unchanged local25 candidate rerun.
- local27-candidate-mmio-l1-fix-same-candidate-rerun: unchanged feb7a1a3... /
  78f9c60f... candidate rerun after the fresh control again reached
  entered-el1-asm and entered-el1 callback but did not emit el1-mmu-enabled,
  before-el0-eret, lower-AArch64 trap, final classification, or PASS. The lab
  was restored.
- local28-candidate-el1-mmu-without-cache: after changing the EL1 handoff to
  enable SCTLR_EL1.M without SCTLR_EL1.C or SCTLR_EL1.I, local gates passed and
  the candidate archive was 72857e8a..., kernel SHA256 cdd58506..., kernel size
  97,133 bytes. Fresh serial again reached entered-el1-asm and entered-el1
  callback but did not emit el1-mmu-enabled, before-el0-eret, lower-AArch64
  trap, final classification, or PASS. The lab was restored.
- local29-production-timer-control-after-local28: before any further source
  change, the accepted production-timer control archive 739810c8... was
  published from a fresh serial cursor. Serial reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health after local28.
- local30-candidate-el1-mmu-without-cache-same-candidate-rerun: after the
  local29 control, the unchanged local28 archive 72857e8a... / kernel
  cdd58506... was rerun from a fresh serial cursor. TFTP evidence before
  restore records da591740/kernel_2712.img served at 97,133 bytes. Serial again
  reached start, validated, pre-ERET, entered-el1-asm, and entered-el1 callback
  but did not emit el1-mmu-enabled, before-el0-eret, lower-AArch64 trap, final
  classification, or PASS. The lab was restored to the accepted
  production-timer control tree.
- local31-identity-mmu-discriminator: after adding the static checker and
  minimal EL1 identity-map/fault-vector discriminator, local gates passed and
  the candidate archive was ef181fe0..., kernel SHA256 3f522bbe..., kernel
  size 93,647 bytes. Static inspection proved kernel_start=0x200000,
  stack_top=0x35d010, identity block 0x1, discriminator handoff 0x20f0dc, EL1
  fault-vector base 0x20e800, regular vector base 0x200800, UART10 L1 index
  0x41, and UART10 L2 index 0x1e8. Fresh TFTP evidence before restore recorded
  da591740/kernel_2712.img served at 93,647 bytes. The first observe window
  was too short for the proof lines and only showed ordinary boot progress, so
  the result was classified as inconclusive pending control/rerun triage. The
  lab was restored.
- local32-production-timer-control-after-local31: after local31, the restored
  accepted production-timer control booted and reached production-timer proof
  lines, but the retained observe window did not capture the final PASS line
  before the next candidate rerun.
- local33-identity-mmu-discriminator-same-candidate-rerun: reran the unchanged
  ef181fe0... / 3f522bbe... discriminator after the local32 control attempt.
  Fresh TFTP evidence recorded repeated da591740/kernel_2712.img serves at
  93,647 bytes. Serial reached start, validated, pre-ERET with
  HCR_EL2=0x80000000, SCTLR_EL1=0x30d00800, TCR_EL1=0x500003510,
  TTBR0_EL1=0x219000, VBAR_EL1=0x20e800, ELR_EL1=0x100000, SPSR_EL1=0x3c0,
  then identity-entered-el1 and the entered-el1 callback with SP=0x35c710. It
  did not emit identity-mmu-enabled, el1-fault-vector, lower-AArch64 trap,
  final classification, or PASS. The lab was restored to the accepted
  production-timer control tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- local34-production-timer-control-after-local33: a fresh post-candidate
  production-timer control reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health after the
  discriminator reset loop.
- local35-identity-mmu-discriminator-same-candidate-rerun: after the local34
  production-timer PASS, the unchanged local31/local33 archive ef181fe0... /
  kernel 3f522bbe... was rerun from a fresh serial cursor. Fresh TFTP evidence
  recorded da591740/kernel_2712.img served at 93,647 bytes before restore.
  Serial again reached start, validated, pre-ERET, identity-entered-el1, and
  the entered-el1 callback, but no identity-mmu-enabled, EL1 fault-vector,
  lower trap, classification, or PASS. The lab was restored to the accepted
  production-timer control tree.
- local36-at-s1e1r-translation-probe: added proof-only EL1 AT S1E1R probes
  before SCTLR_EL1.M to inspect the active handoff/vector/stack/UART/unmapped
  addresses on hardware. Local gates passed; candidate archive SHA256 was
  3a90e895..., kernel size 94,385 bytes. The run reached the handoff PAR line
  only: identity-at-handoff par=0x000000000020fb00. Static inspection then
  found the diagnostic hex printer clobbered LR across nested putc calls, so
  the run was classified as a diagnostic-bug inconclusive rather than a new
  hardware boundary. The lab was restored.
- local37-production-timer-control-after-local36: a fresh post-candidate
  production-timer control reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health after local36.
- local38-at-s1e1r-translation-probe-same-candidate-rerun: after local37, the
  unchanged local36 archive 3a90e895... was rerun from a fresh serial/TFTP
  cursor and reproduced the same identity-at-handoff-only result before
  restore. This completed the required unchanged-candidate triage before the LR
  fix.
- local39-at-s1e1r-lr-fix: fixed the proof-only hex printer to preserve LR and
  reran local gates. Candidate archive SHA256 was 8fb49cc8..., kernel size
  94,401 bytes. Serial reached all pre-M=1 AT probe lines:
  identity-at-handoff par=0x000000000020fb00,
  identity-at-vectors par=0x000000000020eb00,
  identity-at-stack par=0x000000000035db00,
  identity-at-uart par=0x000000107d001b00, and
  identity-at-unmapped par=0x0000000080000b00, then still did not emit
  identity-mmu-enabled, EL1 fault-vector, lower trap, classification, or PASS.
  The lab was restored.
- local40-production-timer-control-after-local39: a fresh post-candidate
  production-timer control reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health after local39.
- local41-at-s1e1r-lr-fix-same-candidate-rerun: after local40, the unchanged
  local39 archive 8fb49cc8... was rerun from fresh serial/TFTP cursors. Fresh
  TFTP evidence recorded da591740/kernel_2712.img served at 94,401 bytes.
  Serial reproduced all pre-M=1 PAR probe lines through
  identity-at-unmapped, then stopped before identity-mmu-enabled,
  EL1 fault-vector, lower trap, classification, or PASS.
- local42-sctlr-enable-sequence-markers: split the SCTLR enable sequence with
  proof-only markers after the SCTLR_EL1 MSR and after DSB SY. Local gates
  passed; candidate archive SHA256 was 10fdc5b7..., kernel size 94,577 bytes.
  Serial reached identity-after-sctlr-msr and identity-after-sctlr-dsb, but
  stopped before the post-ISB identity-mmu-enabled marker, fault vector, lower
  trap, classification, or PASS. The lab was restored.
- local43-production-timer-control-after-local42: a fresh post-candidate
  production-timer control reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health after local42.
- local44-sctlr-enable-sequence-markers-same-candidate-rerun: after local43,
  the unchanged local42 archive 10fdc5b7... was rerun from fresh serial/TFTP
  cursors. Fresh TFTP evidence recorded da591740/kernel_2712.img served at
  94,577 bytes and reproduced the post-MSR/post-DSB but no post-ISB marker
  boundary before restore.
- local45-tcr-ips-40bit: changed the proof-only TCR_EL1.IPS discriminator from
  48-bit to 40-bit physical-address size, which still covers low DRAM and the
  BCM2712 UART window. Local gates passed; candidate archive SHA256 was
  4388119b..., kernel size 94,577 bytes. Serial showed TCR_EL1=0x200003510 and
  again reached identity-after-sctlr-msr and identity-after-sctlr-dsb, but not
  identity-mmu-enabled, fault vector, lower trap, classification, or PASS.
- local46-production-timer-control-after-local45: a fresh post-candidate
  production-timer control reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health after local45.
- local47-tcr-ips-40bit-same-candidate-rerun: after local46, the unchanged
  local45 archive 4388119b... was rerun from fresh serial/TFTP cursors. Fresh
  TFTP evidence recorded da591740/kernel_2712.img served at 94,577 bytes and
  reproduced the post-MSR/post-DSB but no post-ISB marker boundary. The lab was
  restored to the accepted production-timer control tree.
- local48-post-isb-immediate-marker: added a literal-free post-ISB UART marker
  and a literal-free EL1 fault-vector marker so the next observation would not
  depend on rodata loads or the formatted marker path after M=1. Local gates
  passed; candidate archive SHA256 was 635ac8d5..., kernel SHA256 e495a1a2...,
  kernel size 96,065 bytes. Serial still reached identity-after-sctlr-msr and
  identity-after-sctlr-dsb, but not identity-post-isb-immediate,
  el1-fault-immediate, identity-mmu-enabled, lower trap, classification, or
  PASS.
- local49-production-timer-control-after-local48: a fresh production-timer
  control reached classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health after local48.
- local50-post-isb-immediate-marker-same-candidate-rerun: after local49, the
  unchanged local48 archive 635ac8d5... was rerun from fresh serial/TFTP
  cursors. Fresh TFTP evidence recorded da591740/kernel_2712.img served at
  96,065 bytes and reproduced the post-MSR/post-DSB but no literal-free
  post-ISB marker or fault-vector marker boundary. The lab was restored.
- local51-normal-noncacheable-walk: changed only the proof EL1 normal-memory
  MAIR attribute and TCR_EL1 table-walk cacheability from WBWA to
  non-cacheable. Local gates passed; candidate archive SHA256 was 171dcd43...,
  kernel SHA256 8c5f4fbe..., kernel size 96,065 bytes. Serial showed
  TCR_EL1=0x200003010 and still reached identity-after-sctlr-msr and
  identity-after-sctlr-dsb, but not the literal-free post-ISB marker, fault
  vector, lower trap, classification, or PASS.
- local52-production-timer-control-after-local51: a fresh production-timer
  control reached classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health after local51.
- local53-normal-noncacheable-walk-same-candidate-rerun: after local52, the
  unchanged local51 archive 171dcd43... was rerun from fresh serial/TFTP
  cursors. Fresh TFTP evidence recorded da591740/kernel_2712.img served at
  96,065 bytes and reproduced the same post-MSR/post-DSB but no post-ISB
  marker boundary. The lab was restored to the accepted production-timer
  control tree.
- local54-4k-identity-pages: replaced the proof EL1 identity mapping's 2 MiB
  executable block with 4 KiB page descriptors for the linked kernel-through-
  stack range while keeping the same non-cacheable TCR/MAIR discriminator.
  Local gates passed; candidate archive SHA256 was 722a09c9..., kernel SHA256
  3cb610af..., kernel size 96,065 bytes. Static inspection reported
  identity_l2=0x1 and identity_pages=0x200..0x35e. Fresh serial/TFTP evidence
  recorded da591740/kernel_2712.img served at 96,065 bytes. Serial still
  reached identity-after-sctlr-msr and identity-after-sctlr-dsb, but not the
  literal-free post-ISB marker, fault vector, lower trap, classification, or
  PASS.
- local55-production-timer-control-after-local54: a fresh production-timer
  control reached classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS, proving lab health after local54.
- local56-4k-identity-pages-same-candidate-rerun: after local55, the unchanged
  local54 archive 722a09c9... / kernel 3cb610af... was rerun from fresh
  serial/TFTP cursors. Fresh TFTP evidence recorded da591740/kernel_2712.img
  served at 96,065 bytes and reproduced the same post-MSR/post-DSB but no
  post-ISB marker boundary. The lab was restored to the accepted
  production-timer control tree.

## Supervisor Intervention Analysis

### Problem Statement

The physical failure is now specifically at the EL1 translation-enable boundary
inside the staged Pi 5 EL0 trap proof. The EL2-to-EL1 handoff reaches the
assembly target and the Rust callback with SCTLR_EL1.M clear. The next step
sets SCTLR_EL1.M, leaves C and I clear in local28, executes DSB SY and ISB, and
then should print rpi5-el0-trap-proof: el1-mmu-enabled. On Pi 5, that marker
never appears, and neither do the before-EL0-ERET marker, lower-AArch64 trap
saved-state line, final classification, or PASS.

### Required Invariant Before M=1

Before SCTLR_EL1.M is enabled, QEMU and Pi 5 should agree on this invariant:

- TTBR0_EL1 points at the EL1 proof root table and covers every VA that can be
  touched immediately after M=1: the current instruction stream, marker rodata,
  the active EL1 stack, EL1 exception vectors, user text, user stack, and UART
  MMIO.
- TTBR1_EL1 must not participate in the proof path; every post-M=1 PC, SP,
  VBAR, and MMIO VA must be a low TTBR0 address, or TTBR1_EL1 must be
  explicitly reported and proven harmless.
- TCR_EL1, MAIR_EL1, and all descriptors agree on 4 KiB granules, low TTBR0
  addresses, a physical-address-size setting sufficient for low DRAM and the
  BCM2712 UART window, normal-cacheable attributes for code/data/stack/tables,
  and device attributes for UART MMIO.
- VBAR_EL1 points at exception vectors that are mapped and executable after
  M=1; any synchronous fault during or after the ISB can reach a vector that can
  report without relying on unmapped data.
- The active EL1 stack VA is mapped writable, and the current PC plus the next
  marker string are mapped executable/readable at the addresses actually used
  by the firmware-loaded image.
- UART10 MMIO at 0x10_7d00_1000 is mapped device-nGnRE in the EL1 tables, so
  the assembly marker path can print after M=1.

### Contradicting Evidence

- local16b reached entered-el1-asm and entered-el1 callback with SCTLR_EL1.M
  clear, proving the EL2-to-EL1 ERET target executes after staged EL1 entry.
- local18 reran the same staged-EL1 candidate after a fresh production-timer
  PASS and reproduced entered-el1-asm and entered-el1 callback, excluding a
  one-off lab failure.
- local19 added the low MMIO table-level correction but still reached only the
  EL1 entry markers.
- local22 added post-SCTLR and pre-EL0-ERET markers; serial stopped before
  el1-mmu-enabled.
- local24 reran local22 unchanged after a fresh production-timer PASS and
  reproduced the same stop before el1-mmu-enabled.
- local25 moved BCM2712 MMIO to the EL1 L1 slot; local27 reran it unchanged
  after local26 production-timer PASS, and both still stopped before
  el1-mmu-enabled.
- local28 enabled SCTLR_EL1.M without enabling EL1 C/I; local29 production-timer
  control passed, and local30 reran local28 unchanged with fresh 97,133-byte
  candidate TFTP fetches. It still stopped before el1-mmu-enabled.
- The production-timer controls local17, local20, local23, local26, and local29
  reached classification and PASS, so the lab, TFTP path, power cycle, serial
  path, EL2 MMU/cache path, SMP startup, and timer preemption proof remain
  healthy outside this EL1 translation-enable boundary.

### Proven Facts And Open Assumptions

Proven facts:

- The local28 candidate identity is 72857e8a... with kernel SHA256 cdd58506...
  and kernel size 97,133 bytes.
- The Pi 5 fetches the candidate from TFTP; local30 retained fresh
  da591740/kernel_2712.img fetches at 97,133 bytes before restore.
- The candidate reaches EL1 with SCTLR_EL1.M clear, TCR_EL1=0x500003510,
  TTBR0_EL1=0x241000, VBAR_EL1=0x200800, ELR_EL1=0x100000,
  SPSR_EL1=0x3c0, and SP=0x384710.
- The failure happens before the first UART marker after the SCTLR_EL1.M write,
  DSB SY, and ISB sequence.
- The accepted production-timer control still passes after local28.

Unproven assumptions:

- The root/l1/l2/l3 descriptor bases in the retained archive are exactly the
  physical addresses reported in TTBR0_EL1 and cover all post-M=1 instruction,
  rodata, data, stack, vector, user, and MMIO accesses.
- The descriptor attributes, shareability, access permissions, PXN/UXN bits,
  and cacheability are legal for Pi 5 hardware at this transition.
- Cache clean, TLB invalidation, DSB, and ISB ordering are sufficient for
  hardware page-table visibility.
- VBAR_EL1 remains viable after M=1 and can report a synchronous abort without
  needing unmapped stack, rodata, or formatting paths.
- The post-M=1 UART print path can reach UART10 MMIO through the EL1 map.
- A synchronous fault after M=1 would be printable rather than resetting or
  entering an unobservable vector path.

### Candidate Approaches

Approach A is a static EL1 translation-table proof/checker plus retained
archive inspection. It should parse the built image, symbol addresses, and table
descriptor constants, then prove the M=1 continuation PC, marker rodata, active
EL1 stack, VBAR_EL1, low user pages, and UART10 MMIO translate through TTBR0_EL1
with the intended attributes. This is the lowest-risk local gate and can catch
table construction, address-mask, attribute, and archive-identity errors before
another hardware run, but it cannot by itself prove Pi 5 barrier/TLB behavior or
fault-vector printability.

Approach B is a minimal EL1 identity-map and fault-vector diagnostic. It should
install a deliberately small TTBR0_EL1 map that identity-maps the current code
page(s), marker rodata, active EL1 stack, VBAR_EL1 vectors, and UART10 MMIO;
install a proof-only EL1 vector that writes fixed UART markers and raw
ESR_EL1/ELR_EL1/FAR_EL1 without formatting; then enable SCTLR_EL1.M and emit a
single post-ISB marker before doing anything with EL0. This changes the hardware
question from "does the complete user proof work" to "can EL1 translation,
vectors, stack, and UART survive M=1 on the simplest observable map."

### Chosen Discriminator

Use Approach B as the decisive next hardware discriminator, gated by the static
checker from Approach A before publication. This is the smallest discriminator
that can separate the remaining failure classes:

- If the static checker fails, the issue is table construction, archive
  identity, or missing PC/stack/vector/MMIO coverage and no hardware run is
  needed.
- If the static checker passes and the post-M=1 marker appears, translation
  table basics, instruction fetch, stack survival, barrier/TLB sequencing, and
  UART MMIO mapping are sufficient; the bug is in the richer user/EL0 proof
  mapping or later EL0 transition.
- If the EL1 fault-vector marker appears, exception-vector reachability and
  post-M=1 UART reporting work, and ESR/ELR/FAR can identify the missing or
  illegal access.
- If neither post-M=1 nor fault-vector markers appear on the minimal identity
  map, the likely failure is instruction fetch at the SCTLR enable boundary,
  vector-base reachability, or barrier/TLB sequencing, not the user SVC path.

The existing proof-only markers and staged EL1 handoff remain in the bounded
task for reproducibility of local28/local30 and to keep the next discriminator
anchored to the same failure boundary. They should be removed or folded into
the accepted proof once the minimal identity/fault-vector diagnostic identifies
the failing class. They do not broaden the task into general syscalls, process
loading, descriptor I/O, filesystem, shell, networking, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Validation

- fmt/lint: cargo fmt --all -- --check passed after the latest code changes.
- unit tests: cargo -Zjson-target-spec test passed with QEMU on PATH.
- QEMU/substitute: scripts/qemu-el0-trap-smoke.sh passed after the latest
  code changes.
- image/archive inspection: scripts/rpi5-archive-review.sh on
  target/talos-rpi5-el0-trap-proof-local54-boot.tar.gz passed after each candidate
  rebuild; latest local candidate archive SHA256 is
  722a09c98473eada9c4b62d9ecb4687691af4f0f90027ef7afe8284ca5e96127.
- static inspection: scripts/rpi5-el0-trap-proof-static-check.sh passed for
  the focused ELF, proving identity_l2=0x1, identity_pages=0x200..0x35e,
  handoff, vectors, stack, and UART10 index coverage for the minimal
  page-descriptor discriminator before publication.
- static inspection: scripts/rpi5-el0-trap-proof-static-check.sh passed after
  the AT S1E1R probe and LR-preservation changes, with the latest handoff at
  0x20f0e4, proof vectors at 0x20e800, stack top at 0x35e010, and UART10
  coverage at L1 index 0x41 / L2 index 0x1e8.
- serialized Pi 5 hardware boot/output: candidate runs reached the planned
  start, validation, pre-ERET register lines, identity-entered-el1 marker, and
  EL1 Rust callback, but no identity-mmu-enabled marker, EL1 fault-vector
  report, lower-AArch64 trap, classification, or PASS lines yet.
- repeated hardware run: local33 reran the unchanged local31 candidate after a
  control attempt and reproduced the same no-post-M=1-marker boundary.
- repeated hardware run: local38 reran the unchanged local36 AT probe after a
  production-timer PASS and reproduced the diagnostic-bug stop after the first
  PAR line, allowing the LR-preservation fix.
- serialized Pi 5 hardware boot/output: local39 with the LR fix printed all
  pre-M=1 PAR lines, but PAR reports physical identity values even for the
  deliberately unmapped probe while SCTLR_EL1.M is clear. This proves the probe
  path and UART reporting work before M=1, but it does not prove the EL1 tables
  will be used after M=1.
- serialized Pi 5 hardware boot/output: local42 and unchanged rerun local44
  printed both identity-after-sctlr-msr and identity-after-sctlr-dsb, then
  stopped before the post-ISB identity-mmu-enabled marker. This moves the
  hardware boundary from the SCTLR write/DSB pair to the context synchronization
  point where EL1 translation takes effect.
- serialized Pi 5 hardware boot/output: local45 changed TCR_EL1.IPS to 40-bit
  and unchanged rerun local47 reproduced the same post-MSR/post-DSB but no
  post-ISB boundary, so unsupported 48-bit IPS is not the discriminator.
- serialized Pi 5 hardware boot/output: local48 added literal-free post-ISB
  and EL1 fault-vector markers; unchanged rerun local50 reproduced the same
  post-MSR/post-DSB boundary without either immediate marker, so rodata loads,
  BL side effects, and the formatted marker path are not required to explain
  the stop.
- serialized Pi 5 hardware boot/output: local51 changed the proof EL1
  normal-memory attribute and table-walk cacheability to non-cacheable; unchanged
  rerun local53 reproduced the same boundary with TCR_EL1=0x200003010, so
  WBWA table-walk cacheability is not the discriminator.
- serialized Pi 5 hardware boot/output: local54 replaced the executable
  identity block with 4 KiB page descriptors for the linked kernel-through-stack
  range; unchanged rerun local56 reproduced the same post-MSR/post-DSB but no
  post-ISB boundary, so L2 block descriptor legality is not the discriminator.
- serialized Pi 5 hardware boot/output: local57 added a source-backed
  translation feature/legal-shape report and fixed the BCM2712 MMIO L2
  descriptor end index from the wrapped 0 value to exclusive index 512. The
  first local57 observe window retained fresh 98,817-byte candidate TFTP
  evidence but no Talos proof lines before the observe window ended.
- repeated hardware run: local58 production-timer control reached
  classification=pi5-production-timer-preemption-complete and
  rpi5-production-timer-preemption: PASS after local57. local59 reran the
  unchanged local57 candidate and proved the discriminator: Pi 5 reported
  parange-bits=40, 4 KiB granule support, MMIO L2 range 0x1e0..0x1ff,
  identity-post-isb-immediate, identity-mmu-enabled, and a deliberate EL1
  fault-vector report for the unmapped read. This identified the missing MMIO
  mapping as the post-ISB reporting blocker.
- serialized Pi 5 hardware boot/output: local60 switched from the deliberate
  EL1 fault discriminator back to the EL0 trap path. It reached
  entered-el1-asm, el1-mmu-enabled, before-el0-eret, and a lower-AArch64 SVC
  trap at the proof vector, confirming the EL0 transition but not yet the
  regular Rust exception handler classification/PASS.
- serialized Pi 5 hardware boot/output: local61 restored VBAR_EL1 to the
  regular exception vectors before ERET to EL0. It reached the expected
  lower-aarch64-sync trap, raw ESR 0x56007a10, final
  classification=pi5-el0-trap-proof-complete, and rpi5-el0-trap-proof: PASS.
- serialized Pi 5 hardware boot/output: local62 reran the cleaned final
  candidate after removing unused proof-only identity/fault-vector
  diagnostics. It retained archive SHA256 2b3002ab..., kernel SHA256
  fc8e5429..., fresh 97,781-byte da591740/kernel_2712.img TFTP evidence,
  lower-aarch64-sync saved-state output, final
  classification=pi5-el0-trap-proof-complete, and rpi5-el0-trap-proof: PASS.
- repeated hardware run: local34 production-timer control passed after local33,
  proving the lab, TFTP, serial, EL2 MMU/cache path, SMP startup, and timer
  preemption proof remain healthy outside the minimal EL1 M=1 discriminator.
- repeated hardware run: local37, local40, local43, local46, local49, local52,
  and local55 production-timer controls passed after the AT probe,
  SCTLR-sequence, TCR/cacheability, and 4 KiB page-descriptor iterations,
  proving lab health after those candidates.
- restore proof: latest status after local62 restore reports tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 for the
  accepted production-timer control tree.

## Next Action

Accepted. local62 is the retained final physical Pi 5 evidence for this task:
candidate archive 2b3002ab... / kernel fc8e5429... printed the source-backed
translation feature/legal-shape report, entered EL1 with regular VBAR_EL1,
enabled EL1 translation, entered EL0, captured the lower-AArch64 SVC trap in
the regular Rust exception path, and reported
classification=pi5-el0-trap-proof-complete plus rpi5-el0-trap-proof: PASS.
The lab was restored to the accepted production-timer control tree. General
syscall ABI, process loading, descriptor I/O, filesystem, shell, networking,
SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain out
of scope.
