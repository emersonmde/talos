# Phase 12 RP1 Ethernet Clock/Reset Read-Only Baseline Closeout

Task id: phase12-rp1-ethernet-clock-reset-readonly-baseline-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clock-reset-readonly-baseline-frontier-closed
Evidence level: static inspection of accepted proof evidence, task records,
documentation, and git history.

## Goal

Close out the accepted read-only clock/reset baseline proof and decide whether
the next bounded ownership step is mechanically objective.

## Findings

- fixed: reconciled the accepted proof classification
  rp1-ethernet-clock-reset-readonly-baseline-report-visibility-control-output
  from commit d37b18ad12aa0f3763f54444068e175525a52662.
- fixed: confirmed candidate capture-chain-v4 joined selected tree
  047815dc8bfde65c28be5d4a5844eb5bf83c4dc60749d7a9c76c8dce402599c3,
  expected da591740/kernel_2712.img fetches at 50056 bytes, run-unique serial
  freshness, final pre-restore identity, and restore proof.
- fixed: confirmed candidate serial retained observed-window MACB_MID identity
  context 0x1c001000fc/raw 0x70109/idnum 0x7/rev 0x109 plus selected
  pclk/hclk/tsu_clk/tx_clk baseline report facts.
- fixed: confirmed the paired control capture-chain-v4 joined selected tree
  16745426bc0d0f1cc2b1844f48d6e656a8c900afb6fcca42caee5553afc7f4fd,
  expected da591740/kernel_2712.img fetches at 49176 bytes, run-unique serial
  freshness, final pre-restore identity, and restore proof while withholding
  candidate-only baseline facts.
- fixed: confirmed the lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  hardwareTestLock release.
- not-an-issue: legacy serial-drain-not-empty-before-power suggestions in the
  underlying bundle summaries do not weaken the accepted proof because the
  capture-chain-v4 identity joins passed for candidate and control.
- deferred: no mechanically objective write-backed clock/reset ownership task
  follows from report visibility/control output alone. Exact register target,
  write/restore semantics, shared RP1_CLK_SYS safety, reset-controller target,
  GPIO32/PHY reset, MDIO/PHY, interrupts, DMA, descriptor rings, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition require
  supervisor-planned scope and acceptance criteria.

No findings were removed.

## Accepted Boundary

The accepted frontier is closed at report visibility/control output. The
candidate proves the accepted read-only clock/reset baseline report path is
visible on Pi 5 serial and retains the observed-window MACB_MID identity
context plus selected pclk/hclk/tsu_clk/tx_clk facts. The paired control proves
the same report/capture path while withholding candidate-only clock/reset
facts.

This closeout does not accept clock/reset ownership, RP1 MMIO writes,
clock/reset writes, reset-controller ownership, GPIO32/PHY reset ownership,
MDIO/PHY ownership, DMA, descriptor rings, transfer completion, interrupt
completion, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## Same-Shaped Retry Policy

Same-shaped read-only baseline report visibility hardware retries are closed
for this candidate/control pair. A future task must provide materially
different scope and explicit acceptance criteria, such as an exact
source-backed register/restore contract, a shared-clock safety proof, or a
separate PHY/MDIO/GPIO32 slice. This closeout does not choose such a task.

## Evidence

- Proof task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-closeout/evidence-map.json.

## Validation

- static inspection: proof task record, proof classification/evidence map,
  capture summary, project docs, and git history reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Supervisor planning is required for the next explicit Phase 12.1 clock/reset
ownership slice. No mechanically objective write-backed ownership follow-up is
selected from this report visibility/control proof alone.
