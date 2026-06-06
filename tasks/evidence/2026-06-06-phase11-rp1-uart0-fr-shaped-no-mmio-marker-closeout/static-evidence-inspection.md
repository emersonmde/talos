# Static Evidence Inspection

Task id: phase11-rp1-uart0-fr-shaped-no-mmio-marker-closeout-20260606

## Inputs Inspected

- tasks/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core.md.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/evidence-map.json.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/static-inspection.md.
- tasks/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator.md.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator/evidence-map.json.
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-pi5-discriminator/classification.json.
- docs/src/project/phase11-rp1-pcie-map-contract.md.

## Findings

- The accepted source/static task produced
  target/talos-rpi5-rp1-uart0-fr-shaped-no-mmio-marker-core.tar.gz with
  archive SHA-256
  05a6801471ffd5cb3ae61f450734728f7980d8a2c4db20b3a6280d83b470a484,
  boot-tree identity
  05f68072e4f1653c10eadfefbe099c92cefdde024b7f7d985b7c785c48011e45, and a
  45,600-byte kernel_2712.img.
- Static evidence proves the selected scenario emits the FR-read-shaped
  start and pre-mmio-read lines, reports
  classification=no-mmio-marker-before-rp1-read, flushes UART10, and then
  repeatedly emits TALOS: fr-no-mmio-loop.
- Static evidence also proves the no-MMIO marker path does not call
  read_rp1_reg_u32, does not construct 0x1f_0003_0018, and does not
  execute RP1 UART0 FR MMIO before the marker loop.
- The accepted Pi 5 clean run staged tree
  2bd7db27d7bdf27a356c81408fefce059148f61e332fb3a207d280913b6ec27d, with
  effective_kernel=kernel_2712.img and expected fetch
  da591740/kernel_2712.img at 45,600 bytes.
- Stable same-cursor TFTP evidence from cursor 4134781 retained 13 events
  and two served candidate kernel fetches before restore.
- Starting from saturated serial cursor 4194304, the repaired direct-read
  path retained 70,004 bytes and 2,730 occurrences of
  TALOS: fr-no-mmio-loop from the same clean candidate run.
- The post-run boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and
  hardwareTestLock was unlocked/restored before completion.
- The evidence accepts FR-shaped no-MMIO marker visibility only. It does not
  accept RP1 UART0 FR volatile-load execution, mapped/read-value behavior,
  unmapped/trap behavior, firmware-state behavior, GPIO, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  Milestone 11.2, or a phase transition.

## Closeout Classification

fr-shaped-no-mmio-marker-visible

This closeout reconciles a non-repetitive discriminator: the selected
FR-read-shaped path reaches repeated UART10 pre-MMIO marker output when the
volatile RP1 UART0 FR load is absent. The next actual RP1 UART0 FR-read proof
must be supervisor-planned with explicit acceptance gates and must not infer
RP1 mapping behavior from this no-MMIO marker evidence.
