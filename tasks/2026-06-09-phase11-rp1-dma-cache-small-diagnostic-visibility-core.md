# Phase 11 RP1 DMA/Cache Small Diagnostic Visibility Core

Task: phase11-rp1-dma-cache-small-diagnostic-visibility-core-20260609

Status: accepted

Evidence level: static inspection, local implementation, fmt/lint/typecheck,
unit tests, JSON checks, and git diff checks.

## Goal

Implement the bounded local/static candidate/control visibility-report surface
for the accepted RP1 DMA small diagnostic plan without running hardware or
programming DMA.

## Scope

- Consume accepted small diagnostic plan evidence and the accepted
  hardware-proof contract.
- Add a local/static candidate report and paired no-plan/control report shape
  that a later Pi 5 proof can print over serial.
- Preserve the accepted plan contract id, source contract id, envelope
  contract id, local/static plan classification, rp1_dma compatible string,
  RP1 bus base, translated CPU physical base, channel count, target count,
  interrupt and clock names, CPU/RP1 buffer addresses, descriptor length, line
  coverage, direction, cacheability, owner transition, IOMMU classification,
  rejected runtime claims, unresolved gaps, and hardware-proof boundary
  classification.
- Ensure the paired control uses the same reporting path while withholding
  accepted plan evidence and carrying an explicit no-plan control
  classification.
- Reject report input that claims RP1 channel ownership, descriptor rings,
  transfer completion, interrupt completion, hardware/device completion,
  Ethernet/storage readiness, networking, SSH, Milestone 11.3 completion, or
  phase transition.
- Record findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
RP1 MMIO writes, DMA channel programming, descriptor-ring construction,
transfer polling, interrupt acknowledgement, device-consumer work,
Ethernet/storage driver work, networking, SSH, Milestone 12 work, hardware
validation claim, Milestone 11.3 completion, or phase transition.

No general driver DMA API, DMA-safe allocation or pinning expansion, or
IOMMU/coherent/non-cacheable runtime policy.

## Implementation

- Added DMA_CACHE_SMALL_DIAGNOSTIC_SOURCE_CONTRACT_ID and
  DMA_CACHE_SMALL_DIAGNOSTIC_VISIBILITY_REPORT_CONTRACT_ID.
- Added candidate, no-plan control, and hardware-proof boundary
  classifications for the small diagnostic visibility report.
- Added rejected hardware-claim and retained-risk vocabularies copied from the
  accepted hardware-proof contract.
- Added DmaCacheSmallDiagnosticVisibilityReportKind,
  DmaCacheSmallDiagnosticVisibilityReportInput,
  DmaCacheSmallDiagnosticVisibilityReport,
  DmaCacheSmallDiagnosticVisibilityReportEvidence, and
  DmaCacheSmallDiagnosticVisibilityReportError.
- Added build_dma_cache_small_diagnostic_visibility_report,
  dma_cache_small_diagnostic_visibility_report_evidence, and rejected evidence
  formatting.
- Candidate validation requires accepted small diagnostic plan evidence and
  preserves the accepted prerequisite identities, RP1 DMA controller facts,
  buffer/cache fields, rejected runtime claims, unresolved gaps, and
  hardware-proof boundary classification.
- Control validation requires no plan evidence and emits the same report
  contract/source identity plus explicit no-plan control classification.
- Validators reject RP1 MMIO writes, RP1 channel ownership, DMA channel
  programming, descriptor-ring readiness, transfer completion, interrupt
  completion, hardware/device completion, Ethernet/storage readiness,
  networking, SSH, Milestone 11.3 completion, and phase transition claims.

## Findings

- fixed: implemented the local/static candidate visibility report in
  src/dma_cache.rs without adding hardware execution or DMA programming.
- fixed: implemented the paired no-plan control report through the same report
  evidence path while withholding accepted plan evidence.
- fixed: candidate evidence preserves the plan, source, envelope, executor,
  maintenance-sequence, sync-plan, descriptor, and source-inventory identities.
- fixed: candidate evidence preserves RP1 DMA compatible string, RP1 bus base,
  translated CPU physical base, channel count, target count, interrupt name,
  clock names, CPU/RP1 buffer addresses, descriptor length, cache line
  coverage, direction, cacheability, owner transition, IOMMU classification,
  rejected runtime claims, unresolved gaps, hardware-proof boundary
  classification, rejected hardware claims, and retained risks.
- fixed: validators reject invalid report shapes, non-accepted plan evidence,
  missing prerequisite identities, missing rejected runtime claims, missing
  unresolved gaps, unsupported cacheability/IOMMU claims, and all premature
  runtime/hardware readiness claims.
- deferred: Pi 5 serial visibility/control proof, hardwareTestLock
  serialization, boot archive publication, RP1 DMA channel ownership,
  descriptor-ring layout and ownership, transfer completion, interrupt
  completion, runtime IOMMU policy, DMA-safe allocation or pinning expansion,
  device-consumer selection, live hardware DMA proof, Ethernet, storage,
  networking, SSH, Milestone 12 work, Milestone 11.3 completion, and phase
  transition.
- not-an-issue: no docs/src update was required because this task adds only
  local/static implementation and task/evidence records without changing the
  accepted roadmap frontier.

No findings were removed.

## Rejected Claims

This task does not accept:

- Pi 5 hardware validation.
- Boot archive publication or hardwareTestLock acquisition.
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

- static inspection: reviewed the accepted hardware-proof contract, accepted
  plan-core, accepted plan-closeout, and src/dma_cache.rs changes.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet dma_cache passed with the
  Talos QEMU path configured; the custom runner executed 456 tests.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- diff checks: git diff --check and git diff --cached --check passed.

The first focused test invocation failed before executing tests because the
QEMU runner could not find qemu-system-aarch64. Rerunning with the documented
Talos PATH resolved the environment issue and passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Implementation exposes candidate and paired no-plan/control visibility
  report evidence derived only from accepted small diagnostic plan evidence or
  explicit control input: satisfied.
- Candidate report includes all required plan identity, RP1 controller,
  buffer/cache, rejected-claim, unresolved-gap, and hardware-proof boundary
  fields named by the accepted hardware-proof contract: satisfied.
- Control report preserves the reporting path while withholding accepted plan
  evidence and carrying an explicit no-plan/control classification: satisfied.
- Validators reject overclaiming inputs for RP1 channel ownership, descriptor
  rings, transfer completion, interrupt completion, hardware/device
  completion, Ethernet/storage readiness, networking, SSH, Milestone 11.3
  completion, and phase transition: satisfied.
- Focused tests cover accepted candidate report construction, accepted control
  report construction, and deterministic rejection cases: satisfied.
- Accepted implementation/evidence is committed before the closeout task
  starts: satisfied by the commit recorded in supervisor state after this
  task.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-small-diagnostic-visibility-closeout-20260609 on the
next worker wake. That checkpoint must reconcile only the accepted
local/static visibility report implementation and must not run hardware,
publish boot archives, acquire hardwareTestLock, program RP1 MMIO or DMA
channels, create descriptor rings, claim transfer or interrupt completion,
implement Ethernet, storage, networking, SSH, accept Milestone 11.3
completion, or create a phase transition.
