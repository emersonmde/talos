# Phase 10 Argv/Argc Exec Init Closeout

Task: phase10-argv-argc-exec-init-closeout-20260603

Status: accepted

## Scope

Close out the accepted minimal startup ABI slice for the explicit
VFS-backed `exec /bin/init` path. This reconciles the accepted task record,
retained QEMU/substitute evidence, descriptor-backed VFS cat and
lifecycle/laststatus regressions, deferred surfaces, and the next queued
empty-envp startup ABI task.

No runtime code changed. No QEMU scenario was rerun by this checkpoint. No
Pi 5 hardware action, boot archive publication, power-cycle, or
hardwareTestLock acquisition was performed.

## Findings And Dispositions

- fixed: The accepted minimal argc/argv task replaced the prior
  `minimal-empty-argc0` startup placeholder for the explicit `/bin/init`
  path with a kernel/user startup record carrying
  `minimal-argc1-argv0-init`, `argc=1`, non-null argv state,
  `argv[0]=/bin/init`, an argv0 user pointer, and copied startup bytes.
- fixed: The accepted shell-visible `exec /bin/init` transcript now reports
  the startup ABI state from `source=initial-user-stack-record` in the same
  VFS/open/read, loader, launch, lifecycle, and status lineage.
- fixed: The accepted initial-user-stack smoke expectations were updated to
  the new non-empty startup payload, copied-byte count, and lowered initial
  stack pointer.
- fixed: Missing and non-executable exec targets remain deterministic
  negative controls and do not create successful lifecycle records.
- fixed: Descriptor-backed `cat /etc/banner.txt` and lifecycle/`laststatus`
  regressions remained passing after the startup ABI slice.
- not-an-issue: `envp-null=true` is retained as an empty/null environment
  observation for the current stack record, but this closeout does not accept
  an explicit empty-envp startup ABI contract beyond the prior transcript
  field.
- not-an-issue: Hardware was unused because this checkpoint and the accepted
  argc/argv slice make no physical Pi 5 claim.
- deferred: explicit empty envp contract, environment variables, auxv, TLS,
  libc startup, PATH lookup, arbitrary executable dispatch, descriptor
  inheritance across exec, wait/waitpid, asynchronous execution, multiple
  children, scheduler-owned process lifetime, process replacement, pipes,
  redirection, writable filesystem, hardware proof, networking, and SSH remain
  outside the accepted frontier.

## Accepted Frontier

The accepted startup ABI claim is intentionally narrow:

- `exec /bin/init` still opens and reads `/bin/init` through the
  descriptor-backed VFS/open/read path before program loading.
- the loader, process-install, address-space, materialization, launch,
  initial-stack, lifecycle, and status chain is preserved.
- the initial stack record carries `argc=1`, `argv[0]=/bin/init`,
  `argv-null=false`, `envp-null=true`, an argv0 user pointer, and
  `copied-startup-bytes=0x22`.
- shell-visible `exec /bin/init` reports
  `state=minimal-argc1-argv0-init`, `argc=0x1`, `argv0=/bin/init`, and
  `source=initial-user-stack-record` from that record.
- `laststatus` reports the same latest lifecycle record produced by the
  `exec /bin/init` path.
- `cat /etc/banner.txt` remains the descriptor-backed VFS/open/read
  regression surface.

This does not accept general POSIX process startup. It does not claim PATH
lookup, arbitrary argv construction, environment-variable propagation, auxv,
TLS, libc startup, descriptor inheritance, wait/waitpid, process replacement,
or Pi 5 hardware behavior.

## Evidence Map

- accepted argc/argv task record:
  `tasks/2026-06-03-phase10-minimal-argv-argc-exec-init.md`.
- accepted argc/argv commit:
  `0dcc3457a25a37ec91fd6d303f20225413ddfc10`.
- QEMU/substitute argc/argv exec transcript:
  `tasks/evidence/2026-06-03-phase10-minimal-argv-argc-exec-init/qemu-local-shell-argv-argc-smoke.log`.
  Static inspection found the expected
  `exec-startup-abi state=minimal-argc1-argv0-init argc=0x0000000000000001
  argv0=/bin/init ... argv-null=false envp-null=true
  source=initial-user-stack-record` line, deterministic `exec-not-found`
  and `exec-not-executable` negative controls, matching `last-process`
  lifecycle output, `errors=0 classification=qemu-local-shell-vfs-exec-complete`,
  and `PASS`.
- QEMU/substitute descriptor-backed VFS cat regression:
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`.
  Static inspection found descriptor-backed input/output status, the
  `cat /etc/banner.txt` command, `errors=0
  classification=qemu-local-cat-banner-complete`, and `PASS`.
- QEMU/substitute lifecycle/laststatus regression:
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`.
  Static inspection found the same argc/argv startup ABI line, lifecycle
  status, `laststatus` output from `source=lifecycle-record`,
  `errors=0 classification=qemu-local-shell-vfs-exec-complete`, and
  `PASS`.

## Next Task Check

The queued `phase10-empty-envp-exec-init-20260603` task remains
mechanically justified after this closeout because the accepted
`/bin/init` startup ABI path now carries stable argc/argv evidence and
already exposes `envp-null=true` as a transcript field. That next task must
still implement and prove only a deterministic empty-envp startup ABI record;
it must not expand into environment variables, PATH lookup, arbitrary exec,
auxv/TLS, descriptor inheritance, wait/waitpid, pipes, redirection, writable
filesystem, Pi 5 proof, networking, or SSH.

## Validation

- static inspection: accepted task record and retained argc/argv, VFS cat, and
  lifecycle/laststatus evidence logs were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
