# Phase 11 RP1 DMA Cache Small Diagnostic Visibility V3 Retry

Task: phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry-20260609
Status: accepted
Classification: rp1-dma-cache-small-diagnostic-visibility-v3-proof-accepted

## Goal

Resolve the blocked RP1 DMA small-diagnostic Pi 5 visibility/control proof
using the accepted run-unique serial visibility discriminator, without
expanding into live DMA.

## Scope

- Consume the accepted proof blocker/closeout from commit 8d1ce237 and the
  accepted run-unique discriminator repair from commit 44707dbe.
- Inspect the retained blocker evidence and existing visibility scripts before
  any code or hardware change.
- Add only the task-owned nonce support needed for the existing visibility
  candidate/control scenarios to use capture-nonce absence before power and
  presence after power as the freshness discriminator.
- Acquire hardwareTestLock, run serialized no-plan control and candidate Pi 5
  proofs, capture identity/TFTP/serial/restore evidence, and release the lock.

## Non-Goals

No live DMA, RP1 MMIO writes, DMA channel programming, channel ownership,
descriptor-ring construction or ownership, transfer completion, interrupt
completion, Ethernet/storage readiness, networking, SSH, Milestone 11.3
completion by implication, or phase transition.

## Implementation

The existing visibility candidate/control scenarios did not emit a run-unique
capture nonce. The task-owned change adds optional TALOS_CAPTURE_NONCE support
to the candidate and no-plan control image scripts, prints capture-nonce on
both visibility report lines, and extends the archive review scripts to verify
the nonce when supplied.

The runtime report contract is otherwise unchanged. The candidate still prints
accepted local/static small diagnostic plan evidence. The control still uses
the same report path while withholding accepted plan evidence and carrying
classification=no-plan-rp1-dma-small-diagnostic-visibility-control.

## Findings

- fixed: existing visibility scenarios lacked run-unique nonce output, so the
  repaired checker could not be applied to this proof shape.
- fixed: candidate and control image scripts now accept and validate
  TALOS_CAPTURE_NONCE using the established Phase 11 character/length rules.
- fixed: candidate and control report loops now emit the task-owned
  capture-nonce token immediately after the serial marker.
- fixed: candidate and control review scripts can assert that a supplied nonce
  is present in the staged kernel image.
- fixed: paired no-plan control passed Pi 5 capture, run-unique, and
  boot-staging identity gates.
- fixed: candidate plan visibility output passed Pi 5 capture, run-unique, and
  boot-staging identity gates.
- not-an-issue: both runs used saturated direct-read serial shape, but the
  accepted discriminator requires only nonce absence before power and presence
  after power when the other V3 identity/TFTP/final/restore gates pass.
- deferred: live RP1 DMA behavior, channel ownership, descriptor rings,
  transfer completion, interrupt completion, Ethernet/storage, networking,
  SSH, Milestone 11.3 closeout, and phase transition remain separate work.

No findings were removed.

## Hardware Evidence

Baseline restored tree before and after the run:

    a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

No-plan control:

- Staged tree: 3813289a7df48f04313329b90073683fb07eb0188b719290a862af587f86739b
- Kernel bytes: 48704
- Kernel SHA-256:
  5bad74366a4e510940e25424beda0dedfce5598574dab8f1bb4590d2e51489d1
- Capture nonce: dma-vis-v3-control-20260609T211037Z-8d1ce237
- Run-unique check: passed; nonce absent before power and present 29 times
  after power.
- Boot-staging identity check: passed.
- Serial output: no-plan control report path visible with accepted plan fields
  withheld.

Candidate:

- Staged tree: f0b229ab1da582050f68af75b1de9953e9010b2cc4443ff3ee0002789e7572b2
- Kernel bytes: 49968
- Kernel SHA-256:
  d46844db2747fa9457844acf66b473f63d3dca345bb7e306bd284f82ba49d60a
- Capture nonce: dma-vis-v3-candidate-20260609T211037Z-8d1ce237
- Run-unique check: passed; nonce absent before power and present 20 times
  after power.
- Boot-staging identity check: passed.
- Serial output: accepted local/static small diagnostic plan visibility report
  fields visible, including plan/source/envelope identity, RP1 DMA controller
  facts, buffer/cache fields, rejected runtime/hardware claims, and retained
  risks.

## Validation

- static inspection: accepted blocker/closeout, accepted run-unique repair,
  existing candidate/control scripts, and touched source/scripts inspected.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests via QEMU substitute: cargo -Zjson-target-spec test --quiet
  dma_cache passed.
- script syntax: bash -n passed for touched image/review scripts and retained
  capture/checker scripts.
- archive inspection: candidate/control archive review scripts passed with
  task-owned capture nonces.
- serial hardware boot/output: paired no-plan control and candidate both
  passed run-unique replay and boot-staging identity checks.
- lab-controller API: publish, status, TFTP, serial, restore, and post-restore
  evidence retained.
- restore evidence: final lab status returned to the baseline tree.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Run uses run-unique nonce absence before power and presence after power while
  retaining selected-tree identity, expected TFTP bytes, final identity, and
  restore gates: satisfied.
- Candidate serial output includes accepted visibility fields and rejected
  runtime/hardware claims: satisfied.
- Paired no-plan control uses the same report path, withholds accepted plan
  evidence, and passes repaired freshness/identity gates: satisfied.
- Classification rejects live DMA, RP1 MMIO writes, channel ownership,
  descriptor rings, transfer completion, interrupt completion,
  Ethernet/storage readiness, networking, SSH, Milestone 11.3 completion, and
  phase transition: satisfied.
- HardwareTestLock acquisition, release, and restore evidence: satisfied in
  durable state and task evidence.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-small-diagnostic-visibility-v3-closeout-20260609 on the
next worker wake. Do not start live DMA, descriptor-ring/channel-ownership,
Milestone 11.3 closeout, or a phase transition from this retry alone.
