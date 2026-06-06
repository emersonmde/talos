# Phase 11 RP1 Post-Handoff Marker Reset Capture Recheck Closeout Static Evidence Inspection

Task id: phase11-rp1-post-handoff-marker-reset-capture-recheck-closeout-20260606

## Inputs Inspected

- tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-core.md
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/evidence-map.json
- tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-closeout.md
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-closeout/evidence-map.json
- tasks/2026-06-06-phase11-pi5-capture-invariant-harness-core.md
- tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5.md
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5/candidate-run/capture-invariant-summary.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Evidence Reconciliation

The accepted source/static core created only a no-RP1-MMIO marker/reset
candidate: target/talos-rpi5-post-handoff-marker-reset-core.tar.gz. Static
inspection records archive SHA-256
73a74db1d08d89a3aa371d5329bc6158553bef172a82f0b479598bc29f15acaa, kernel size
51,736 bytes, arm64 Image header fields text_offset=0,
header_image_size=51736, flags=12, ARMd, and _start -> rust_entry -> marker
writes -> smc #0 provenance. The path does not parse BootInfo, enter
target::init, run scheduler work, or perform the RP1 UART0 flag-register read
before the marker/reset discriminator.

The earlier Pi 5 marker/reset discriminator retained publication and restore
evidence, but classified the boundary as staging-capture-blocked because the
stable same-cursor TFTP windows did not prove candidate-tied fetches before
restore. That closeout explicitly left reset side-effect evidence, visible
post-handoff marker observability, marker-path hang/fault evidence, and RP1
UART0 FR-read readiness unaccepted.

The repaired capture-invariant recheck reran the same accepted candidate under
the stable pre-restore TFTP rule. Preflight identity matched tree
37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2, effective
kernel kernel_2712.img, and 51,736-byte da591740/kernel_2712.img. Final
pre-restore identity still matched the selected tree and fetch bytes.

Stable same-cursor TFTP evidence from fresh cursor 4111814 reached cursor_end
4118569, stabilized for three samples, retained 65 events, and included 10
served da591740/kernel_2712.img fetches at 51,736 bytes. Fresh serial from
cursor 4113931 retained 19,625 bytes over 90 seconds and contained 10
firmware NETWORK occurrences, but no TALOS: kernel_main and no
rpi5-rp1-post-handoff-marker-reset marker text.

The lab restored snapshot
phase11-post-handoff-marker-reset-capture-recheck-pre-20260606T0852Z after the
candidate run, and post-restore status returned tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
hardware lock is released/restored in durable supervisor state.

## Final Classification

Classification: reset-side-effect-accepted-marker-visibility-blocked.

Accepted:

- candidate archive publication and selected boot-tree identity;
- stable pre-restore candidate kernel fetch evidence;
- PSCI reset-loop side-effect evidence for the selected marker/reset
  candidate;
- restore to the pre-run boot tree.

Not accepted:

- visible post-handoff serial observability;
- TALOS: rust_entry or marker text visibility;
- marker-path serial output;
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

- fixed: the capture-invariant recheck replaces the earlier
  staging-capture-blocked classification for this candidate by proving
  candidate-tied fetches before restore.
- fixed: repeated candidate fetches across the 90-second run accept the
  reset-loop side effect for the selected no-RP1-MMIO marker/reset candidate.
- fixed: restored-tree proof remains intact after the recheck.
- deferred: visible marker output remains unaccepted because the fresh serial
  window omitted TALOS: kernel_main, TALOS: rust_entry, and the unique
  marker/reset text.
- deferred: the next bounded discriminator should be supervisor-planned around
  post-handoff marker visibility or UART10 marker-path observability before
  RP1 MMIO is retried.
- deferred: the queued RP1 UART0 FR-read refresh remains blocked because the
  dependency requires visible post-handoff serial observability.
- not-an-issue: no additional hardware run is required for this closeout; its
  scope is reconciliation of already committed evidence.
- not-an-issue: no RP1 mapped/unmapped, GPIO, interrupts, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
  transition behavior is accepted.

## Next-Step Decision

No task currently queued after this closeout is mechanically unblocked. The
queued RP1 UART0 FR-read refresh depends on accepted visible post-handoff
serial observability. Reset-side-effect-only evidence is insufficient for that
promotion. Supervisor planning is required for the smallest post-handoff marker
visibility discriminator before returning to the serial-reported RP1 UART0
flag-register read.
