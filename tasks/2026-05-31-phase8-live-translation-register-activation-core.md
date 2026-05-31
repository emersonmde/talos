# Phase 8 Live Translation-Register Activation Core Task

Task: phase8-live-translation-register-activation-core-20260531

Status: accepted

## Scope

Milestone 8.3 implementation of the accepted model/substitute-only live
translation-register activation boundary. The work added a
LiveTranslationRegisterActivation record that consumes the accepted
KernelHalfDescriptorImageInstallation, preserves copied Phase 8 lineage and
TTBR0/TTBR1 provenance, records compatibility/blocker state, and exposes
deterministic rejection and teardown behavior.

Changed files:

- build.rs
- src/live_descriptor_image_installation.rs
- src/live_translation_register_activation.rs
- src/main.rs
- src/target/qemu_virt.rs
- scripts/qemu-live-translation-register-activation-smoke.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-live-translation-register-activation-core.md
- tasks/evidence/2026-05-31-qemu-live-translation-register-activation-smoke-core/qemu-live-translation-register-activation-smoke.log

## Outcome

The implementation adds boundary identity
phase8-live-translation-register-activation-v1 and policy
model-ttbr0-ttbr1-activation-commit-below-live-registers-v1. It publishes
only a model-level activation-commit intent with copied installation, loader,
install, address-space, materialization, launch, stack, activation-plan,
reachability, and descriptor-image identities.

The model records installation-published and below-live-register input state,
TTBR0 materialized-root provenance, TTBR1 descriptor-image kernel-root
provenance, compatibility-only TCR/MAIR state, mutation-blocked SCTLR state,
blocked ASID/TLB/barrier/live-register state, preserved kernel diagnostic
reachability, idempotent teardown, deterministic errors, and zero live side
effects. It still does not perform architectural TTBR/TCR/MAIR/SCTLR writes,
active-root descriptor copy, ASID allocation, live TLBI, DSB/ISB execution,
lower-EL ERET, scheduler publication, process-table mutation, descriptor-table
publication, filesystem syscall behavior, QEMU hardware mutation, or Pi 5
hardware action.

The QEMU/substitute route
qemu_live_translation_register_activation_smoke is wired and the retained log
is present for the later smoke-core task. That route is evidence for this core
task only as QEMU/substitute smoke-routing coverage; the queued smoke-core
task still owns accepting retained smoke evidence as its primary goal.

## Evidence

- Implementation:
  src/live_translation_register_activation.rs.
- QEMU/substitute smoke route:
  scripts/qemu-live-translation-register-activation-smoke.sh and
  src/target/qemu_virt.rs.
- Retained QEMU/substitute log:
  tasks/evidence/2026-05-31-qemu-live-translation-register-activation-smoke-core/qemu-live-translation-register-activation-smoke.log.
- Final retained smoke classification:
  qemu-live-translation-register-activation-smoke-complete.
- Final retained smoke PASS line:
  qemu-live-translation-register-activation-smoke: PASS.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 334 no_std
  tests.
- QEMU/substitute:
  scripts/qemu-live-translation-register-activation-smoke.sh passed and
  retained the smoke log.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: hardwareTestLock remained unlocked/restored and unused; no Pi 5
  archive publication, power cycle, TFTP action, serial observation, or
  hardware proof was performed.

## Commit

Recorded in durable supervisor state after acceptance.
