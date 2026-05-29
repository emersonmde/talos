# Phase 7 Descriptor Close Core Closeout Checkpoint

Status: accepted as the Milestone 7.4 descriptor lifetime and close-core
closeout checkpoint after the accepted target-independent descriptor close
core. This checkpoint adds no Rust behavior, assembly behavior, QEMU run,
Pi 5 hardware run, boot archive publication, hardware-lock acquisition,
close/dup/read syscall surface, process loading, VFS/filesystem behavior,
shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Accepted Scope

This slice closes out the first target-independent process-owned descriptor
close boundary:

- descriptor lifetime and close source inventory:
  phase7-descriptor-lifetime-close-source-inventory-20260529 at
  0de2bf2be47986da3220d9fb3edea534448822b8;
- descriptor lifetime and close contract:
  phase7-descriptor-lifetime-close-contract-20260529 at
  4ff46a6f68bf8349ba0b974d610a8ceb3d92ccd1;
- descriptor close core:
  phase7-descriptor-close-core-20260529 at
  1e8cdd6fcb4bd16cbb04febd56529b66b0579182.

The accepted capability is narrow. ProcessDescriptorStore now exposes
close_current_descriptor(), which resolves an optional current ProcessOwnerId
to a mutable owner DescriptorTable and applies table-local DescriptorTable
close semantics. Closing an occupied descriptor removes that slot and returns
the removed DescriptorEntry. Missing current owner, unknown owner, invalid
descriptor, empty descriptor, and double close map to BadDescriptor. Closing
one duplicate leaves the other descriptor valid, and later allocation reuses
the lowest available slot according to the existing table allocation rule.

## Evidence Matrix

| Task | Evidence level | Retained evidence |
| --- | --- | --- |
| descriptor lifetime and close source inventory | static source and documentation inspection | docs/src/project/phase7-descriptor-lifetime-close-source-inventory.md |
| descriptor lifetime and close contract | static source and documentation inspection | docs/src/project/phase7-descriptor-lifetime-close-contract.md |
| descriptor close core | fmt/lint, unit tests, static inspection, documentation build | tasks/2026-05-29-phase7-descriptor-close-core.md |

The descriptor close core changed:

- src/posix.rs;
- tasks/2026-05-29-phase7-descriptor-close-core.md.

Focused unit-test coverage accepted by the core task:

- process_descriptor_close_stdout_blocks_descriptor_write_lookup;
- process_descriptor_close_stderr_follows_table_local_rule;
- process_descriptor_close_failures_map_to_ebadf;
- process_descriptor_close_reuses_lowest_slot_and_preserves_duplicates.

The accepted core validation passed:

- cargo fmt --all -- --check;
- cargo -Zjson-target-spec test with 226 no_std tests;
- git diff --check;
- mdbook build.

## Deferred Surfaces

The closeout accepts no close syscall, dup syscall, read syscall, syscall
number, lower-EL ABI route, QEMU close/dup/read smoke, Pi 5 physical
close/dup/read proof, boot archive publication, process loading,
fork/spawn/exec, close-on-exec application, process exit teardown,
open-file-description reference counting, object finalizer, VFS/filesystem
lookup, regular file, directory, pipe, socket, stdin/read behavior, TTY
blocking/readiness, EOF, nonblocking flag, wait queue, signal, restart
semantic, per-thread errno storage, shell behavior, libc/Rust std stdio,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
or full POSIX descriptor readiness.

## Residual Risk

The accepted close helper is target-independent unit-test evidence only. It
proves ProcessDescriptorStore can apply table-local close semantics to a
current owner, but it does not prove any lower-EL syscall route, QEMU serial
observation, Pi 5 physical behavior, live process teardown, descriptor-table
inheritance, close-on-exec behavior, or final object release. Later tasks need
explicit contracts and evidence before making those claims.

## Recommended Next Task

The next bounded Milestone 7.4 task should be a documentation-only syscall
source inventory for close, dup, and read descriptor operations, for example
phase7-close-dup-read-syscall-source-inventory-20260529.

That task should map the currently accepted syscall ABI, descriptor-write
dispatch path, ProcessDescriptorStore lookup boundary, pointer-copy helper,
copy-out requirements for read, and evidence gaps before any close/dup/read
syscall contract. It should not implement Rust behavior, run QEMU, acquire
hardwareTestLock, publish a boot archive, run Pi 5 hardware, or advance
process loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, or a phase transition.

## Validation

- static evidence review: reviewed accepted inventory, contract, core task
  record, changed-file list, and unit-test evidence.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
