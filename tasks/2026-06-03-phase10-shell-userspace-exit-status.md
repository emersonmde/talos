# Phase 10 Shell Userspace Exit Status

Task: phase10-shell-userspace-exit-status-20260603

Status: accepted

## Goal

Expose the narrowest deterministic userspace completion/status observation for
the shell-visible VFS-backed `/bin/init` exec path.

## Changed Files

- src/initramfs.rs
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- src/program_loader.rs
- src/process_install.rs
- src/process_address_space.rs
- src/process_page_table_materialization.rs
- scripts/qemu-local-serial-command-loop-smoke.sh
- scripts/qemu-initial-userspace-process-launch-smoke.sh
- docs/src/roadmap.md
- tasks/evidence/2026-06-03-phase10-shell-userspace-exit-status/

## Findings And Dispositions

- fixed: `/bin/init` reached the accepted SVC launch boundary without a
  deterministic completion/status value. The fixture now executes
  `movz x0, #0` immediately before `svc #0x7a10`, preserving the same
  516-byte ELF file length while expanding the text segment to eight bytes.
- fixed: The shell-visible exec output did not distinguish userspace
  completion/status from kernel builtin success output. `exec /bin/init` now
  reports `talos: exec-status boundary=lower-aarch64-svc-status-equivalent
  marker=0x0000000000007a10 status=0x0000000000000000 complete=true` after
  reading the program bytes through the VFS/open/read path and decoding the
  status/SVC instruction pair from those bytes.
- fixed: Model-layer fixture expectations still assumed a four-byte text
  segment and stale digest. Updated loader, process-install, address-space, and
  materialization tests to the new text copy/zero accounting and digest
  `0x96ee5866736d445b`.
- fixed: The lower-EL userspace launch smoke now asserts the SVC frame carries
  `x0=0` and records `status=0x0000000000000000 complete=true`.
- not-an-issue: Existing kernel-backed builtins remain regression/control
  surfaces only; no new fake command expansion was added.
- deferred: This is not a general process lifecycle, wait, process table,
  argv/envp, PATH, signal, pipe, redirection, writable filesystem, or hardware
  proof implementation.

## Accepted Boundary

The accepted status boundary is:

- `/bin/init` remains the VFS-backed initramfs executable at `/bin/init`;
- the file is read through descriptor-backed `TalosOpen`/`TalosRead` before
  shell exec reporting;
- the program's text begins with an explicit status-producing instruction
  followed by the accepted SVC marker;
- QEMU lower-EL launch evidence observes the real exception frame with
  `x0=0`, `marker=0x7a10`, and `elr=0x0000000000010108`;
- shell-visible `exec /bin/init` reports the same zero status as a
  userspace/SVC status-equivalent observation;
- missing and non-executable exec targets remain deterministic negative cases;
- VFS-backed `cat /etc/banner.txt` still passes as a regression.

## Evidence

- Source/unit evidence: `cargo -Zjson-target-spec test --quiet` passed
  repo-wide with 378 no_std tests.
- QEMU/substitute shell userspace status transcript:
  `tasks/evidence/2026-06-03-phase10-shell-userspace-exit-status/qemu-local-shell-userspace-status-smoke.log`
  with the `exec-status` line and `qemu-local-shell-vfs-exec: PASS`.
- QEMU/substitute VFS cat regression:
  `tasks/evidence/2026-06-03-phase10-shell-userspace-exit-status/qemu-local-cat-banner-regression.log`
  with `qemu-local-cat-banner: PASS`.
- QEMU/substitute lower-EL userspace launch evidence:
  `tasks/evidence/2026-06-03-qemu-initial-userspace-process-launch/qemu-initial-userspace-process-launch-smoke.log`
  with `frame available=true x0=0x0000000000000000`, status zero, and
  `qemu-initial-userspace-process-launch-smoke: PASS`.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- cargo -Zjson-target-spec check --quiet: passed.
- QEMU/substitute shell userspace status smoke: passed.
- QEMU/substitute VFS cat regression: passed.
- QEMU/substitute initial userspace launch smoke: passed.
- mdbook build: passed.
- git diff --check: passed.
- git diff --cached --check: passed.

## Non-Goals Preserved

No general process table, waitpid, signals, job control, scheduling policy,
argv/envp/auxv/TLS, PATH lookup, pipes, redirection, writable filesystem,
networking, SSH, RP1/PCIe, DMA/cache policy, Pi 5 hardware proof, or hardware
lock usage was accepted.
