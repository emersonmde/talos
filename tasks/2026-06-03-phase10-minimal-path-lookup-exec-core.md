# Phase 10 Minimal Path Lookup Exec Core

Task: phase10-minimal-path-lookup-exec-core-20260603

## Summary

Accepted the minimal fixed `/bin` lookup slice for VFS-backed shell exec. The
shell now accepts a bare first `exec` token such as `exec status42 alpha beta`,
resolves that token only to `/bin/status42`, reads the resolved executable
through descriptor-backed VFS/open/read, and runs it through the accepted
program-loader, startup ABI, launch, lifecycle/status, descriptor inheritance,
waitpid, and laststatus chain.

The accepted argv0 policy for this slice is canonical resolved path argv0:
bare `exec status42 alpha beta` records `argv0=/bin/status42` with literal
`argv1=alpha` and `argv2=beta`. This keeps the startup stack payload and copied
startup byte count aligned with the executable path consumed by the loader.

This does not add an environment-backed PATH variable, current-directory
search, command hashing, shell builtin conversion, quoting, escaping, globbing,
variables, pipes, redirection, userspace stdio I/O through inherited
descriptors, writable filesystem behavior, Pi 5 hardware proof, networking, or
SSH.

## Findings And Disposition

- fixed: `exec init` and other bare executable tokens previously failed as
  invalid relative paths. Added bounded fixed `/bin/<name>` resolution for the
  first exec token when it contains no slash.
- fixed: bare-name exec now canonicalizes `argv[0]` to the resolved `/bin/...`
  path while preserving later literal argv tokens, avoiding startup byte-count
  mismatches between argv0 and the loader source path.
- fixed: QEMU shell transcript coverage had no PATH-style lookup scenario.
  Added `qemu_local_shell_path_lookup`, a retained smoke wrapper, and evidence
  assertions for bare lookup, absolute exec regression, waitpid, laststatus,
  descriptor inheritance, loader temporary descriptor non-leak, negative
  controls, and descriptor-backed cat.
- fixed: relative/path-like names such as `exec bin/status42` remain
  unsupported and fail with `exec-invalid-path`; missing bare names such as
  `exec missing` resolve only far enough to fail deterministic VFS lookup with
  `exec-not-found`.
- not-an-issue: absolute `exec /bin/status42 gamma`, `exec /bin/init`, and
  `exec /bin/zero` continue through the accepted absolute VFS exec path.
- deferred: environment-backed PATH, current-directory search, command hashing,
  broad shell grammar, userspace stdio I/O through inherited descriptors,
  pipes, redirection, writable filesystem behavior, hardware proof, networking,
  and SSH remain out of scope.

## Evidence

- unit tests: `cargo -Zjson-target-spec test --quiet` passed with QEMU 9.2.0
  on PATH, including the new
  `local_command_loop_resolves_bare_exec_name_through_fixed_bin_lookup` test.
- QEMU/substitute feature smoke:
  `tasks/evidence/2026-06-03-phase10-minimal-path-lookup-exec-core/qemu-local-shell-path-lookup-smoke.log`.
  The log contains `exec status42 alpha beta`,
  `talos: exec path=/bin/status42 source=vfs-open-read`,
  inherited `fd0`/`fd1`/`fd2`, `loader-temp-open=false`,
  `state=literal-argv-absolute-empty-envp`, `argc=0x3`,
  `argv0=/bin/status42 argv1=alpha argv2=beta`, nonzero status `0x2a`,
  matching `waitpid` and `laststatus`, bare `exec init` and `exec zero`
  zero-status controls, absolute `exec /bin/status42 gamma`, deterministic
  negatives for missing bare name, path-like relative name, directory,
  non-executable files, and unsupported glob grammar, descriptor-backed
  `cat /etc/banner.txt`, `classification=qemu-local-shell-path-lookup-complete`,
  and `qemu-local-shell-path-lookup: PASS`.
- diff hygiene: `git diff --check` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff hygiene: `git diff --cached --check` passed before commit.

## Validation Summary

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-path-lookup-smoke.sh --quiet`
  passed and retained the evidence log above.
- docs: `/home/node/.cargo/bin/mdbook build` passed.

## Next Action

Promote `phase10-minimal-path-lookup-exec-closeout-20260603` after this task is
accepted and committed. The closeout should reconcile the accepted fixed `/bin`
lookup frontier before any broader shell grammar, userspace stdio, pipe,
redirection, filesystem mutation, networking, or SSH planning.
