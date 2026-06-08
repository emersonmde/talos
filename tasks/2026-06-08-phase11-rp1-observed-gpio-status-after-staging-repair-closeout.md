# Phase 11 RP1 Observed GPIO Status After Staging Repair Closeout

Task id: phase11-rp1-observed-gpio-status-after-staging-repair-closeout-20260608

Status: accepted

Classification: observed-gpio14-status-ctrl-visible-frontier-closed

## Goal

Close out the boot-staging repair and observed GPIO14 STATUS/CTRL retry chain,
reconciling accepted claims, rejected claims, retained risks, and the next
Milestone 11.2 planning boundary.

## Scope

- Reconciled the boot-staging identity repair, the known-good no-MMIO control,
  the repaired real observed GPIO14 STATUS/CTRL proof, restore evidence, and
  retained blocker history.
- Recorded the accepted and unaccepted claims for GPIO14 STATUS/CTRL
  visibility, GPIO ownership, event generation, interrupts, endpoint ownership,
  broad RP1 mapping, DMA/cache, storage, generated-root, networking, SSH,
  Milestone 11.3, and phase transition.
- Updated roadmap and RP1/PCIe map contract docs for the accepted frontier.
- Set the durable state handoff to supervisor planning because this closeout
  creates no worker-owned next task.

## Findings And Disposition

- fixed: the boot-staging identity repair explains why prior nonce-bearing
  serial GPIO14 STATUS/CTRL text was retained but not acceptable when TFTP and
  final identity matched the baseline tree.
- fixed: the boot-staging identity checker now requires selected-tree identity,
  expected TFTP fetch bytes, final pre-restore selected-tree identity, and
  restore proof before any marker-visible serial result can support an RP1/GPIO
  claim.
- fixed: the no-MMIO/no-RP1 known-good control passed the repaired staging
  procedure with selected tree
  35a30932a7f8e76d8cfa657b7419ec1d5e7e8ce450c5ae898c32e957636734f1, two
  49,072-byte candidate TFTP fetches, final selected-tree identity, run-unique
  checker pass, boot-staging identity checker pass, and restore proof.
- fixed: the real observed GPIO14 STATUS/CTRL retry passed the repaired
  procedure with selected tree
  5a499384497595de18d05f250fe146352d964953c9ff759642cc8d20384e0ea6, two
  49,784-byte candidate TFTP fetches, final selected-tree identity, 38
  task-owned result records, gpio14-status-raw=0xabe3300,
  gpio14-ctrl-raw=0x84, ctrl-funcsel=4, run-unique checker pass,
  boot-staging identity checker pass, and restore proof.
- fixed: accepted claims are limited to read-only observed GPIO14 STATUS/CTRL
  visibility under the repaired boot-staging identity procedure.
- fixed: docs now record the accepted frontier and retained risks without
  implying GPIO ownership, event generation, interrupt pending/delivery, GIC
  acknowledgement, endpoint ownership, broad RP1 mapping, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, or phase transition.
- deferred: repository evidence still does not explain the lab mechanism that
  produced the earlier selected-tree/TFTP/final identity mismatch; the accepted
  checker quarantines that risk for future hardware tasks.
- deferred: next Milestone 11.2 work requires supervisor planning around a
  new explicit feature slice or discriminator. Same-shaped GPIO14 STATUS/CTRL
  reruns are closed unless future planning supplies new acceptance criteria.
- not-an-issue: this closeout performs no hardware run and does not acquire
  hardwareTestLock.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-closeout/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-closeout/classification.json.
- Boot-staging identity repair record:
  tasks/2026-06-08-phase11-pi5-boot-staging-identity-repair-core.md.
- Known-good control record:
  tasks/2026-06-08-phase11-pi5-boot-staging-identity-known-good-control-pi5.md.
- Real observed GPIO14 STATUS/CTRL proof record:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5.md.
- Repair/control/real retained evidence maps and checker outputs under:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-repair-core/,
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-known-good-control-pi5/,
  and
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5/.

## Validation

- static inspection: repair, control, real proof, restore evidence, retained
  classification JSON, and evidence maps inspected.
- jq empty on retained evidence-map/classification JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as observed-gpio14-status-ctrl-visible-frontier-closed. The frontier
is limited to the observed GPIO14 STATUS/CTRL source contract, local/static
core, serial-drain repair, run-unique capture marker contract, boot-staging
identity discriminator, no-MMIO/no-RP1/no-GIC control proof, and real Pi 5
read-only observed GPIO14 STATUS/CTRL visibility proof. It does not accept GPIO
ownership, event generation, interrupt pending/delivery, GIC acknowledgement,
endpoint ownership, broad RP1 mapping, pad/RIO/clock/reset ownership,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or phase
transition.

No mechanically unblocked worker-owned next task remains in the explicit queue.
Supervisor planning is required for the next Milestone 11.2 feature slice.
