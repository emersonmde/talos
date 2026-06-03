# Phase 10 Minimal Argv/Argc Exec Init

Task: phase10-minimal-argv-argc-exec-init-20260603

Status: accepted

## Scope

Add the smallest real startup ABI record for the existing VFS-backed
`exec /bin/init` path: `argc=1` and `argv[0]=/bin/init`. This stays inside
the accepted explicit `/bin/init` fixture path and does not add PATH lookup,
arbitrary executable dispatch, pipes, redirection, writable filesystem,
networking, SSH, Pi 5 proof, wait/waitpid, scheduler process-table semantics,
descriptor inheritance across exec, envp, auxv, TLS, or real process
replacement.

## Findings And Dispositions

- fixed: The initial user stack record no longer publishes the accepted
  `minimal-empty-argc0` placeholder for the `/bin/init` path. It now records
  `minimal-argc1-argv0-init`, `argc=1`, non-null argv state,
  `argv[0]=/bin/init`, an argv0 user pointer, and copied startup bytes.
- fixed: The initial stack SP now reflects the copied startup payload inside
  the accepted usable stack range instead of leaving SP at the stack top.
- fixed: The live address-space activation invariant no longer rejects a
  valid aligned SP below the top of the usable stack once startup bytes are
  present; it still requires the SP to stay inside the accepted usable stack
  range and preserves stack/data/execute checks.
- fixed: `exec /bin/init` now prints a shell-visible
  `talos: exec-startup-abi ... source=initial-user-stack-record` line tied to
  the same VFS/open/read, loader, launch, stack, lifecycle, and status
  transcript.
- fixed: QEMU shell exec smoke expectations now require eight exec response
  lines and grep the argc/argv startup ABI line.
- fixed: The older initial-user-stack QEMU smoke expectations now match the
  new non-empty startup payload, copied-byte accounting, and lowered initial
  SP instead of expecting the retired argc0 placeholder.
- fixed: Missing and non-executable exec targets remain deterministic negative
  controls and do not produce successful lifecycle records.
- not-an-issue: The argc/argv wrapper uses the existing
  `qemu_local_shell_vfs_exec` boot scenario label/classification because the
  underlying scenario is still the same explicit `/bin/init` exec feature; the
  retained evidence path and ABI transcript identify this task's proof.
- deferred: envp, auxv, TLS, environment variables, PATH lookup, arbitrary
  executable dispatch, descriptor inheritance, wait/waitpid, asynchronous
  process execution, process replacement, scheduler-owned process lifetime,
  pipes, redirection, writable filesystem, hardware proof, networking, and SSH
  remain outside the accepted frontier.

## Accepted Frontier

The accepted startup ABI behavior is intentionally narrow:

- `exec /bin/init` still opens and reads `/bin/init` through the
  descriptor-backed VFS/open/read path before program loading.
- The loader/process-install/address-space/materialization/launch/stack chain
  is preserved.
- The initial stack record now carries `argc=1`, `argv[0]=/bin/init`,
  `argv-null=false`, `envp-null=true`, an argv0 user pointer, and
  `copied-startup-bytes=0x22`.
- Shell-visible `exec /bin/init` reports the startup ABI line from the
  initial-user-stack record and then reports the same accepted lifecycle/status
  record.
- `laststatus` still reports the same latest lifecycle record.
- `cat /etc/banner.txt` remains the descriptor-backed VFS/open/read
  regression surface.
- `exec /missing` and `exec /etc/banner.txt` remain negative controls.

This does not accept general POSIX process startup. It does not claim envp,
PATH, arbitrary argv construction, libc startup, waitpid, descriptor
inheritance, process replacement, or Pi 5 hardware behavior.

## Evidence Map

- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 379
  no_std tests, including updated initial stack, local command loop, and live
  address-space activation coverage.
- QEMU/substitute: argc/argv exec smoke retained at
  `tasks/evidence/2026-06-03-phase10-minimal-argv-argc-exec-init/qemu-local-shell-argv-argc-smoke.log`;
  transcript ends with `errors=0 classification=qemu-local-shell-vfs-exec-complete`
  and `PASS`.
- QEMU/substitute: descriptor-backed VFS cat regression retained at
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`;
  transcript ends with `errors=0 classification=qemu-local-cat-banner-complete`
  and `PASS`.
- QEMU/substitute: lifecycle/laststatus regression retained at
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`;
  transcript ends with `errors=0 classification=qemu-local-shell-vfs-exec-complete`
  and `PASS`.
- QEMU/substitute: additional initial-user-stack smoke retained at
  `tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log`;
  transcript ends with `errors=0 classification=qemu-initial-user-stack-smoke-complete`
  and `PASS`.

## Validation

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-argv-argc-smoke.sh --quiet`
  passed.
- QEMU/substitute: `scripts/qemu-local-cat-banner-smoke.sh --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-last-process-status-smoke.sh --quiet`
  passed.
- QEMU/substitute: `scripts/qemu-initial-user-stack-smoke.sh --quiet` passed
  after updating the stale startup placeholder expectations.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused. No Pi 5 hardware,
power-cycle, or boot archive publication was performed.

Acceptance commit: recorded in durable supervisor state after commit creation.
