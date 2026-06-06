# Phase 11 RP1 UART0 FR-Read Closeout

Task id: phase11-rp1-uart0-fr-read-closeout-20260606

Status: accepted

## Goal

Reconcile the refreshed local/static RP1 UART0 flag-register read candidate and
the serialized Pi 5 proof before any later Phase 11 work.

## Scope

- Inspect the accepted refresh-core task and retained Pi 5 proof evidence.
- Record the final narrow RP1 UART0 FR-read classification.
- Update the Phase 11 contract and roadmap so the blocker boundary is explicit.

## Non-Goals

No source change, hardware run, boot publication, hardware-lock acquisition,
RP1 constant change, GPIO, pin mux, clocks/resets, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, or
phase transition.

## Findings And Disposition

- fixed: refresh-core evidence proves the selected local/static candidate
  branches from `rust_entry`, emits UART10 early-serial start/pre-MMIO markers,
  and performs exactly one 32-bit volatile load from `0x1f_0003_0018` if
  reached.
- fixed: first candidate Pi 5 proof evidence ties the published candidate tree
  `25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71` and
  45,832-byte `da591740/kernel_2712.img` to two stable pre-restore TFTP
  fetches.
- fixed: the same first candidate run retained a fresh serial cursor at
  `4194304`, but all serial observations from that cursor returned zero bytes
  and did not show `rpi5-rp1-uart0-fr-read: start`,
  `rpi5-rp1-uart0-fr-read: pre-mmio-read`, `mapped/read-value`, or `PASS`.
- fixed: mandatory inconclusive-run triage was completed. The restored
  known-good control retained two 104,136-byte kernel fetches but also returned
  zero serial bytes from cursor `4194304`; the candidate rerun also returned
  zero serial bytes from cursor `4194304` and retained stable zero-event TFTP
  evidence.
- fixed: restore hygiene is retained. The lab returned to tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, and the
  hardware lock was released/restored by the Pi 5 proof task.
- removed: no RP1 mapped/read-value, unmapped/trap, firmware-state,
  pre-MMIO-reachability, GPIO, interrupt, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.2, or phase-transition claim is
  accepted from the proof.
- deferred: the smallest next discriminator is the already queued serial
  cursor/capture completeness repair. Another same-shaped RP1 UART0 FR-read
  hardware rerun is deferred until the serial cursor saturation path is repaired
  or decisively classified.
- not-an-issue: candidate publication/fetch evidence is valid for the first
  candidate run, but TFTP fetch alone cannot classify RP1 MMIO behavior without
  fresh serial output from the diagnostic.

## Classification

`serial-capture-saturated-after-candidate-fetch`.

Accepted claims are limited to the refreshed source/static candidate, first-run
candidate publication/fetch, restore hygiene, and the serial-capture blocker
evidence. RP1 UART0 FR-read readiness remains blocked.

## Evidence

- Static evidence inspection:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-closeout/static-evidence-inspection.md`
- Evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-closeout/evidence-map.json`
- Refresh-core task:
  `tasks/2026-06-06-phase11-rp1-uart0-fr-read-refresh-core.md`
- Pi 5 proof task:
  `tasks/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof.md`
- Pi 5 proof classification:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/classification.json`

## Validation

- static evidence inspection: passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- diff checks: `git diff --check` passed before staging.
- staged diff checks: `git diff --cached --check` passed before commit.

## Result

Accepted closeout with blocker classification
`serial-capture-saturated-after-candidate-fetch`.

The next mechanically queued task is
`phase11-serial-cursor-saturation-repair-core-20260606`; it must not acquire
hardware and must repair or decisively classify serial cursor completeness
before any same-shaped RP1 FR-read hardware rerun.
