# Talos Device Tree and Boot Reports Review

Task: talos-review-device-tree-boot-reports-20260603
Status: accepted

## Scope

Reviewed the raw FDT cursor helpers, /chosen bootargs parsing, /memory and
/reserved-memory extraction, RPi5 DTB/memory scan call sites, and RPi5 boot
report formatting helpers.

## Findings

- Fixed: DeviceTree::memory_banks stopped scanning at the first root
  memory@... node. Valid FDTs may expose memory through multiple root memory
  nodes, so later banks could be omitted from boot memory planning and reports.
  The scanner now closes one memory node and continues scanning until FDT_END,
  with regression coverage for two root memory nodes.
- Fixed: raw FDT pointer arithmetic used unchecked usize addition after bounds
  checks on block-relative offsets. Malformed physical base/offset pairs could
  wrap before volatile reads or static slice construction. The raw cursor and
  string helpers now use checked address arithmetic for every derived pointer.
- Fixed: RPi5 report helpers derived DTB/layout end addresses with unchecked
  addition and multiplication. The report paths now use checked arithmetic and
  print an explicit unavailable/overflow state instead of wrapping derived
  addresses.
- Not an issue: /chosen bootargs remains a narrow property lookup rather than a
  generic chosen-node model. Current consumers only need bootargs, and broad
  stdout-path or initrd parsing would be feature expansion outside this review.
- Not an issue: /reserved-memory currently reports only immediate child reg
  ranges and no-map/reusable flags. That is enough for current memory exclusion
  policy; deeper node semantics or ranges translation should wait until a
  feature needs it.
- Deferred: RPi5 early progress markers around DTB/reserved-memory scanning are
  still useful hardware boundary markers after recent boot evidence gaps. This
  review did not remove them because they are active diagnostics, not stale
  bring-up noise.

## Changes

- src/device_tree/memory.rs continues /memory scanning across multiple root
  memory@... nodes and adds a no_std regression fixture for that FDT shape.
- src/device_tree/raw.rs centralizes checked derived-address arithmetic for raw
  reads, node names, property values, and strings-block lookups.
- src/boot/rpi5_reports.rs guards derived end-address reporting for DTB ranges
  and post-allocator translation-table layout output.

No new hardware run, feature surface, userspace behavior, filesystem behavior,
networking, RP1/PCIe, UART interrupt ownership, or DMA/cache policy was added.

## Validation

- Static inspection: reviewed src/device_tree/*, src/boot/rpi5_reports.rs, and
  the direct DTB/report call sites in src/boot/rpi5.rs with rg/sed.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 358 no_std
  tests, including device_tree_reads_multiple_root_memory_nodes.
- default target check: cargo -Zjson-target-spec check --quiet passed.
- RPi5 target check: TALOS_BOOT_SCENARIO=rpi5_local_serial_command_loop cargo
  -Zjson-target-spec check --target targets/aarch64-talos-rpi5-bcm2712.json
  --quiet passed.
- QEMU target check: TALOS_BOOT_SCENARIO=qemu_local_serial_command_loop cargo
  -Zjson-target-spec check --target targets/aarch64-talos-virt.json --quiet
  passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed after adding this
  task record.
- static diff hygiene: git diff --check and git diff --cached --check passed.
- hardwareTestLock remained unlocked/restored and unused; no hardware claim was
  made.

## Remaining Risks

- The parser still intentionally supports only the FDT cell widths currently
  accepted by Talos memory policy. Supporting zero-size-cells buses or address
  translation through ranges should come with the feature that needs those
  semantics.
- RPi5 report formatting remains early-boot UART oriented. A future structured
  boot-report sink would be a broader subsystem boundary change and was not
  introduced here.

Accepted commit: recorded in durable state for
talos-review-device-tree-boot-reports-20260603.
