# Phase 10 Pi 5 Generated-Root Boot Transport Contract Static Inspection

Task: phase10-pi5-generated-root-boot-transport-contract-20260605

## Sources Reviewed

- tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-core.md
- tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-closeout.md
- docs/src/project/phase10-generated-root-no-rebuild-transport-contract.md
- docs/src/project/lab-controller.md
- scripts/rpi5-boot-tree.sh
- scripts/rpi5-local-serial-command-loop-boot-tree.sh
- scripts/rpi5-archive-review.sh
- scripts/qemu-local-shell-generated-root-no-rebuild-transport-smoke.sh
- tools/generated-root-artifact.rs
- src/initramfs.rs
- src/device_tree/chosen.rs
- src/boot/rpi5.rs
- linker.ld
- docs/src/roadmap.md
- Public Raspberry Pi documentation mirror:
  https://raw.githubusercontent.com/raspberrypi/documentation/master/documentation/asciidoc/computers/config_txt/boot.adoc

## Static Findings

- fixed: The accepted local/QEMU generated-root transport already has a stable
  artifact format, deterministic fallback policy, and two-artifact same-kernel
  evidence. The Pi 5 task should reuse that artifact format instead of defining
  a new boot-only format.
- fixed: Raspberry Pi firmware supports an explicit initramfs file plus address
  config command, including followkernel placement. The lab boot archive
  contract already permits boot files beyond the required kernel/config/DTB set,
  and the TFTP evidence endpoint can observe the initramfs fetch.
- fixed: The selected Pi 5 transport is an external generated-root artifact
  named initramfs_2712 in the boot archive, with matching root and da591740/
  serial-prefix copies, selected by initramfs initramfs_2712 followkernel in
  both config.txt files.
- fixed: Runtime source evidence must come from firmware-provided FDT /chosen
  initramfs bounds, not from the QEMU fixed loader address. The future candidate
  must parse linux,initrd-start and linux,initrd-end, check the range against
  the artifact parser limits, and then call the same generated-root selection
  path.
- removed: Reusing the QEMU fixed physical address 0x47000000 as a Pi 5
  hardware assumption. That address is a QEMU loader-device contract only.
- deferred: Writable persistence, SD/USB/block drivers, networking, SSH, and
  Phase 11 remain out of scope.
- not-an-issue: No archive publication, power-cycle, hardwareTestLock
  acquisition, QEMU rerun, or runtime code change is required for this contract
  task.

## Required Candidate Evidence

- Candidate archive path:
  target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz.
- Boot tree directory:
  target/rpi5-generated-root-boot-transport-candidate-20260605-boot-tree.
- Kernel files: kernel_2712.img and identical kernel8.img, plus mirrored
  da591740/kernel_2712.img and da591740/kernel8.img.
- Artifact files: initramfs_2712 and identical da591740/initramfs_2712.
- Config line in both config files:
  initramfs initramfs_2712 followkernel.
- Static archive review must record archive SHA-256, kernel SHA-256/size,
  artifact SHA-256/size, file listing, config line, and proof strings for
  rootinfo, /generated/manifest.txt, and /generated/status7.
- Hardware proof must capture candidate identity, fresh serial cursor, fresh
  TFTP cursor/delta, observed artifact fetch (initramfs_2712 or
  da591740/initramfs_2712), serial rootinfo showing source firmware-initramfs or
  equivalent accepted wording, generated file content, generated executable
  status, prompt/readiness, classification/PASS, and post-run restore evidence.

## Inconclusive-Run Triage

Before changing code after any inconclusive Pi 5 run, record:

1. Candidate identity: commit, git status, archive SHA-256, kernel SHA-256,
   artifact SHA-256, kernel size, artifact size, and archive-review output.
2. Fresh serial cursor proving the observe window starts after candidate
   publish and power cycle.
3. TFTP delta proving the Pi fetched the candidate kernel and generated-root
   artifact after candidate publish.
4. Known-good control through named snapshot/restore unless the failure is
   already a clear Talos proof failure with complete candidate fetch and serial
   evidence.
5. Candidate rerun with fresh serial and TFTP cursors before implementation
   changes.
