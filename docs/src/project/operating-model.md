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

## Roles

The project lead owns prioritization, integration, and final decisions. Specialist agents can advise or implement, but their output is evidence for the lead to review.

Expected specialist roles:

- Kernel Architect: OS structure, subsystem boundaries, and POSIX path.
- Board Bring-Up Engineer: Pi 5 boot, MMIO, UART, timers, interrupts, and board-specific drivers.
- Automation Engineer: UniFi power control, PXE image publishing, serial log capture, boot classification, and lab APIs.
- Reference Researcher: Linux, U-Boot, Raspberry Pi firmware docs, ARM docs, and hardware datasheets.
- Cautious Reviewer: failure modes, unsafe assumptions, scope creep, and rollback strategy.

## Definition of Done

A milestone is done only when:

- The code or lab infrastructure works against its acceptance criteria.
- The result has been verified with the smallest meaningful test.
- The roadmap status is updated.
- Relevant architecture, hardware, ADR, or runbook docs are updated.
- Remaining risks are documented.

A working kernel without accurate documentation is incomplete.
