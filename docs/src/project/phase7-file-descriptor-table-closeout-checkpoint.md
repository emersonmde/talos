# Phase 7 File Descriptor Table Closeout Checkpoint

Status: accepted checkpoint for
phase7-file-descriptor-table-closeout-checkpoint-20260530.

## Scope

This checkpoint reconciles the accepted Milestone 7.4 file descriptor table
work after descriptor-backed stdio write, close, dup, and fixed-stdin read
evidence were accepted. It adds no Rust or assembly behavior, reruns no QEMU
scenario, performs no Raspberry Pi 5 hardware action, publishes no boot
archive, and acquires no hardwareTestLock.

It does not accept process loading, VFS/filesystem behavior, local shell,
networking, SSH, runtime-console0/TTY/hardware stdin, object finalization,
dup2/fcntl, signals, wait queues, nonblocking I/O, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, or a Phase 8 transition.

## Accepted Task And Evidence Map

| Task | Commit | Evidence level |
| --- | --- | --- |
| phase7-file-descriptor-table-source-inventory-20260529 | 3f8d14f334486b39b9816991f23b194dced5019b | static documentation/source inspection |
| phase7-process-descriptor-table-contract-20260529 | adc0ed9ea37fe35b0c45dd19666ba68fe8546187 | static documentation/source inspection |
| phase7-process-descriptor-table-core-20260529 | a30944d53aefd58ca89a7d197d12bae0790beb73 | fmt/unit tests/static inspection |
| phase7-qemu-process-descriptor-stdio-smoke-plan-20260529 | b314ab881f82a07da32bd1db88786a4dbf6d471e | static documentation/source inspection |
| phase7-qemu-process-descriptor-stdio-smoke-core-20260529 | fe17a6d99a634903639e5a9b8d9d5a5644822c0c | QEMU/substitute serial evidence |
| phase7-process-descriptor-table-closeout-checkpoint-20260529 | c6d80056a64f268cb39036f6af74510893850730 | static documentation inspection |
| phase7-descriptor-lifetime-close-source-inventory-20260529 | 0de2bf2be47986da3220d9fb3edea534448822b8 | static documentation/source inspection |
| phase7-descriptor-lifetime-close-contract-20260529 | 4ff46a6f68bf8349ba0b974d610a8ceb3d92ccd1 | static documentation/source inspection |
| phase7-descriptor-close-core-20260529 | 1e8cdd6fcb4bd16cbb04febd56529b66b0579182 | fmt/unit tests/static inspection |
| phase7-descriptor-close-core-closeout-checkpoint-20260529 | c537670fa9879257db403f260b4a3797f9fd829a | static documentation inspection |
| phase7-close-dup-read-syscall-source-inventory-20260529 | 8e17c1d0be80f860ef83bc02a01035dacd78d439 | static documentation/source inspection |
| phase7-close-syscall-contract-20260529 | 687ef5c04e745853230d61ef64845ec90ddb337c | static documentation/source inspection |
| phase7-close-syscall-core-20260529 | ab8915b9696a046b367830e9f5acfd632ee98788 | fmt/unit tests/static inspection |
| phase7-qemu-close-syscall-smoke-plan-20260529 | cfe3098d559ea21cd69d411f03e456064b265ee7 | static documentation/source inspection |
| phase7-qemu-close-syscall-smoke-core-20260529 | 3be4e1a76e1a065a846f1ebb226bc3e8554c2acf | QEMU/substitute serial evidence |
| phase7-close-syscall-closeout-checkpoint-20260529 | 626f688b230b20cb4a4e1b156cb8c1bb425107e1 | static documentation inspection |
| phase7-pi5-close-syscall-proof-plan-20260529 | 12426dec0d266aa578777ff32dd7e26f4e02d17b | static documentation inspection |
| phase7-pi5-close-syscall-proof-20260529 | 586041eae89944df10e0fa8f3ee7d5fda6a5f6d4 | serialized Pi 5 hardware boot/output |
| phase7-pi5-close-syscall-proof-closeout-checkpoint-20260529 | 5d009cfcbc13f300c34e20284add093d9975032b | static documentation inspection |
| phase7-dup-syscall-contract-20260529 | 041ca2f449afc9bd7889497720702b4f4f849bc3 | static documentation/source inspection |
| phase7-dup-syscall-core-20260529 | 2c30e4446f6611edb2bea1b75f226a6e919bf310 | fmt/unit tests/static inspection |
| phase7-qemu-dup-syscall-smoke-plan-20260529 | 37401fb7d9ff4924acd8a9ed072db1ec3441b261 | static documentation/source inspection |
| phase7-qemu-dup-syscall-smoke-core-20260529 | 5cce637bab95b227f5a98aba99b9104d2a017751 | QEMU/substitute serial evidence |
| phase7-dup-syscall-closeout-checkpoint-20260529 | 84e2306b7e0d1397e3ea002d71c32f6d57556595 | static documentation inspection |
| phase7-pi5-dup-syscall-proof-plan-20260529 | 332853301d62a0a6283236dcfb997b941c8e4add | static documentation inspection |
| phase7-pi5-dup-syscall-proof-20260529 | e4f52f5 | serialized Pi 5 hardware boot/output |
| phase7-pi5-dup-syscall-proof-closeout-checkpoint-20260529 | 56eb38a89cfcd81a330242c69491020532ee7169 | static documentation inspection |
| phase7-read-stdin-source-inventory-20260529 | c00267891b928e53b25c8ebdbe6a6a0dc549e0ae | static documentation/source inspection |
| phase7-read-stdin-contract-20260529 | 49d292935b4bff2220946e9eb7fe6b60de209a26 | static documentation/source inspection |
| phase7-read-stdin-core-20260529 | 613c85a1423677a764f031328530e59b3f7998ea | fmt/unit tests/static inspection |
| phase7-qemu-read-stdin-smoke-plan-20260529 | e48180bf4f61dbe1cc1294614c1acec7618fcbc9 | static documentation/source inspection |
| phase7-qemu-read-stdin-smoke-core-20260529 | cb0e816d68fa63d525c04fd6fd50ecae3d1960f8 | QEMU/substitute serial evidence |
| phase7-read-stdin-closeout-checkpoint-20260529 | 62eedfdc2b5b265f9ca400ee86b391d81fbfbee4 | static documentation inspection |
| phase7-pi5-read-stdin-proof-plan-20260530 | 4e32c29a3f739ddd45c29dc27a7dc5c0e1297dff | static documentation inspection |
| phase7-pi5-read-stdin-proof-20260530 | c76a1518e7de6d5f989f7d0bc646df5524adcd60 | serialized Pi 5 hardware boot/output |
| phase7-pi5-read-stdin-proof-closeout-checkpoint-20260530 | 7ac9d7d416f8e361fa445443fadd234e6da40b2a | static documentation inspection |

Retained evidence anchors:

- Process-owned stdio QEMU/substitute proof:
  tasks/evidence/2026-05-29-qemu-process-descriptor-stdio-smoke-core/qemu-process-descriptor-stdio-smoke.log.
- QEMU close syscall proof:
  tasks/evidence/2026-05-29-qemu-close-syscall-smoke-core/qemu-close-syscall-smoke.log.
- Pi 5 close syscall proof:
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/proof-lines.txt.
- QEMU dup syscall proof:
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.
- Pi 5 dup syscall proof:
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/proof-lines.txt.
- QEMU read/stdin proof:
  tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log.
- Pi 5 read/stdin proof:
  tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/proof-lines.txt.

The Pi 5 proof task records also retain candidate archive and kernel identity,
TFTP deltas, fresh serial cursors, known-good controls after inconclusive
runs, unchanged-candidate reruns, hardware-lock timelines, and post-restore
boot-tree evidence.

## Accepted Milestone 7.4 Frontier

Milestone 7.4 now accepts this bounded file descriptor table frontier:

1. A ProcessOwnerId-backed ProcessDescriptorStore can own an inherited stdio
   DescriptorTable with fd 0, fd 1, and fd 2 installed for the current proof
   owner.
2. Descriptor lookup through the current ProcessOwnerId routes inherited
   stdout and stderr writes to runtime-console0 on the QEMU/substitute path.
3. talos_close x8 = 2 removes current-owner descriptor slots with deterministic
   -EBADF and -EINVAL failure behavior and is proven in QEMU/substitute and
   serialized Pi 5 evidence.
4. talos_dup x8 = 3 duplicates occupied descriptors into the lowest free slot,
   returns -EMFILE for a full table, keeps source and duplicate descriptors
   independently closeable, and is proven in QEMU/substitute and serialized
   Pi 5 evidence.
5. talos_read x8 = 4 reads from fixed proof stdin through fd 0 and duplicated
   fd 3, preserves cursor/user memory on EFAULT/EINVAL/EBADF cases, returns
   zero at bounded EOF, and is proven in QEMU/substitute and serialized Pi 5
   evidence.
6. talos_nop, unknown-syscall -ENOSYS, proof-only talos_copy_probe quarantine,
   and diagnostic marker 0x7a10 quarantine remain intact across the accepted
   descriptor proof scenarios.

This satisfies the Milestone 7.4 roadmap criteria only for descriptor-backed
console streams and documented descriptor lifetime/close semantics at the
accepted proof frontier. The accepted streams are inherited stdio descriptors
and fixed proof stdin in focused built-in proof scenarios, not a general POSIX
I/O environment.

## Deferred Surfaces

The following remain blocked until later explicit tasks accept their
contracts and gates:

- runtime-console0-backed stdin, TTY raw/canonical input, hardware UART input,
  pipes, sockets, regular files, directories, device nodes, and
  filesystem-backed reads or writes;
- blocking/readiness, nonblocking flags, poll/select, wait queues,
  signal/restart behavior, Ctrl-C/Ctrl-D terminal behavior, foreground process
  groups, sessions, and terminal ownership;
- process loading, ELF parsing, argv/envp setup, PID allocation, exit/wait,
  credentials, descriptor inheritance across exec, close-on-exec, dup2/fcntl,
  open-file-description reference counting, object finalization, and object
  teardown;
- VFS/filesystem behavior, path copying, local shell, libc/Rust std stdio,
  portable userland, networking, and SSH;
- per-thread errno storage, demand paging, recoverable lower-EL data-abort
  copy tables, partial user copies on EFAULT, process-fatal user-fault policy,
  mmap, shared memory, copy-on-write, and user DMA buffers;
- RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and broader
  cache-maintenance policy.

## Residual Risks

- All accepted descriptor syscalls use focused built-in lower-EL proof payloads
  and a kernel-owned current ProcessOwnerId, not loaded user programs.
- The accepted stdin source is fixed proof input. Runtime-console0, TTY, and
  hardware input ownership remain unaccepted.
- Descriptor entries are copied for dup. Open-file-description reference
  counting and final object release remain unaccepted.
- EFAULT is proven through explicit helper validation before side effects, not
  through resumable lower-EL data-abort recovery.
- The physical proofs depended on serialized hardwareTestLock discipline and
  same-candidate triage after inconclusive runs. Future physical claims should
  preserve that evidence standard.

## Next Boundary

Milestone 7.4 is closed for the bounded descriptor-table frontier above. The
next objective task should be a supervisor-planned Phase 7 final closeout or
frontier checkpoint before any Phase 8 transition is considered.

That later checkpoint should reconcile Phase 7.1 through Phase 7.4 as one
accepted POSIX/EL0/syscall/descriptor frontier, decide whether any remaining
Phase 7 task is required, and only then recommend the first filesystem and
program-loading task. This worker is not creating that task in this checkpoint.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: reviewed the accepted process descriptor,
  close/dup/read, QEMU/substitute, Pi 5 proof, restore, and deferred-surface
  records listed above.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout checkpoint.
