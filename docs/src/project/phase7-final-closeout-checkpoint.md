# Phase 7 Final Closeout Checkpoint

Status: accepted checkpoint for phase7-final-closeout-checkpoint-20260530.

## Scope

This documentation-only checkpoint closes Phase 7 for the bounded POSIX,
EL0, syscall, copy-helper, and file-descriptor frontier recorded by the
accepted final-frontier inventory. It reconciles Phase 7.1 through Phase 7.4
accepted work by commit, evidence level, deferred surface, and residual risk.

It adds no Rust or assembly behavior, reruns no QEMU scenario, performs no
Raspberry Pi 5 hardware action, publishes no boot archive, and acquires no
hardwareTestLock. It does not implement or accept filesystem/program loading,
VFS behavior, local shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, runtime-console0/TTY/hardware stdin, or
full POSIX readiness.

## Accepted Phase 7 Evidence Map

| Slice | Accepted checkpoint | Commit | Evidence level |
| --- | --- | --- | --- |
| Phase 7.1 POSIX baseline | phase7-posix-baseline-closeout-checkpoint-20260528 | b6480823a6de9900f281e25ae8c201f49305666d | static documentation/source inspection; target-independent fmt/unit tests |
| Phase 7.2 EL0/address space | phase7-el0-trap-proof-closeout-checkpoint-20260529 | ccc9c8ba2fcc8cc441c5c2b38d85bf0a87e278f8 | QEMU/substitute serial evidence; serialized Pi 5 hardware boot/output; static documentation/source inspection |
| Phase 7.3 syscall ABI/dispatch and copy boundary | phase7-syscall-abi-dispatch-closeout-checkpoint-20260529 | d2e26c385880e82fc15d40682ab5fc25e05d2994 | target-independent unit tests; QEMU/substitute serial evidence; serialized Pi 5 hardware boot/output; static documentation/source inspection |
| Phase 7.4 file descriptor table | phase7-file-descriptor-table-closeout-checkpoint-20260530 | cf748c75ea84aec1975436a0a50d377fee4a9fbf | target-independent unit tests; QEMU/substitute serial evidence; serialized Pi 5 hardware boot/output; static documentation/evidence inspection |
| Phase 7 final frontier inventory | phase7-final-frontier-source-inventory-20260530 | 78b2db740e0965710f31f8182d917c4c3ffe56ae | static documentation/evidence inspection |

Retained evidence anchors remain in the accepted slice closeouts and task
records. The final-frontier inventory summarizes the QEMU/substitute and Pi 5
anchors, including:

- lower-EL trap proof:
  tasks/evidence/2026-05-28-pi5-el0-trap-proof/local62-clean-final-lower-el0-trap/proof-lines.txt;
- syscall routing proof:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log
  and
  tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-proof-lines.txt;
- pointer-copy and descriptor-write proofs:
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log,
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-proof-lines.txt,
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log,
  and
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt;
- file-descriptor proofs:
  tasks/evidence/2026-05-29-qemu-process-descriptor-stdio-smoke-core/qemu-process-descriptor-stdio-smoke.log,
  tasks/evidence/2026-05-29-qemu-close-syscall-smoke-core/qemu-close-syscall-smoke.log,
  tasks/evidence/2026-05-29-pi5-close-syscall-proof/local19-store-clean-candidate/proof-lines.txt,
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log,
  tasks/evidence/2026-05-29-pi5-dup-syscall-proof/local8-unchanged-candidate-rerun/proof-lines.txt,
  tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log,
  and
  tasks/evidence/2026-05-30-pi5-read-stdin-proof/local5-candidate-tftp-wait-rerun/proof-lines.txt.

The Pi 5 proof records also retain candidate identity, archive and kernel
digests, TFTP deltas, fresh serial cursors, known-good controls after
inconclusive runs, unchanged-candidate reruns, hardware-lock timelines, and
post-restore boot-tree proof.

## Closed Phase 7 Frontier

Phase 7 is closed for this bounded frontier:

1. Target-independent POSIX error names, lexical path parsing, descriptor-table
   vocabulary, and first descriptor contracts exist for later filesystem and
   process-loading work.
2. A built-in lower-EL payload can enter lower AArch64/EL0, trap through SVC,
   return to the kernel, and report QEMU/substitute and Pi 5 PASS evidence.
3. Stable svc #0 dispatch accepts x8 syscall numbers, x0-through-x5 scalar
   arguments, negative-errno returns, talos_nop, unknown-syscall -ENOSYS, and
   proof-only diagnostic quarantine.
4. Whole-range copy_from_user and copy_to_user validate before side effects
   and map recoverable helper failures to EFAULT for the accepted syscall
   helpers.
5. Descriptor-backed stdout/stderr writes through inherited runtime-console0
   stdio are proven in QEMU/substitute and serialized Pi 5 evidence.
6. ProcessOwnerId-backed inherited stdio, process descriptor lookup, close,
   dup, and fixed-proof-stdin read through fd 0/fd 3 are accepted for the
   focused target-independent, QEMU/substitute, and Pi 5 proof frontiers.

No remaining bounded Phase 7 implementation or evidence task is required
before entering Phase 8 source-inventory planning.

## Deferred Surfaces

The following remain outside the accepted Phase 7 frontier:

- runtime-console0-backed stdin, TTY raw/canonical input, hardware UART input,
  pipes, sockets, regular files, directories, device nodes, filesystem-backed
  reads/writes, and VFS/path lookup;
- process loading, ELF parsing, argv/envp setup, PID allocation, exit/wait,
  credentials, sessions, controlling TTY, descriptor inheritance across exec,
  close-on-exec, dup2/fcntl, open-file-description reference counting, object
  finalization, and object teardown;
- local shell, libc/Rust std stdio, portable userland, filesystem/program
  loading implementation, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy;
- blocking/readiness, nonblocking flags, poll/select, wait queues,
  signal/restart behavior, per-thread errno storage, demand paging,
  resumable lower-EL data-abort copy recovery, mmap, shared memory,
  copy-on-write, and user DMA buffers.

## Residual Risks

- The accepted lower-EL and descriptor evidence uses focused built-in proof
  payloads, not loaded user programs.
- The accepted stdin source is fixed proof data, not runtime-console0, TTY, or
  hardware input.
- Descriptor objects are sufficient for inherited stdio and focused
  close/dup/read behavior, but open-file-description final release remains
  unaccepted.
- EFAULT evidence is based on explicit helper validation before side effects,
  not resumable lower-EL data-abort recovery.
- Future physical claims should preserve the serialized hardwareTestLock,
  candidate/archive/TFTP identity, known-good control, unchanged rerun, and
  restore-proof standards used by accepted Phase 7 Pi 5 evidence.

## Verdict

Phase 7 is closed for the accepted bounded frontier. The next mechanically
derivable task is
phase8-filesystem-program-loading-source-inventory-20260530, but only after
durable state records phaseCheckpointStatus.phase7FinalCloseoutRecommendsPhase8
as true and the task remains explicitly queued/blocked on that flag.

This verdict is a recommendation to begin Phase 8 source inventory. It is not
a Phase 8 implementation, filesystem/program-loading acceptance, shell claim,
networking claim, SSH claim, or hardware claim.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: reviewed the accepted final-frontier inventory,
  Phase 7.1 through Phase 7.4 closeouts, retained QEMU/substitute evidence,
  retained Pi 5 evidence, hardware-lock/restore summaries, deferred surfaces,
  and residual-risk lists.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout checkpoint.
