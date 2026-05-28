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

## Spawn, Exec, Exit, and Wait

The first implementation can choose a small subset, but it should keep these concepts separate:

- spawn creates a process or task from a kernel-selected program image
- exec replaces the current process image
- exit records status and terminates the process
- wait observes child process completion

A shell-only command dispatcher is acceptable as a diagnostic tool, but it should not become the only way programs run.

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
target-independent table core is implemented.
