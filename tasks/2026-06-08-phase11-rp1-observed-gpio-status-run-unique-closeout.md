# Phase 11 RP1 Observed GPIO Status Run-Unique Closeout

Task id: phase11-rp1-observed-gpio-status-run-unique-closeout-20260608

Status: accepted

Classification: observed-gpio-status-run-unique-capture-blocked-frontier-closed

## Goal

Close out the run-unique observed GPIO14 STATUS/CTRL capture chain and record
which claims are accepted, rejected, or still blocked.

## Scope

- Reconciled the v2/v3 blockers, run-unique capture-marker core, run-unique
  no-MMIO control proof, real run-unique blocker, restore evidence, accepted
  claims, rejected claims, and retained risks.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for observed GPIO14 STATUS/CTRL
  visibility, GPIO ownership, event generation, interrupt pending/delivery, GIC
  acknowledgement, endpoint ownership, broad RP1 mapping, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, and phase transition.
- Updated roadmap and RP1/PCIe map contract docs for the closed run-unique
  blocker frontier.
- Set nextAction to supervisor planning rather than creating a worker-owned
  follow-up task.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, GPIO writes, interrupt enablement or delivery, GIC
acknowledgement, endpoint config retry, bridge setup write, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, phase transition, or capture
freshness relaxation.

## Findings And Disposition

- fixed: reconciled the earlier v2 real GPIO14 STATUS/CTRL blocker. The
  marker-visible values remained non-decisive because the repaired v2
  pre-power serial freshness gate failed for both the real candidate and the
  required known-good control.
- fixed: reconciled the V3 freshness blocker. The V3 rule correctly rejected
  same-shaped retained evidence when the constant required marker was already
  visible before power; this did not accept or reject RP1/GPIO hardware
  behavior.
- fixed: reconciled the run-unique capture-marker core. The accepted
  pi5-capture-transaction-run-unique-v1 replay keeps V3 identity checks and
  adds a task-owned capture-nonce marker requirement, without changing the
  GPIO/RP1 source contract.
- fixed: reconciled the run-unique no-MMIO/no-RP1/no-GIC control proof as
  no-mmio-observed-gpio-status-run-unique-control-visible. The proof selected
  tree 2e0fbbdc8da0ec3066ddc4b74949887c8bcf80c70ac6c4a68edffb5dca6f5173,
  retained empty-read-before-power, saw the nonce marker after power, observed
  two matching 49,072-byte da591740/kernel_2712.img TFTP fetches, kept final
  identity on the selected tree, and restored the lab to the baseline tree.
- fixed: reconciled the real run-unique candidate as capture-staging-blocked.
  The primary run retained 41 nonce-bearing result markers and marker-visible
  gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, ctrl-funcsel=4, but the
  run-unique checker rejected the proof for TFTP expected-fetch byte mismatch
  and final selected-tree identity mismatch.
- fixed: reconciled the clean same-shaped retry. It used a fresh nonce, drained
  1,095,168 bytes without reaching empty-read-before-power, did not observe the
  required nonce marker after power, saw baseline-sized 104,136-byte TFTP
  fetches, and restored the lab.
- fixed: recorded accepted claims precisely. The run-unique capture marker
  contract and run-unique no-MMIO control proof are accepted; the real
  GPIO14 STATUS/CTRL run is accepted only as a committed capture-staging
  blocker.
- fixed: made the same-shaped retry policy explicit. Same-shaped observed
  GPIO14 STATUS/CTRL hardware retries remain blocked unless supervisor
  planning supplies a different capture/staging repair, a different
  discriminator, or new acceptance criteria.
- deferred: supervisor planning must choose any later capture-staging repair,
  alternate RP1/GPIO discriminator, GPIO/event-generation path,
  interrupt-delivery work, or blocker-driven Milestone 11.2 slice. This
  closeout creates no worker-owned follow-up task.
- deferred: GPIO14 STATUS/CTRL visibility, GPIO ownership, event generation,
  interrupt pending generation, interrupt delivery, GIC acknowledgement,
  endpoint ownership, broad RP1 mapping, pad/RIO/clock/reset ownership,
  DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, and
  phase transition remain outside this accepted frontier.
- not-an-issue: the marker-visible real serial values are retained evidence,
  but they are non-decisive because the run identity joined to baseline TFTP
  and final selected-tree state instead of the staged candidate.

No findings were removed.

## Closeout Classification

Accepted as observed-gpio-status-run-unique-capture-blocked-frontier-closed.

The accepted frontier is limited to the source/evidence-backed GPIO14
STATUS/CTRL observed-aperture contract, the local/static real/control core,
the serial-drain freshness repair procedure, the run-unique capture marker
contract, the run-unique no-MMIO/no-RP1/no-GIC control proof, and the committed
real Pi 5 capture-staging blocker. The selected real operation remains exactly
two 32-bit volatile reads from observed CPU physical addresses 0x1c000d0070
and 0x1c000d0074.

The real candidate's marker-visible values are retained but not accepted as
GPIO14 STATUS/CTRL visibility. The primary run-unique proof failed because
the retained TFTP/final identity matched the baseline tree, not the selected
candidate tree. The clean retry failed the same proof chain before accepting a
fresh marker after power and also retained baseline-sized TFTP fetches.

This closeout does not accept GPIO14 STATUS/CTRL visibility, GPIO ownership,
event generation, interrupt pending generation, interrupt delivery, GIC
acknowledgement, endpoint ownership, broad RP1 mapping, pad/RIO/clock/reset
ownership, DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3,
or a phase transition.

Same-shaped endpoint config identity, bridge/setup-state, 0x1f RP1 peripheral,
0x1c UART0 FR, and real 0x1c GPIO14 STATUS/CTRL hardware reruns remain closed
unless a future supervisor task supplies a different discriminator, a
capture/staging repair, or new acceptance criteria. Supervisor planning is
required for the next Milestone 11.2 frontier.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-closeout/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-closeout/classification.json.
- Static reconciliation:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-closeout/static-reconciliation.md.
- Run-unique capture-marker core:
  tasks/2026-06-08-phase11-pi5-run-unique-capture-marker-core.md.
- Run-unique control Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5.md.
- Run-unique real Pi 5 blocker:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-pi5.md.
- Classification records:
  tasks/evidence/2026-06-08-phase11-pi5-run-unique-capture-marker-core/classification.json,
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5/classification.json,
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-pi5/classification.json.

## Validation

- static inspection: v2/v3 blocker evidence, run-unique core, run-unique
  control proof, real run-unique blocker, restore evidence, and evidence maps
  inspected.
- jq empty on retained evidence-map and classification JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as observed-gpio-status-run-unique-capture-blocked-frontier-closed.

Next action: no worker-owned follow-up task is created by this closeout. Set
planningNeeded=true for supervisor planning of the next Milestone 11.2 frontier
if work should continue.
