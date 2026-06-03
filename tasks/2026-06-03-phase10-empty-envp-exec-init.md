# Phase 10 Empty Envp Exec Init

Task: phase10-empty-envp-exec-init-20260603

Status: accepted

## Scope

Extend the same explicit VFS-backed `exec /bin/init` startup ABI path with
a deterministic empty environment record. This keeps the accepted
`argc=1` and `argv[0]=/bin/init` path and does not add environment
variables, PATH lookup, arbitrary executable dispatch, auxv, TLS, libc
startup, descriptor inheritance, wait/waitpid, pipes, redirection, writable
filesystem, networking, SSH, Pi 5 proof, or process replacement.

## Findings And Dispositions

- fixed: The initial user stack startup payload now records
  `minimal-argc1-argv0-init-empty-envp`, preserving `argc=1` and
  `argv[0]=/bin/init` while adding an explicit `empty-envp0` state,
  `envp-entries=0`, and an envp NULL-slot user pointer.
- fixed: The copied startup payload now accounts for argc, argv[0], argv
  NULL, envp NULL, and the argv0 C string. The copied byte count is now
  `0x2a`; the aligned initial SP remains inside the accepted stack range.
- fixed: Shell-visible `exec /bin/init` now reports empty-envp evidence on
  the same `source=initial-user-stack-record` line as the argc/argv state.
- fixed: QEMU/substitute smoke expectations now require the empty-envp ABI
  fields and retain a task-specific empty-envp evidence log.
- fixed: The initial-user-stack smoke expectations now match the explicit
  empty-envp payload accounting and pointers.
- fixed: Descriptor-backed VFS cat and lifecycle/`laststatus` regressions
  remained passing with the expanded startup ABI record.
- not-an-issue: `envp-null=true` remains present because the accepted
  empty environment is represented as envp[0] == NULL, not as user-controlled
  environment variables.
- deferred: environment variables, PATH lookup, arbitrary executable
  dispatch, auxv, TLS, libc startup, descriptor inheritance, wait/waitpid,
  asynchronous execution, multiple children, process replacement, pipes,
  redirection, writable filesystem, hardware proof, networking, and SSH
  remain outside the accepted frontier.

## Accepted Frontier

The accepted startup ABI behavior is still intentionally narrow:

- `exec /bin/init` opens and reads `/bin/init` through the descriptor-backed
  VFS/open/read path before program loading.
- the loader, process-install, address-space, materialization, launch,
  initial-stack, lifecycle, status, and `laststatus` chain is preserved.
- the initial stack record carries `argc=1`, `argv[0]=/bin/init`,
  `argv-null=false`, `envp-null=true`, `envp-state=empty-envp0`,
  `envp-entries=0`, an envp NULL-slot user pointer, an argv0 user pointer,
  and `copied-startup-bytes=0x2a`.
- `exec /missing` and `exec /etc/banner.txt` remain deterministic
  negative controls and do not create successful lifecycle records.
- `cat /etc/banner.txt` remains the descriptor-backed VFS/open/read
  regression surface.

This does not accept general POSIX startup, environment-variable propagation,
PATH lookup, arbitrary argv construction, auxv/TLS, libc startup, descriptor
inheritance, wait/waitpid, process replacement, or Pi 5 hardware behavior.

## Evidence Map

- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 379
  no_std tests.
- QEMU/substitute: empty-envp exec smoke retained at
  `tasks/evidence/2026-06-03-phase10-empty-envp-exec-init/qemu-local-shell-empty-envp-smoke.log`;
  transcript includes
  `exec-startup-abi state=minimal-argc1-argv0-init-empty-envp ... envp-null=true envp-state=empty-envp0 envp-entries=0x0000000000000000 ... copied-startup-bytes=0x000000000000002a source=initial-user-stack-record`,
  deterministic negative exec controls, lifecycle/`laststatus`, VFS cat,
  `errors=0 classification=qemu-local-shell-vfs-exec-complete`, and
  `PASS`.
- QEMU/substitute: descriptor-backed VFS cat regression retained at
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`;
  transcript ends with `errors=0 classification=qemu-local-cat-banner-complete`
  and `PASS`.
- QEMU/substitute: lifecycle/laststatus regression retained at
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`;
  transcript includes the same empty-envp startup ABI line, matching
  lifecycle status, `laststatus`, `errors=0
  classification=qemu-local-shell-vfs-exec-complete`, and `PASS`.
- QEMU/substitute: initial-user-stack regression retained at
  `tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log`;
  transcript includes `envp-state=empty-envp0`, `envp-entries=0`,
  `copied-startup-bytes=42`, `errors=0
  classification=qemu-initial-user-stack-smoke-complete`, and `PASS`.

## Validation

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-empty-envp-smoke.sh --quiet`
  passed.
- QEMU/substitute: `scripts/qemu-local-cat-banner-smoke.sh --quiet`
  passed.
- QEMU/substitute:
  `scripts/qemu-local-shell-last-process-status-smoke.sh --quiet` passed.
- QEMU/substitute: `scripts/qemu-initial-user-stack-smoke.sh --quiet`
  passed.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused. No Pi 5 hardware,
power-cycle, or boot archive publication was performed.

Acceptance commit: recorded in durable supervisor state after commit creation.
