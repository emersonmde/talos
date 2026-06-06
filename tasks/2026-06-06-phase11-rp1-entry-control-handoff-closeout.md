# Phase 11 RP1 Entry-Control Handoff Closeout

Task id: phase11-rp1-entry-control-handoff-closeout-20260606

Status: accepted

## Goal

Close out the RP1 entry-control handoff discriminator and decide the next
mechanically safe Phase 11 step without inferring RP1 mapped or unmapped
behavior.

## Scope

- Reconciled the accepted source/static handoff-reset core evidence with the
  accepted serialized Pi 5 discriminator evidence.
- Recorded accepted, deferred, and not-an-issue findings.
- Updated the Phase 11 RP1/PCIe map contract and roadmap with the reconciled
  boundary.
- Did not run hardware, publish a boot archive, acquire the hardware lock, or
  change runtime/kernel source.

## Final Classification

Classification: pre-bootinfo-handoff-reachability-accepted.

The source/static core proves the selected 45,248-byte
target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz image routes
from _start to rust_entry and then to PSCI SYSTEM_RESET before BootInfo,
target initialization, boot reports, memory planning, allocator setup, or RP1
MMIO. The Pi 5 discriminator then published only that archive and captured four
45,248-byte da591740/kernel_2712.img fetches across two boot sequences from one
power cycle. The repeated boot/fetch behavior is accepted as the reset
side effect and therefore proves pre-BootInfo rust_entry handoff reachability.

## Findings And Disposition

- fixed: the prior candidate-fetch-without-entry-control boundary is narrowed;
  a fetched candidate can reach rust_entry when the first handoff action is a
  hardware-visible PSCI reset side effect.
- fixed: the Pi 5 proof retained stable pre-restore TFTP evidence before
  restore, including four candidate kernel fetches from fresh cursor 4101006.
- fixed: restore hygiene was retained; post-restore tree hash matched pre-run
  tree hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: candidate serial visibility remains unresolved because the fresh
  serial window did not expose TALOS: kernel_main or entry-control marker text.
- deferred: RP1 UART0 FR mapped/read-value, unmapped/trap, and firmware-state
  behavior remain unaccepted until a separately planned diagnostic reaches a
  decisive classification.
- not-an-issue: staging/capture is not the active blocker for this boundary
  because the accepted hardware evidence includes stable same-cursor candidate
  TFTP fetches.
- not-an-issue: no Milestone 11.2, GPIO, interrupt, DMA/cache, storage,
  generated-root, networking, SSH, or broader PCIe behavior is accepted here.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-closeout/evidence-map.json.
- Source/handoff core task:
  tasks/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core.md.
- Pi 5 discriminator task:
  tasks/2026-06-06-phase11-rp1-entry-control-handoff-pi5-discriminator.md.
- Core commit: d6042e06cfd9a76329c3733c2727320cf2c55e33.
- Pi 5 discriminator commit: e5f79f75048139f786a8398ed7c9a0a0eed9b396.

## Validation

- static evidence inspection: completed.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as pre-bootinfo-handoff-reachability-accepted.

This closeout accepts candidate fetch and pre-BootInfo rust_entry handoff
reachability only. It does not accept TALOS: kernel_main serial visibility,
entry-control UART marker visibility, RP1 mapped/read-value, RP1
unmapped/trap, firmware-state behavior, GPIO ownership, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, or a
phase transition.

No later task is mechanically unblocked in the existing queue. Supervisor
planning is required for a focused post-handoff observability or entry-control
repair before returning to the RP1 UART0 flag-register read.
