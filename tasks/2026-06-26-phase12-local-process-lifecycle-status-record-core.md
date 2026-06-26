# Phase 12 Local Process Lifecycle Status Record Core

Task id: phase12-local-process-lifecycle-status-record-core-20260626
Status: accepted
Owner: worker
Classification: local-process-lifecycle-status-record-core-accepted

## Goal

Implement the narrowest kernel-owned lifecycle/status record that carries the
accepted `/bin/init` VFS-backed launch zero-status result to shell-visible
output while Phase 12 live network reachability remains paused.

## Scope

This task is a local POSIX/VFS/userspace continuation only. It preserves the
accepted `/bin/init` descriptor-backed VFS/open/read, loader, process-install,
address-space, materialization, initial-stack, lower-AArch64 SVC
status-equivalent, `laststatus`, deterministic negative exec, and VFS cat
regression surfaces. It does not perform lab, Pi 5 hardware, boot publication,
packet I/O, ping, OpenSSH, remote receipt, compatibility, ssh-ready, or phase
transition work.

## Implementation

- Added `LocalCommandInitLifecycleStatusRecord` with identity
  `phase12-local-process-lifecycle-status-record-v1`.
- The record is emitted only when the accepted VFS exec path launches the
  single `/bin/init` fixture.
- The shell-visible output now includes:
  `talos: init-lifecycle-status record=phase12-local-process-lifecycle-status-record-v1 ... status=0x0000000000000000 observed-status=0x0000000000000000 ... source=kernel-owned-lifecycle-status-record`.
- Existing generic `exec-lifecycle`, `exec-status`, `laststatus`,
  `waitpid`, non-init exec, and negative exec behavior remain intact.
- Updated QEMU scenario expectations for the extra `/bin/init` response line.
- Updated roadmap/frontier docs to mark this local lifecycle/status record as
  accepted while keeping live network/SSH reachability paused.

## Findings And Disposition

- fixed: `exec /bin/init` now emits a named kernel-owned lifecycle/status
  record instead of relying only on the generic lifecycle line and
  `lower-aarch64-svc-status-equivalent` print.
- fixed: the named record carries the accepted zero status from the
  descriptor-backed `/bin/init` VFS/open/read, loader, launch, and SVC status
  lineage to shell-visible output.
- fixed: QEMU scenario response-count expectations now match the extra
  `/bin/init` lifecycle/status output.
- fixed: retained deterministic missing, relative, directory, non-executable,
  and empty-file exec failures; unknown targets still do not look successful.
- fixed: retained `cat /etc/banner.txt` through the descriptor-backed
  initramfs/VFS open/read path.
- not-an-issue: kernel-backed built-ins remain regression/control surfaces
  only; this task adds no fake shell command expansion.
- removed: a duplicate task-owned smoke wrapper was discarded during iteration
  after the existing VFS exec smoke proved the same feature path and retained
  better-established serial-driver behavior.
- deferred: broad process tables, process replacement, scheduler handoff,
  waitpid expansion, arbitrary executable dispatch, PATH lookup, argv/envp/auxv
  expansion, descriptor inheritance expansion, writable filesystem,
  networking, SSH, RP1/PCIe, DMA/cache policy, and Pi 5 hardware proof remain
  outside this task.

## Evidence

- QEMU/substitute focused shell lifecycle/status transcript:
  `tasks/evidence/2026-06-26-phase12-local-process-lifecycle-status-record-core/qemu-local-shell-vfs-exec-smoke.log`.
- The retained transcript proves:
  - `exec /bin/init` reads `/bin/init` through `source=vfs-open-read`.
  - loader, launch, descriptor inheritance, startup ABI, and initial-stack
    lineage are still printed before lifecycle/status output.
  - `init-lifecycle-status record=phase12-local-process-lifecycle-status-record-v1`
    reports zero status and observed status for `/bin/init`.
  - `laststatus` still reports the same `/bin/init` lifecycle identity.
  - `exec /missing`, `exec bin/init`, `exec /bin`,
    `exec /etc/banner.txt`, and `exec /empty` fail closed.
  - `cat /etc/banner.txt` still prints `Talos initramfs fixture`.
  - final classification is `qemu-local-shell-vfs-exec-complete` with PASS.
- Retained VFS cat regression also remains available at
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests/QEMU runner substitute: `cargo -Zjson-target-spec test --quiet`
  passed with 823 talos no_std tests.
- focused QEMU/substitute shell lifecycle/status smoke:
  `TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT=54547 ./scripts/qemu-local-shell-vfs-exec-smoke.sh`
  passed and retained the task-owned transcript above.
- retained VFS cat regression:
  `./scripts/qemu-local-cat-banner-smoke.sh` passed earlier in this task and
  the accepted VFS exec transcript also includes `cat /etc/banner.txt`.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed.
- whitespace checks: `git diff --check` passed.
- task-owned JSON evidence: conditional skip, no task-owned JSON evidence was
  created.

Evidence levels: fmt/lint/typecheck, unit tests through the QEMU runner,
QEMU/substitute shell smoke, QEMU/substitute VFS cat regression, docs build,
and diff checks.

## Acceptance

Accepted as local-process-lifecycle-status-record-core-accepted.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.
No boot archive was published, no hardwareTestLock was acquired, no packet I/O
or OpenSSH attempt ran, and no ssh-ready, remote-receipt, compatibility, or
phase-transition claim is accepted.

Acceptance commit: recorded in durable supervisor state after commit creation.
