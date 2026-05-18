# Phase 2 Pi 5 First Light

## Task

- Title: Pi 5 firmware handoff and firmware-preserved serial first light
- Owner: Seldon / supervisor loop
- Date: 2026-05-18
- Milestone: Phase 2.1
- Scope: Prepare, review, publish, and validate the first Talos boot archive for the physical Raspberry Pi 5 lab.

## Goal

Boot Talos through the normal Raspberry Pi 5 firmware path and capture enough serial evidence to prove that Talos code reached the CPU entry point.

## Acceptance Criteria

- A local boot tree can be staged with `config.txt`, `cmdline.txt`, `bcm2712-rpi-5-b.dtb`, and Talos `kernel_2712.img`.
- Pre-hardware review confirms the archive contents match the lab-controller contract and does not require direct UniFi access or credential handling.
- The lab publishes the archive through `PUT /boot/archive`.
- A single controlled power cycle reaches Talos code on the Pi 5.
- Serial output includes Talos version, exception level, DTB physical address, target name, service summary, and panic/halt behavior if boot fails later.
- A failed boot can be rolled back, and the evidence record captures the archive digest, publish result, power-cycle time, serial tail, classification, suspected cause, and next action.

Required validation level:

- Local preparation: formatting/lint/typecheck plus Pi 5 target build.
- Pre-hardware readiness: static inspection of staged archive contents.
- Acceptance: lab-controller API path plus serial hardware boot/output.

## Context

- Lab API: `http://talos-lab-api:8080`.
- Serial is currently reported as `configured=false`, so autonomous boot-result classification remains unavailable until the physical serial path is attached.
- The lab API currently lists a valid Pi 5 boot tree, including `bcm2712-rpi-5-b.dtb`, `kernel_2712.img`, `initramfs_2712`, and overlays.
- The supervisor must not publish archives or power-cycle the Pi without the hardware-test lock, acceptance criteria, and pre-hardware review.
- Talos has local staging helpers: `scripts/rpi5-image.sh` and `scripts/rpi5-boot-tree.sh`.

## Work Performed

- Added local Pi 5 image generation and boot-tree staging scripts.
- Documented the staging flow in the lab-controller notes.
- Marked the Phase 2.1 archive-publish criterion as having local staging support, with publish still intentionally not run.

## Evidence

Completed local checks:

~~~bash
sh -n scripts/rpi5-image.sh scripts/rpi5-boot-tree.sh
./scripts/rpi5-image.sh
./scripts/rpi5-boot-tree.sh target/test-pi-boot-source target/test-rpi5-boot-tree
tar -C target/test-rpi5-boot-tree -czf target/test-talos-rpi5-boot.tar.gz .
cargo fmt --check
cargo -Zjson-target-spec test
cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json
mdbook build
~~~

Results:

- Shell syntax check passed.
- Pi 5 image generation produced `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- Synthetic boot-tree staging produced the required archive-root files: `config.txt`, `cmdline.txt`, `bcm2712-rpi-5-b.dtb`, and `kernel_2712.img`.
- Synthetic tar archive inspection showed only relative safe paths.
- QEMU no_std tests passed: 2 tests.
- Pi 5 target build passed.
- mdBook build passed.

## Review

- Pre-hardware review findings: pending.
- Hardware test evidence: pending serial configuration and hardware-test lock.
- Post-hardware review findings: pending.

## Result

Local Phase 2.1 preparation is in place. The next hardware-facing step is a pre-hardware archive review using a real Pi firmware boot source, then exactly one controlled lab publish/power-cycle once serial evidence is available.

## Follow-Up

- Add a way to obtain or snapshot the current lab boot tree as a local `rpi5-boot-tree.sh` source without relying on direct host access.
- Once serial is configured, run the pre-hardware review and acquire the hardware-test lock before publishing the first Talos archive.
