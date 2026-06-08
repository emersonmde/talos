# Static Problem Analysis

Task id: phase11-pi5-serial-drain-freshness-repair-core-20260608

## Inspected Inputs

- tasks/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5.md
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5/evidence-map.json
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5/classification.json
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5/identity-join.json
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5/known-good-identity-join.json
- scripts/rpi5-capture-invariant-proof-bundle.sh
- scripts/rpi5-proof-identity-join-check.sh
- scripts/rpi5-capture-identity-join-retained-fixtures.sh
- tasks/2026-06-06-phase11-serial-cursor-saturation-repair-core.md
- tasks/2026-06-07-phase11-pi5-capture-identity-join-repair-core.md
- docs/src/project/lab-controller.md

## Invariant

Saturated direct-read serial can support a hardware classification only when
serial-drain-before-power.json proves the pre-power /serial/read drain reached
an empty device-buffer read.

## Contradiction

Both the observed GPIO status no-MMIO control candidate and the known-good
production-timer control fetched their expected kernels, emitted expected
serial markers, and restored to baseline, but both failed the same serial
freshness predicate. The pre-power drains each made 16 attempts, accumulated
182,528 bytes, ended at cursor 4,194,304, and still had an 11,408-byte final
read.

## Assumptions Still Unproven

- The old fixed 16-read drain can clear the available stale serial backlog.
- Visible markers after a non-empty saturated pre-power drain are necessarily
  fresh for the just-staged candidate.
- The shared candidate/known-good serial freshness failure says anything about
  RP1 behavior.

## Approaches

- Relax v2 identity join: rejected because it would accept stale saturated
  serial as feature evidence.
- Repeat the same 16-read hardware procedure: rejected because it has no new
  discriminator.
- Expose and retain bounded drain parameters, then require either an empty read
  or an explicit bounded-drain-exhausted blocker: accepted.

## Decisive Discriminator

The next retry must retain configured pre-power drain bounds and classify the
drain as empty-read-before-power or bounded-drain-exhausted-before-power. Only
empty-read-before-power can permit saturated direct-read serial to reach the
normal v2 identity join. bounded-drain-exhausted-before-power remains a
capture-staging blocker.

## Workaround Quarantine

The 96-attempt retry is a bounded procedure for the current freshness blocker.
It does not relax pi5-capture-transaction-v2 and should be removed or reduced
after a lab-controller monotonic serial cursor or equivalent freshness signal
exists.
