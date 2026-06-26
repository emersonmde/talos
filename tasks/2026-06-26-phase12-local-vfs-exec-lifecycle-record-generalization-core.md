# Phase 12 Local VFS Exec Lifecycle Record Generalization Core

Task id: phase12-local-vfs-exec-lifecycle-record-generalization-core-20260626
Status: accepted
Owner: worker
Classification: local-vfs-exec-lifecycle-record-generalization-core-accepted

## Goal

Generalize the accepted kernel-owned lifecycle/status record from the single
`/bin/init` fixture into a path-aware direct VFS exec lifecycle/status surface
for the existing `/bin/init`, `/bin/zero`, and `/bin/status42` fixtures while
Phase 12 live network reachability remains paused.

## Scope

This task is a local POSIX/VFS/userspace continuation only. It preserves
accepted descriptor-backed VFS/open/read, loader, initial-stack, descriptor
inheritance, `waitpid`, `laststatus`, deterministic negative exec, and
`cat /etc/banner.txt` behavior. It does not perform lab, Pi 5 hardware, boot
publication, packet I/O, ping, OpenSSH, remote receipt, compatibility,
ssh-ready, pipeline behavior expansion, or phase-transition work.

## Implementation

- Added `LocalCommandVfsExecLifecycleStatusRecord` with identity
  `phase12-local-vfs-exec-lifecycle-status-record-v2`.
- The v2 record is emitted for direct VFS exec lifecycle records whose path is
  exactly `/bin/init`, `/bin/zero`, or `/bin/status42`.
- Preserved the accepted `init-lifecycle-status
  record=phase12-local-process-lifecycle-status-record-v1` line for
  `/bin/init` as a regression control.
- Updated local unit expectations, QEMU-side expected dispatch counts, and the
  focused serial smoke assertions for the extra foreground lifecycle/status
  line.
- Updated the VFS exec smoke wrapper so callers can override the evidence
  directory/log path for task-owned retained transcripts.

## Findings And Disposition

- fixed: `exec /bin/zero` now emits the same versioned
  `vfs-exec-lifecycle-status` surface as `/bin/init`, with path-aware zero
  status.
- fixed: `exec /bin/status42` now emits the versioned
  `vfs-exec-lifecycle-status` surface with path-aware status
  `0x000000000000002a`.
- fixed: the accepted `/bin/init` v1 line remains present while the new v2
  record carries the same zero-status lifecycle through the generalized surface.
- fixed: QEMU scenario accounting now expects the extra foreground record for
  direct VFS exec fixtures; background job summaries remain on their existing
  accounting path.
- fixed: task-owned smoke evidence can now be retained through environment
  overrides instead of being forced into the older Phase 10 evidence directory.
- not-an-issue: kernel-backed built-ins remain regression/control surfaces only;
  no new fake command is counted as operating-system progress.
- deferred: broad process tables, fork, async jobs, signals, job control,
  PATH/environment expansion, arbitrary descriptor syntax, persistent
  filesystem semantics, networking, SSH, RP1/PCIe, DMA/cache policy, pipeline
  behavior expansion, and Pi 5 hardware proof remain outside this task.
- removed: no code path was removed; the older init-only record remains as a
  regression control.

## Evidence

- QEMU/substitute focused shell lifecycle/status transcript:
  `tasks/evidence/2026-06-26-phase12-local-vfs-exec-lifecycle-record-generalization-core/qemu-local-shell-vfs-exec-smoke.log`.
- The retained transcript proves:
  - `exec /bin/status42` reads through `source=vfs-open-read`, emits
    `vfs-exec-lifecycle-status record=phase12-local-vfs-exec-lifecycle-status-record-v2`,
    and reports status/observed-status `0x000000000000002a`.
  - `exec /bin/init` still emits the accepted
    `init-lifecycle-status record=phase12-local-process-lifecycle-status-record-v1`
    plus the new v2 generalized record with zero status.
  - `exec /bin/zero` emits the new v2 generalized record with zero status.
  - `laststatus` remains consistent for `/bin/status42`, `/bin/init`, and
    `/bin/zero`.
  - `exec /missing`, `exec bin/init`, `exec /bin`,
    `exec /etc/banner.txt`, and `exec /empty` fail closed.
  - `cat /etc/banner.txt` still prints `Talos initramfs fixture`.
  - final classification is `qemu-local-shell-vfs-exec-complete` with PASS.
- Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests/QEMU runner substitute: `cargo -Zjson-target-spec test --quiet`
  passed.
- focused QEMU/substitute shell lifecycle/status smoke:
  `TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT=54548 TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR=tasks/evidence/2026-06-26-phase12-local-vfs-exec-lifecycle-record-generalization-core TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG=tasks/evidence/2026-06-26-phase12-local-vfs-exec-lifecycle-record-generalization-core/qemu-local-shell-vfs-exec-smoke.log ./scripts/qemu-local-shell-vfs-exec-smoke.sh`
  passed and retained the task-owned transcript above.
- retained VFS cat regression: covered in the focused VFS exec transcript by
  `cat /etc/banner.txt` through descriptor-backed initramfs/VFS open/read.
- task-owned JSON evidence: conditional skip, no task-owned JSON evidence was
  created.
- docs validation, whitespace checks, and commit hash are recorded after final
  validation/commit.

Evidence levels: fmt/lint/typecheck, unit tests through the QEMU runner,
QEMU/substitute shell smoke, QEMU/substitute VFS cat regression, docs build, and
diff checks.

## Acceptance

Accepted as local-vfs-exec-lifecycle-record-generalization-core-accepted.

No boot archive was published, no hardwareTestLock was acquired, no lab or Pi 5
hardware action ran, no packet I/O or OpenSSH attempt ran, and no ssh-ready,
remote-receipt, compatibility, live reachability, or phase-transition claim is
accepted.

Acceptance commit: recorded in durable supervisor state after commit creation.
