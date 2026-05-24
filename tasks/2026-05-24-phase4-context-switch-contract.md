# Phase 4 Context-Switch Contract

Task: phase4-context-switch-contract-20260524

## Goal

Define the AArch64 EL2 cooperative context-switch contract before adding switch
assembly or timer-driven preemption.

## Accepted Contract

The first switch is a cooperative boot-CPU kernel-thread switch at current EL2.
It is entered from normal kernel control flow and resumes as an ordinary
function-call return in the selected task. It does not switch exception levels,
does not use an EL0 context, and does not use an IRQ exception frame.

The minimal saved state is:

- `x19` through `x29`;
- `x30` as the resumed link register, or an equivalent saved program counter
  for a freshly bootstrapped task;
- `SP_EL2` for the task's kernel stack pointer.

`x0` through `x18` remain caller-saved scratch according to the normal AArch64
procedure-call rules. The first assembly switch may clobber them, and Rust code
must cross the cooperative yield boundary through an ABI that lets the compiler
preserve any live values normally.

`ContextFrame` currently stores only a stack pointer and program counter. The
implementation task must grow it, or pair it with an architecture-owned saved
frame, before assembly switching so the saved `x19..x30` state has an explicit
home. `KernelStack` remains the owner of per-task stack bounds.

## Stack And Ownership Invariants

- Every switched task is a kernel thread running at EL2 in the shared kernel
  address space.
- `SP_EL2` is 16-byte aligned at public call boundaries.
- Saved stack pointers stay inside the owning `KernelStack` bounds.
- Newly bootstrapped tasks enter through a kernel-thread trampoline or entry
  shim with an initialized saved stack pointer and resume address.
- No process address space, user stack, descriptor table, syscall state, file
  descriptor, filesystem, console/TTY, networking, or SSH state is attached to
  the task.
- No SMP migration, secondary-core run queue, spinlock, or cross-core
  memory-ordering policy exists in this contract.

## Critical Section Boundary

Scheduler-owned global state must be coherent while the current task, runnable
queue, task states, and saved context pointers are changed. The accepted
`single_core_irq_mask_save()` / `single_core_irq_restore()` primitive may
protect exactly that short boot-CPU invariant around a cooperative switch.

The interrupt-masked section must not allocate, format, print, block, sleep, or
run arbitrary callbacks. It is not an SMP lock, interrupt-safe lock hierarchy,
preemption-disable counter, or lower-EL interrupt policy.

## Preemption Deferrals

Timer-driven preemption needs a separate exception-frame contract. It must
define asynchronous saving for caller-saved state, `ELR_EL2`, `SPSR_EL2`,
timer acknowledge/reprogram/EOI ordering, and the rule that diagnostics or
blocking work happen outside the IRQ hot path. Those items remain deferred until
after cooperative switching and voluntary dispatch are accepted.

## Next Implementation Gate

The next bounded implementation task may add the smallest cooperative switch
primitive and prove it in QEMU with two kernel-thread contexts, separate kernel
stacks, and bounded progress counters. The proof should report progress outside
the switch hot path. Pi 5 hardware is not required unless the implementation
changes hardware-facing boot or timer behavior.

## Local Validation

- static inspection: `git status --short` was clean before documentation edits.
- fmt/lint/typecheck: `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.
