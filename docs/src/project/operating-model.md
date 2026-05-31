# Operating Model

Talos is managed as a long-running engineering program. The kernel is the primary deliverable, but the roadmap, ADRs, task notes, lab runbooks, and architecture documentation must stay accurate as the code evolves.

## Default Work Pattern

Long work should not block the Telegram thread.

Use this pattern:

1. Acknowledge the work and state the intended result.
2. Start a TaskFlow, detached task, or subagent when work may run long.
3. Keep minimum durable state needed to resume.
4. Wake or report back when the task completes.
5. Update docs or task records before declaring the work done.

## Feature-Led Planning

Planner work starts from the smallest useful user-visible capability, not from
the next provable internal slice. The plan should name the interaction a person
or normal program will be able to perform when the task lands, then choose the
thinnest implementation path that makes that interaction real.

For example, local console work should prefer "type a line over serial, press
Enter, receive a command response" over another isolated stdin proof. A first
shell may use kernel-backed fake built-ins while user program execution is still
immature, as long as the interaction uses the same TTY/stdio/process surfaces
that the real shell will grow into.

Planner tasks must avoid mechanically expanding every subsystem into
inventory, contract, smoke plan, smoke core, and closeout tasks. Use those
documents only when they reduce risk for a real feature. If the next step is
clear enough to code, assign a feature implementation task.

Diagnostics are a fallback, not the default unit of progress. Add one only
after a feature attempt is blocked by an unknown that cannot be resolved by code
review, ordinary tests, or direct feature validation. A diagnostic must state the
feature it unblocks, the exact question it answers, and how it will be removed
or promoted after the answer is known.

## Roles

The project lead owns prioritization, integration, and final decisions. Specialist agents can advise or implement, but their output is evidence for the lead to review.

Expected specialist roles:

- Kernel Architect: OS structure, subsystem boundaries, and POSIX path.
- Board Bring-Up Engineer: Pi 5 boot, MMIO, UART, timers, interrupts, and board-specific drivers.
- Automation Engineer: power control, PXE image publishing, serial log capture, boot classification, and lab APIs.
- Reference Researcher: Linux, Raspberry Pi firmware docs, Circle/RPi bare-metal examples, ARM docs, and hardware datasheets.
- Cautious Reviewer: failure modes, unsafe assumptions, scope creep, and rollback strategy.

## Definition of Done

A milestone is done only when:

- Acceptance criteria are written clearly enough to review.
- The code or lab infrastructure works against its acceptance criteria.
- The acceptance criteria exercise the intended feature directly whenever
  possible.
- The result has been verified with the smallest meaningful feature-level test.
- Hardware-dependent claims have completed the pre-hardware review, single-board hardware test, and post-hardware evidence review loop.
- The roadmap status is updated.
- Relevant architecture, hardware, ADR, or runbook docs are updated.
- Remaining risks are documented.

A working kernel without accurate documentation is incomplete.
