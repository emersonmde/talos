# Pi 5 Boot Staging Identity Repair Core Static Analysis

Task id: phase11-pi5-boot-staging-identity-repair-core-20260608

## Problem Statement

The run-unique observed GPIO14 STATUS/CTRL primary run retained nonce-bearing
serial text, but its stable TFTP delta and final pre-restore identity matched
the restored baseline tree instead of the selected candidate tree. A clean retry
with a fresh nonce then saw baseline-sized TFTP fetches and no required marker.

## Invariant

A Pi 5 hardware proof may use serial output only after one boot-staging identity
chain joins:

- pre-power selected tree hash and effective kernel;
- expected `da591740/kernel_2712.img` path and byte count;
- stable same-cursor TFTP delta before restore;
- final pre-restore status and boot files still showing the selected tree;
- post-restore identity recorded after the run.

If any part points at the baseline tree, the run is staging evidence only and
cannot support RP1/GPIO behavior.

## Contradicting Evidence

- accepted control: selected tree
  `2e0fbbdc8da0ec3066ddc4b74949887c8bcf80c70ac6c4a68edffb5dca6f5173`,
  two matching 49,072-byte TFTP kernel fetches, final identity still on the
  selected tree, and restore to baseline.
- primary real run: selected tree
  `37e1259dc7d881008d37c0071e9efa5152042ade0fb87c78e5225f1addf4405d`,
  but two 104,136-byte TFTP kernel fetches and final identity on baseline
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- clean real retry: selected tree
  `93dffa56914a3bd3b2385b888638568f312bafe71b4f49a0ade60a3462252c6b`,
  but two 104,136-byte TFTP kernel fetches and final identity on the same
  baseline tree.

## Explanation

The primary run could retain marker-visible GPIO14 STATUS/CTRL text because the
serial cursor was saturated and the fallback direct-read path can return retained
device-buffer text that is not by itself joined to the just-powered boot. The
run-unique nonce made stale constant-marker reuse less likely, but it did not
make serial text authoritative when TFTP and final identity prove the Pi fetched
the baseline-sized kernel and the visible boot tree had returned to baseline.

The clean retry used a fresh nonce that was not in retained serial text, so it
correctly saw no required marker while TFTP and final identity again showed the
baseline tree.

## Unproven Assumptions

- The reason the lab-visible boot tree returned to baseline between preflight
  and final identity is not proven by repository evidence alone.
- The TFTP bytes annotations are trusted only because both failed runs retained
  stable pre-restore TFTP deltas before restore.
- Serial direct-read retention is treated as a capture/staging risk, not as a
  claim about current hardware behavior.

## Approaches Considered

- No-change procedure only: rely on the existing run-unique checker rejection.
  Rejected because it does not expose boot-staging identity as a standalone
  discriminator before the next hardware task.
- Add a staging-only replay discriminator: accepted. It ignores serial/RP1 text
  and makes selected tree, expected fetch bytes, final identity, and restore the
  first mechanical gate.
- Add lab-controller publish or TFTP service changes: deferred. The current task
  has no external service mutation scope and the retained evidence already gives
  a decisive repository-side discriminator.

## Smallest Decisive Discriminator

`scripts/rpi5-boot-staging-identity-check.sh` replays retained proof-bundle
evidence and emits `boot-staging-identity-ready` only when candidate selected
tree, expected TFTP fetch bytes, final pre-restore selected tree, and restore
identity all match. With the baseline tree supplied, it also rejects a selected
or final identity that is baseline after a distinct candidate was published.

## Workaround Removal Or Quarantine Plan

The staging discriminator is a quarantine gate for serialized Pi 5 proof tasks.
If later lab-controller changes make publish/TFTP identity atomic and observable,
keep the helper as a cheap regression check or retire it only after retained
known-good and real proof bundles pass without special-case baseline checks.
