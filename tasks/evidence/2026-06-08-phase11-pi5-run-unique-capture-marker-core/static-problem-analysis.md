# Static Problem Analysis

Task id: phase11-pi5-run-unique-capture-marker-core-20260608

## Problem Statement

The v3 no-MMIO observed GPIO14 STATUS/CTRL control selected the expected tree,
served the expected kernel twice, preserved final selected-tree identity, and
restored the lab, but the saturated direct-read serial freshness rule rejected
the run because the constant control marker was already present in retained
pre-power serial output.

## Invariant

A saturated direct-read serial proof can support a hardware classification only
when the required post-power marker is proven fresh for that proof run. A marker
that can appear unchanged in retained pre-power output cannot establish
freshness.

## Contradicting Evidence

clean-v3-check.json rejected the control proof with
required-marker-present-before-power: the constant marker
TALOS: rp1-observed-gpio-status-control appeared 616 times before power and
36 times after power. Treating that as fresh output would accept stale serial.

## Unproven Assumptions

- A constant marker is enough if TFTP and final selected-tree identity match.
- Bounded pre-power drain exhaustion proves no stale copy of a constant marker
  remains.
- The marker-visible clean V3 control proves anything about GPIO14 STATUS/CTRL
  hardware behavior.

## Approaches Compared

- Repeat V3 with the same constant marker. Removed: it can reproduce the exact
  stale-marker failure class.
- Relax V3 when TFTP/final identity match. Removed: it accepts stale serial
  output as fresh serial evidence.
- Require a lab-controller monotonic serial cursor beyond the retained log cap.
  Deferred: it is the stronger long-term primitive, but a repository-side
  discriminator can make the queued proof mechanically testable now.
- Embed a task-owned nonce in the diagnostic marker and require that exact
  marker in the replay checker. Fixed: stale pre-power output from old runs
  cannot contain a not-yet-built nonce unless the proof itself is stale or the
  procedure reused the nonce.

## Smallest Decisive Discriminator

TALOS_CAPTURE_NONCE=<nonce> is embedded into the observed GPIO status
result/control serial marker at build time. The next control proof must pass
scripts/rpi5-proof-identity-join-run-unique-check.sh, which keeps the V3
identity/freshness checks and additionally requires the retained
required_marker to include the expected capture-nonce=<nonce> value.

The mechanical procedure is:

1. Generate one nonce for the staged archive.
2. Build and archive the observed GPIO status candidate/control with
   TALOS_CAPTURE_NONCE=<nonce>.
3. Static-review the archive with --capture-nonce <nonce>.
4. Capture with --serial-marker "TALOS: rp1-observed-gpio-status-control capture-nonce=<nonce>".
5. Replay with rpi5-proof-identity-join-run-unique-check.sh --nonce <nonce>.
6. Retain selected-tree/TFTP/final-identity evidence and restore proof before
   accepting any control or real hardware classification.

## Workaround Quarantine

The run-unique nonce is a serial freshness workaround for the saturated retained
cursor path. It should be removed or demoted when the lab controller exposes a
monotonic serial freshness source beyond the retained log cap. It does not
change the GPIO/RP1 source contract, GPIO ownership, interrupt claims, or any
phase boundary.
