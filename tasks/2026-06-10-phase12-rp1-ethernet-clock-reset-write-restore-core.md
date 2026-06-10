# Phase 12 RP1 Ethernet Clock/Reset Write-Restore Core

Task id: phase12-rp1-ethernet-clock-reset-write-restore-core-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clock-reset-write-restore-core-accepted
Evidence level: static inspection, local/static Rust report-model tests, and
Raspberry Pi 5 boot-scenario compile checks only. No Pi 5 hardware run, boot
archive publication, hardwareTestLock acquisition, or runtime hardware action
was performed.

## Goal

Implement the bounded candidate/control write-restore proof surface for the
accepted Ethernet-private CLK_ETH_TSU_CTRL target without running Pi 5
hardware.

## Scope

- Consumed the accepted write-target source contract commit
  c16b209a4fd75199306ec0cf1655c0e5e2e9fbf2.
- Added a local/static Rust contract and report surface for the selected
  CLK_ETH_TSU_CTRL idempotent write/restore target at Talos address
  0x1c00018134.
- Candidate evidence records target identity, selected register address,
  target fields, preserved fields, allowed pre-read/raw write rule,
  pre-read/write/post-read/restore-write/restore-read plan, safety invariants,
  retained risks, future proof classifications, and rejected claims.
- Paired control evidence uses the same report path while withholding writable
  target construction and candidate-only clock/reset facts.
- Added boot-scenario entries and local image helper scripts for a future
  serialized candidate/control proof; these were compile-checked only.
- Added focused tests for accepted candidate, accepted control, shape bypasses,
  and forbidden shared-clock/runtime/downstream claims.

## Non-Goals

No Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no unscoped RP1 MMIO writes, no pclk/hclk RP1_CLK_SYS writes,
no CLK_ETH_CTRL write, no reset-controller writes, no GPIO32/PHY reset, no
MDIO, no DMA, no descriptors, no interrupts or completions, no packet I/O, no
networking, no sockets, no SSH, no Phase 12.2, and no phase transition.

## Implemented Surface

Source module:

- src/rp1_ethernet.rs

Candidate contract:

    contract: phase12-rp1-ethernet-clock-reset-write-target-source-contract-v1
    report contract: phase12-rp1-ethernet-clock-reset-write-restore-report-contract-v1
    target: rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restore
    clock name: tsu_clk
    clock id: 29
    register: CLK_ETH_TSU_CTRL
    observed RP1 base: 0x1c00000000
    source offset: 0x018134
    Talos register address: 0x1c00018134
    width: 32
    access: 32-bit little-endian volatile load/store
    allowed write value: pre-read-raw-value-only

The local/static report is intentionally a contract-shaped evidence surface,
not proof that the hardware path has been exercised. Future hardware proof
classification is limited to the accepted source-contract vocabulary.

Boot-scenario surface:

- rpi5_rp1_ethernet_clock_reset_write_restore_candidate
- rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control
- scripts/rpi5-rp1-ethernet-clock-reset-write-restore-candidate-image.sh
- scripts/rpi5-rp1-ethernet-clock-reset-write-restore-control-image.sh

These paths compile, but were not staged or run on hardware by this task.

## Findings

- fixed: implemented the exact accepted CLK_ETH_TSU_CTRL candidate report
  with register address 0x1c00018134, preserved-field inventory, and
  idempotent pre_raw write/restore operation sequence.
- fixed: implemented the paired no-clock-write/no-Ethernet control report
  through the same report path while withholding target construction and
  candidate-only write/restore facts.
- fixed: added deterministic rejection coverage for RP1_CLK_SYS transition,
  CLK_ETH_CTRL write, reset-controller, GPIO32/PHY reset, MDIO/PHY,
  interrupt, DMA/descriptor, packet I/O, networking, SSH, Phase 12.2, and
  phase-transition claims.
- fixed: added build-script, entry dispatch, target report emitters, and image
  helper scripts for future candidate/control proof construction.
- not-an-issue: no hardwareTestLock was acquired because the task is local,
  static, and compile/test-only.
- deferred: serialized Pi 5 write/restore proof, restore evidence from real
  hardware, CLK_ETH_CTRL, GPIO32/PHY reset, MDIO/PHY, interrupts, DMA,
  descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future tasks.

No findings were removed.

## Validation

- Static inspection: accepted source contract, touched Rust modules, build
  scenario routing, and helper scripts.
- cargo fmt --all repaired formatting before focused validation.
- Focused tests:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet ethernet_clock_reset
  passed with 480 no_std tests, including the new write/restore candidate and
  control tests.
- Boot-scenario compile check:
  TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_clock_reset_write_restore_candidate cargo -Zjson-target-spec check --target targets/aarch64-talos-rpi5-bcm2712.json --quiet
  passed.
- Boot-scenario compile check:
  TALOS_BOOT_SCENARIO=rpi5_rp1_ethernet_clock_reset_write_restore_no_mmio_control cargo -Zjson-target-spec check --target targets/aarch64-talos-rpi5-bcm2712.json --quiet
  passed.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-core/evidence-map.json.
- Source implementation:
  src/rp1_ethernet.rs.
- Future boot-scenario surface:
  build.rs, src/main.rs, src/target/rpi5.rs, and
  scripts/rpi5-rp1-ethernet-clock-reset-write-restore-*-image.sh.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clock-reset-write-restore-closeout-20260610 on the next
worker wake if it remains queued and dependencies are satisfied. The closeout
must reconcile this local/static implementation and compile/test evidence
without running hardware, publishing boot archives, acquiring
hardwareTestLock, broadening to runtime ownership, or accepting Ethernet
driver behavior, packet I/O, networking, SSH, Phase 12.2, or a phase
transition.
