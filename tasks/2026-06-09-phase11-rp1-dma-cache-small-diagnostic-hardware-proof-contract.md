# Phase 11 RP1 DMA/Cache Small Diagnostic Hardware-Proof Contract

Task: phase11-rp1-dma-cache-small-diagnostic-hardware-proof-contract-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Define the smallest serialized Pi 5 proof contract for exposing the accepted
local/static RP1 DMA small diagnostic plan without starting live RP1 DMA.

## Scope

- Use the accepted small diagnostic plan closeout because it explicitly
  selected this guarded source-contract boundary.
- Decide whether future hardware proof is allowed, blocked, or limited to plan
  visibility/control output.
- If a future hardware proof is allowed, define candidate identity, artifact
  digest, serial output, TFTP delta, restore evidence, hardwareTestLock
  ownership, and inconclusive-run triage requirements.
- Preserve blockers for RP1 channel ownership, descriptor rings, transfer
  completion, interrupt completion, IOMMU/runtime policy, allocation/pinning
  expansion, device-consumer selection, Ethernet, storage, networking, SSH,
  Milestone 11.3 completion, and phase transition.
- Record findings with disposition.

## Non-Goals

No runtime source changes, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 MMIO writes, DMA channel programming,
descriptor-ring construction, transfer completion, interrupt completion,
Ethernet/storage driver work, networking, SSH, Milestone 12 work, hardware
validation claim, Milestone 11.3 completion, or phase transition.

## Retained Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-source-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-closeout.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-source-contract/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-core/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-closeout/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-closeout/evidence-map.json
- src/dma_cache.rs
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Contract Decision

Future Pi 5 proof is allowed only as a serialized plan visibility/control
output proof. It is not allowed as a live DMA proof, channel-programming proof,
descriptor-ring proof, transfer-completion proof, interrupt-completion proof,
device-consumer proof, Ethernet/storage readiness proof, networking proof, SSH
proof, Milestone 11.3 completion proof, or phase-transition proof.

The reason is tied to accepted evidence: the plan core already constructs and
formats local/static RP1 DMA small diagnostic plan evidence from accepted
DmaCacheDriverDiagnosticEnvelopeEvidence plus retained rp1_dma controller
source facts. The accepted frontier still has no RP1 DMA channel ownership,
descriptor-ring layout or ownership, transfer-completion policy,
interrupt-completion policy, IOMMU/runtime policy, allocation/pinning
expansion, hardware proof, or device-specific consumer. A future Pi 5 proof
may therefore only show that the accepted plan evidence is visible and
distinguishable on the real target, paired with a control that preserves the
output path without claiming plan acceptance or touching RP1 DMA.

## Future Hardware-Proof Shape

A future supervisor-planned Pi 5 proof task is mechanically objective only if
it stays within this boundary:

- Candidate output reports the accepted small diagnostic plan contract id,
  source contract id, envelope contract id, local/static plan classification,
  rp1_dma compatible string, RP1 bus base, translated CPU physical base,
  channel count, target count, interrupt and clock names, CPU/RP1 buffer
  addresses, descriptor length, line coverage, direction, cacheability, owner
  transition, IOMMU classification, rejected runtime claims, unresolved gaps,
  and hardware-proof boundary classification.
- The paired control preserves the same boot/serial reporting path while
  withholding accepted plan evidence or using an explicit no-plan control
  classification.
- Candidate and control code must not perform RP1 MMIO writes, DMA channel
  programming, descriptor-ring construction, cache-maintenance execution for a
  live driver buffer, transfer polling, interrupt acknowledgement, device
  consumer activity, Ethernet/storage driver work, networking, or SSH.
- The hardware output classification must remain limited to
  small-diagnostic-plan-visible-on-pi5 or an equivalent visibility/control
  phrase. It must not include driver DMA completion, hardware/device
  completion, DMA transfer completion, or interrupt completion.
- The task must acquire hardwareTestLock before staging or running the Pi 5
  candidate and must release it with restore evidence.

## Required Hardware Evidence

If a future task runs this proof, it must capture:

- hardwareTestLock owner, task id, acquisition time, release time, and restore
  status;
- candidate identity from the lab API effective kernel/tree fields before the
  run;
- kernel artifact digest and staged boot tree identity;
- fresh serial cursor before the run and serial output after the run;
- TFTP delta for the candidate and paired control;
- candidate serial records naming the plan contract id, source contract id,
  local/static classification, hardware-proof boundary classification, and
  rejected runtime/hardware claims;
- paired-control serial records that prove the output path without accepted
  plan evidence;
- restore evidence after the run;
- classification JSON that labels the evidence level as serial hardware
  boot/output and rejects all live-DMA claims.

Before changing code after any inconclusive Pi 5 run, the future task must run
the standard triage sequence: candidate identity, fresh serial cursor, TFTP
delta, known-good control, then candidate rerun.

Failed hardware boots are evidence, not incidents, unless evidence becomes
confused/unworkable or concrete hardware/lab risk requires restore.

## Findings

- fixed: recognized the accepted plan closeout as selecting this guarded
  contract-only boundary.
- fixed: limited any future Pi 5 proof to accepted plan visibility/control
  output rather than live RP1 DMA behavior.
- fixed: defined the required candidate/control evidence shape, including
  candidate identity, artifact digest, serial output, TFTP delta, restore
  evidence, hardwareTestLock serialization, and classification requirements.
- fixed: required inconclusive-run triage before code changes after any future
  inconclusive Pi 5 run.
- deferred: RP1 DMA channel ownership, descriptor-ring layout and ownership,
  transfer completion, interrupt completion, runtime IOMMU policy, DMA-safe
  allocation or pinning expansion, device-consumer selection, live hardware
  DMA proof, Ethernet, storage, networking, SSH, Milestone 12 work, Milestone
  11.3 completion, and phase transition.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only a source contract for a future serialized proof.

No findings were removed.

## Rejected Claims

This contract does not accept:

- Pi 5 hardware validation.
- Boot archive publication or hardwareTestLock acquisition by this task.
- RP1 MMIO writes or DMA channel programming.
- RP1 DMA channel ownership.
- DMA descriptor rings or descriptor-ring ownership.
- Transfer completion or interrupt completion.
- Hardware/device completion.
- Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Cache-coherent, non-cacheable, or IOMMU-backed runtime policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- Milestone 11.3 completion by implication.
- Phase transition.

## Validation

- static inspection: reviewed the accepted source-contract, plan-core, and
  plan-closeout task records, classification JSON, evidence map, src/dma_cache.rs,
  roadmap, and project contract doc.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract states whether future hardware proof is allowed, blocked, or
  limited to plan visibility/control output, with the reason tied to accepted
  evidence: satisfied.
- Contract includes hardwareTestLock serialization and inconclusive-run triage
  before code changes for any future hardware proof: satisfied.
- Contract preserves all unresolved RP1 channel, descriptor-ring,
  transfer-completion, interrupt-policy, IOMMU/runtime, allocation/pinning,
  hardware/device-consumer, Ethernet/storage/networking/SSH, Milestone 11.3,
  and phase-transition gaps: satisfied.
- Accepted contract is committed or the task is blocked with a precise reason:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

No explicit queued follow-up task exists. Set planningNeeded=true for
supervisor planning of a bounded small diagnostic plan visibility/control proof
or a checkpoint. Any future proof must be separately scoped, serialized through
hardwareTestLock, and limited to plan visibility/control output. It must not
program RP1 MMIO or DMA channels, create descriptor rings, claim transfer or
interrupt completion, implement Ethernet, storage, networking, SSH, accept
Milestone 11.3 completion, or create a phase transition.
