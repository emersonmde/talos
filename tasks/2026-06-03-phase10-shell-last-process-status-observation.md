# Phase 10 Shell Last Process Status Observation

Task: phase10-shell-last-process-status-observation-20260603

Status: accepted

## Goal

Expose the most recent VFS-launched process status through a shell-visible
observation backed by the accepted lifecycle/status record.

## Scope

- Add the smallest shell-visible observation command, `laststatus`, for the
  last `exec /bin/init` lifecycle record.
- Preserve the accepted VFS/open/read, loader, launch, lifecycle, and
  lower-AArch64 SVC status-equivalent lineage for `exec /bin/init`.
- Keep `exec /missing`, `exec /etc/banner.txt`, `cat /etc/banner.txt`, and
  ready-prompt behavior deterministic.
- Record explicit no-prior-process behavior.

## Non-Goals Preserved

No waitpid ABI, blocking wait, multiple-child management, asynchronous process
execution, zombie-retention policy beyond the single retained lifecycle record,
job control, signals, PATH lookup, argv/envp/auxv/TLS, descriptor inheritance
changes, pipes, redirection, writable filesystem, Pi 5 proof, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache policy is accepted by this
task.

## Changed Files

- `src/local_command_loop.rs`
- `src/target/qemu_virt.rs`
- `scripts/qemu-local-serial-command-loop-smoke.sh`
- `scripts/qemu-local-shell-last-process-status-smoke.sh`
- `docs/src/roadmap.md`
- `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`
- `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-cat-banner-regression.log`

## Findings And Dispositions

- fixed: The shell had an explicit lifecycle/status record but no separate
  shell-visible way to observe the last record after `exec /bin/init`. The
  new `laststatus` command reports the same pid, shell owner, exited state,
  status, observed-status, and reaped state from the lifecycle record.
- fixed: No-prior-process observation is deterministic:
  `talos: last-process none`.
- fixed: The QEMU shell exec/status transcript now includes `laststatus`
  immediately after `exec /bin/init`, while retaining the negative exec cases
  and VFS cat regression inside the same boot scenario.
- fixed: The local command-loop test capture sink was too small for a
  three-command lifecycle transcript; its fixed test buffer is now larger.
- not-an-issue: `laststatus` is still a kernel command surface, but its output
  is directly backed by the accepted lifecycle/status record rather than a fake
  shell expansion.
- deferred: Already-observed/reaped behavior does not mutate state yet; this
  boundary reports the current accepted `reaped=true` record. General wait
  semantics and zombie policy remain future work.

## Accepted Boundary

The accepted shell-visible status observation is intentionally narrow:

- `exec /bin/init` still reads `/bin/init` through descriptor-backed
  VFS/open/read;
- loader, process-install, address-space, materialization, stack, launch, and
  lower-AArch64 SVC status-equivalent lineage remain visible;
- `laststatus` reports the latest lifecycle record with the same identity and
  zero status;
- `laststatus` with no prior process reports `talos: last-process none`;
- missing and non-executable exec targets remain deterministic negative cases
  and do not create successful lifecycle records.

Representative retained output:

```text
talos> exec /bin/init
talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true
talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record
talos> laststatus
talos: last-process pid=0x0000000000100001 parent=shell owner=0x0000000000000001 state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record
```

## Evidence

- Source/unit evidence: `cargo -Zjson-target-spec test --quiet` passed
  repo-wide with 379 no_std tests.
- Typecheck evidence: `cargo -Zjson-target-spec check --quiet` passed.
- QEMU/substitute shell last-process-status transcript:
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`
  includes `exec /bin/init`, `laststatus`, matching lifecycle identity and
  status, deterministic negative exec cases, VFS cat regression, final
  `qemu-local-shell-vfs-exec-complete`, and `qemu-local-shell-vfs-exec: PASS`.
- QEMU/substitute VFS cat regression:
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-cat-banner-regression.log`
  was retained after this change and ends with `qemu-local-cat-banner: PASS`.

## Validation

- `cargo fmt --all -- --check`: passed.
- `cargo -Zjson-target-spec check --quiet`: passed.
- `cargo -Zjson-target-spec test --quiet`: passed.
- QEMU/substitute shell last-process-status smoke: passed.
- QEMU/substitute VFS cat regression: passed.
- `/home/node/.cargo/bin/mdbook build`: passed.
- `git diff --check`: passed.
- `git diff --cached --check`: passed before commit.

hardwareTestLock remained unlocked/restored and unused.

Acceptance commit: recorded in durable supervisor state after commit creation.
