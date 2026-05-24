# Phase 5 Console Model Checkpoint

Task: phase5-console-model-checkpoint-20260524

## Goal

Checkpoint the accepted Phase 5.1 console device model before allowing Milestone 5.2 TTY/stdio design work.

## Reconciliation

Accepted Phase 5.1 commits:

- 34a3108: console device-model source inventory.
- bde4079: output-only runtime console write core.
- 21b0847: internal write-result contract.
- 0580166: default runtime console identity, runtime-console0.
- 2925fa6: local input-source inventory and first-input recommendation.

The accepted model routes normal kernel diagnostics through runtime-console0 while target modules keep QEMU PL011 and Pi 5 UART10 backend ownership. The write-result contract remains an internal kernel-console result, not POSIX errno or syscall ABI.

## Go/No-Go

Go for Milestone 5.2 planning only.

phase5-tty-stdio-shape-doc-20260524 is the next bounded task. It may define TTY/stdio behavior and descriptor shape in documentation. It must not implement TTY behavior, UART RX, descriptors, syscalls, userspace, shell behavior, hardware tests, filesystems, networking, SSH, or scheduler blocking I/O.

## Open Risks

- stdin has no input backend yet.
- Pi 5 input has no hardware proof yet and must not be inferred from output-only UART10 evidence.
- Descriptor, syscall, and blocking semantics remain unimplemented.
- UART interrupts, input buffering, line discipline, echo, canonical mode, PTY behavior, and local shell behavior remain deferred.

## Validation

- fmt/lint/typecheck: git status --short was clean before edits.
- fmt/lint/typecheck: git diff --check passed.
- static inspection: mdbook was unavailable in the container, so the mdBook build was not run.
- Rust fmt/tests were not required because this task changed only Markdown documentation and durable task state.
