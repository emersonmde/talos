# Phase 7 Syscall ABI and Dispatch Closeout Checkpoint

Status: accepted as the documentation-only closeout for Milestone 7.3 syscall
ABI and dispatch. This checkpoint adds no Rust or assembly behavior, QEMU
rerun, Pi 5 hardware rerun, boot archive publication, hardwareTestLock
acquisition, Milestone 7.4 implementation, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, or phase transition.

## Accepted Inputs

| Task | Commit | Evidence level |
| --- | --- | --- |
| phase7-syscall-abi-source-inventory-20260529 | 19f1dae4ff66fde61c778e7364fcf29edc1fcb58 | static documentation/source inspection |
| phase7-syscall-abi-contract-20260529 | 380994e6003c048c4b88497e52c327c18ca3dffd | static documentation/source inspection |
| phase7-syscall-dispatch-core-20260529 | 734160cee68e69c02c0aea124ba185ea7e36bdc3 | fmt/unit tests/static inspection |
| phase7-syscall-trap-routing-source-inventory-20260529 | 92529b0536901b52f7aa1ee5dee5317ff80308a2 | static documentation/source inspection |
| phase7-syscall-trap-routing-contract-20260529 | 10aa4423db70b80a134edc31dbb4c7c34a9f7554 | static documentation/source inspection |
| phase7-qemu-syscall-smoke-plan-20260529 | 51584084d97f93e794ed531a747cf0174e9c4950 | static documentation/source inspection |
| phase7-qemu-syscall-smoke-core-20260529 | 3abaf63ec11830137df15f0e3947161cad11688c | QEMU/substitute serial evidence |
| phase7-syscall-routing-closeout-checkpoint-20260529 | 96c47459ba181dbbbf411d52a8f6ff8906fde3e6 | static documentation inspection |
| phase7-pi5-syscall-proof-plan-20260529 | fd79657c4d3834616a578e410f28f661323c7db8 | static documentation inspection |
| phase7-pi5-syscall-proof-20260529 | 63ee22e4c1d01e772b0f530835355bf7ef3d7d80 | serialized Pi 5 hardware boot/output |
| phase7-pi5-syscall-proof-closeout-checkpoint-20260529 | 60fc1cc51f35b5db0066615410c87a8ac7f4a081 | static documentation inspection |
| phase7-copyin-copyout-helper-contract-20260529 | 4da7da68022b737c2e76531071b4624272d6d3f2 | static documentation/source inspection |
| phase7-copyin-copyout-helper-core-20260529 | b675a6f10fbb3e91781f98bd0ae63290ee4e967c | fmt/unit tests/static inspection |
| phase7-copyin-copyout-helper-closeout-checkpoint-20260529 | d2476b42d256e71e7874ec38d424b412a9c36bd6 | static documentation inspection |
| phase7-pointer-taking-syscall-source-inventory-20260529 | 56d1df22cafd6329eb9c836c9e75a2fc5a5fc2a7 | static documentation/source inspection |
| phase7-pointer-taking-syscall-contract-20260529 | ddefb045443010a3de0dd89a046454df93f192c2 | static documentation/source inspection |
| phase7-qemu-pointer-copy-smoke-plan-20260529 | 75414541efb936f467ce57e270b1701edcba9b3d | static documentation/source inspection |
| phase7-qemu-pointer-copy-smoke-core-20260529 | 10c23e00e04173fa9b8af987273b047d2dd4e2e3 | QEMU/substitute serial evidence |
| phase7-pointer-copy-closeout-checkpoint-20260529 | a30883bc5b4458850fe369b4558c27dc97736258 | static documentation inspection |
| phase7-pi5-pointer-copy-proof-plan-20260529 | a5a1b9856f057a456bdcdb52eeaa523fab5c7adb | static documentation inspection |
| phase7-pi5-pointer-copy-proof-20260529 | af0a3590b904be6d5b95ecc884da27bb48cff718 | serialized Pi 5 hardware boot/output |
| phase7-pi5-pointer-copy-proof-closeout-checkpoint-20260529 | 445696a3253a8c015d76e113673781bd7f388caf | static documentation inspection |
| phase7-descriptor-syscall-source-inventory-20260529 | 96dda33fbca64ed71c6d8ea76d21e4fd030463c4 | static documentation/source inspection |
| phase7-descriptor-syscall-contract-20260529 | 23429329540dfa87ebc13a5086829173400791ea | static documentation/source inspection |
| phase7-qemu-descriptor-write-smoke-plan-20260529 | dd338a284f8c9ba47c36b0735ade498664ff439f | static documentation/source inspection |
| phase7-descriptor-write-core-20260529 | e462f45ff98fe5196900c2c5ce8783a997349568 | fmt/unit tests/QEMU regression build gates |
| phase7-qemu-descriptor-write-smoke-core-20260529 | 26c36ffaada05e4ba598144c44f49210534b233a | QEMU/substitute serial evidence |
| phase7-descriptor-write-closeout-checkpoint-20260529 | d00b1939ed49266b107d5d130a64e6851a5f628a | static documentation inspection |
| phase7-pi5-descriptor-write-proof-plan-20260529 | 194a9de74603be601fc9b89b324efb4886e9e4fb | static documentation inspection |
| phase7-pi5-descriptor-write-proof-20260529 | f2762a9015053e6cd6cf60e54dd4d92789fddc3d | serialized Pi 5 hardware boot/output |
| phase7-pi5-descriptor-write-proof-closeout-checkpoint-20260529 | 4ad3f0a523d9243644ed3108eade78651f2c3600 | static documentation inspection |

## Retained Evidence

Milestone 7.3 keeps these retained evidence anchors:

- QEMU scalar syscall smoke:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log.
- QEMU EL0 diagnostic regression:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-el0-trap-smoke.log.
- Pi 5 scalar syscall proof:
  tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- QEMU proof-only pointer-copy smoke:
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log.
- Pi 5 proof-only pointer-copy proof:
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- QEMU descriptor-write smoke:
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.
- Pi 5 descriptor-write proof:
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt.

The Pi 5 proof tasks also retain candidate identity, kernel/archive digests,
TFTP deltas, hardware-lock timelines, known-good controls after inconclusive
runs, unchanged candidate reruns, and post-restore boot-tree proofs in their
task evidence directories.

## Accepted Capability

Milestone 7.3 now accepts the following bounded syscall ABI and dispatch
capability:

1. Lower-AArch64 svc #0 is the first stable syscall trap vocabulary.
2. x8 carries the syscall number; x0 through x5 carry scalar arguments; x0 is
   the only return register; negative x0 values encode -errno.
3. talos_nop x8 = 0 returns 0, and unknown syscall numbers return -ENOSYS.
4. Production lower-AArch64 SVC routing extracts arguments from the saved
   frame, calls the target-independent dispatch core, writes only saved x0,
   preserves ELR/SPSR, and keeps diagnostic marker 0x7a10 outside stable
   syscall dispatch.
5. copy_from_user and copy_to_user provide target-independent whole-range
   validation, read/write permission checks, all-or-nothing byte movement, and
   EFAULT mapping for recoverable syscall helper failures.
6. The proof-only talos_copy_probe x8 = 0x7001 is accepted only in its named
   QEMU/Pi 5 proof scenarios. It proves copy-in/copy-out helper plumbing
   through the syscall boundary but is not a general ABI surface.
7. talos_write x8 = 1 is accepted for fd 1 stdout and fd 2 stderr writes from
   lower-EL UserData through inherited stdio descriptors backed by
   runtime-console0, with fd 0/fd 99 -EBADF, guard-range -EFAULT, nonzero
   reserved x3 -EINVAL, scalar syscall regressions, proof-only copy-probe
   quarantine, and diagnostic-marker quarantine.

The accepted physical frontier is serialized Raspberry Pi 5 evidence for
stable scalar syscall routing, proof-only pointer-copy helper plumbing, and
descriptor-backed stdout/stderr writes for the proof-owned inherited
runtime-console0 stdio slice.

## Deferred Surfaces

The following surfaces remain blocked until later explicit tasks accept their
contracts and gates:

- stdin/read, close, dup, pipe, socket, poll/select, blocking/readiness,
  descriptor lifetime, close-on-exec, and descriptor inheritance beyond the
  proof-owned inherited stdio slice.
- process-owned descriptor tables, process-owned address spaces, PID/task
  ownership for user processes, argv/envp setup, program loading, exit/wait,
  credentials, sessions, controlling TTY, and userland scheduling semantics.
- VFS/filesystem behavior, path copying, filesystem-backed data, local shell,
  networking, SSH, and portable userland.
- partial writes, per-thread errno storage, restart semantics, signals,
  resumable lower-EL user faults, demand paging, copy-on-write, shared memory,
  mmap, and lower-EL fault-table recovery.
- RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, socket-backed
  descriptors, and non-proof runtime-console device ownership.

Milestone 7.3 therefore closes the stable syscall ABI and dispatch slice, not
the full POSIX descriptor, process, filesystem, shell, or networking contract.

## Residual Risks

- The accepted Pi 5 descriptor-write and pointer-copy hardware proofs use
  focused built-in lower-EL payloads, not loaded user programs.
- The accepted descriptor writes use proof-owned inherited runtime-console0
  descriptors, not process-owned descriptor tables or filesystem/socket
  backing objects.
- EFAULT is currently proven by pre-side-effect helper validation, not by
  resumable lower-EL data-abort recovery.
- The Pi 5 proof tasks each needed same-candidate triage after an inconclusive
  run. The retained known-good controls and unchanged reruns make the evidence
  acceptable, but future physical claims should keep the same triage rule.

## Milestone Verdict

Milestone 7.3 is closed for the bounded syscall ABI and dispatch frontier. Its
acceptance criteria are satisfied by retained QEMU/substitute evidence,
serialized Pi 5 hardware evidence, unit tests, formatting checks,
documentation checks, and whitespace checks:

- return values are exercised by talos_nop success and unknown-syscall
  -ENOSYS on QEMU and Pi 5;
- invalid calls are exercised by unknown syscall numbers, proof-only syscall
  quarantine outside named scenarios, fd0/fd99 -EBADF, and reserved-register
  -EINVAL;
- fault handling is exercised by copy helper guard-range -EFAULT at QEMU and
  Pi 5 evidence levels.

The next bounded task should be
phase7-file-descriptor-table-source-inventory-20260529, documentation-only,
under Milestone 7.4. That task should inventory process-owned descriptor-table
owners and contracts before any close/dup/read, VFS/filesystem, shell,
networking, SSH, or hardware action.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added this closeout, linked it from SUMMARY,
  updated roadmap current status and Milestone 7.3 status, updated the
  decision log, and added the task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker
  state.
