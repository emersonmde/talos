# Phase 10 /dev/null Stdin Redirection Closeout

Task: phase10-dev-null-stdin-redirection-closeout-20260604
Status: accepted

## Scope

Close out the accepted stdin-from-/dev/null redirection behavior before the /dev/null stdio frontier checkpoint or read-only regular-file input redirection.

The accepted input redirection behavior is exactly 'exec stdin </dev/null'. The launched VFS-backed '/bin/stdin' fixture gets child fd0 rebound to the explicit '/dev/null' source device. The mutation is child-only, and the shell restores fd0 after the child exits.

The redirection record reports 'op=source', 'source-path=/dev/null', 'source-stream=null-source', and 'source-route=device:/dev/null'. 'TalosRead' from that device returns zero bytes as true device-source EOF without polling runtime-console0, and the stdin fixture reports 'read-source=device:/dev/null' plus 'read-result=null-source-eof/no-data' through the accepted stdout/status path.

This closeout does not add code and does not expand into regular-file input redirection, output regular-file redirection, append/truncate, writable filesystem behavior, arbitrary descriptor syntax, broader file/device semantics, multi-stage/concurrent pipelines, Pi 5 proof, networking, SSH, or a phase transition.

## Findings

- fixed: The accepted stdin core records '/dev/null' as an explicit fd0 source device, distinct from the accepted fd1/fd2 '/dev/null' sink device forms.
- fixed: The accepted evidence maps child-only fd0 rebinding, null-source route identity, zero-byte EOF/no-data behavior, and shell fd0 restoration.
- fixed: The evidence map retains runtime-console0 stdin, stdin readiness, terminal EOF, stdout/stderr '/dev/null' sinks, descriptor redirection, descriptor-mixing pipelines, VFS exec, lifecycle/status, waitpid, laststatus, negative controls, and descriptor-backed cat as regression coverage.
- fixed: The roadmap now has a closeout entry that prevents acceptance drift from exact 'exec stdin </dev/null' into regular-file input redirection, writable filesystem behavior, or broader file/device semantics.
- not-an-issue: The stdin smoke log includes a later visible stdin payload because the following normal 'exec stdin' control intentionally proves shell fd0 restoration after the redirected child exits.
- deferred: 'exec stdin </etc/banner.txt', 'exec stdin < /dev/null', 'exec stdout </dev/null', output regular-file redirection, append/truncate, shorthand/broader descriptor syntax, writable filesystem behavior, broader file/device semantics, multi-stage/concurrent pipelines, Pi 5 proof, networking, SSH, and a phase transition remain deferred. The queued /dev/null stdio frontier closeout is the only mechanically unblocked follow-up.

## Evidence Map

- stdin-from-/dev/null task smoke: 'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log' records command 3 'exec stdin </dev/null', 'fd0=device', 'exec-redirection op=source ... source-path=/dev/null ... source-stream=null-source source-route=device:/dev/null', 'exec-stdin ... read-source=device:/dev/null ... read-result=null-source-eof/no-data', zero-byte read/return, 'exec-status ... complete=true source=lifecycle-record', final 'qemu-local-shell-dev-null-stdin-redirection-complete', errors=0, and PASS.
- normal stdin restoration control: the same log records a following 'exec stdin' with 'read-source=runtime-console0/local-input' and a visible 'talos-console0' payload through stdout.
- deterministic negatives: the same log records unsupported 'exec stdout </dev/null', 'exec stdin </etc/banner.txt', and 'exec stdin < /dev/null' as deterministic negative forms outside the accepted surface.
- descriptor-backed VFS control: the same log records 'cat /etc/banner.txt' printing 'Talos initramfs fixture' after the redirection and restoration checks.
- retained stdout/stderr /dev/null sink controls: 'tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log' and 'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log'.
- retained stdin controls: 'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log', 'tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log', 'tasks/evidence/2026-06-03-phase10-stdin-eof-no-data-core/qemu-local-shell-stdin-eof-no-data-smoke.log', and 'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
- retained descriptor redirection and pipeline controls: 'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log', 'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log', 'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log', and 'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
- retained lifecycle/status, waitpid, laststatus, and descriptor-backed file I/O controls: 'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log', 'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log', and 'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted: exactly 'exec stdin </dev/null'; '/dev/null' as an explicit fd0 source device for the child only; child-only rebinding of fd0 with shell restoration after the child exits; zero-byte true device-source EOF/no-data without polling runtime-console0; stdout/status reporting through the accepted userspace fixture path; and retained controls for stdout/stderr '/dev/null' sinks, normal stdio, descriptor redirection, descriptor-mixing pipelines, VFS exec, lifecycle/status, 'waitpid', 'laststatus', deterministic negatives, and descriptor-backed cat.

Deferred: regular-file input redirection such as 'exec stdin </etc/banner.txt'; output regular-file redirection; append/truncate; writable filesystem behavior; arbitrary descriptor syntax; broader file/device semantics; here-docs; arbitrary path expansion; multi-stage/concurrent pipelines; pipefail; jobs; async execution; fork; signals; job control; Pi 5 proof; networking; SSH; and any phase transition.

## Next Step Requirement

The queued /dev/null stdio redirection frontier closeout is mechanically unblocked and must remain docs/evidence reconciliation only. After that checkpoint, the next queued feature-led implementation task is read-only regular-file stdin redirection, still bounded to descriptor-backed VFS/open/read and not output file redirection or writable filesystem mutation.

## Validation

- static inspection: accepted stdin core task record, task-owned evidence log, retained stdout/stderr '/dev/null' sink controls, stdin controls, descriptor redirection/pipeline controls, waitpid/laststatus, and descriptor-backed cat evidence were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final closeout commit recorded in supervisor state.
