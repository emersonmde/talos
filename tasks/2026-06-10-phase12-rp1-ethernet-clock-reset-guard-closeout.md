# Phase 12 RP1 Ethernet Clock/Reset Guard Closeout

Task id: phase12-rp1-ethernet-clock-reset-guard-closeout-20260610
Status: accepted
Owner: worker
Evidence level: static inspection, JSON validation, documentation build, diff
checks

## Goal

Reconcile the accepted local/static clock/reset guard core and decide whether a
serialized read-only Pi 5 baseline proof is mechanically objective.

## Scope

- Consumed accepted guard-core task
  phase12-rp1-ethernet-clock-reset-guard-core-20260610 at commit
  6a9b91175da6442f21733ea1f4f2d8d1f4914ee0.
- Reconciled the candidate guard report, paired no-clock-reset/no-Ethernet
  control report, validators, rejected claims, retained risks, docs, and git
  history.
- Preserved observed-window MACB_MID identity only as context: target
  0x1c001000fc, raw 0x70109, idnum 0x7, rev 0x109.
- Closed same-shaped local/static guard retries for this candidate/control pair.
- Selected the already queued serialized read-only clock/reset baseline Pi 5
  proof as the next mechanically objective bounded task.

## Non-Goals

No runtime implementation changes except docs/evidence closeout, no hardware
run, no boot archive publication, no hardwareTestLock acquisition, no RP1 MMIO
writes, no clock/reset writes or ownership, no GPIO32/PHY reset, no MDIO, no
DMA, no descriptor rings, no interrupts/completions, no packet I/O, no
networking, no sockets, no SSH, no Phase 12.2, and no phase transition.

## Findings

- fixed: reconciled guard-core evidence with the accepted ownership contract.
- fixed: confirmed candidate construction carries source-backed pclk/hclk,
  tsu_clk, tx_clk metadata, read-only baseline requirements, future
  write-backed invariants, rejected claims, and retained risks.
- fixed: confirmed the paired control uses the same report path while
  withholding candidate-only Ethernet clock/reset facts.
- fixed: confirmed validators reject guard-contract bypasses and forbidden
  runtime, hardware, ownership, downstream Ethernet, and phase claims.
- fixed: updated Phase 12 docs and roadmap frontier wording for the guard core
  and closeout.
- deferred: serialized Pi 5 read-only baseline proof, exact clock-manager
  register/restore mapping, write-backed clock/reset ownership, GPIO32/PHY
  reset ownership, MDIO/PHY, interrupts, DMA, descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is a static
  closeout.

No findings were removed.

## Accepted Boundary

The accepted boundary is local/static guard evidence only. The guard core proves
that Talos can deterministically construct candidate and paired-control
clock/reset ownership reports from the accepted contract and reject overclaims.
It does not prove hardware visibility, runtime clock/reset ownership, safe
clock/reset writes, Ethernet driver readiness, packet I/O, networking, sockets,
SSH, Phase 12.2, or a phase transition.

Same-shaped local/static guard retries are closed for this candidate/control
pair unless a future task supplies materially different evidence or acceptance
criteria.

## Selected Next Action

The next mechanically objective task is
phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof-20260610.

That task is already queued with explicit scope, non-goals, acceptance
criteria, validation gates, docs, and evidence requirements. It must acquire
hardwareTestLock before archive publication, staging, or power cycling. It may
only prove read-only baseline visibility/current-state, a precise
sentinel/fault/blocker with identity retained, or a precise staging/capture
blocker. It must not write RP1 MMIO, write clocks/resets, claim clock/reset
ownership, touch GPIO32/PHY reset, perform MDIO, DMA, descriptor, interrupt,
packet, networking, socket, SSH, Phase 12.2, or phase-transition work.

## Evidence

- static inspection: tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-guard-core.md.
- static inspection: tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-guard-core/classification.json.
- static inspection: tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-guard-core/evidence-map.json.
- static inspection: src/rp1_ethernet.rs.
- static inspection: git show --stat --oneline
  6a9b91175da6442f21733ea1f4f2d8d1f4914ee0.
- JSON validation: jq empty on task-owned evidence-map/classification JSON
  passed.
- diff check: git diff --check passed.
- documentation build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed before commit.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles guard-core evidence without expanding acceptance:
  satisfied.
- Checkpoint states same-shaped local/static guard retries are closed:
  satisfied.
- NextAction selects the read-only baseline Pi 5 proof only because the queued
  task is explicit and mechanically objective: satisfied.
- Accepted checkpoint committed before follow-up starts: satisfied by commit
  recorded in supervisor state after acceptance.
