# Phase 10 Minimal Waitpid Lifecycle Observation Core

Task: phase10-minimal-waitpid-lifecycle-observation-core-20260603

Status: accepted

## Scope

Expose the smallest shell-visible wait/waitpid-shaped lifecycle observation
for the accepted descriptor-backed VFS exec path. The surface consumes one
completed child lifecycle/status record and reports deterministic no-child
behavior before exec and after the record has been consumed.

This task keeps execution on the accepted VFS/open/read, program-loader,
process-install, address-space, materialization, initial-stack, launch,
lifecycle, and status path. It does not add asynchronous execution, multiple
children, broad zombie-table policy, process replacement, fork, signals, PATH
lookup, broad argv/envp, descriptor inheritance expansion, pipes, redirection,
writable filesystem, Pi 5 proof, networking, or SSH.

## Findings And Dispositions

- fixed: Added a narrow `waitpid` shell command with no arguments. It reports
  `talos: waitpid no-child source=lifecycle-record` when there is no waitable
  lifecycle record.
- fixed: Successful VFS-backed `exec` now stores one waitable lifecycle/status
  record alongside the existing non-consuming latest-process record.
- fixed: `waitpid` consumes the waitable record and reports process identity,
  shell parent ownership, executable path, exited state, status,
  observed-status, and `reaped=true source=lifecycle-record`.
- fixed: A second `waitpid` after consuming the record deterministically
  reports no-child instead of fabricating another successful lifecycle record.
- fixed: `laststatus` remains a non-consuming regression/control surface and
  still reports the latest accepted lifecycle identity/status after wait
  consumption.
- fixed: The QEMU local command-loop harness has a dedicated
  `qemu_local_shell_waitpid` scenario and retained smoke wrapper so older
  exec/status smoke evidence remains stable.
- fixed: The help/status command list and local command-loop version/boundary
  now advertise the accepted waitpid observation surface.
- deferred: asynchronous execution, multiple children, broad zombie-table
  policy, process replacement, fork, signals, PATH lookup, broad argv/envp,
  descriptor inheritance expansion, pipes, redirection, writable filesystem,
  Pi 5 proof, networking, and SSH remain outside this frontier.

## Accepted Frontier

The accepted process-management frontier now includes a minimal waitpid-style
observation of the single completed child record produced by the VFS-backed
exec path:

- `waitpid` before any successful exec reports no-child.
- `exec /bin/status42` still reaches descriptor-backed VFS/open/read,
  loader, startup ABI, launch, lifecycle/status, and status `0x2a`.
- `waitpid` after `exec /bin/status42` reports the same path, shell owner,
  exited state, and status `0x2a` from the lifecycle/status record.
- a second `waitpid` after the same exec reports no-child.
- `laststatus` remains non-consuming and agrees with the latest lifecycle
  semantics after wait observation.
- `/bin/init` and `/bin/zero` remain zero-status controls through the same
  accepted absolute VFS exec path and wait observation surface.
- `exec /missing`, `exec init`, `exec /bin`,
  `exec /etc/banner.txt`, and `exec /empty` remain deterministic negative
  controls.
- `cat /etc/banner.txt` remains descriptor-backed.

This does not accept asynchronous execution, multiple children, broad zombie
management, process replacement, fork, signals, PATH lookup, descriptor
inheritance expansion, pipes, redirection, writable filesystem, Pi 5 hardware
behavior, networking, or SSH.

## Evidence Map

- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 382 no_std
  tests.
- QEMU/substitute: waitpid lifecycle observation evidence retained at
  `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`;
  transcript includes no-child before exec, `exec /bin/status42`,
  `waitpid` status `0x2a`, already-consumed no-child, matching
  `laststatus`, zero-status `/bin/init` and `/bin/zero` wait controls,
  deterministic negative controls for `/missing`, `init`, `/bin`,
  `/etc/banner.txt`, and `/empty`, descriptor-backed
  `cat /etc/banner.txt`, final participants=18 expected=18 errors=0,
  classification=`qemu-local-shell-waitpid-complete`, and PASS.
- QEMU/substitute regression: the same retained transcript covers
  `/bin/status42`, `/bin/init`, `/bin/zero`, negative exec controls,
  `laststatus`, and descriptor-backed `cat /etc/banner.txt`.

## Validation

- formatting: `cargo fmt --all -- --check` passed after applying
  `cargo fmt --all`.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-waitpid-lifecycle-smoke.sh --quiet`
  passed and retained the evidence log above.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused. No Pi 5 hardware,
power-cycle, or boot archive publication was performed.

Acceptance commit: recorded in durable supervisor state after commit creation.
