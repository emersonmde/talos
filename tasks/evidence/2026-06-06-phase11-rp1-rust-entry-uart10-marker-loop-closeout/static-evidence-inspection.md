# Phase 11 RP1 Rust-Entry UART10 Marker Loop Closeout Static Evidence Inspection

Task id: phase11-rp1-rust-entry-uart10-marker-loop-closeout-20260606

## Inputs Inspected

- tasks/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core.md
- tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-core/static-inspection.md
- tasks/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator.md
- tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/candidate-run/capture-invariant-summary.json
- tasks/evidence/2026-06-06-phase11-rp1-rust-entry-uart10-marker-loop-pi5-discriminator/validation-summary.txt
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Evidence Reconciliation

The accepted source/static core created only a reset-independent UART10 marker
loop candidate: target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz.
Static inspection records archive SHA-256
ab6de452670427cee2d411cbcd2a92602331e9d03a9d68dae20b75d649d1565b, kernel
SHA-256 6335cc2f229c38258d88000fe968248ca2e47d61e47f874bf246862e0d2b248a,
kernel size 45,328 bytes, and arm64 Image header fields text_offset=0,
header_image_size=45328, flags=12. Disassembly proves _start -> rust_entry ->
run_rust_entry_uart10_marker_loop before BootInfo parsing, target::init, boot
reports, allocator/memory planning, scheduler work, PSCI SYSTEM_RESET, or RP1
UART0 MMIO.

The selected marker loop writes TALOS: reu10-loop through the existing UART10
early-phase helper and waits on BCM2712 UART10 FR at 0x107d001018. That is not
the RP1 UART0 flag register at 0x1f00030018. String and symbol checks retained
in the core evidence show the RP1 UART0 FR-read report strings and symbols are
absent from the selected image.

The Pi 5 discriminator published only that accepted archive. Candidate
identity matched selected tree
1d7cdd3d265fb983ec77d9281098d6a920e0bc957a1f0a15f279fe35c618ee6c,
effective kernel kernel_2712.img, and a 45,328-byte
da591740/kernel_2712.img. Stable same-cursor TFTP evidence retained 13 events,
including two served 45,328-byte da591740/kernel_2712.img fetches before
restore.

Fresh serial evidence started from cursor 4133556 and retained 60,748 bytes
through cursor 4194304 over 32 seconds. The deadline-looped observation found
TALOS: reu10-loop 2,961 times. Because the marker appears only on the selected
rust_entry marker-loop path, this accepts visible post-handoff Rust-entry
UART10 marker observability for that selected candidate.

The task restored snapshot phase11-rust-entry-uart10-marker-loop-pre-20260606T1032Z.
Post-restore status returned tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and durable
state records the hardwareTestLock unlocked/restored.

## Final Classification

Classification: post-handoff-rust-entry-uart10-marker-visible.

Accepted:

- source/static no-RP1-MMIO Rust-entry UART10 marker-loop candidate;
- selected archive and kernel identity;
- stable pre-restore candidate fetch evidence;
- visible post-handoff Rust-entry UART10 marker output from the same candidate
  hardware run;
- restore to the pre-run boot tree.

Not accepted:

- RP1 UART0 FR-read result or readiness beyond unblocking the existing refresh
  core;
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

- fixed: the core and hardware proof now establish visible Rust-side UART10
  marker output after rust_entry without relying on PSCI reset side effects.
- fixed: candidate identity, stable TFTP fetches, fresh serial cursor, marker
  occurrences, restore state, and lock release are all retained.
- fixed: the accepted boundary is narrow enough to unblock the existing RP1
  UART0 FR-read refresh core while keeping RP1 mapped/unmapped behavior out of
  scope.
- deferred: the actual RP1 UART0 FR read must be refreshed and then proven by
  the separate queued core/proof tasks.
- not-an-issue: known-good control and candidate rerun were unnecessary
  because the first candidate run was not inconclusive for capture or staging.
- not-an-issue: no hardware run, boot publication, kernel/RP1 source change,
  RP1 MMIO read, GPIO, interrupts, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.2, or phase transition belongs
  to this closeout.

## Next-Step Decision

phase11-rp1-uart0-fr-read-refresh-core-20260606 is mechanically unblocked for
the next worker wake, provided hardwareTestLock remains unlocked/restored. The
promotion is limited to that already queued local refresh core. The dependent
Pi 5 proof and final closeout remain queued behind the refresh core and must
not be promoted until their explicit dependencies are satisfied.
