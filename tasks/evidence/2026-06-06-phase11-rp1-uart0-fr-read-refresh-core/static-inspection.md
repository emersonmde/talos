# Static Inspection

Task: phase11-rp1-uart0-fr-read-refresh-core-20260606

## Source

- `src/main.rs` now includes `rpi5_rp1_uart0_fr_read` in the Pi 5 diagnostic
  scenario allow-list, branches from `rust_entry` to
  `target::rpi5::run_rp1_uart0_fr_read_diagnostic()`, and excludes that
  scenario from the normal BootInfo/target initialization path.
- `src/target/rpi5.rs` keeps `RP1_UART0_FR = RP1_UART0_BASE + 0x18`, with
  `RP1_UART0_BASE = 0x1f_0003_0000`, so the selected address is
  `0x1f_0003_0018`.
- `run_rp1_uart0_fr_read_diagnostic` writes start and pre-MMIO markers through
  `write_early_static`, flushes UART10, calls `read_rp1_reg_u32(RP1_UART0_FR)`
  once, reports contract id `phase11-rp1-pcie-map-contract-v1`, target
  `rp1-uart0-fr-read`, address, width, raw value, `mapped/read-value`, and
  PASS only after the read returns, then spins forever.

## Image and Archive

- archive: `target/talos-rpi5-rp1-uart0-fr-read-refresh-core.tar.gz`
- archive SHA-256:
  `da35a26e817fd30b81874a701171de1b9d47c47024d5fc405a7068ca3b2e5d60`
- root kernel SHA-256:
  `1e4dc50ad6ac8662beec8fd92e41930e2b083956eb46f715924729aff8fa131a`
- prefixed kernel SHA-256:
  `1e4dc50ad6ac8662beec8fd92e41930e2b083956eb46f715924729aff8fa131a`
- kernel size: 45,832 bytes
- arm64 Image header: `text_offset=0`, `header_image_size=45832`,
  `flags=12`, `magic=ARMd`

## Disassembly

Retained disassembly:
`tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-refresh-core/disassembly.txt`.

Key observations:

- `rust_entry` contains a direct branch to
  `run_rp1_uart0_fr_read_diagnostic`.
- The diagnostic emits the pre-MMIO marker before constructing the RP1 address.
- The diagnostic constructs `x20 = 0x1f_0003_0018`:
  `mov x20, #0x18`, `movk x20, #0x3, lsl #16`,
  `movk x20, #0x1f, lsl #32`.
- The RP1 read is `ldr w19, [x20]`, a single 32-bit load from
  `0x1f_0003_0018`.
- Other `ldr w10, [x9, #0x18]` instructions in the retained snippet are
  UART10 flag-register polling for early serial output flushes.

## Findings

- fixed: the scenario is now reachable from `rust_entry`.
- fixed: early output uses the UART10 early-serial helper instead of
  pre-initialization `println!`.
- fixed: archive review passes for the refreshed boot tree.
- not-an-issue: no hardware proof was run or claimed by this task.
- deferred: trap/firmware-state/mapped-value classification remains for the
  later serialized Pi 5 proof.

## Non-Acceptance

This inspection does not accept RP1 mapped/read-value, unmapped/trap,
firmware-state behavior, GPIO ownership, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
transition behavior.
