# Phase 8 Live Descriptor-Image Installation Core Task

Task: phase8-live-descriptor-image-installation-core-20260531

Status: implemented; pending final commit/state update.

## Scope

Implemented the target-independent model-only descriptor-image installation
boundary selected by the accepted contract and smoke plan. The implementation
adds a KernelHalfDescriptorImageInstallation record that binds an accepted
KernelHalfDescriptorImage to an accepted LiveAddressSpaceActivationPlan below
live translation-register activation.

No Pi 5 hardware action, boot archive publication, hardwareTestLock
acquisition, lower-EL ERET, scheduler runnable publication, process lifecycle,
descriptor-backed filesystem syscall, shell, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy occurred.

## Changed Files

- src/live_descriptor_image_installation.rs
- src/main.rs
- tasks/2026-05-31-phase8-live-descriptor-image-installation-core.md

## Implementation Evidence

- Boundary identity:
  phase8-live-descriptor-image-installation-v1.
- Policy identity:
  model-installed-ttbr1-descriptor-image-below-live-registers-v1.
- Success path copies loader, activation, reachability, descriptor-image,
  source, address-space, materialization, entry, stack, TTBR0 provenance,
  TTBR1 descriptor-image provenance, coverage, permissions, and blocker state.
- Deterministic rejection path records EINVAL for missing/destroyed/identity
  failures, ENOEXEC for lineage mismatch, EBUSY for already-installed/live
  side-effect state, EACCES for forbidden EL0/diagnostic-reachability cases,
  ENOMEM for installation-record exhaustion, and ENOSYS for live-register
  requests.
- Rollback and failure cases allocate no installation record before validation
  completes.
- Teardown releases only the installation-record-local lease, preserves the
  descriptor-image and activation inputs, and is idempotent.
- Side-effect accounting remains false for TTBR/TCR/MAIR/SCTLR mutation,
  active-root copy, descriptor-table publication, ASID allocation, TLB
  mutation, live DSB/ISB, lower-EL ERET, scheduler publication,
  process-table mutation, filesystem mutation, and hardware action.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed, 329 no_std tests.
- QEMU/substitute: not run in this implementation task; the queued
  phase8-qemu-live-descriptor-image-installation-smoke-core-20260531 task owns
  retained smoke evidence.
- documentation: mdbook build not required because no mdBook docs were touched.
- hardware: hardwareTestLock remained unlocked/restored and unused.

## Next

After commit/state acceptance, the mechanically next queued task is
phase8-qemu-live-descriptor-image-installation-smoke-core-20260531, provided
the accepted smoke plan still matches this implementation boundary and the repo
has no relevant uncommitted conflicts.
