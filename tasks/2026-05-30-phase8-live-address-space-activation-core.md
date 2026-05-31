# Phase 8 Live Address-Space Activation Core Task

Task: phase8-live-address-space-activation-core-20260530
Status: accepted

## Scope

Implemented the target-independent LiveAddressSpaceActivationPlan preflight
boundary selected by the accepted contract and smoke plan. This is an
inspectable model record only: it copies accepted loader/install/address-space/
materialization/launch/stack identities, records TTBR0 root provenance from
the materialized root lease, records blocked TTBR1/kernel-half, TCR, MAIR,
SCTLR, ASID, TLB, barrier, lower-EL, runnable, lifecycle, startup ABI,
filesystem syscall, and Pi 5 proof states, and keeps all live side effects
false.

No TTBR/TCR/MAIR/SCTLR write, live TLBI, DSB/ISB activation sequence, ASID
allocation, lower-EL ERET, scheduler runnable publication, process lifecycle,
descriptor-table mutation, filesystem syscall behavior, QEMU smoke evidence,
Pi 5 hardware proof, boot archive publication, shell, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy was added.

## Changed Source Paths

- src/live_address_space_activation.rs
- src/main.rs
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-live-address-space-activation-core.md

## Accepted Behavior

- Boundary identity: phase8-live-address-space-activation-plan-v1.
- Activation policy:
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1.
- TTBR0 provenance: materialized-process-root-lease with no TTBR0 write.
- TTBR1/kernel policy: blocked-no-accepted-kernel-half-map.
- TCR/MAIR states: compatibility-record-only.
- SCTLR state: mutation-blocked.
- ASID/TLB/barrier states: blocked-no-asid-allocation,
  blocked-no-live-tlbi, and planned-only-no-live-dsb-isb.
- Launch binding state: model-only-activation-preflight-ready.
- Runnable publication: blocked-no-runnable-publication.
- The plan owns only a plan-local record lease; teardown is idempotent and
  leaves materialization, launch, stack, and image ownership untouched.

## Focused Tests

Unit evidence added in src/live_address_space_activation.rs:

- builds_preflight_plan_with_blocked_live_activation_fields
- records_required_kernel_reachability_without_live_side_effects
- rejects_live_register_and_publication_requests_without_partial_activation
- teardown_releases_only_plan_local_lease_and_is_idempotent
- rejects_identity_and_entry_disagreements_before_publication
- rolls_back_resource_exhaustion_without_partial_activation

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed; 312 no_std tests passed,
  including six live-address-space activation tests.
- QEMU/substitute: not yet applicable for this core task because no runnable
  qemu_live_address_space_activation_smoke script was introduced; the queued
  smoke-core task owns retained QEMU/substitute evidence.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

The implementation satisfies the accepted activation/preflight boundary with
stable identity and accepted lineage while preserving deterministic
no-partial-activation and no-runnable-publication behavior. Live register
mutation and user execution remain blocked for later explicitly planned work.
