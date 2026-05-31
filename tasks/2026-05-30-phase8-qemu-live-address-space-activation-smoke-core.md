# Phase 8 QEMU Live Address-Space Activation Smoke Core Task

Task: phase8-qemu-live-address-space-activation-smoke-core-20260530

Status: accepted

## Scope

Milestone 8.3 QEMU/substitute evidence for the accepted target-independent
live address-space activation preflight boundary.

Changed files:

- build.rs
- src/main.rs
- src/live_address_space_activation.rs
- src/process_page_table_materialization.rs
- src/program_loader.rs
- src/target/qemu_virt.rs
- scripts/qemu-live-address-space-activation-smoke.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-qemu-live-address-space-activation-smoke-core.md
- tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log

Non-goals honored: no Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, physical serial observation, live TTBR/TCR/MAIR/
SCTLR write, ASID allocation, live TLB mutation, lower-EL ERET, scheduler
runnable publication, process lifecycle, shell behavior, descriptor-backed
filesystem syscalls, writable filesystem, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Outcome

The task added qemu_live_address_space_activation_smoke as a QEMU-only boot
scenario and retained script. The scenario consumes the accepted /bin/init
ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace,
ProcessPageTableMaterialization, InitialProcessLaunchPlan, and
InitialUserStackPlan chain, then constructs the accepted
phase8-live-address-space-activation-plan-v1 preflight record without live
register mutation or runnable publication.

The retained smoke log proves:

- accepted loader, install, address-space, materialization, launch, stack, and
  activation boundary identities;
- activation policy
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1;
- TTBR0 root provenance from the materialized process root lease with no
  TTBR0_EL1 write;
- blocked TTBR1/kernel-half policy;
- TCR/MAIR compatibility-record-only state and SCTLR mutation-blocked state;
- blocked ASID allocation, live TLB invalidation, barrier, and live-register
  sequence states;
- required kernel reachability prerequisites for VBAR_EL1, exception vectors,
  active kernel stack, kernel text/data, allocator, UART/MMIO diagnostics,
  scheduler code/data, and panic/fault reporting;
- model-only activation-preflight-ready launch binding from
  blocked-no-ttbr-activation;
- zero TTBR/TCR/MAIR/SCTLR, ASID, TLB, barrier, lower-EL ERET, scheduler,
  process-table, and descriptor-table side effects;
- idempotent teardown of only the plan-local lease; and
- deterministic identity, descriptor, forbidden-range, missing-reachability,
  live-register, scheduler-publication, lower-EL, and resource-exhaustion
  rejection cases with no partial activation or runnable publication.

## Evidence

- QEMU/substitute smoke log:
  tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log
- Classification:
  qemu-live-address-space-activation-smoke: final participants=15 expected=15
  errors=0 classification=qemu-live-address-space-activation-smoke-complete
- PASS line:
  qemu-live-address-space-activation-smoke: PASS
- Conditional QEMU/substitute regressions:
  scripts/qemu-initial-user-stack-smoke.sh,
  scripts/qemu-initial-process-launch-smoke.sh,
  scripts/qemu-process-page-table-materialization-smoke.sh,
  scripts/qemu-process-address-space-smoke.sh,
  scripts/qemu-process-install-smoke.sh, and
  scripts/qemu-program-loader-smoke.sh passed.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests/QEMU-substitute: cargo -Zjson-target-spec test passed with 312
  no_std tests.
- QEMU/substitute live-address-space-activation smoke:
  scripts/qemu-live-address-space-activation-smoke.sh passed and retained the
  required log.
- conditional QEMU/substitute regressions:
  scripts/qemu-initial-user-stack-smoke.sh,
  scripts/qemu-initial-process-launch-smoke.sh,
  scripts/qemu-process-page-table-materialization-smoke.sh,
  scripts/qemu-process-address-space-smoke.sh,
  scripts/qemu-process-install-smoke.sh, and
  scripts/qemu-program-loader-smoke.sh passed because this task touched target
  boot-scenario routing and the accepted Phase 8 fixture chain used by the
  smoke.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed.

## Residual Blocked Surfaces

Live TTBR/TCR/MAIR/SCTLR mutation, ASID allocation, live TLB invalidation,
live DSB/ISB activation sequencing, lower-EL ERET, scheduler runnable
publication, process lifecycle, exec/spawn/wait, broad argv/envp/auxv/TLS ABI,
shell, descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain blocked until later explicit
tasks accept their contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
