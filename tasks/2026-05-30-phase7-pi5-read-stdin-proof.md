# Phase 7 Pi 5 Read And Stdin Proof

Task: phase7-pi5-read-stdin-proof-20260530
Status: accepted

## Scope

This task carries the accepted fixed-stdin talos_read invariant to serialized
Raspberry Pi 5 hardware evidence. The implementation adds only the focused
rpi5_read_stdin_proof boot scenario, image and boot-tree helpers, fixed
ProcessDescriptorStore-backed stdin proof state, and retained lab evidence for
the physical proof.

It does not add runtime-console0/TTY/hardware stdin, pipes, sockets, regular
files, filesystem reads, process loading, shell behavior, networking, SSH,
object finalization, RP1/PCIe work, UART interrupt ownership,
DMA/cache-driver policy, or full POSIX descriptor readiness.

## Implementation Evidence

- fmt/lint: cargo fmt --all -- --check passed before hardware.
- unit tests: cargo -Zjson-target-spec test passed before hardware.
- QEMU/substitute: scripts/qemu-read-stdin-smoke.sh passed before hardware.
- QEMU/substitute regressions: scripts/qemu-syscall-smoke.sh,
  scripts/qemu-descriptor-write-smoke.sh, scripts/qemu-close-syscall-smoke.sh,
  and scripts/qemu-dup-syscall-smoke.sh passed before hardware.

## Hardware Evidence

Evidence directory:
tasks/evidence/2026-05-30-pi5-read-stdin-proof/.

- local1-candidate: serial hardware boot/output inconclusive. The candidate
  archive was built from implementation commit
  fd2be8ea42ddf88dd4cff120439ab1d3df51bce1, published under
  hardwareTestLock, and restored afterward, but the serial transcript did not
  reach rpi5-read-stdin-proof markers.
- local2-known-good-control: serial hardware boot/output accepted-control. The
  restored production-timer preemption image printed
  pi5-production-timer-preemption-complete and PASS, confirming the lab serial,
  power-cycle, and restored boot tree path were still healthy.
- local3-unchanged-candidate-rerun: hardware automation/evidence
  inconclusive. The unchanged fd2be8e candidate was republished after the
  accepted control, and post-publish status showed the 114816-byte candidate in
  the boot tree. Follow-up inspection found the retained
  tftp-delta-before-restore.json only contains 104136-byte restored-control
  kernel fetches, so local3 did not prove that the candidate image was served.
  Serial captured only a NUL/newline and no rpi5-read-stdin-proof markers. The
  pre-run snapshot pre-pi5-read-stdin-proof-local3-20260530 was restored
  afterward; the restored kernel_2712.img size is 104136 bytes.
- local4/local6 corrected rerun attempts: hardware automation/evidence
  inconclusive. These attempts were bounded to re-running the unchanged
  fd2be8e candidate after local3 inspection, but the collection scripts exited
  early and restore handling won the race before the Pi fetched the candidate.
  Retained local6 TFTP evidence contains only 104136-byte restored-control
  kernel fetches, and post-restore status shows the accepted 104136-byte boot
  tree restored. No source changes were made from these attempts.
- local5-candidate-tftp-wait-rerun: serial hardware boot/output accepted. The
  unchanged fd2be8e candidate archive
  target/talos-rpi5-read-stdin-proof-local5-boot.tar.gz has SHA-256
  5f91281b2dcdfb1bca6fddd6dde6c3f0b39d89f4a4274a5bf91127d8ba833983.
  Static archive review recorded kernel_size=114816,
  header_image_size=114816, and matching kernel_2712.img/kernel8.img digest
  1b7417340d4b0dc44e741683464900500667929c2089b4c1ea88dc050f06d014.
  Retained TFTP delta shows da591740/kernel_2712.img served to
  10.42.1.4/88:a2:9e:ae:c8:7f with 114816 bytes before restore. Retained
  serial evidence contains fd 0 read, duplicated fd 3 short read, EOF,
  -EFAULT/-EINVAL/-EBADF error cases, talos_nop and unknown-syscall
  regressions, proof-only copy-probe quarantine, diagnostic-marker quarantine,
  final classification=pi5-read-stdin-proof-complete, and PASS. The
  post-snapshot restore status shows the prior accepted 104136-byte
  kernel_2712.img/kernel8.img boot tree restored.

## Acceptance

Accepted for the bounded Raspberry Pi 5 fixed-stdin talos_read proof. The
accepted physical claim is limited to the focused rpi5_read_stdin_proof
scenario: inherited fd 0 and duplicated fd 3 share the fixed proof stdin buffer,
copy-out faults preserve cursor and user memory, invalid/reserved/error cases
return the expected errno values, scalar syscall regressions still route, and
proof-only diagnostic surfaces remain quarantined.

Validation levels retained for acceptance:

- fmt/lint/typecheck: cargo fmt --all -- --check.
- unit tests: cargo -Zjson-target-spec test.
- QEMU/substitute: scripts/qemu-read-stdin-smoke.sh.
- QEMU/substitute regressions: scripts/qemu-syscall-smoke.sh,
  scripts/qemu-descriptor-write-smoke.sh, scripts/qemu-close-syscall-smoke.sh,
  and scripts/qemu-dup-syscall-smoke.sh.
- image/archive inspection: local5 archive review and digest files.
- lab-controller API and TFTP delta: local5 candidate archive publication and
  da591740/kernel_2712.img 114816-byte fetch.
- serial hardware boot/output: local5 proof-lines.txt.
- restore proof: local5 post-snapshot-restore-status.json.
- documentation/whitespace gates: git diff --check, mdbook build, and
  git diff --cached --check before acceptance commit.

Deferred surfaces remain blocked: runtime-console0/TTY/hardware stdin, pipes,
sockets, regular files, filesystem reads, process loading, shell behavior,
networking, SSH, object finalization, RP1/PCIe work, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor readiness.
