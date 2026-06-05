# Phase 11 RP1 Diagnostic Entry-Control Source Core

Task: phase11-rp1-diagnostic-entry-control-source-core-20260605
Status: accepted

## Goal

Create a task-owned Pi 5 entry-control candidate that distinguishes Rust entry/handoff reachability from the prior RP1 register-read diagnostic path without running hardware or touching RP1 MMIO.

## Work Performed

- Added rpi5_rp1_entry_control as a focused Pi 5 boot scenario.
- Added scripts/rpi5-rp1-entry-control-image.sh and scripts/rpi5-rp1-entry-control-boot-tree.sh to produce the task-owned image and mirrored da591740/ boot tree.
- Routed the scenario immediately after the normal Pi 5 rust_entry early-phase line, before BootInfo parsing, target::init, and the RP1 GPIO/pin flushes that happen in the normal Pi 5 init path.
- Added a no-RP1-MMIO serial discriminator: rpi5-rp1-entry-control: rust-entry-control, rpi5-rp1-entry-control: no-rp1-mmio, classification=entry-control-reached, and PASS.
- Kept the existing RP1 UART0 FR read diagnostic unchanged except for cfg/allow cleanup needed by the new focused scenario.

## Findings

- fixed: the prior pre-MMIO diagnostic marker lived much later in the normal Pi 5 boot path, after Rust entry, boot-info parsing, target init, boot reports, memory planning, and allocator/reporting work. The new discriminator runs immediately after the accepted Rust-entry marker and stops before the first normal RP1 MMIO side effect.
- fixed: the new task-owned image has a unique marker/PASS path and contains no rpi5-rp1-uart0-fr-read, phase11-rp1-pcie-map-contract-v1, or mapped/read-value strings.
- fixed: focused-scenario dead-code warnings are suppressed through the existing crate-level focused Pi 5 scenario allow-list, matching established proof-scenario practice rather than weakening global warnings.
- removed: no raw assembly UART provenance markers were reintroduced.
- not-an-issue: root and da591740/ prefixed kernels match, text_offset=0, header_image_size=51808, flags=12, and magic=ARMd.
- deferred: only the later serialized Pi 5 proof can classify whether hardware reaches the entry-control marker/PASS after fetching the candidate.

## Evidence

- Source/script/image comparison notes: tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/source-script-image-comparison.md.
- Candidate identity: tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/candidate-identity.txt.
- Archive review: tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/archive-review.log.
- Boot-tree review: tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/rpi5-rp1-entry-control-boot-tree.log.
- Marker review: tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/marker-review.txt.
- Symbol/header/section review: tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/symbol-header-section-review.txt.
- Validation summary: tasks/evidence/2026-06-05-phase11-rp1-diagnostic-entry-control-source-core/validation-summary.txt.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 423 no_std tests using the documented local QEMU path.
- image/archive review: scripts/rpi5-rp1-entry-control-boot-tree.sh target/rpi5-local-cat-banner-boot-tree-local1 target/rpi5-rp1-entry-control-source-core-boot-tree and scripts/rpi5-archive-review.sh target/talos-rpi5-rp1-entry-control-source-core.tar.gz passed.
- static marker review: candidate kernel SHA-256 b3e62b950cf007a0ee8d1d7f420fd8c26c28573c5b6925a7f0d93d0b77a367ea, size 51808, contains the entry-control marker/PASS lines, and does not contain RP1 read/mapped strings or quarantined assembly marker strings.
- build/static comparison: accepted control, blocked RP1 read, revised pre-MMIO read, and entry-control images all retain text_offset=0, matching header/file size, flags=12, and ARMd magic.

## Result

Accepted and committed as a source/local entry-control candidate. The next queued Pi 5 proof may publish only target/talos-rpi5-rp1-entry-control-source-core.tar.gz from this accepted source task and must classify observed serial evidence as entry-control reached, pre-entry/handoff blocker after fetch, staging/capture blocker, or a legitimate fixed-candidate diagnostic path if the source evidence changes in a later task.
