# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Preflight Core

Task id: phase12-rp1-ethernet-gpio32-phy-reset-preflight-core-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-gpio32-phy-reset-preflight-core-local-static-accepted
Evidence level: static inspection, fmt, and focused local/static Rust
report-model tests only. No Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, GPIO/RIO/pad/MMIO write, PHY reset assertion or
deassertion, MDIO transaction, packet I/O, networking, sockets, SSH, Phase
12.2, or phase transition was performed.

## Goal

Implement the bounded local/static candidate/control report surface for the
accepted GPIO32 / ETH_RST_N read-only preflight contract.

## Scope

- Consumed accepted source contract commit
  278b754ff3a8f589429cd91aaadc3085db6e7b90.
- Added a local/static Rust source contract and preflight report surface for
  the GPIO32 / ETH_RST_N PHY reset prerequisite.
- Candidate evidence records the accepted input frontier, rp1_eth identity,
  RGMII-ID phy1, RP1 GPIO32 / ETH_RST_N route, active-low polarity, logical
  assertion/deassertion mapping, 5 ms reset duration, Linux MACB MDIO reset
  hook relationship, Phase 11 GPIO constraints, future write/restore
  invariants, rejected claims, retained risks, and source evidence.
- Paired control evidence uses the same report path while withholding
  candidate-only GPIO32, ETH_RST_N, PHY reset, MDIO, and Ethernet facts.
- Added focused tests for accepted candidate construction, accepted control
  construction, source/shape bypass rejection, GPIO ownership rejection, PHY
  reset assertion/deassertion rejection, MDIO/PHY ownership rejection, runtime
  write rejection, packet I/O rejection, and phase-transition rejection.

## Non-Goals

No Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no RP1 GPIO/RIO/pad/MMIO write, no PHY reset assertion or
deassertion, no MDIO transaction, no clock/reset write, no Ethernet driver
behavior, no DMA/descriptors, no interrupts, no packet I/O, no networking,
no sockets, no SSH, no Phase 12.2, and no phase transition.

## Implemented Surface

Source module:

- src/rp1_ethernet.rs

Candidate contract:

    source contract: phase12-rp1-ethernet-gpio32-phy-reset-source-contract-v1
    source task: phase12-rp1-ethernet-gpio32-phy-reset-source-contract-20260610
    report contract: phase12-rp1-ethernet-gpio32-phy-reset-preflight-report-contract-v1
    controller: rp1_eth
    compatible: raspberrypi,rp1-gem / cdns,macb
    PHY mode: rgmii-id
    PHY handle/node/reg: phy1 / ethernet-phy@1 / 0x1
    GPIO route: rp1_gpio line 32 / ETH_RST_N
    polarity: active-low
    logical assertion: value 1 drives ETH_RST_N physically low
    logical deassertion: value 0 drives ETH_RST_N physically high
    reset duration: 5 ms

The local/static report is a contract-shaped evidence surface, not hardware
proof. Future hardware proof classification is limited to read-only preflight
visibility/control output and must still provide candidate/control identity,
TFTP, serial freshness, final identity, and task-owned JSON if selected.

## Findings

- fixed: implemented the exact accepted GPIO32 PHY-reset candidate report with
  source identities, active-low logical/physical reset semantics, 5 ms source
  reset duration, Linux MDIO reset hook relationship, retained Phase 11 GPIO
  constraints, rejected claims, retained risks, and future write/restore
  safety invariants.
- fixed: implemented the paired no-GPIO/no-Ethernet control report through the
  same report path while withholding candidate-only GPIO32/ETH_RST_N/PHY-reset
  facts.
- fixed: validators reject source-contract bypasses, control fact leakage,
  GPIO ownership, PHY reset assertion/deassertion, MDIO/PHY ownership, runtime
  writes, Ethernet readiness, broad MMIO readiness, interrupts, DMA/descriptors,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition.
- not-an-issue: no hardwareTestLock was acquired because this task is
  local/static and test-only.
- deferred: serialized read-only Pi 5 preflight visibility proof, write-backed
  GPIO32 reset ownership, MDIO/PHY ownership, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition remain future tasks.

No findings were removed.

## Validation

- Static inspection: accepted GPIO32 PHY-reset source contract and touched
  src/rp1_ethernet.rs report surface.
- Formatting:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo fmt --all
  passed.
- Focused tests:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet rp1_ethernet_gpio32_phy_reset_preflight
  passed with 486 no_std tests, including the new GPIO32 PHY-reset candidate,
  control, and rejection tests.
- Docs:
  /home/node/.cargo/bin/mdbook build
  passed; HTML was written under book/ and the search-index size warning was
  informational.

No boot-scenario compile checks were required because this task did not add or
change candidate/control boot scenarios.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-preflight-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-preflight-core/evidence-map.json.
- Source implementation:
  src/rp1_ethernet.rs.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gpio32-phy-reset-preflight-closeout-20260610 on the next
worker wake if dependencies remain satisfied. The closeout must reconcile this
local/static implementation without running hardware, publishing boot
archives, acquiring hardwareTestLock, accepting GPIO ownership, asserting or
deasserting PHY reset, performing MDIO, accepting Ethernet driver behavior,
packet I/O, networking, SSH, Phase 12.2, or a phase transition.
