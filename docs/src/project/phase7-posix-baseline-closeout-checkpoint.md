# Phase 7 POSIX Baseline Closeout Checkpoint

Status: accepted as the Phase 7.1 POSIX baseline closeout checkpoint. This
document reconciles the accepted contract, path/error model, descriptor-table
contract, and descriptor-table core evidence before any EL0, SVC/syscall ABI,
VFS, filesystem, program-loader, descriptor I/O, networking, SSH, or shell work
starts.

This checkpoint does not add Rust implementation beyond the already accepted
target-independent Phase 7.1 cores. It does not add a boot scenario, QEMU proof,
Pi 5 hardware proof, lower-EL entry, syscall ABI, runtime console/TTY
descriptor I/O, VFS lookup, filesystem object, program loader, pipe, socket,
shell behavior, RP1/PCIe work, UART interrupt ownership, or DMA/cache-driver
policy.

## Accepted Work

- Phase 7 POSIX contract source inventory:
  docs/src/project/phase7-posix-contract-source-inventory.md maps the accepted
  scheduler task/process separation, runtime-console and TTY stdio direction,
  diagnostic command-channel limits, lower-EL readiness limits, and retained
  Phase 4 through Phase 6 gates.
- Phase 7 POSIX contract baseline:
  docs/src/project/phase7-posix-contract-baseline.md defines errno-style names,
  lexical path normalization semantics, process lifetime vocabulary,
  descriptor vocabulary, stdio inheritance shape, and early loader argument and
  environment vocabulary.
- Phase 7 path/error model core:
  src/posix.rs implements the first no_std PosixError vocabulary and
  allocation-free lexical path normalizer. The task record is
  tasks/2026-05-28-phase7-path-error-model-core.md.
- Phase 7 descriptor-table contract:
  docs/src/project/phase7-descriptor-table-contract.md narrows process-local
  descriptor table invariants, close and dup semantics, inherited stdio, and
  deterministic descriptor errors.
- Phase 7 descriptor-table core:
  src/posix.rs implements the first target-independent fixed-capacity
  descriptor table data model. It covers inherited stdio, allocation,
  exact-slot allocation, lookup, close, dup, access checks, TTY-only checks,
  reserved object kinds, and deterministic PosixError results. The task record
  is tasks/2026-05-28-phase7-descriptor-table-core.md.

## Evidence Summary

- static inspection: Phase 7.1 documentation separates diagnostic commands from
  shell, syscall, program-loader, VFS, and descriptor I/O semantics.
- static inspection: lower-EL documentation still treats the accepted EL2
  identity map as a kernel bring-up map, not as userspace isolation or syscall
  readiness.
- static inspection: the accepted Rust code is contained in target-independent
  POSIX primitives under src/posix.rs and is wired through src/main.rs only as a
  module.
- unit tests: path/error model acceptance recorded
  `cargo -Zjson-target-spec test` passing with 172 no_std tests, including 16
  POSIX path/error tests.
- unit tests: descriptor-table core acceptance recorded
  `cargo -Zjson-target-spec test` passing with 183 no_std tests, including 11
  descriptor-table tests.
- documentation: the accepted contract and closeout documents are part of the
  mdBook summary.
- QEMU/hardware: Phase 7.1 made no QEMU boot, boot-image, or Pi 5 hardware
  claim. The most recent hardware frontier remains the accepted Phase 6.3
  production timer/preemption proof.

## Retained Gates

Target-independent Rust work after this checkpoint should keep using:

- `cargo fmt --all -- --check`;
- `cargo -Zjson-target-spec test`;
- `git diff --check`;
- `mdbook build` when documentation is touched.

Runtime or boot-path tasks must explicitly choose additional gates rather than
inheriting them by implication. Existing retained runtime gates remain
available for explicit future tasks:

- `scripts/qemu-smoke.sh`;
- `scripts/qemu-timer-preemption-smoke.sh`;
- `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`;
- `scripts/qemu-shared-runqueue-migration-smoke.sh`;
- `scripts/qemu-load-balancing-smoke.sh`;
- `scripts/qemu-multicore-preemption-smoke.sh`;
- `scripts/qemu-production-timer-preemption-smoke.sh`;
- serialized Pi 5 reproduction helpers only for tasks with explicit hardware
  acceptance criteria and hardwareTestLock ownership.

## Remaining Deferrals And Risks

- There is no PID allocator, process table, parent/child storage, exit status,
  wait queue, signal state, credential model, process group, session model, or
  controlling TTY ownership.
- There is no process-owned address space, lower-EL vector routing, EL0 return
  path, user stack, user heap, user code mapping, copy-in/copy-out helper, or
  bad-user-pointer validation.
- There is no SVC/syscall ABI or numeric errno return convention.
- Descriptor I/O is not implemented. The descriptor-table core does not route
  bytes to runtime-console0, TTY input, VFS, filesystems, pipes, sockets, or
  devices.
- The diagnostic command channel remains kernel-owned diagnostics, not a shell,
  syscall path, program namespace, filesystem command interface, or loader.
- There is no filesystem, program loader, local shell, networking, SSH,
  RP1/PCIe work, UART interrupt ownership, or DMA/cache-driver policy.

## Next Recommendation

The next supervisor-planned task should be a Phase 7.2 EL0 trap path and user
address-space source inventory. It should reconcile the accepted lower-EL
readiness document, exception-vector constraints, memory-map permissions,
scheduler task/process separation, PosixError return vocabulary, and
descriptor-table ownership before any implementation starts.

That task should remain an inventory or contract unless the supervisor creates
explicit implementation acceptance criteria. It should not start SVC/syscall
ABI, VFS, filesystem, program loading, descriptor I/O, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Validation

- static inspection: git status --short was clean before edits.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- unit tests: cargo -Zjson-target-spec test passed with 183 no_std tests,
  because this checkpoint reconciles accepted Rust implementation tasks.
