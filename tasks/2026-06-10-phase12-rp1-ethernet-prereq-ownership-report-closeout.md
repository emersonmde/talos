# Phase 12 RP1 Ethernet Prerequisite Ownership Report Closeout

Task id: phase12-rp1-ethernet-prereq-ownership-report-closeout-20260610
Status: accepted
Owner: worker
Evidence level: static inspection, JSON validation, docs validation, diff checks

## Goal

Close out the local/static prerequisite ownership report frontier before any
serialized Pi 5 proof.

## Scope

- Reconciled the accepted source contract
  phase12-rp1-ethernet-prereq-ownership-source-contract-20260610.
- Reconciled the accepted report-core implementation and evidence at commit
  769d5ef4e7224b9e89a5c249e987dc7335678b50.
- Preserved the accepted local/static candidate and paired control report
  claims without expanding them to runtime ownership or Ethernet readiness.
- Closed same-shaped local/static report retries for this candidate/control
  pair unless future scope supplies materially different evidence or
  acceptance criteria.
- Selected the queued serialized Pi 5 prerequisite proof as mechanically
  objective, limited to report visibility/control output.

## Non-Goals

No runtime source changes, no Pi 5 hardware run, no boot archive publication,
no hardwareTestLock acquisition, no RP1 MMIO writes, no clock/reset writes, no
GPIO/RIO/pad writes, no PHY reset assertion/deassertion, no MDIO
transactions, no DMA, no descriptor rings, no interrupts, no packet I/O, no
networking, no sockets, no SSH, no Phase 12.2 work, and no phase transition.

## Reconciliation

The accepted source contract selected a local/static ownership report for the
write-backed and ownership-backed prerequisites that Linux's RP1 GEM path
uses before packet behavior: pclk/hclk/tsu_clk/tx_clk, GPIO32 PHY reset,
MDIO/PHY handling, RP1_INT_ETH, DMA descriptor rings, and interrupt/completion
handling. It deliberately selected no new hardware read field and no ownership
write.

The accepted report-core implementation matches that contract. Its candidate
report carries:

- contract id phase12-rp1-ethernet-prereq-ownership-contract-v1;
- source task id phase12-rp1-ethernet-prereq-ownership-source-contract-20260610;
- observed-window MACB_MID identity context at 0x1c001000fc with raw 0x70109,
  idnum 0x7, rev 0x109, marked context-only;
- rp1_eth source window metadata and translated comparator/sentinel metadata;
- RP1_INT_ETH 6;
- pclk/hclk/tsu_clk/tx_clk clock names and clock ids 12, 12, 29, and 16;
- no-clock-reset-ownership policy;
- RGMII-ID phy1 / ethernet-phy@1 / reg 0x1 and RP1 GPIO32 active-low reset
  duration 5 ms;
- no-phy-reset-or-mdio-ownership policy;
- no-live-dma-or-descriptor-ownership policy;
- Cadence/RP1 config metadata and source evidence paths;
- rejected runtime/hardware claims and retained risks.

The paired control uses the same report path while withholding the
candidate-only Ethernet prerequisite facts and carrying
no-ownership-no-ethernet-rp1-ethernet-prereq-control. That control boundary is
kept intact for the serialized proof.

## Accepted Claims

- deterministic local/static RP1 Ethernet prerequisite ownership candidate
  report construction;
- deterministic paired no-ownership/no-Ethernet prerequisite control report
  construction through the same report path;
- source-backed prerequisite metadata for rp1_eth clocks, interrupt, PHY reset,
  PHY/MDIO, and DMA/descriptor dependencies;
- observed-window MACB_MID identity retained as context-only metadata;
- validators reject source-shape bypasses and forbidden
  ownership/readiness claims.

## Rejected Claims

- Ethernet driver readiness;
- broad Ethernet MMIO readiness;
- RP1 MMIO writes;
- clock/reset ownership or writes;
- GPIO32 ownership or PHY reset assertion/deassertion;
- MDIO transactions or PHY ownership;
- interrupt delivery, handler ownership, or completion;
- DMA, descriptor rings, channel ownership, or transfer completion;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2 work;
- phase transition;
- hardware ownership or runtime prerequisite ownership.

## Retained Risks

- observed-window MACB_MID identity does not prove clocks, PHY, MDIO, DMA,
  interrupts, or packet behavior;
- source facts identify required prerequisites but not Talos ownership;
- report-core evidence is local/static and must not be treated as hardware
  proof;
- the selected serialized proof can prove only report visibility/control
  output unless future scope supplies different acceptance criteria.

## Same-Shaped Retry Policy

Same-shaped local/static report-core retries are closed for the current
candidate/control pair. A future local/static retry needs materially different
evidence, a different report contract, or different acceptance criteria from a
supervisor-planned task.

## Findings

- fixed: reconciled accepted report-core evidence against the source contract
  without expanding acceptance to runtime or hardware ownership.
- fixed: corrected the accepted report-core evidence JSON commit fields from
  pending to 769d5ef4e7224b9e89a5c249e987dc7335678b50.
- fixed: documented accepted local/static report claims and retained rejected
  runtime/hardware claims.
- fixed: closed same-shaped local/static report retries for this exact
  candidate/control pair.
- fixed: selected the queued serialized Pi 5 prerequisite proof as
  mechanically objective because it is limited to capture-chain visibility of
  the already accepted report output and paired control path.
- deferred: actual clock/reset, GPIO32/PHY reset, MDIO/PHY, interrupt, DMA,
  descriptor-ring, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition work remain future tasks.
- not-an-issue: no hardwareTestLock was acquired because this closeout is
  docs/evidence-only.

No findings were removed.

## Evidence

- static inspection: accepted source contract, accepted report-core task
  record, report-core JSON, src/rp1_ethernet.rs, phase12 project docs, and
  roadmap.
- JSON validation: jq empty on task-owned evidence-map/classification JSON.
- diff check: git diff --check.
- docs validation: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles accepted report-core evidence against the source
  contract without expanding acceptance to runtime/hardware ownership or
  Ethernet readiness: satisfied.
- Checkpoint states same-shaped local/static report retries are closed unless
  future scope supplies materially different evidence or acceptance criteria:
  satisfied.
- Checkpoint nextAction explicitly selects the serialized Pi 5 prerequisite
  proof task: satisfied.
- Accepted checkpoint is committed before any Pi 5 prerequisite proof starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-prereq-ownership-pi5-proof-20260610 on the next worker
wake if hardwareTestLock is unlocked. That task may acquire hardwareTestLock
and run only the serialized candidate/control prerequisite report visibility
proof selected here. It must not program RP1 MMIO, write clocks/resets/GPIO,
assert or deassert PHY reset, perform MDIO transactions, create DMA
descriptors or rings, claim interrupts/completions, perform packet I/O, add
networking/sockets/SSH, start Phase 12.2, or infer a phase transition.
