# Phase 10 Absolute VFS Exec Dispatch Core

Task: phase10-absolute-vfs-exec-dispatch-core-20260603

Status: accepted

## Scope

Generalize the shell-visible exec <absolute-path> path beyond the special
/bin/init fixture while keeping execution backed by descriptor-backed
VFS/open/read, the accepted program-loader boundary, process-install,
address-space, materialization, initial-stack, launch, lifecycle, and status
records.

This task adds only a smallest zero-status non-init executable fixture,
/bin/zero. It does not add PATH lookup, relative command dispatch, arbitrary
argv beyond argv[0], environment variables, auxv/TLS, libc startup,
descriptor inheritance, wait/waitpid, multiple children, asynchronous
execution, pipes, redirection, writable filesystem, Pi 5 proof, networking, or
SSH.

## Findings And Dispositions

- fixed: exec now accepts absolute VFS paths through a single bounded
  dispatcher instead of hard-coding only /bin/init.
- fixed: Added /bin/zero as a read-only initramfs regular file under /bin,
  backed by the same narrow static ELF64/AArch64 bytes and lower-AArch64 SVC
  status-equivalent convention as /bin/init.
- fixed: The shell-visible exec transcript now reports the executed path from
  the loader image source, so /bin/zero and /bin/init are distinguishable in
  the VFS/open/read, startup ABI, lifecycle, status, and laststatus records.
- fixed: The initial-stack payload now preserves the accepted /bin/init state
  label while recording a path-generic
  minimal-argc1-argv0-absolute-empty-envp state for non-init absolute
  dispatch. argv[0] is the executed absolute path and envp remains the
  accepted deterministic empty envp.
- fixed: laststatus now includes the latest lifecycle path as well as PID,
  shell parent owner, exited state, status, observed status, and reaped state.
- fixed: Missing paths, relative/PATH-style names, directories, non-ELF regular
  files, and empty files fail deterministically without creating successful
  lifecycle records.
- fixed: /bin listings now include both accepted executable fixture names,
  init and zero.
- fixed: The QEMU/substitute shell exec smoke now covers /bin/zero,
  laststatus, /bin/init, laststatus, missing-path, relative-path, directory,
  non-ELF, empty-file, and descriptor-backed cat regression controls in one
  retained PASS transcript.
- deferred: PATH lookup, broad argv/envp, auxv/TLS, libc startup, descriptor
  inheritance, wait/waitpid, asynchronous execution, multiple children,
  process replacement, nonzero status variation, pipes, redirection, writable
  filesystem, hardware proof, networking, and SSH remain outside this
  frontier.

## Accepted Frontier

The accepted execution frontier now includes absolute VFS dispatch for
/bin/init and /bin/zero:

- successful execs open and read executable bytes through the descriptor-backed
  TalosOpen/TalosRead path before program loading.
- the accepted loader, process-install, address-space, materialization,
  initial-stack, launch, lifecycle, and status chain is preserved.
- /bin/init keeps the accepted startup ABI state
  minimal-argc1-argv0-init-empty-envp.
- /bin/zero reports minimal-argc1-argv0-absolute-empty-envp, argc=1,
  argv[0]=/bin/zero, non-null argv state, deterministic empty envp, and
  copied-startup-bytes=0x2a.
- laststatus reports the latest successful lifecycle path and zero status.
- exec /missing, exec init, exec /bin, exec /etc/banner.txt, and exec /empty
  are deterministic negative controls.
- cat /etc/banner.txt remains descriptor-backed.

This does not accept PATH search, relative executable dispatch, multiple
arguments, environment variables, wait/waitpid, descriptor inheritance, pipes,
redirection, writable filesystem, nonzero status variation, Pi 5 hardware
behavior, networking, or SSH.

## Evidence Map

- unit tests: cargo -Zjson-target-spec test --quiet passed with 380 no_std
  tests.
- QEMU/substitute: absolute VFS exec dispatch evidence retained at
  tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log;
  transcript includes exec /bin/zero, argv0=/bin/zero,
  state=minimal-argc1-argv0-absolute-empty-envp, path=/bin/zero, laststatus
  for /bin/zero, /bin/init regression with
  state=minimal-argc1-argv0-init-empty-envp, laststatus for /bin/init,
  deterministic negative controls for /missing, init, /bin, /etc/banner.txt,
  and /empty, descriptor-backed cat /etc/banner.txt, errors=0
  classification=qemu-local-shell-vfs-exec-complete, and PASS.
- QEMU/substitute regression: the same retained dispatch transcript includes
  the required /bin/init, laststatus, and descriptor-backed VFS cat regression
  surfaces.

## Validation

- formatting: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute: scripts/qemu-local-serial-command-loop-smoke.sh with
  TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO=qemu_local_shell_vfs_exec and
  task-specific evidence output passed.
- static inspection: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

hardwareTestLock remained unlocked/restored and unused. No Pi 5 hardware,
power-cycle, or boot archive publication was performed.

Acceptance commit: recorded in durable supervisor state after commit creation.
