# Phase 10 Shell Userspace Exit Status Closeout

Task: phase10-shell-userspace-exit-status-closeout-20260603

Status: accepted

## Scope

Documentation-only closeout for the accepted shell-visible VFS/userspace
execution and status frontier.

This checkpoint reconciles the accepted VFS-backed shell cat path, shell
exec /bin/init boundary, userspace status-equivalent SVC observation, retained
QEMU/substitute evidence, deferred lifecycle surfaces, and the next
supervisor-owned planning need.

No runtime code changed. No QEMU scenario was rerun for this checkpoint. No
Pi 5 hardware action, boot archive publication, power-cycle, or
hardwareTestLock acquisition was performed.

## Records

- Project closeout:
  docs/src/project/phase10-shell-userspace-exit-status-closeout-checkpoint.md.
- Accepted shell cat task:
  tasks/2026-06-03-phase10-shell-backed-by-userspace-and-vfs.md.
- Accepted shell cat commit:
  3ea7479e1728b0cee79502577917c852674c6d55.
- Accepted shell exec task:
  tasks/2026-06-03-phase10-shell-vfs-exec-boundary.md.
- Accepted shell exec commit:
  0e418a08c5ab3a5e2b9c67c9d48a007ca70d85cb.
- Accepted shell exec closeout commit:
  9f7abaee806698d5d378bc9a0c6649af8163893a.
- Accepted userspace status task:
  tasks/2026-06-03-phase10-shell-userspace-exit-status.md.
- Accepted userspace status commit:
  4c136c523cd410a0a43dfc3e986aaea17016930d.
- Retained shell cat evidence:
  tasks/evidence/2026-06-03-phase10-shell-backed-by-userspace-and-vfs/qemu-local-shell-vfs-cat-smoke.log.
- Retained shell exec evidence:
  tasks/evidence/2026-06-03-phase10-shell-vfs-exec-boundary/qemu-local-shell-vfs-exec-smoke.log.
- Retained userspace status evidence:
  tasks/evidence/2026-06-03-phase10-shell-userspace-exit-status/qemu-local-shell-userspace-status-smoke.log.
- Retained userspace status VFS cat regression:
  tasks/evidence/2026-06-03-phase10-shell-userspace-exit-status/qemu-local-cat-banner-regression.log.
- Retained lower-EL userspace launch evidence:
  tasks/evidence/2026-06-03-qemu-initial-userspace-process-launch/qemu-initial-userspace-process-launch-smoke.log.

## Findings And Dispositions

- fixed: The accepted shell cat task made `cat /etc/banner.txt` read through
  the descriptor-backed initramfs/VFS `TalosOpen`/`TalosRead` path rather than
  expanding a fake command fixture.
- fixed: The accepted shell exec task made `exec /bin/init` source bytes from
  `/bin/init` through VFS/open/read before program loading, process install,
  address-space planning, descriptor-image materialization, initial-stack
  planning, and launch-boundary reporting.
- fixed: The accepted userspace status task made `/bin/init` put a
  deterministic zero status in `x0` before `svc #0x7a10`; the QEMU lower-EL
  launch smoke observes the real exception frame with `x0=0`, and the shell
  transcript reports the matching `lower-aarch64-svc-status-equivalent`
  status.
- fixed: Negative exec paths remain deterministic: `exec /missing` reports
  `exec-not-found`, and `exec /etc/banner.txt` reports
  `exec-not-executable`.
- fixed: Existing VFS cat and shell exec QEMU/substitute regressions remained
  green after the status boundary landed.
- not-an-issue: Kernel-backed built-ins still appear in help/status. They are
  retained regression/control surfaces and were not counted as new OS
  progress.
- not-an-issue: Pi 5 hardware was not run. This closeout accepts only the
  QEMU/substitute and lower-EL launch evidence already retained by the bounded
  implementation tasks; no new physical board behavior is claimed.
- deferred: A general process table, scheduler-visible process lifetime,
  wait/waitpid, real exit teardown, signals, job control, foreground process
  groups, argv/envp/auxv/TLS, PATH lookup, external command search, pipes,
  redirection, writable filesystem, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache policy remain outside the accepted frontier.

## Accepted Frontier

The accepted shell-visible behavior now backed by real VFS/userspace layers is:

- `cat /etc/banner.txt` opens and reads the initramfs file through the accepted
  descriptor-backed VFS/open/read path and returns the file bytes to the shell
  transcript.
- `exec /bin/init` opens and reads `/bin/init` through the same accepted file
  path, parses the ELF with the program loader, derives process install,
  address-space, materialization, initial-stack, and initial launch records,
  and reports the lower-AArch64 SVC launch-boundary-equivalent signal.
- `/bin/init` now carries a deterministic zero status value immediately before
  its accepted SVC marker. The lower-EL launch smoke observes that value in
  the exception frame, and shell-visible `exec /bin/init` reports the matching
  status-equivalent completion observation.

The accepted shell output distinguishes the userspace/SVC status-equivalent
observation from retained kernel built-ins by printing:

- `source=vfs-open-read`;
- `/bin/init` source bytes and digest `0x96ee5866736d445b`;
- loader, launch, stack, address-space, and materialization lineage;
- `exec-status boundary=lower-aarch64-svc-status-equivalent`;
- `marker=0x0000000000007a10`;
- `status=0x0000000000000000 complete=true`.

This is still deliberately narrower than Unix process execution. It is not
execve, process replacement, scheduler-owned process lifetime, waitpid,
descriptor inheritance across exec, PATH lookup, an argv/envp ABI, an
interactive userspace shell, or arbitrary program execution from the
filesystem.

## Deferred Work And Risks

Remaining gaps after this checkpoint:

- process lifecycle: no scheduler-owned task, process table entry, wait queue,
  exit teardown, zombie/reap state, or parent-child relationship has been
  accepted.
- syscall surface: the status observation is SVC/status-equivalent evidence,
  not a general exit syscall ABI or wait syscall ABI.
- user ABI: argv/envp/auxv/TLS, process-local cwd, descriptor inheritance, and
  executable permission policy remain unimplemented.
- shell semantics: PATH lookup, external command dispatch beyond the explicit
  `/bin/init` fixture, quoting, globbing, pipes, redirection, jobs, terminal
  sessions, termios, and signals remain deferred.
- filesystem: writable files, mounts, symlinks, broad normalization, mutable
  metadata, and arbitrary executable files remain deferred.
- hardware: Pi 5 proof for this shell/userspace status path is deferred until
  a later task states a smallest physical claim that QEMU/substitute evidence
  cannot answer.
- platform work: networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain later-phase work.

## Next Planning Handoff

The next feature-led local execution capability should be supervisor-planned,
because no later explicit queued task exists in durable state after this
closeout. The evidence points to a narrow process lifecycle or wait/status
boundary as the next plausible slice: make the launched `/bin/init` status
flow through an explicit kernel-owned lifecycle record that the shell can
observe, without broadening into PATH lookup, argv/envp, pipes, redirection,
writable filesystem, networking, SSH, or Pi 5 proof.

The worker should not invent that task. The supervisor should decompose it
with explicit dependencies, acceptance criteria, validation gates, docs, and
evidence requirements before another worker promotes work.

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
