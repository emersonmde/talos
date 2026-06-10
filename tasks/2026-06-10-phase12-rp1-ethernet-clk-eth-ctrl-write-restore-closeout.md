# Phase 12 RP1 Ethernet CLK_ETH_CTRL Write-Restore Closeout

Task id: phase12-rp1-ethernet-clk-eth-ctrl-write-restore-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clk-eth-ctrl-write-restore-core-frontier-closed
Evidence level: static inspection of accepted source contract, accepted
local/static core evidence, tests, documentation, and git history.

## Goal

Close out the accepted local/static CLK_ETH_CTRL write/restore core and decide
whether the serialized Pi 5 proof is mechanically objective.

## Findings

- fixed: reconciled the accepted CLK_ETH_CTRL source contract
  rp1-ethernet-clk-eth-ctrl-source-contract-accepted from commit
  fc12771db08a5837d304ecf6e8ed254e1f456db0.
- fixed: reconciled the accepted write/restore core classification
  rp1-ethernet-clk-eth-ctrl-write-restore-core-local-static-accepted from
  commit e8cc2925bcb3f43286c0dbb07296c9abf59f4026.
- fixed: confirmed the local/static candidate report preserves the exact
  selected target, CLK_ETH_CTRL at 0x1c00018064, allowed write value
  pre-read-raw-value-only, operation sequence, preserved-field inventory,
  safety invariants, retained risks, and future proof classification set.
- fixed: confirmed the paired control uses the same report path while
  constructing no writable clock target and withholding candidate-only
  CLK_ETH_CTRL facts.
- fixed: confirmed focused tests covered accepted candidate, accepted control,
  and deterministic rejection of source-contract bypass, shared RP1_CLK_SYS
  pclk/hclk writes, same-shaped TSU retry claims, non-idempotent transitions,
  reset/GPIO/MDIO/DMA/descriptor/interrupt/packet/network/socket/SSH claims,
  Phase 12.2, and phase-transition claims.
- fixed: confirmed no candidate/control boot scenarios were added or changed
  by the local/static core, so the conditional boot-scenario compile gates did
  not apply.
- fixed: selected the serialized Pi 5 CLK_ETH_CTRL candidate/control proof as
  mechanically objective because the accepted contract/core already define the
  exact candidate target, paired control, allowed classifications, validation
  gates, restore requirements, and evidence JSON.
- not-an-issue: hardwareTestLock was not acquired because this closeout is
  static reconciliation only.
- deferred: serialized Pi 5 write/restore proof, hardware restore evidence,
  broad clock/reset ownership, shared-clock ownership, reset-controller
  ownership, GPIO32/PHY reset, MDIO/PHY, interrupts, DMA, descriptors, packet
  I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
  future or explicitly rejected scope.

No findings were removed.

## Accepted Boundary

The accepted frontier is closed at a local/static report surface for one exact
Ethernet-private transmit-clock control target. The core provides a candidate
report for CLK_ETH_CTRL at 0x1c00018064 and a paired no-clock-write/no-Ethernet
control through the same output path. It does not prove a runtime RP1 MMIO
write, hardware readback, restore behavior on Pi 5, broad clock/reset
ownership, Ethernet driver readiness, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

The next serialized proof is mechanically objective because the accepted
source contract names one target, operation sequence, preserved fields, paired
control requirements, rejected claims, retained risks, and classification set;
the accepted core implements and tests the candidate/control reporting surface;
the proof task already defines candidate/control scenarios, capture gates,
restore requirements, and task-owned evidence artifacts; and hardwareTestLock
is currently unlocked.

## Same-Shaped Retry Policy

Same-shaped local/static CLK_ETH_CTRL write/restore report retries are closed
for this candidate/control pair. Future local/static work needs materially
different scope or acceptance criteria. The next selected boundary is the
serialized Pi 5 candidate/control write/restore proof.

## Evidence

- Source contract task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-source-contract.md.
- Source contract classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-source-contract/classification.json.
- Core task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-core.md.
- Core classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-core/classification.json.
- Core evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-core/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-closeout/evidence-map.json.

## Validation

- static inspection: accepted source contract, core task record,
  classification/evidence JSON, tests, docs, and git history reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof-20260610 on the
next worker wake if hardwareTestLock remains unlocked. The proof must acquire
hardwareTestLock before archive publication, staging, or power cycling and
capture candidate/control selected-tree identity, expected TFTP fetches, fresh
serial markers, final pre-restore identity, restore verification,
classification JSON, evidence map, capture summary, and archive reviews. Do
not broaden to clock/reset ownership, shared-clock ownership, reset-controller,
GPIO32/PHY, MDIO, DMA, descriptors, interrupts, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.
