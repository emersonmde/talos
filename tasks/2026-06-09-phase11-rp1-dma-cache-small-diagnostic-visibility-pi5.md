# Phase 11 RP1 DMA Cache Small Diagnostic Visibility Pi 5 Proof

Task: phase11-rp1-dma-cache-small-diagnostic-visibility-pi5-20260609
Status: blocked
Classification: rp1-dma-cache-small-diagnostic-visibility-pi5-control-freshness-blocked

## Goal

Run the serialized Pi 5 visibility/control proof for the accepted RP1 DMA cache
small diagnostic report surface.

## Scope

- Acquire and release hardwareTestLock for the serialized Pi 5 proof.
- Stage candidate and paired no-plan control boot archives.
- Capture candidate/control lab identity, artifact digests, fresh serial/TFTP
  cursors, serial output, TFTP deltas, restore evidence, and classification.
- Keep acceptance limited to plan visibility/control output.

## Non-Goals

No RP1 MMIO writes, DMA channel programming, descriptor-ring construction,
transfer completion, interrupt completion, Ethernet/storage work, networking,
SSH, Milestone 11.3 completion, or phase transition.

## Implementation

Added two Pi 5 boot scenarios and staging/review scripts:

- rpi5_rp1_dma_cache_small_diagnostic_visibility_candidate
- rpi5_rp1_dma_cache_small_diagnostic_visibility_no_plan_control

The candidate scenario prints the accepted visibility report fields from the
local/static plan: report/source/plan identities, RP1 DMA controller identity,
CPU/RP1 buffer addresses, cache-line coverage, direction, cacheability, owner
transition, IOMMU classification, rejected runtime/hardware claims, retained
risks, and the hardware-proof boundary classification.

The control scenario uses the same serial report path while withholding plan
evidence and carrying no-plan-rp1-dma-small-diagnostic-visibility-control.

## Findings

- fixed: added candidate/control Pi 5 report scenarios that construct no RP1
  MMIO address and perform no DMA programming.
- fixed: added archive, boot-tree, image, and review scripts for candidate and
  paired control.
- fixed: candidate rerun produced marker-visible serial output with a clean
  pi5-capture-transaction-v2 identity join.
- deferred: the paired no-plan control marker was visible, but both control
  attempts were rejected by serial-drain-not-empty-before-power; the task is
  blocked on control freshness, not on report construction.
- not-an-issue: initial candidate/control marker visibility is retained only as
  inconclusive evidence because the pre-power drain rejected those captures.
- not-an-issue: known-good control also reached its marker while retaining the
  same pre-power drain freshness rejection, confirming this is a capture-chain
  freshness issue.

## Hardware Evidence

Pre-run restored tree:

    a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

Candidate rerun:

- Tree: f9087adb7e83ca60efc52441939c9b8fc56c250504a699d86e8b6c12bf0327be
- Kernel bytes: 49864
- Kernel SHA-256:
  05ba4badca3f7e2ec59968574002161ed6b15fb34b57f64b0cc1ff708a9df691
- Serial marker:
  TALOS: rp1-dma-cache-small-diagnostic-visibility-candidate
- Identity join: clean, no rejection reasons.

Control reruns:

- Tree: c5c2be8b818559863dc3a8ac0997b45035603b948ba515c02b6871bfad0f9e03
- Kernel bytes: 48600
- Kernel SHA-256:
  79deec34a939e964926158519ba21551a80626b7310f16773fd2033335619fc2
- Serial marker:
  TALOS: rp1-dma-cache-small-diagnostic-visibility-control
- Identity join: rejected by serial-drain-not-empty-before-power on both
  control attempts.

Known-good control:

- Marker rpi5-production-timer-preemption: PASS was visible.
- Identity join was also rejected by serial-drain-not-empty-before-power.

Final restore returned to the pre-run tree:

    a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

## Validation

- static inspection: accepted visibility-core task, closeout task, and
  src/dma_cache.rs fields inspected before implementation.
- fmt/lint/typecheck: candidate and control Pi 5 images built successfully.
- archive inspection: candidate/control archive review scripts passed and
  recorded artifact digests.
- serial hardware boot/output: candidate rerun marker visible with clean
  identity join.
- serial hardware boot/output: paired control markers visible but freshness
  rejected by the identity join.
- lab-controller API: pre-run status/files/snapshots, publish status, serial
  window, TFTP delta, restore, and post-restore status retained.
- restore evidence: final status matched the pre-run restored tree.

## Acceptance

- Task record lists findings with disposition: satisfied.
- hardwareTestLock acquisition/release/restore recorded: satisfied in durable
  state after release.
- Candidate identity, artifact digest, staged tree, fresh serial cursor/output,
  and TFTP delta captured: satisfied.
- Candidate serial output includes required accepted visibility report fields
  and rejected runtime/hardware claims: satisfied on clean candidate rerun.
- Paired control serial output proves the same path with no accepted plan
  evidence: blocked by control freshness; marker visible but identity join
  rejected by serial-drain-not-empty-before-power.
- Classification rejects live-DMA, RP1 MMIO/DMA programming, descriptor rings,
  transfer/interrupt completion, Ethernet/storage/networking/SSH, Milestone
  11.3 completion, and phase transition: satisfied.
- Inconclusive run triage recorded: satisfied with candidate identity, fresh
  serial/TFTP evidence, known-good control, unchanged candidate rerun, and
  repeated control reruns.

## Next Action

Blocked for supervisor planning. The next task should not repeat the same
paired-control capture shape unless it changes the freshness discriminator or
acceptance criteria for saturated/non-empty pre-power serial drain evidence.
