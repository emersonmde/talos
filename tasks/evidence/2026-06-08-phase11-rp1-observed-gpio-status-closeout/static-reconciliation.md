# Phase 11 RP1 Observed GPIO Status Closeout Static Reconciliation

Task id: phase11-rp1-observed-gpio-status-closeout-20260608

## Inputs Inspected

- tasks/2026-06-08-phase11-rp1-observed-gpio-status-source-contract.md
- tasks/2026-06-08-phase11-rp1-observed-gpio-status-core.md
- tasks/2026-06-08-phase11-pi5-serial-drain-freshness-repair-core.md
- tasks/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry.md
- tasks/2026-06-08-phase11-rp1-observed-gpio-status-pi5.md
- tasks/evidence/2026-06-08-phase11-pi5-serial-drain-freshness-repair-core/classification.json
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry/classification.json
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-pi5/classification.json
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5-retry/evidence-map.json
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-pi5/evidence-map.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Reconciliation

The accepted source contract selected exactly two read-only 32-bit volatile
loads from observed CPU physical addresses 0x1c000d0070 and 0x1c000d0074.
Retained Linux source identifies those offsets as RP1 IO_BANK0 GPIO14
STATUS/CTRL. The contract explicitly excluded GPIO ownership, event
generation, interrupt pending/delivery, broad RP1 mapping, endpoint ownership,
DMA/cache, networking, SSH, Milestone 11.3, and phase transition.

The local/static core implemented the selected real candidate and paired
control. The real archive size was 49,656 bytes and the paired no-MMIO control
archive size was 48,952 bytes. Static archive review accepted the report
shape, classification vocabulary, selected observed 0x1c addresses, and
absence of forbidden same-shaped 0x1f GPIO, IO_BANK0, PCIe, GIC, and control
strings.

The serial-drain freshness repair accepted a procedure change, not a hardware
claim. It made the pre-power serial-drain bounds explicit and preserved the
v2 rule that saturated direct-read serial remains non-decisive unless the
pre-power /serial/read drain reaches empty-read-before-power.

The paired control proof is accepted as
no-mmio-observed-gpio-status-control-visible. The accepted unchanged control
rerun selected tree
133f2a9b4a4c5c21b206d8f9eb8eba4ffe41f787a69b33bced0ea164a8cf83ab, retained an
empty pre-power drain, observed two 48,952-byte
da591740/kernel_2712.img TFTP fetches, retained 41 task-owned control markers,
kept final pre-restore identity on the selected tree, and restored to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The real candidate completed as capture-staging-blocked. It selected tree
52b5f11000b24f6f6d00ab1b9aaa4d62a4d4114486a0302ad593b713a08c2559, observed
two 49,656-byte da591740/kernel_2712.img TFTP fetches, retained final
selected-tree identity, restored to the baseline tree, and emitted 42
task-owned result markers with marker-visible values
gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, ctrl-funcsel=4, and
classification=observed-aperture-gpio14-status-ctrl-visible. Those values are
retained but not accepted because the repaired pre-power serial drain
exhausted 96 attempts, read 1,095,168 bytes, and did not reach
empty-read-before-power.

The required known-good production-timer control after the inconclusive real
candidate selected tree
407d10f6ed4457e89f9023f769c00920a4ebbe0f42ca65b0165b8db014140697, retained
matching TFTP/final identity evidence and restore proof, but failed the same
repaired freshness discriminator. The unchanged real candidate was therefore
not rerun.

## Accepted Claims

- The source contract for GPIO14 STATUS/CTRL observed-aperture reads is
  accepted.
- The local/static real and control candidates match that contract.
- The serial-drain freshness repair procedure is accepted as the current v2
  capture procedure.
- The paired no-MMIO/no-RP1/no-GIC control proof is accepted.
- The real Pi 5 run is accepted only as a committed capture-staging blocker.
- The marker-visible real serial values are retained evidence but are not
  decisive hardware visibility evidence.

## Rejected Claims

- Observed 0x1c GPIO14 STATUS/CTRL visibility is not accepted.
- GPIO ownership is not accepted.
- Event generation and interrupt pending generation are not accepted.
- Interrupt delivery and GIC acknowledgement are not accepted.
- Endpoint ownership and broad RP1 mapping are not accepted.
- Pad/RIO/clock/reset ownership is not accepted.
- DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, and
  phase transition are not accepted.

## Same-Shaped Rerun Policy

Same-shaped endpoint config identity, bridge/setup-state, 0x1f RP1 peripheral,
0x1c UART0 FR, and real 0x1c GPIO14 STATUS/CTRL hardware reruns remain closed
unless a future supervisor task supplies a different discriminator or new
acceptance criteria. This closeout creates no worker-owned follow-up task.
