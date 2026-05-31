# Phase 8 QEMU Live Descriptor-Image Installation Smoke Core Task

Task: phase8-qemu-live-descriptor-image-installation-smoke-core-20260531

Status: accepted.

## Scope

Implemented and ran the accepted QEMU/substitute smoke scenario
qemu_live_descriptor_image_installation_smoke for the model-only
descriptor-image installation boundary. The smoke records the accepted fixture
identity, success binding, input state, TTBR provenance, kernel-half coverage,
permission policy, blocked live-register states, teardown behavior,
deterministic rejection cases, zero live side effects, final classification,
and PASS line.

No Pi 5 hardware action, boot archive publication, hardwareTestLock
acquisition, live translation-register mutation, active-root descriptor copy,
ASID allocation, TLB invalidation, lower-EL ERET, scheduler publication,
process lifecycle, filesystem syscall, shell, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy occurred.

## Changed Files

- build.rs
- src/main.rs
- src/target/qemu_virt.rs
- scripts/qemu-live-descriptor-image-installation-smoke.sh
- tasks/2026-05-31-phase8-qemu-live-descriptor-image-installation-smoke-core.md
- tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log

## Evidence

- retained smoke log:
  tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log.
- final classification:
  qemu-live-descriptor-image-installation-smoke-complete.
- PASS line:
  qemu-live-descriptor-image-installation-smoke: PASS.
- exact final line:
  qemu-live-descriptor-image-installation-smoke: final participants=15 expected=15 errors=0 classification=qemu-live-descriptor-image-installation-smoke-complete.
- adjacent Phase 8 QEMU/substitute regressions: not applicable; this task added
  the new smoke entry point/script and retained log only. It did not change
  src/program_loader.rs, src/process_install.rs, src/process_address_space.rs,
  src/process_page_table_materialization.rs, src/initial_process_launch.rs,
  src/initial_user_stack.rs, src/live_address_space_activation.rs,
  src/kernel_half_reachability.rs, or src/kernel_half_descriptor_image.rs.

## Validation

- QEMU/substitute: scripts/qemu-live-descriptor-image-installation-smoke.sh
  passed and retained the required log.
- unit tests: cargo -Zjson-target-spec test --quiet passed, 329 no_std tests.
- fmt/lint: cargo fmt --all -- --check passed.
- static whitespace inspection: git diff --check passed.
- documentation: mdbook build not required because no mdBook docs were touched.
- hardware: hardwareTestLock remained unlocked/restored and unused.

## Next

After commit/state acceptance, the mechanically next queued task is
phase8-live-descriptor-image-installation-closeout-checkpoint-20260531,
provided this retained QEMU/substitute evidence remains conclusive, the repo
has no relevant uncommitted conflicts, and hardwareTestLock is unlocked and
restored.
