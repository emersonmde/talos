# Phase 10 Shell VFS Exec Closeout Checkpoint

Status: accepted

Task: phase10-shell-vfs-exec-closeout-checkpoint-20260603

## Scope

This checkpoint closes out the first shell-visible execution boundary backed
by the accepted VFS, descriptor, loader, userspace-launch, and stack-planning
layers. It reconciles the accepted implementation task, retained
QEMU/substitute evidence, deterministic negative cases, preserved regressions,
deferred lifecycle surfaces, and the next feature-led handoff.

No runtime code changed. No QEMU scenario was rerun. No Pi 5 hardware action,
boot archive publication, power-cycle, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- accepted shell VFS exec boundary task:
  tasks/2026-06-03-phase10-shell-vfs-exec-boundary.md.
- accepted implementation commit:
  0e418a08c5ab3a5e2b9c67c9d48a007ca70d85cb.
- retained QEMU/substitute shell exec transcript:
  tasks/evidence/2026-06-03-phase10-shell-vfs-exec-boundary/qemu-local-shell-vfs-exec-smoke.log.
- retained QEMU/substitute VFS cat regression:
  tasks/evidence/2026-06-03-phase10-shell-vfs-exec-boundary/qemu-local-cat-banner-regression.log.
- source/unit evidence recorded by the implementation task:
  cargo -Zjson-target-spec test --quiet passed repo-wide with focused shell
  exec positive and negative tests.
- implementation validation recorded by the implementation task:
  cargo fmt --all -- --check,
  cargo -Zjson-target-spec check --quiet,
  cargo -Zjson-target-spec test --quiet,
  scripts/qemu-local-shell-vfs-exec-smoke.sh --quiet,
  scripts/qemu-local-cat-banner-smoke.sh --quiet with task-local evidence
  overrides, git diff --check, mdbook build, and git diff --cached --check
  all passed.

The retained shell exec transcript proves:

- help and status expose exec as part of the current command surface.
- stdio still reports descriptor-backed fd0/fd1/fd2 and runtime-console0
  backing.
- exec /bin/init prints source=vfs-open-read, source byte count and digest,
  program-loader fixture identity, entry, segment count, process launch
  boundary, stack boundary, address-space id, materialization id, initial stack
  pointer, and the explicit lower-aarch64-svc-launch-boundary-equivalent
  signal.
- exec /missing returns deterministic exec-not-found.
- exec /etc/banner.txt returns deterministic exec-not-executable.
- cat /etc/banner.txt remains a VFS-backed regression and prints
  Talos initramfs fixture.
- the run returns to a ready prompt and ends with
  qemu-local-shell-vfs-exec-complete plus exact PASS line
  qemu-local-shell-vfs-exec: PASS.

The retained VFS cat regression also proves cat /etc/banner.txt, ls /bin,
empty-command, unknown-command, and ready-prompt behavior still pass after the
exec boundary landed.

## Findings And Dispositions

- fixed: The accepted implementation task made a shell-visible exec /bin/init
  request consume /bin/init through the descriptor-backed TalosOpen/TalosRead
  path before program loading, process installation, address-space planning,
  materialization planning, initial-stack planning, and launch-boundary
  observation.
- fixed: Missing and non-executable exec targets now have deterministic
  negative responses instead of looking like successful or generic fake command
  output.
- fixed: Task-local QEMU/substitute shell exec evidence and a VFS cat
  regression were retained under the accepted evidence directory.
- not-an-issue: Existing kernel-backed built-ins still appear in help/status.
  They remain regression/control surfaces and were not counted as new
  operating-system progress.
- not-an-issue: Pi 5 hardware was not run. The accepted claim is
  QEMU/substitute-level shell-visible consumption of already accepted generic
  VFS/userspace launch layers; no new physical board behavior was claimed.
- deferred: Process lifecycle and completion/status handoff remain absent. The
  shell observes a launch-boundary-equivalent signal, not process exit.
- deferred: argv/envp/auxv/TLS, PATH lookup, external command dispatch,
  descriptor inheritance across exec, general process table, wait/exit,
  writable filesystem, pipes, redirection, networking, SSH, RP1/PCIe, and
  DMA/cache policy remain out of scope.

## Accepted Frontier

The accepted capability is a shell-visible exec /bin/init boundary backed by
real initramfs/VFS descriptor reads and the accepted userspace launch planning
chain. The shell can request /bin/init, source the bytes through
TalosOpen/TalosRead, parse the static ELF fixture with the program loader,
derive process-install/address-space/materialization/initial-stack/launch
records, print enough lineage to prove the VFS source path and launch boundary,
and return to the prompt.

This is not yet a full execve, process replacement, process lifecycle, status
propagation, scheduler handoff, external command lookup, PATH search, argv/envp
ABI, descriptor inheritance, or interactive userspace shell. The existing
built-ins remain deterministic kernel-side controls unless explicitly backed by
accepted VFS, descriptor, syscall, or userspace layers.

## Deferred Surfaces

Still deferred after this checkpoint:

- userspace completion/status handoff, wait/exit lifecycle, process table,
  scheduler integration for launched programs, and shell observation of real
  process completion.
- general argv/envp/auxv/TLS, initial userspace ABI expansion, descriptor
  inheritance across exec, current-directory inheritance, and process-local
  filesystem context.
- PATH lookup, external command dispatch, executable permission policy beyond
  the current deterministic /bin/init fixture boundary, and arbitrary
  filesystem-backed program execution.
- pipes, redirection, foreground/background jobs, job control, terminal
  sessions, termios, signals, and shell history.
- writable filesystem state, mounts, symlinks, broad path normalization, and
  mutable file metadata.
- Pi 5 proof for this exec boundary until a later task records a smallest
  physical claim that QEMU/substitute evidence cannot answer.
- networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Next Task Handoff

The next mechanically dependency-based task is
phase10-shell-userspace-exit-status-20260603, after this closeout is accepted
and committed. It should implement the narrowest userspace status/completion
observation tied to the accepted lower-EL/SVC or equivalent launch-boundary
mechanism, while retaining VFS cat and shell exec regressions.

This handoff should not broaden into process tables, waitpid, job control,
PATH lookup, pipes, redirection, writable filesystems, networking, SSH, or
hardware proof unless a later supervisor-owned task explicitly scopes those
surfaces.

## Validation

- static inspection: accepted shell exec task record, implementation commit,
  retained QEMU/substitute shell exec transcript, and retained VFS cat
  regression were inspected.
- static inspection: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
