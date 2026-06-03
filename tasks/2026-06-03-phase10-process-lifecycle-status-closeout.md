# Phase 10 Process Lifecycle Status Closeout

Task: phase10-process-lifecycle-status-closeout-20260603

Status: accepted

## Scope

Checkpoint the accepted Phase 10 process lifecycle/status shell behavior after
the lifecycle-record core and shell-visible `laststatus` observation tasks.

This closeout reconciles accepted task records, retained QEMU/substitute
transcripts, deferred surfaces, residual risks, and the next local execution
capability recommended from the evidence. No runtime code changed. No QEMU
scenario was rerun for this checkpoint. No Pi 5 hardware action, boot archive
publication, power-cycle, or hardwareTestLock acquisition was performed.

## Records

- Accepted lifecycle/status record task:
  `tasks/2026-06-03-phase10-process-lifecycle-status-record-core.md`.
- Accepted lifecycle/status record commit:
  `2c15d90a567852b43a39c4c99e0597d5ea63f66d`.
- Retained lifecycle/status transcript:
  `tasks/evidence/2026-06-03-phase10-process-lifecycle-status-record-core/qemu-local-shell-lifecycle-status-smoke.log`.
- Retained lifecycle/status VFS cat regression:
  `tasks/evidence/2026-06-03-phase10-process-lifecycle-status-record-core/qemu-local-cat-banner-regression.log`.
- Accepted shell last-process status observation task:
  `tasks/2026-06-03-phase10-shell-last-process-status-observation.md`.
- Accepted shell last-process status observation commit:
  `6ac36195af5c5cd86c1badc6e616ae4f1d8e377e`.
- Retained shell last-process-status transcript:
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`.
- Retained shell last-process-status VFS cat regression:
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-cat-banner-regression.log`.

## Findings And Dispositions

- fixed: `exec /bin/init` no longer reports status only as an immediate
  decoded lower-AArch64 SVC observation. The accepted lifecycle/status record
  task routes that zero status through a kernel-owned lifecycle record with
  pid, shell parent ownership, exited state, observed status, and reaped state.
- fixed: `laststatus` provides a separate shell-visible observation of the
  most recent lifecycle record and reports the same pid and zero status as the
  preceding `exec /bin/init` transcript.
- fixed: No-prior-process observation is deterministic:
  `talos: last-process none`.
- fixed: Missing and non-executable exec targets remain deterministic negative
  cases and do not create successful lifecycle records.
- not-an-issue: `laststatus` is still a kernel command-loop surface, but its
  accepted output is backed by the lifecycle/status record instead of a fake
  command fixture or unrelated canned shell expansion.
- not-an-issue: Pi 5 hardware was not run. The accepted frontier here is
  QEMU/substitute evidence for generic local shell/VFS/userspace/lifecycle
  behavior; no physical board behavior is claimed.
- deferred: General wait/waitpid, asynchronous execution, multiple children,
  scheduler-owned process lifetime, real exit teardown, zombie policy beyond
  the single retained record, argv/envp/auxv/TLS, PATH lookup, pipes,
  redirection, writable filesystem, hardware proof, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache policy remain outside the
  accepted frontier.

## Accepted Frontier

The accepted shell-visible behavior now backed by explicit lifecycle/status
records is:

- `exec /bin/init` opens and reads the executable through the descriptor-backed
  VFS/open/read path, parses it with the program loader, derives the accepted
  process installation/address-space/materialization/initial-stack/launch
  records, and observes the accepted lower-AArch64 SVC status-equivalent
  boundary.
- The lower-AArch64 SVC status-equivalent zero status is captured in a
  kernel-owned lifecycle/status record with identity
  `pid=0x0000000000100001`, parent `shell`,
  `owner=0x0000000000000001`, `state=exited`,
  `status=0x0000000000000000`,
  `observed-status=0x0000000000000000`, and `reaped=true`.
- `laststatus` reports that latest lifecycle/status record with
  `source=lifecycle-record`, preserving the same identity and zero status.
- `cat /etc/banner.txt` remains a descriptor-backed VFS/open/read regression
  surface in the retained transcripts.
- `exec /missing` and `exec /etc/banner.txt` remain negative controls and do
  not claim process launch or lifecycle success.

This is deliberately narrower than Unix process management. It is not
`waitpid`, asynchronous child execution, a scheduler process table, process
replacement, descriptor inheritance across exec, PATH lookup, argv/envp setup,
an interactive userspace shell, arbitrary executable support, or Pi 5 proof.

## Evidence Map

- static inspection: lifecycle/status task record
  `tasks/2026-06-03-phase10-process-lifecycle-status-record-core.md`.
- static inspection: shell last-process-status task record
  `tasks/2026-06-03-phase10-shell-last-process-status-observation.md`.
- QEMU/substitute: lifecycle/status smoke retained under
  `tasks/evidence/2026-06-03-phase10-process-lifecycle-status-record-core/qemu-local-shell-lifecycle-status-smoke.log`.
- QEMU/substitute: last-process-status smoke retained under
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`.
- QEMU/substitute: VFS cat regressions retained under both lifecycle/status
  evidence directories listed above.
- static inspection: recent commits include `2c15d90` for the lifecycle record
  and `6ac3619` for the `laststatus` observation.

## Next Planning Recommendation

The evidence supports supervisor planning for the next smallest local execution
capability: a minimal argv/argc userspace ABI slice for the existing
`exec /bin/init` path. A mechanically promotable task should keep the same
explicit `/bin/init` fixture, pass only `argc=1` and `argv[0]="/bin/init"`
through the accepted initial-stack/userspace-memory boundary, and retain a
shell-visible transcript proving that the observed userspace result is backed
by that ABI state.

That recommendation is intentionally before PATH lookup, arbitrary executable
dispatch, pipes, redirection, writable filesystem, networking, SSH, or Pi 5
proof. The worker must not create or promote it until the supervisor records an
explicit queued task with dependencies, acceptance criteria, validation gates,
docs, and evidence requirements.

## Validation

- static inspection: accepted lifecycle/status records, retained
  QEMU/substitute evidence, VFS cat regressions, and recent commits were
  inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
