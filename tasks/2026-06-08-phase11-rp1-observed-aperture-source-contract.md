# Phase 11 RP1 Observed-Aperture Source Contract

Task id: phase11-rp1-observed-aperture-source-contract-20260608

Status: accepted

Classification: accepted-observed-aperture-source-contract

## Goal

Define the smallest source/evidence-backed observed-aperture discriminator
after the accepted bridge/setup-state proof showed a visible outbound-window
mismatch against the earlier source-expected PCIe 0 -> CPU
0x1f_0000_0000 RP1 aperture assumption.

## Scope

- Reviewed the accepted bridge/setup closeout and hardware classification as
  the blocker being answered.
- Reviewed retained BCM2712/RP1 device-tree source references for the RP1 UART0
  register offset and original 0x1f pcie2 translation.
- Reviewed retained first-light and decision-log evidence for the
  firmware-preserved 0x1c_0003_0000 RP1 UART0 mapping.
- Selected one read-only observed-aperture discriminator:
  rp1-uart0-fr-observed-aperture-read.
- Defined exact target, address, width, report fields, classifications,
  allowed reads, forbidden writes/paths, and paired control requirements.
- Updated roadmap and RP1/PCIe map contract docs for the accepted
  source-contract boundary.
- Recorded findings with disposition.

## Non-Goals

No runtime source changes, hardware run, boot archive publication,
hardwareTestLock acquisition, endpoint config retry, same-shaped 0x1f RP1
peripheral rerun, same-shaped bridge/setup-state rerun, BAR discovery or
programming, bridge setup writes, PERST/link-control changes, GPIO/pad/clock
or reset writes, interrupt enablement or delivery, GIC IAR/EOIR, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: selected a qualitatively different observed-aperture read instead of
  a same-shaped endpoint config, 0x1f RP1 peripheral, or bridge/setup-state
  rerun. The accepted bridge/setup-state proof observed window 0 CPU high
  fields of 0x1c while the source-expected visible setup contract required
  0x1f.
- fixed: retained source backing for the target register identity. Linux
  rp1.dtsi declares RP1 UART0 as a PL011 block at RP1 bus 0xc0_4003_0000, and
  the PL011 flag register is the existing Talos offset 0x18.
- fixed: tied the selected CPU address to retained evidence, not a new broad
  mapping claim. The selected address is 0x1c_0003_0018: the retained
  firmware-preserved RP1 UART0 base 0x1c_0003_0000 plus the source-backed
  PL011 FR offset 0x18.
- fixed: named the accepted 0x1f mismatch as context only. The real
  bridge/setup proof accepted visible link/preflight, root-complex class code
  0x060400, and outbound window 0 registers, but retained win0_lo=0x80000000,
  win0_base_limit=0x3ff00000, win0_base_hi=0x1c, and win0_limit_hi=0x1c.
- fixed: required a paired no-MMIO/no-RP1/no-PCIe/no-GIC control that
  preserves the output shape while constructing no forbidden address.
- deferred: live RP1 ownership, endpoint ownership, broad RP1 mapping, BAR
  discovery/programming, interrupt delivery, GPIO/clock ownership, DMA/cache,
  networking, SSH, Milestone 11.3, and phase transition require later
  supervisor-planned tasks.
- not-an-issue: this task does not try to reconcile why source device-tree
  window 0 describes 0x1f while accepted setup-state evidence observed 0x1c;
  the next feature step is a read-only discriminator that decides whether the
  observed aperture returns a meaningful value on hardware.

No findings were removed.

## Accepted Source Contract

Contract id:
phase11-rp1-observed-aperture-source-contract-v1

~~~text
target: rp1-uart0-fr-observed-aperture-read
operation: read-only observed RP1 aperture discriminator
source target: RP1 UART0 PL011 flag register
source RP1 bus address: 0xc0_4003_0018
observed CPU physical address: 0x1c_0003_0018
width: 32-bit volatile little-endian load
retained source-expected comparator: 0x1f_0003_0018 remains blocked for same-shaped reruns
~~~

Allowed real-candidate sequence:

1. Emit a start marker that names
   phase11-rp1-observed-aperture-source-contract-v1.
2. Emit a before-read marker.
3. Perform exactly one 32-bit volatile load from CPU physical
   0x1c_0003_0018.
4. If the load returns, emit the report fields and one terminal
   classification.

No other MMIO load is selected. No MMIO store is selected.

## Source And Evidence Reconciliation

- Retained Linux rp1.dtsi declares RP1 UART0 as compatible arm,pl011-axi with
  reg = <0xc0 0x40030000 0x0 0x100>. The selected register is the PL011 flag
  register at offset 0x18.
- Retained Linux BCM2712/RP1 device-tree source still documents the earlier
  pcie2 non-prefetchable PCIe 0 -> CPU 0x1f_0000_0000 translation that led to
  the blocked 0x1f RP1 aperture assumption.
- Retained first-light task evidence and decision-log entry 2026-05-19 record
  0x1c_0003_0000 as the firmware-preserved RP1 UART0 physical mapping used for
  first-light diagnostics, while keeping 0x1f as the pcie2
  non-prefetchable CPU window.
- The accepted bridge/setup-state Pi 5 proof retained visible link/preflight
  and root-complex class-code evidence, but the visible outbound window 0
  values did not match the source-expected 0x1f window. The retained CPU high
  fields were 0x1c.
- Therefore the next discriminator tests whether the observed/firmware-
  preserved 0x1c RP1 UART0 flag-register aperture produces a different,
  bounded hardware classification. It does not reaccept the previous 0x1f
  mapping or claim that all RP1 peripherals are reachable through 0x1c.

## Report Fields

- contract id and target name.
- source RP1 bus address, observed CPU physical address, width, and register
  offset.
- raw value if the load returns.
- raw-is-deaddead, raw-is-all-ones, raw-is-zero, and raw-is-pl011-fr-shaped
  booleans. The PL011 FR-shaped boolean may check only that bits outside the
  locally modeled PL011 FR mask 0x1ff are clear.
- retained bridge/setup mismatch context: win0_lo=0x80000000,
  win0_base_limit=0x3ff00000, win0_base_hi=0x1c, win0_limit_hi=0x1c, and
  prior outbound-window0-matches=false.
- terminal classification.

Accepted classifications:

- observed-aperture-rp1-uart0-fr-visible
- observed-aperture-rp1-uart0-fr-sentinel
- observed-aperture-rp1-uart0-fr-all-ones
- observed-aperture-rp1-uart0-fr-zero
- observed-aperture-rp1-uart0-fr-no-return-or-trap
- observed-aperture-rp1-uart0-fr-inconclusive-capture
- no-mmio-observed-aperture-control-visible
- staging/build-blocker

Classification rules:

- observed-aperture-rp1-uart0-fr-visible: the load returns, raw is not
  0xdead_dead, not 0xffff_ffff, not 0x0000_0000, and the report preserves the
  raw value for later review. If raw-is-pl011-fr-shaped is false, the result
  is still visible but cannot be used as UART ownership evidence.
- observed-aperture-rp1-uart0-fr-sentinel: the load returns 0xdead_dead.
- observed-aperture-rp1-uart0-fr-all-ones: the load returns 0xffff_ffff.
- observed-aperture-rp1-uart0-fr-zero: the load returns 0x0000_0000.
- observed-aperture-rp1-uart0-fr-no-return-or-trap: the before-read marker is
  present and the accepted capture path shows no post-read report or records a
  trap/fault boundary.
- observed-aperture-rp1-uart0-fr-inconclusive-capture: capture, TFTP, serial,
  selected-tree identity, or restore evidence cannot support one of the above
  classifications after required triage.
- no-mmio-observed-aperture-control-visible: the paired control preserves the
  same output shape and classification vocabulary without constructing any
  forbidden MMIO address.
- staging/build-blocker: the candidate or control cannot be built, archived,
  reviewed, or staged under the task-owned gates.

## Paired Control Requirements

The paired control must preserve report shape and classification vocabulary
while constructing no BCM2712 PCIe, RP1 peripheral/SYSINFO/clock/GPIO/MSI-X,
MIP, GIC, DMA, or other MMIO address. It must not construct 0x1c_0003_0018,
0x1f_0003_0018, any 0x1f_0000_0000 RP1 peripheral address, any
0x10_0012_0000 PCIe2 controller address, any MIP address, or any GIC address.

The control may use constants such as simulated raw values and strings so that
serial output shape remains comparable to the real candidate.

## Accepted Claims

- The first-principles question is whether a different observed RP1 CPU
  aperture explains the accepted 0x1f sentinel/all-ones behavior and
  outbound-window mismatch.
- The selected discriminator is different from the blocked same-shaped 0x1f
  RP1, endpoint config identity, and bridge/setup-state reruns.
- The allowed real operation is exactly one 32-bit volatile read from
  0x1c_0003_0018.
- The accepted control requirement forbids constructing BCM2712 PCIe, RP1,
  MIP, GIC, GPIO, clock/reset, DMA, or other MMIO addresses.

## Rejected Claims And Retained Risks

This contract does not accept live RP1 ownership, endpoint ownership, broad
RP1 mapping, UART ownership, interrupt delivery, GPIO/clock ownership,
DMA/cache, networking, SSH, Milestone 11.3, or phase transition.

Same-shaped endpoint config identity, same-shaped bridge/setup-state, and
same-shaped 0x1f RP1 hardware reruns remain closed unless a future supervisor
task supplies a different discriminator or new acceptance criteria.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-source-contract/evidence-map.json.
- Accepted bridge/setup closeout:
  tasks/2026-06-08-phase11-rp1-bridge-setup-closeout.md.
- Accepted bridge/setup Pi 5 classification:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-pi5/classification.json.
- Retained source inspection:
  tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/source-inspection-notes.md,
  tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi.
- Retained first-light and decision evidence:
  tasks/2026-05-18-phase-2-pi5-first-light.md,
  docs/src/decisions/README.md.

## Validation

- static source/evidence inspection: passed for retained RP1 UART0 register
  source, accepted bridge/setup mismatch evidence, and retained 0x1c
  firmware-preserved mapping evidence.
- git diff --check: passed.
- docs validation: mdbook build passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as accepted-observed-aperture-source-contract. The queued
observed-aperture core task is mechanically unblocked on a future worker wake.
