# Run-Unique Observed GPIO Status Closeout Reconciliation

## Reviewed Evidence

- v2 observed GPIO status closeout:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-closeout.md.
- V3 capture freshness core:
  tasks/2026-06-08-phase11-pi5-capture-freshness-v3-core.md.
- V3 observed GPIO status control blocker:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-v3-control-pi5.md.
- Run-unique capture-marker core:
  tasks/2026-06-08-phase11-pi5-run-unique-capture-marker-core.md.
- Run-unique no-MMIO control proof:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5.md.
- Run-unique real candidate blocker:
  tasks/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-pi5.md.

## Reconciliation

The v2 chain did not accept real GPIO14 STATUS/CTRL visibility because the
candidate and required known-good control failed the repaired pre-power serial
freshness discriminator. V3 was a valid successor discriminator, but the clean
same-shaped control proof found the constant marker already present before
power. That made constant-marker same-shaped retries stale by construction.

The run-unique core repaired only the marker freshness discriminator. It added
task-owned capture-nonce markers and a replay checker that keeps V3's selected
tree, expected-fetch, TFTP, final pre-restore identity, and restore checks.
This is not a GPIO/RP1 hardware feature.

The run-unique no-MMIO control proof passed. Its accepted claim is limited to
the capture/output path: the selected tree, two 49,072-byte candidate TFTP
fetches, final selected-tree identity, restore proof, and nonce-bearing control
markers were coherent.

The real run-unique candidate did not pass. The primary run retained
nonce-bearing result markers and marker-visible GPIO14 STATUS/CTRL values, but
TFTP/final identity showed baseline state rather than the selected candidate
tree. The clean retry used a fresh nonce but failed serial freshness and again
showed baseline-sized TFTP fetches. Those failures are capture/staging
failures; they are not evidence of accepted GPIO14 STATUS/CTRL visibility.

## Accepted Claims

- The observed GPIO14 STATUS/CTRL source contract and local/static core remain
  accepted.
- The run-unique capture marker contract is accepted as a freshness
  discriminator.
- The run-unique no-MMIO/no-RP1/no-GIC control proof is accepted as control
  output/capture evidence.
- The real run-unique GPIO14 STATUS/CTRL attempt is accepted only as a
  committed capture-staging blocker.

## Rejected Claims

- GPIO14 STATUS/CTRL visibility.
- GPIO ownership.
- Event generation.
- Interrupt pending generation or delivery.
- GIC acknowledgement.
- Endpoint ownership.
- Broad RP1 mapping.
- Pad/RIO/clock/reset ownership.
- DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or
  phase transition.

## Next Action

No worker-owned task is mechanically unblocked by this closeout. Supervisor
planning is required before any further same-shaped GPIO14 STATUS/CTRL retry or
new Milestone 11.2 frontier work.
