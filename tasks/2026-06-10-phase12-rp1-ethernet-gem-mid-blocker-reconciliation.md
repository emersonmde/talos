# Phase 12 RP1 Ethernet GEM MID Blocker Reconciliation

Task: phase12-rp1-ethernet-gem-mid-blocker-reconciliation-20260610

Status: accepted

Classification: rp1-ethernet-gem-mid-retained-0x1f-window-sentinel

## Scope

Reconcile the accepted GEM MID Pi 5 result, where a read-only load from
`MACB_MID` at CPU physical `0x1f001000fc` returned `raw=0xdeaddead`,
against accepted Phase 11 RP1 read-only boundaries. This task is source/local
analysis only. It does not run hardware, publish boot archives, acquire
hardwareTestLock, implement Ethernet behavior, program RP1/DMA/interrupts, or
start Phase 12.2 work.

## Retained Inputs

- `phase12-rp1-ethernet-gem-mid-pi5-proof-20260609` accepted the serialized
  candidate/control proof at commit
  `41116dc167166679c8f0ec89fac3ae39d49f6cd7`. The no-Ethernet/no-MMIO
  control proved the reporting path. The candidate reached the bounded
  `MACB_MID` read and returned `raw=0xdeaddead`.
- The Phase 12 source contract cites Linux RP1 device-tree and MACB evidence:
  `rp1_eth` is `raspberrypi,rp1-gem` / `cdns,macb` at RP1 bus
  `0xc0_40100000`; `MACB_MID` is offset `0x00fc`; the Pi 5 RP1 ranges map
  RP1 bus `0xc0_40000000..0xc0_4040ffff` to CPU physical
  `0x1f_0000_0000..`. The resulting source translation for `MACB_MID` is
  `0x1f001000fc`.
- Phase 11 retained a translated-RP1-aperture sentinel boundary: the accepted
  PCIe endpoint/config discriminator source contract records prior
  `SYSINFO_CHIP_ID`, `SYSINFO_PLATFORM`, and `CLK_ADC_CTRL` reads returning
  `0xdeaddead` through the `0x1f_0000_0000` translated RP1 aperture.
- Phase 11 also accepted observed-aperture visibility boundaries: the
  clock/reset dependency Pi 5 proof read `0x1c00000000`
  `SYSINFO_CHIP_ID=0x20001927` and selected clock-manager registers without
  `0xdeaddead`; the GPIO16 observed-aperture closeout retained visible
  GPIO/RIO/pad/source fields under `0x1c...`, classified as a non-GPIO-function
  blocker rather than an aperture sentinel.
- Talos code keeps both address families explicit in `src/target/rpi5.rs`:
  `RP1_SYSINFO_CHIP_ID=0x1f_0000_0000`,
  `RP1_SYSINFO_OBSERVED_APERTURE_CHIP_ID=0x1c_0000_0000`, and the
  `PCIE_MISC_PCIE_STATUS` host-controller discriminator at `0x10_0012_4068`.

## Explanation Evaluation

- Not-an-issue: source-translation typo. The retained Linux sources and Talos
  constants agree on `rp1_eth` at RP1 bus `0xc0_40100000`, `MACB_MID` at
  `0x00fc`, and CPU physical `0x1f001000fc`. No contradictory retained
  source was found.
- Fixed: retained address-decode / bridge-enable blocker, refined as an
  `0x1f` translated-RP1-window sentinel. The GEM MID `0xdeaddead` result has
  the same qualitative shape as the accepted Phase 11 `0x1f`
  SYSINFO/clock-window sentinel. It is not a trap and not a successful live GEM
  identity read.
- Deferred: Ethernet-local clock/reset dependency. Linux ties `rp1_eth` to
  `RP1_CLK_SYS`, `RP1_CLK_ETH_TSU`, and `RP1_CLK_ETH`, and the accepted
  clock/reset proof shows `clk-sys-enabled=false` in the observed aperture.
  That could matter before live Ethernet behavior, but it does not explain away
  the broader `0x1f` sentinel pattern by itself because observed RP1
  SYSINFO/clock reads are visible while translated `0x1f` RP1 reads have
  produced `0xdeaddead`.
- Deferred: bridge/window programming. The PCIe2 host status/source contract is
  the correct class of evidence for this, but this reconciliation does not run
  or add hardware behavior. It selects a same-run discriminator instead of
  accepting a bridge fix or endpoint/config claim.
- Not-an-issue: same-shaped GEM MID-only rerun. The next proof must include a
  changed discriminator; a repeat of the accepted GEM MID-only candidate/control
  shape would only repro the current blocker.

## Classification

The accepted GEM MID blocker is refined to
`rp1-ethernet-gem-mid-retained-0x1f-window-sentinel`.

This keeps the address-decode/bridge-enable blocker, but narrows the immediate
unknown: distinguish a translated `0x1f` RP1-window sentinel from live
observed-aperture RP1 visibility in the same run before any Ethernet driver,
packet, DMA, interrupt, networking, sockets, SSH, Phase 12.2, or phase
transition work.

## Selected Next Discriminator

Select exactly one follow-up:
`phase12-rp1-ethernet-gem-mid-decode-discriminator-core-20260610`.

Required discriminator shape for that local/static core:

- Candidate report: same-run read-only positive-control load of observed RP1
  `SYSINFO_CHIP_ID` at `0x1c00000000` plus read-only load of `MACB_MID` at
  `0x1f001000fc`.
- Expected candidate classifications include:
  `observed-rp1-positive-control-gem-mid-0x1f-window-sentinel`,
  `observed-rp1-positive-control-and-gem-mid-visible`,
  `observed-rp1-positive-control-sentinel`, and
  `staging/build-blocker`.
- Paired control: same reporting path, no observed RP1 MMIO target and no
  Ethernet MMIO target, with explicit no-MMIO/control classification.
- The discriminator must reject Ethernet driver readiness, broad Ethernet MMIO
  readiness, RP1 MMIO/DMA programming, descriptor rings, interrupt completion,
  clock/reset ownership, PHY reset ownership, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition claims.

## Evidence

- Classification:
  `tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-blocker-reconciliation/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-blocker-reconciliation/evidence-map.json`.
- Accepted GEM MID proof:
  `tasks/2026-06-09-phase12-rp1-ethernet-gem-mid-pi5-proof.md` and
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-pi5-proof/classification.json`.
- Accepted Phase 11 boundaries:
  `tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract.md`,
  `tasks/2026-06-09-phase11-rp1-clock-reset-dependency-pi5.md`,
  `tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-pi5/classification.json`,
  and `tasks/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-closeout.md`.
- Source/code inspection:
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi`,
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts`,
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-source-contract/source/linux-rpi-6.12-macb.h`,
  `src/rp1_ethernet.rs`, and `src/target/rpi5.rs`.

## Validation

- Static inspection of accepted task records, classification JSON, retained
  source excerpts, and Talos RP1 translation/reporting code: passed.
- `jq empty` on task-owned evidence-map/classification JSON: passed.
- `git diff --check`: passed.
- `/home/node/.cargo/bin/mdbook build`: passed because docs/src files were
  touched.
- `git diff --cached --check`: passed before commit.

## Next Action

Mechanically promote
`phase12-rp1-ethernet-gem-mid-blocker-reconciliation-closeout-20260610` on
the next worker wake. The closeout should explicitly select
`phase12-rp1-ethernet-gem-mid-decode-discriminator-core-20260610` with the
same-run observed `SYSINFO_CHIP_ID` positive-control plus `MACB_MID`
discriminator shape above, unless it finds a concrete contradiction in this
accepted reconciliation.
