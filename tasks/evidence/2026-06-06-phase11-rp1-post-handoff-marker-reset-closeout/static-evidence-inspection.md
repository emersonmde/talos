# Phase 11 RP1 Post-Handoff Marker Reset Closeout Static Evidence Inspection

Task id: phase11-rp1-post-handoff-marker-reset-closeout-20260606

## Inputs Inspected

- tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-core.md
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/evidence-map.json
- tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator.md
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/tftp-delta-stable-pre-restore.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/tftp-delta-late-before-restore.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/rerun-tftp-delta-stable-pre-restore.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/control-tftp-delta-stable-pre-restore.json
- docs/src/project/phase11-rp1-pcie-map-contract.md

## Evidence Reconciliation

The accepted core task created only a no-RP1-MMIO marker/reset candidate:
target/talos-rpi5-post-handoff-marker-reset-core.tar.gz. Static image
inspection records archive SHA-256
73a74db1d08d89a3aa371d5329bc6158553bef172a82f0b479598bc29f15acaa, kernel
SHA-256 42367beda5de1d0564417e6267a59bd5ae5b770798fa4a3cbb3c0ce101554350,
kernel size 51,736 bytes, arm64 Image header fields text_offset=0,
header_image_size=51736, flags=12, ARMd, and _start -> rust_entry -> marker
writes -> smc #0 provenance. The selected path does not parse BootInfo, enter
target::init, or perform the RP1 UART0 flag-register read before the
marker/reset discriminator.

The completed Pi 5 discriminator published only that archive. Lab status after
publication showed tree hash
37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2,
effective kernel kernel_2712.img, and 51,736-byte root and
da591740/kernel_2712.img files. The first candidate run captured fresh serial
cursor 4110717 and fresh TFTP cursor 4106410. Serial reached Raspberry Pi
firmware/RP1 NETWORK output, but no TALOS: rust_entry text or
rpi5-rp1-post-handoff-marker-reset marker appeared.

The first stable same-cursor TFTP sample before restore reported zero events.
A late first-run query from the same cursor later reported 26 events and four
da591740/kernel_2712.img lines, but by then status had already returned to the
restored tree; the endpoint's current-file byte annotation therefore could not
serve as candidate identity proof. The candidate rerun and the restored
known-good control also retained fresh firmware serial and stable zero-event
TFTP samples in their bounded windows.

The lab was restored after each run to pre-run tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and the
hardware lock was released/restored.

## Final Classification

Classification: staging-capture-blocked.

Accepted:

- source/static no-RP1-MMIO marker/reset candidate identity;
- candidate archive publication;
- restore to the pre-run boot tree;
- retained blocker evidence showing capture/staging uncertainty.

Not accepted:

- visible post-handoff serial observability;
- reset side-effect evidence;
- marker-path hang/fault before reset;
- RP1 UART0 FR-read readiness;
- RP1 mapped/read-value behavior;
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

- fixed: the core source evidence isolates the intended marker/reset hardware
  question before BootInfo parsing, target initialization, boot reports, memory
  planning, allocator setup, or RP1 MMIO.
- fixed: publication and restore identity were retained and tied to exact tree
  hashes and archive/kernel sizes.
- deferred: stable candidate-tied TFTP fetch evidence is missing for the
  marker/reset run, rerun, and restored control, so the next bounded direction
  must resolve staging/capture or post-handoff observability before RP1 MMIO.
- deferred: the late first-run TFTP replay is useful capture-timing evidence,
  but not candidate identity proof because it was observed after the restored
  tree had become current again.
- deferred: visible post-handoff serial marker output and reset-side-effect
  proof remain blocked by the staging/capture classification.
- not-an-issue: no additional hardware run is required for this closeout; its
  scope is reconciliation of already committed evidence.
- not-an-issue: the queued RP1 UART0 FR-read refresh is not mechanically
  unblocked because the closeout does not classify
  post-handoff-serial-observability-accepted.

## Next-Step Decision

No task currently queued after this closeout is mechanically unblocked. The
queued RP1 UART0 FR-read refresh depends on
post-handoff-serial-observability-accepted, which this evidence explicitly does
not accept. Supervisor planning is required for the next bounded
staging/capture or post-handoff observability discriminator before returning to
the serial-reported RP1 UART0 flag-register read.
