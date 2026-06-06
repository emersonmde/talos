# Phase 11 Pi 5 Capture Transaction V2 Closeout

Task id: phase11-pi5-capture-transaction-v2-closeout-20260606

Status: accepted

## Goal

Reconcile the capture-transaction forensics and no-RP1-MMIO sentinel proof into
an explicit decision on whether the RP1 UART0 FR-read candidate is mechanically
safe to retry.

## Scope

- Reviewed the accepted capture-transaction forensics core evidence.
- Reviewed the accepted no-RP1-MMIO sentinel hardware evidence.
- Recorded findings with disposition.
- Stated accepted and unaccepted claims, deferred risks, and the exact next
  mechanically unblocked task.
- Updated the Phase 11 contract, lab-controller proof notes, and roadmap for
  the accepted proof-chain boundary.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition, RP1
source change, RP1 constants change, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
transition.

This closeout does not accept RP1 UART0 FR mapped/read-value behavior,
bus-fault/trap behavior, or firmware-state behavior. It only decides whether
the repaired capture transaction is strong enough to run the already queued
RP1 FR-read v2 proof task.

## Classification

proof-chain-ready-for-rp1-fr-read-v2.

The forensics core accepted pi5-capture-transaction-v2 as the repaired proof
contract. The contract requires selected candidate identity, effective kernel,
expected fetch path and byte count, an empty pre-power /serial/read drain,
fresh serial evidence, stable same-cursor TFTP evidence before restore, final
pre-restore selected identity, restore identity, and one shared run label.
Replaying the old f274ff7 candidate under that contract remained
capture-staging-blocked because it lacked the empty drain proof and had
restored-tree TFTP/final identity.

The no-RP1-MMIO sentinel then passed that v2 contract on Pi 5 hardware. The
selected archive was target/talos-rpi5-rp1-final-preload-marker-hold-core.tar.gz,
SHA-256 07af64b86908f36c63d368589d79c76aebd492a81906a39586a2c5902d8b9287,
staged as tree 101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47
with effective kernel kernel_2712.img and a 45,816-byte
da591740/kernel_2712.img. The clean rerun proved an empty pre-power
/serial/read drain, two stable 45,816-byte candidate TFTP fetches, final
pre-restore selected-tree identity, 7,489 retained occurrences of
TALOS: fr-final-preload-hold-loop, and restore to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Together, those accepted records satisfy the closeout criterion for
proof-chain-ready-for-rp1-fr-read-v2. The next mechanically unblocked task is
the already queued phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5-20260606,
provided hardwareTestLock remains unlocked/restored at promotion time.

## Findings And Disposition

- fixed: reconciled the accepted forensics repair with the accepted no-MMIO
  sentinel hardware proof under the same v2 contract.
- fixed: retained the decisive sentinel identity join: selected tree,
  effective kernel, expected 45,816-byte fetches, empty pre-power serial drain,
  fresh marker serial, final pre-restore identity, and restore proof.
- fixed: updated the Phase 11 contract and roadmap to state that the proof
  chain is ready for the queued RP1 UART0 FR-read hold-control v2 proof.
- fixed: updated the lab-controller proof notes to make the current v2
  readiness boundary explicit for the next RP1 proof.
- removed: the compromised first sentinel attempt remains excluded from
  accepted feature evidence because restore happened while the capture session
  was still running.
- removed: retained f274ff7 saturated serial output still cannot be promoted to
  RP1 behavior because it lacks the v2 empty-drain identity join.
- deferred: the queued RP1 FR-read v2 proof must still prove selected
  hold-control candidate identity and serial/TFTP/final/restore join before
  accepting mapped/read-value, bus-fault/trap, or other RP1 behavior.
- not-an-issue: no hardware lock acquisition was required for this static
  closeout.

## Evidence

- Static closeout inspection:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-v2-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-v2-closeout/evidence-map.json.
- Forensics core task:
  tasks/2026-06-06-phase11-pi5-capture-transaction-forensics-core.md.
- Forensics core classification:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-forensics-core/classification.json.
- No-MMIO sentinel task:
  tasks/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5.md.
- No-MMIO sentinel classification:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5/classification.json.
- No-MMIO sentinel evidence map:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5/evidence-map.json.

## Validation

- static evidence inspection: completed for forensics and sentinel records.
- git diff --check: passed.
- mdbook build: passed because docs/src changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as proof-chain-ready-for-rp1-fr-read-v2.

The next mechanically unblocked task is
phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5-20260606 on a later worker
wake if hardwareTestLock remains unlocked/restored. No broader Phase 11 work,
Milestone 11.2 work, or RP1 behavior claim is accepted by this closeout.
