# Static Inspection

Task id: phase11-rp1-uart0-fr-read-hold-control-core-20260606

Evidence level: static/source inspection plus image/archive/disassembly
inspection.

## Findings

- fixed: build.rs registers rpi5_rp1_uart0_fr_read_hold_control as an explicit
  boot scenario.
- fixed: src/main.rs routes the selected Pi 5 scenario directly from rust_entry
  to run_rp1_uart0_fr_read_hold_control before BootInfo parsing, target
  initialization, boot reports, memory planning, allocator setup, scheduler
  work, or command-loop work.
- fixed: src/target/rpi5.rs emits the unique pre-read control marker through
  write_early_static and wait_uart10_empty_early_phase before touching RP1.
- fixed: the selected disassembly constructs 0x1f00030018 in x20 with
  mov x20, #24; movk x20, #3, lsl #16; movk x20, #31, lsl #32; then performs
  exactly one contracted ldr w19, [x20].
- fixed: post-read reporting includes phase11-rp1-pcie-map-contract-v1,
  rp1-uart0-fr-read, address, width, raw value, mapped/read-value
  classification, and a unique terminal hold marker.
- deferred: Pi 5 candidate fetch, pre-read control visibility, post-read return
  visibility, trap/no-return behavior, and restore hygiene remain for the
  queued serialized Pi 5 discriminator.
- not-an-issue: UART10 early-serial helper polling retains ldr w10, [x9, #24]
  instructions against x9 = 0x107d001000. Those are not RP1 loads.

## Retained Evidence

- Archive identity:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/candidate-identity.txt.
- Archive review:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/archive-review.txt.
- Selected strings:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/string-review.txt.
- Symbols:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/symbol-review.txt.
- Disassembly proof:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/disassembly-review.txt.

## Classification

local-static-hold-control-rp1-uart0-fr-read-candidate-accepted.
