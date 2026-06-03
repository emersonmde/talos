# Phase 10 Literal Argv Exec Core

Task: phase10-literal-argv-exec-core-20260603

## Summary

Accepted the narrow literal argv propagation slice for absolute VFS-backed
exec. The shell now splits `exec` arguments into deterministic whitespace
tokens, keeps `argv[0]` as the absolute executable path, copies up to three
additional literal words into the startup ABI record, and rejects unsupported
shell grammar without reporting it as accepted behavior.

This remains tied to the accepted VFS/open/read, program-loader, process
install/address-space/materialization, startup stack, lifecycle/status,
waitpid/laststatus, and standard descriptor inheritance records. It does not
add PATH lookup, quoting, escaping, globbing, variables, broad envp, pipes,
redirection, userspace stdio through inherited descriptors, writable
filesystem behavior, hardware proof, networking, or SSH.

## Findings And Disposition

- fixed: `exec` previously treated the whole argument tail as one path, which
  made `exec /bin/status42 alpha beta` invalid. Added bounded literal token
  parsing and a copied argv record for the VFS exec request.
- fixed: the initial user stack model had a fixed argc=1 payload layout.
  Added a narrow startup-payload entry point that records dynamic argc and
  copied startup bytes while leaving the original argc=1 planner path intact.
- fixed: QEMU shell transcript coverage had no literal argv scenario. Added
  `qemu_local_shell_literal_argv` plus a wrapper script and retained evidence
  log.
- fixed: unsupported glob and escape-style tokens now fail deterministically
  with `exec-invalid-path` before any successful lifecycle record is created.
- not-an-issue: existing `/bin/init`, `/bin/zero`, `/bin/status42`,
  `waitpid`, `laststatus`, standard descriptor inheritance, loader temporary
  descriptor non-leak, and descriptor-backed cat surfaces remained compatible
  and are covered in the retained QEMU/substitute log.
- deferred: quoted strings, escaping, globbing, variables, command
  substitution, environment variables, PATH lookup, userspace stdio I/O,
  pipes, redirection, writable filesystem behavior, Pi 5 hardware proof,
  networking, and SSH remain out of scope.

## Evidence

- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 384 no_std
  tests, including `local_command_loop_execs_absolute_vfs_program_with_literal_argv`
  and `local_command_loop_rejects_unsupported_literal_exec_grammar`.
- QEMU/substitute feature smoke:
  `tasks/evidence/2026-06-03-phase10-literal-argv-exec-core/qemu-local-shell-literal-argv-smoke.log`.
  The log contains `exec /bin/status42 alpha beta`,
  `state=literal-argv-absolute-empty-envp`, `argc=0x3`,
  `argv0=/bin/status42 argv1=alpha argv2=beta`,
  `loader-temp-open=false`, inherited `fd0`/`fd1`/`fd2`, `waitpid`,
  `laststatus`, `/bin/init` and `/bin/zero` controls, unsupported glob
  rejection, missing and relative exec negatives, descriptor-backed
  `cat /etc/banner.txt`, `classification=qemu-local-shell-literal-argv-complete`,
  and `qemu-local-shell-literal-argv: PASS`.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed.
- diff hygiene: `git diff --check` and `git diff --cached --check` passed.

## Validation Summary

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with QEMU 9.2.0
  on PATH.
- QEMU/substitute: `scripts/qemu-local-shell-literal-argv-smoke.sh --quiet`
  passed and retained the evidence log above.
- docs: `/home/node/.cargo/bin/mdbook build` passed.

## Next Action

Promote `phase10-literal-argv-exec-closeout-20260603` after this task is
accepted and committed. The closeout should reconcile the accepted literal
argv frontier before any minimal fixed `/bin` PATH-style lookup planning.
