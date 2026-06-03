# Phase 10 Process Lifecycle Status Record Core

Task: phase10-process-lifecycle-status-record-core-20260603

Status: accepted

## Goal

Make the shell-visible exec /bin/init status flow through an explicit
kernel-owned lifecycle/status record without broadening into general process
management.

## Scope

- Preserve the accepted VFS/open/read, loader, address-space, materialization,
  stack, launch, and lower-AArch64 SVC status-equivalent lineage for
  exec /bin/init.
- Add a stable lifecycle/status record for the launched fixture with identity,
  shell parent ownership, exited state, zero status, observed status, and
  reaped state.
- Keep exec /missing and exec /etc/banner.txt deterministic negative cases
  that do not create successful lifecycle records.
- Retain QEMU/substitute evidence for lifecycle/status exec and VFS cat
  regression behavior.

## Non-Goals Preserved

No general process table, scheduler-owned runnable process, wait/waitpid,
asynchronous execution, real exit teardown, PATH lookup, argv/envp/auxv/TLS,
descriptor inheritance across exec, pipes, redirection, writable filesystem,
Pi 5 hardware proof, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache policy is accepted by this task.

## Changed Files

- src/local_command_loop.rs
- src/target/qemu_virt.rs
- scripts/qemu-local-serial-command-loop-smoke.sh
- scripts/qemu-local-shell-lifecycle-status-smoke.sh
- docs/src/roadmap.md
- tasks/evidence/2026-06-03-phase10-process-lifecycle-status-record-core/

## Findings And Dispositions

- fixed: exec /bin/init previously reported the decoded status directly from
  the lower-AArch64 SVC status-equivalent boundary. It now creates an explicit
  LocalCommandProcessLifecycleRecord and prints
  exec-status ... source=lifecycle-record.
- fixed: The lifecycle record now carries stable identity
  pid=0x0000000000100001, shell parent ownership owner=0x0000000000000001,
  state=exited, status zero, observed-status=0x0000000000000000, and
  reaped=true.
- fixed: The QEMU target-side shell exec oracle still expected six response
  lines for exec /bin/init; it now requires the lifecycle line by expecting
  seven response lines.
- fixed: The lifecycle smoke wrapper initially tried to use a new harness
  label, but the QEMU boot scenario emits the fixed qemu-local-shell-vfs-exec
  label. The wrapper now keeps that target label while writing retained
  evidence to this task's evidence directory.
- not-an-issue: Missing and non-executable exec targets remain simple
  deterministic negative cases, not lifecycle records.
- deferred: Shell observation of the last lifecycle record is intentionally
  left for phase10-shell-last-process-status-observation-20260603.

## Accepted Boundary

The accepted lifecycle/status boundary is still narrow:

- the shell command is explicitly exec /bin/init;
- /bin/init bytes are sourced through descriptor-backed VFS/open/read;
- loader, process-install, address-space, materialization, stack, and launch
  lineage remain visible in the transcript;
- the lifecycle record wraps the decoded zero status from the accepted
  lower-AArch64 SVC status-equivalent boundary;
- the shell-visible status line says source=lifecycle-record;
- negative exec paths are deterministic and do not report successful
  lifecycle state.

Representative retained output:

talos: exec-lifecycle pid=0x0000000000100001 parent=shell owner=0x0000000000000001 state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true
talos: exec-status boundary=lower-aarch64-svc-status-equivalent marker=0x0000000000007a10 status=0x0000000000000000 complete=true source=lifecycle-record

## Evidence

- Source/unit evidence: cargo -Zjson-target-spec test --quiet passed
  repo-wide with 378 no_std tests.
- Static/typecheck evidence: cargo -Zjson-target-spec check --quiet passed.
- QEMU/substitute lifecycle/status transcript:
  tasks/evidence/2026-06-03-phase10-process-lifecycle-status-record-core/qemu-local-shell-lifecycle-status-smoke.log
  with exec-lifecycle, source=lifecycle-record, deterministic negative exec
  cases, VFS cat regression inside the same transcript, and
  qemu-local-shell-vfs-exec: PASS.
- QEMU/substitute VFS cat regression:
  tasks/evidence/2026-06-03-phase10-process-lifecycle-status-record-core/qemu-local-cat-banner-regression.log
  with Talos initramfs fixture and qemu-local-cat-banner: PASS.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- cargo -Zjson-target-spec check --quiet: passed.
- QEMU/substitute shell lifecycle/status smoke: passed.
- QEMU/substitute VFS cat regression: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --check: passed.
- git diff --cached --check: passed.

HardwareTestLock remained unlocked/restored and unused.

Acceptance commit: recorded in durable supervisor state after commit creation.
