# Run-Unique Serial Visibility Discriminator Blocker Analysis

Task id: phase11-pi5-run-unique-serial-visibility-discriminator-core-20260609

## Source Evidence

- Prior blocker task:
  tasks/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5.md.
- Prior retained capture bundle:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/control-run/.
- Prior run-unique checker output:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/run-unique-check.json.
- Prior boot-staging identity checker output:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5/boot-staging-identity-check.json.

## Expected Invariant

A decisive Pi 5 proof needs all of these:

- selected candidate identity before power;
- expected TFTP candidate kernel fetches with the expected byte count;
- final pre-restore selected-tree identity;
- restore proof;
- current-run serial visibility.

For saturated serial windows, current-run serial visibility must not be inferred
from retained old output. The decisive discriminator is a task-owned
run-unique nonce token that is absent from all retained pre-power serial drain
responses and present in the post-power serial window.

## Contradicting Evidence

The prior control blocker had valid staging/TFTP/final/restore evidence:

- selected tree:
  56d2c8171b5424a77358c4732238161bcd12f68739a54993e9af9d00cc1996fb;
- expected fetch: da591740/kernel_2712.img, 48,744 bytes;
- TFTP delta: two matching expected candidate fetches;
- final pre-restore tree: same selected tree;
- restore tree:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The serial path was saturated before power:

- pre-power drain: 16 attempts, 182,528 bytes, final cursor 4,194,304,
  not empty;
- post-power window: saturated direct read from cursor 4,194,304 with
  617,634 bytes;
- stale GPIO14 ownership-route output was visible in the window;
- kernel_main was not visible;
- the exact prior required marker string was counted zero times.

However, static inspection of the retained serial window found the actual
GPIO16 control output line did contain the task nonce token
capture-nonce=gpio16ctl20260609T030329Z-c2a7c390 after power, while that token
was absent before power. The old required marker was brittle because it expected
capture-nonce to appear immediately after the marker name, but the actual Talos
line prints contract fields before capture-nonce.

## Unproven Assumptions

- The prior GPIO16 no-MMIO control hardware behavior is not accepted by this
  task. The committed blocker remains the source record for that run.
- A future GPIO16 control retry must still retain candidate identity, TFTP,
  final identity, restore proof, and current-run serial visibility.
- Stale serial output remains rejected when the nonce token is present before
  power or absent after power.

## Selected Discriminator

The checker now keeps the v3 identity/TFTP/final/restore gates and repairs only
the serial visibility discriminator:

- exact v3 marker matching still passes when available;
- when exact marker matching fails solely because the field order differs, the
  checker accepts a nonce-token differential only if all non-serial v3 gates
  pass;
- nonce token present before power is rejected;
- nonce token absent after power is rejected;
- staging/TFTP/final/restore mismatches remain rejected.

## Findings And Disposition

- fixed: repaired the run-unique checker to use the nonce token as the
  current-run serial visibility discriminator while preserving non-serial
  identity gates.
- fixed: retained replay evidence for nonce-token-present, marker-absent,
  stale-before-power, and staging-mismatch cases.
- deferred: no Pi 5 hardware retry is performed by this local/static task.
- not-an-issue: no capture-helper API change is required; future runs may pass
  either the exact marker or any marker string containing the run-unique nonce,
  because the checker verifies the nonce token in retained pre/post serial.

No findings were removed.
