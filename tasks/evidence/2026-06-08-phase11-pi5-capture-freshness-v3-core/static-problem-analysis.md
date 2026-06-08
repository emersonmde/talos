# Static Problem Analysis

Task id: phase11-pi5-capture-freshness-v3-core-20260608

## Invariant

A Pi 5 hardware classification needs one proof run joining selected boot tree,
expected fetch, fresh serial evidence, stable TFTP delta, final pre-restore
identity, and restore identity. When the serial cursor is saturated, direct-read
serial can be decisive only if the required post-power marker is proven not to
be stale retained output.

## Contradiction

The observed GPIO14 STATUS/CTRL real candidate and the known-good
production-timer control both selected their expected trees, retained matching
TFTP/final-identity evidence, and restored to the baseline tree. Both failed v2
because 96 pre-power drain attempts read 1,095,168 bytes and did not reach
empty-read-before-power. Treating that as GPIO behavior would be wrong; it was a
capture freshness blocker.

## Unproven Assumptions

- A bounded drain that does not end empty proves no stale marker remains.
- Marker-visible saturated direct-read serial is fresh without comparing it to
  pre-power drained output.
- The identical candidate and known-good v2 failure says anything about RP1 GPIO
  behavior.

## Approaches Compared

- Relax v2 to accept saturated direct-read markers plus matching TFTP. Rejected:
  this would accept the stale-marker replay class.
- Require a new lab-controller monotonic serial cursor endpoint before any more
  hardware. Deferred: it is a stronger long-term fix but blocks a bounded
  repository-side discriminator.
- Add a v3 replay contract that keeps v2 identity/TFTP/final-restore checks and
  accepts non-empty saturated serial only when the required marker is absent in
  all pre-power drain responses and present after power. Accepted: it is the
  smallest discriminator that rejects stale marker replay while allowing a
  decisive known-good proof.

## Smallest Decisive Discriminator

`scripts/rpi5-proof-identity-join-v3-check.sh` replays the retained proof
bundle. It accepts either an empty pre-power drain or, only for saturated
direct-read mode, a marker differential:

- required marker count before power is zero;
- required marker count after power is non-zero;
- TFTP and final selected-tree identity still match.

The stale-marker fixture injects the required marker into pre-power drain
evidence and is rejected for `required-marker-present-before-power`.

## Workaround Quarantine

V3 is limited to queued capture-freshness repair/proof tasks. It should be
removed or demoted when the lab controller exposes a monotonic serial freshness
source that works beyond the retained cursor cap. It does not change the source
contract, runtime RP1 diagnostics, or any GPIO/interrupt acceptance boundary.
