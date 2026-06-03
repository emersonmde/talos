# Phase 10 VFS Exec Nonzero Status Core

Task: phase10-vfs-exec-nonzero-status-core-20260603

Status: accepted

## Scope

Prove shell-visible lifecycle/status reporting varies with the executed
VFS-backed userspace image by adding the smallest absolute initramfs
executable fixture that exits with a deterministic nonzero lower-AArch64 SVC
status.

This task keeps execution on the accepted descriptor-backed VFS/open/read,
program-loader, process-install, address-space, materialization,
initial-stack, launch, lifecycle, and status path. It does not add PATH lookup,
relative command dispatch, broad argv/envp, auxv/TLS, libc startup, descriptor
inheritance, wait/waitpid, asynchronous execution, multiple children, pipes,
redirection, writable filesystem, Pi 5 proof, networking, or SSH.

## Findings And Dispositions

- fixed: Added /bin/status42 as a read-only initramfs regular file under /bin,
  backed by the same static ELF64/AArch64 fixture generator as /bin/init and
  /bin/zero, with x0=42 before the accepted svc #0x7a10 marker.
- fixed: Absolute shell exec /bin/status42 now dispatches through the accepted
  descriptor-backed VFS/open/read and loader/process/launch/startup/lifecycle/
  status chain.
- fixed: The shell-visible nonzero transcript reports
  state=minimal-argc1-argv0-absolute-empty-envp, argc=1,
  argv0=/bin/status42, deterministic empty envp, and
  copied-startup-bytes=0x2e.
- fixed: The lifecycle record and exec-status line report the same nonzero
  0x2a status derived from the launched VFS-backed executable bytes.
- fixed: laststatus after exec /bin/status42 reports the same latest lifecycle
  identity and nonzero status.
- fixed: /bin/init and /bin/zero remain zero-status controls through the
  accepted absolute VFS exec path.
- fixed: Missing paths, relative/PATH-style names, directories, non-ELF
  regular files, and empty files still fail deterministically without creating
  successful lifecycle records.
- fixed: Descriptor-backed cat /etc/banner.txt remains covered by the same
  retained QEMU/substitute transcript.
- fixed: /bin listings now include status42 along with init and zero, and the
  affected unit/QEMU expectations were updated.
- fixed: The task-specific smoke wrapper now keeps the compile-time
  qemu_local_shell_vfs_exec label/classification so serial command injection
  matches the scenario ready markers.
- deferred: PATH lookup, broad argv/envp, auxv/TLS, libc startup, descriptor
  inheritance, wait/waitpid, asynchronous execution, multiple children,
  process replacement, pipes, redirection, writable filesystem, hardware
  proof, networking, and SSH remain outside this frontier.

## Accepted Frontier

The accepted execution/status frontier now includes status variation from a
VFS-backed userspace image:

- /bin/status42 is a read-only initramfs executable fixture whose text sets
  the lower-AArch64 SVC status-equivalent value to 0x2a.
- successful exec /bin/status42 opens and reads executable bytes through the
  descriptor-backed TalosOpen/TalosRead path before program loading.
- the accepted loader, process-install, address-space, materialization,
  initial-stack, launch, lifecycle, and status chain is preserved.
- the lifecycle record, exec-status, and following laststatus agree on path
  /bin/status42 and status 0x2a.
- /bin/init and /bin/zero remain zero-status controls through the same
  accepted absolute VFS exec path.
- exec /missing, exec init, exec /bin, exec /etc/banner.txt, and exec /empty
  remain deterministic negative controls.
- cat /etc/banner.txt remains descriptor-backed.

This does not accept PATH search, relative executable dispatch, multiple
arguments, environment variables, wait/waitpid, descriptor inheritance, pipes,
redirection, writable filesystem, Pi 5 hardware behavior, networking, or SSH.

## Evidence Map

- unit tests: cargo -Zjson-target-spec test --quiet passed with 381 no_std
  tests.
- QEMU/substitute: nonzero VFS exec status evidence retained at
  tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log;
  transcript includes exec /bin/status42, argv0=/bin/status42,
  state=minimal-argc1-argv0-absolute-empty-envp, lifecycle/status 0x2a,
  matching laststatus, zero-status /bin/init and /bin/zero controls,
  deterministic negative controls for /missing, init, /bin, /etc/banner.txt,
  and /empty, descriptor-backed cat /etc/banner.txt, final participants=15
  expected=15 errors=0, classification=qemu-local-shell-vfs-exec-complete,
  and PASS.
- QEMU/substitute regression: the same retained transcript includes the
  required zero-status /bin/init, latest laststatus, absolute dispatch
  negative controls, and descriptor-backed VFS cat regression surfaces.

## Validation

- formatting: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute: scripts/qemu-local-shell-nonzero-vfs-exec-status-smoke.sh
  passed and retained the evidence log above.
- static inspection: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

hardwareTestLock remained unlocked/restored and unused. No Pi 5 hardware,
power-cycle, or boot archive publication was performed.

Acceptance commit: recorded in durable supervisor state after commit creation.
