# Phase 10 VFS Exec Nonzero Status Closeout

Task: phase10-vfs-exec-nonzero-status-closeout-20260603

Status: accepted

## Scope

Close out the accepted nonzero VFS exec/status frontier. This checkpoint
reconciles the accepted implementation, retained QEMU/substitute evidence,
zero-status controls, deterministic negative controls, descriptor-backed VFS
cat regression, deferred surfaces, and the next local execution primitive.

No runtime code changed. No QEMU scenario was rerun by this checkpoint. No
Pi 5 hardware action, boot archive publication, power-cycle, or
hardwareTestLock acquisition was performed.

## Findings And Dispositions

- fixed: The accepted status frontier has a single closeout record tying
  `/bin/status42` to descriptor-backed VFS/open/read, loader, startup ABI,
  launch, lifecycle/status, and shell-visible `laststatus` evidence.
- fixed: Static inspection confirmed the retained `/bin/status42` transcript
  reports `exec path=/bin/status42 source=vfs-open-read`,
  `argv0=/bin/status42`,
  `state=minimal-argc1-argv0-absolute-empty-envp`, deterministic empty envp,
  copied startup bytes for the longer absolute path, lifecycle/status
  `0x2a`, matching observed status, and matching `laststatus`.
- fixed: Static inspection confirmed `/bin/init` and `/bin/zero` remain
  zero-status controls through the same accepted absolute VFS exec route.
- fixed: Static inspection confirmed `/missing`, `init`, `/bin`,
  `/etc/banner.txt`, and `/empty` remain deterministic negative exec controls
  without successful lifecycle records in the retained transcript.
- fixed: Descriptor-backed `cat /etc/banner.txt` remained present in the same
  retained PASS transcript, keeping file I/O regression evidence attached to
  this exec/status frontier.
- not-an-issue: Hardware proof is absent because the task and this closeout
  make only static-inspection and QEMU/substitute claims.
- deferred: wait/waitpid, asynchronous execution, multiple children, zombie
  policy, process replacement, descriptor inheritance across exec, broader
  argv/envp, environment variables, auxv/TLS, libc startup, PATH lookup,
  pipes, redirection, writable filesystem, Pi 5 proof, networking, and SSH
  remain outside the accepted frontier.

## Accepted Frontier

The accepted shell-visible exec/status behavior is deliberately narrow:

- `exec /bin/status42` is the accepted deterministic nonzero executable path.
- `/bin/status42` opens and reads executable bytes through descriptor-backed
  `TalosOpen`/`TalosRead` before program loading.
- the accepted loader, process-install, address-space, materialization,
  initial-stack, launch, lifecycle, status, and shell-visible `laststatus`
  chain is preserved.
- `/bin/status42` carries `argc=1`, `argv[0]=/bin/status42`, non-null argv
  state, `envp-state=empty-envp0`, `envp-entries=0`, copied startup bytes for
  the longer absolute path, and status `0x2a`.
- `/bin/init` and `/bin/zero` remain zero-status controls through the same
  accepted absolute VFS exec route.
- `exec /missing`, `exec init`, `exec /bin`, `exec /etc/banner.txt`, and
  `exec /empty` are deterministic negative controls.
- `cat /etc/banner.txt` remains descriptor-backed.

This does not accept wait/waitpid, asynchronous execution, multiple children,
zombie policy, descriptor inheritance, PATH lookup, arbitrary arguments,
environment-variable propagation, auxv/TLS, libc startup, process
replacement, pipes, redirection, writable filesystem, hardware behavior,
networking, or SSH.

## Evidence Map

- accepted nonzero VFS exec status task record:
  `tasks/2026-06-03-phase10-vfs-exec-nonzero-status-core.md`.
- accepted nonzero VFS exec status commit:
  `cd59daf192b1833a3c1e2cf2b7fff2225099f52a`.
- QEMU/substitute nonzero VFS exec status transcript:
  `tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log`.
  Static inspection found `exec /bin/status42`,
  `exec path=/bin/status42 source=vfs-open-read`,
  `argv0=/bin/status42`,
  `state=minimal-argc1-argv0-absolute-empty-envp`, deterministic empty envp,
  lifecycle/status `0x2a`, and `laststatus` for `/bin/status42`.
- QEMU/substitute zero-status controls: the same retained transcript found
  `exec /bin/init` and `exec /bin/zero` each reaching the accepted
  descriptor-backed VFS/program-loader/userspace lifecycle path with zero
  lifecycle/status records and matching `laststatus`.
- QEMU/substitute negative-control evidence: the same retained transcript
  found deterministic failures for `/missing`, `init`, `/bin`,
  `/etc/banner.txt`, and `/empty` before the final PASS classification.
- QEMU/substitute descriptor-backed VFS cat regression: the same retained
  transcript found `cat /etc/banner.txt` returning
  `Talos initramfs fixture`.
- PASS/classification evidence: the same retained transcript ended with
  `final participants=15 expected=15 errors=0
  classification=qemu-local-shell-vfs-exec-complete` and `PASS`.

## Next Feature Recommendation

The next feature-led local execution primitive should be a minimal
wait/waitpid-style lifecycle observation backed by the accepted
kernel-owned lifecycle/status record. The evidence now proves that status can
vary by VFS-backed userspace image and that `laststatus` can read the latest
lifecycle record; the next useful step is to make that lifecycle/status state
observable through a POSIX-shaped process-management surface rather than
adding command-search convenience.

That task should stay narrow: one child/lifecycle record, deterministic status
delivery, deterministic no-child behavior, and retained `/bin/status42`,
`/bin/init`, `/bin/zero`, negative exec, and descriptor-backed cat
regressions. It should not expand into asynchronous execution, multiple
children, broad zombie policy, descriptor inheritance, PATH lookup, broad
argv/envp, pipes, redirection, writable filesystem, hardware proof,
networking, or SSH.

## Validation

- static inspection: accepted nonzero status task record and retained
  QEMU/substitute evidence log were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
