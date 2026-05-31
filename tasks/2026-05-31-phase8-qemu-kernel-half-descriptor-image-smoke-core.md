# Phase 8 QEMU Kernel-Half Descriptor-Image Smoke Core Task

Task: phase8-qemu-kernel-half-descriptor-image-smoke-core-20260531
Status: accepted

## Scope

Milestone 8.3 QEMU/substitute evidence for the accepted non-installed
KernelHalfDescriptorImage construction boundary.

Changed files:

- build.rs
- src/main.rs
- src/target/qemu_virt.rs
- scripts/qemu-kernel-half-descriptor-image-smoke.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-qemu-kernel-half-descriptor-image-smoke-core.md
- tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log

Non-goals honored: no Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, physical serial observation, live TTBR0_EL1/
TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 write, ASID allocation, TLB mutation,
activation DSB/ISB, lower-EL ERET, scheduler runnable publication, process
lifecycle, shell behavior, descriptor-backed filesystem syscalls, writable
filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache
driver policy.

## Outcome

The task added qemu_kernel_half_descriptor_image_smoke as a QEMU-only boot
scenario and retained script. The scenario consumes the accepted /bin/init
ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace,
ProcessPageTableMaterialization, LiveAddressSpaceActivationPlan, and
KernelHalfReachabilityPlan chain, then constructs the accepted
phase8-kernel-half-descriptor-image-v1 record without installing the image or
mutating live translation state.

The retained smoke log proves copied accepted lineage, TTBR0 materialized-root
provenance, TTBR1 model-owned kernel-root image intent, required kernel
coverage, privileged-only normal/device descriptor attributes, model-owned
root/table leases, deterministic no-partial-image rejections, idempotent
teardown, and zero live TTBR/TCR/MAIR/SCTLR/TLB/barrier/lower-EL/scheduler/
process/descriptor-table side effects.

## Evidence

- QEMU/substitute smoke log:
  tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log
- Classification:
  qemu-kernel-half-descriptor-image-smoke: final participants=17 expected=17
  errors=0 classification=qemu-kernel-half-descriptor-image-smoke-complete
- PASS line:
  qemu-kernel-half-descriptor-image-smoke: PASS
- Conditional QEMU/substitute regression:
  not applicable. This task did not change src/kernel_half_reachability.rs,
  src/live_address_space_activation.rs, src/initial_user_stack.rs,
  src/initial_process_launch.rs, or accepted activation/reachability
  vocabulary; it added a new QEMU-only scenario around the already accepted
  descriptor-image core.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests/QEMU-substitute: cargo -Zjson-target-spec test passed with 324
  no_std tests.
- QEMU/substitute kernel-half descriptor-image smoke:
  scripts/qemu-kernel-half-descriptor-image-smoke.sh passed and retained the
  required log.
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

## Next Action

Next queued task: phase8-kernel-half-descriptor-image-closeout-checkpoint-20260531.
Promote it on the next worker wake if this task is accepted and committed with
conclusive retained QEMU/substitute evidence and hardwareTestLock remains
unlocked/restored.
