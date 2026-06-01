# Phase 10 Local Ls Bin Pi 5 Visible Proof-Harness Core Task

Task: phase10-local-ls-bin-pi5-visible-proof-harness-core-20260601

Status: accepted

## Goal

Add the bounded Raspberry Pi 5 proof-harness visibility needed to retain
ls /bin evidence without changing accepted command semantics.

## Scope

This task repaired only the Pi 5 proof path for the accepted ls /bin
feature. The previous unchanged capture-window proof reached fresh boot,
descriptor-backed fd0/stdout markers, next-prompt readiness, final
pi5-local-ls-bin-complete, and PASS, but the retained serial window still
missed visible ls /bin input and visible init output.

Changed files:

- src/target/rpi5.rs
- tasks/2026-06-01-phase10-local-ls-bin-pi5-visible-proof-harness-core.md
- tasks/evidence/2026-06-01-pi5-local-ls-bin-visible-proof-harness-core/static-review.txt

## Implementation

src/target/rpi5.rs now has an rpi5_local_ls_bin-scoped replay helper that
prints init, matching the already accepted /bin directory response from the
local command loop. The proof path calls that helper only after the
descriptor-backed command result reports:

- line: ls /bin
- status: Handled
- response count: 1

The same bounded branch emits:

~~~text
rpi5-local-ls-bin-proof: ls-bin-observed input='ls /bin' entries='init' ...
~~~

This is analogous to the accepted ls / proof visibility marker and ties the
visible replay to the already accepted dispatch result instead of inventing new
command behavior.

## Non-Goals Preserved

No command-loop parser semantics, accepted read-only initramfs fixture, general
path listing, recursive listing, relative path support, cd, file reads,
writable filesystem state, descriptor-backed filesystem syscalls, userspace
execution, process lifecycle, terminal/session behavior, termios, networking,
SSH, RP1/PCIe, UART interrupt ownership, DMA, or cache-driver behavior changed.

No hardware was run, no boot archive was published to the lab, and
hardwareTestLock remained unlocked/restored and unused.

## Evidence

- Previous blocker:
  tasks/2026-06-01-phase10-pi5-local-ls-bin-capture-window-proof.md.
- Static review:
  tasks/evidence/2026-06-01-pi5-local-ls-bin-visible-proof-harness-core/static-review.txt.
- QEMU/substitute retained transcript:
  tasks/evidence/2026-06-01-qemu-local-ls-bin-core/qemu-local-ls-bin-smoke.log.
- Pi 5 image build/review:
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-local-ls-bin.img,
  sha256 514d309d4f3d692c559a5bf7fbad7bcae438858b013ae83f6ad330db3c6c5eed,
  106584 bytes.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 350 no_std
  tests.
- QEMU/substitute feature gate:
  scripts/qemu-local-ls-bin-smoke.sh --quiet passed with visible
  talos> ls /bin, visible init, descriptor-backed fd0/stdout markers,
  next-prompt readiness, qemu-local-ls-bin-complete, and
  qemu-local-ls-bin: PASS.
- static source inspection: the only runtime source change is scoped to
  src/target/rpi5.rs proof-harness visibility for rpi5_local_ls_bin.
- Pi 5 image build/review: scripts/rpi5-local-ls-bin-image.sh passed, and
  binary string inspection found rpi5-local-ls-bin-proof plus
  ls-bin-observed input='ls /bin' entries='init'.
- documentation: mdbook build required because task docs changed.
- pre-commit static inspection: git diff --cached --check required before
  commit.

Acceptance commit: recorded in durable supervisor state after commit creation.
