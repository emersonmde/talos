# Static Inspection

Reviewed:

- `tasks/2026-06-05-phase11-staging-capture-discriminator-closeout.md`
- `tasks/2026-06-05-phase11-known-good-capture-staging-pi5-discriminator.md`
- `tasks/evidence/2026-06-05-phase11-known-good-capture-staging-pi5-discriminator/observed-summary.json`
- `tasks/2026-06-05-phase11-rp1-register-read-pi5-proof.md`
- `tasks/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof.md`
- `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-pi5-proof/known-good-serial-observe-followup.json`
- `docs/src/project/lab-controller.md`
- `docs/src/project/phase11-rp1-pcie-map-contract.md`

Findings:

- fixed: the next known-good runtime proof now has an explicit serial
  observation rule instead of relying on an implicit `/serial/observe` window.
- fixed: accepted known-good runtime evidence and the latest fetch-without-
  readiness evidence are compared by tree hash and 104,136-byte kernel size.
- fixed: the default current-control success marker is
  `rpi5-production-timer-preemption: PASS`, paired with `TALOS: kernel_main`.
- deferred: the next serialized task must prove or reject runtime readiness;
  this task changes only the evidence contract and helper.
- not-an-issue: `talos>` is still recorded by the helper as a diagnostic
  boolean, but readiness requires the proof-recorded success marker; the
  task/evidence record pins the current restored tree to the production-timer
  PASS marker.
