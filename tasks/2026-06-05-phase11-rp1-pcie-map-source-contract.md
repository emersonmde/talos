# Phase 11 RP1/PCIe Map Source Contract

## Task

- Title: Phase 11 RP1/PCIe map source contract
- Owner: worker
- Date: 2026-06-05
- Milestone: Phase 11 Milestone 11.1, RP1 and PCIe Mapping
- Scope: source-backed RP1/PCIe address contract and next diagnostic target only

## Goal

Give the next implementation task a concrete, source-backed RP1 register-read target so Talos can test the Pi 5 RP1/PCIe mapping before taking ownership of GPIO, interrupts, DMA, networking, SSH, or broader device drivers.

## Acceptance Criteria

- A Phase 11 RP1/PCIe mapping contract records source references, known CPU physical addresses, firmware-preserved-state assumptions, and known limitations.
- The contract identifies a stable register-read diagnostic target with address, width, and expected classification.
- The next diagnostic-core task can mechanically determine implementation scope and hardware proof criteria from the contract.
- No Phase 12 networking/SSH, RP1 interrupt, GPIO ownership, or DMA/cache-driver behavior is claimed.

## Context

Phase 10 is accepted at the local shell and local/QEMU generated-root transport frontier. Phase 11 starts with RP1/PCIe mapping because later GPIO, interrupt, DMA, Ethernet, networking, and storage work all depend on the Pi 5 I/O substrate. The current lab boot tree is restored, `hardwareTestLock` is unlocked/restored, and the lab boot config includes `enable_rp1_uart=1`.

## Work Performed

- Downloaded and retained Raspberry Pi Linux `rpi-6.12.y` device-tree source files for BCM2712, Pi 5, and RP1 under the task evidence directory.
- Reconciled `pcie2` non-prefetchable CPU mapping and the RP1 child peripheral range into a first address-translation contract.
- Selected RP1 UART0 PL011 flag-register read as the first non-destructive diagnostic target: `0xc0_4003_0018` RP1 bus, `0x1f_0003_0018` CPU physical, 32-bit volatile read, expected class `mapped/read-value`.
- Added `docs/src/project/phase11-rp1-pcie-map-contract.md` and linked it from the mdBook summary.
- Updated roadmap Milestone 11.1 notes with the accepted contract boundary.

## Evidence

- Static source/docs/reference inspection: `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/source-inspection-notes.md`.
- Retained source files: `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi`, `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts`, `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi.dtsi`, `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi`.
- Lab-controller API status read: current boot config reports `kernel=kernel_2712.img`, `enable_rp1_uart=1`, effective kernel `kernel_2712.img`, tree hash `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

## Review

Pre-hardware review findings:

- fixed: source-backed address translation and target selection are recorded;
- fixed: the first diagnostic is read-only and does not require GPIO/pinmux, interrupt, reset, clock, or DMA ownership;
- deferred: exact RP1 driver ownership, interrupts, DMA/cache policy, Ethernet, networking, SSH, and storage drivers;
- not-an-issue: a variable PL011 flag-register value is expected; the proof should classify read success rather than assert one exact value.

Hardware test evidence: not required and not run for this source-contract task.
Post-hardware review findings: not applicable.

## Result

Accepted contract candidate: `phase11-rp1-pcie-map-contract-v1`. The next queued diagnostic-core task can implement a narrow RP1 UART0 PL011 flag-register read at CPU physical `0x1f_0003_0018` and classify the result without broadening into GPIO, interrupts, DMA, networking, SSH, or generated-root work.

## Follow-Up

Promote `phase11-rp1-register-read-diagnostic-core-20260605` only after this task is accepted and committed. That task should add the smallest diagnostic code/script/docs needed to build and statically review the selected read target; it still must not run hardware.
