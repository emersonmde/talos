# Vision

Talos is a bare-metal Rust operating system with Raspberry Pi 5 as its first
supported physical architecture.

The goal is not to clone Linux or produce a one-off board demo. The goal is to
build a small, understandable kernel with enough Unix-like structure to become a
practical system: multi-core preemptive scheduling, processes, file descriptors,
pipes, filesystem support, networking, SSH access, a shell, and basic
command-line programs. Board-specific code should stay behind target boundaries
so the design remains generic, while Pi 5 remains the first real source of
hardware truth.

## North Star

> A Raspberry Pi 5 boots Talos from the network, brings up enough hardware and networking to accept an SSH connection, and allows the user to interact with a shell while multiple programs run concurrently across the Pi 5 CPU cores.

This means usable has a concrete definition:

- The system can be built, published, booted, power-cycled, and inspected without manually reflashing an SD card.
- Boot and kernel logs are captured automatically from the serial console.
- The kernel uses multiple CPU cores.
- Preemptive scheduling allows multiple runnable tasks or programs to make progress.
- A network path exists from the development machine to the kernel.
- A remote shell can be reached over SSH.
- The shell can run useful built-ins and, later, separate user programs.

## Initial Non-Goals

These are future goals, but not requirements for the first usable system:

- Full POSIX compliance.
- Running unmodified Unix binaries.
- Supporting arbitrary Raspberry Pi models or non-Raspberry Pi boards.
- A production-grade security model.
- A complete package manager or self-hosted development environment.
- Porting large toolchains such as GCC.

The design should not close these doors. Talos should grow toward POSIX-like abstractions from the start, without letting full compatibility dominate the first milestone.

## Hardware Lab Strategy

The development loop should move from manual SD-card iteration to an automated physical lab:

- The Pi 5 is powered through a PoE hat connected to a managed switch.
- A host-side controller exposes narrow APIs for power off, power on, and power cycle operations.
- A host-side boot service publishes the image that the Pi 5 should network boot.
- A host-side serial service captures UART output and exposes logs to the agent environment.
- The containerized agent environment consumes APIs rather than requiring broad host privileges or direct hardware ownership.

The intended control loop is:

1. Build a kernel image.
2. Publish it as the active network boot image.
3. Power-cycle the Pi 5.
4. Capture serial output.
5. Classify the boot result.
6. Feed the result back into the next implementation step.

The lab loop is a prerequisite for sustained autonomous development. It should be treated as product infrastructure, not as a side script.

## Capability Layers

Talos should grow through independently testable layers:

1. Board bring-up: boot image, firmware handoff, UART, panic reporting, timers, interrupts, and hardware discovery.
2. Memory foundations: physical memory discovery, page allocation, virtual memory, kernel heap, and eventual user/kernel separation.
3. Multi-core preemption: secondary core bring-up, per-core state, timer interrupts, scheduler, and SMP-safe synchronization.
4. Process and syscall model: kernel threads first, then user processes and a stable syscall boundary.
5. File descriptors and I/O: standard streams, pipes, devices, files, and sockets under one descriptor model.
6. Filesystem: initramfs or ramfs first, VFS next, persistent storage later.
7. Networking and SSH: clean split between board drivers, packet buffers, network stack, sockets, and user-facing services.

## POSIX Direction

Full POSIX support is a later phase, but Talos should use Unix-compatible concepts where practical:

- Processes and threads.
- File descriptors.
- Pipes.
- Path-based filesystem operations.
- Sockets.
- Standard streams.
- Exec-style program loading.
- Wait and exit status.

Avoid early shortcuts that would make future POSIX compatibility painful, such as shell-only command dispatch, global singleton program state, or networking APIs that cannot map to sockets.

## Documentation Deliverable

The finished project should include accurate documentation that explains how the system works and why it was built that way:

- Maintained roadmap with current milestones and status.
- ADRs for major architectural decisions.
- Hardware reference notes with source links or citations.
- Bring-up and lab automation runbooks.
- Architecture documents for core subsystems.
- Task records for significant agent or human work.

If implementation and documentation disagree, the project is not done.
