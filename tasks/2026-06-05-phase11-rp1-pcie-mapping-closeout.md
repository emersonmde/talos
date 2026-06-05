# Phase 11 RP1/PCIe Mapping Closeout

Task: phase11-rp1-pcie-mapping-closeout-20260605

Status: accepted with hardware blocker; Milestone 11.1 stable register-read
mapping is not accepted.

## Goal

Reconcile the first Phase 11 RP1/PCIe mapping slice without broadening into
GPIO ownership, interrupts, DMA/cache policy, networking, SSH, storage, or a
revised diagnostic shape.

## Outcome

Milestone 11.1 has an accepted source contract and accepted local diagnostic
candidate, but the Pi 5 hardware proof blocked before the diagnostic reached
its serial classification output. The accepted closeout boundary is:

`blocked-pre-entry-or-handoff-after-candidate-fetch`

The retained hardware evidence proves candidate publication and TFTP fetch of
the selected diagnostic image, and the known-good control proves the restored
boot tree and serial capture path remained viable. It does not prove that the
RP1 UART0 flag register is mapped, unmapped, faulting, or firmware-dependent.

## Evidence Map

- source contract: `tasks/2026-06-05-phase11-rp1-pcie-map-source-contract.md`
  at commit `8de1f8c`.
- contract doc: `docs/src/project/phase11-rp1-pcie-map-contract.md`.
- diagnostic core: `tasks/2026-06-05-phase11-rp1-register-read-diagnostic-core.md`
  at commit `083ffe92616b78a2f01586a527e5bcbacb8f0bd8`.
- hardware proof/blocker:
  `tasks/2026-06-05-phase11-rp1-register-read-pi5-proof.md` at commit
  `37c9492aa9f72ab584fa5699869f78d3e15ab0ab`.
- closeout inspection:
  `tasks/evidence/2026-06-05-phase11-rp1-pcie-mapping-closeout/static-evidence-inspection.md`.

## Findings

- fixed: the mapping contract records the source-backed `pcie2` to RP1
  translation and selects RP1 UART0 PL011 `FR` at CPU physical
  `0x1f_0003_0018` as the first read-only target.
- fixed: the diagnostic core is isolated behind
  `TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_read` and performs only a single
  32-bit volatile read before serial reporting.
- fixed: the hardware proof retained candidate identity, fresh serial/TFTP
  cursors, pre-restore TFTP deltas, known-good control output, restore
  evidence, and post-hardware review.
- deferred: source-level investigation of the pre-entry/handoff failure needs
  supervisor planning before any code changes or revised diagnostic shape.
- not-an-issue: archive publication and TFTP placement worked; the Pi fetched
  the selected candidate kernel in both decisive candidate runs.
- not-an-issue: no RP1 mapped/read-value claim is made from the blocked proof.

## Validation

- static evidence inspection: passed; see
  `tasks/evidence/2026-06-05-phase11-rp1-pcie-mapping-closeout/static-evidence-inspection.md`.
- diff hygiene: `git diff --check` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed with the
  existing large search-index warning.
- staged diff hygiene: `git diff --cached --check` passed before commit.

## Next Action

Request supervisor planning for the next bounded Phase 11 slice. The worker
must not infer Milestone 11.2, networking, SSH, GPIO ownership, interrupts,
DMA/cache policy, storage work, or a revised RP1 diagnostic from this closeout.
