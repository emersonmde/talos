# Phase 8 QEMU Kernel-Half Reachability Smoke Core Task

Task: phase8-qemu-kernel-half-reachability-smoke-core-20260531

Status: accepted

## Scope

Milestone 8.3 QEMU/substitute evidence for the accepted target-independent
kernel-half reachability preflight boundary.

Changed files:

- build.rs
- src/main.rs
- src/program_loader.rs
- src/target/qemu_virt.rs
- scripts/qemu-kernel-half-reachability-smoke.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-qemu-kernel-half-reachability-smoke-core.md
- tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log

Non-goals honored: no Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, physical serial observation, live TTBR0_EL1/
TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 write, ASID allocation, TLB mutation,
activation DSB/ISB, lower-EL ERET, scheduler runnable publication, process
lifecycle, shell behavior, descriptor-backed filesystem syscalls, writable
filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache
driver policy.

## Outcome

The task added qemu_kernel_half_reachability_smoke as a QEMU-only boot
scenario and retained script. The scenario consumes the accepted /bin/init
ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace,
ProcessPageTableMaterialization, InitialProcessLaunchPlan,
InitialUserStackPlan, and LiveAddressSpaceActivationPlan chain, then
constructs the accepted phase8-kernel-half-reachability-plan-v1 preflight
record without descriptor-image construction, live register mutation, or
runnable publication.

The retained smoke log proves:

- accepted loader, install, address-space, materialization, launch, stack,
  activation, and kernel-half boundary identities;
- kernel-half policy preflight-ttbr1-shared-kernel-root-reachability-v1;
- TTBR0 root provenance from the materialized process root lease with no
  TTBR0_EL1 write;
- TTBR1 shared privileged kernel-root policy with descriptor image blocked;
- required kernel text, rodata, data, bss, vector, active-stack, heap,
  page-frame, UART/MMIO diagnostic, scheduler, and panic/fault reachability;
- split TCR compatibility-record-only state, normal/device MAIR
  compatibility-record-only state, and SCTLR mutation-blocked state;
- blocked ASID allocation, live TLB invalidation, barrier, and live-register
  sequence states;
- zero TTBR/TCR/MAIR/SCTLR, descriptor-image, ASID, TLB, barrier,
  lower-EL ERET, scheduler, process-table, and descriptor-table side effects;
- idempotent teardown of only the plan-local lease while input records remain
  owned by their existing teardown paths; and
- deterministic identity, missing-kernel-range, missing-diagnostic/
  fault-reporting, forbidden-EL0-access, bad-device-attribute, live-register,
  descriptor-image, scheduler-publication, lower-EL, and resource-exhaustion
  rejection cases with no partial plan.

## Evidence

- QEMU/substitute smoke log:
  tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log
- Classification:
  qemu-kernel-half-reachability-smoke: final participants=16 expected=16
  errors=0 classification=qemu-kernel-half-reachability-smoke-complete
- PASS line:
  qemu-kernel-half-reachability-smoke: PASS
- Conditional QEMU/substitute regression:
  scripts/qemu-program-loader-smoke.sh passed because this task touched
  program_loader.rs only to expose the accepted fixture identity helper for the
  new scenario. Install, address-space, materialization, launch, stack, and
  live-activation owners were not behaviorally changed.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests/QEMU-substitute: cargo -Zjson-target-spec test passed with 318
  no_std tests.
- QEMU/substitute kernel-half reachability smoke:
  scripts/qemu-kernel-half-reachability-smoke.sh passed and retained the
  required log.
- conditional QEMU/substitute regression:
  scripts/qemu-program-loader-smoke.sh passed; other adjacent Phase 8 smokes
  were not applicable because their source owners were not behaviorally
  changed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed.

## Residual Blocked Surfaces

Kernel-half descriptor-image construction, live TTBR/TCR/MAIR/SCTLR mutation,
ASID allocation, live TLB invalidation, live DSB/ISB activation sequencing,
lower-EL ERET, scheduler runnable publication, process lifecycle,
exec/spawn/wait, broad argv/envp/auxv/TLS ABI, shell, descriptor-backed
filesystem syscalls, Pi 5 hardware proof, writable filesystems, persistent
storage, networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache
driver policy remain blocked until later explicit tasks accept their
contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
