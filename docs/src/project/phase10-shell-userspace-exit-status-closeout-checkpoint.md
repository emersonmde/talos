# Phase 10 Shell Userspace Exit Status Closeout Checkpoint

Status: accepted

Task: phase10-shell-userspace-exit-status-closeout-20260603

## Scope

This checkpoint closes out the first shell-visible VFS/userspace execution
status frontier. It reconciles the accepted shell cat, shell exec, and
userspace status tasks; retained QEMU/substitute and lower-EL launch evidence;
deterministic negative exec behavior; preserved regressions; deferred process
lifecycle surfaces; and the next supervisor-owned planning handoff.

No runtime code changed. No QEMU scenario was rerun by this checkpoint. No
Pi 5 hardware action, boot archive publication, power-cycle, or
hardwareTestLock acquisition was performed.

## Reviewed Evidence

- accepted shell cat task:
  tasks/2026-06-03-phase10-shell-backed-by-userspace-and-vfs.md.
- accepted shell cat commit:
  3ea7479e1728b0cee79502577917c852674c6d55.
- retained shell cat transcript:
  tasks/evidence/2026-06-03-phase10-shell-backed-by-userspace-and-vfs/qemu-local-shell-vfs-cat-smoke.log.
- accepted shell exec task:
  tasks/2026-06-03-phase10-shell-vfs-exec-boundary.md.
- accepted shell exec commit:
  0e418a08c5ab3a5e2b9c67c9d48a007ca70d85cb.
- retained shell exec transcript:
  tasks/evidence/2026-06-03-phase10-shell-vfs-exec-boundary/qemu-local-shell-vfs-exec-smoke.log.
- accepted userspace status task:
  tasks/2026-06-03-phase10-shell-userspace-exit-status.md.
- accepted userspace status commit:
  4c136c523cd410a0a43dfc3e986aaea17016930d.
- retained shell userspace status transcript:
  tasks/evidence/2026-06-03-phase10-shell-userspace-exit-status/qemu-local-shell-userspace-status-smoke.log.
- retained post-status VFS cat regression:
  tasks/evidence/2026-06-03-phase10-shell-userspace-exit-status/qemu-local-cat-banner-regression.log.
- retained lower-EL launch transcript:
  tasks/evidence/2026-06-03-qemu-initial-userspace-process-launch/qemu-initial-userspace-process-launch-smoke.log.

The retained userspace status transcript proves:

- `help`, `status`, and `stdio` still expose the current descriptor-backed
  command surface.
- `exec /bin/init` reads source bytes through VFS/open/read and reports source
  byte count plus digest `0x96ee5866736d445b`.
- the shell output includes loader, launch, stack, address-space, and
  materialization lineage.
- the shell output includes
  `exec-status boundary=lower-aarch64-svc-status-equivalent
  marker=0x0000000000007a10 status=0x0000000000000000 complete=true`.
- the lower-EL launch transcript observes the real exception frame with
  `x0=0x0000000000000000`, marker `0x7a10`, and `complete=true`.
- `exec /missing` and `exec /etc/banner.txt` remain deterministic negative
  cases.
- `cat /etc/banner.txt` still prints `Talos initramfs fixture` through the
  VFS-backed path.
- the transcript returns to a ready prompt and ends with
  `qemu-local-shell-vfs-exec-complete` and
  `qemu-local-shell-vfs-exec: PASS`.

The retained post-status VFS cat regression also proves `cat /etc/banner.txt`,
`ls /bin`, empty-command, unknown-command, and ready-prompt behavior still
pass after the status boundary landed.

## Findings And Dispositions

- fixed: The accepted shell cat task made the shell-visible file operation
  consume the descriptor-backed initramfs/VFS `TalosOpen`/`TalosRead` path.
- fixed: The accepted shell exec task made `exec /bin/init` consume `/bin/init`
  through VFS/open/read before loader, process-install, address-space,
  materialization, initial-stack, and launch-boundary records.
- fixed: The accepted userspace status task added a deterministic zero status
  before the `/bin/init` SVC marker and surfaced that status through both
  lower-EL launch evidence and shell-visible `exec /bin/init` output.
- fixed: Missing and non-executable exec targets remain deterministic:
  `exec-not-found` and `exec-not-executable`.
- fixed: Existing VFS cat and shell exec QEMU/substitute regressions remained
  passing after the status handoff.
- not-an-issue: Kernel-backed built-ins remain visible as regression/control
  surfaces. They were not expanded or counted as new OS progress.
- not-an-issue: Hardware was unused because this checkpoint makes no new
  physical Pi 5 claim.
- deferred: General process lifecycle, wait/waitpid, process tables, real
  exit teardown, signals, job control, argv/envp/auxv/TLS, PATH lookup,
  descriptor inheritance, pipes, redirection, writable filesystem, hardware
  proof, networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache
  policy remain outside the accepted frontier.

## Accepted Frontier

Talos now has QEMU/substitute-proven shell-visible behavior backed by real
VFS/userspace layers:

- `cat /etc/banner.txt` reads file bytes through descriptor-backed
  initramfs/VFS open/read.
- `exec /bin/init` reads executable bytes through the same file path, then
  derives the accepted loader, process install, address-space,
  materialization, initial-stack, and initial launch records.
- `/bin/init` produces a deterministic zero status before the accepted
  lower-AArch64 SVC marker, the lower-EL launch smoke observes that status in
  the exception frame, and shell-visible `exec /bin/init` reports the matching
  status-equivalent completion observation.

This is narrower than Unix process execution. The shell observes a
status-equivalent lower-EL/SVC boundary for a VFS-backed fixture. It does not
yet own a real process lifecycle, process replacement model, wait/reap
protocol, external command lookup, argv/envp ABI, descriptor inheritance, or
arbitrary executable-file policy.

## Deferred Surfaces

Still deferred after this checkpoint:

- scheduler-owned process lifecycle, process table entries, exit teardown,
  zombie/reap state, parent-child status handoff, wait, and waitpid.
- a general exit syscall ABI or wait syscall ABI beyond the accepted
  SVC/status-equivalent observation.
- argv/envp/auxv/TLS, descriptor inheritance across exec, process-local cwd,
  and executable permission policy.
- PATH lookup, external command dispatch beyond explicit `/bin/init`, quoting,
  globbing, pipes, redirection, foreground/background jobs, job control,
  terminal sessions, termios, signals, and shell history.
- writable filesystem state, mounts, symlinks, broad path normalization, and
  mutable file metadata.
- Pi 5 proof until a later task records a smallest physical claim not answered
  by QEMU/substitute evidence.
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Next Planning Handoff

Update: the originally deferred narrow handoff below was later accepted by
`phase12-local-process-lifecycle-status-record-core-20260626`, while live
Phase 12 network reachability stayed paused. That continuation preserved the
accepted `/bin/init` VFS/open/read, loader, launch, and zero-status SVC
evidence, then added shell-visible
`init-lifecycle-status record=phase12-local-process-lifecycle-status-record-v1`
as a named kernel-owned lifecycle/status record for the single accepted
`/bin/init` fixture path. General process tables, waitpid expansion, process
replacement, PATH lookup, arbitrary executable dispatch, writable filesystem,
networking, SSH, and Pi 5 hardware claims remain outside that continuation.

No later explicit queued task exists after this closeout. The worker should
therefore stop after acceptance and set durable state for supervisor planning.

The then-next plausible feature-led slice was a narrow kernel-owned process
lifecycle or wait/status boundary: preserve the accepted `/bin/init` VFS read,
launch, and zero-status SVC evidence, then make that status flow through an
explicit lifecycle record the shell can observe. That task should be planned by
the supervisor with explicit scope, dependencies, acceptance criteria,
validation gates, docs, and evidence requirements before a worker promotes it.

This handoff should not broaden into PATH lookup, argv/envp, pipes,
redirection, writable filesystem, networking, SSH, RP1/PCIe, DMA/cache policy,
or Pi 5 proof unless a later supervisor-owned task explicitly scopes those
surfaces.

## Validation

- static inspection: accepted shell cat, shell exec, exec closeout, userspace
  status, retained shell transcripts, retained VFS cat regressions, retained
  lower-EL launch evidence, and recent commits were inspected.
- static inspection: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
