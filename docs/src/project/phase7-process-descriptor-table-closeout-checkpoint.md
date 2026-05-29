# Phase 7 Process Descriptor Table Closeout Checkpoint

Status: accepted as the Milestone 7.4 process-owned descriptor-table closeout
checkpoint after the accepted QEMU/substitute process descriptor stdio smoke
core. This checkpoint adds no Rust behavior, assembly behavior, QEMU run,
Pi 5 hardware run, boot archive publication, hardware-lock acquisition,
stdin/read, close/dup/read syscall behavior, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Accepted Scope

This slice closes out the first process-owned descriptor-table boundary:

- documentation contract:
  phase7-process-descriptor-table-contract-20260529 at
  adc0ed9ea37fe35b0c45dd19666ba68fe8546187;
- target-independent core:
  phase7-process-descriptor-table-core-20260529 at
  a30944d53aefd58ca89a7d197d12bae0790beb73;
- QEMU/substitute smoke plan:
  phase7-qemu-process-descriptor-stdio-smoke-plan-20260529 at
  b314ab881f82a07da32bd1db88786a4dbf6d471e;
- QEMU/substitute smoke core:
  phase7-qemu-process-descriptor-stdio-smoke-core-20260529 at
  fe17a6d99a634903639e5a9b8d9d5a5644822c0c.

The accepted capability is narrow. A ProcessOwnerId-backed
ProcessDescriptorStore can own one inherited-stdio DescriptorTable, resolve the
current owner through the accepted lookup API, and route lower-AArch64
talos_write fd 1/fd 2 through the process-owned table to runtime-console0 in
QEMU/substitute evidence. The accepted fd/error regressions remain fd 0 and fd
99 as -EBADF, guard-range copy_from_user failure as -EFAULT, nonzero reserved
syscall registers as -EINVAL, talos_nop success, unknown syscall -ENOSYS,
proof-only talos_copy_probe quarantine, and diagnostic marker quarantine.

## Evidence Matrix

| Task | Evidence level | Retained evidence |
| --- | --- | --- |
| process descriptor table contract | static documentation inspection | docs/src/project/phase7-process-descriptor-table-contract.md |
| process descriptor table core | fmt/lint, unit tests, static inspection | cargo -Zjson-target-spec test passed with 222 no_std tests |
| QEMU process descriptor stdio smoke plan | static documentation inspection | docs/src/project/phase7-qemu-process-descriptor-stdio-smoke-plan.md |
| QEMU process descriptor stdio smoke core | QEMU/substitute serial boot/output, regressions, unit tests | tasks/evidence/2026-05-29-qemu-process-descriptor-stdio-smoke-core/qemu-process-descriptor-stdio-smoke.log |

The retained QEMU/substitute smoke log reports:

    qemu-process-descriptor-stdio-smoke: current-descriptor-table lookup=process-owned owner=0x0000000000000001 resolved=true stdio=inherited runtime-console=runtime-console0
    qemu-process-descriptor-stdio-smoke: final participants=8 expected=8 errors=0 classification=qemu-process-descriptor-stdio-smoke-complete
    qemu-process-descriptor-stdio-smoke: PASS

The smoke-core task also records cargo fmt, cargo tests, the focused
qemu-process-descriptor-stdio smoke, descriptor-write/syscall/pointer-copy QEMU
regressions, git diff --check, and mdbook build as passed.

## Deferred Surfaces

The closeout accepts no Pi 5 physical descriptor-table proof. It also keeps
stdin/read behavior, close/dup/read syscalls, descriptor lifetime and close
semantics, PID allocation, fork/spawn/exec, process loading, process-owned
address spaces, VFS/filesystem lookup, regular files, directories, pipes,
sockets, device registries, TTY blocking/readiness, EOF, nonblocking flags,
wait queues, signals, restart semantics, open-file-description reference
counting, shell behavior, libc/Rust std stdio, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, and full POSIX descriptor claims
blocked.

## Residual Risk

The process-owned lookup proof is still a QEMU/substitute boundary. It proves
the lower-AArch64 syscall route can use ProcessDescriptorStore in the accepted
scenario, but it does not prove the same path on Pi 5 hardware, under a live
process loader, with dynamic process lifetime, or with descriptor operations
beyond inherited-stdio write. Later tasks need explicit contracts and evidence
for those claims.

## Recommended Next Task

The next bounded Milestone 7.4 task should be a documentation-only descriptor
lifetime and close-semantics source inventory, for example
phase7-descriptor-lifetime-close-source-inventory-20260529.

That task should map the current DescriptorTable close/dup primitives,
open-file-description vocabulary, inherited stdio lifetime, owner teardown
gaps, and unit-test evidence needed before any close/dup/read syscall contract.
It should not implement Rust behavior, run QEMU, acquire hardwareTestLock,
publish a boot archive, run Pi 5 hardware, or advance process loading,
VFS/filesystem, shell, networking, or SSH.

## Validation

- static evidence review: reviewed accepted contract/core/smoke plan/smoke core
  records and retained QEMU/substitute evidence.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
