# Phase 10 Startup ABI Closeout

Task: phase10-startup-abi-closeout-20260603

Status: accepted

## Scope

Close out the minimal startup ABI frontier for the explicit VFS-backed
`exec /bin/init` path. This reconciles the accepted argc/argv and empty-envp
task records, retained QEMU/substitute evidence, descriptor-backed VFS cat and
lifecycle/`laststatus` regressions, deferred startup ABI surfaces, and the
next feature recommendation.

No runtime code changed. No QEMU scenario was rerun by this checkpoint. No
Pi 5 hardware action, boot archive publication, power-cycle, or
hardwareTestLock acquisition was performed.

## Findings And Dispositions

- fixed: The accepted startup ABI frontier now has a single closeout record
  that ties `argc=1`, `argv[0]=/bin/init`, non-null argv state,
  `envp-state=empty-envp0`, `envp-entries=0`, the envp NULL-slot pointer, and
  `copied-startup-bytes=0x2a` to the same initial-user-stack record.
- fixed: The retained shell-visible `exec /bin/init` transcript reports the
  startup ABI from `source=initial-user-stack-record` in the same
  descriptor-backed VFS/open/read, loader, launch, lifecycle, and status
  lineage as the accepted exec path.
- fixed: Descriptor-backed VFS cat, lifecycle/`laststatus`, and
  initial-user-stack regressions remained passing in retained evidence after
  the empty-envp slice.
- removed: The earlier `minimal-empty-argc0` startup placeholder is no longer
  part of the accepted `/bin/init` startup ABI frontier.
- not-an-issue: `envp-null=true` remains present because the accepted
  environment is represented as an empty envp vector with envp[0] == NULL,
  not as propagated environment variables.
- not-an-issue: Hardware proof is absent because the startup ABI slices and
  this closeout make only QEMU/substitute and static evidence claims.
- deferred: environment variables, PATH lookup, arbitrary executable
  dispatch, arbitrary argv construction, auxv, TLS, libc startup, descriptor
  inheritance, wait/waitpid, asynchronous execution, multiple children,
  process replacement, pipes, redirection, writable filesystem, Pi 5 proof,
  networking, and SSH remain outside the accepted frontier.

## Accepted Frontier

The accepted startup ABI behavior remains intentionally narrow:

- `exec /bin/init` opens and reads `/bin/init` through the descriptor-backed
  VFS/open/read path before program loading.
- the loader, process-install, address-space, materialization, launch,
  initial-stack, lifecycle, status, and `laststatus` chain is preserved.
- the initial stack record carries `argc=1`, `argv[0]=/bin/init`,
  `argv-null=false`, `envp-null=true`, `envp-state=empty-envp0`,
  `envp-entries=0`, an envp NULL-slot user pointer, an argv0 user pointer,
  and `copied-startup-bytes=0x2a`.
- shell-visible `exec /bin/init` reports that startup ABI state from
  `source=initial-user-stack-record`.
- `exec /missing` and `exec /etc/banner.txt` remain deterministic negative
  controls and do not create successful lifecycle records.
- `cat /etc/banner.txt` remains the descriptor-backed VFS/open/read
  regression surface.

This does not accept general POSIX process startup. It does not claim
environment-variable propagation, PATH lookup, arbitrary argv construction,
auxv/TLS, libc startup, descriptor inheritance, wait/waitpid, process
replacement, or Pi 5 hardware behavior.

## Evidence Map

- accepted minimal argc/argv task record:
  `tasks/2026-06-03-phase10-minimal-argv-argc-exec-init.md`.
- accepted minimal argc/argv commit:
  `0dcc3457a25a37ec91fd6d303f20225413ddfc10`.
- accepted argc/argv closeout task record:
  `tasks/2026-06-03-phase10-argv-argc-exec-init-closeout.md`.
- accepted argc/argv closeout commit:
  `7babda61355e4f6762447c4ef1039ee03452c55e`.
- accepted empty-envp task record:
  `tasks/2026-06-03-phase10-empty-envp-exec-init.md`.
- accepted empty-envp commit:
  `f934e5054b2c101e8f92928ffb0cfb246f304911`.
- QEMU/substitute empty-envp exec transcript:
  `tasks/evidence/2026-06-03-phase10-empty-envp-exec-init/qemu-local-shell-empty-envp-smoke.log`.
  Static inspection found
  `exec-startup-abi state=minimal-argc1-argv0-init-empty-envp argc=0x0000000000000001
  argv0=/bin/init ... argv-null=false envp-null=true
  envp-state=empty-envp0 envp-entries=0x0000000000000000
  copied-startup-bytes=0x000000000000002a
  source=initial-user-stack-record`, deterministic negative controls,
  lifecycle/`laststatus`, VFS cat, `errors=0
  classification=qemu-local-shell-vfs-exec-complete`, and `PASS`.
- QEMU/substitute minimal argc/argv exec transcript:
  `tasks/evidence/2026-06-03-phase10-minimal-argv-argc-exec-init/qemu-local-shell-argv-argc-smoke.log`.
  Static inspection found the prior `minimal-argc1-argv0-init` startup ABI
  line, deterministic negative controls, matching lifecycle output,
  `errors=0 classification=qemu-local-shell-vfs-exec-complete`, and `PASS`.
- QEMU/substitute descriptor-backed VFS cat regression:
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`.
  Static inspection found descriptor-backed input/output status, the
  `cat /etc/banner.txt` command, `errors=0
  classification=qemu-local-cat-banner-complete`, and `PASS`.
- QEMU/substitute lifecycle/`laststatus` regression:
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`.
  Static inspection found the empty-envp startup ABI line, lifecycle status,
  `laststatus` output from `source=lifecycle-record`, `errors=0
  classification=qemu-local-shell-vfs-exec-complete`, and `PASS`.
- QEMU/substitute initial-user-stack regression:
  `tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log`.
  Static inspection found `envp-state=empty-envp0`, `envp-entries=0`,
  `copied-startup-bytes=42`, `errors=0
  classification=qemu-initial-user-stack-smoke-complete`, and `PASS`.

## Next Feature Recommendation

The next feature-led step should be supervisor planning for absolute VFS
executable dispatch before PATH lookup, wait/waitpid, descriptor inheritance,
or shell command expansion. Current evidence already proves descriptor-backed
VFS open/read, loader, launch, lifecycle/status, startup ABI, and deterministic
negative exec controls for an explicit absolute `/bin/init` path. General
PATH lookup would add command-search policy before arbitrary absolute exec is
real; wait/waitpid and descriptor inheritance need broader process ownership
and lifetime semantics than the current single accepted exec lifecycle.

The worker is not creating that task in this closeout. Durable state should
request supervisor planning after this accepted checkpoint.

## Validation

- static inspection: accepted startup ABI task records and retained evidence
  logs were inspected.
- static inspection: `git diff --check` passed.
- documentation: `. /home/node/.cargo/env && /home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
