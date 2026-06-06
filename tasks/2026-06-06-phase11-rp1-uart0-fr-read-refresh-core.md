# Phase 11 RP1 UART0 FR-Read Refresh Core

Task: phase11-rp1-uart0-fr-read-refresh-core-20260606
Status: accepted
Owner: worker

## Goal

Refresh the narrow local/static RP1 UART0 PL011 flag-register read candidate
after Rust-entry UART10 marker visibility was accepted.

## Scope

- Wire `TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_read` directly from
  `rust_entry`.
- Keep the diagnostic read-only: exactly one 32-bit volatile read from
  `RP1_UART0_FR` at `0x1f_0003_0018`.
- Emit serial text through the UART10 early-serial helper that identifies the
  contract id, target, address, width, raw value if the read returns, and final
  classification.
- Build and review a non-published Pi 5 boot archive for the later serialized
  hardware proof.

## Non-Goals

No hardware run, boot publication, hardware lock acquisition, GPIO ownership,
pin mux changes, UART programming, RP1 clocks/resets, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, or
phase transition.

## Changes

- Fixed `src/main.rs` so the `rpi5_rp1_uart0_fr_read` boot scenario branches
  to `target::rpi5::run_rp1_uart0_fr_read_diagnostic()` from `rust_entry`
  and is excluded from the normal BootInfo/target initialization path.
- Refreshed `run_rp1_uart0_fr_read_diagnostic` to use the UART10 early-serial
  helper, perform one volatile 32-bit load from `RP1_UART0_FR`, report
  `mapped/read-value` only if the read returns, then halt in a spin loop.
- Updated the RP1/PCIe map contract and roadmap to describe this accepted
  local/static boundary without accepting hardware mapping behavior.

## Findings

- fixed: the FR-read helper existed, but the boot scenario was not wired from
  `rust_entry`; the refreshed scenario now reaches the diagnostic directly.
- fixed: the prior helper used `crate::println!` even though this diagnostic
  runs before normal target initialization; output now uses the accepted UART10
  early-serial helper.
- fixed: disassembly now shows `rust_entry` branching directly to
  `run_rp1_uart0_fr_read_diagnostic`.
- fixed: disassembly shows the diagnostic constructs `0x1f_0003_0018` and
  performs one `ldr w19, [x20]` from that address. Other nearby loads are
  UART10 flag-register polling for output flushes.
- not-an-issue: the raw `strings` output coalesces adjacent static strings in
  the image, but the required discriminator strings are present.
- deferred: bus-fault/trap and firmware-state classifications require the
  later serialized Pi 5 proof under `hardwareTestLock`.
- deferred: RP1 mapped/read-value behavior remains unaccepted until hardware
  proof evidence reaches the returned-read line.

## Evidence

- static inspection:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-refresh-core/static-inspection.md`
- evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-refresh-core/evidence-map.json`
- archive review:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-refresh-core/archive-review.txt`
- candidate archive:
  `target/talos-rpi5-rp1-uart0-fr-read-refresh-core.tar.gz`

The candidate archive SHA-256 is
`da35a26e817fd30b81874a701171de1b9d47c47024d5fc405a7068ca3b2e5d60`.
The reviewed `kernel_2712.img` is 45,832 bytes with
`text_offset=0`, `header_image_size=45832`, and `flags=12`.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with
  `423 passed`.
- image/archive inspection:
  `scripts/rpi5-archive-review.sh target/talos-rpi5-rp1-uart0-fr-read-refresh-core.tar.gz`
  passed.
- static image/header/symbol/disassembly inspection retained.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- diff checks: `git diff --check` passed before staging.

## Acceptance

Accepted as local/static refresh of the RP1 UART0 FR-read candidate only. This
does not accept RP1 mapped/read-value, unmapped/trap, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, or phase transition.
