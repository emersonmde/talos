# Phase 10 Absolute VFS Exec Dispatch Closeout

Task: phase10-absolute-vfs-exec-dispatch-closeout-20260603

Status: accepted

## Scope

Close out the accepted absolute VFS executable dispatch frontier. This
checkpoint reconciles the accepted implementation, retained
QEMU/substitute dispatch evidence, descriptor-backed VFS cat and `laststatus`
regressions, deterministic negative controls, deferred surfaces, and the next
queued nonzero-status executable task.

No runtime code changed. No QEMU scenario was rerun by this checkpoint. No
Pi 5 hardware action, boot archive publication, power-cycle, or
hardwareTestLock acquisition was performed.

## Findings And Dispositions

- fixed: The accepted dispatch frontier has a single closeout record tying
  `/bin/zero` and `/bin/init` to the same descriptor-backed VFS/open/read,
  loader, process-install, address-space, materialization, initial-stack,
  launch, lifecycle, status, and `laststatus` chain.
- fixed: The retained non-init exec transcript reports
  `exec path=/bin/zero source=vfs-open-read`, `argv0=/bin/zero`,
  `state=minimal-argc1-argv0-absolute-empty-envp`, deterministic empty envp,
  zero lifecycle/status, and path-aware `laststatus`.
- fixed: The retained `/bin/init` regression still reports the accepted
  `minimal-argc1-argv0-init-empty-envp` startup ABI through the same
  descriptor-backed VFS/program-loader/userspace lifecycle path.
- fixed: Missing paths, relative/PATH-style names, directories,
  non-executable/non-ELF files, and empty files remain deterministic negative
  controls and do not create successful lifecycle records.
- fixed: Descriptor-backed `cat /etc/banner.txt` remained present in the same
  retained PASS transcript, keeping the file-I/O regression attached to this
  exec frontier.
- not-an-issue: Hardware proof is absent because the task and this closeout
  make only QEMU/substitute and static-evidence claims.
- deferred: nonzero status variation, PATH lookup, broad argv/envp,
  environment variables, auxv/TLS, libc startup, descriptor inheritance,
  wait/waitpid, asynchronous execution, multiple children, process
  replacement, pipes, redirection, writable filesystem, Pi 5 proof,
  networking, and SSH remain outside the accepted frontier.

## Accepted Frontier

The accepted dispatch behavior is deliberately narrow:

- `exec /bin/zero` and `exec /bin/init` are the accepted absolute executable
  paths.
- both successful paths open and read executable bytes through
  descriptor-backed `TalosOpen`/`TalosRead` before program loading.
- both successful paths preserve the accepted loader, process-install,
  address-space, materialization, launch, initial-stack, lifecycle, status,
  and shell-visible `laststatus` chain.
- `/bin/zero` carries `argc=1`, `argv[0]=/bin/zero`, non-null argv state,
  `envp-state=empty-envp0`, `envp-entries=0`, copied startup bytes, and zero
  lower-AArch64 SVC status-equivalent lifecycle output.
- `/bin/init` remains the zero-status control with
  `state=minimal-argc1-argv0-init-empty-envp`.
- `exec /missing`, `exec init`, `exec /bin`, `exec /etc/banner.txt`, and
  `exec /empty` are deterministic negative controls.
- `cat /etc/banner.txt` remains descriptor-backed.

This does not accept PATH search, relative executable dispatch, arbitrary
arguments, environment-variable propagation, auxv/TLS, libc startup,
wait/waitpid, descriptor inheritance across exec, asynchronous process
management, process replacement, pipes, redirection, writable filesystem,
hardware behavior, networking, or SSH.

## Evidence Map

- accepted absolute VFS exec dispatch task record:
  `tasks/2026-06-03-phase10-absolute-vfs-exec-dispatch-core.md`.
- accepted absolute VFS exec dispatch commit:
  `791294538039fb26f194611d1c8ca14d7ddb37e7`.
- QEMU/substitute absolute VFS exec dispatch transcript:
  `tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log`.
  Static inspection found `exec /bin/zero`, `exec path=/bin/zero
  source=vfs-open-read`, `argv0=/bin/zero`,
  `state=minimal-argc1-argv0-absolute-empty-envp`, deterministic empty envp,
  zero lifecycle/status, and `laststatus` for `/bin/zero`.
- QEMU/substitute `/bin/init` regression: the same retained transcript found
  `exec /bin/init`, `exec path=/bin/init source=vfs-open-read`,
  `state=minimal-argc1-argv0-init-empty-envp`, zero lifecycle/status, and
  `laststatus` for `/bin/init`.
- QEMU/substitute negative-control evidence: the same retained transcript
  found deterministic failures for `/missing`, `init`, `/bin`,
  `/etc/banner.txt`, and `/empty` before the final PASS classification.
- QEMU/substitute descriptor-backed VFS cat regression: the same retained
  transcript found `cat /etc/banner.txt` returning `Talos initramfs fixture`.
- PASS/classification evidence: the same retained transcript ended with
  `final participants=13 expected=13 errors=0
  classification=qemu-local-shell-vfs-exec-complete` and `PASS`.

## Next Task Check

The queued `phase10-vfs-exec-nonzero-status-core-20260603` task remains
mechanically justified after this closeout because accepted evidence now
proves more than the historical `/bin/init` special case: an absolute
non-init VFS executable can reach loader, startup ABI, launch, lifecycle, and
shell-visible status through the accepted route. The next task should vary
the launched VFS-backed program's status and make `laststatus` report that
same nonzero lifecycle record.

That task must remain bounded to nonzero status variation through the accepted
absolute dispatch path. It must not expand into PATH lookup, broad argv/envp,
environment variables, auxv/TLS, libc startup, descriptor inheritance,
wait/waitpid, asynchronous execution, multiple children, pipes, redirection,
writable filesystem, Pi 5 proof, networking, or SSH.

## Validation

- static inspection: accepted dispatch task record and retained QEMU/substitute
  evidence log were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
