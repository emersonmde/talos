# Roadmap

Talos is planned as a long-running Raspberry Pi 5 operating-system program, not as a single bring-up spike. The roadmap is organized around user-visible capabilities and validation gates. Each phase should leave the repository in a buildable, documented state.

The near-term strategy is dual-target:

- talos-aarch64-virt: a fast QEMU virt target for architecture work, tests, and CI.
- talos-rpi5-bcm2712: the physical Raspberry Pi 5 target, validated through the lab controller and serial console.

QEMU does not currently model the Raspberry Pi 5, BCM2712, or RP1. The physical Pi 5 lab is therefore the authority for board behavior. QEMU is still valuable for generic AArch64 boot, exceptions, MMU, scheduler, and pure subsystem tests.

The Pi 5 boot path should follow the normal firmware contract first. The EEPROM bootloader loads the kernel image directly, prefers kernel_2712.img, falls back to kernel8.img, and passes the physical device-tree address in x0 according to the arm64 boot ABI. Talos should implement that handoff before considering any custom boot path.

## Current Status

Talos is in Phase 8 Milestone 8.3 after the accepted Phase 7 final closeout
checkpoint recommended the first bounded filesystem/program-loading planning
task, the Phase 8 source inventory was accepted, and the read-only
initramfs/VFS contract, smoke plan, target-independent core, and
QEMU/substitute smoke were accepted. The read-only initramfs/VFS closeout
checkpoint is accepted and recommends a documentation-only program-loader
source inventory before any loader implementation or shell work. That
program-loader source inventory is now accepted and recommends a
documentation-only loader format contract before any parser, mapper, process
install, or shell task. That loader format contract is now accepted and
chooses a narrow static ELF64/AArch64 subset, deterministic rejection matrix,
segment permission/zero-fill/entry validation policy, and process-install
boundary before any implementation. The
QEMU/substitute program-loader smoke plan is accepted, naming fixture identity
phase8-program-loader-elf64-aarch64-v1, the retained future smoke evidence
path, exact PASS/classification vocabulary, and deterministic negative cases.
The target-independent program-loader core is now accepted: /bin/init is the
immutable static ELF64/AArch64 fixture, the loader returns an image plan only
with digest, UserText/UserData segment classification, file-copy ranges,
explicit BSS zero-fill, entry validation, and deterministic errors for bad
identity, unsupported dynamic/interpreter headers, malformed ranges, W+X,
out-of-range/overlap, bad entry, and file-range overflow. QEMU/substitute
program-loader smoke evidence is now accepted from the retained
qemu_program_loader_smoke log, which proves the image-plan-only success and
negative cases without process launch or hardware claims. Process address-space
installation remains blocked until later explicit tasks. The program-loader
closeout checkpoint is now accepted and recommends a documentation-only
process-install source inventory before any address-space installation,
lower-EL launch, or shell implementation.
The process-install source inventory is now followed by an accepted
documentation-only process-install contract. The first process-install
boundary is target-independent and metadata-only: a ProcessImageInstallPlan
derived from a validated ProgramImagePlan, preserving exact UserText/UserData
permissions, ordered file-copy and zero-fill page records, deterministic
errors, and all-or-nothing semantics. It accepts no frame allocation,
page-table mutation, scheduler handoff, lower-EL launch, argv/envp, descriptor
inheritance, shell, hardware, or filesystem syscall behavior. The next bounded
task is a QEMU/substitute process-install smoke plan for this metadata-only
boundary.
That QEMU/substitute process-install smoke plan is now accepted. It defines
qemu_process_install_smoke, loader fixture identity
phase8-program-loader-elf64-aarch64-v1, install boundary identity
phase8-process-install-plan-v1, retained evidence path
tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log,
exact PASS/classification vocabulary, metadata-only success observations,
deterministic no-partial-install rejection cases, and conditional regression
gates. Process-install implementation remains the next queued bounded task;
hardware, physical page allocation, page-table mutation, lower-EL launch,
argv/envp, scheduler handoff, shell, and filesystem syscall behavior remain
blocked.
The metadata-only process-install core and QEMU/substitute smoke are now
accepted. The retained qemu_process_install_smoke evidence proves that the
accepted /bin/init ProgramImagePlan derives a ProcessImageInstallPlan with
preserved entry, footprint, ordered UserText/UserData page records, exact
R-X/RW- permissions, explicit copy/zero-fill ranges, zero side effects, and
deterministic no-partial-install rejections for bad plan invariants, overlap,
permission widening, bad entry, and budget overflow. This still accepts no
physical frame allocation, page-table mutation, process creation, lower-EL
launch, argv/envp, exec/spawn/wait, shell, filesystem syscall behavior,
hardware proof, writable filesystem, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.
The
accepted Phase 7
frontier includes the Phase
6.3 production scheduler runtime closeout,
the full Phase 7.1 POSIX baseline slice, the Phase 7.2 EL0/address-space source
inventory, the Phase 7.2 EL0 trap/address-space contract, and the first
target-independent user-memory permission core. The accepted Phase 7.2 contract
defines the first user/kernel virtual-address split vocabulary, lower-EL
trap/return invariants, user fault classes, copy-in/copy-out preconditions,
evidence levels, and blocked surfaces. The accepted QEMU EL0 trap smoke plan
defined the first lower-EL proof boundary: one QEMU-only built-in user payload,
a diagnostic SVC marker trap back to the kernel, saved-state output, and
PASS/classification evidence. The accepted QEMU implementation now reports
classification=qemu-el0-trap-smoke-complete and qemu-el0-trap-smoke: PASS
from retained QEMU/substitute serial evidence. The serialized Pi 5 proof is
also accepted: local62 retained physical serial evidence contains the
source-backed translation feature report, regular VBAR_EL1 handoff,
lower-AArch64 synchronous SVC trap state,
classification=pi5-el0-trap-proof-complete, and rpi5-el0-trap-proof: PASS.
This accepts only the bounded lower-EL trap path. General SVC/syscall ABI,
VFS, filesystem, program loading, descriptor I/O, networking, SSH, and shell
work remain blocked until later explicit bounded tasks accept their contracts
and gates. The accepted Phase 7.3 syscall ABI source inventory maps the source
owners and gaps for SVC exception decoding, syscall number and argument
registers, return/error convention, user-copy preconditions, descriptor-table
interaction, and process/task ownership. The accepted Phase 7.3 syscall ABI
contract fixes lower-AArch64 svc #0, x8 syscall numbers, x0 through x5 scalar
arguments, x0 negative errno returns, talos_nop = 0, and unknown syscall =
-ENOSYS. The accepted target-independent syscall dispatch core implements that
bounded vocabulary and unit-tested return/error encoding without production
exception routing, QEMU, or hardware work. The accepted syscall trap-routing
source inventory maps production lower-AArch64 SVC detection, svc immediate
validation, x8 syscall-number extraction, x0-through-x5 argument capture, x0
return mutation, ELR/SPSR handling, diagnostic marker quarantine, and
non-syscall fallback. The accepted syscall trap-routing contract fixes the
production routing preconditions, frame mutation rules, failure classes,
diagnostic marker quarantine, and mandatory QEMU syscall smoke boundary. The
accepted QEMU syscall smoke plan defines the qemu_syscall_smoke invariant,
stable svc #0 talos_nop and unknown-syscall return observations, exact
classification/PASS lines, retained QEMU/substitute evidence, and diagnostic
marker quarantine requirements before implementation. The accepted QEMU syscall
smoke core adds a recoverable lower-AArch64 svc #0 routing boundary, mutates
saved x0 through the target-independent dispatch core, preserves the diagnostic
qemu-el0-trap-smoke proof, and retains QEMU/substitute serial evidence with
classification=qemu-syscall-smoke-complete and qemu-syscall-smoke: PASS. This
does not prove Pi 5 production syscall routing or unblock descriptor I/O,
copy-in/copy-out, process loading, filesystem, shell, networking, or SSH. The
accepted Phase 7.3 syscall routing closeout checkpoint reconciles those commits
and retained logs, closes out only the QEMU/substitute production syscall
routing frontier, and recommends a documentation-only Pi 5 syscall proof plan
before any serialized hardware action or before choosing copy-in/copy-out or
descriptor syscall work. The accepted Pi 5 syscall proof plan defines the
physical invariant for stable svc #0 talos_nop and unknown-syscall return
observations, diagnostic marker 0x7a10 quarantine, hardwareTestLock ownership,
fresh serial/TFTP evidence, candidate identity, inconclusive-run triage,
restoration requirements, and exact PASS/classification lines for the later
hardware proof. It does not acquire hardwareTestLock, publish an archive, run
Pi 5 hardware, or unblock descriptor I/O, copy-in/copy-out, process loading,
filesystem, shell, networking, or SSH.
The serialized Pi 5 syscall proof is now accepted. Retained local3 physical
serial evidence shows stable lower-AArch64 svc #0 reaching the production
syscall dispatch core on Pi 5: talos_nop returns x0 = 0, unknown syscall number
17 returns x0 = 0xffffffffffffffda (-ENOSYS), diagnostic marker 0x7a10 remains
outside production dispatch, and the proof reports
classification=pi5-syscall-proof-complete plus rpi5-syscall-proof: PASS. The
first candidate run was inconclusive, so the accepted evidence includes the
required same-candidate triage: candidate identity, fresh serial and TFTP
cursors, a passing production-timer known-good control, an unchanged candidate
rerun, and restore proof for the prior accepted boot tree. This accepts only
physical production routing for the first scalar syscall boundary; descriptor
I/O, copy-in/copy-out, process loading, filesystem, shell, networking, and SSH
remain blocked.
The accepted Pi 5 syscall proof closeout reconciles the accepted syscall ABI,
dispatch core, production trap routing, QEMU routing evidence, Pi 5 hardware
proof, hardware-lock timeline, restore proof, and deferred surfaces. It
accepts no new Rust or assembly behavior and performs no QEMU or Pi 5 rerun.
It recommends the documentation-only copy-in/copy-out helper contract as the
next bounded task before any pointer-taking syscall or descriptor I/O
implementation.
The accepted copy-in/copy-out helper contract defines target-independent
helper inputs, outputs, validation order, EFAULT mapping, all-or-nothing
partial-copy policy, recoverable versus process-fatal fault boundaries, and
unit-testable cases. It names phase7-copyin-copyout-helper-core-20260529 as
the next bounded implementation task and requires supervisor planning before
promotion because the current durable queue names only the contract task.
The accepted copy-in/copy-out helper core adds target-independent
copy_from_user and copy_to_user helpers in src/posix.rs. The helpers validate
the complete user range before byte movement, use UserAccessKind::Read for
copy-in and UserAccessKind::Write for copy-out, return the exact requested
length on success, map user-boundary failures to EFAULT, reserve EINVAL for
malformed kernel-side helper use, and preserve all-or-nothing behavior. Unit
tests cover success, zero-length, null guard, kernel range, wraparound, copy
limit, unmapped gaps, no-access mappings, permission mismatches,
backing-storage gaps, short kernel buffers, and destination preservation. The
copy-in/copy-out helper closeout reconciles this target-independent byte-copy
frontier and recommends phase7-pointer-taking-syscall-source-inventory-20260529
as the next bounded planning task. Pointer-taking syscalls, descriptor I/O,
process loading, filesystem, shell, networking, and SSH remain blocked until
later explicit tasks accept their contracts and gates. The accepted
pointer-taking syscall source inventory maps source owners and gaps for frame
argument extraction, syscall-number allocation, user-memory mapping
provenance, copy helper invocation, return/error encoding, QEMU smoke
ownership, and diagnostic-surface quarantine. It recommends supervisor planning
for phase7-pointer-taking-syscall-contract-20260529 before any implementation
or QEMU pointer-copy smoke plan; phase7-qemu-pointer-copy-smoke-plan-20260529
remains dependency-blocked until that contract is accepted. Descriptor I/O,
process loading, VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy
hardware proof remain blocked.
The accepted pointer-taking syscall contract fixes the first lower-EL
pointer-copy boundary as proof-only and QEMU/substitute scoped:
talos_copy_probe uses stable svc #0 with x8 = 0x7001 only in the later
qemu_pointer_copy_smoke scenario, assigns x0 as user pointer, x1 as length,
x2 as expected byte, x3 as replacement byte, and x4/x5 as reserved zeros, and
defines success, zero-length, -EFAULT, -EINVAL, and -ENOSYS observations. It
uses a fixed QEMU substitute UserData mapping/backing store at
0x0000_0000_0011_0000..0x0000_0000_0011_1000 and keeps diagnostic marker
0x7a10 proof-only. It unblocks only the documentation-only
phase7-qemu-pointer-copy-smoke-plan-20260529 task; descriptor I/O, process
loading, VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy
hardware proof remain blocked.
The accepted QEMU pointer-copy smoke plan defines the
qemu_pointer_copy_smoke QEMU/substitute invariant for proof-only
talos_copy_probe: fixed UserData backing storage, a 16-byte success case that
copies 0x2a bytes in and writes 0xa5 bytes back, a guard-range EFAULT case,
an unknown-syscall -ENOSYS regression, and diagnostic marker quarantine. The
accepted QEMU pointer-copy smoke core implements that boundary with
TALOS_BOOT_SCENARIO=qemu_pointer_copy_smoke, routes x8 = 0x7001 only in that
scenario, invokes the accepted copy_from_user and copy_to_user helpers, keeps
x8 = 0x7001 as -ENOSYS outside the proof scenario, and retains QEMU/substitute
serial evidence at
tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log
with classification=qemu-pointer-copy-smoke-complete and
qemu-pointer-copy-smoke: PASS. This accepts only QEMU/substitute pointer-copy
through lower-EL syscall routing; descriptor I/O, process loading,
VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy hardware proof
remain blocked.
The accepted pointer-copy closeout checkpoint reconciles the contract, smoke
plan, core implementation, retained QEMU evidence, regression gates, and
deferred surfaces. It accepts no new Rust or assembly behavior and performs no
QEMU or Pi 5 rerun. It recommends supervisor planning for a documentation-only
Pi 5 pointer-copy proof plan before any serialized hardware action or before
choosing descriptor syscall work.
The accepted Pi 5 pointer-copy proof plan defines the physical invariant for
proof-only talos_copy_probe on serialized Raspberry Pi 5 hardware: stable
svc #0 with x8 = 0x7001 must prove the 16-byte 0x2a-to-0xa5 success copy,
guard-range -EFAULT, unknown-syscall -ENOSYS, diagnostic marker quarantine,
hardwareTestLock ownership, fresh serial/TFTP evidence, candidate identity,
inconclusive-run triage, restoration proof, and exact
classification=pi5-pointer-copy-proof-complete plus
rpi5-pointer-copy-proof: PASS lines. It does not acquire hardwareTestLock,
publish an archive, run Pi 5 hardware, or unblock descriptor I/O, process
loading, filesystem, shell, networking, or SSH.
The serialized Pi 5 pointer-copy proof is now accepted. Retained local3
physical serial evidence shows stable lower-AArch64 svc #0 reaching the
proof-only talos_copy_probe path on Pi 5: the 16-byte success case returns
x0 = 16 and rewrites UserData from 0x2a to 0xa5, the guard-range request
returns x0 = 0xfffffffffffffff2 (-EFAULT), unknown syscall number 17 returns
x0 = 0xffffffffffffffda (-ENOSYS), diagnostic marker 0x7a10 remains outside
production dispatch, and the proof reports
classification=pi5-pointer-copy-proof-complete plus
rpi5-pointer-copy-proof: PASS. The first candidate run was inconclusive, so
the accepted evidence includes candidate identity, fresh serial and TFTP
cursors, a passing production-timer known-good control, an unchanged candidate
rerun, hardwareTestLock release, and restore proof for the prior accepted boot
tree. This accepts only the physical proof-only pointer-copy boundary;
descriptor I/O, process loading, filesystem, shell, networking, SSH, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, and stable POSIX descriptor
claims remain blocked.
The accepted Pi 5 pointer-copy proof closeout reconciles the accepted
QEMU/substitute pointer-copy evidence, Pi 5 hardware proof evidence,
hardware-lock timeline, restore proof, proof-only status, residual risks, and
deferred surfaces. It accepts no new Rust or assembly behavior and performs no
QEMU or Pi 5 rerun. It recommends the documentation-only descriptor syscall
source inventory as the next bounded task before any descriptor syscall
contract or implementation.
The accepted descriptor syscall source inventory maps the source owners and
gaps for descriptor table operations, lower-EL syscall argument extraction,
copy helper use, runtime-console/TTY backing, return/error encoding,
task/process ownership, and retained QEMU evidence style. It recommends the
next descriptor syscall contract slice as a stdout/stderr write boundary backed
by runtime-console0, while keeping stdin/read, close, dup, process loading,
VFS/filesystem, shell, networking, SSH, live process-owned address spaces,
blocking/readiness, signals, restart semantics, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and stable POSIX descriptor claims blocked.
The accepted descriptor syscall contract defines the first stable
descriptor-backed syscall slice as talos_write with x8 = 1, fd/user-pointer/
length arguments in x0/x1/x2, reserved zero x3 through x5, copy_from_user
validation, descriptor-table lookup and write-access checks, runtime-console0
as the only backing object, and exact byte-count or negative-errno returns. It
keeps stdin/read, close, dup, process loading, VFS/filesystem, shell,
networking, SSH, live process-owned address spaces, blocking/readiness,
signals, restart semantics, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor claims blocked.
The accepted QEMU descriptor-write smoke plan defines the
qemu_descriptor_write_smoke QEMU/substitute invariant for stable talos_write
x8 = 1: fd 1/fd 2 success through inherited stdio descriptors,
copy_from_user(), and runtime-console0, fd 0 and invalid-fd -EBADF, guard-range
-EFAULT, reserved-register -EINVAL, talos_nop and unknown-syscall regressions,
proof-only talos_copy_probe quarantine, and exact
classification=qemu-descriptor-write-smoke-complete plus
qemu-descriptor-write-smoke: PASS evidence for the later implementation. It
does not add implementation, QEMU, or hardware evidence and keeps stdin/read,
close, dup, process loading, VFS/filesystem, shell, networking, SSH, live
process-owned address spaces, blocking/readiness, signals, restart semantics,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and physical
descriptor-write claims blocked.
The accepted QEMU descriptor-write smoke core adds the
qemu_descriptor_write_smoke lower-AArch64 svc #0 scenario and retained
QEMU/substitute evidence for talos_write x8 = 1. The evidence proves fd 1 and
fd 2 write 18-byte UserData buffers through inherited stdio descriptors,
copy_from_user(), and runtime-console0; fd 0 and fd 99 return -EBADF without
additional console bytes; the guard range returns -EFAULT without console
bytes; a nonzero reserved register returns -EINVAL without console bytes;
talos_nop and unknown-syscall regressions remain intact; x8 = 0x7001 remains
quarantined as -ENOSYS outside proof scenarios; and the diagnostic marker
0x7a10 remains proof-only. This accepts only QEMU/substitute descriptor-backed
stdout/stderr write evidence. Pi 5 descriptor-write hardware proof,
stdin/read, close, dup, process loading, VFS/filesystem, shell, networking,
SSH, live process-owned address spaces, blocking/readiness, signals, restart
semantics, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and
full POSIX descriptor claims remain blocked.
The accepted descriptor-write closeout checkpoint reconciles the source
inventory, talos_write contract, smoke plan, descriptor-write core,
retained QEMU evidence, scalar/pointer-copy regression gates, residual risks,
and deferred surfaces. It recommends a documentation-only
phase7-pi5-descriptor-write-proof-plan-20260529 before any serialized Pi 5
descriptor-write hardware action, and keeps stdin/read, close, dup, process
loading, VFS/filesystem, shell, networking, SSH, live process-owned address
spaces, blocking/readiness, signals, restart semantics, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, physical descriptor-write claims,
and full POSIX descriptor claims blocked.
The accepted Pi 5 descriptor-write proof plan defines the physical invariant
for talos_write fd 1/fd 2 on serialized Raspberry Pi 5 hardware: stable svc #0
with x8 = 1 must write the stdout and stderr buffers through copy_from_user(),
inherited stdio descriptors, and runtime-console0; fd 0 and fd 99 must return
-EBADF; guard-range writes must return -EFAULT; nonzero reserved registers
must return -EINVAL; talos_nop and unknown-syscall regressions must remain
intact; talos_copy_probe x8 = 0x7001 and diagnostic marker 0x7a10 must remain
quarantined; and the proof must report
classification=pi5-descriptor-write-proof-complete plus
rpi5-descriptor-write-proof: PASS. It does not acquire hardwareTestLock,
publish an archive, run Pi 5 hardware, or unblock stdin/read, close, dup,
process loading, filesystem, shell, networking, or SSH.
The serialized Pi 5 descriptor-write proof is now accepted. Retained local3
physical serial evidence shows stable lower-AArch64 svc #0 reaching the
descriptor-write dispatch path on Pi 5: fd 1 stdout and fd 2 stderr write
18-byte UserData buffers through copy_from_user(), inherited stdio
descriptors, and runtime-console0; fd 0 and fd 99 return -EBADF without extra
console bytes; the guard range returns -EFAULT; a nonzero reserved register
returns -EINVAL; talos_nop and unknown-syscall regressions remain intact; x8 =
0x7001 remains quarantined as -ENOSYS; diagnostic marker 0x7a10 remains
proof-only; and the proof reports
classification=pi5-descriptor-write-proof-complete plus
rpi5-descriptor-write-proof: PASS. The first candidate run was inconclusive,
so the accepted evidence includes candidate identity, fresh serial and TFTP
cursors, a passing production-timer known-good control, an unchanged candidate
rerun, hardwareTestLock release, and restore proof for the prior accepted boot
tree. This accepts only the physical descriptor-backed stdout/stderr write
boundary; stdin/read, close, dup, process loading, filesystem, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor claims remain blocked.
The accepted Pi 5 descriptor-write proof closeout reconciles the accepted
QEMU/substitute descriptor-write smoke, physical Pi 5 descriptor-write proof,
hardware-lock timeline, restore proof, residual risks, and blocked surfaces. It
accepts no new Rust or assembly behavior and performs no QEMU or Pi 5 rerun.
It recommends the documentation-only Milestone 7.3 syscall ABI/dispatch
closeout checkpoint before any Milestone 7.4 file-descriptor-table source
inventory or broader descriptor work.
The accepted Milestone 7.3 syscall ABI/dispatch closeout reconciles scalar
syscall routing, QEMU and Pi 5 syscall proof, copy-in/copy-out helpers,
proof-only pointer-copy evidence, descriptor-write QEMU/Pi 5 evidence,
diagnostic-surface quarantine, hardware-lock/restore proof, residual risks, and
blocked surfaces. Milestone 7.3 is closed for the bounded lower-AArch64 svc #0
ABI and dispatch frontier: x8 syscall numbers, x0-through-x5 arguments, x0
return/-errno encoding, talos_nop, unknown-syscall -ENOSYS, copy helper
plumbing, proof-only talos_copy_probe, and talos_write fd 1/fd 2 to
runtime-console0. Process-owned descriptors, stdin/read, close, dup, program
loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and full POSIX descriptor readiness remain
blocked.
The accepted Milestone 7.4 file descriptor table source inventory maps the
source owners, accepted contracts, retained evidence, and missing contract
boundaries for moving from proof-owned inherited stdio descriptors to
process-owned descriptor tables. It recommends a documentation-only
phase7-process-descriptor-table-contract-20260529 as the next bounded task and
keeps stdin/read, close/dup syscalls, VFS/filesystem, path copying, process
loading, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor claims blocked.
The accepted process descriptor table contract defines the first
process-owned descriptor table boundary: a ProcessOwnerId-backed owner record,
inherited fd 0/fd 1/fd 2 installation, runtime-console0-backed stdout/stderr
identity, current-process descriptor-table lookup, and deterministic retained
descriptor errors. It recommends phase7-process-descriptor-table-core-20260529
as the next target-independent implementation task and keeps PID allocation,
process loading, close/dup/read syscalls, VFS/filesystem, stdin behavior,
shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, physical proof, and full POSIX descriptor claims
blocked.
The accepted process descriptor table core adds the first target-independent
process-owned descriptor owner/store surface. A ProcessOwnerId can now own one
inherited-stdio DescriptorTable in a bounded ProcessDescriptorStore, current
owner lookup maps missing current task/owner/table state to -EBADF for
descriptor syscalls, and focused unit tests preserve inherited stdio plus
retained descriptor-table errors. It adds no live syscall routing, QEMU or
Pi 5 proof, close/dup/read syscall behavior, process loading,
VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, or full POSIX descriptor claim.
The accepted QEMU process descriptor stdio smoke plan defines the next
QEMU/substitute proof boundary: lower-AArch64 talos_write fd 1/fd 2 must route
through a ProcessOwnerId-backed ProcessDescriptorStore and inherited stdio
DescriptorTable, with current-owner lookup evidence, retained fd/error
regressions, talos_nop and unknown-syscall regressions, proof-only
talos_copy_probe quarantine, diagnostic marker quarantine, exact
classification/PASS lines, and retained QEMU log path. It adds no
implementation, QEMU run, Pi 5 hardware action, or hardware-lock work and
keeps close/dup/read, process loading, VFS/filesystem, shell, networking, SSH,
physical proof, and full POSIX descriptor claims blocked.
The accepted QEMU process descriptor stdio smoke core adds the first
lower-AArch64 QEMU/substitute evidence for process-owned descriptor-table
lookup. The qemu_process_descriptor_stdio_smoke scenario creates
ProcessOwnerId 1, installs inherited stdio in ProcessDescriptorStore, resolves
the current owner through the accepted lookup API, routes talos_write fd 1/fd
2 to runtime-console0 through that table, and retains fd/error, scalar
syscall, copy-probe quarantine, diagnostic-marker quarantine, and PASS
evidence at
tasks/evidence/2026-05-29-qemu-process-descriptor-stdio-smoke-core/qemu-process-descriptor-stdio-smoke.log.
It remains QEMU/substitute evidence only; Pi 5 physical proof, stdin/read,
close/dup/read, process loading, VFS/filesystem, shell, networking, SSH, and
full POSIX descriptor claims remain blocked.
The accepted process descriptor table closeout reconciles the contract,
target-independent core, QEMU/substitute smoke plan, retained smoke evidence,
validation gates, residual risks, and blocked surfaces for the first
process-owned descriptor-table slice. It accepts ProcessOwnerId-backed
inherited stdio lookup and talos_write fd 1/fd 2 routing to runtime-console0
only at the QEMU/substitute evidence level. Pi 5 physical descriptor-table
proof, stdin/read, close/dup/read, descriptor lifetime and close semantics,
process loading, VFS/filesystem, shell, networking, SSH, and full POSIX
descriptor claims remain blocked. The next bounded Milestone 7.4 task should
be supervisor-planned as a documentation-only descriptor lifetime and
close-semantics source inventory before any close/dup/read syscall contract.
The accepted descriptor lifetime and close source inventory maps
DescriptorTable table-local close/dup primitives, DescriptorEntry/Object/Access
vocabulary, ProcessDescriptorStore owner-table mutation, inherited stdio
lifetime, accepted unit-test evidence, missing close/double-close/reuse/dup
evidence, and owner teardown/reference-count gaps. It recommends the
documentation-only phase7-descriptor-lifetime-close-contract-20260529 as the
next bounded Milestone 7.4 task. Close/dup/read syscalls, process loading,
VFS/filesystem, shell, networking, SSH, Pi 5 physical close/dup/read claims,
and full POSIX descriptor readiness remain blocked.
The accepted descriptor lifetime and close contract defines table-local slot
removal, process-owned mutable lookup through ProcessDescriptorStore, EBADF
error cases, dup/reuse interaction, and open-file-description reference-count
vocabulary. It recommends phase7-descriptor-close-core-20260529 as the next
target-independent Milestone 7.4 implementation task. Close/dup/read syscalls,
process loading, VFS/filesystem, shell, networking, SSH, Pi 5 physical
close/dup/read claims, object finalization, and full POSIX descriptor
readiness remain blocked.
The accepted descriptor close core closeout reconciles the accepted source
inventory, contract, target-independent implementation, focused unit-test
evidence, validation gates, and deferred surfaces for process-owned descriptor
close semantics. It accepts only ProcessDescriptorStore::close_current_descriptor()
applying table-local DescriptorTable::close() to the current owner, with EBADF
for missing/unknown owners and invalid, empty, or already closed descriptors.
Close/dup/read syscalls, lower-EL ABI, QEMU close/dup/read smoke, Pi 5 physical
close/dup/read proof, process loading, VFS/filesystem, shell, networking, SSH,
object finalization, and full POSIX descriptor readiness remain blocked. The
next bounded Milestone 7.4 task should be supervisor-planned as a
documentation-only close/dup/read syscall source inventory.
The accepted close/dup/read syscall source inventory maps the current syscall
dispatch, lower-EL routing, copy helper, ProcessDescriptorStore, DescriptorTable,
descriptor entry/object, runtime-console0, TTY, and stdin/read source owners.
It separates accepted process-owned descriptor-write and target-independent
close/dup/copy-helper evidence from unproven close, dup, and read syscalls.
Close is the smallest next user-visible descriptor operation because the
target-independent close helper is already accepted; dup and read still need
additional policy around fd allocation, read byte sources, EOF,
blocking/readiness, nonblocking behavior, signal/restart policy, and object
lifetime. The next bounded Milestone 7.4 task should be
phase7-close-syscall-contract-20260529. Dup/read, QEMU/Pi 5 close/dup/read
proof, process loading, VFS/filesystem, shell, networking, SSH, object
finalization, and full POSIX descriptor readiness remain blocked.
The accepted close syscall contract defines the first user-visible descriptor
close boundary: stable svc #0 with x8 = 2, descriptor argument in x0,
reserved-zero x1 through x5, x0 = 0 on success, -EBADF for missing/unknown
owners and invalid, empty, or already closed descriptors, and -EINVAL for
nonzero reserved arguments. The contract routes the later implementation
through ProcessDescriptorStore::close_current_descriptor() and preserves
talos_nop, talos_write, unknown-syscall, descriptor-write, and proof-only
pointer-copy quarantine behavior. The next bounded Milestone 7.4 task should
be phase7-close-syscall-core-20260529. Dup/read, QEMU/Pi 5 close/dup/read
proof, process loading, VFS/filesystem, shell, networking, SSH,
open-file-description finalization, and full POSIX descriptor readiness remain
blocked.
The accepted close syscall core adds stable syscall number 2 for talos_close
and a target-independent process descriptor dispatch helper. Close validates
reserved-zero x1 through x5, resolves the current process owner through
ProcessDescriptorStore, clears occupied descriptor slots with x0 = 0, returns
-EBADF for missing/unknown owner or invalid/empty/already-closed descriptor
cases, and returns -EINVAL for reserved-register violations. Focused no_std
tests prove stdout/stderr close, EBADF failures, no-mutation EINVAL, duplicate
slot preservation, and talos_write regression after close. The QEMU syscall
and descriptor-write smokes remain passing regressions. The accepted QEMU close
syscall smoke plan then fixed the lower-AArch64 QEMU/substitute invariant for
closing fd 1 and fd 2 through the current ProcessOwnerId-backed descriptor
table. The accepted QEMU close syscall smoke core retains that evidence:
qemu_close_syscall_smoke closes fd 1/fd 2 through
ProcessDescriptorStore::close_current_descriptor(), proves later talos_write on
closed descriptors returns -EBADF without runtime-console0 side effects, proves
fd 2 remains writable after fd 1 is closed and after a failed reserved close,
and preserves talos_nop, unknown-syscall, copy-probe quarantine, and
diagnostic-marker quarantine. Dup/read, Pi 5 physical close/dup/read proof,
process loading, VFS/filesystem, shell, networking, SSH, object finalization,
and full POSIX descriptor readiness remain blocked. The next bounded Milestone
7.4 task should be phase7-close-syscall-closeout-checkpoint-20260529.
The accepted close syscall closeout reconciles the accepted source inventory,
contract, target-independent core, QEMU close smoke plan, retained
QEMU/substitute close evidence, validation gates, and deferred surfaces. It
accepts only the current ProcessOwnerId-backed talos_close QEMU/substitute
frontier and does not add Rust behavior, QEMU rerun, Pi 5 hardware run, or
hardwareTestLock activity. Pi 5 physical close proof, dup/read syscalls,
process loading, VFS/filesystem, shell, networking, SSH, object finalization,
and full POSIX descriptor readiness remain blocked. The next bounded Milestone
7.4 task should be a documentation-only Pi 5 close syscall proof plan before
any serialized hardware action.
The accepted Pi 5 close syscall proof plan defines the serialized physical
rpi5_close_syscall_proof boundary for carrying the QEMU/substitute talos_close
invariant to hardware. It requires hardwareTestLock ownership, candidate
archive and kernel identity, fresh serial and TFTP evidence, restoration proof,
and exact observations for close(fd 1), close(fd 2), write-after-close -EBADF,
reserved-argument -EINVAL no-mutation, repeated/invalid close -EBADF,
talos_nop, unknown-syscall -ENOSYS, copy-probe quarantine, diagnostic-marker
quarantine, final classification, and PASS. This plan does not run hardware or
accept a physical close claim. The next bounded Milestone 7.4 task should be
phase7-pi5-close-syscall-proof-20260529. Dup/read, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, and full POSIX
descriptor readiness remain blocked.
The serialized Pi 5 close syscall proof is now accepted. Retained local19
hardware evidence carries the QEMU/substitute talos_close invariant to
Raspberry Pi 5: the focused rpi5_close_syscall_proof payload closes fd 1 and
fd 2 through the current ProcessOwnerId-backed ProcessDescriptorStore,
proves write-after-close returns -EBADF before runtime-console0 side effects,
preserves reserved-argument -EINVAL no-mutation, repeated/invalid close
-EBADF, talos_nop, unknown-syscall -ENOSYS, copy-probe quarantine, diagnostic
marker quarantine, final classification=pi5-close-syscall-proof-complete, and
PASS. The physical fix cleans the initialized ProcessDescriptorStore static to
PoC before the EL2-to-EL1/EL0 proof handoff; local18 showed the pre-fix EL1
handler saw owner-present=false. This accepts only the physical talos_close
proof. Dup/read, process loading, VFS/filesystem, shell, networking, SSH,
object finalization, and full POSIX descriptor readiness remain blocked. The
next bounded Milestone 7.4 task should be
phase7-pi5-close-syscall-proof-closeout-checkpoint-20260529.
The accepted Pi 5 close syscall proof closeout reconciles the close syscall
source inventory, contract, target-independent core, QEMU/substitute close
smoke, serialized Pi 5 physical proof, hardware-lock timeline, restore proof,
and deferred surfaces. It accepts only the physical talos_close proof for the
focused rpi5_close_syscall_proof scenario. Dup/read, process loading,
VFS/filesystem, stdin/read object policy, shell, networking, SSH, object
finalization, broader cache/DMA policy, and full POSIX descriptor readiness
remain blocked. The next bounded Milestone 7.4 task should be the already
queued documentation-only phase7-dup-syscall-contract-20260529.
The accepted dup syscall contract defines talos_dup as stable syscall number 3
on svc #0 with x0 as the source descriptor and x1 through x5 reserved zero. It
duplicates an occupied descriptor in the current ProcessOwnerId-backed
ProcessDescriptorStore into the lowest free slot, returns the new descriptor
number, maps invalid/empty/closed or missing-owner sources to -EBADF, maps a
full table to -EMFILE, and maps reserved arguments to -EINVAL without
mutation. The contract preserves talos_nop, talos_write, talos_close,
unknown-syscall, and proof-only copy-probe behavior, and recommends
phase7-dup-syscall-core-20260529 as the next bounded target-independent
implementation task. Read syscall behavior, stdin/read object policy, QEMU/Pi
5 dup/read proof, process loading, VFS/filesystem, shell, networking, SSH,
dup2/fcntl, object finalization, broader cache/DMA policy, and full POSIX
descriptor readiness remain blocked.
The accepted dup syscall core adds stable syscall number 3 for talos_dup and
routes it through dispatch_process_descriptor() and
ProcessDescriptorStore::dup_current_descriptor(). It validates reserved-zero x1
through x5, duplicates occupied source descriptors into the lowest free slot,
returns -EBADF for invalid, empty, closed, missing-owner, or unknown-owner
sources, returns -EMFILE for full tables, and returns -EINVAL for reserved
argument violations without mutation. Descriptor writes now rely on the copied
DescriptorEntry access and StdioOutput object kind, so duplicated stdout/stderr
descriptors can remain writable after the source is closed while stdin/read
behavior stays blocked. Focused no_std tests prove stdout/stderr/stdin
duplication cases, duplicate/source independence across close, no-mutation
reserved failures, full-table EMFILE, and existing nop/write/close/unknown
regressions. QEMU dup smoke, Pi 5 physical dup proof, read/stdin behavior,
process loading, VFS/filesystem, shell, networking, SSH, object finalization,
dup2/fcntl, and full POSIX descriptor readiness remain blocked. The next
bounded Milestone 7.4 task should be
phase7-qemu-dup-syscall-smoke-plan-20260529.
The accepted QEMU dup syscall smoke plan defines the bounded
qemu_dup_syscall_smoke substitute proof before lower-EL runtime evidence is
claimed. It requires a ProcessOwnerId-backed four-slot inherited stdio table,
current-owner lookup through ProcessDescriptorStore, talos_dup(fd 1) returning
fd 3, deterministic -EMFILE and -EINVAL cases, writes through source and
duplicate stdout descriptors, close-one-descriptor preservation, closed
descriptor -EBADF, talos_nop, unknown-syscall -ENOSYS, copy-probe quarantine,
diagnostic-marker quarantine, final
classification=qemu-dup-syscall-smoke-complete, and PASS. It does not run QEMU
or hardware. Pi 5 physical dup proof, read/stdin behavior, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, and full POSIX
descriptor readiness remain blocked. The next bounded Milestone 7.4 task
should be phase7-qemu-dup-syscall-smoke-core-20260529.
The accepted QEMU dup syscall smoke core adds qemu_dup_syscall_smoke and
retains lower-AArch64 QEMU/substitute evidence at
tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.
The smoke creates ProcessOwnerId 1 with a four-slot inherited stdio table,
routes talos_dup through the current ProcessDescriptorStore lookup, proves fd
1 duplicates to fd 3, table-full -EMFILE, reserved-register -EINVAL,
runtime-console0 writes through both source and duplicate descriptors,
close(fd 1) preserving fd 3, closed-descriptor -EBADF, talos_nop,
unknown-syscall -ENOSYS, copy-probe quarantine, diagnostic-marker quarantine,
final classification=qemu-dup-syscall-smoke-complete, and PASS. It is
QEMU/substitute evidence only. Pi 5 physical dup proof, read/stdin behavior,
process loading, VFS/filesystem, shell, networking, SSH, object finalization,
dup2/fcntl, and full POSIX descriptor readiness remain blocked. The next
bounded Milestone 7.4 task should be
phase7-dup-syscall-closeout-checkpoint-20260529.
The accepted QEMU dup syscall closeout checkpoint reconciles the dup contract,
target-independent core, QEMU smoke plan, retained QEMU/substitute dup
evidence, descriptor-write and close regression gates, residual risks, and
deferred surfaces. It accepts no new Rust or assembly behavior and performs no
QEMU or Pi 5 rerun. It recommends the already queued documentation-only
phase7-pi5-dup-syscall-proof-plan-20260529 before any serialized physical dup
proof. Pi 5 physical dup proof, read/stdin behavior, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, dup2/fcntl, and
full POSIX descriptor readiness remain blocked.
The accepted Pi 5 dup syscall proof plan defined the serialized physical
rpi5_dup_syscall_proof invariant, including hardwareTestLock ownership,
candidate identity, fresh serial/TFTP evidence, inconclusive-run triage,
restore proof, exact dup/write/close/error/quarantine/PASS lines, and blocked
deferred surfaces. The serialized Pi 5 dup syscall proof is now accepted.
Retained local8 physical evidence proves fd 1 duplicates to fd 3,
full-table -EMFILE, reserved-register -EINVAL, writes through source and
duplicate stdout descriptors, close(fd 1) preserving fd 3, duplicate close,
closed-descriptor -EBADF, scalar and unknown-syscall regressions, copy-probe
quarantine, final
classification=pi5-dup-syscall-proof-complete, and PASS. Retained local7
production-timer control evidence proves lab health after the earlier
inconclusive candidate/control runs. Read/stdin behavior, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, dup2/fcntl, and
full POSIX descriptor readiness remain blocked.
The accepted Pi 5 dup syscall proof closeout reconciles the QEMU and Pi 5 dup
frontier, retained local7/local8 evidence, archive/TFTP identity, restore
proof, hardware-lock timeline, residual risks, and deferred surfaces. The
accepted read/stdin source inventory maps the current owners for syscall
dispatch, copy_to_user/user-memory validation, ProcessDescriptorStore lookup,
inherited fd 0, runtime-console0, TTY/stdin surfaces, and retained
write/close/dup evidence. It lists the unresolved read/stdin policy gaps for
byte source, EOF, blocking/readiness, partial reads, nonblocking mode,
restart/signals, copy-out failure handling, object lifetime/finalization, and
physical proof. It accepts no read behavior and recommends the documentation-only
phase7-read-stdin-contract-20260529 as the next bounded Milestone 7.4 task.
The accepted read/stdin contract defines talos_read as stable syscall number 4
with x0 fd, x1 destination pointer, x2 requested count, x3 through x5 reserved
zero, and x0 byte-count/0 EOF or negative errno return encoding. The first
bounded stdin source is fixed proof input shared by fd 0 and duplicates of
fd 0, with immediate readiness, proof-buffer short reads, 0 at bounded EOF,
copy_to_user all-or-nothing failure ordering, and no runtime-console0, TTY,
filesystem, pipe, socket, signal, wait-queue, or hardware input claim. It
recommends phase7-read-stdin-core-20260529 as the next bounded
target-independent implementation task.
The accepted read/stdin core adds stable syscall number 4 for talos_read,
`FixedStdin` proof-buffer state, target-independent descriptor dispatch through
`ProcessDescriptorStore`, and focused no_std coverage for fd 0 and duplicated
stdin reads, proof-buffer short reads, 0 EOF, reserved-register -EINVAL,
copy-out -EFAULT without cursor advance, fd/error -EBADF, non-stdin
readable-object -ENOTSUP, missing fixed source -ENOTSUP, and
scalar/write/close/dup/unknown/copy-probe regressions. This is
target-independent implementation
evidence only. QEMU/substitute lower-AArch64 read evidence, Pi 5 physical read
proof, runtime-console0/TTY/hardware stdin, process loading, VFS/filesystem,
shell, networking, SSH, object finalization, dup2/fcntl, signals, wait queues,
nonblocking I/O, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor readiness remain blocked. The accepted QEMU
read/stdin smoke plan defines the qemu_read_stdin_smoke QEMU/substitute
invariant for stable talos_read x8 = 4: fd 0 duplication, fixed proof stdin
length/cursor validation, copy-out -EFAULT, reserved-register -EINVAL,
fd/error -EBADF, fd 0 success copying talos, duplicated-fd short read copying
-stdin-qemu\n, bounded EOF, talos_nop and unknown-syscall regressions,
copy-probe quarantine, diagnostic-marker quarantine, and exact
classification=qemu-read-stdin-smoke-complete plus PASS lines. It does not run
QEMU or hardware. Pi 5 physical read proof, runtime-console0/TTY/hardware
stdin, process loading, VFS/filesystem, shell, networking, SSH, object
finalization, dup2/fcntl, signals, wait queues, nonblocking I/O, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, and full POSIX descriptor
readiness remain blocked. The next bounded Milestone 7.4 task should be
phase7-qemu-read-stdin-smoke-core-20260529.
The accepted QEMU read/stdin smoke core adds qemu_read_stdin_smoke and
retained QEMU/substitute evidence for lower-AArch64 stable talos_read x8 = 4.
It proves current-owner ProcessDescriptorStore lookup, fd 0 duplication to
fd 3, fixed proof stdin bytes talos-stdin-qemu\n, copy-out -EFAULT without
cursor advance, reserved-register -EINVAL without mutation, fd/error -EBADF,
fd 0 read success copying talos, duplicated-fd short read copying
-stdin-qemu\n, bounded EOF, talos_nop and unknown-syscall regressions,
copy-probe quarantine, diagnostic-marker quarantine, and
classification=qemu-read-stdin-smoke-complete plus PASS. This is
QEMU/substitute evidence only. Pi 5 physical read proof,
runtime-console0/TTY/hardware stdin, process loading, VFS/filesystem, shell,
networking, SSH, object finalization, dup2/fcntl, signals, wait queues,
nonblocking I/O, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor readiness remain blocked. The next bounded
Milestone 7.4 task should be
phase7-read-stdin-closeout-checkpoint-20260529.
The accepted read/stdin closeout checkpoint reconciles the source inventory,
contract, target-independent core, QEMU/substitute smoke plan, retained
qemu_read_stdin_smoke evidence, regression gates, residual risks, and
deferred surfaces. It keeps accepted behavior bounded to fixed proof stdin on
the target-independent and QEMU/substitute lower-AArch64 paths, with retained
classification=qemu-read-stdin-smoke-complete plus PASS evidence. Pi 5
physical read proof, runtime-console0/TTY/hardware stdin, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, dup2/fcntl,
signals, wait queues, nonblocking I/O, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor readiness remain blocked.
The next mechanically derivable task should be the documentation-only
phase7-pi5-read-stdin-proof-plan-20260530, queued by the supervisor before any
hardware action.
The accepted Pi 5 read/stdin proof plan and serialized proof carry the
QEMU/substitute fixed-stdin talos_read invariant to Raspberry Pi 5 hardware.
Retained local5 evidence ties the unchanged fd2be8e candidate archive to a
114816-byte da591740/kernel_2712.img TFTP fetch, serial output proving fd 0
read, duplicated fd 3 short read, EOF, -EFAULT/-EINVAL/-EBADF error cases,
talos_nop and unknown-syscall regressions, copy-probe quarantine,
diagnostic-marker quarantine, final
classification=pi5-read-stdin-proof-complete, and PASS, followed by restore
proof for the prior accepted 104136-byte boot tree. The accepted claim remains
limited to fixed proof stdin in the focused rpi5_read_stdin_proof scenario.
runtime-console0/TTY/hardware stdin, process loading, VFS/filesystem, shell,
networking, SSH, object finalization, dup2/fcntl, signals, wait queues,
nonblocking I/O, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor readiness remain blocked.
The accepted Pi 5 read/stdin proof closeout reconciles the QEMU/substitute
read/stdin evidence, Pi 5 hardware proof evidence, hardware-lock timeline,
restore proof, fixed-proof-stdin status, residual risks, and deferred surfaces.
It accepts no new Rust or assembly behavior and performs no QEMU or Pi 5
rerun. The next mechanically derivable Milestone 7.4 task is the already queued
phase7-file-descriptor-table-closeout-checkpoint-20260530, and no Phase 8
transition is claimed by this closeout.
The accepted Milestone 7.4 file descriptor table closeout reconciles
process-owned inherited stdio, descriptor-backed stdout/stderr writes,
descriptor lifetime/close semantics, stable talos_close, stable talos_dup,
fixed-proof-stdin talos_read, QEMU/substitute evidence, serialized Pi 5
physical evidence, hardware-lock/restore records, residual risks, and blocked
surfaces. Milestone 7.4 is closed only for the bounded descriptor-table
frontier: ProcessOwnerId-backed inherited stdio, runtime-console0-backed fd 1
and fd 2 writes, close, dup, fixed proof stdin through fd 0/fd 3, scalar
regressions, and diagnostic-surface quarantine. runtime-console0/TTY/hardware
stdin, pipes, sockets, regular files, VFS/filesystem, process loading, shell,
networking, SSH, object finalization, dup2/fcntl, signals, wait queues,
nonblocking I/O, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor readiness remain blocked. The next objective task
should be a supervisor-planned Phase 7 final closeout or frontier checkpoint
before any Phase 8 transition is considered.
The accepted Phase 7 final frontier source inventory reconciles the accepted
Phase 7.1 POSIX baseline, Phase 7.2 lower-EL/address-space proof, Phase 7.3
syscall/copy boundary, and Phase 7.4 descriptor-table work by commit and
evidence level. It identifies no remaining bounded Phase 7 implementation or
evidence task before the final closeout checkpoint, but it does not set a
Phase 8 transition flag or accept filesystem/program-loading behavior. The next
mechanically unblocked queued task is
phase7-final-closeout-checkpoint-20260530.
The accepted Phase 7 final closeout checkpoint closes Phase 7 for that bounded
frontier and records the durable recommendation flag for the first Phase 8
source-inventory task. It does not implement or accept filesystem/program
loading, shell, networking, SSH, runtime-console0/TTY or hardware stdin,
object finalization, RP1/PCIe, UART interrupt ownership, DMA/cache-driver
policy, or full POSIX readiness.

Near-term direction after the accepted Phase 7 closeout:

- Start with the accepted Phase 8 filesystem/program-loading source inventory
  as the source-owner and gap map for Milestone 8.1.
- The next recommended task is the documentation-only
  phase8-readonly-initramfs-vfs-contract-20260530 contract. It should define
  the read-only initial filesystem/VFS boundary before ELF/program loading or
  shell work.
- Keep QEMU, host-side unit tests, and static documentation gates first. Reserve
  serialized Pi 5 runs for the smallest physical claim that cannot be proven on
  the QEMU/substitute path.
- Preserve the deferred-surface boundary: runtime-console0/TTY/hardware stdin,
  process loading, descriptor I/O beyond the accepted write/close/dup/read
  frontiers, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, object finalization, dup2/fcntl, signals, wait queues,
  nonblocking I/O, and DMA/cache-driver policy remain out of scope until
  explicit tasks accept their contracts and gates.
- Treat the roadmap target as a usable local operating system: TTY, shell,
  separate user programs, and interaction/program-based tests that exercise new
  kernel features through the normal kernel/userspace boundary.

The recently accepted Phase 6.3 scheduler frontier includes
evidence-retention, diagnostic-surface, roadmap-refresh,
productionization-boundary, CPU-local scheduler service boundary,
CPU-local scheduler service core, CPU-local scheduler service closeout,
secondary scheduler service-loop source inventory, service-loop core, QEMU
smoke, Pi 5 proof, service-loop closeout, shared run-queue/migration source
inventory, contract, core, QEMU proof, Pi 5 proof, closeout,
load-balancing source inventory, load-balancing policy contract,
load-balancing core, QEMU load-balancing proof, Pi 5 load-balancing proof,
load-balancing closeout, multi-core preemption source inventory, contract,
core, QEMU proof, Pi 5 proof, closeout, production scheduler runtime source
inventory, production timer/preemption contract, production timer/preemption
core, QEMU production timer/preemption proof, Pi 5 production
timer/preemption proof, and production scheduler runtime closeout tasks are
accepted. The accepted
shared
run-queue/migration slice provides a bounded source-owner publish and
destination-owner consume path with explicit migration states and deterministic
errors; QEMU reports
classification=qemu-shared-runqueue-migration-complete, and serialized Pi 5
hardware reports all four physical-core participants completing the named
invariant with
classification=pi5-shared-runqueue-migration-complete. The accepted
load-balancing source inventory identifies policy inputs, freshness checks,
failure modes, and the split between target selection, fairness/affinity,
remote reschedule notification, and the existing migration mechanism. The
accepted load-balancing policy contract keeps the first policy deterministic,
runnable-only, SharedRunQueue-backed, and polling-only unless a later task
adds remote reschedule notification. The accepted load-balancing core adds
target-independent front-runnable selection and publication through
SharedRunQueue with deterministic unit-tested rejection paths. QEMU
load-balancing proof is accepted with
classification=qemu-load-balancing-smoke-complete, proving deterministic
front-runnable selection, source-local removal, shared handoff,
destination-local enqueue, metadata refresh, and PASS. Serialized Pi 5
load-balancing proof is accepted with
classification=pi5-load-balancing-complete for the same named invariant.
The accepted load-balancing closeout preserves those retained gates and
recommends multi-core preemption source inventory as the next bounded
Phase 6.3 task. The accepted multi-core preemption source inventory maps the
timer IRQ, owner-local scheduler service, secondary service-loop, IPI/wake,
metadata, SharedRunQueue, and load-balancing boundaries that the next contract
must preserve. The accepted multi-core preemption contract keeps timer/IPI
paths as bounded recorders and requires owner-local normal control flow to
perform scheduler mutation after interrupt return; it names deterministic
defer/reject outcomes for stale metadata, wrong-owner access,
nested/preemption-disabled sections, pending remote wake, and full queues.
The accepted multi-core preemption core adds target-independent per-owner
pending timer-preemption state, duplicate request coalescing, explicit nested
preemption-disable defer behavior, and an owner-local service entry that
preflights owner/current-task authority before draining wake queues or
mutating scheduler state. The accepted QEMU multi-core preemption proof adds
qemu_multicore_preemption_smoke and
scripts/qemu-multicore-preemption-smoke.sh; logical CPUs 1, 2, and 3 each
record only local pending timer-preemption state, prove the record step does
not mutate scheduler state, then service the request through owner-local normal
control flow with classification=qemu-multicore-preemption-smoke-complete. Pi
5 proof reports classification=pi5-multicore-preemption-complete,
participants=3, expected=3, errors=0, and PASS for the same invariant. The
accepted multi-core preemption closeout preserves the retained gates and
requires a new bounded productionization task before any further scheduler
runtime integration or Phase 7 work. The accepted production scheduler
runtime source inventory maps those retained diagnostic surfaces against the
normal boot, timer, and owner-local runtime paths. The accepted production
timer/preemption contract, core, focused QEMU proof, serialized Pi 5 proof,
and closeout checkpoint establish the first production timer IRQ recording
and owner-local service boundary. Normal QEMU and Pi 5 timer IRQ handlers now
record bounded local production preemption state, and
ProductionSchedulerRuntime services pending preemption only from owner-local
normal control flow. The Pi 5 proof reports
classification=pi5-production-timer-preemption-complete, participants=3,
expected=3, errors=0, and PASS. The accepted Phase 7 POSIX contract source
inventory maps the scheduler task/process boundary, runtime-console and TTY
stdio direction, diagnostic command limitations, lower-EL readiness limits,
and retained validation gates that constrain the POSIX baseline contract. The
accepted Phase 7 POSIX contract baseline defines the first errno-style names,
lexical path normalization semantics, process lifetime vocabulary,
descriptor-operation vocabulary, stdio inheritance shape, early loader
argument/environment vocabulary, and target-independent test seams. The
accepted Phase 7 path/error model core implements the first no_std lexical path
normalizer and PosixError vocabulary. The accepted descriptor-table contract
keeps descriptors process-local, separates descriptor entries from underlying
kernel objects, fixes close/dup and inherited stdio edge cases, and blocks
runtime console/TTY I/O integration until a later explicit task. The accepted
descriptor-table core adds the first fixed-capacity process-local descriptor
table data model with inherited stdio entries, allocation, exact-slot
allocation, lookup, close, dup, access checks, reserved object kinds, and
deterministic PosixError results, all covered by target-independent no_std unit
tests. The accepted Phase 7.1 closeout checkpoint reconciles this evidence and
keeps EL0, syscall, VFS, filesystem, program-loader, descriptor I/O,
networking, SSH, and shell work deferred for supervisor-planned tasks.
The accepted Phase 7.2 EL0/address-space source inventory maps exception
vectors and saved frames, same-EL ERET diagnostics, the broad EL2 identity map,
page-frame ownership, scheduler task/process separation, PosixError/EFAULT
vocabulary, descriptor-table ownership, retained gates, and implementation gaps
before a lower-EL contract. The accepted Phase 7.2 EL0 trap/address-space
contract defines the canonical user range below 0x0000_8000_0000_0000, null
guard, user text/data/heap/stack/guard vocabulary, kernel-only mapping policy,
validated user trap-return frame requirements, user fault classes, and
copy-in/copy-out preconditions. The accepted user-memory permission core adds
target-independent user range, mapping permission, and access validation with
unit coverage for null, wraparound, kernel-range, guard, unmapped, permission,
and length-limit cases. The accepted QEMU EL0 trap smoke plan fixes the first
QEMU-only lower-EL proof invariant and expected output:
classification=qemu-el0-trap-smoke-complete and qemu-el0-trap-smoke: PASS
after a built-in EL0 payload executes diagnostic SVC marker 0x7a10 and the
kernel reports saved user state. The accepted QEMU EL0 trap smoke core
implements that bounded scenario, retaining serial evidence at
tasks/evidence/2026-05-28-qemu-el0-trap-smoke-core/qemu-el0-trap-smoke.txt
with the saved lower-AArch64 synchronous trap state, final classification, and
PASS. The serialized Pi 5 EL0 trap proof is also accepted: retained physical
serial evidence in
tasks/evidence/2026-05-28-pi5-el0-trap-proof/local62-clean-final-lower-el0-trap/
reports the Pi 5 translation feature registers, regular VBAR_EL1 handoff,
lower-AArch64 synchronous SVC trap state, classification=pi5-el0-trap-proof-complete,
and rpi5-el0-trap-proof: PASS. This proves the bounded lower-EL trap path on
hardware only; general syscall ABI, process loading, descriptor I/O,
filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain deferred. The accepted Phase 7.3 syscall ABI
source inventory maps SVC exception decoding, proof-marker boundaries, syscall
number and argument register gaps, return/error convention gaps, user-copy
preconditions, descriptor-table interaction, and process/task ownership before
any syscall implementation. The accepted Phase 7.3 syscall ABI contract fixes
the first stable syscall boundary: lower-AArch64 svc #0, syscall number in x8,
scalar arguments in x0 through x5, x0 as the sole return register, negative
errno returns, talos_nop = 0, and unknown syscall = -ENOSYS. The accepted
Phase 7.3 target-independent syscall dispatch core implements the pure
dispatch vocabulary and return/error encoding, and the accepted trap-routing
source inventory maps the production exception-routing boundary for lower-EL
SVC detection, argument capture, return mutation, ELR/SPSR handling,
diagnostic marker quarantine, and non-syscall fallback. The accepted
trap-routing contract limits production routing to lower-AArch64 svc #0,
requires x8/x0-through-x5 dispatch into the accepted core, preserves ELR/SPSR,
keeps diagnostic 0x7a10 proof-only, and requires QEMU syscall smoke before any
production routing claim.
The obsolete-bloat inventory and removal sweep are accepted before the
multi-core preemption core: historical QEMU secondary-core discriminator paths
and old Pi 5 allocator, exception, panic, and translation-fault proof-only
scripts/cfg/source paths are retired while accepted evidence summaries remain
in task records. Direct IRQ/IPI-context scheduling, running-task migration,
non-diagnostic secondary runtime roles, EL0/syscalls, filesystem, networking,
SSH, and shell work remain deferred until the next explicit bounded task accepts
them.

Accepted status and historical completed facts:

- Talos project directory created separately from Daedalus.
- mdBook documentation skeleton created.
- Lab controller documented and reachable from OpenClaw at http://talos-lab-api:8080.
- TFTP boot archive publishing and PoE control API are documented.
- Minimal Rust no_std AArch64 kernel skeleton created for QEMU virt.
- Pi 5 target definition and target boundary stubs created.
- Early target service descriptors added for boot info, UART kind, timer kind,
  interrupt-controller kind, MMIO map, and device tree pointer.
- Pi 5 kernel image and boot-tree staging scripts added for local archive
  preparation.
- Physical Pi 5 first-light reached Talos code.
- Readable Talos-origin serial output is available through the lab controller.
- Exception and panic diagnostics report useful AArch64 state.
- The Pi 5 boot path parses firmware handoff state and DTB memory metadata.
- Early EL2 stage-1 translation, instruction cache, and data cache have booted
  on hardware while preserving serial output.
- A no-free bootstrap allocator and narrow Rust alloc-crate diagnostics for
  Box, Vec, String, and alloc-backed formatting have hardware evidence.
- Phase 3 has an accepted closeout checkpoint for the current memory, MMU, and
  kernel-runtime boundary. The checkpoint recommends planning Phase 4 next while
  preserving explicit deferrals for high memory, DMA/cache ownership, lower-EL
  userspace, SMP, filesystem/userland, and networking.
- Phase 4 has a source-backed interrupt/timer inventory naming the first QEMU
  virt and Pi 5 GICv2 plus ARM generic-timer targets.
- QEMU virt has a focused EL2 timer-interrupt smoke: with virtualization
  enabled, CNTHP_*_EL2 raises PPI 10 / INTID 26 through GICv2, the current-EL
  IRQ frame path acknowledges and EOIs it, and execution returns to a bounded
  post-IRQ workload.
- Pi 5 has a focused EL2 timer-interrupt smoke using the same CNTHP_*_EL2 /
  PPI 10 / INTID 26 shape through GIC-400. Serialized lab evidence shows the
  candidate image was fetched, the IRQ handler acknowledged and EOI'd INTID 26,
  and execution returned to a bounded post-IRQ workload.
- The Phase 4 timer-smoke checkpoint reconciles the accepted QEMU and Pi 5
  evidence, and monotonic tick accounting now reprograms the EL2 physical
  timer for four periodic ticks on QEMU and Pi 5 before reporting outside the
  IRQ path.
- Phase 4.1/4.2 has a pre-scheduler closeout checkpoint covering the accepted
  interrupt-controller, EL2 physical timer, periodic tick, and single-core
  interrupt-mask/restore boundary. Milestone 4.3 may start with a bounded
  scheduler-shape task that checks task/process terminology against the early
  POSIX note before committing scheduler structs.
- Phase 4.3 scheduler shape is accepted as a single-core, kernel-thread-first
  boundary. The next bounded implementation step is scheduler structs and a
  runnable queue, without context switching, preemption time slicing, SMP,
  userspace, file descriptors, console/TTY, filesystem, networking, or SSH.
- Phase 4.3 now has the first scheduler data structures: scheduler-local task
  IDs, kernel-thread state, per-task kernel stack and context placeholders, an
  optional future process-owner hook, a fixed single-core runnable queue, and
  unit tests for the queue/state invariants. Context switching, sleep queues,
  preemption, SMP, userspace, descriptors, console/TTY, filesystem, networking,
  and SSH remain deferred.
- Phase 4.3 has a documented EL2 cooperative context-switch contract for
  single-core kernel threads. The first QEMU context-switch smoke is accepted:
  two kernel-thread contexts with separate stacks make bounded progress through
  the AArch64 save/restore primitive, and the implementation reports switch,
  current-task, and runnable-task state outside the switch hot path.
- Phase 4.3 voluntary-yield dispatch is accepted in QEMU. The single-core
  scheduler can requeue a running task, select the next runnable task, count
  voluntary yields and dispatch switches, and cross the cooperative switch
  boundary while keeping the short scheduler mutation window IRQ-masked.
  Timer-driven preemption and async exception-frame switching remain deferred.
- The Phase 4.3 preemption-entry policy checkpoint is accepted. The next bounded
  task may attempt a QEMU-only timer-preemption smoke that preserves
  acknowledge/reprogram/EOI ordering, keeps scheduler switching and diagnostics
  out of the IRQ hot path, and remains single-core EL2 kernel-thread only.
- Phase 4.3 QEMU timer-preemption smoke is accepted. EL2 timer ticks now record
  bounded preemption requests in the IRQ hot path, then kernel-thread code
  performs scheduler dispatch and context switching outside IRQ context. Two
  QEMU kernel threads make progress from timer-driven preemption with zero
  voluntary-yield dispatches.
- Phase 4.3 Pi 5 timer-preemption hardware proof is accepted. The physical Pi
  5 fetched the 103,152-byte candidate kernel over TFTP, reached the EL2
  timer-preemption smoke, and reported task1=3, task2=3, ticks=6, requests=6,
  handled=6, timer-preemptions=6, dispatch-switches=6, voluntary-yields=0,
  INTID 26, unexpected=0, and PASS before the pre-run boot snapshot was
  restored.
- Phase 4.3 scheduler/preemption contract consolidation is accepted. The
  production boundary is the single-core scheduler data model, short
  IRQ-masked scheduler mutation windows, and an IRQ hot path limited to
  acknowledge/classify/tick/request/reprogram/EOI. The QEMU and Pi 5
  timer-preemption boot images remain validation surfaces, not supported
  kernel interfaces.
- Phase 4 closeout is accepted. The checkpoint reconciles the accepted QEMU and
  Pi 5 interrupt/timer/preemption evidence, names remaining deferrals and
  risks, and allows Phase 5 planning to start with a bounded local console
  device-model source inventory.
- Phase 5 console device-model source inventory is accepted. The current early
  logging surfaces are inventoried, the early/runtime console ownership
  boundary is documented, and descriptor/TTY compatibility constraints are
  named without implementing descriptor tables, input, userspace, filesystem,
  networking, SSH, or shell behavior.
- Phase 5 runtime console write core and write-result contract are accepted.
  Normal kernel output now routes through the named
  `runtime_console::write_default_console_output` boundary while preserving
  `print!` / `println!` and the existing target-owned polling PL011 backends.
  Pi 5 normal serial output is intended to be preserved through the existing
  firmware-preserved UART10 backend.
- Phase 5 default console identity is accepted. The output-side runtime console
  is named `runtime-console0`; later `stdout` and `stderr` descriptors
  should attach to that console through descriptor-owned handles instead of
  calling target backends directly.
- Phase 5 console input-source inventory is accepted. QEMU PL011 polling RX is
  the recommended first input implementation proof; Pi 5 input should follow
  only with serialized hardware evidence, preferably starting from the accepted
  UART10 console path before revisiting RP1 UART0 risk.
- Phase 5.1 console model checkpoint is accepted. The console model is
  output-capable and input-planned: normal diagnostics route through
  runtime-console0, target modules own QEMU/Pi 5 PL011 backend selection,
  and Milestone 5.2 may start with a documentation-only TTY/stdio shape task.
- Phase 5.2 TTY/stdio shape is accepted as a design boundary. Raw mode,
  canonical-lite line assembly, newline/backspace/echo/control-character
  policy, and descriptor-facing stdin/stdout/stderr shape are documented.
- Phase 5.2 QEMU polling TTY RX, the shared line-discipline core, the internal
  console input result contract, and the Pi 5 UART10 polling RX proof are
  accepted. QEMU and Pi 5 both use the same injected byte sequence through the
  runtime-console/TTY boundary and report deterministic echo, line, truncation,
  and control-event evidence without adding descriptors, syscalls, userspace,
  shell behavior, UART interrupts, networking, SSH, or scheduler blocking I/O.
- The Phase 5.2 TTY/stdio closeout checkpoint is accepted. The next
  supervisor-planned slice should be a Milestone 5.3 local kernel diagnostic
  command-channel source inventory, not an implementation shortcut around the
  accepted runtime-console and TTY boundaries.
- Phase 5.3 local diagnostic command-channel source inventory is accepted. The
  command channel must consume completed TTY lines, write bounded responses
  through runtime-console0, classify existing diagnostics before exposing them,
  and remain separate from descriptor/syscall/POSIX shell semantics.
- Phase 5.3 diagnostic command-channel contract is accepted. The
  target-independent parser/dispatcher consumes complete TTY lines, bounds
  command and argument tokens, exposes deterministic help/list/status responses,
  reports unknown and malformed commands, and keeps the response sink attached
  to runtime-console0 without adding a shell, descriptor table, syscall ABI,
  filesystem command execution, networking, SSH, SMP, UART interrupts, or
  scheduler blocking I/O.
- Phase 5.3 QEMU diagnostic command-channel smoke is accepted. The QEMU serial
  transcript proves `help`, `list`, deterministic unknown-command handling,
  and `status` through the accepted polling TTY line path and
  runtime-console0 response sink without adding Pi 5 hardware behavior,
  descriptors, syscalls, userspace shell behavior, filesystem-backed commands,
  networking, SSH, SMP, UART interrupts, or scheduler blocking I/O.
- Phase 5.3 Pi 5 diagnostic command-channel proof is accepted. The serialized
  UART10 hardware transcript proves the same `help`, `list`, `bogus`, and
  `status` command sequence through canonical-lite TTY input and
  runtime-console0 responses, with TFTP evidence tying the output to the
  staged candidate image.
- The Phase 5.3 diagnostic command-channel closeout checkpoint is accepted.
  Milestone 5.3 now has reconciled source inventory, parser/dispatcher
  contract, QEMU smoke, and Pi 5 UART10 hardware proof evidence. The accepted
  command channel remains kernel-owned and diagnostic-only; descriptor tables,
  syscalls, userspace shell behavior, filesystem-backed commands, networking,
  SSH, SMP, UART interrupts, RP1 UART0, and scheduler blocking I/O remain
  deferred.
- Phase 6.1 secondary-core bring-up source inventory and contract is accepted.
  PSCI with the firmware/DTB SMC conduit is the default bring-up path;
  spin-table and custom mailbox approaches remain fallback research. Before
  scheduler work, each secondary core must prove MPIDR/logical identity,
  exclusive stack ownership, per-core state registration, and controlled
  handoff.
- Phase 6.1 QEMU secondary-core discriminator is accepted. Under QEMU virt with
  EL2 virtualization enabled, PSCI `CPU_ON` through SMC starts secondary CPUs
  1, 2, and 3; each reports distinct MPIDR affinity, runs on its reserved
  stack, reaches `handoff-ready`, and parks without claiming Pi 5 hardware
  behavior.
- Phase 6.1 Pi 5 PSCI secondary-core alive proof is accepted. Serialized
  hardware evidence shows the Pi fetched the 90,784-byte candidate image and
  cores 1, 2, and 3 reported MPIDR affinities `0x100`, `0x200`, and
  `0x300`, distinct owned stack slots, `handoff-ready` state, and
  `pi5-psci-smc-secondary-cores-alive` before the pre-run boot snapshot was
  restored.
- Phase 6.1 controlled secondary-core workload is accepted. QEMU and serialized
  Pi 5 hardware evidence show secondary cores 1, 2, and 3 reach
  `workload-complete` with `progress=64 target=64 ok=true` through the
  accepted PSCI/trampoline/stack boundary while the production scheduler
  remains single-core.
- The Phase 6.1 secondary-core bring-up closeout checkpoint is accepted.
  Milestone 6.1 now has reconciled source inventory, QEMU discriminator,
  per-core state/stacks, Pi 5 PSCI alive proof, and controlled secondary-core
  workload evidence. SMP-safe primitives, scheduler migration, shared run
  queues, cross-core wakeups, userspace, descriptors, filesystem, networking,
  SSH, shell behavior, UART interrupts, RP1/PCIe, and DMA/cache policy remain
  deferred.
- Phase 6.2 SMP-safe primitives source inventory and contract is accepted. It
  separates local IRQ masking, SMP mutual exclusion, memory ordering, and cache
  maintenance; carries forward the accepted Pi 5 cache-maintenance lesson; and
  names `phase6-spinlock-barrier-core-20260524` as the first bounded
  implementation task before scheduler migration or shared run queues.
- Phase 6.2 spinlock/barrier core is accepted. `src/smp_sync.rs` provides a
  narrow `SpinLock<T>`, RAII guard, AArch64 IRQ-save lock composition, and a
  named `dmb ish` full-barrier boundary without wiring scheduler migration,
  shared run queues, cross-core wakeups, or cache maintenance into the lock.
- Phase 6.2 QEMU SMP lock contention smoke is accepted. QEMU virt with
  `-smp 4` starts secondary cores 1, 2, and 3 through the accepted PSCI
  trampoline path; each core contends on the shared `SpinLock<T>` for 64
  iterations, and the transcript reports `counter=192 expected=192`,
  `participants=3`, `errors=0`, and
  `qemu-smp-lock-contention-complete`. This remains QEMU/substitute evidence;
  the separate hardware-locked Pi 5 proof below closes the physical
  cache/coherence claim.
- Phase 6.2 Pi 5 SMP lock cache/coherence proof is accepted. Serialized Pi 5
  hardware evidence shows the boot CPU and logical cores 1, 2, and 3 in the
  accepted cacheable-MMU regime before generic lock access; each secondary
  reports stable identity and `ok=true`; the final invariant reports
  `counter=192 expected=192 participants=3 errors=0`,
  `mixed-cache-mmu=false`,
  `classification=pi5-smp-lock-cache-coherence-complete`, and `PASS`.
- The Phase 6.2 SMP-safe primitives closeout checkpoint is accepted. Milestone
  6.2 now has reconciled source inventory, generic `SpinLock<T>` and barrier
  implementation, QEMU SMP lock contention evidence, serialized Pi 5 physical
  cache/coherence proof, and proof-scaffolding cleanup. Scheduler migration,
  shared run queues, cross-core wakeups, IPIs, userspace, descriptors,
  filesystem, networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and
  DMA/cache-coherent driver policy remain deferred.
- Phase 6.3 scheduler migration readiness, per-core scheduler state, and QEMU
  per-core scheduler ownership evidence are accepted. The scheduler now has a
  CPU-local ownership data boundary and QEMU substitute evidence that logical
  CPUs 0 through 3 can publish distinct local scheduler ownership snapshots
  while secondary production dispatch, shared run queues, task migration,
  cross-core wakeups, and IPIs remain deferred.
- Phase 6.3 cross-core wakeup/IPI source inventory is accepted. The selected
  path was raw SGI delivery first: a QEMU-only SGI/IPI smoke for target-list
  mapping, acknowledgement/EOI, and per-core counters before any scheduler
  wakeup implementation, followed by a serialized Pi 5 proof before SGIs are
  accepted for physical scheduler wakeups.
- Phase 6.3 raw SGI delivery is accepted on both QEMU and Pi 5. The QEMU proof
  shows SGI INTID 1 target-list delivery to logical CPUs 1, 2, and 3; the
  serialized Pi 5 proof shows the physical GIC-400 path delivering and EOI'ing
  SGI INTID 1 on logical CPUs 1, 2, and 3. These are raw interrupt-delivery
  proofs, not scheduler wakeup or remote enqueue implementations.
- Phase 6.3 remote wakeup ownership source inventory is accepted. The selected
  first model is a bounded per-target remote wake-request list: a remote sender
  may publish a bounded request and signal with SGI INTID 1, while the target
  CPU owns request consumption and any later local scheduler effect.
- Phase 6.3 QEMU remote wake-request evidence and the cross-core wakeup
  closeout checkpoint are accepted. QEMU proves request publication, duplicate
  coalescing, SGI signaling, target-owned observation/EOI/consumption, and
  cross-owner mutation rejection for logical CPUs 1, 2, and 3. This is
  scheduler-facing substitute evidence, not a Pi 5 scheduler wakeup claim.
- Phase 6.3 Pi 5 remote wake-request evidence is accepted. The serialized
  hardware proof shows CPU 0 publishing bounded requests for logical CPUs 1, 2,
  and 3, duplicate coalescing for target 1, SGI INTID 1 delivery/EOI,
  target-owned request consumption, drained queues, rejected cross-owner local
  scheduler mutation, and deferred secondary production dispatch. This proves
  request publication/signaling/consumption only; local runnable transitions
  from remote requests remain deferred.
- Phase 6.3 target-owned wake consumption contract is accepted. A remote CPU
  may not mutate another CPU's runnable queue; after a target consumes a
  remote request outside IPI context, only that target may transition one of
  its own blocked local tasks to runnable under local scheduler rules. The
  QEMU target-owned wake-consumption proof is also accepted. It proves
  blocked-to-runnable local transitions for diagnostic tasks on logical CPUs
  1, 2, and 3 after request drain, duplicate coalescing, duplicate-local
  enqueue rejection, cross-owner rejection, drained queues, SGI INTID 1
  observation/EOI, and no production secondary dispatch.
- Phase 6.3 Pi 5 remote-wake-to-local-runnable evidence is accepted. The
  serialized hardware proof carries the QEMU target-owned wake-consumption
  invariant to physical Pi 5: after bounded request drain, logical CPUs 1, 2,
  and 3 each transition only their own diagnostic blocked task to runnable,
  reject duplicate local enqueue, preserve SGI INTID 1 observation/EOI,
  preserve duplicate request coalescing and cross-owner rejection, and leave
  production secondary dispatch deferred.
- The Phase 6.3 remote wakeup scheduler-integration closeout checkpoint is
  accepted. It reconciles raw SGI delivery, bounded remote wake-request
  publication/consumption, target-owned local Blocked -> Runnable transitions,
  retained gates, deferrals, and risks. Talos is ready for a
  supervisor-planned production secondary scheduler dispatch source inventory
  and contract, not implementation, shared run queues, task migration,
  multi-core preemption, Phase 7, filesystem, networking, SSH, or shell work.
- Phase 6.3 production secondary scheduler dispatch source inventory and
  contract is accepted. The first implementation may dispatch only explicitly
  seeded CPU-local diagnostic kernel threads on secondary CPUs, from normal
  secondary control flow, with per-core current-task reporting and local
  runnable transitions. Shared run queues, global task lookup, remote enqueue,
  task migration, load balancing, work stealing, multi-core preemption, Phase
  7, filesystem, networking, SSH, and shell work remain deferred.
- Phase 6.3 QEMU production secondary dispatch evidence is accepted. Under
  QEMU virt, logical CPUs 1, 2, and 3 enter the explicit
  `SecondaryProductionDiagnostic` role, dispatch bounded CPU-local diagnostic
  tasks, publish stable local ownership/current-task/counter snapshots, and
  reject cross-owner local scheduler mutation. This is substitute evidence only;
  Pi 5 production secondary dispatch remains the next hardware proof.
- Phase 6.3 Pi 5 production secondary dispatch evidence is accepted. On
  serialized Pi 5 hardware, logical CPUs 1, 2, and 3 enter the explicit
  `SecondaryProductionDiagnostic` role, dispatch bounded CPU-local diagnostic
  tasks, publish stable local ownership/current-task/counter snapshots, and
  reject cross-owner local scheduler and dispatch attempts. Shared scheduler
  metadata, shared run queues, task migration, load balancing, multi-core
  preemption, Phase 7, filesystem, networking, SSH, and shell work remain
  deferred.
- The Phase 6.3 production secondary dispatch closeout checkpoint is accepted.
  It reconciles the source inventory, implementation, QEMU substitute proof,
  and Pi 5 hardware proof for the CPU-local production secondary diagnostic
  dispatch slice. The next bounded worker task should be a shared scheduler
  metadata source inventory and contract, not shared run queue implementation,
  task migration, multi-core preemption, Phase 7, filesystem, networking, SSH,
  or shell work.
- Phase 6.3 shared scheduler metadata source inventory and contract is
  accepted. The next bounded implementation task should add only local-owner
  metadata types and APIs for scheduler task identity, owning CPU, task state,
  optional process owner, stack bounds, current/runnable membership, and stale
  snapshot rejection. It must preserve CPU-local runnable queue ownership and
  does not authorize shared run queues, remote enqueue, migration, load
  balancing, multi-core preemption, Phase 7, filesystem, networking, SSH, or
  shell work.
- Phase 6.3 shared scheduler metadata core is accepted at static/unit-test and
  retained QEMU-smoke evidence levels. The core adds a bounded
  owner-published metadata table, task snapshots, explicit duplicate/unknown/
  invalid-owner/stale-snapshot outcomes, and a named SpinLock boundary for
  future shared use. It still does not authorize shared run queues, remote
  enqueue, migration, load balancing, multi-core preemption, Phase 7,
  filesystem, networking, SSH, shell work, RP1/PCIe, UART interrupt ownership,
  or DMA/cache-coherent driver policy.
- Phase 6.3 QEMU shared scheduler metadata evidence is accepted. Under QEMU
  SMP, logical CPUs 0 through 3 publish/query the owner-only metadata table,
  prove boot-task and owner-task lookup, reject cross-owner scheduler and
  metadata mutation, preserve target-owned local runnable queues, and report
  classification=qemu-shared-scheduler-metadata-complete. Serialized Pi 5
  proof remains required before treating the invariant as physical hardware
  evidence.
- Phase 6.3 Pi 5 shared scheduler metadata evidence is accepted. On serialized
  Pi 5 hardware, logical CPUs 0 through 3 publish/query the owner-only metadata
  table, prove boot-task and owner-task lookup, reject cross-owner scheduler
  and metadata mutation, preserve local runnable queues, and report
  classification=pi5-shared-scheduler-metadata-complete. This is hardware
  evidence for bounded shared metadata only; shared run queues, remote enqueue,
  migration, load balancing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, shell work, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-coherent driver policy remain deferred.
- The Phase 6.3 shared scheduler metadata closeout checkpoint is accepted. It
  reconciles the source inventory, core implementation, QEMU substitute proof,
  and Pi 5 hardware proof for the bounded owner-published metadata table. The
  next bounded task should audit evidence retention and repository bloat before
  broader scheduler productionization; shared run queues, task migration,
  load balancing, multi-core preemption, Phase 7, filesystem, networking, SSH,
  and shell work remain deferred.
- The evidence-retention policy and bloat audit is accepted. Task records and
  compact evidence summaries are the durable source of truth; large raw lab
  captures should move out of Git only through explicit cleanup with preserved
  classifications, digests, and artifact identity.
- The diagnostic-surface retirement audit is accepted. Current QEMU and Pi 5
  Phase 6.3 proof scripts are named retained gates, while older one-off
  diagnostic paths are queued for bounded retirement only after replacement or
  summary coverage is explicit.
- The senior-review maintainability remediation checkpoint is accepted: stale
  Pi 5 probe/proof surfaces were removed, validation hygiene was restored, the
  Pi 5 boot pipeline is split into named phases, and cross-module tests now
  live in owning modules.
- The Phase 6.3 secondary scheduler service-loop closeout checkpoint is
  accepted. It reconciles the source inventory, core implementation, QEMU
  substitute smoke, and serialized Pi 5 hardware proof for one owner-local
  secondary service cycle. The next bounded task should inventory shared
  run-queue and migration requirements before any shared topology
  implementation; load balancing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, and shell work remain deferred.
- The Phase 6.3 shared run-queue and migration source inventory is accepted.
  It names the owner-local runnable queue assumptions, target-owned remote wake
  boundary, owner-published metadata model, SMP lock boundary, proof-routing
  surfaces, and migration blockers. The next bounded task should be the shared
  run-queue/migration contract before any implementation.
- The Phase 6.3 shared run-queue and migration contract is accepted. It keeps
  task mutation single-owner, separates remote wake from remote enqueue,
  defines local-IRQ-then-SMP-lock ordering, names acquire/release publication
  rules, specifies owner-local/reserved/shared-queued/destination-enqueued/
  rejected migration states, and keeps load balancing and multi-core
  preemption deferred.
- The Phase 6.3 shared run-queue core is accepted. It adds the
  target-independent `SharedRunQueue` owner-transfer surface, source-owner
  publication, destination-owner consumption, metadata owner transfer, local
  runnable queue removal, deterministic failure reporting, and unit-tested
  migration states. QEMU and Pi 5 proof tasks remain separate; the core does
  not add target selection, load balancing, work stealing, multi-core
  preemption, userspace, filesystem, networking, SSH, or shell behavior.
- The Phase 6.3 QEMU shared run-queue/migration smoke is accepted. It adds the
  `qemu_shared_runqueue_migration` diagnostic and proves task 107 moving from
  source owner 0 to destination owner 1 through the implemented
  `SharedRunQueue` publish/consume APIs, with source queue removal,
  destination-local enqueue, shared queue drain, metadata owner transfer, and
  classification=qemu-shared-runqueue-migration-complete.
- The Phase 6.3 Pi 5 shared run-queue/migration proof is accepted. It adds the
  `rpi5_shared_runqueue_migration` diagnostic and focused Pi 5 image/boot-tree
  scripts, records archive/kernel digests, TFTP identity, cursor-valid serial,
  classification, PASS output, and restore evidence, and reports
  participants=4 expected=4 with
  classification=pi5-shared-runqueue-migration-complete.
- The Phase 6.3 load-balancing source inventory is accepted. It records the
  scheduler, metadata, run-queue, wake, timer, SMP, and diagnostic surfaces
  available before policy design; names policy inputs and stale/invalid input
  failure modes; and separates target selection, fairness/affinity, remote
  reschedule notification, and the accepted shared run-queue migration
  mechanism before any implementation.
- The Phase 6.3 load-balancing policy contract is accepted. It defines a
  conservative deterministic policy boundary over accepted inputs, preserves
  SharedRunQueue as the only owner-transfer mechanism, keeps remote reschedule
  polling-only for the first implementation, and defers fairness, affinity,
  work stealing, running-task migration, and multi-core preemption.
- The Phase 6.3 load-balancing core is accepted. It adds
  `LoadBalancingPolicy`, `LoadBalancingPlan`, and deterministic policy
  errors for front-runnable source selection, destination role/capacity
  checks, shared queue backpressure, stale generation rejection through
  `SharedRunQueue::publish_migration`, and single-owner queue membership. It
  does not add QEMU or Pi 5 proof claims, work stealing, running-task
  migration, remote scheduler execution in IPI context, multi-core preemption,
  Phase 7, filesystem, networking, SSH, or shell behavior.
- The Phase 6.3 QEMU load-balancing smoke is accepted. It adds the
  `qemu_load_balancing_smoke` boot scenario and
  `scripts/qemu-load-balancing-smoke.sh`, proves the accepted
  `LoadBalancingPolicy` path over `SharedRunQueue`, and reports
  classification=qemu-load-balancing-smoke-complete.
- The Phase 6.3 Pi 5 load-balancing proof is accepted. It adds the
  `rpi5_load_balancing_proof` boot scenario,
  `scripts/rpi5-load-balancing-image.sh`, and
  `scripts/rpi5-load-balancing-boot-tree.sh`, proves the same deterministic
  `LoadBalancingPolicy` path on serialized Pi 5 hardware, and reports
  classification=pi5-load-balancing-complete.
- The Phase 6.3 load-balancing closeout checkpoint is accepted. It reconciles
  the source inventory, policy contract, target-independent core, QEMU
  substitute proof, serialized Pi 5 proof, retained gates, and remaining
  deferrals. The next bounded recommendation is
  `phase6-multicore-preemption-source-inventory-20260527`, a documentation
  and source-inventory task before any multi-core preemption implementation.
- The Phase 6.3 multi-core preemption source inventory is accepted. It maps
  the accepted timer IRQ, scheduler, CPU-local service, secondary
  service-loop, IPI/wake, metadata, SharedRunQueue, and load-balancing
  boundaries; names CPU-local versus cross-core assumptions; and recommends
  `phase6-multicore-preemption-contract-20260527` before implementation.
- The Phase 6.3 multi-core preemption contract, target-independent core, QEMU
  substitute proof, and serialized Pi 5 hardware proof are accepted. The
  retained QEMU proof reports
  classification=qemu-multicore-preemption-smoke-complete after logical CPUs 1,
  2, and 3 record local pending timer-preemption state without scheduler
  mutation and then service it through owner-local normal scheduler control
  flow. The retained Pi 5 proof reports
  classification=pi5-multicore-preemption-complete with participants=3 and
  PASS for the same invariant.
- The Phase 6.3 multi-core preemption closeout checkpoint is accepted. It
  reconciles the accepted source inventory, contract, target-independent core,
  QEMU substitute proof, serialized Pi 5 proof, retained gates, risks, and
  remaining deferrals before any later scheduler productionization or phase
  transition.

Blocked or pending:

- No next scheduler or phase-transition task is accepted yet. The supervisor
  should plan the next explicit bounded task before any further scheduler
  productionization or Phase 7 work proceeds. Work stealing, running-task
  migration, remote reschedule, userspace, descriptors, filesystem,
  networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and
  DMA/cache-coherent driver policy remain deferred.
- Large raw accepted evidence remains in Git until external artifact storage or
  an explicit no-delete manifest-only cleanup is approved. Do not delete
  tracked accepted evidence during unrelated feature work.
- The roadmap order below now prioritizes a local Unix-like OS before network
  shell access. Ethernet and SSH should reuse the local process, stdio, TTY,
  filesystem, and syscall mechanisms rather than define them.

## Roadmap Principles

- Use Rust for kernel code, with small AArch64 assembly stubs where the hardware requires it.
- Use established Rust kernel development practices where they fit: pinned nightly toolchain, explicit custom targets, no_std, build-std, small unsafe boundaries, narrow target abstractions, and QEMU-backed smoke tests for generic architecture work.
- Keep POSIX direction visible from the start: processes, file descriptors, pipes, paths, sockets, exit/wait, and exec-style program loading should shape interfaces even before compatibility is complete.
- Keep kernel, libraries, and programs as separate product areas. Kernel code
  owns scheduling, memory, drivers, syscalls, VFS, and process isolation;
  libraries own userspace ABI wrappers and reusable runtime support; programs
  own shell, utilities, and interaction-based tests. Use OSDev Wiki examples as
  reference material when shaping these boundaries, but fit the result to the
  Talos Rust/no_std and Pi 5 constraints.
- Prefer local OS capability before remote access: serial/local TTY, stdio,
  user processes, ramfs/initramfs, VFS, libc, and a local shell come before
  Ethernet and SSH on the critical path.
- Reuse proven libraries where they shorten the path without hiding kernel
  responsibilities. smoltcp is preferred for TCP/IP evaluation over
  hand-rolling TCP; Rust uutils is preferred for core utilities once the Rust
  userspace target is viable.
- Treat self-hosting as a long-term north star, not a committed roadmap phase.
  Native compilers such as GCC, LLVM, or rustc require a mature userspace,
  filesystem, process model, libc/Rust std target, linker, storage, memory
  reclamation, and developer tooling.
- Treat Pi 5 hardware facts as evidence, not assumptions. Device tree, Linux drivers, Raspberry Pi firmware docs, Circle/RPi bare-metal examples, serial logs, and lab results should be cited in task notes.
- Keep board-specific code behind clear target boundaries. The QEMU virt target and Pi 5 target should share architecture code where possible, but not pretend to have the same devices.
- Prefer small, inspectable milestones with a boot/test gate over broad subsystem rewrites.
- Every milestone must update docs, ADRs, or task records when it changes architecture or hardware understanding.

## Phase 0: Planning, Sources, and Lab Loop

Goal: make the development system trustworthy before kernel implementation accelerates.

Milestone 0.1: Source Map

- Build a curated source index for Pi 5, BCM2712, RP1, ARMv8-A, QEMU, Linux, Raspberry Pi firmware, Circle/RPi bare-metal, and Daedalus references.
- Record which sources are authoritative and which are advisory.
- Identify missing datasheets or areas that require Linux-source archaeology.

Acceptance criteria:

- project/reference-notes.md lists primary source URLs and known gaps.
- Open hardware questions are tracked as future research tasks.

Milestone 0.2: Lab Controller Readiness

- Verify health, status, boot files, boot archive upload, power cycle, rollback, and serial endpoints.
- Keep network-controller credentials outside OpenClaw; use only the lab API.
- Establish a boot-attempt record format with archive digest, power-cycle time, serial tail, and result classification.

Acceptance criteria:

- A known-good Raspberry Pi OS boot archive can be published, power-cycled, and observed.
- Serial output is available through the API after the physical cable is installed.
- Failed boots can be rolled back without manual SD-card intervention.

Milestone 0.3: Initial ADRs

- Decide target split: QEMU virt plus physical Pi 5.
- Decide Rust toolchain and repository layout.
- Decide boot image contract and lab automation contract.
- Add an early POSIX/process shape note before scheduler task structures harden.

Acceptance criteria:

- ADRs exist for the target strategy, boot/lab loop, and Rust toolchain.
- The early POSIX/process shape note exists and is referenced before implementing scheduler task structs.

## Phase 1: Rust Kernel Skeleton and Fast Test Target

Goal: create a minimal Rust kernel that builds reproducibly and runs under QEMU virt.

Milestone 1.1: Toolchain and Image Build

- Add a custom AArch64 target JSON, Cargo config, linker script, build script, and image conversion step.
- Reuse Daedalus patterns where they still apply: build-std, alloc, panic-strategy abort, redzone disabled, explicit linker memory layout, and assembly build integration.
- Produce artifacts for both talos-aarch64-virt and talos-rpi5-bcm2712 even if the Pi 5 artifact is initially a stub.
- Pin the Rust nightly with rust-toolchain.toml and document the exact build and test commands.
- Decide target-feature policy, relocation model, inline assembly policy, compiler_builtins memory intrinsic handling, and no-unwind guarantees.

Acceptance criteria:

- cargo build produces a kernel artifact.
- The artifact layout documents load address, stack, BSS, heap reservation, and exception-vector alignment.
- Formatting and basic lint gates exist.
- Toolchain drift is detectable through CI or an explicit local check.
- Linker map or equivalent layout output can be inspected when early boot fails.

Milestone 1.2: QEMU Boot Smoke Test

- Boot on QEMU virt with a simple serial console message.
- Add a custom bare-metal test harness modeled after Daedalus, including success/failure exit through QEMU.
- Keep hardware-only behavior out of unit tests; expose it as diagnostics once real hardware exists.

Acceptance criteria:

- cargo test or an equivalent runner boots QEMU and exits with pass/fail status.
- Panic output reaches the QEMU serial console.
- Pure Rust modules can define no_std test cases.

Milestone 1.3: Early Architecture Boundaries

- Define target abstractions for boot info, UART, timer, interrupt controller, MMIO map, and device tree access.
- Keep the interfaces narrow enough to avoid overengineering before hardware facts are known.

Acceptance criteria:

- QEMU virt implements enough target operations for boot and test output. [done: QEMU test gate]
- Pi 5 target has explicit stubs or early implementations with documented unknowns. [done: build gate, pending hardware evidence]

## Phase 2: Raspberry Pi 5 First Light

Goal: boot Talos on physical Pi 5 and get reliable serial output.

Milestone 2.1: Firmware Handoff and Firmware-Preserved Serial

- Build a Pi 5 boot tree that satisfies the lab controller archive contract: config.txt, cmdline.txt, bcm2712-rpi-5-b.dtb, and kernel_2712.img or kernel8.img.
- Prefer kernel_2712.img for the Pi 5 artifact; keep kernel8.img fallback behavior documented only as firmware compatibility.
- Configure AArch64 entry, stack, BSS clearing, panic path, and serial output.
- Implement the arm64 boot ABI: x0 contains the physical DTB address, interrupts are masked, the MMU is off, and non-secure EL2 is preferred while EL1 is allowed.
- Start by using serial state preserved by firmware. Do not assume Talos owns UART clocks, GPIO muxing, or RP1 reset behavior yet.
- Check config.txt serial settings, baud rate, DTB aliases, chosen stdout-path, and whether enable_rp1_uart=1 is required for the attached cable path.

Acceptance criteria:

- The lab can publish the Talos boot archive. [local staging tool exists; publish not yet run]
- Power cycle reaches Talos code on the Pi 5.
- Serial output includes a version string, exception level, core ID, and panic path.
- A failed boot can be rolled back.

Milestone 2.2: Boot-State and UART Ownership Discovery

- Preserve and inspect the firmware-provided device tree from x0 enough to confirm memory and chosen boot arguments.
- Record actual firmware handoff state: exception level, MMU/cache state, DTB address if provided, core startup behavior, and UART clock assumptions.
- Compare observations against Linux device tree and Raspberry Pi documentation.
- Split firmware-preserved serial from Talos-owned UART initialization. The firmware console serial10 maps to BCM2712 uarta; the 40-pin header UART is RP1 UART0 and can be firmware-initialized with enable_rp1_uart=1.
- Verify serial still works after cache and MMU transitions.

Acceptance criteria:

- Architecture docs describe the actual Pi 5 handoff observed in the lab.
- UART path and ownership assumptions are documented before any UART driver is treated as stable.
- Any mismatch with assumptions becomes an ADR or tracked task.

Milestone 2.3: Exception Vectors and Panic Diagnostics

- Install AArch64 exception vectors.
- Dump ESR, FAR, ELR, SPSR, and general registers on synchronous exceptions.
- Add a deliberate exception diagnostic.

Acceptance criteria:

- A deliberate fault produces a readable serial dump.
- The dump includes enough state to debug early MMU and driver faults.

## Phase 3: Memory, MMU, and Kernel Runtime

Goal: build the foundations for safe Rust allocation, virtual memory, and later userspace.

Status: accepted for the current closeout boundary. See
[Phase 3 Closeout Checkpoint](project/phase3-closeout-checkpoint.md) for the
accepted capabilities, commit references, deferred work, and Phase 4
recommendation.

Milestone 3.1: Physical Memory Map

- Determine usable DRAM and reserved regions from device tree and firmware observations.
- Define kernel image, stack, heap, boot info, and early allocator regions.
- Avoid hardcoding a single RAM size.

Acceptance criteria:

- Boot log reports memory regions.
- Early allocator avoids kernel image, stack, DTB, and reserved firmware regions.

Milestone 3.2: Page Tables and MMU

- Implement early identity mappings for kernel memory and required MMIO.
- Map normal memory cacheable and MMIO as device memory.
- Keep translation setup compatible with SMP and future EL0 isolation.

Acceptance criteria:

- Pi 5 boots with MMU enabled.
- Serial still works after MMU enable.
- A page-fault diagnostic is available.

Milestone 3.3: Kernel Heap and Core Runtime

- Add a simple allocator first, then evolve toward a free-capable allocator when needed.
- Enable Rust alloc for Box, Vec, String, and collections.
- Keep allocation failure behavior explicit.

Acceptance criteria:

- Allocation tests pass under QEMU.
- Pi 5 diagnostic confirms heap allocation and panic-on-OOM behavior.

## Phase 4: Interrupts, Timers, and Preemption

Goal: move from cooperative boot code to timer-driven kernel scheduling.

Status: accepted for the current closeout boundary. See
[Phase 4 Closeout Checkpoint](project/phase4-closeout-checkpoint.md) for the
accepted capabilities, commit references, deferred work, risks, and Phase 5
planning recommendation.

Milestone 4.1: Interrupt Controller

- Identify the Pi 5 interrupt controller topology from device tree and Linux references. Current evidence points to GIC-400 / GICv2, with distributor and CPU interfaces in the 0x10_7fff9000 region.
- Bring up enough GIC support for architectural timer and UART interrupts.
- Keep QEMU virt and Pi 5 interrupt-controller setup target-specific.

Acceptance criteria:

- Timer interrupt fires on QEMU virt.
- Timer interrupt fires on Pi 5.
- IRQ entry/exit preserves register state.

Milestone 4.2: Monotonic Time and Preemption Tick

- Implement monotonic time based on the ARM generic timer first. The BCM2835-compatible 1 MHz system timer at 0x10_7c003000 is a secondary board timer path, not the first scheduler clock.
- Add scheduler tick accounting.
- Make interrupt masking and critical sections explicit.

Acceptance criteria:

- Serial diagnostics show periodic ticks without polling.
- Tick handling remains stable under simple workload loops.

Milestone 4.3: Kernel Threads and Scheduler

- Define the scheduler shape against the early POSIX note before committing
  structs. [done: single-core kernel-thread-first boundary]
- Implement kernel task structures and a single-core runnable queue before
  context switch, sleeping, yielding, or preemptive time slicing.
- Start with one core; keep data structures ready for SMP.
- Check task/process terminology and lifetime assumptions against the early POSIX shape note before committing scheduler structs. [done: scheduler shape note]

Acceptance criteria:

- Multiple kernel threads make progress under preemption.
- A diagnostic shows task state and context-switch counts.

## Phase 5: Local Console, TTY, and Kernel Diagnostics

Goal: make Talos locally operable over serial before adding network access.

Milestone 5.1: Console Device Model

- Split early boot logging from a runtime console device.
- Preserve the proven firmware-preserved UART path while defining the ownership
  boundary for later Talos-owned serial drivers.
- Route console reads and writes through interfaces that can become file
  descriptors and TTY devices.

Acceptance criteria:

- Kernel diagnostics can write through a runtime console abstraction.
- The early boot logger and runtime console ownership rules are documented.
- Console paths do not depend on ad hoc shell-only code.

Milestone 5.2: TTY and Stdio Shape

- Define the first TTY line discipline: raw/canonical input policy, newline
  handling, backspace, echo, and control-character behavior.
- Model stdin, stdout, and stderr as descriptor-capable streams even before
  full userspace exists.
- Keep the design compatible with later PTY/SSH sessions.

Acceptance criteria:

- A local serial TTY diagnostic can accept input and echo/process lines.
- Stdio streams can be represented by the same descriptor model planned for
  user processes.
- TTY behavior and known POSIX gaps are documented.

Milestone 5.3: Local Kernel Diagnostic Command Channel

- Add a constrained local diagnostic command channel over the serial TTY.
- Keep commands clearly kernel-owned until EL0 programs and a real shell exist.
- Prefer diagnostics that exercise scheduler, memory, filesystem, and process
  state without becoming permanent shell architecture.

Acceptance criteria:

- A user at the serial console can run bounded kernel diagnostic commands.
- Diagnostic commands are separated from the later user shell design.
- The command channel remains usable while scheduler/timer work is active.

## Phase 6: SMP and Multi-Core Scheduling

Goal: use all Pi 5 CPU cores with correct synchronization and preemptive scheduling.

Status: Milestone 6.1 is accepted through the secondary-core bring-up closeout
checkpoint. Milestone 6.2 has an accepted SMP-safe primitive source inventory,
contract, first spinlock/barrier core, QEMU SMP contention smoke, and physical
Pi 5 lock cache/coherence proof. Milestone 6.3 has accepted the first
scheduler-migration slice, raw QEMU/Pi 5 SGI delivery, remote wake-request
publication/consumption evidence, the target-owned wake-consumption contract,
QEMU and Pi 5 blocked-to-runnable target-owned wake proofs, and the remote
wakeup scheduler-integration closeout, plus the production secondary dispatch
closeout checkpoint and shared scheduler metadata closeout. See
[Phase 6 Secondary-Core Bring-Up Closeout Checkpoint](project/phase6-secondary-core-bringup-closeout-checkpoint.md)
and
[Phase 6 Secondary-Core Bring-Up Source Inventory](project/phase6-secondary-core-bringup-source-inventory.md),
plus
[Phase 6 SMP-Safe Primitives Source Inventory](project/phase6-smp-safe-primitives-source-inventory.md)
and
[Phase 6 Scheduler Migration Slice Checkpoint](project/phase6-scheduler-migration-slice-checkpoint.md),
and
[Phase 6 Remote Wakeup Scheduler Integration Closeout](project/phase6-remote-wakeup-scheduler-integration-closeout.md),
and
[Phase 6 Production Secondary Dispatch Closeout](project/phase6-production-secondary-dispatch-closeout-checkpoint.md),
and
[Phase 6 Shared Scheduler Metadata Source Inventory](project/phase6-shared-scheduler-metadata-source-inventory.md),
and
[Phase 6 Shared Scheduler Metadata Closeout](project/phase6-shared-scheduler-metadata-closeout-checkpoint.md),
and
[Phase 6 CPU-Local Scheduler Service Boundary Source Inventory](project/phase6-cpu-local-scheduler-service-boundary-source-inventory.md),
and
[Phase 6 CPU-Local Scheduler Service Closeout](project/phase6-cpu-local-scheduler-service-closeout-checkpoint.md),
and
[Phase 6 Secondary Scheduler Service Loop Source Inventory](project/phase6-secondary-scheduler-service-loop-source-inventory.md),
and
[Phase 6 Secondary Scheduler Service Loop Closeout](project/phase6-secondary-scheduler-service-loop-closeout-checkpoint.md),
and
[Phase 6 Shared Run-Queue and Migration Source Inventory](project/phase6-shared-runqueue-migration-source-inventory.md),
and
[Phase 6 Shared Run-Queue and Migration Contract](project/phase6-shared-runqueue-migration-contract.md),
and
[Phase 6 Shared Run-Queue Migration Closeout Checkpoint](project/phase6-shared-runqueue-migration-closeout-checkpoint.md).
The target-independent CPU-local scheduler service core is accepted in
`tasks/2026-05-26-phase6-cpu-local-scheduler-service-core.md`.
The target-independent secondary scheduler service-loop core is accepted in
`tasks/2026-05-26-phase6-secondary-scheduler-service-loop-core.md`, with
QEMU substitute and serialized Pi 5 proof records retained as diagnostic
gates.
The target-independent shared run-queue core is accepted in
`tasks/2026-05-26-phase6-shared-runqueue-core.md`, with QEMU substitute proof
accepted in
`tasks/2026-05-26-phase6-qemu-shared-runqueue-migration-smoke.md` and
serialized Pi 5 proof accepted in
`tasks/2026-05-26-phase6-pi5-shared-runqueue-migration-proof.md`.
The shared run-queue/migration closeout reconciles source inventory, contract,
core implementation, QEMU substitute proof, Pi 5 hardware proof, retained
diagnostics, and deferred work. The load-balancing source inventory and policy
contract are accepted, and the target-independent load-balancing core is
accepted in `tasks/2026-05-27-phase6-load-balancing-core.md`. The QEMU
substitute proof is accepted in
`tasks/2026-05-27-phase6-qemu-load-balancing-smoke.md`, and the serialized
Pi 5 proof is accepted in
`tasks/2026-05-27-phase6-pi5-load-balancing-proof.md`. The load-balancing
closeout is accepted in
`docs/src/project/phase6-load-balancing-closeout-checkpoint.md`; its next
bounded Phase 6.3 recommendation is multi-core preemption source inventory.
The multi-core preemption source inventory is accepted in
`docs/src/project/phase6-multicore-preemption-source-inventory.md`; its
contract, target-independent core, QEMU substitute proof, and serialized Pi 5
proof are accepted in the corresponding Phase 6.3 task records. The bounded
multi-core preemption closeout checkpoint is accepted in
docs/src/project/phase6-multicore-preemption-closeout-checkpoint.md. The
production scheduler runtime source inventory is accepted in
`docs/src/project/phase6-production-scheduler-runtime-source-inventory.md`,
and the production timer/preemption contract is accepted in
`docs/src/project/phase6-production-timer-preemption-contract.md`. The first
production timer/preemption core is accepted in
`tasks/2026-05-28-phase6-production-timer-preemption-core.md`. The focused
QEMU production timer/preemption smoke is accepted in
`tasks/2026-05-28-phase6-qemu-production-timer-preemption-smoke.md`, and the
serialized Pi 5 production timer/preemption proof is accepted in
`tasks/2026-05-28-phase6-pi5-production-timer-preemption-proof.md`. The
production scheduler runtime closeout is accepted in
`docs/src/project/phase6-production-scheduler-runtime-closeout-checkpoint.md`.
Phase 7 or later subsystem work remains blocked until the supervisor creates
the next explicit bounded task.
Before broader Phase 6.3 productionization, the accepted
[Evidence Retention Policy](project/evidence-retention-policy.md) and
[Diagnostic Surface Policy](project/diagnostic-surface-policy.md) govern which
raw artifacts and proof-only scripts remain retained gates versus cleanup
candidates.

Milestone 6.1: Secondary Core Bring-Up

- Observe firmware core startup behavior.
- Use PSCI as the primary secondary-core bring-up path; Raspberry Pi Linux device tree advertises PSCI 1.0 with SMC and cpu_on 0xc4000003.
- Treat spin-table or custom mailbox bring-up as fallback research, not the default plan.
- Add per-core stacks, per-core state, and CPU-local data.

Acceptance criteria:

- All four Cortex-A76 cores report alive through serial diagnostics.
- Secondary cores can run a controlled kernel-thread workload.

Milestone 6.2: SMP-Safe Primitives

- Implement spin locks, interrupt-safe locks, atomics policy, memory barriers, and per-core critical-section rules.
- Review any inherited Daedalus synchronization assumptions before reuse.

Acceptance criteria:

- Stress diagnostics run shared counters and queues across cores.
- Lock misuse and interrupt-context constraints are documented.

Milestone 6.3: Multi-Core Preemptive Scheduler

- Support per-core run queues or a global scheduler with clear tradeoffs.
- Add load balancing only after correctness is established.
- Keep task migration visible in diagnostics.

Acceptance criteria:

- Multiple CPU-bound tasks run across all cores.
- Preemption continues to work under cross-core wakeups.

## Phase 7: POSIX Contract, EL0, Syscalls, and File Descriptors

Goal: introduce Unix-like execution boundaries without attempting full POSIX yet.

Milestone 7.1: POSIX Contract Baseline

- Define the first Talos error model and errno mapping.
- Define path normalization, absolute and relative paths, root, current working directory, and namespace assumptions.
- Define initial descriptor operations: open, read, write, close, dup, pipe, and descriptor inheritance.
- Define process lifetime concepts: spawn or exec, exit status, wait, parent/child relationship, and signal deferrals.
- Define the early loader ABI and argument/environment story.

Acceptance criteria:

- A POSIX-baseline design note exists before VFS or process code grows around convenient shortcuts.
- Host-side tests cover path normalization and descriptor-table edge cases.

Accepted progress:

- Phase 7 POSIX contract source inventory is accepted. It maps the accepted
  scheduler task/process separation, runtime-console and TTY stdio direction,
  diagnostic command-channel limits, lower-EL readiness limits, and retained
  Phase 4 through Phase 6 gates that constrain the first POSIX baseline
  contract.
- Phase 7 POSIX contract baseline is accepted. It defines the first
  errno-style names, path normalization semantics, process lifetime vocabulary,
  descriptor operation vocabulary, stdio inheritance shape, and early
  loader/argument/environment vocabulary.
- Phase 7 path/error model core is accepted. It adds the target-independent
  no_std path normalizer and PosixError vocabulary with unit tests.
- Phase 7 descriptor-table contract is accepted. It keeps descriptor entries
  process-local, separates entries from shared object handles, defines
  close/dup/inherited-stdio semantics, and names deterministic descriptor
  table errors for the next core implementation. Runtime console/TTY
  descriptor I/O integration, EL0, SVC/syscall ABI, VFS, filesystem, program
  loading, networking, SSH, and shell work remain blocked.
- Phase 7 descriptor-table core is accepted. It adds the first fixed-capacity
  process-local descriptor table model, inherited stdio entries, allocation,
  lookup, close, dup, access checks, reserved object kind tags, and
  deterministic PosixError results with target-independent no_std unit tests.
- Phase 7.1 POSIX baseline closeout is accepted. It reconciles the accepted
  contract, path/error, and descriptor-table evidence, preserves the retained
  gates, and recommends a supervisor-planned Phase 7.2 source inventory before
  any EL0 trap path, user address-space, syscall, VFS, filesystem, program
  loader, descriptor I/O, networking, SSH, or shell implementation.

Milestone 7.2: EL0 Trap Path and User Address Spaces

- Split kernel and user mappings.
- Add user stacks, trap return, copy-in/copy-out helpers, and fault handling.
- Validate exception return and bad user pointers before stabilizing the syscall ABI.

Acceptance criteria:

- A simple user-mode payload runs and traps back to the kernel.
- Invalid user memory access traps without corrupting the kernel.
- Negative tests cover bad pointers and invalid trap state.

Accepted progress:

- Phase 7 EL0 address-space source inventory is accepted. It names the accepted
  exception-vector and saved-frame surfaces, same-EL ERET diagnostic boundary,
  EL2 translation setup, early page-frame ownership, scheduler task/process
  separation, POSIX error vocabulary, descriptor-table ownership, retained
  gates, diagnostic-only surfaces, and implementation gaps that constrain the
  first EL0 trap-return and user address-space contract.
- Phase 7 EL0 trap and address-space contract is accepted. It defines the
  first canonical user range and null guard, user text/data/heap/stack/guard
  vocabulary, kernel-only mapping policy while a user task runs, validated
  user trap-return frame requirements, user fault classes, copy-in/copy-out
  preconditions, evidence levels, and blocked surfaces. The next implementation
  task remains target-independent user range and permission validation only.

Milestone 7.3: Syscall ABI

- Add an SVC-based syscall path from lower exception level.
- Define stable error handling and numeric syscall IDs.

Acceptance criteria:

- A minimal syscall test exercises return values, invalid calls, and fault handling.

Accepted progress:

- Phase 7 syscall ABI source inventory is accepted. It maps lower-EL
  synchronous exception entry, diagnostic SVC proof surfaces, missing syscall
  number and argument-register contracts, PosixError return/error constraints,
  user-copy preconditions, descriptor-table interaction, and process/task
  ownership. It keeps marker 0x7a10 diagnostic-only and recommends
  phase7-syscall-abi-contract-20260529 before any syscall implementation,
  QEMU rerun, Pi 5 hardware run, descriptor I/O, process loading, VFS,
  filesystem, shell, networking, or SSH work.
- Phase 7 syscall ABI contract is accepted. It defines lower-AArch64 svc #0 as
  the first stable syscall trap, keeps diagnostic SVC marker 0x7a10 out of the
  ABI, assigns x8 as the syscall-number register, x0 through x5 as scalar
  argument registers, x0 as the sole return register, negative x0 as -errno,
  talos_nop = 0, unknown syscall = -ENOSYS, and a first target-independent
  dispatch proof slice. Production exception-handler integration, QEMU syscall
  smoke, Pi 5 hardware proof, descriptor I/O, process loading, VFS, filesystem,
  shell, networking, and SSH remain blocked.
- Phase 7 syscall dispatch core is accepted. It adds a target-independent
  syscall module with stable svc #0 vocabulary, diagnostic marker quarantine,
  talos_nop dispatch, unknown-syscall -ENOSYS handling, scalar x0-through-x5
  argument preservation in the pure dispatch layer, and errno encoding for the
  accepted subset. Production exception-handler integration, QEMU syscall
  smoke, Pi 5 hardware proof, pointer-copy syscalls, descriptor I/O, process
  loading, VFS, filesystem, shell, networking, and SSH remain blocked.
- Phase 7 syscall trap-routing source inventory is accepted. It maps exact
  source owners and gaps for lower-AArch64 SVC detection, svc immediate
  validation, x8 syscall-number extraction, x0-through-x5 argument capture,
  x0 return mutation, ELR/SPSR handling, diagnostic marker 0x7a10 quarantine,
  and non-syscall fallback. It recommends
  phase7-syscall-trap-routing-contract-20260529 before production exception
  routing, QEMU syscall smoke, Pi 5 hardware proof, descriptor I/O,
  copy-in/copy-out, process loading, VFS, filesystem, shell, networking, or
  SSH work.
- Phase 7 QEMU syscall smoke core is accepted. It adds qemu_syscall_smoke,
  routes only lower-AArch64 svc #0 through the target-independent dispatch core,
  returns talos_nop x0 = 0 and unknown syscall x0 = -ENOSYS to the user payload,
  quarantines diagnostic marker 0x7a10 outside production dispatch, and retains
  QEMU/substitute PASS evidence. Pi 5 production syscall proof, descriptor I/O,
  copy-in/copy-out, process loading, VFS, filesystem, shell, networking, and SSH
  remain blocked.
- Phase 7 syscall routing closeout checkpoint is accepted. It reconciles the
  syscall ABI contract, dispatch core, trap-routing contract, QEMU smoke
  implementation, retained QEMU evidence, diagnostic-marker quarantine, and
  deferred surfaces before any physical syscall proof or pointer/descriptor
  syscall work.
- Phase 7 Pi 5 syscall proof plan is accepted. It defines the later serialized
  rpi5_syscall_proof invariant, exact physical PASS/classification and
  syscall-observation lines, hardwareTestLock acquisition/release rules,
  candidate identity, fresh serial/TFTP evidence, inconclusive-run triage,
  restoration requirements, and diagnostic marker 0x7a10 quarantine. No Pi 5
  run, archive publication, descriptor I/O, copy-in/copy-out, process loading,
  filesystem, shell, networking, or SSH behavior is accepted by the plan.
- Phase 7 Pi 5 syscall proof is accepted. It adds rpi5_syscall_proof and the
  focused Pi 5 lower-AArch64 svc #0 recovery path, then retains serialized
  physical evidence that talos_nop returns x0 = 0, unknown syscall number 17
  returns x0 = -ENOSYS, diagnostic marker 0x7a10 is not dispatched as a stable
  syscall, and the final line reports
  classification=pi5-syscall-proof-complete with rpi5-syscall-proof: PASS.
  The proof includes candidate identity, fresh TFTP serves of
  da591740/kernel_2712.img at 101408 bytes, fresh serial evidence, a passing
  production-timer known-good control after the first inconclusive run, an
  unchanged candidate rerun, and post-restore tree-hash proof. Descriptor I/O,
  copy-in/copy-out, process loading, VFS/filesystem, shell, networking, and SSH
  remain blocked.
- Phase 7 Pi 5 syscall proof closeout checkpoint is accepted. It reconciles
  the ABI, dispatch, production trap routing, QEMU syscall smoke evidence, Pi 5
  physical proof evidence, hardware-lock timeline, restore proof, and blocked
  surfaces. It recommends phase7-copyin-copyout-helper-contract-20260529 as the
  next bounded documentation-only task before pointer-taking syscall or
  descriptor I/O implementation.
- Phase 7 copy-in/copy-out helper contract is accepted. It defines the first
  target-independent helper boundary: whole-range validation before byte
  movement, copy-in read access, copy-out write access, deterministic EFAULT
  mapping for null/kernel-range/unmapped/permission/wraparound failures,
  all-or-nothing copy behavior, and a split between recoverable syscall helper
  failures and future process-fatal lower-EL abort classifications. It names
  phase7-copyin-copyout-helper-core-20260529 as the next bounded implementation
  task, pending supervisor planning. Pointer-taking syscalls, descriptor I/O,
  process loading, VFS/filesystem, shell, networking, and SSH remain blocked.
- Phase 7 pointer-taking syscall source inventory is accepted. It maps source
  owners and gaps for lower-AArch64 frame argument extraction, x8 syscall
  number ownership, user-memory mapping provenance, copy_from_user/copy_to_user
  invocation, x0 return/error encoding, QEMU smoke ownership, and proof-only
  diagnostic-surface quarantine. It recommends supervisor planning for
  phase7-pointer-taking-syscall-contract-20260529 before any implementation or
  QEMU pointer-copy smoke plan. Descriptor I/O, process loading,
  VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy hardware proof
  remain blocked.
- Phase 7 pointer-taking syscall contract is accepted. It defines the first
  pointer-taking syscall as proof-only talos_copy_probe, routed through stable
  svc #0 with x8 = 0x7001 only in the later QEMU/substitute smoke scenario.
  x0 is the user pointer, x1 is a 0-through-32 byte length, x2 is the expected
  byte, x3 is the replacement byte, and x4/x5 are reserved zeros. Success
  copies in, validates the byte pattern, copies out, and returns the copied
  length; user-boundary failures return -EFAULT, malformed proof setup returns
  -EINVAL, and x8 = 0x7001 outside the proof scenario remains -ENOSYS. The
  contract names the fixed QEMU substitute UserData mapping and keeps
  descriptor I/O, process loading, VFS/filesystem, shell, networking, SSH, and
  Pi 5 pointer-copy hardware proof blocked.
- Phase 7 QEMU pointer-copy smoke plan is accepted. It defines the later
  qemu_pointer_copy_smoke QEMU/substitute invariant for proof-only
  talos_copy_probe: UserData at 0x0000_0000_0011_0000, a 16-byte success case
  that copies 0x2a bytes in and writes 0xa5 bytes back, a guard-range EFAULT
  case returning -EFAULT, an unknown-syscall regression returning -ENOSYS, and
  diagnostic marker 0x7a10 quarantine before classification/PASS. The plan
  names phase7-qemu-pointer-copy-smoke-core-20260529 as the next bounded
  implementation task and keeps descriptor I/O, process loading,
  VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy hardware proof
  blocked.
- Phase 7 QEMU pointer-copy smoke core is accepted. It adds
  qemu_pointer_copy_smoke, a proof-only talos_copy_probe route for x8 = 0x7001
  scoped to that scenario, explicit substitute UserData backing storage,
  copy_from_user/copy_to_user helper invocation, success and guard EFAULT
  observations, an unknown-syscall -ENOSYS regression, diagnostic marker
  quarantine, and retained QEMU/substitute PASS evidence. Descriptor I/O,
  process loading, VFS/filesystem, shell, networking, SSH, and Pi 5
  pointer-copy hardware proof remain blocked.
- Phase 7 pointer-copy closeout checkpoint is accepted. It reconciles the
  pointer-taking syscall contract, QEMU pointer-copy smoke plan, core
  implementation, retained QEMU/substitute evidence, scalar syscall and EL0
  diagnostic regressions, proof-only status, and blocked surfaces. It
  recommends supervisor planning for a documentation-only Pi 5 pointer-copy
  proof plan before any hardware action, and keeps descriptor I/O, process
  loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, DMA/cache-driver policy, and stable POSIX descriptor claims
  blocked.
- Phase 7 Pi 5 pointer-copy proof plan is accepted. It translates the accepted
  QEMU/substitute talos_copy_probe boundary into a future serialized physical
  proof with required success-copy, guard-range EFAULT, unknown-syscall,
  diagnostic-quarantine, classification, PASS, candidate-identity,
  fresh-serial/TFTP, hardwareTestLock, restoration, and inconclusive-run
  triage evidence. It does not run hardware and keeps descriptor I/O, process
  loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, DMA/cache-driver policy, and stable POSIX descriptor claims
  blocked.
- Phase 7 Pi 5 pointer-copy proof is accepted. It adds the focused
  rpi5_pointer_copy_proof scenario and scripts, then retains serialized Pi 5
  evidence with success-copy, guard-range -EFAULT, unknown-syscall -ENOSYS,
  diagnostic-marker quarantine, classification=pi5-pointer-copy-proof-complete,
  and rpi5-pointer-copy-proof: PASS. The evidence includes the required
  inconclusive-run triage and restore proof. Descriptor I/O, process loading,
  VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, and stable POSIX descriptor claims remain blocked.
- Phase 7 Pi 5 pointer-copy proof closeout checkpoint is accepted. It
  reconciles the pointer-taking contract, QEMU/substitute pointer-copy smoke,
  Pi 5 physical proof, hardware-lock timeline, restore proof, proof-only
  talos_copy_probe status, retained evidence paths, and blocked surfaces. It
  recommends phase7-descriptor-syscall-source-inventory-20260529 as the next
  bounded documentation-only task before descriptor syscall contracts or
  implementations.
- Phase 7 descriptor syscall source inventory is accepted. It maps
  src/posix.rs descriptor tables and copy helpers, src/syscall.rs stable svc #0
  dispatch, lower-AArch64 saved-frame argument capture, runtime-console0 and
  TTY backing surfaces, scheduler task/process ownership gaps, and retained
  QEMU evidence ownership. It recommends
  phase7-descriptor-syscall-contract-20260529 as a stdout/stderr write
  contract slice before any descriptor implementation. stdin/read, close, dup,
  process loading, VFS/filesystem, shell, networking, SSH, live process-owned
  address spaces, blocking/readiness, signals, restart semantics, RP1/PCIe,
  UART interrupt ownership, DMA/cache-driver policy, and stable POSIX
  descriptor claims remain blocked.
- Phase 7 QEMU descriptor-write smoke plan is accepted. It defines the later
  qemu_descriptor_write_smoke QEMU/substitute invariant for talos_write fd 1
  and fd 2 success through inherited stdio descriptors, copy_from_user(), and
  runtime-console0; fd 0 and invalid-fd -EBADF; guard-range -EFAULT;
  reserved-register -EINVAL; talos_nop and unknown-syscall regressions; and
  proof-only talos_copy_probe quarantine. It names
  phase7-descriptor-write-core-20260529 as the next bounded implementation
  task and keeps stdin/read, close, dup, process loading, VFS/filesystem,
  shell, networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, physical descriptor-write claims, and full POSIX
  descriptor claims blocked.
- Phase 7 descriptor-write closeout checkpoint is accepted. It reconciles the
  descriptor source inventory, talos_write contract, QEMU descriptor-write
  smoke plan, descriptor-write core, retained
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log
  evidence, scalar and pointer-copy regression gates, residual risks, and
  deferred surfaces. It recommends
  phase7-pi5-descriptor-write-proof-plan-20260529 as the next bounded
  documentation-only planning task before any Pi 5 descriptor-write hardware
  action. stdin/read, close, dup, process loading, VFS/filesystem, shell,
  networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, physical descriptor-write claims, and full POSIX
  descriptor claims remain blocked.
- Phase 7 Pi 5 descriptor-write proof plan is accepted. It defines the
  serialized physical proof invariant for talos_write fd 1/fd 2 through
  copy_from_user(), inherited stdio descriptors, and runtime-console0; fd and
  pointer errno cases; talos_nop and unknown-syscall regressions;
  talos_copy_probe and diagnostic-marker quarantine; hardwareTestLock
  ownership; candidate identity; fresh serial/TFTP evidence; restoration; and
  inconclusive-run triage. It names
  phase7-pi5-descriptor-write-proof-20260529 as the next bounded hardware
  task and keeps stdin/read, close, dup, process loading, VFS/filesystem,
  shell, networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, and full POSIX descriptor claims blocked.
- Phase 7 Pi 5 descriptor-write proof is accepted. It adds the focused
  rpi5_descriptor_write_proof scenario, Pi 5 descriptor-write lower-AArch64
  svc #0 handling, image/boot-tree helpers, retained local evidence, and
  retained lab evidence. The accepted local3 rerun includes fd 1 stdout and fd
  2 stderr runtime-console0 writes, fd0/fd99 -EBADF, guard -EFAULT, reserved
  x3 -EINVAL, talos_nop, unknown syscall -ENOSYS, copy-probe quarantine,
  diagnostic-marker quarantine, classification=pi5-descriptor-write-proof-complete,
  and rpi5-descriptor-write-proof: PASS. The first candidate run was
  inconclusive, so the retained evidence records candidate identity, fresh
  serial/TFTP cursors, a passing production-timer known-good control, an
  unchanged candidate rerun, and restore proof. It recommends
  phase7-pi5-descriptor-write-proof-closeout-checkpoint-20260529 next and
  keeps stdin/read, close, dup, process loading, VFS/filesystem, shell,
  networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, and full POSIX descriptor claims blocked.
- Phase 7 Pi 5 descriptor-write proof closeout checkpoint is accepted. It
  reconciles the descriptor syscall contract, QEMU descriptor-write smoke,
  retained Pi 5 local3 proof evidence, hardware-lock timeline, restore proof,
  residual risks, and blocked surfaces. It recommends
  phase7-syscall-abi-dispatch-closeout-checkpoint-20260529 as the next
  documentation-only Milestone 7.3 closeout task before any Milestone 7.4
  source inventory. stdin/read, close, dup, process loading, VFS/filesystem,
  shell, networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, and full POSIX descriptor claims remain blocked.
- Phase 7 syscall ABI/dispatch closeout checkpoint is accepted. It reconciles
  all accepted Milestone 7.3 tasks, commits, retained QEMU/substitute and Pi 5
  evidence paths, validation gates, diagnostic-surface quarantine, hardware
  lock/restore proof, and deferred surfaces. Milestone 7.3 is closed for the
  bounded lower-AArch64 svc #0 syscall ABI and dispatch frontier: x8 syscall
  numbers, x0-through-x5 arguments, x0 return/-errno encoding, stable
  talos_nop and unknown-syscall returns, copy_from_user/copy_to_user helper
  plumbing, proof-only talos_copy_probe, and talos_write fd 1/fd 2 writes to
  runtime-console0 through proof-owned inherited stdio descriptors. It
  recommends phase7-file-descriptor-table-source-inventory-20260529 as the
  next documentation-only Milestone 7.4 task. stdin/read, close, dup, process
  loading, VFS/filesystem, shell, networking, SSH, live process-owned address
  spaces, blocking/readiness, signals, restart semantics, RP1/PCIe, UART
  interrupt ownership, DMA/cache-driver policy, and full POSIX descriptor
  claims remain blocked.
- Phase 7 file descriptor table source inventory is accepted. It maps accepted
  descriptor table data-model owners, talos_write syscall dispatch, copy helper
  and user-memory boundaries, runtime-console/TTY stdio backing,
  scheduler/task process-owner vocabulary, deferred VFS/filesystem/device
  surfaces, and retained QEMU/Pi 5 descriptor-write evidence. It recommends
  phase7-process-descriptor-table-contract-20260529 as the next bounded
  documentation-only Milestone 7.4 task and does not add implementation, QEMU,
  Pi 5 hardware, or hardware-lock work.
- Phase 7 process descriptor table contract is accepted. It defines a
  ProcessOwnerId-backed descriptor-table owner, inherited stdio installation,
  runtime-console0 stdout/stderr backing, current-owner descriptor-table
  lookup, retained descriptor error behavior, and the next bounded
  phase7-process-descriptor-table-core-20260529 implementation task. PID
  allocation, process loading, close/dup/read syscalls, VFS/filesystem, stdin
  behavior, shell, networking, SSH, physical proof, and full POSIX descriptor
  claims remain blocked.
- Phase 7 process descriptor table core is accepted. It implements only the
  target-independent ProcessDescriptorOwner and bounded ProcessDescriptorStore
  for ProcessOwnerId-backed inherited stdio tables, current-owner lookup, and
  deterministic -EBADF/-EINVAL/-EMFILE error behavior. Live syscall routing,
  close/dup/read syscalls, process loading, VFS/filesystem, shell, networking,
  SSH, physical proof, and full POSIX descriptor claims remain blocked.
- Phase 7 QEMU process descriptor stdio smoke plan is accepted. It defines the
  QEMU/substitute proof that talos_write fd 1/fd 2 must use a
  ProcessOwnerId-backed process-owned inherited stdio table rather than the
  earlier proof-owned table, while preserving fd/error regressions,
  copy-probe quarantine, diagnostic-marker quarantine, and blocked physical
  claims.
- Phase 7 QEMU process descriptor stdio smoke core is accepted. It proves in
  QEMU/substitute output that lower-AArch64 talos_write fd 1/fd 2 resolves the
  current ProcessOwnerId through ProcessDescriptorStore and writes through the
  process-owned inherited stdio table to runtime-console0. It preserves fd 0
  and fd 99 -EBADF, guard-range -EFAULT, reserved-register -EINVAL, talos_nop,
  unknown-syscall -ENOSYS, copy-probe quarantine, diagnostic-marker
  quarantine, and exact PASS/classification evidence. Pi 5 physical proof,
  stdin/read, close/dup/read, process loading, VFS/filesystem, shell,
  networking, SSH, and full POSIX descriptor claims remain blocked.
- Phase 7 process descriptor table closeout is accepted. It closes the first
  process-owned descriptor-table slice at the QEMU/substitute evidence level,
  records the accepted contract/core/smoke commits and evidence path, and
  preserves blocked Pi 5 physical proof, stdin/read, close/dup/read,
  descriptor lifetime and close semantics, process loading, VFS/filesystem,
  shell, networking, SSH, and full POSIX descriptor claims. The next bounded
  Milestone 7.4 task should be supervisor-planned as a documentation-only
  descriptor lifetime and close-semantics source inventory.
- Phase 7 descriptor lifetime and close source inventory is accepted. It maps
  src/posix.rs table-local close/dup behavior, ProcessDescriptorStore
  owner-table mutation, inherited stdio lifetime, retained descriptor evidence,
  missing close/double-close/reuse/dup unit evidence, and open-file-description
  finalization gaps. It recommends
  phase7-descriptor-lifetime-close-contract-20260529 as the next
  documentation-only Milestone 7.4 task. Close/dup/read syscalls, process
  loading, VFS/filesystem, shell, networking, SSH, physical close/dup/read
  proof, and full POSIX descriptor readiness remain blocked.
- Phase 7 descriptor lifetime and close contract is accepted. It defines the
  supported table-local close rule, process-owned close lookup through
  ProcessDescriptorStore, EBADF cases, dup/reuse interaction, and deferred
  open-file-description finalization. It recommends
  phase7-descriptor-close-core-20260529 as the next target-independent
  Milestone 7.4 task. Close/dup/read syscalls, process loading,
  VFS/filesystem, shell, networking, SSH, physical close/dup/read proof,
  object finalization, and full POSIX descriptor readiness remain blocked.
- Phase 7 descriptor close core closeout is accepted. It records the accepted
  source inventory, contract, target-independent close helper implementation,
  changed files, focused unit tests, and validation gates. It accepts only
  ProcessDescriptorStore::close_current_descriptor() applying table-local close
  semantics to the current owner. Close/dup/read syscalls, lower-EL ABI, QEMU
  close/dup/read smoke, Pi 5 physical proof, process loading, VFS/filesystem,
  shell, networking, SSH, object finalization, and full POSIX descriptor
  readiness remain blocked. The next bounded Milestone 7.4 task should be a
  documentation-only close/dup/read syscall source inventory.
- Phase 7 close syscall core is accepted. It adds the target-independent
  talos_close syscall number/dispatch path and routes close through
  ProcessDescriptorStore::close_current_descriptor() with focused no_std
  tests. QEMU syscall and descriptor-write regression smokes still pass, but
  QEMU/Pi 5 close proof, dup/read, process loading, VFS/filesystem, shell,
  networking, SSH, object finalization, and full POSIX descriptor readiness
  remain blocked. The next bounded Milestone 7.4 task should be
  phase7-qemu-close-syscall-smoke-plan-20260529.
- Phase 7 QEMU close syscall smoke plan is accepted. It defines the later
  qemu_close_syscall_smoke QEMU/substitute invariant for talos_close on fd 1
  and fd 2 through the current ProcessOwnerId-backed descriptor table,
  closed-descriptor talos_write -EBADF behavior, unaffected descriptor
  behavior, scalar syscall regressions, and proof-only diagnostic quarantine.
  The next bounded Milestone 7.4 task should be
  phase7-qemu-close-syscall-smoke-core-20260529. Pi 5 physical close proof,
  dup/read syscalls, process loading, VFS/filesystem, shell, networking, SSH,
  object finalization, and full POSIX descriptor readiness remain blocked.
- Phase 7 QEMU close syscall smoke core is accepted. It adds and retains
  qemu_close_syscall_smoke evidence proving current-owner talos_close on fd 1
  and fd 2, closed-descriptor talos_write -EBADF behavior without
  runtime-console0 side effects, unaffected fd 2 writes after closing fd 1 and
  after a failed reserved close, repeated-close/badfd EBADF behavior, and
  talos_nop/unknown/copy-probe/diagnostic quarantine regressions. The next
  bounded Milestone 7.4 task should be
  phase7-close-syscall-closeout-checkpoint-20260529. Pi 5 physical close proof,
  dup/read syscalls, process loading, VFS/filesystem, shell, networking, SSH,
  object finalization, and full POSIX descriptor readiness remain blocked.
- Phase 7 close syscall closeout is accepted. It reconciles the accepted close
  syscall source inventory, contract, target-independent core, QEMU smoke plan,
  retained QEMU/substitute close smoke evidence, validation gates, and deferred
  surfaces. The accepted capability remains stable talos_close x8 = 2 through
  the current ProcessOwnerId-backed ProcessDescriptorStore at QEMU/substitute
  evidence level. The next bounded Milestone 7.4 task should be a
  documentation-only Pi 5 close syscall proof plan. Pi 5 physical close proof,
  dup/read syscalls, process loading, VFS/filesystem, shell, networking, SSH,
  object finalization, and full POSIX descriptor readiness remain blocked.
- Phase 7 Pi 5 close syscall proof plan is accepted. It defines the serialized
  rpi5_close_syscall_proof hardware plan, including lock ownership, candidate
  identity, fresh serial/TFTP requirements, inconclusive-run triage,
  restoration proof, exact close/write/error/quarantine/classification/PASS
  output, and deferred surfaces. No hardware run, archive publication, or
  physical close claim is made by the plan. The next bounded Milestone 7.4
  task should be phase7-pi5-close-syscall-proof-20260529. Dup/read, process
  loading, VFS/filesystem, shell, networking, SSH, object finalization, and
  full POSIX descriptor readiness remain blocked.
- Phase 7 QEMU dup syscall closeout is accepted. Subsequent Milestone 7.4 work
  has accepted the Pi 5 close proof and closeout, dup syscall contract, dup
  syscall core, QEMU dup smoke plan, QEMU dup smoke core, and this
  documentation-only closeout. The retained QEMU/substitute dup evidence proves
  fd 1 duplicates to fd 3, full-table -EMFILE, reserved-register -EINVAL,
  writes through source and duplicate stdout descriptors, close(fd 1)
  preserving fd 3, closed-descriptor -EBADF cases, scalar regressions,
  copy-probe quarantine, diagnostic-marker quarantine, and
  classification=qemu-dup-syscall-smoke-complete plus PASS. The next bounded
  Milestone 7.4 task should be
  phase7-pi5-dup-syscall-proof-plan-20260529. Pi 5 physical dup proof,
  read/stdin behavior, process loading, VFS/filesystem, shell, networking,
  SSH, object finalization, dup2/fcntl, and full POSIX descriptor readiness
  remain blocked.
- Phase 7 Pi 5 dup syscall proof plan is accepted. It defines the serialized
  rpi5_dup_syscall_proof hardware plan, including lock ownership, candidate
  identity, fresh serial/TFTP requirements, inconclusive-run triage,
  restoration proof, exact dup/write/close/error/quarantine/classification/PASS
  output, and deferred surfaces. No hardware run, archive publication, or
  physical dup claim is made by the plan. The next bounded Milestone 7.4 task
  should be phase7-pi5-dup-syscall-proof-20260529. Read/stdin behavior,
  process loading, VFS/filesystem, shell, networking, SSH, object finalization,
  dup2/fcntl, and full POSIX descriptor readiness remain blocked.
- Phase 7 Pi 5 dup syscall proof is accepted. Retained local8 physical serial
  evidence proves current-owner lookup, fd 1 dup to fd 3, full-table -EMFILE,
  reserved-register -EINVAL, source and duplicate stdout writes, close(fd 1)
  preserving fd 3, duplicate close, closed-descriptor -EBADF, talos_nop,
  unknown-syscall -ENOSYS, copy-probe quarantine,
  classification=pi5-dup-syscall-proof-complete, and PASS. local7 is the
  accepted known-good production-timer control after earlier inconclusive
  local4/local5/local6 evidence. The boot tree was restored to the
  pre-pi5-dup-syscall-proof-local1-20260529 snapshot. The next bounded
  Milestone 7.4 task should be
  phase7-pi5-dup-syscall-proof-closeout-checkpoint-20260529. Read/stdin
  behavior, process loading, VFS/filesystem, shell, networking, SSH, object
  finalization, dup2/fcntl, and full POSIX descriptor readiness remain
  blocked.
- Phase 7 Pi 5 dup syscall proof closeout is accepted. It reconciles the dup
  contract/core, retained QEMU/substitute smoke, serialized local8 Pi 5
  hardware proof, local7 known-good control, hardware-lock timeline, restore
  proof, residual risks, and deferred surfaces. The accepted frontier is a
  focused physical talos_dup x8 = 3 proof through the current
  ProcessOwnerId-backed ProcessDescriptorStore, including fd 1 to fd 3,
  -EMFILE, -EINVAL, source/duplicate writes, independent close behavior,
  -EBADF cases, scalar/unknown regressions, copy-probe quarantine, diagnostic
  marker quarantine, classification=pi5-dup-syscall-proof-complete, and PASS.
  The next bounded Milestone 7.4 task should be a supervisor-queued
  documentation-only read/stdin source inventory. Read/stdin behavior, process
  loading, VFS/filesystem, shell, networking, SSH, object finalization,
  dup2/fcntl, and full POSIX descriptor readiness remain blocked.
- Phase 7 QEMU read/stdin smoke core is accepted. Retained QEMU/substitute
  evidence proves qemu_read_stdin_smoke through the lower-AArch64 stable
  talos_read path with fd 0 duplication, fixed proof stdin, errno cases,
  short-read, EOF, scalar regressions, copy-probe quarantine,
  diagnostic-marker quarantine, classification, and PASS. The next bounded
  Milestone 7.4 task should be
  phase7-read-stdin-closeout-checkpoint-20260529. Pi 5 physical read proof,
  runtime-console0/TTY/hardware stdin, process loading, VFS/filesystem, shell,
  networking, SSH, object finalization, dup2/fcntl, and full POSIX descriptor
  readiness remain blocked.
- Phase 7 read/stdin closeout checkpoint is accepted. The checkpoint reconciles
  the accepted inventory, contract, target-independent core, QEMU smoke plan,
  retained QEMU/substitute evidence, residual risks, and deferred surfaces. The
  next mechanically derivable Milestone 7.4 task should be
  phase7-pi5-read-stdin-proof-plan-20260530, queued explicitly by the
  supervisor before any Pi 5 hardware action.
- Phase 7 Pi 5 read/stdin proof plan is accepted. It defines the serialized
  rpi5_read_stdin_proof hardware plan for carrying the accepted fixed-stdin
  talos_read QEMU/substitute invariant to Raspberry Pi 5. The plan requires
  lock ownership, candidate identity, archive/kernel hashes, fresh serial and
  TFTP evidence, inconclusive-run triage, restoration proof, exact fd 0/fd 3
  read, errno, EOF, scalar-regression, copy-probe quarantine,
  diagnostic-marker quarantine, classification, and PASS output. No hardware
  run, archive publication, or physical read claim is made by the plan. The
  next bounded Milestone 7.4 task should be
  phase7-pi5-read-stdin-proof-20260530. runtime-console0/TTY/hardware stdin,
  process loading, VFS/filesystem, shell, networking, SSH, object finalization,
  dup2/fcntl, and full POSIX descriptor readiness remain blocked.

Milestone 7.4: File Descriptor Table

- Implement per-process descriptor tables.
- Model standard input, output, error, pipes, devices, and later sockets through one interface.
- Status: closed for the bounded descriptor-table frontier accepted by
  phase7-file-descriptor-table-closeout-checkpoint-20260530. The accepted
  frontier covers ProcessOwnerId-backed inherited stdio, descriptor-backed
  stdout/stderr writes, talos_close, talos_dup, and fixed-proof-stdin
  talos_read through fd 0/fd 3. Pipes, devices beyond runtime-console0, TTY or
  hardware stdin, filesystems, sockets, process loading, shell, networking,
  SSH, object finalization, dup2/fcntl, signals, wait queues, nonblocking I/O,
  RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
  descriptor readiness remain deferred.

Phase 7 final frontier checkpoint:

- Phase 7 final frontier source inventory is accepted. It maps Phase 7.1
  through Phase 7.4 accepted capabilities, commit/evidence anchors, deferred
  surfaces, and residual risks. No implementation, QEMU run, Pi 5 run, archive
  publication, hardware-lock acquisition, or Phase 8 transition was performed.
  It reports no remaining bounded Phase 7 implementation or evidence blocker
  before the final closeout checkpoint and recommends
  phase7-final-closeout-checkpoint-20260530 as the next mechanically unblocked
  task.
- Phase 7 final closeout checkpoint is accepted. Phase 7 is closed for the
  bounded POSIX/EL0/syscall/copy-helper/descriptor frontier accepted by the
  Phase 7.1 through Phase 7.4 closeouts. The checkpoint records no remaining
  bounded Phase 7 implementation or evidence blocker before Phase 8 source
  inventory planning, and it recommends
  phase8-filesystem-program-loading-source-inventory-20260530 as the next
  mechanically derivable documentation-only task once the durable
  phaseCheckpointStatus recommendation flag is set. This does not accept
  filesystem/program loading, shell, networking, SSH, runtime-console0/TTY or
  hardware stdin, object finalization, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, or full POSIX readiness.

Acceptance criteria:

- A test process can read/write through descriptor-backed console streams.
- Descriptor lifetime and close semantics are documented.
- Phase 7 closes only for the bounded accepted frontier and explicitly leaves
  Phase 8 runtime capability to later tasks.

## Phase 8: Filesystem and Program Loading

Goal: make Talos able to run more than built-in commands.

Milestone 8.1: Initramfs or Ramfs

- Add an embedded or TFTP-loaded initramfs for early files.
- Implement path lookup, file metadata, and read-only file contents.
- Phase 8 filesystem/program-loading source inventory is accepted. It maps
  existing owners and missing contracts for POSIX path copying, VFS/filesystem
  objects, descriptor inheritance, process identity, address-space setup,
  executable images, argv/envp, and boot/test scenarios. It recommends
  phase8-readonly-initramfs-vfs-contract-20260530 as the next
  documentation-only task and keeps ELF/program loading, process creation,
  shell, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy blocked.
- The read-only initramfs/VFS contract is accepted. It defines the immutable
  initial filesystem content model, root/directory/regular-file vocabulary,
  path-copy and lookup rules, descriptor-facing regular-file read semantics,
  errno precedence, deterministic fixture expectations, and deferred surfaces.
  It recommends phase8-readonly-initramfs-vfs-smoke-plan-20260530 next and
  keeps target-independent core implementation, QEMU runtime evidence, Pi 5
  hardware proof, ELF/program loading, process creation, shell, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy blocked
  until later explicit tasks accept their gates.
- The read-only initramfs/VFS smoke plan is accepted. It defines the
  qemu_readonly_initramfs_vfs_smoke scenario, deterministic fixture contents,
  lookup/read/offset/EOF observations, ENOENT/ENOTDIR/EISDIR/ENAMETOOLONG/
  EBADF/EFAULT/EINVAL/ENOTSUP negative cases, exact PASS/classification lines,
  retained QEMU/substitute evidence path, failure classification, and
  regression gates. It recommends
  phase8-readonly-initramfs-vfs-core-20260530 next, followed by
  phase8-qemu-readonly-initramfs-vfs-smoke-core-20260530 after the core is
  accepted. QEMU runtime evidence, Pi 5 hardware proof, ELF/program loading,
  process creation, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain blocked until later explicit
  tasks accept their gates.
- The target-independent read-only initramfs/VFS core is accepted. It adds the
  immutable fixture object model, deterministic root/directory/regular-file
  nodes, normalized absolute and current-directory-relative lookup,
  regular-file open-file descriptions, all-or-nothing copy_to_user-backed
  reads, offset/EOF behavior, and focused no_std unit tests for accepted
  success and failure cases. It does not wire the filesystem to production
  lower-EL syscalls, run QEMU, run Pi 5 hardware, publish a boot archive, parse
  firmware/TFTP initramfs envelopes, or unblock ELF/program loading, process
  creation, shell, networking, SSH, writable filesystems, persistent storage,
  RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy. The next
  bounded task is phase8-qemu-readonly-initramfs-vfs-smoke-core-20260530.
- The QEMU/substitute read-only initramfs/VFS smoke is accepted. It proves the
  planned fixture identity, lookup, regular-file reads, offset/EOF behavior,
  and deterministic ENOENT/ENOTDIR/EISDIR/ENAMETOOLONG/EBADF/EFAULT/EINVAL/
  ENOTSUP cases, and retains
  tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core/qemu-readonly-initramfs-vfs-smoke.log
  with classification=qemu-readonly-initramfs-vfs-smoke-complete and PASS.
  It does not accept Pi 5 hardware proof, boot archive publication,
  descriptor-backed production filesystem syscalls, open syscall ABI,
  firmware/TFTP initramfs delivery, ELF/program loading, process creation,
  shell, networking, SSH, writable filesystems, persistent storage, RP1/PCIe,
  UART interrupt ownership, or DMA/cache-driver policy. The read-only
  initramfs/VFS closeout checkpoint is accepted and recommends
  phase8-program-loader-source-inventory-20260530 as the next bounded
  documentation-only task before any loader implementation.
- The read-only initramfs/VFS closeout checkpoint is accepted. It reconciles
  the accepted contract, smoke plan, target-independent core, retained
  QEMU/substitute evidence, deferred surfaces, and residual risks, and it
  recommends phase8-program-loader-source-inventory-20260530 as the next
  bounded documentation-only task. It does not accept descriptor-backed
  filesystem syscalls, executable /bin/init, ELF/program loading, process
  creation, shell, Pi 5 hardware proof, networking, SSH, RP1/PCIe, UART
  interrupt ownership, or DMA/cache-driver policy.

Acceptance criteria:

- A diagnostic command or test process can list and read files from the initial filesystem.

Milestone 8.2: VFS

- Add VFS nodes for regular files, directories, devices, and pipes.
- Keep interfaces compatible with future persistent filesystems.

Acceptance criteria:

- Common file operations route through the VFS, not ad hoc shell logic.

Milestone 8.3: Program Loader

- Choose an executable format for early user programs.
- Load a program from initramfs, map it into a process, and pass arguments.
- Phase 8 program-loader source inventory is accepted. It maps the accepted
  read-only initramfs/VFS regular-file input, current source owners for
  filesystem bytes, POSIX errors, user-memory permissions, lower-EL proof
  payloads, scheduler/process-owner placeholders, descriptor inheritance, and
  evidence conventions. It also records missing contracts for executable
  format selection, ELF/header validation, segment permissions, zero-fill,
  entry-point validation, user stack and argv/envp layout, loader error
  mapping, process-install ownership, and descriptor inheritance. The next
  bounded task is phase8-program-loader-format-contract-20260530. ELF parsing,
  loader implementation, process creation, exec/spawn/wait, shell, Pi 5
  hardware proof, writable filesystems, persistent storage, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked until later explicit tasks accept their gates.
- Phase 8 program-loader format contract is accepted. It selects the first
  executable format policy: a narrow static ELF64/AArch64 ET_EXEC subset from
  the accepted read-only initramfs/VFS regular-file boundary. It defines
  header and program-header validation, dynamic/interpreter rejection,
  PT_LOAD segment permission mapping, W^X rejection, user-range and overlap
  checks, BSS zero-fill, entry-point validation, deterministic loader errors,
  and the process/address-space/stack/descriptor boundaries that remain later
  responsibilities. The next bounded task is
  phase8-qemu-program-loader-smoke-plan-20260530. Loader Rust implementation,
  process address-space installation, argv/envp stack construction,
  process creation, exec/spawn/wait, shell, descriptor-backed filesystem
  syscalls, Pi 5 hardware proof, writable filesystems, persistent storage,
  networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy remain blocked until later explicit tasks accept their gates.
- Phase 8 QEMU/substitute program-loader smoke plan is accepted. It defines
  the qemu_program_loader_smoke scenario, fixture identity
  phase8-program-loader-elf64-aarch64-v1, image-plan-only success
  observations for a narrow static ELF64/AArch64 ET_EXEC /bin/init fixture,
  deterministic negative cases for bad magic, dynamic interpreter, W+X
  segment, out-of-user-range segment, overlap, bad entry, and file-range
  overflow, retained evidence path
  tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log,
  PASS/classification lines, and conditional regression gates. The next
  bounded implementation task should be phase8-program-loader-core-20260530
  only after supervisor planning queues it with explicit scope and gates.
  Loader core implementation, process address-space installation, lower-EL
  launch of a loaded image, argv/envp stack construction, process creation,
  exec/spawn/wait, shell, descriptor-backed filesystem syscalls, Pi 5 hardware
  proof, writable filesystems, persistent storage, networking, SSH, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy remain blocked until
  later explicit tasks accept their gates.
- Phase 8 program-loader core is accepted. It adds the target-independent
  ELF64/AArch64 static ET_EXEC image-plan validator for immutable /bin/init
  bytes from the read-only initramfs/VFS fixture, reports digest/source
  identity, ordered UserText/UserData segments, file-copy ranges, explicit BSS
  zero-fill, entry placement, total footprint, and deterministic loader errors
  before any process-owned install surface exists.
- Phase 8 QEMU/substitute program-loader smoke core is accepted. The
  qemu_program_loader_smoke scenario and
  scripts/qemu-program-loader-smoke.sh retain
  tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log
  with fixture identity phase8-program-loader-elf64-aarch64-v1, digest
  0x3892eed223900c65, success image-plan lines, all seven required negative
  errno lines with partial-install=false, final
  classification=qemu-program-loader-smoke-complete, and
  qemu-program-loader-smoke: PASS. This evidence is QEMU/substitute only; Pi 5
  hardware proof, process address-space installation, lower-EL launch of the
  loaded image, argv/envp stack construction, process creation, exec/spawn/wait,
  shell, descriptor-backed filesystem syscalls, writable filesystems,
  persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain blocked until later explicit tasks accept
  their gates.
- Phase 8 program-loader closeout checkpoint is accepted. It reconciles the
  accepted source inventory, format contract, smoke plan, target-independent
  core, retained QEMU/substitute evidence, validation gates, deferred surfaces,
  and residual risks for the image-plan-only frontier. It recommends
  phase8-process-install-source-inventory-20260530 as the next bounded
  documentation-only task and keeps process address-space installation,
  lower-EL launch, argv/envp stack construction, process creation,
  exec/spawn/wait, shell, descriptor-backed filesystem syscalls, Pi 5 hardware
  proof, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy blocked until later explicit tasks accept their
  gates.
- Phase 8 process-install source inventory is accepted. It maps the source
  owners and gaps between the accepted ProgramImagePlan and any future
  process-owned address-space installation: frame allocation, page-table
  mutation, rollback, initial lower-EL frame, user stack, descriptor
  inheritance, process identity, and scheduler handoff. It recommends
  phase8-process-install-contract-20260530 as the next bounded
  documentation-only task. Rust implementation, QEMU execution, Pi 5 hardware
  proof, lower-EL launch of the loaded image, argv/envp construction,
  exec/spawn/wait, shell, descriptor-backed filesystem syscalls, writable
  filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain blocked until later explicit
  tasks accept their gates.
- Phase 8 process-install contract is accepted. It selects a
  target-independent metadata-only ProcessImageInstallPlan boundary derived
  from a validated ProgramImagePlan, with exact UserText/UserData permission
  preservation, ordered page records, clipped file-copy and zero-fill ranges,
  deterministic errors, and all-or-nothing semantics. It accepts no frame
  allocation, physical byte copy, page-table mutation, scheduler handoff,
  lower-EL launch, argv/envp, descriptor inheritance, shell, hardware, or
  filesystem syscall behavior.
- Phase 8 QEMU/substitute process-install smoke plan is accepted. It defines
  qemu_process_install_smoke, fixture identity
  phase8-program-loader-elf64-aarch64-v1, install boundary identity
  phase8-process-install-plan-v1, retained evidence path
  tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log,
  final classification qemu-process-install-smoke-complete, PASS vocabulary,
  success observations for metadata-only ProcessImageInstallPlan derivation,
  deterministic rejection observations for bad plan invariants, overlap,
  permission widening, bad entry, and budget overflow, plus conditional
  regression gates. The next bounded implementation task should be
  phase8-process-install-core-20260530 only because supervisor planning has
  already queued it with explicit scope and gates. Physical page allocation,
  page-table mutation, lower-EL launch, argv/envp construction, process
  creation, exec/spawn/wait, shell, descriptor-backed filesystem syscalls,
  Pi 5 hardware proof, writable filesystems, persistent storage, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked until later explicit tasks accept their gates.
- Phase 8 process-install core is accepted. It adds the metadata-only
  ProcessImageInstallPlan owner in src/process_install.rs, deriving ordered
  page install records from a validated ProgramImagePlan while preserving
  UserText R-X and UserData RW- permissions, exact fixture identity, source
  digest, entry point, total rounded footprint, clipped file-copy ranges,
  explicit zero-fill ranges, and the later action order
  allocate/copy/zero/map. The implementation is target-independent and returns
  deterministic POSIX-shaped errors for malformed plan invariants, overlap,
  permission widening, bad entry, budget overflow, and invalid source ranges.
  No frame allocation, physical byte copy, page-table mutation, process
  creation, descriptor mutation, lower-EL frame, runnable task, QEMU smoke,
  Pi 5 hardware proof, argv/envp construction, exec/spawn/wait, shell,
  writable filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or
  DMA/cache-driver policy is accepted by this core.
- Phase 8 QEMU/substitute process-install smoke core is accepted. It adds
  qemu_process_install_smoke routing, retained evidence at
  tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log,
  and PASS/classification checks for metadata-only success, exact page
  permission preservation, zero physical side effects, and deterministic
  no-partial-install rejections. The next bounded task should be the queued
  phase8-process-install-closeout-checkpoint-20260530 if dependencies remain
  satisfied. Physical page allocation, page-table mutation, lower-EL launch,
  argv/envp construction, process creation, exec/spawn/wait, shell,
  descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
  filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain blocked until later explicit
  tasks accept their gates.

Acceptance criteria:

- A separate user program can be launched and waited on.

## Phase 9: Libc, Rust Std, and Portable Userland

Goal: make existing user programs portable to Talos instead of hand-writing a
complete command suite.

Milestone 9.1: Libc Strategy

- Define the Talos userspace ABI: startup, crt objects, errno, environment,
  arguments, TLS expectations, allocator hooks, and syscall wrappers.
- Evaluate a small libc path first: Talos-native libc shim, relibc, newlib, or
  musl when the syscall surface is mature enough.
- Treat glibc as a later compatibility target, not the first libc goal. It
  assumes a broad Linux-like environment and is too heavy for the first
  userspace porting layer.

Acceptance criteria:

- An ADR chooses the first libc strategy and records why glibc is deferred or
  rejected for the initial port.
- Simple C programs can call libc wrappers for write, read, open, close, exit,
  malloc/free, and basic path operations.
- Host-side and QEMU tests cover syscall-wrapper error behavior.

Milestone 9.2: Rust Userspace Target and Std Subset

- Define a Talos Rust userspace target distinct from the kernel target.
- Bring up enough Rust runtime support for no_std user programs first, then a
  constrained std subset when libc, allocation, filesystem, time, and descriptor
  behavior are ready.
- Keep proc-macros, build scripts, dynamic loading, and native compilation out
  of scope for this milestone.

Acceptance criteria:

- A cross-compiled Rust user program runs on Talos and uses arguments, stdio,
  heap allocation, and file reads.
- The supported and unsupported Rust std APIs are documented.
- Cargo target configuration for Talos userspace exists.

Milestone 9.3: Core Utilities Port

- Prefer Rust uutils/coreutils once the Rust userspace target is viable.
- Keep toybox, busybox, or GNU coreutils as fallback/reference ports if they
  expose missing POSIX semantics more clearly.
- Start with a small command set: cat, echo, true, false, ls, pwd, cp, mv, rm,
  mkdir, and sh-compatible process launching where practical.

Acceptance criteria:

- A cross-compiled utility suite can be packaged into initramfs/ramfs.
- Basic utilities run as separate user programs through the normal process,
  descriptor, and filesystem paths.
- Porting gaps become tracked syscall/libc/VFS tasks instead of local hacks.

## Phase 10: Local Shell and Developer UX

Goal: make Talos useful from a local console before depending on Ethernet.

Milestone 10.1: Local Shell

- Implement or port a small shell that runs as a user program.
- Use the normal process, descriptor, TTY, filesystem, and program-loader
  mechanisms.
- Support built-ins only where they reflect normal shell behavior, not kernel
  shortcuts.

Acceptance criteria:

- A user can interact through the serial TTY, run commands, inspect files, and
  launch separate user programs.
- Shell I/O uses stdin/stdout/stderr descriptors.
- Shell limitations and POSIX gaps are documented.

Milestone 10.2: Pipelines and Process Control

- Add pipes, redirection, exit status, wait, and basic job/process accounting.
- Keep signals minimal at first but avoid designs that make POSIX signals
  impossible later.

Acceptance criteria:

- The shell can run simple pipelines and report exit statuses.
- Multiple user programs can make progress while the shell remains responsive.
- Descriptor inheritance and close-on-exec behavior are tested.

Milestone 10.3: Persistent or Larger Local Storage

- Evaluate SD, USB mass storage, generated image roots, and TFTP-loaded
  initramfs expansion for a practical development filesystem.
- Add a persistent filesystem path only after VFS and block/storage ownership
  rules are clear.

Acceptance criteria:

- Talos can load a nontrivial userland image without rebuilding the kernel for
  every user program change.
- Documentation explains the chosen local storage path and remaining risks.

## Phase 11: RP1, PCIe, DMA, and Hardware Substrate

Goal: understand the Pi 5 I/O substrate before relying on RP1 devices for
networking, GPIO, storage, or broader hardware support.

Milestone 11.1: RP1 and PCIe Mapping

- Determine whether firmware leaves RP1 configured and usable for early
  bare-metal access.
- Map the BCM2712 PCIe2 window, RP1 BAR/peripheral ranges, and address
  translations from device tree.
- Decide how much PCIe enumeration Talos needs for built-in RP1 versus external
  PCIe devices.

Acceptance criteria:

- A hardware note documents CPU physical addresses for initial RP1 access.
- A diagnostic can read a stable RP1 register or otherwise prove RP1 mapping
  assumptions.
- Known limitations around firmware-initialized state are recorded.

Milestone 11.2: RP1 Interrupts, Clocks, and GPIO

- Trace RP1 interrupt delivery into the BCM2712/GIC path.
- Identify clock/reset dependencies needed before Talos-owned RP1 drivers.
- Add a narrow GPIO or status-LED diagnostic only after mapping and interrupt
  assumptions are understood.

Acceptance criteria:

- RP1 interrupt routing is documented with source references.
- A minimal RP1 diagnostic works or the blocker is captured with serial
  evidence.

Milestone 11.3: DMA, IOMMU, and Cache Maintenance

- Determine RP1 DMA addressability, dma-ranges, IOMMU behavior, and
  cache-coherency requirements.
- Define kernel APIs for cache clean/invalidate and DMA-safe buffers before
  Ethernet or block drivers use DMA.

Acceptance criteria:

- DMA buffer ownership and cache-maintenance rules are documented.
- A small DMA or driver-adjacent diagnostic exists before networking depends on
  DMA.

## Phase 12: Networking and SSH Development Access

Goal: reach Talos over the network and make the system usable without serial.

Milestone 12.1: RP1 Ethernet Research Spike

- Study RP1 Ethernet as exposed by Linux device tree: rp1_eth is compatible with raspberrypi,rp1-gem and cdns,macb, behind RP1 PCIe address space.
- Decide whether to implement the Cadence GEM path directly, reuse a no_std driver if viable, or stage networking through a simpler transport first.
- Capture RP1 PCIe, RP1 interrupt routing, clocks, DMA, IOMMU, PHY reset, and cache-coherency implications. RP1 is not a simple fixed MMIO block from the CPU's point of view.

Acceptance criteria:

- A design note or ADR records the chosen Ethernet path.
- Unknown hardware behavior has diagnostic tasks.

Milestone 12.2: Network Device Abstraction

- Reuse the Daedalus idea of a small NetworkDevice trait, but revise it for Talos needs.
- Keep packet parsing and protocol logic testable without hardware.

Acceptance criteria:

- Ethernet, ARP, and IP parsing tests run in QEMU or host-side unit tests.
- Driver-specific code is isolated from protocol code.

Milestone 12.3: IP Stack

- Prefer smoltcp for no_std TCP/IP evaluation rather than hand-rolling TCP
  unless a concrete Talos constraint rules it out.
- Implement packet buffers, ARP, IPv4, ICMP, UDP/TCP, and socket integration.

Acceptance criteria:

- Talos responds to ping on the lab network.
- Talos can establish a TCP connection or accept one through a test service.

Milestone 12.4: Socket Integration

- Integrate sockets with the existing descriptor table, scheduler, blocking I/O,
  poll/wakeup model, and process lifetime rules.
- Add network diagnostics as user programs where possible, not kernel-only
  command paths.

Acceptance criteria:

- User programs can open sockets through the normal syscall/libc path.
- A network diagnostic program can accept or initiate a TCP connection.
- Blocking network I/O does not stall unrelated tasks.

Milestone 12.5: Entropy, Crypto, and SSH Strategy

- Bring up a kernel entropy source suitable for SSH host keys and session crypto.
- Evaluate porting an existing SSH server before writing one. OpenSSH is the
  compatibility target, but a smaller Rust SSH server may be a better first
  user-space port if it fits Talos libc/std and crypto constraints sooner.
- Define host key provisioning, authorized key storage, authentication policy, time requirements, heap-pressure expectations, and failure modes.

Acceptance criteria:

- ADR records the SSH implementation strategy.
- Entropy and key-management diagnostics exist before accepting SSH connections.

Milestone 12.6: SSH and Shell

- Implement or port the chosen SSH server and connect it to the existing local
  shell, PTY/TTY, descriptor, process, and filesystem model.
- Use SSH as the preferred path for user-space development and testing once it
  is reliable. Kernel changes may still use TFTP and lab power control, but
  user programs should not require serial-only workflows.

Acceptance criteria:

- User can connect remotely and run a shell.
- Multiple programs or commands can make progress concurrently.
- User-space programs can be copied, launched, and tested over SSH without using
  serial as the primary interaction channel.

## Phase 13: Toward a Useful Unix-Like System

Goal: grow from a local and remote shell into a practical small OS.

Milestones:

- Process spawning and wait/exit status.
- Permissions and user model sufficient for local experimentation.
- More complete POSIX compatibility review.
- Package/update workflow for user-space programs.
- Broader utility and service ports.
- Native build tools may be explored incrementally, but self-hosting GCC, LLVM,
  or rustc remains a north-star objective outside the committed roadmap.

Acceptance criteria:

- The shell can run separate programs, pipe output, inspect files, and operate
  locally or over SSH.
- Documentation explains how each major subsystem works and what POSIX gaps remain.

## Rolling Documentation Requirements

Each milestone should update at least one of:

- roadmap status
- task record
- architecture doc
- hardware note
- ADR
- lab runbook

Source-backed findings should cite URLs or local file references. Serial logs and boot attempts should be saved when they influence design decisions.
