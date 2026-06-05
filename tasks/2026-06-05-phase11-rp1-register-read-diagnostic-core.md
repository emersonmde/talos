# Phase 11 RP1 Register Read Diagnostic Core

## Task

- Title: Phase 11 RP1 register-read diagnostic core
- Owner: worker
- Date: 2026-06-05
- Milestone: Phase 11 Milestone 11.1, RP1 and PCIe Mapping
- Scope: local implementation and static/build review for the contract-selected RP1 UART0 flag-register read

## Goal

Implement the smallest Pi 5 diagnostic candidate that can later prove the accepted RP1/PCIe map contract on hardware without changing normal shell behavior or broadening into driver ownership.

## Acceptance Criteria

- The diagnostic candidate is implemented behind a narrow explicit Pi 5 proof harness.
- Static review shows the diagnostic performs only the contract-approved non-destructive read/classification path before reporting over accepted serial output.
- Local build/script evidence proves the candidate artifact can be produced and reviewed before hardware.
- Retained regression gates show accepted Phase 10 shell/VFS behavior is not broken by the diagnostic additions.

## Context

The accepted contract `phase11-rp1-pcie-map-contract-v1` maps the RP1 UART0 PL011 flag register from RP1 bus `0xc0_4003_0018` to CPU physical `0x1f_0003_0018`, width 32 bits. This task intentionally does not acquire `hardwareTestLock`, publish an archive, power-cycle the Pi 5, configure GPIO, enable interrupts, or touch DMA/cache policy.

## Work Performed

- Registered a new Pi 5 boot scenario: `rpi5_rp1_uart0_fr_read`.
- Added a boot hook that runs only for that scenario.
- Added `RP1_UART0_FR = 0x1f_0003_0018` and a single 32-bit volatile read path.
- Added serial output for contract id, target, address, width, raw value, `mapped/read-value` classification, and PASS.
- Added focused image and boot-tree helper scripts for the later serialized hardware proof.
- Updated the mapping contract with the implemented local core and artifact helper names.

## Findings

- fixed: the diagnostic is selected only by `TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_read`, so normal accepted shell paths are unchanged.
- fixed: the diagnostic performs no RP1 writes; it reads only `0x1f_0003_0018` as a 32-bit volatile load before reporting.
- fixed: artifact helpers produce a named candidate image and a later hardware-proof boot tree without publishing or acquiring the hardware lock.
- deferred: actual hardware classification, TFTP evidence, serial cursor capture, restore proof, and any bus-fault/trap classification are reserved for `phase11-rp1-register-read-pi5-proof-20260605`.
- not-an-issue: the PL011 flag-register raw value is variable; the hardware proof should classify a successful read rather than require an exact value.

## Evidence

- Static inspection note: `tasks/evidence/2026-06-05-phase11-rp1-register-read-diagnostic-core/static-inspection.md`.
- Build/image review log: `tasks/evidence/2026-06-05-phase11-rp1-register-read-diagnostic-core/rpi5-rp1-uart0-fr-read-image.log`.
- Unit/build gate log: `tasks/evidence/2026-06-05-phase11-rp1-register-read-diagnostic-core/cargo-test.log`.
- Formatting log: `tasks/evidence/2026-06-05-phase11-rp1-register-read-diagnostic-core/cargo-fmt.log`.
- Diff hygiene log: `tasks/evidence/2026-06-05-phase11-rp1-register-read-diagnostic-core/git-diff-check.log`.
- Docs build log: `tasks/evidence/2026-06-05-phase11-rp1-register-read-diagnostic-core/mdbook-build.log`.
- Image identity: `kernel_2712-rp1-uart0-fr-read.img`, SHA-256 `bed60fc8babf5c91117dd1ccb7c9a105af2bcd30cfedcc098a414029b46fe3c5`, size `87392`, arm64 Image header `text_offset=0`, `header_image_size=87392`, `flags=12`, `magic=ARMd`.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 423 no_std tests.
- image/static review: `scripts/rpi5-rp1-uart0-fr-read-image.sh` produced the candidate image and header values listed above.
- QEMU/substitute: not run because this task did not touch shared shell/VFS runtime paths; the unit suite retained the accepted local shell/VFS regressions.
- diff hygiene: `git diff --check` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed with the existing large search-index warning.

Hardware test evidence: not required and not run for this diagnostic-core task.
Post-hardware review findings: not applicable.

## Result

Accepted local diagnostic core candidate: `rpi5-rp1-uart0-fr-read`, contract `phase11-rp1-pcie-map-contract-v1`, address `0x1f_0003_0018`, width 32-bit. The next queued Pi 5 proof task may build/stage this candidate, acquire `hardwareTestLock`, capture candidate identity, fresh serial cursor, TFTP delta, restore evidence, and classify the hardware result.
