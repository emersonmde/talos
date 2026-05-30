# Phase 8 Program Loader Closeout Checkpoint

Status: accepted closeout checkpoint for
phase8-program-loader-closeout-checkpoint-20260530.

## Scope

This documentation-only checkpoint reconciles the accepted Milestone 8.3
program-loader source inventory, format contract, QEMU/substitute smoke plan,
target-independent core, and retained QEMU/substitute smoke evidence before
any process-install, lower-EL launch, shell, or hardware work.

It adds no Rust or assembly behavior, reruns no QEMU scenario, performs no
Raspberry Pi 5 hardware action, publishes no boot archive, and acquires no
hardwareTestLock. It does not accept process address-space installation,
user-frame allocation, page-table mutation, initial user stack, argv/envp,
descriptor inheritance across exec, process creation, exec/spawn/wait, shell
behavior, descriptor-backed filesystem syscalls, writable filesystems,
persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Accepted Capability

The accepted slice is a bounded image-plan-only program-loader frontier:

- the source inventory maps the accepted read-only initramfs/VFS regular-file
  input, filesystem byte owners, POSIX error vocabulary, user-memory
  permissions, lower-EL proof payloads, scheduler/process-owner placeholders,
  descriptor inheritance, and evidence conventions;
- the format contract selects the first executable policy as a narrow static
  ELF64/AArch64 ET_EXEC subset with deterministic header and program-header
  validation, dynamic/interpreter rejection, PT_LOAD permission classification,
  W^X rejection, user-range and overlap checks, explicit BSS zero-fill, entry
  validation, and loader error mapping;
- the QEMU/substitute smoke plan defines the qemu_program_loader_smoke
  scenario, fixture identity, retained evidence path, required
  PASS/classification lines, success observations, negative cases, and
  regression gates;
- the target-independent core adds src/program_loader.rs, immutable /bin/init
  ELF64/AArch64 fixture bytes, ReadOnlyInitramfs::regular_file_bytes(), and
  focused no_std unit coverage for the accepted success and rejection cases;
  and
- the retained QEMU/substitute smoke proves the accepted fixture identity,
  image-plan-only success, UserText/UserData segment classification, entry
  placement, and required deterministic negative cases through the scenario
  log.

The stable fixture identity is phase8-program-loader-elf64-aarch64-v1. The
accepted /bin/init bytes are executable-format data that produce an image plan;
they are not installed into a process address space and are not launched.

## Evidence Reconciliation

| Task | Commit | Evidence level | Result |
| --- | --- | --- | --- |
| phase8-program-loader-source-inventory-20260530 | a4cb53483914622ae90529b16ab814639e14e45d | static documentation/source-owner inspection | accepted loader source-owner map and missing contracts |
| phase8-program-loader-format-contract-20260530 | d6020818bc3d9163b26590858ab7d225e2d62563 | static documentation/source inspection | accepted static ELF64/AArch64 image-plan contract and deferred process boundary |
| phase8-qemu-program-loader-smoke-plan-20260530 | 71b26c62e0f4502278a2c8d6bb63f48cd519c33b | static documentation inspection | accepted QEMU/substitute smoke plan, exact output, and evidence path |
| phase8-program-loader-core-20260530 | 38b3a09ad4e1be353950ad75880b119e7e0b534e | target-independent source/tests plus QEMU/substitute VFS regression | accepted image-plan loader core and deterministic negative unit coverage |
| phase8-qemu-program-loader-smoke-core-20260530 | 2ff02f29962804c8579a324baf41e232c203fc08 | QEMU/substitute serial/log evidence | accepted retained smoke PASS/classification evidence |

Retained QEMU/substitute evidence:

- tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log
- fixture identity: phase8-program-loader-elf64-aarch64-v1,
  stable-elf-manifest digest 0x3892eed223900c65
- final lines:
  qemu-program-loader-smoke: final participants=8 expected=8 errors=0
  classification=qemu-program-loader-smoke-complete
  qemu-program-loader-smoke: PASS

The retained log includes the accepted success image plan for /bin/init,
UserText R-X and UserData RW-/BSS segment observations, entry 0x10100 inside
text, output=image-plan-only with process-created=false, stack-built=false,
and descriptors-installed=false, plus the planned bad-magic,
dynamic-interpreter, wx-segment, out-of-user-range, overlap, bad-entry, and
file-range-overflow errors with partial-install=false.

## Deferred Surfaces

The following remain explicitly blocked until later tasks define and accept
their contracts and validation gates:

- process address-space installation, user-frame allocation, page-table
  mutation, segment materialization, failure unwind, and teardown;
- lower-EL launch of the loaded image, initial exception frame, stack pointer,
  SPSR/PSTATE policy, argv/envp layout, auxiliary vectors, TLS, and libc
  startup compatibility;
- process identity, PID allocation, scheduler handoff, exec/spawn/wait, and
  descriptor inheritance or close-on-exec semantics;
- descriptor-backed production filesystem syscalls, open/read integration for
  filesystem objects, directory iteration, seek, and final object release;
- Pi 5 hardware proof, boot archive publication, TFTP archive identity, and
  firmware/TFTP initramfs transport;
- shell behavior, writable filesystems, persistent storage, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Residual Risks

- The accepted loader still produces only a target-independent image plan.
  Later work must prove frame allocation, page-table population, page
  permissions, zero-fill realization, and unwind behavior before any loaded
  image can run.
- The success fixture is intentionally tiny and static. It proves format and
  mapping-policy boundaries, not libc startup, shell compatibility, or a
  general userspace ABI.
- The retained runtime evidence is QEMU/substitute only. A later hardware task
  must define archive identity, serial/TFTP evidence, hardwareTestLock
  ownership, and restoration rules before claiming Pi 5 loader behavior.
- Descriptor inheritance is not accepted. Later process-install work must
  decide whether the first process receives stdio descriptors, how close-on-exec
  is represented, and where descriptor-table ownership lives.

## Recommended Next Task

The next bounded Phase 8 task should be
phase8-process-install-source-inventory-20260530, documentation-only under
Milestone 8.3.

That inventory should map source owners and missing contracts for installing a
validated image plan into a future process-owned address space: user-frame
allocation, segment mapping, zero-fill materialization, page permissions,
teardown/unwind, initial lower-EL frame inputs, user stack policy, descriptor
inheritance, and scheduler/process ownership. It should keep implementation,
QEMU execution, Pi 5 hardware proof, lower-EL launch, argv/envp bytes,
exec/spawn/wait, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy blocked until later explicit tasks.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: inspected the accepted loader source inventory,
  format contract, QEMU/substitute smoke plan, core task record,
  QEMU/substitute smoke task record, roadmap, ADR index, and retained
  QEMU/substitute evidence path.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout.
