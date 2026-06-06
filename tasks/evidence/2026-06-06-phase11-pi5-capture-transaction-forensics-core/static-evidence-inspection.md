# Static Evidence Inspection

Task id: phase11-pi5-capture-transaction-forensics-core-20260606

## First-Principles Checklist

- Problem statement: the repaired RP1 FR-read candidate run retained
  candidate-like direct-read serial output, but the TFTP delta and final
  pre-restore identity matched the restored known-good tree.
- Invariant: decisive hardware evidence must tie selected candidate boot
  identity, fresh serial, stable TFTP bytes, final pre-restore identity, and
  restore identity to one run label before any RP1 classification is accepted.
- Contradicting evidence: f274ff7 retained 973,431 serial bytes with
  post-read-loop text, while the TFTP delta had two 104,136-byte known-good
  kernel fetches and final pre-restore identity was the restored tree rather
  than the selected 46,320-byte candidate tree.
- Unproven assumptions: saturated direct-read output was fresh after the power
  cycle; the single 500-byte pre-power drain emptied the serial buffer; final
  pre-restore identity was sampled before any restore or external tree change.
- Approach 1: tighten the serial freshness contract by requiring an empty
  pre-power drain before saturated direct-read can count.
- Approach 2: block all direct-read evidence until the lab exposes a monotonic
  serial cursor beyond the retention cap.
- Smallest decisive discriminator: run an already accepted no-RP1-MMIO marker
  sentinel through a v2 bundle that proves empty pre-power serial drain,
  selected-candidate TFTP bytes, final pre-restore identity, and marker output.
- Workaround removal/quarantine: v1 remains historical evidence only; new
  proof tasks use pi5-capture-transaction-v2 and reject stale saturated
  direct-read output without an empty drain.

## Inspected Evidence

- f274ff7 task record:
  tasks/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5.md.
- f274ff7 classification:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/classification.json.
- f274ff7 candidate-run summary:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/candidate-run/capture-invariant-summary.json.
- f274ff7 identity-check replay:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/candidate-run-identity-join-check.json.
- f274ff7 observe-helper trace:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5/observe-helper-trace.stderr.
- Proof helper scripts:
  scripts/rpi5-capture-invariant-proof-bundle.sh and
  scripts/rpi5-proof-identity-join-check.sh.
- Lab contract:
  docs/src/project/lab-controller.md.

## Analysis

The f274ff7 preflight identity was correct: tree
ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0,
effective kernel_2712.img, and expected 46,320-byte
da591740/kernel_2712.img.

The serial cursor was already saturated at 4,194,304. The helper switched to
direct read and retained 973,431 bytes, but the pre-power drain retained 500
bytes of known-good production-timer output and did not prove the serial buffer
was empty before power cycle. That makes the direct-read bytes stale-risky
unless a stricter drain contract is recorded.

The stable TFTP delta and final pre-restore status both matched the restored
known-good tree. This keeps f274ff7 blocked even after the serial freshness
repair, but the missing serial drain was a separate contract gap that could
allow a future saturated direct-read run to look fresher than it is.

## Disposition

- fixed: added v2 serial drain evidence to the capture bundle before taking
  serial and TFTP cursors.
- fixed: v2 checker rejects old bundles without
  serial-drain-before-power.json.
- fixed: v2 checker rejects saturated direct-read unless the pre-power drain
  reached empty.
- fixed: f274ff7 replay now emits capture-staging-blocked under v2 with
  serial-freshness and restored-tree rejection reasons.
- deferred: hardware proof-chain readiness must be demonstrated by the queued
  no-RP1-MMIO sentinel.
- removed: stale saturated direct-read output cannot be used as decisive
  candidate evidence.
- not-an-issue: no RP1 runtime source or constants were required to repair this
  proof contract.
