# Phase 10 Userspace Stderr Through Inherited FD Core

Task: phase10-userspace-stderr-inherited-fd-core-20260603

Status: accepted

## Scope

Implement the narrow userspace stderr-through-inherited-fd2 feature. The task adds one read-only `/bin/stderr` initramfs executable fixture, routes it through the accepted fixed `/bin` lookup and VFS/open/read program-loader path, performs one bounded userspace `TalosWrite` through inherited `fd2=stdio-output`, and records the byte count/return value without adding pipes, redirection, terminal policy, blocking I/O, writable filesystem behavior, hardware proof, networking, or SSH.

## Findings And Dispositions

- fixed: Added `/bin/stderr` to the read-only initramfs fixture and fixed `/bin` listing, with immutable ELF bytes matching the accepted lower-AArch64 zero-status SVC fixture shape.
- fixed: Added the shell exec allow-list entry for `/bin/stderr`, preserving deterministic rejection for directories, empty files, non-executable regular files, relative paths containing `/`, missing paths, and unsupported literal argv grammar.
- fixed: Added a launched-process stderr record. `exec stderr` performs one process descriptor `TalosWrite` through inherited `fd2`, emits `Talos userspace stderr fixture`, and records `exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f source=userspace-talos-write`.
- fixed: Retained the accepted descriptor, VFS, loader, startup, launch, lifecycle/status, consuming `waitpid`, non-consuming `laststatus`, status42, init/zero, negative exec, and descriptor-backed cat evidence chain.
- fixed: Added the task-specific `scripts/qemu-local-shell-userspace-stderr-smoke.sh` scenario and QEMU expected-dispatch entries for `qemu_local_shell_stderr`.
- fixed: Updated the shared local shell smoke expectations for the expanded `/bin` listing after adding `stderr`.
- not-an-issue: `fd2` currently shares the accepted `stdio-output` backend with `fd1`. This task accepts descriptor identity and process-originated fd2 writes, not stdout/stderr stream separation.
- deferred: Distinct stderr stream routing, pipes, redirection, terminal policy, blocking scheduler I/O, async execution, signals, writable filesystem behavior, libc stdio, runtime-console0/TTY-backed stdin expansion, hardware proof, networking, and SSH remain outside this frontier.

## Evidence Map

- QEMU/substitute userspace stderr transcript: `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`. It records `exec stderr`, visible process-originated stderr fixture bytes, resolved `/bin/stderr`, descriptor-backed `source=vfs-open-read`, inherited `fd0=stdio-input`, `fd1=stdio-output`, `fd2=stdio-output`, `loader-temp-open=false`, deterministic empty envp, argv0 `/bin/stderr`, `exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f return=0x000000000000001f source=userspace-talos-write`, zero lifecycle status, matching `waitpid` and `laststatus`, `/bin/status42` nonzero regression, `/bin/init` and `/bin/zero` zero-status controls, deterministic negative exec cases, descriptor-backed `cat /etc/banner.txt`, final participants=18 expected=18 errors=0, classification=`qemu-local-shell-userspace-stderr-complete`, and PASS.
- QEMU/substitute userspace stdout regression transcript: `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`. Rerun after the stderr change; it preserves `exec stdout`, `exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f return=0x000000000000001f source=userspace-talos-write`, lifecycle, `waitpid`, `laststatus`, negative controls, descriptor-backed cat, classification=`qemu-local-shell-userspace-stdout-complete`, and PASS.
- QEMU/substitute userspace stdin regression transcript: `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`. Rerun after the stderr change; it preserves `exec stdin`, `exec-stdin fd=0x0000000000000000 bytes=0x000000000000000a return=0x000000000000000a stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000002f stdout-return=0x000000000000002f source=userspace-talos-read+userspace-talos-write`, lifecycle, `waitpid`, `laststatus`, retained stdout fixture, negative controls, descriptor-backed cat, classification=`qemu-local-shell-userspace-stdin-complete`, and PASS.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed, 388 tests.
- QEMU/substitute: `scripts/qemu-local-shell-userspace-stderr-smoke.sh --quiet` passed and retained the stderr evidence log.
- QEMU/substitute regression: `scripts/qemu-local-shell-userspace-stdout-smoke.sh --quiet` passed and refreshed retained stdout evidence.
- QEMU/substitute regression: `scripts/qemu-local-shell-userspace-stdin-smoke.sh --quiet` passed and refreshed retained stdin evidence.
- static inspection: `bash -n scripts/qemu-local-serial-command-loop-smoke.sh scripts/qemu-local-shell-userspace-stderr-smoke.sh` passed.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before commit.

## Next Action

Promote `phase10-userspace-stderr-inherited-fd-closeout-20260603` after this task is accepted and committed. That closeout should reconcile that fd2 writes are accepted through a process descriptor operation while distinct stderr stream separation, pipes, redirection, blocking I/O, writable filesystem behavior, hardware proof, networking, and SSH remain deferred.
