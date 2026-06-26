# Early POSIX Shape

This is not a full POSIX design. It is an early guardrail so kernel threads, processes, descriptors, and the scheduler do not grow in a direction that makes Unix-like behavior painful later.

## Naming

Talos should distinguish:

- Task: schedulable execution context known to the scheduler.
- Kernel thread: task running only in kernel address space.
- Process: resource-owning container with address space, descriptor table, current working directory, credentials later, and one or more tasks.
- User thread: task executing in a process address space at EL0.

Early milestones may implement only kernel threads, but scheduler structures should not assume that every task owns global resources directly.

## Process Shape

A process should eventually own:

- process ID
- parent process ID
- address space
- descriptor table
- current working directory
- root directory or namespace handle
- exit status
- child state for wait
- signal state later

The scheduler should schedule tasks, not processes. A task should point at its owning process when user processes exist.

## Descriptor Shape

File descriptors should be process-local integer handles. Descriptor operations should be designed around:

- open
- read
- write
- close
- dup
- pipe
- socket later
- inheritance across spawn or exec

The target abstraction is one descriptor table per process, with descriptor entries referencing open file descriptions or kernel objects. Console, pipes, files, devices, and sockets should use the same descriptor-facing operations even if their internals differ.

## Path Shape

Talos should define path behavior before implementing VFS:

- absolute paths start at the process root
- relative paths start at the current working directory
- dot and dot-dot normalization must be explicit
- path lookup should return structured errors, not strings
- current working directory belongs to the process, not the shell

## Error Shape

Internal Rust errors can be rich enums. The syscall boundary should translate them into stable numeric errors. The mapping does not need to be complete initially, but it should reserve a path toward familiar errno-style behavior.

The architecture-quality review for
talos-review-posix-syscall-descriptors-20260603 tightened that rule: the
syscall boundary now maps every current PosixError variant to an explicit
errno number. Future PosixError additions should add syscall encoding tests
with the new variant instead of relying on a fallback such as ENOSYS.

Phase 12.4 now has a private socket ABI contract for the accepted
descriptor-backed AF_INET/SOCK_STREAM surface. It uses the stable Talos syscall
trap shape (svc #0, x8 selector, x0 through x5 scalar arguments, x0 return,
negative x0 errno) and records the current private socket selector vocabulary
before adding no_std/userspace wrappers. That contract is not POSIX/Linux
compatibility and does not accept libc/std sockets, UDP/raw sockets, live packet
I/O, hardware reachability, SSH, or a public stable socket ABI.

## Spawn, Exec, Exit, and Wait

The first implementation can choose a small subset, but it should keep these concepts separate:

- spawn creates a process or task from a kernel-selected program image
- exec replaces the current process image
- exit records status and terminates the process
- wait observes child process completion

A shell-only command dispatcher is acceptable as a diagnostic tool, but it should not become the only way programs run.

Phase 12's local POSIX continuation adds the first bounded process-table
substrate for accepted local VFS exec forms. Direct foreground fixtures
/bin/init, /bin/zero, and /bin/status42 record an internal kernel-owned
process-table lifecycle/status entry with stable pid, shell parent, owner,
path, exited state, status, observed-status, and reaped state. The same
bounded substrate now also records exact two-stage pipeline producer/consumer
entries with distinct stable pids and accepted background /bin/status42 and
/bin/zero job entries while preserving the existing shell-visible waitpid,
laststatus, and jobs accounting surfaces. That is still not a public process
enumeration API, Linux procfs compatibility, fork, process groups, waitpid option model, PID
reuse policy, or scheduler-concurrent process model.

The first process-status VFS view is `/proc/talos/processes`. It is
Talos-private, read-only, versioned as `talos-processes-v1`, descriptor-backed
through the same `cat`/open/read path used by accepted VFS files, and reports
only bounded process-table records for direct VFS exec, exact two-stage
pipelines, and accepted background jobs. It is not Linux procfs compatibility,
`/proc/self`, `/proc/<pid>`, a public process enumeration ABI, or a
scheduler-concurrent process model. The zero-argument `ps` shell view is a
thin presentation of that same Talos-private VFS file; unsupported arguments
or options are outside the accepted boundary.

The 2026-06-26 local POSIX frontier checkpoint keeps that whole chain at
static/unit/QEMU-substitute evidence level: descriptor-backed VFS file I/O,
direct fixture loading, local userspace launch/status, bounded process-table
records, waitpid/laststatus/jobs accounting, `/proc/talos/processes`, and
zero-argument VFS-backed `ps` are accepted; live networking/SSH, Pi 5 hardware
proof, scheduler concurrency, fork/signals, process groups/sessions, broad
procfs, PID policy expansion, persistent storage, arbitrary pipelines,
pipefail, and phase transition remain deferred pending supervisor planning.

The first accepted multistage pipeline remains deliberately narrow:
`exec stdout | exec stdin | exec stdin`. It is shell-visible and VFS-backed,
but still serialized and bounded. The three fixtures exchange bytes through
two pipe descriptor handoffs and install three process-table records that
`waitpid`, `/proc/talos/processes`, and zero-argument `ps` can observe.
The bounded `pipestatus` shell surface now reads those same process-table
records for exact two-stage and accepted three-stage pipelines, preserving
default final-stage status while exposing a labeled `pipefail-status`
observation for a nonzero producer case. This is not POSIX shell
compatibility and does not accept arbitrary pipeline grammar, unbounded
pipeline length, pipeline concurrency, or a shell option framework.

The 2026-06-26 local pipeline frontier checkpoint keeps that pipeline layer at
static/unit/QEMU-substitute evidence level. Accepted pipeline behavior is still
limited to exact two-stage local forms, the accepted
`exec stdout | exec stdin | exec stdin` three-stage form, bounded
process-table records, `/proc/talos/processes`, zero-argument VFS-backed
`ps`, and `pipestatus` with `pipefail-status` labeled
`bounded-observation-not-posix-shell`. Live networking/SSH, Pi 5 hardware
proof, scheduler concurrency, fork/signals, process groups/sessions, broad
procfs/Linux `ps`, persistent storage, arbitrary shell grammar, unbounded
pipelines, POSIX pipefail compatibility, and phase transition remain deferred
pending supervisor planning.

## Scheduler Implications

Before implementing scheduler structs, check that:

- task lifetime can detach from process lifetime later
- per-task kernel stack and register state are separate from process resources
- blocking I/O can sleep a task without blocking the whole process model
- wakeups can target tasks
- descriptor and address-space pointers can be added without redesign

This note is now expanded by
[Phase 7 POSIX Contract Baseline](phase7-posix-contract-baseline.md), which
defines the accepted Phase 7.1 errno, path, process lifetime, descriptor,
stdio inheritance, and early loader vocabulary before VFS, syscalls, and user
processes are implemented. The descriptor portion is narrowed by the accepted
[Phase 7 Descriptor Table Contract](phase7-descriptor-table-contract.md),
which fixes process-local descriptor table entries, dup/close behavior,
reserved stdio handles, and deterministic descriptor error cases before the
target-independent table core is implemented. The accepted Phase 7.1
descriptor-table core now implements those target-independent table semantics
without runtime console/TTY I/O, syscall ABI, EL0, VFS, filesystem, pipes,
sockets, shell behavior, networking, SSH, or hardware claims.
