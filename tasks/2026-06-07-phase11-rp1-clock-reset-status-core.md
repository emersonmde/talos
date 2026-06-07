# Task: Phase 11 RP1 Clock/Reset Status Core

Task ID: phase11-rp1-clock-reset-status-core-20260607

Status: accepted

Evidence level: static/archive inspection, fmt/lint/typecheck, unit tests

## Goal

Implement the accepted read-only RP1 clock manager status diagnostic core and
paired no-MMIO/no-RP1/no-GIC control locally, producing candidate artifacts but
no hardware run.

## Scope

- Used only the target, allowed reads, report fields, forbidden operations,
  classifications, and control requirements accepted by
  phase11-rp1-clock-reset-status-source-contract-20260607.
- Implemented the real candidate as one read-only RP1 clock manager snapshot:
  PLL_SYS_CS, CLK_SYS_CTRL, CLK_SYS_DIV_INT, CLK_SYS_SEL,
  CLK_SLOW_SYS_CTRL, CLK_UART_CTRL, CLK_UART_DIV_INT, and CLK_UART_SEL.
- Implemented the paired control candidate with the same serial/output shape,
  simulated zero raw values, and no constructed RP1 clock/reset, GPIO/RIO/pads,
  MSI-X/PCIe/MIP, or GIC MMIO addresses.
- Retained static/archive evidence for the real and control candidate
  boundary.

## Non-Goals

No Pi 5 hardware run, hardwareTestLock acquisition, published boot archive,
RP1 clock/reset writes, GPIO writes, event generation, interrupt enablement or
delivery, GIC IAR/EOIR acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  rpi5_rp1_clock_manager_status_read and
  rpi5_rp1_clock_manager_status_no_mmio_control.
- fixed: added RP1 clock manager constants for the accepted read-only
  translated addresses from the source contract.
- fixed: the real candidate emits
  TALOS: rp1-clock-manager-status-result with the accepted contract id, target,
  clock manager base, selected address/raw fields, decoded PLL lock/refdiv,
  clock enable/source/aux/divider/status fields, retained GPIO14/GPIO16 fsel 13
  blocker context, and classification=rp1-clock-manager-status-visible.
- fixed: the real candidate performs only the accepted read operations: the
  eight 32-bit volatile loads named by the source contract.
- fixed: the control candidate emits
  TALOS: rp1-clock-manager-status-control with the same output shape,
  not-constructed address fields, simulated zero raw values, and
  classification=simulated/control.
- fixed: control archive and disassembly review prove the control image omits
  the real diagnostic marker, real classification, selected RP1/GIC address
  strings, and forbidden RP1 clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC
  MMIO address construction.
- deferred: Pi 5 control run, Pi 5 real diagnostic run, RP1 clock/reset writes,
  GPIO ownership retries, event generation, interrupt delivery, handler
  ownership, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
  enumeration, Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR polling loads/stores remain present in both
  candidates because they are the existing firmware-preserved serial flush
  path, not RP1 clock/reset/GPIO/RIO/pads/MSI-X/PCIe/MIP/GIC MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-clock-manager-status-read-core.tar.gz
- Archive SHA256:
  22d7592ef17ff5641e724d5b9ebcb3496071cb9237e22adadc915358079d268d
- kernel_2712.img SHA256:
  4267655039e0882377407302f012c610f0b011a23507c1a6aabcbd515f83dd8b
- kernel_2712.img size: 47280 bytes
- Marker: TALOS: rp1-clock-manager-status-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-clock-manager-status-no-mmio-control-core.tar.gz
- Archive SHA256:
  9fff700d3f1f324954e29d0179e6695b118f0e4bd68d10318cdb968fabf99e55
- kernel_2712.img SHA256:
  d0f3150df81a78f1a9bc7960390256d8162ec49ca92348604ac64ab7336ddbab
- kernel_2712.img size: 47120 bytes
- Marker: TALOS: rp1-clock-manager-status-control

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- Archive review:
  scripts/rpi5-rp1-clock-manager-status-read-review.sh passed.
- Archive review:
  scripts/rpi5-rp1-clock-manager-status-no-mmio-control-review.sh passed.
- Static disassembly/source inspection: passed; real candidate constructs only
  the accepted 0x1f00018000/0x1f00020000 clock manager read addresses and
  performs eight accepted 32-bit volatile loads; control candidate constructs
  no forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock/reset MMIO address.
- git diff --check: pass.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check: pass.

## Result

Accepted. This accepts only the local/static real candidate, the local/static
no-MMIO/no-RP1/no-GIC control candidate, and their archive/static boundary. It
does not accept Pi 5 hardware behavior, RP1 clock/reset ownership, RP1
clock/reset writes, GPIO ownership, GPIO event generation, interrupt delivery,
GIC acknowledgement, ISR/handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or a phase
transition.

## Follow-Up

Promote phase11-rp1-clock-reset-status-control-pi5-20260607 only after this
task is accepted and committed and hardwareTestLock remains unlocked/restored.
The next task owns the serialized Pi 5 no-MMIO/no-RP1/no-GIC control proof
before any real clock manager status diagnostic run.
