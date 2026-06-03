# Phase 10 Standard Descriptor Inheritance Exec Core

Task: phase10-standard-descriptor-inheritance-exec-core-20260603

Status: accepted

## Scope

Prove the accepted VFS-backed shell exec path carries a minimal standard
descriptor inheritance record for stdin, stdout, and stderr without leaking
the loader/VFS temporary descriptor used to read executable bytes.

This task keeps execution on the existing descriptor-backed VFS/open/read,
program-loader, process-install, address-space, materialization,
initial-stack, launch, lifecycle/status, laststatus, and waitpid observation
path. It does not add userspace reads/writes through inherited descriptors,
dup/close inheritance policy beyond the standard slots, fork, asynchronous
execution, multiple children, PATH lookup, broad argv/envp, pipes,
redirection, writable filesystem, hardware proof, networking, or SSH.

## Findings And Dispositions

- fixed: Successful VFS-backed exec now records a minimal `exec-descriptors`
  transcript line tied to the shell process descriptor table owner.
- fixed: The descriptor record proves inherited standard slots `fd0`,
  `fd1`, and `fd2` as `stdio-input`, `stdio-output`, and `stdio-output`.
- fixed: The descriptor record is emitted for `/bin/status42`, `/bin/init`,
  and `/bin/zero`, so the nonzero and zero-status exec controls share the
  same standard-descriptor inheritance surface.
- fixed: The record reports `loader-temp-fd=0x3 loader-temp-open=false`,
  proving the descriptor-backed `TalosOpen`/`TalosRead` executable-load
  descriptor was closed before the launched process descriptor set is
  recorded.
- fixed: The QEMU local command-loop harness and target-side response-count
  checks now account for the extra exec transcript line.
- fixed: Added a task-specific descriptor-inheritance smoke wrapper that
  retains evidence under this task while keeping the existing compiled
  `qemu_local_shell_vfs_exec` label/classification contract.
- not-an-issue: The descriptor inheritance record is observational only; it
  does not grant userspace stdio read/write semantics beyond already accepted
  descriptor table lineage.
- deferred: userspace stdio I/O through inherited descriptors, broad
  descriptor inheritance policy, close-on-exec behavior, dup/close policy
  across exec, fork, asynchronous execution, multiple children, PATH lookup,
  broad argv/envp, pipes, redirection, writable filesystem, Pi 5 hardware
  proof, networking, and SSH remain outside this frontier.

## Accepted Frontier

The accepted local execution frontier now includes a standard descriptor
inheritance record for the VFS-backed exec path:

- `exec /bin/status42`, `exec /bin/init`, and `exec /bin/zero` still read
  executable bytes through descriptor-backed VFS/open/read before the accepted
  loader/process/launch/startup/lifecycle/status chain.
- each successful exec transcript now reports `fd0=stdio-input`,
  `fd1=stdio-output`, and `fd2=stdio-output` from
  `source=shell-process-descriptor-table`.
- each successful exec transcript reports `loader-temp-fd=0x3` and
  `loader-temp-open=false`, proving the loader/VFS temporary file descriptor
  is absent from the inherited standard descriptor set.
- `/bin/status42` remains the nonzero status control with status `0x2a`.
- `/bin/init` and `/bin/zero` remain zero-status controls.
- `laststatus`, `waitpid`, deterministic negative exec controls, and
  descriptor-backed `cat /etc/banner.txt` remain covered by retained
  QEMU/substitute evidence.

This does not accept userspace stdio I/O through inherited descriptors, broad
descriptor inheritance rules, close-on-exec policy, fork, asynchronous
execution, multiple children, PATH lookup, broad argv/envp, pipes,
redirection, writable filesystem, Pi 5 hardware behavior, networking, or SSH.

## Evidence Map

- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 382 no_std
  tests.
- QEMU/substitute descriptor inheritance: evidence retained at
  `tasks/evidence/2026-06-03-phase10-standard-descriptor-inheritance-exec-core/qemu-local-shell-standard-descriptor-inheritance-smoke.log`;
  transcript includes `exec-descriptors` for `/bin/status42`, `/bin/init`,
  and `/bin/zero`, inherited `fd0`/`fd1`/`fd2`, `loader-temp-open=false`,
  matching `laststatus` for nonzero and zero controls, deterministic negative
  exec controls for `/missing`, `init`, `/bin`, `/etc/banner.txt`, and
  `/empty`, descriptor-backed `cat /etc/banner.txt`, final participants=15
  expected=15 errors=0, classification=qemu-local-shell-vfs-exec-complete,
  and PASS.
- QEMU/substitute waitpid regression: evidence retained at
  `tasks/evidence/2026-06-03-phase10-standard-descriptor-inheritance-exec-core/qemu-local-shell-standard-descriptor-inheritance-waitpid-regression.log`;
  transcript includes no-child before exec, `exec /bin/status42`,
  `waitpid` consuming status `0x2a`, already-consumed no-child, matching
  `laststatus`, zero-status `/bin/init` and `/bin/zero` wait controls,
  descriptor inheritance records for each successful exec, deterministic
  negative exec controls, descriptor-backed `cat /etc/banner.txt`, final
  participants=18 expected=18 errors=0, classification=qemu-local-shell-waitpid-complete,
  and PASS.

## Validation

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-standard-descriptor-inheritance-smoke.sh --quiet`
  passed and retained the descriptor-inheritance evidence log above.
- QEMU/substitute regression: direct `qemu_local_shell_waitpid` command-loop
  smoke passed and retained the waitpid regression log above.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused. No Pi 5 hardware,
power-cycle, or boot archive publication was performed.

Acceptance commit: recorded in durable supervisor state after commit creation.
