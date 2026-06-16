# Phase 12.1 RP1 Ethernet BCM54213PE Post-TX-Order Link-Not-Ready Source Checkpoint

Task id: phase12-rp1-ethernet-bcm54213pe-post-txorder-link-not-ready-source-checkpoint-20260616

Status: accepted

Classification:
bcm54213pe-post-txorder-link-not-ready-no-distinct-source-backed-discriminator-pause

Evidence level: static/source/task evidence inspection, task-owned JSON
evidence, docs build, and diff checks. No runtime code change, Pi 5 hardware
run, boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, TFTP/serial capture, GPIO32/PHY reset action, MDIO/Broadcom write,
interrupt ownership, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Reconcile the accepted corrected BCM54213PE RGMII delay TX-order
timeout/link-not-ready frontier against the retained Phase 12.1 source and
evidence set. Select exactly one qualitatively distinct next discriminator only
if it is source-backed and feature-led; otherwise record an explicit pause so
supervisor planning can choose the next boundary.

## Scope Performed

- Inspected the accepted TX-order closeout and retained Pi 5 proof facts.
- Reconciled the prior BMCR/autoneg restart, convergence timeout, RGMII delay,
  TX selected-read, GPIO32/reset, physical-link, BCM54213PE config_init, and
  read-only preflight evidence.
- Rejected the queued link-ready packet-readiness checkpoint because no
  link-ready or autoneg-complete frontier is accepted.
- Rejected same-shaped status polling, BMCR restart retry, convergence wait
  tuning, and marker/capture-only retry shapes.
- Recorded an explicit source checkpoint pause because the remaining possible
  feature paths require supervisor selection of new scope before implementation.

## Findings

- fixed: the accepted TX-order closeout remains decisive for the current
  frontier. The corrected candidate reached RX delay read/write/readback, TX
  selected read/readback, observed TX GTXCLK_EN already set, skipped the
  redundant TX write under the accepted policy, executed exactly one BMCR
  restart, and completed eight bounded convergence samples ending
  link-ready-terminal=false.
- fixed: the current feature frontier is link readiness, not packet transport.
  Packet I/O, networking, sockets, SSH, Phase 12.2, and a phase transition stay
  blocked until a future proof accepts link-ready or autoneg-complete evidence.
- blocked: the existing link-ready packet-readiness checkpoint remains
  dependency-gated because the accepted frontier is timeout/link-not-ready, not
  link-ready/autoneg-complete.
- rejected: another BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000/MACB_NSR
  poll, another bare BMCR restart, another convergence wait-count tweak, or
  another marker/capture-only retry would repeat accepted evidence and is not
  selected.
- deferred: GPIO32 / ETH_RST_N reset ownership is the most direct remaining
  feature path, but accepted event-state evidence still says the guarded
  IRQRESET clear preserved no-output invariants while event bits persisted as
  0x08800000. Re-entering reset ownership needs a new supervisor-planned
  discriminator that explains why the persistent event-state blocker no longer
  controls the risk.
- deferred: MII_CTRL1000 master-mode configuration remains source-backed but
  not selected. Linux gates the write on PHY_BRCM_EN_MASTER_MODE, and accepted
  Talos evidence has not selected that dev_flags bit.
- deferred: Broadcom APD, EEE, LED, WOL, expansion, suspend/resume, and
  interrupt paths are broader PHY lifecycle or interrupt-ownership work. They
  need separate source contracts if supervisor selects them later.
- deferred: partner/link-state investigation remains context only. The physical
  Ethernet precondition is accepted, and another operator check or passive
  status sample would not itself advance the feature path.
- removed: no stale source, helper, task, or evidence file was removed.
- not-an-issue: no hardware lock or inconclusive-run triage was needed because
  this checkpoint is source/task/evidence-only.

## Reconciliation

The accepted TX-order proof already exercised the thinnest BCM54213PE feature
path that followed the source-backed config_init branch: RX and TX RGMII delay
handling, exactly one BMCR restart, and bounded convergence. It still ended
link-not-ready. That closes same-shaped delay/restart/status progress for this
frontier.

The next useful step must be qualitatively different. The retained source set
contains plausible categories, but none is mechanically ready inside this task:

- GPIO32 / ETH_RST_N reset ownership is feature-led, but the accepted
  persistent-or-firmware-owned event-state blocker still controls reset risk.
- MII_CTRL1000 master-mode writes are source-backed only under a dev_flags gate
  that is not selected by accepted board evidence.
- Interrupt ISR/IMR/ECR work can acknowledge or configure interrupts and does
  not directly establish link readiness.
- APD, EEE, suspend/resume, LED, WOL, expansion, and MACB/phylink work are
  broader than this checkpoint and need explicit supervisor scope.

Because none of those paths has an already explicit task contract in this
checkpoint, selected_next_task is null and selected_discriminator is null.
Supervisor planning is required before the worker promotes the queued generic
discriminator core or any hardware, reset, interrupt, packet, networking, SSH,
Phase 12.2, or phase-transition work.

## Frontier

Closed input frontier:
bcm54213pe-rgmii-delay-tx-order-frontier-closed-timeout-link-not-ready.

Accepted input evidence: selected-tree identity, same-power-cycle TFTP byte
agreement, serial freshness, final identity, restore proof, no-MDIO/no-Ethernet
control, RX delay read/write/readback, TX selected read/readback with GTXCLK_EN
already set, skipped redundant TX write under the accepted policy, exactly one
BMCR restart, and bounded convergence samples ending link-not-ready.

Checkpoint result:
bcm54213pe-post-txorder-link-not-ready-no-distinct-source-backed-discriminator-pause.

Selected next task: null.

Selected discriminator: null.

Required next boundary: supervisor planning for a new source-backed feature
path or explicit longer pause.

## Evidence

- TX-order closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-closeout.md.
- TX-order closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-closeout/classification.json.
- TX-order Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/classification.json.
- BCM54213PE config-init source contract:
  tasks/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract.md.
- BCM54213PE read-only source contract:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract.md.
- GPIO32 reset and event-state context:
  docs/src/project/phase12-networking-ssh.md.
- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-txorder-link-not-ready-source-checkpoint/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-txorder-link-not-ready-source-checkpoint/evidence-map.json.

## Acceptance Check

- Accepted post-TX-order timeout/link-not-ready frontier reconciled against
  prior BMCR/autoneg, convergence, RGMII delay, GPIO32/reset, physical-link,
  and BCM54213PE source evidence: satisfied.
- Exactly one next discriminator was selected or explicit pause/blocker was
  recorded with planningNeeded=true: satisfied with explicit pause.
- Same-shaped status polling and wait tweaks are rejected: satisfied.
- Packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
  rejected unless a future accepted proof establishes link-ready/autoneg-complete:
  satisfied.

## Validation

- static/source/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Stop at supervisor planning. Do not promote
phase12-rp1-ethernet-bcm54213pe-link-not-ready-discriminator-core-20260616
unless a future supervisor update or accepted task supplies an exact selected
discriminator and source surface. Do not run hardware, mutate the lab, perform
GPIO32/PHY reset action, touch interrupt ownership, start packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition from this
checkpoint.
