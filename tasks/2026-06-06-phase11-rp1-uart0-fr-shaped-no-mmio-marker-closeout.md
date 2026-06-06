# Phase 11 RP1 UART0 FR-Shaped No-MMIO Marker Closeout

Task id: phase11-rp1-uart0-fr-shaped-no-mmio-marker-closeout-20260606

Status: accepted

## Goal

Close out the FR-shaped no-MMIO marker discriminator without accepting RP1
UART0 flag-register mapped/read-value behavior or broadening Phase 11.

## Scope

- Reconciled the accepted no-MMIO marker source/static evidence and Pi 5
  discriminator evidence.
- Recorded the exact accepted and unaccepted claims for the selected
  rpi5_rp1_uart0_fr_shaped_no_mmio_marker candidate.
- Identified the smallest next planning need before any actual RP1 UART0
  flag-register volatile-read proof.
- Did not publish a boot archive, acquire hardwareTestLock, run hardware,
  change RP1 source, execute the RP1 UART0 FR volatile load, change RP1
  constants, or accept GPIO, interrupts, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.2, or a phase transition.

## Findings And Disposition

- fixed: source/static evidence retained the accepted no-MMIO marker archive
  SHA-256 05a6801471ffd5cb3ae61f450734728f7980d8a2c4db20b3a6280d83b470a484,
  boot-tree identity
  05f68072e4f1653c10eadfefbe099c92cefdde024b7f7d985b7c785c48011e45,
  and 45,600-byte kernel_2712.img identity.
- fixed: static disassembly proved the selected scenario branches from
  rust_entry into the FR-shaped UART10 reporting path and repeated
  TALOS: fr-no-mmio-loop without calling read_rp1_reg_u32, constructing
  0x1f_0003_0018, or executing RP1 UART0 FR MMIO before the marker loop.
- fixed: the Pi 5 discriminator published only the accepted archive, staged
  tree 2bd7db27d7bdf27a356c81408fefce059148f61e332fb3a207d280913b6ec27d,
  and retained stable pre-restore TFTP evidence from cursor 4134781 with
  13 events and two served 45,600-byte candidate kernel fetches.
- fixed: the repaired saturated-cursor direct-read serial path started at
  cursor 4194304 and retained 70,004 bytes with firmware NETWORK output and
  2,730 occurrences of TALOS: fr-no-mmio-loop.
- fixed: restore evidence returned the lab boot tree to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and
  hardwareTestLock was released/restored before completion.
- removed: the no-MMIO marker path did not execute the RP1 UART0 FR volatile
  load, so it cannot classify RP1 mapped/read-value, unmapped/trap, or
  firmware-state behavior.
- not-an-issue: no known-good control was required for the accepted clean run
  because candidate identity, stable candidate fetch evidence, repeated marker
  serial evidence, restore proof, and lock release were retained.
- deferred: another actual RP1 UART0 FR-read proof needs supervisor planning
  for a new bounded discriminator or revised proof shape; this closeout does
  not promote another same-shaped FR-read hardware rerun.

## Closeout Classification

fr-shaped-no-mmio-marker-visible

The accepted boundary is limited to the FR-read-shaped path reaching visible
UART10 pre-MMIO marker output when the volatile RP1 UART0 FR load is absent.
This is source/static plus Pi 5 serial/TFTP evidence for the no-MMIO
discriminator, not an RP1 mapping proof.

The smallest next planning need is an explicit supervisor-planned task for the
actual RP1 UART0 FR volatile-read boundary that uses the now-accepted
FR-shaped pre-MMIO marker visibility without repeating the prior
candidate-fetch-reset-loop-without-visible-fr-marker proof shape. The next
task must define whether it is testing one-shot pre-MMIO line visibility,
delayed/repeated pre-MMIO markers followed by the volatile load, trap/hang
classification, or another precise source/static discriminator before any
hardware run.

## Accepted Claims

- The no-MMIO marker candidate archive was selected, published, and fetched by
  the Pi 5 lab.
- The selected FR-shaped no-MMIO path reached repeated UART10 marker output on
  the Pi 5.
- The marker was observed from the same accepted clean run that retained
  stable candidate TFTP fetch evidence.
- The lab boot tree was restored before hardware-lock release.

## Unaccepted Claims

- RP1 UART0 FR volatile-read execution.
- RP1 UART0 FR mapped/read-value behavior.
- RP1 UART0 FR unmapped/trap or hang-at-MMIO boundary.
- Firmware-state behavior for the RP1 UART0 flag-register read.
- GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH,
  broader PCIe, Milestone 11.2, or a phase transition.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-closeout/evidence-map.json.
- Source/static task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core.md.
- Pi 5 discriminator task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator.md.

## Validation

- static inspection of no-MMIO marker core and Pi 5 discriminator records:
  passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

Accepted with classification fr-shaped-no-mmio-marker-visible.

No explicit queued follow-up task remains mechanically unblocked by this
closeout. Supervisor planning is required before any actual RP1 UART0
flag-register volatile-read proof or further Phase 11 feature expansion.
