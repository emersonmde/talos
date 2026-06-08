# Phase 11 RP1 Observed GPIO Status Closeout

Task id: phase11-rp1-observed-gpio-status-closeout-20260608

Status: accepted

Classification: observed-gpio-status-capture-blocked-frontier-closed

## Goal

Close out the observed-aperture GPIO14 STATUS/CTRL discriminator chain and
record whether later GPIO/event/interrupt work is objectively unblocked.

## Scope

- Reconciled the accepted source contract, local/static core, serial-drain
  freshness repair, paired no-MMIO/no-RP1/no-GIC control proof, real Pi 5
  blocker, restore evidence, accepted claims, rejected claims, and retained
  risks.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for observed 0x1c GPIO visibility,
  GPIO ownership, event generation, interrupt pending/delivery, GIC
  acknowledgement, DMA/cache, storage, generated-root, networking, SSH,
  Milestone 11.3, and phase transition.
- Updated roadmap and RP1/PCIe map contract docs for the closed blocker
  frontier.
- Set nextAction to supervisor planning rather than creating a worker-owned
  follow-up task.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, endpoint config retry, BAR discovery or programming,
bridge setup writes, PERST/link-control changes, GPIO/pad/clock/reset writes,
interrupt enablement or delivery, GIC acknowledgement, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, phase transition,
source-contract expansion, or serial freshness relaxation.

## Findings And Disposition

- fixed: reconciled the accepted source contract boundary. The selected real
  operation was exactly two read-only 32-bit volatile loads from observed CPU
  physical addresses 0x1c000d0070 and 0x1c000d0074, corresponding to retained
  RP1 Linux IO_BANK0 GPIO14 STATUS/CTRL source addresses 0xc0400d0070 and
  0xc0400d0074. This was a visibility discriminator only, not a GPIO
  ownership contract.
- fixed: reconciled the local/static core evidence. The real candidate
  implemented the accepted two-read observed-aperture contract, and the paired
  control preserved the output shape without constructing forbidden RP1, PCIe,
  MIP, GIC, GPIO, RIO, pads, clock/reset, DMA, or other MMIO addresses.
- fixed: reconciled the serial-drain freshness repair. The accepted helper
  repair added explicit drain bounds and preserved the v2 rule that saturated
  direct-read serial is decisive only after an empty pre-power /serial/read
  drain.
- fixed: reconciled the paired no-MMIO control proof as
  no-mmio-observed-gpio-status-control-visible. The accepted unchanged rerun
  passed pi5-capture-transaction-v2 with no rejection reasons, retained two
  48,952-byte TFTP fetches, 41 task-owned control markers, final selected-tree
  identity, and restore to the baseline tree.
- fixed: reconciled the real Pi 5 run as capture-staging-blocked. The real
  candidate retained matching selected-tree identity, two 49,656-byte TFTP
  fetches, restore proof, and 42 task-owned result markers, but the pre-power
  serial drain exhausted 96 attempts, read 1,095,168 bytes, and never reached
  empty-read-before-power.
- fixed: reconciled the required known-good control after the inconclusive
  real run. It selected the production-timer tree, retained matching TFTP and
  final identity evidence, but failed the same repaired serial freshness
  discriminator, so the unchanged real candidate was correctly not rerun.
- fixed: recorded the accepted claims precisely. The source contract,
  local/static core, freshness-repair procedure, paired control proof, and
  committed real-run blocker are accepted; GPIO14 STATUS/CTRL visibility from
  the marker-visible real serial output is not accepted.
- fixed: made the same-shaped rerun policy explicit. Same-shaped real GPIO14
  STATUS/CTRL hardware reruns remain closed unless a future supervisor task
  supplies a different discriminator, a first-class freshness repair, or new
  acceptance criteria.
- deferred: supervisor planning must choose any later capture-freshness repair,
  alternate observed-aperture discriminator, GPIO/event-generation path,
  interrupt-delivery work, or blocker-driven Milestone 11.2 slice. This
  closeout creates no worker-owned follow-up task.
- deferred: observed 0x1c GPIO14 STATUS/CTRL visibility, GPIO ownership, event
  generation, interrupt pending generation, interrupt delivery, GIC
  acknowledgement, endpoint ownership, broad RP1 mapping, pad/RIO/clock/reset
  ownership, DMA/cache, storage, generated-root, networking, SSH,
  Milestone 11.3, and phase transition remain outside this accepted frontier.
- not-an-issue: the marker-visible real serial values
  gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, ctrl-funcsel=4, and
  classification=observed-aperture-gpio14-status-ctrl-visible remain retained
  evidence, but they are non-decisive because the capture freshness invariant
  failed in both the candidate and known-good control.

No findings were removed.

## Closeout Classification

Accepted as observed-gpio-status-capture-blocked-frontier-closed.

The accepted frontier is limited to the source/evidence-backed GPIO14
STATUS/CTRL observed-aperture contract, the local/static real/control core,
the serial-drain freshness repair procedure, the paired
no-MMIO/no-RP1/no-GIC control proof, and the committed real Pi 5
capture-staging blocker. The selected real operation was exactly two 32-bit
volatile reads from observed CPU physical addresses 0x1c000d0070 and
0x1c000d0074.

The real candidate's marker-visible values are retained but not accepted as
GPIO14 STATUS/CTRL visibility. pi5-capture-transaction-v2 rejected the real
run for serial-drain-not-empty-before-power and
saturated-direct-read-without-empty-pre-power-drain, and the required
known-good production-timer control failed the same repaired freshness
discriminator. The unchanged real candidate was therefore not rerun.

This closeout does not accept observed 0x1c GPIO14 STATUS/CTRL visibility,
GPIO ownership, event generation, interrupt pending generation, interrupt
delivery, GIC acknowledgement, endpoint ownership, broad RP1 mapping,
pad/RIO/clock/reset ownership, DMA/cache, storage, generated-root, networking,
SSH, Milestone 11.3, or a phase transition.

Same-shaped endpoint config identity, bridge/setup-state, 0x1f RP1 peripheral,
0x1c UART0 FR, and real 0x1c GPIO14 STATUS/CTRL hardware reruns remain closed
unless a future supervisor task supplies a different discriminator or new
acceptance criteria. Supervisor planning is required for the next
Milestone 11.2 frontier.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-closeout/evidence-map.json.
- Static reconciliation:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-closeout/static-reconciliation.md.
- Source contract:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-source-contract.md.
- Local/static core:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-core.md.
- Serial-drain freshness repair:
  tasks/2026-06-08-phase11-pi5-serial-drain-freshness-repair-core.md.
- Control Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry.md.
- Real Pi 5 blocker:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-pi5.md.
- Classification records:
  tasks/evidence/2026-06-08-phase11-pi5-serial-drain-freshness-repair-core/classification.json,
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry/classification.json,
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-pi5/classification.json.

## Validation

- static inspection: source contract, local/static core, repair evidence,
  control proof, real blocker, restore evidence, and evidence maps inspected.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as observed-gpio-status-capture-blocked-frontier-closed.

Next action: no worker-owned follow-up task is created by this closeout.
Set planningNeeded=true for supervisor planning of the next Milestone 11.2
frontier if work should continue.
