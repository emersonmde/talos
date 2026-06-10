# Phase 12 RP1 Ethernet CLK_ETH_CTRL Write-Restore Core

Task id: phase12-rp1-ethernet-clk-eth-ctrl-write-restore-core-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clk-eth-ctrl-write-restore-core-local-static-accepted
Evidence level: static inspection, fmt, and focused local/static Rust
report-model tests only. No Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, or runtime RP1 MMIO action was performed.

## Goal

Implement the bounded local/static candidate/control write-restore report
surface for the accepted CLK_ETH_CTRL / RP1_CLK_ETH / rp1_eth tx_clk source
contract without running hardware or broadening clock/reset ownership.

## Scope

- Consumed accepted source contract commit
  fc12771db08a5837d304ecf6e8ed254e1f456db0.
- Added a local/static Rust contract and report surface for the selected
  CLK_ETH_CTRL idempotent write/restore target at Talos address
  0x1c00018064.
- Candidate evidence records contract identity, target identity, register
  address, volatile width/access, pre-read-raw-only write rule, preserved
  fields, pre-read/write/post-read/restore-write/restore-read operation
  sequence, future proof classifications, rejected claims, and retained risks.
- Paired control evidence uses the same report path while constructing no
  writable clock target and withholding candidate-only CLK_ETH_CTRL facts.
- Added focused tests for accepted candidate report construction, accepted
  control report construction, source/shape bypass rejection, shared-clock
  write rejection, TSU same-shaped retry rejection, non-idempotent transition
  rejection, DMA/descriptor rejection, and phase-transition rejection.

## Non-Goals

No Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no RP1 MMIO write, no non-idempotent clock transition, no
RP1_CLK_SYS pclk/hclk write, no CLK_ETH_TSU_CTRL retry, no divider/select/PLL
or frequency-counter write, no reset-controller write, no GPIO32/PHY reset, no
MDIO/PHY ownership, no DMA, no descriptors, no interrupts, no packet I/O, no
networking, no sockets, no SSH, no Phase 12.2, and no phase transition.

## Implemented Surface

Source module:

- src/rp1_ethernet.rs

Candidate contract:

    contract: phase12-rp1-ethernet-clk-eth-ctrl-write-target-source-contract-v1
    source task: phase12-rp1-ethernet-clk-eth-ctrl-source-contract-20260610
    report contract: phase12-rp1-ethernet-clk-eth-ctrl-write-restore-report-contract-v1
    target: rp1-ethernet-clk-eth-ctrl-idempotent-write-restore
    clock name: tx_clk
    clock id: 16
    register: CLK_ETH_CTRL
    observed RP1 base: 0x1c00000000
    source offset: 0x018064
    Talos register address: 0x1c00018064
    width: 32
    access: 32-bit little-endian volatile load/store
    allowed write value: pre-read-raw-value-only

The local/static report is a contract-shaped evidence surface, not proof that
the hardware path has been exercised. Future hardware proof classification is
limited to the accepted source-contract vocabulary and must still provide
candidate/control identity, TFTP, serial freshness, final identity, and restore
evidence.

## Findings

- fixed: implemented the exact accepted CLK_ETH_CTRL candidate report with
  register address 0x1c00018064, preserved-field inventory, idempotent
  pre_raw write/restore operation sequence, retained risks, and future proof
  classifications.
- fixed: implemented the paired no-CLK_ETH_CTRL-write/no-Ethernet control
  report through the same report path while withholding writable target
  construction and candidate-only CLK_ETH_CTRL facts.
- fixed: validators reject source-contract bypasses, control target leakage,
  TSU same-shaped retry claims, non-idempotent transition claims,
  shared-clock writes, reset/GPIO/MDIO/DMA/descriptor/interrupt/packet/network
  claims, Phase 12.2, and phase transition.
- not-an-issue: no hardwareTestLock was acquired because this task is
  local/static and test-only.
- deferred: serialized Pi 5 candidate/control proof, boot archive publication,
  restore evidence, broad clock/reset ownership, shared-clock ownership,
  reset-controller ownership, GPIO32/PHY reset, MDIO/PHY ownership, DMA,
  descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2,
  and phase transition remain future tasks.

No findings were removed.

## Validation

- Static inspection: accepted CLK_ETH_CTRL source contract, retained source
  excerpts referenced by the contract, existing TSU write/restore proof
  closeout, and touched source module.
- Formatting:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo fmt --all
  passed.
- Focused tests:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet ethernet_clock_reset
  passed with 483 no_std tests, including the new CLK_ETH_CTRL candidate,
  control, and rejection tests.
- Docs:
  /home/node/.cargo/bin/mdbook build
  passed; HTML was written under book/ and the search-index size warning was
  informational.

No boot-scenario compile checks were required because this task did not add or
change candidate/control boot scenarios.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-core/evidence-map.json.
- Source implementation:
  src/rp1_ethernet.rs.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clk-eth-ctrl-write-restore-closeout-20260610 on the next
worker wake if dependencies remain satisfied. The closeout must reconcile this
local/static implementation without running hardware, publishing boot
archives, acquiring hardwareTestLock, broadening to hardware visibility,
accepting broad clock/reset ownership, accepting Ethernet driver behavior,
packet I/O, networking, SSH, Phase 12.2, or a phase transition.
