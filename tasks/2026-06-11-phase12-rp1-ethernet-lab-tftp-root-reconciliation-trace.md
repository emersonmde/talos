# Phase 12 RP1 Ethernet Lab TFTP Root Reconciliation Trace

Task: `phase12-rp1-ethernet-lab-tftp-root-reconciliation-trace-20260611`

## Goal

Trace and classify the lab publication path versus the dnsmasq-served TFTP
root/cache without running hardware, so the failed selected-tree invariant has
a concrete repair target or explicit external blocker.

## Scope

- Acquired `hardwareTestLock` before mutating the shared lab boot root.
- Published a minimal no-MDIO/no-Ethernet/no-MMIO-target staging sentinel
  archive.
- Captured baseline, post-publish, restore, post-restore, boot-file, and TFTP
  cursor evidence through the lab-controller API.
- Inspected the publication/capture helper assumptions relevant to selected
  tree identity.
- Did not power-cycle hardware, read serial, write serial, retry register-vector
  code, or make Ethernet/runtime changes.

## Findings

- not-an-issue: `scripts/rpi5-proof-identity-join-v4-check.sh` already rejects
  API-visible `/boot/files` identity as sufficient. It requires
  same-power-cycle TFTP-served bytes, final pre-restore identity, restore proof,
  and serial freshness before decisive RP1 hardware classification.
- not-an-issue: the staging sentinel archive helper produced root and
  `da591740/` copies with the no-MDIO/no-Ethernet/no-MMIO-target scenario.
  Post-publish `/boot/files` reported `da591740/kernel_2712.img` at the
  archive byte count, 47,816 bytes.
- deferred: current repository helpers cannot prove the actual dnsmasq-served
  root/cache without either a hardware fetch or a lab-service endpoint exposing
  that serving root independently of the API-visible boot tree.
- fixed: the lab boot root was restored to the baseline tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
  before releasing `hardwareTestLock`.

## Evidence

- classification:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-lab-tftp-root-reconciliation-trace/classification.json`
- evidence map:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-lab-tftp-root-reconciliation-trace/evidence-map.json`
- no-power summary:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-lab-tftp-root-reconciliation-trace/no-power-trace-summary.json`
- archive identity:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-lab-tftp-root-reconciliation-trace/archive-identity.json`

The no-power trace showed:

- baseline tree:
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
- post-publish API-visible tree:
  `48571aa9965e5d2926d2ca5f9651fae90eb5923f63bb651237203442f101bf77`
- post-publish effective kernel: `kernel_2712.img`
- expected fetch: `da591740/kernel_2712.img`
- archive and API-visible post-publish fetch bytes: 47,816
- TFTP events after publish without power: 0
- post-restore tree:
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`

## Classification

`lab-api-service-blocked-live-tftp-root-unobservable`

The lab-controller API-visible publication path changed `/status` and
`/boot/files` as expected, and the restore path returned to baseline. Because
no hardware fetch was allowed, `/tftp/logs` correctly showed no new fetches.
The deployed API does not expose a read-only proof of the actual dnsmasq-served
root/cache. That leaves the prior hardware contradiction unresolved at the lab
service boundary rather than in Talos runtime MDIO/MAN code.

## Next Valid Path

Block for lab-service/operator reconciliation before any further MDIO
register-vector hardware retry. The next mechanically valid path is to expose
or repair a read-only lab API check of the actual dnsmasq-served TFTP root/cache
from the same serving path, or to explicitly authorize operator-level
inspection. Do not treat API-visible `/status` or `/boot/files` identity
alone as sufficient to retry register-vector hardware.

## Validation

- static inspection: publication/capture helpers and lab-controller docs.
- image/archive inspection: no-MDIO/no-Ethernet sentinel archive strings,
  file list, kernel sizes, and hashes.
- lab-controller API: baseline, publish, post-publish, TFTP cursor/delta,
  restore, and post-restore evidence.
- jq empty: passed for task-owned JSON evidence.
- git diff --check: passed.
- mdbook build: not required; no `docs/src` files were touched.
- git diff --cached --check: passed.
