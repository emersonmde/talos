# Phase 10 Read-Only Regular-File Stdin Redirection Core

Task: phase10-readonly-regular-file-stdin-redirection-core-20260604
Status: accepted

## Scope

Accept exactly one read-only regular-file input-redirection form:
`exec stdin </etc/banner.txt`. The launched VFS-backed `/bin/stdin` child gets
fd0 rebound to a read-only initramfs regular-file descriptor for
`/etc/banner.txt`; the shell restores fd0 after the child exits.

This is not output-file redirection, writable filesystem behavior, append or
truncate semantics, arbitrary path expansion, shorthand syntax, or broader
descriptor grammar. The accepted surface is intentionally one regular-file
source backed by the existing descriptor-backed initramfs/VFS/open/read path.

## Findings

- fixed: Added `StdinFromEtcBanner` as the exact `</etc/banner.txt` fd0
  source form for the VFS-backed `/bin/stdin` fixture. The parser accepts only
  the no-space token and execution restricts the redirection to `/bin/stdin`.
- fixed: Bumped the local shell read-only file-description capacity from one
  to two so child fd0 can hold the regular-file source while the loader opens
  the executable through its temporary descriptor.
- fixed: The redirection path closes fd0, opens `/etc/banner.txt` through the
  existing `TalosOpen` initramfs syscall path, requires it to occupy fd0, then
  restores the original shell fd0 after the child exits and removes the file
  description.
- fixed: The child descriptor table reports `fd0=regular-file`; the
  redirection record reports `op=source`, `source-path=/etc/banner.txt`,
  `source-stream=regular-file`, and `source-route=initramfs:/etc/banner.txt`.
- fixed: The stdin fixture reads the banner through `TalosRead`, verifies the
  exact fixture bytes, records `read-source=initramfs:/etc/banner.txt`, and
  checks EOF after the read with `read-result=regular-file-eof-after-read`.
- fixed: A following normal `exec stdin` control consumes `talos-console0`,
  proving shell fd0 restoration. The task smoke also retains `/dev/null` stdin,
  unsupported stdout input-redirection, shorthand-negative, and
  descriptor-backed `cat /etc/banner.txt` controls.
- fixed: Added no_std unit coverage, a dedicated QEMU/substitute wrapper,
  kernel boot scenario label/classification, expected dispatch rows, and
  task-owned evidence.
- not-an-issue: The local shell read-only file table remains deliberately tiny;
  two slots are sufficient for this feature because only fd0 and the loader
  temporary descriptor are live at the same time.
- deferred: output regular-file redirection, append/truncate, writable
  filesystem mutation, arbitrary descriptor syntax, arbitrary path expansion,
  here-docs, broader pipes, Pi 5 proof, networking, SSH, and a phase
  transition.

## Evidence

- QEMU/substitute task smoke:
  `tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log`
  records command 3 `exec stdin </etc/banner.txt`, `fd0=regular-file`,
  `exec-redirection op=source ... source-path=/etc/banner.txt ...
  source-stream=regular-file source-route=initramfs:/etc/banner.txt`,
  `exec-stdin ... bytes=0x18 return=0x18
  read-source=initramfs:/etc/banner.txt ...
  read-result=regular-file-eof-after-read`, `waitpid`, `laststatus`, normal
  `exec stdin` restoration through runtime-console0/local-input, retained
  `exec stdin </dev/null`, deterministic negative forms, descriptor-backed
  `cat /etc/banner.txt`, final
  `qemu-local-shell-readonly-regular-file-stdin-redirection-complete`, and PASS.
- Retained /dev/null stdin source evidence:
  `tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log`.
- Retained runtime-console0 stdin and EOF/readiness controls:
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-stdin-eof-no-data-core/qemu-local-shell-stdin-eof-no-data-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
- Retained descriptor redirection, pipeline, waitpid/laststatus, and cat
  controls:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`,
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: focused `cargo -Zjson-target-spec test --quiet local_command_loop`
  passed 405 no_std tests, including the new regular-file stdin redirection
  unit test.
- QEMU/substitute: task-owned read-only regular-file stdin redirection smoke
  passed with retained PASS log.
- full unit tests: `cargo -Zjson-target-spec test --quiet` passed 405 no_std
  tests.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed after the roadmap update.
- staged diff check: `git diff --cached --check` passed.

## Commit

Commit: accepted implementation and evidence committed; final SHA recorded in
durable supervisor state.
