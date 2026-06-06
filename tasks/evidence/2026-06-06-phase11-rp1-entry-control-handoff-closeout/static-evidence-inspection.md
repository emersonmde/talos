# Phase 11 RP1 Entry-Control Handoff Closeout Static Evidence Inspection

Task id: phase11-rp1-entry-control-handoff-closeout-20260606

## Inputs Inspected

- tasks/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core.md
- tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/evidence-map.json
- tasks/2026-06-06-phase11-rp1-entry-control-handoff-pi5-discriminator.md
- tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-pi5-discriminator/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-pi5-discriminator/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-pi5-discriminator/tftp-delta-stable-followup-pre-restore.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Evidence Reconciliation

The accepted core task created only a no-RP1-MMIO handoff-reset candidate:
target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz. Static image
inspection records archive SHA-256
ee251a145b88df55fd162b0150a82d62a9671906f401948524d27d45929516c6, kernel
SHA-256 38170a7fe229b37bfb358479f09d45a14a342af86b16c51d36b3c33023255594,
kernel size 45,248 bytes, arm64 Image header fields text_offset=0,
header_image_size=45248, flags=12, ARMd, and _start -> rust_entry -> smc #0
side-effect provenance.

The accepted Pi 5 discriminator published only that archive, staged tree
760e7e3c59c3d6d6da4f465c9f67fc53a445bfa18850c6a76f2a3972af680d2d, and
captured fresh serial cursor 4107969 and fresh TFTP cursor 4101006. Stable
same-cursor pre-restore TFTP follow-up retained 26 events and four served
45,248-byte da591740/kernel_2712.img fetches across two boot sequences at
05:51:46/05:51:47 and 05:52:04/05:52:05 UTC after one power cycle. The lab was
restored to pre-run tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Those repeated candidate boot/fetch sequences match the core task's PSCI
SYSTEM_RESET side effect, which is routed immediately from rust_entry before
BootInfo parsing, target initialization, boot reports, memory planning,
allocator setup, and the RP1 UART0 flag-register read path.

## Final Classification

Classification: pre-bootinfo-handoff-reachability-accepted.

Accepted:

- source/static no-RP1-MMIO handoff-reset candidate identity;
- candidate archive publication;
- candidate TFTP fetch;
- pre-BootInfo rust_entry handoff reachability by repeated PSCI reset
  side effect;
- post-run restore to the pre-run boot tree.

Not accepted:

- TALOS: kernel_main serial visibility for the candidate;
- entry-control UART marker visibility;
- RP1 UART0 flag-register mapped/read-value behavior;
- RP1 unmapped/trap behavior;
- firmware-state behavior;
- GPIO ownership;
- interrupts;
- DMA/cache behavior;
- storage or generated-root progress;
- networking or SSH;
- broader PCIe;
- Milestone 11.2;
- phase transition.

## Findings And Disposition

- fixed: candidate fetch and pre-BootInfo handoff reachability are no longer
  blocked by the prior fetch-without-entry-control evidence; the reset
  side-effect candidate reached rust_entry far enough to reboot through PSCI.
- fixed: stale TFTP capture risk is mitigated for this proof by stable
  same-cursor pre-restore replay before restore and a follow-up sample with
  four candidate kernel fetches.
- fixed: restore hygiene is retained and matches the pre-run tree hash after
  the hardware run.
- deferred: candidate serial output still does not expose TALOS: kernel_main or
  entry-control marker text from the fresh serial window, so any next RP1
  diagnostic that relies on serial-only classification needs a supervisor-
  planned observability/entry-control repair first.
- deferred: RP1 register-read hardware classification remains unaccepted until
  a later explicitly planned diagnostic reaches a decisive mapped/read-value,
  trap, firmware-state, or staging/build classification.
- not-an-issue: the task does not need a new hardware run; its scope is static
  reconciliation of already committed core and Pi 5 discriminator evidence.
- not-an-issue: the conditional known-good serial-completeness queued tasks are
  not mechanically unblocked because the marker-boundary closeout accepted
  valid known-good Talos readiness instead of classifying ready-for-serial-
  completeness-pi5-discriminator.

## Next-Step Decision

The mechanically safe next direction is supervisor planning for a focused
post-handoff observability or entry-control repair before returning to the
RP1 UART0 flag-register read. Staging/capture is not the active blocker for
this boundary, and the pre-BootInfo source/handoff blocker is accepted as
resolved by the reset side effect. A direct return to the serial-reported RP1
register-read diagnostic would overstate what this closeout proves because
candidate serial visibility remains unaccepted.
