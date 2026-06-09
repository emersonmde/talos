# Phase 11 RP1 DMA Cache Small Diagnostic Visibility Proof Closeout

Task: phase11-rp1-dma-cache-small-diagnostic-visibility-proof-closeout-20260609
Status: accepted
Classification: rp1-dma-cache-small-diagnostic-visibility-proof-control-freshness-blocked

## Goal

Close out the Pi 5 small diagnostic visibility/control proof without accepting
live DMA or RP1 device behavior by implication.

## Scope

- Reconcile the committed Pi 5 candidate/control blocker from
  phase11-rp1-dma-cache-small-diagnostic-visibility-pi5-20260609.
- Preserve the accepted candidate serial visibility evidence and the blocked
  paired-control evidence boundary.
- State the rejected live-DMA, RP1 MMIO, descriptor-ring, transfer,
  interrupt, networking, Milestone 11.3, and phase-transition claims.
- Decide the next boundary from the committed evidence.

## Non-Goals

No runtime source changes, no additional Pi 5 hardware run, no boot archive
publication, no hardwareTestLock acquisition, no RP1 MMIO writes, no DMA
channel programming, no descriptor-ring construction, no transfer completion,
no interrupt completion, no Ethernet/storage work, no networking, no SSH, no
Milestone 12 work, no Milestone 11.3 completion by implication, and no phase
transition.

## Findings

- fixed: documented the candidate rerun as serial-visible plan report evidence
  with clean pi5-capture-transaction-v2 identity join, matching staged tree,
  two matching TFTP fetches, and retained restore proof.
- fixed: documented the paired no-plan control evidence as blocked because the
  repeated control capture preserved the same serial report path and marker but
  failed the freshness discriminator with serial-drain-not-empty-before-power.
- fixed: documented that the known-good control failed the same freshness
  discriminator, making this a capture-chain blocker rather than evidence of
  report construction failure.
- deferred: any same-shaped Pi 5 visibility/control retry requires supervisor
  planning with a changed freshness discriminator or changed acceptance
  criteria for saturated/non-empty pre-power serial drain evidence.
- not-an-issue: no additional hardware run was performed for this closeout;
  the task scope is a static checkpoint over committed hardware evidence.
- not-an-issue: no live DMA, RP1 MMIO write, channel ownership, descriptor
  ring, transfer completion, interrupt completion, Ethernet/storage,
  networking, SSH, Milestone 11.3 completion, or phase transition is accepted.

## Evidence Reconciliation

Accepted candidate visibility evidence:

- Task:
  phase11-rp1-dma-cache-small-diagnostic-visibility-pi5-20260609
- Commit: 8a1d6ed5
- Candidate rerun tree:
  f9087adb7e83ca60efc52441939c9b8fc56c250504a699d86e8b6c12bf0327be
- Candidate kernel_2712.img SHA-256:
  05ba4badca3f7e2ec59968574002161ed6b15fb34b57f64b0cc1ff708a9df691
- Candidate serial marker:
  TALOS: rp1-dma-cache-small-diagnostic-visibility-candidate
- Candidate marker count: 20
- Candidate identity join rejection reasons: none
- Candidate TFTP evidence: stable, two matching 49,864-byte fetches

Blocked paired-control evidence:

- Control rerun tree:
  c5c2be8b818559863dc3a8ac0997b45035603b948ba515c02b6871bfad0f9e03
- Control kernel_2712.img SHA-256:
  79deec34a939e964926158519ba21551a80626b7310f16773fd2033335619fc2
- Control serial marker:
  TALOS: rp1-dma-cache-small-diagnostic-visibility-control
- Control marker count: 30
- Control TFTP evidence: stable, two matching 48,600-byte fetches
- Control identity join rejection reason:
  serial-drain-not-empty-before-power

Known-good control:

- Required marker: rpi5-production-timer-preemption: PASS
- Marker visible: yes
- Identity join rejection reason: serial-drain-not-empty-before-power

Restore evidence:

- Pre-run tree:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
- Post-restore tree:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
- Restored to pre-run tree: yes

## Accepted Claims

- The candidate Pi 5 boot scenario emits the accepted RP1 DMA cache small
  diagnostic visibility report over serial.
- The candidate rerun has a clean pi5-capture-transaction-v2 identity join.
- The candidate rerun serial output includes accepted plan/report identities,
  local/static plan classification, hardware-proof boundary classification,
  visibility report fields, unresolved gaps, and rejected runtime/hardware
  claims.
- Candidate/control scenarios remain report-only and do not program RP1 MMIO,
  DMA channels, descriptor rings, transfers, or interrupts.
- The lab was restored to the pre-run tree after the hardware sequence.

## Blocked And Rejected Claims

The full Pi 5 visibility/control proof remains blocked because the paired
control proof did not pass the freshness discriminator. Same-shaped hardware
visibility retries are closed unless future scope supplies a different
freshness discriminator or explicit changed acceptance criteria for saturated
pre-power serial drain evidence.

Rejected claims:

- live DMA
- RP1 MMIO writes
- RP1 DMA channel ownership
- DMA channel programming
- descriptor-ring construction or ownership
- transfer completion
- interrupt completion
- hardware/device completion
- Ethernet readiness
- storage readiness
- networking
- SSH
- Milestone 11.3 completion
- phase transition

## Validation

- static inspection: accepted/blocked Pi 5 proof task record, evidence map,
  classification JSON, candidate/control/known-good capture summaries, project
  contract doc, and roadmap.
- JSON checks: jq empty on task-owned evidence-map/classification JSON.
- documentation build: /home/node/.cargo/bin/mdbook build.
- diff checks: git diff --check and git diff --cached --check.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles Pi 5 candidate/control visibility evidence without
  expanding acceptance to live DMA, RP1 MMIO/DMA programming, descriptor
  rings, transfer completion, interrupt completion, Ethernet/storage,
  networking, SSH, Milestone 11.3 completion, or phase transition: satisfied.
- Checkpoint states whether same-shaped hardware visibility retries are closed
  or blocked with a specific evidence reason: satisfied; closed unless future
  scope changes the freshness discriminator or acceptance criteria.
- NextAction selects one bounded follow-up only if evidence makes it
  mechanically objective: no worker-owned follow-up is mechanically objective
  from this evidence, so supervisor planning is required.

## Next Action

Set planningNeeded=true for supervisor planning. Do not repeat the same paired
no-plan control capture shape without a changed freshness discriminator or
changed acceptance criteria for saturated/non-empty pre-power serial drain
evidence. Do not start descriptor-ring/channel-ownership work, Milestone 11.3
closeout, or any phase transition from this checkpoint alone.
