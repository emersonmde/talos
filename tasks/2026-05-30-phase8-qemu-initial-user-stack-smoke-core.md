# Phase 8 QEMU Initial User Stack Smoke Core Task

Task: phase8-qemu-initial-user-stack-smoke-core-20260530

Status: accepted

## Scope

Milestone 8.3 QEMU/substitute evidence for the accepted target-independent
initial user stack boundary.

Changed files:

- build.rs
- src/main.rs
- src/initial_process_launch.rs
- src/initial_user_stack.rs
- src/process_install.rs
- src/process_page_table_materialization.rs
- src/program_loader.rs
- src/target/qemu_virt.rs
- scripts/qemu-initial-user-stack-smoke.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-qemu-initial-user-stack-smoke-core.md
- tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log

Non-goals honored: no Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, physical serial observation, live TTBR/TCR/MAIR/
SCTLR write, ASID allocation, live TLB mutation, lower-EL ERET, scheduler
runnable publication, process lifecycle, shell behavior, descriptor-backed
filesystem syscalls, writable filesystem, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Outcome

The task added qemu_initial_user_stack_smoke as a QEMU-only boot scenario and
retained script. The scenario consumes the accepted /bin/init ProgramImagePlan,
ProcessImageInstallPlan, ProcessAddressSpace, ProcessPageTableMaterialization,
and InitialProcessLaunchPlan chain, then constructs the accepted
phase8-initial-user-stack-plan-v1 stack record without publishing runnable
lower-EL state.

The retained smoke log proves:

- accepted fixture, install, address-space, materialization, launch, and stack
  boundary identities;
- InitialUserStackPlan success publication with source digest and lineage
  copied from the accepted records;
- stack top and initial SP 0x0000_8000_0000_0000 with 16-byte alignment;
- usable range [0x0000_7fff_ffff_c000, 0x0000_8000_0000_0000);
- guard range [0x0000_7fff_ffff_b000, 0x0000_7fff_ffff_c000);
- four stack-owned USER_DATA page leases, copied_bytes=0, and
  zeroed_bytes=0x4000;
- one unmapped guard page with no frame lease and no descriptor;
- minimal-empty-argc0 startup metadata with argv/envp NULL and auxv/TLS still
  blocked-pending-startup-abi;
- model-only launch-plan stack-ready binding with blocked-no-ttbr-activation;
- zero TTBR/TCR/MAIR/SCTLR, ASID, TLB, lower-EL ERET, scheduler,
  process-table, and descriptor-table side effects;
- idempotent teardown; and
- deterministic identity, range, overlap, executable-stack, capacity,
  already-stack-ready, and live-launch rejection cases with no partial stack
  or launch state.

## Evidence

- QEMU/substitute smoke log:
  tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log
- Classification:
  qemu-initial-user-stack-smoke: final participants=13 expected=13 errors=0
  classification=qemu-initial-user-stack-smoke-complete
- PASS line:
  qemu-initial-user-stack-smoke: PASS
- Conditional QEMU/substitute regressions:
  scripts/qemu-initial-process-launch-smoke.sh,
  scripts/qemu-process-page-table-materialization-smoke.sh,
  scripts/qemu-process-address-space-smoke.sh,
  scripts/qemu-process-install-smoke.sh, and
  scripts/qemu-program-loader-smoke.sh passed.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests/QEMU-substitute: cargo -Zjson-target-spec test passed with 306
  no_std tests.
- QEMU/substitute initial-user-stack smoke:
  scripts/qemu-initial-user-stack-smoke.sh passed and retained the required
  log.
- conditional QEMU/substitute regressions:
  scripts/qemu-initial-process-launch-smoke.sh,
  scripts/qemu-process-page-table-materialization-smoke.sh,
  scripts/qemu-process-address-space-smoke.sh,
  scripts/qemu-process-install-smoke.sh, and
  scripts/qemu-program-loader-smoke.sh passed because this task touched target
  boot-scenario routing and the loader/install/address-space/materialization/
  launch fixture chain used by the smoke.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed.

## Residual Blocked Surfaces

Live TTBR activation, ASID/TLB sequencing, lower-EL ERET, scheduler runnable
publication, process lifecycle, exec/spawn/wait, broad argv/envp/auxv/TLS ABI,
shell, descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain blocked until later explicit
tasks accept their contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
