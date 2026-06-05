# Phase 11 RP1 Diagnostic Pre-entry/Handoff Source Core

Task: `phase11-rp1-diagnostic-preentry-handoff-source-core-20260605`
Status: accepted

## Goal

Compare the accepted Pi 5 boot path with the RP1 UART0 FR diagnostic candidate, fix a concrete source/script/image issue if found, and retain enough static evidence for a later serialized Pi 5 proof. This task does not run hardware or claim an RP1 mapping result.

## Work Performed

- Compared the RP1 diagnostic candidate against the accepted Pi 5 boot-tree/script/linker/entry shape used by prior prompt-capable controls.
- Kept raw assembly entry-provenance markers out of this candidate because prior Phase 10 evidence quarantined that path after it made accepted prompt-capable controls fail.
- Fixed the diagnostic ordering so it emits `rpi5-rp1-uart0-fr-read: start` and `rpi5-rp1-uart0-fr-read: pre-mmio-read`, flushes UART10, and only then performs the single 32-bit volatile read from `0x1f_0003_0018`.

## Findings

- fixed: the previous diagnostic performed the risky RP1 MMIO read before any diagnostic-specific serial line, so the later hardware proof could not distinguish reaching Talos/pre-MMIO from stopping at the RP1 read boundary.
- fixed: the revised diagnostic keeps the one-read contract but adds a pre-MMIO serial discriminator before the read.
- removed: raw `TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO` assembly marker routing is not added; prior evidence records that marker path as invasive for accepted Pi 5 controls.
- not-an-issue: the candidate helper scripts use the same standard Pi 5 image and boot-tree pattern as accepted controls: build a scenario image, copy it to both `kernel_2712.img` and `kernel8.img`, and mirror required files into the `da591740/` TFTP prefix.
- not-an-issue: the linker/header/entry placement remains the standard Pi 5 shape with `text_offset=0`, `flags=12`, `magic=ARMd`, and `_start` at `0x200000`.
- deferred: the next hardware proof must classify whether the revised candidate reaches the pre-MMIO marker, the returned read-value classification, or remains blocked before visible Talos output.

## Evidence

- Source/script/image comparison notes: `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core/source-script-image-comparison.md`.
- Candidate identity: `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core/candidate-identity.txt`.
- Archive review: `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core/archive-review.log`.
- Marker review: `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core/marker-review.txt`.
- Symbol/section review: `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core/symbol-and-section-review.txt`.
- Boot config and listing: `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core/boot-config-and-listing.txt`.
- Validation summary: `tasks/evidence/2026-06-05-phase11-rp1-diagnostic-preentry-handoff-source-core/validation-summary.txt`.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 423 no_std tests.
- image/archive review: `scripts/rpi5-rp1-uart0-fr-read-boot-tree.sh` rebuilt the revised candidate boot tree, and `scripts/rpi5-archive-review.sh` passed.
- static image review: candidate archive `target/talos-rpi5-rp1-uart0-fr-read-preentry-handoff-source-core.tar.gz` has SHA-256 `2640ab9ceabee343ee1426b7137e1597687517f56d3b61f58a7ac0e7ab4b6608`; root and prefixed kernels match SHA-256 `4500b99a4405f91176d39dc8178fcd396611e97577eb98c357927df05de6f792`, size `87480`, `text_offset=0`, `header_image_size=87480`, `flags=12`, `magic=ARMd`.
- static marker review: revised image contains `rpi5-rp1-uart0-fr-read: start`, `rpi5-rp1-uart0-fr-read: pre-mmio-read`, `phase11-rp1-pcie-map-contract-v1`, `classification=mapped/read-value`, and PASS; it does not contain `TALOS: asm_start` or `TALOS: asm_pre_rust_entry`.
- diff hygiene: `git diff --check` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed with the existing large search-index warning.

## Result

Accepted revised local candidate for a later serialized Pi 5 proof. The next proof may publish only this revised candidate and classify whether hardware reaches the pre-MMIO marker, returns the mapped/read-value line, or remains blocked before visible Talos output.
