# Phase 12 RP1 Ethernet Clock/Reset Write-Restore Closeout

Task id: phase12-rp1-ethernet-clock-reset-write-restore-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clock-reset-write-restore-core-frontier-closed
Evidence level: static inspection of accepted source contract, accepted
local/static core evidence, tests, documentation, and git history.

## Goal

Close out the accepted local/static CLK_ETH_TSU_CTRL write/restore core and
decide whether the serialized Pi 5 proof is mechanically objective.

## Findings

- fixed: reconciled the accepted write-target source contract
  rp1-ethernet-clock-reset-write-target-source-contract-accepted from commit
  c16b209a4fd75199306ec0cf1655c0e5e2e9fbf2.
- fixed: reconciled the accepted write/restore core classification
  rp1-ethernet-clock-reset-write-restore-core-accepted from commit
  26f4ba1e5563abf2bea2b2a649824b8eba05b980.
- fixed: confirmed the local/static candidate report preserves the exact
  selected target, CLK_ETH_TSU_CTRL at 0x1c00018134, allowed write value
  pre-read-raw-value-only, operation sequence, preserved-field inventory,
  safety invariants, retained risks, and future proof classification set.
- fixed: confirmed the paired control uses the same report path while
  withholding writable target construction and candidate-only clock/reset
  facts.
- fixed: confirmed focused tests covered accepted candidate, accepted control,
  and deterministic rejection of shared-clock, CLK_ETH_CTRL, reset-controller,
  GPIO32/PHY, MDIO/PHY, interrupt, DMA/descriptor, packet, networking, SSH,
  Phase 12.2, and phase-transition claims.
- fixed: confirmed candidate/control boot-scenario routing and image helpers
  were compile/syntax checked only and were not staged, archived, or run on
  hardware.
- not-an-issue: hardwareTestLock was not acquired because this closeout is
  static reconciliation only.
- deferred: serialized Pi 5 write/restore proof, hardware restore evidence,
  broad clock/reset ownership, CLK_ETH_CTRL, GPIO32/PHY reset, MDIO/PHY,
  interrupts, DMA, descriptors, packet I/O, networking, sockets, SSH, Phase
  12.2, and phase transition remain future or explicitly rejected scope.

No findings were removed.

## Accepted Boundary

The accepted frontier is closed at a local/static report surface for one exact
Ethernet-private clock-manager target. The core provides a candidate report
for CLK_ETH_TSU_CTRL at 0x1c00018134 and a paired no-clock-write control
through the same output path. It does not prove a runtime RP1 MMIO write,
hardware readback, restore behavior on Pi 5, Ethernet driver readiness, packet
I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

The next serialized proof is mechanically objective because the accepted
source contract names one target, operation sequence, preserved fields, paired
control requirements, rejected claims, retained risks, and classification set;
the accepted core implements and compile-checks the candidate/control reporting
surface; and hardwareTestLock is currently unlocked.

## Same-Shaped Retry Policy

Same-shaped local/static write/restore report retries are closed for this
candidate/control pair. Future local/static work needs materially different
scope or acceptance criteria. The next selected boundary is the serialized Pi 5
candidate/control write/restore proof.

## Evidence

- Source contract task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-target-source-contract.md.
- Source contract classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-target-source-contract/classification.json.
- Core task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-core.md.
- Core classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-core/classification.json.
- Core evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-core/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-closeout/evidence-map.json.

## Validation

- static inspection: source contract, core task record, classification/evidence
  JSON, tests, docs, and git history reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof-20260610 on the next
worker wake if hardwareTestLock remains unlocked. The proof must acquire
hardwareTestLock before archive publication, staging, or power cycling and
capture candidate/control selected-tree identity, expected TFTP fetches, fresh
serial markers, final pre-restore identity, restore verification,
classification JSON, evidence map, capture summary, and archive reviews. Do
not broaden to clock/reset ownership, shared-clock ownership, reset-controller,
GPIO32/PHY, MDIO, DMA, descriptors, interrupts, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.
