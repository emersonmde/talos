# Phase 7 POSIX Contract Source Inventory

Status: accepted as a Phase 7.1 source inventory before the first POSIX
baseline contract. This document changes documentation only. It does not add
Rust implementation, boot scenarios, QEMU runs, Pi 5 hardware runs, EL0 entry,
SVC/syscall ABI, descriptor tables, VFS, filesystem objects, program loading,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

This inventory follows the accepted Phase 6 production scheduler runtime
closeout. It maps the accepted Phase 4 through Phase 6 surfaces that constrain
the first POSIX contract baseline so the next task can define errno, path,
process, descriptor, and loader vocabulary without treating diagnostic
shortcuts as kernel/user interfaces.

## Accepted Constraints

### Scheduler Task And Process Separation

The accepted scheduler model in src/scheduler.rs and
docs/src/architecture/scheduler.md schedules tasks, not POSIX processes. TaskId
is scheduler-local and rejects zero; it is not a process ID.
Task::kernel_thread creates a kernel-thread task with no process owner, and
ProcessOwnerId remains an optional future extension point only.

The following scheduler facts constrain Phase 7.1:

- per-task kernel stack and saved register state belong to a task, not to a
  future process;
- process-owned resources such as address spaces, descriptor tables, current
  working directory, credentials, children, signals, exit status, and wait
  state remain deferred;
- wakeups target scheduler tasks, not POSIX processes;
- scheduler diagnostics may use labels or task IDs, but must not define PID,
  parent PID, process group, session, signal, or shell command semantics;
- SharedSchedulerMetadata publishes advisory task snapshots with optional
  process-owner placeholders, but it is not a global process table;
- SharedRunQueue and LoadBalancingPolicy move only runnable, non-current tasks
  through owner-transfer boundaries, not running processes or process
  resources.

The POSIX baseline can reference these names as implementation constraints, but
it must define process lifetime and process-owned resources separately before
any user process, wait, exec, descriptor inheritance, or current-working-
directory behavior is implemented.

### Runtime Console, TTY, And Stdio Direction

The accepted Phase 5 console model names runtime-console0 as the default
runtime console identity for normal kernel diagnostics. Target modules own the
physical PL011 backends for QEMU and Pi 5, while runtime_console owns the
internal console-facing write and polling input result contracts.

The accepted TTY shape sits above the runtime console backend. The current TTY
and line discipline provide raw/canonical-lite input behavior, newline and
backspace/delete handling, echo policy, bounded lines, and control-event
classification for diagnostics. Those contracts deliberately do not allocate
file descriptors, sleep scheduler tasks, define syscall results, or own process
resources.

The first POSIX baseline should preserve this direction:

- stdin, stdout, and stderr become process-local descriptor-facing handles
  later;
- initial stdout and stderr descriptors should attach to runtime-console0
  through descriptor-owned handles rather than calling QEMU or Pi 5 target
  backends directly;
- initial stdin should attach to the accepted console input side only after
  descriptor lifetime, readiness or blocking policy, and scheduler sleep/wakeup
  policy exist;
- console and TTY internal result names are kernel contracts, not errno values
  or userspace ABI.

### Diagnostic Command Channel Limitations

The accepted diagnostic command channel consumes complete TTY lines and writes
bounded responses through runtime-console0. It is kernel-owned,
target-independent, and intentionally small. It proves help/list/status and
unknown-command behavior over the accepted polling TTY path on QEMU and Pi 5.

The Phase 7.1 POSIX baseline must explicitly reject treating this surface as:

- a shell grammar;
- a syscall path;
- a program loader;
- a process spawning or exec path;
- a filesystem command interface;
- an environment-variable, path-lookup, pipe, redirection, globbing, script,
  job-control, session, signal, or process-group contract.

Diagnostic commands remain useful validation surfaces, but they must not become
the only way programs run and must not bypass descriptor, VFS, syscall, or EL0
contracts.

### Memory And Lower-EL Readiness

The accepted memory and lower-EL readiness docs establish an EL2 kernel map,
not a userspace isolation contract. The current map uses TTBR0_EL2 and broad
identity mappings for low DRAM and required MMIO. It proves early kernel
execution with EL2 stage-1 translation, not user/kernel separation.

The POSIX baseline may define process address-space vocabulary, but EL0 and
syscall implementation remain blocked until later tasks define and validate:

- user/kernel virtual address split and permissions;
- per-process page tables or address-space handles;
- user stacks, heaps, code/data mappings, and guard gaps;
- lower-EL vector routing, trap return, and fault policy;
- copy-in/copy-out helpers and invalid-user-pointer behavior;
- SVC/syscall argument, return-value, and error conventions.

No Phase 7.1 contract should treat the existing EL2 identity map as permission
to run untrusted payloads.

### Retained Validation Gates

The accepted Phase 4 through Phase 6 gates that constrain the first POSIX
baseline are:

- cargo -Zjson-target-spec test for target-independent scheduler, console, TTY,
  diagnostic command, and scheduler-runtime unit coverage;
- scripts/qemu-smoke.sh for the base QEMU boot path;
- scripts/qemu-timer-preemption-smoke.sh for the earlier single-core timer
  preemption boundary;
- scripts/qemu-secondary-scheduler-service-loop-smoke.sh for owner-local
  scheduler service ordering;
- scripts/qemu-shared-runqueue-migration-smoke.sh for the accepted
  owner-transfer path;
- scripts/qemu-load-balancing-smoke.sh for deterministic polling load-balancing
  over SharedRunQueue;
- scripts/qemu-multicore-preemption-smoke.sh for the diagnostic multi-core
  local-record/owner-service invariant;
- scripts/qemu-production-timer-preemption-smoke.sh for the production timer
  IRQ adapter and owner-local service boundary;
- serialized Pi 5 reproduction helpers for explicit future hardware tasks.

The next Phase 7.1 contract task is docs-only, so it should require git diff
--check and mdbook build. Later Rust implementation tasks should use the
host/unit gate first, then add QEMU or hardware gates only when the task changes
boot/runtime behavior.

## Design-Ready Contracts

These surfaces are ready to be referenced by the POSIX baseline:

- scheduler-local task vocabulary, task state, per-task kernel stack/register
  ownership, and optional future process-owner hook;
- process as a future resource-owning container for address space, descriptor
  table, current working directory, root/namespace handle, credentials, exit
  status, wait state, and one or more tasks;
- process-local integer file descriptors whose entries reference open file
  descriptions or kernel objects;
- runtime-console0 as the accepted default console object behind future stdio
  descriptors;
- TTY line discipline as a console client above runtime-console input/output,
  not as descriptor/syscall/process owner;
- structured internal errors that later map to stable errno-style values at a
  syscall boundary;
- absolute and relative path vocabulary from the early POSIX note, with root
  and current working directory owned by the future process rather than by a
  shell singleton.

## Implementation Gaps

The first POSIX contract must keep these gaps explicit:

- no PID allocator, parent/child relation, process table, exit status, wait
  queue, signal state, credential model, process group, or session model;
- no process-owned address space, user stack, EL0 state, trap frame, SVC path,
  or user/kernel copy helper;
- no descriptor table, descriptor entry lifetime, open file description, dup,
  close, read, write, pipe, socket, readiness, nonblocking, EOF, or partial I/O
  policy;
- no path normalization implementation, root/current-working-directory storage,
  namespace handle, VFS lookup, mount model, inode, directory, or file object;
- no program loader, spawn, exec, argument vector, environment vector, or
  inherited stdio implementation;
- no scheduler sleep/wakeup integration for blocking descriptor I/O;
- no filesystem-backed commands, local shell, networking, SSH, RP1/PCIe, UART
  interrupt ownership, or DMA/cache-driver policy.

## Next Contract Task

The next bounded task should be phase7-posix-contract-baseline-20260528. It
should add the first POSIX baseline contract for errno/error vocabulary, path
normalization semantics, process lifetime vocabulary, descriptor operation
vocabulary, stdio inheritance shape, and early loader/argument/environment
story.

That task should remain documentation-only unless a narrow compile-time example
is explicitly justified. It must not start EL0, SVC/syscall ABI, descriptor
table implementation, VFS, filesystem, program loading, networking, SSH, shell
behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Validation

- static inspection: reviewed scheduler, console, TTY, diagnostic command,
  lower-EL readiness, early POSIX, Phase 6 production runtime closeout, roadmap,
  and decision-log surfaces.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
