# Phase 7 Read And Stdin Closeout Checkpoint

Status: accepted as the documentation-only Milestone 7.4 read/stdin QEMU
frontier closeout after the accepted
[Phase 7 QEMU Read And Stdin Smoke Plan](phase7-qemu-read-stdin-smoke-plan.md)
and retained QEMU/substitute evidence from
\`phase7-qemu-read-stdin-smoke-core-20260529\`.

This checkpoint adds no Rust behavior, assembly behavior, QEMU rerun, Pi 5
hardware run, boot archive publication, hardwareTestLock acquisition,
implementation change, process loading, VFS/filesystem behavior, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, object
finalization, or DMA/cache-driver policy.

## Accepted Frontier

The read/stdin slice now has four accepted layers:

- source inventory: syscall dispatch, copy_to_user/user-memory validation,
  ProcessDescriptorStore lookup, inherited fd 0, runtime-console0/TTY source
  owners, and retained write/close/dup evidence are mapped without accepting
  read behavior;
- contract: stable talos_read syscall number 4 uses x0 fd, x1 destination,
  x2 count, x3 through x5 reserved zero, and x0 byte-count/0 EOF or negative
  errno return encoding;
- target-independent core: FixedStdin proof-buffer state, fd 0 and duplicated
  StdioInput reads, short-read, EOF, copy-out failure ordering, descriptor
  errors, reserved-register rejection, and scalar/write/close/dup regressions
  pass in no_std tests;
- QEMU/substitute smoke: qemu_read_stdin_smoke proves the lower-AArch64 svc #0
  path through the current ProcessOwnerId-backed ProcessDescriptorStore,
  fixed proof stdin, copy_to_user(), descriptor regressions, quarantine checks,
  final classification, and PASS.

The retained QEMU log is
\`tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log\`.
It contains:

- fd 0 duplicated to fd 3 through the process-owned descriptor table;
- copy-out guard failure returning -EFAULT without consuming stdin;
- reserved x3 returning -EINVAL without user-memory or cursor mutation;
- fd 1 and fd 99 returning -EBADF without mutation;
- fd 0 reading \`talos\` from fixed proof stdin;
- duplicated fd 3 reading the remaining \`-stdin-qemu\\n\` as a bounded short
  read;
- EOF returning 0 with the proof cursor unchanged;
- talos_nop and unknown-syscall regressions;
- talos_copy_probe quarantine as -ENOSYS in the read smoke;
- diagnostic marker 0x7a10 remaining proof-only;
- \`classification=qemu-read-stdin-smoke-complete\` and
  \`qemu-read-stdin-smoke: PASS\`.

## Validation Evidence

The accepted read/stdin core and QEMU smoke evidence includes:

- fmt/lint/typecheck: \`cargo fmt --all -- --check\` passed;
- unit tests: \`cargo -Zjson-target-spec test\` passed with 248 no_std tests;
- QEMU/substitute: \`scripts/qemu-read-stdin-smoke.sh\` passed and retained the
  log path above;
- QEMU/substitute regressions: \`scripts/qemu-syscall-smoke.sh\`,
  \`scripts/qemu-descriptor-write-smoke.sh\`,
  \`scripts/qemu-close-syscall-smoke.sh\`, and
  \`scripts/qemu-dup-syscall-smoke.sh\` passed;
- static inspection: \`git diff --check\` passed;
- documentation: \`mdbook build\` passed;
- staged whitespace inspection: \`git diff --cached --check\` passed before the
  accepted smoke-core commit.

An optional \`scripts/qemu-pointer-copy-smoke.sh\` run attempted during the
smoke core failed to compile in an unrelated pre-existing
\`src/target/rpi5.rs\` pointer-copy finish path before QEMU execution. It was
not an acceptance gate for the read/stdin smoke core and does not change this
checkpoint's accepted frontier.

## Residual Risks

This closeout accepts QEMU/substitute lower-AArch64 evidence only. It does not
prove physical Pi 5 read/stdin behavior. It also does not prove:

- runtime-console0-backed stdin, TTY raw/canonical input, hardware UART input,
  pipes, sockets, regular files, directories, or filesystem-backed reads;
- blocking, readiness, nonblocking flags, poll/select, wait queues, signal
  restart, Ctrl-C/Ctrl-D terminal behavior, foreground process groups, or
  terminal sessions;
- process loading, fork/spawn/exec, descriptor inheritance across exec,
  close-on-exec application, process exit teardown, open-file-description
  reference counting, or object finalizers;
- per-thread errno storage, demand paging, recoverable lower-EL data-abort
  copy tables, partial user copies on EFAULT, or process-fatal user-fault
  policy;
- shell behavior, libc/Rust std stdio, networking, SSH, RP1/PCIe, UART
  interrupt ownership, DMA/cache-driver policy, or full POSIX descriptor
  readiness.

The accepted read/stdin behavior remains bounded to fixed proof input supplied
by the kernel/QEMU substitute scenario and target-independent tests.

## Next Task

The next mechanically derivable task should be
\`phase7-pi5-read-stdin-proof-plan-20260530\`, documentation-only.

That plan should define one serialized Pi 5 proof for the already accepted
talos_read/fixed-stdin invariant, including candidate identity, archive and
TFTP evidence, fresh serial cursor, exact expected fd 0/fd 3/error/quarantine
classification/PASS lines, known-good control and unchanged-candidate rerun
rules for inconclusive boots, restore proof, and hardwareTestLock ownership.
It should not run hardware, publish an archive, or broaden read/stdin into
runtime-console0, TTY, filesystem, pipe, socket, shell, networking, RP1/PCIe,
UART interrupt, object finalization, or DMA/cache-driver work.

## Validation

- static inspection: reviewed the accepted read/stdin source inventory,
  contract, target-independent core task, QEMU smoke plan, QEMU smoke core task
  record, retained evidence path, roadmap, and decision log.
- static documentation diff: added this closeout checkpoint, linked it from
  SUMMARY, updated roadmap current status, updated the decision log, and added
  the task record.
- whitespace inspection: \`git diff --check\` passed.
- documentation: \`mdbook build\` passed.
- staged whitespace inspection: \`git diff --cached --check\` passed before
  commit.
- Rust fmt/tests, QEMU runs, Pi 5 hardware runs, archive publication, and
  hardwareTestLock acquisition were not required because this task changes
  only Markdown documentation and durable worker state.
