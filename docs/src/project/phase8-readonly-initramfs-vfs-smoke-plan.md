# Phase 8 Read-Only Initramfs/VFS Smoke Plan

Status: accepted as the documentation-only Milestone 8.1 read-only
initramfs/VFS QEMU/substitute smoke plan after the accepted
[Phase 8 Read-Only Initramfs/VFS Contract](phase8-readonly-initramfs-vfs-contract.md).
This plan adds no Rust behavior, assembly behavior, QEMU run, Pi 5 hardware
run, boot archive publication, hardware-lock acquisition, VFS implementation,
initramfs parser, descriptor-backed filesystem read, ELF/program loader,
argv/envp setup, process creation, shell behavior, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

The purpose of this plan is to make the next implementation tasks mechanical:
first add the target-independent read-only initramfs/VFS object core, then add
one QEMU/substitute smoke that proves the accepted fixture, lookup, read,
offset, EOF, and deterministic errno boundary.

## Smoke Invariant

The first QEMU/substitute read-only initramfs/VFS smoke must demonstrate one
bounded invariant:

1. Talos builds a QEMU-only or host-substitute scenario named
   qemu_readonly_initramfs_vfs_smoke.
2. The scenario constructs the accepted deterministic fixture from immutable
   in-kernel or test-owned bytes. It must not consume TFTP, firmware-provided
   archives, writable storage, block devices, or host filesystem state.
3. The fixture identity line records the stable fixture name and a deterministic
   digest over the byte contents and directory topology used by the run.
4. The root directory contains exactly these contract-required nodes:
   - /etc/banner.txt: a regular file containing the ASCII bytes
     "Talos initramfs fixture\\n".
   - /bin/init: a regular file containing the ASCII bytes
     "not-executable-yet\\n".
   - /empty: a zero-length regular file.
   - /dir/nested.txt: a regular file containing the ASCII bytes
     "nested fixture\\n".
5. Lookup of /, /etc/banner.txt, /bin/init, /empty, and /dir/nested.txt proves
   root, directory, regular-file, length, and multi-component traversal.
6. Descriptor-facing read of /etc/banner.txt through a fixture open-file
   description proves byte copying, offset advancement, short EOF read, and
   stable metadata. The first read must return the full 24-byte banner and set
   the offset to 24. A second read from the same open-file description must
   return 0 and leave the offset at 24.
7. Descriptor-facing read of /empty returns 0 and leaves the offset at 0.
8. Negative cases prove the accepted error precedence for ENOENT, ENOTDIR,
   EISDIR, ENAMETOOLONG, EBADF, EFAULT, EINVAL, and ENOTSUP without mutating
   file offsets or descriptor state.
9. The smoke prints final classification and PASS only after all success,
   offset/EOF, and negative errno observations have been recorded.

The fixture digest algorithm should be simple and deterministic. The
implementation task may use a stable textual manifest hash or a byte-level
fixture hash, but it must print the algorithm name and digest in the retained
log. If implementation work needs different fixture paths, file bytes, error
cases, scenario name, or required output fields, it must stop for supervisor
planning instead of accepting a changed smoke.

## Required Output

The implementation script must retain the serial or substitute log at:

    tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core/qemu-readonly-initramfs-vfs-smoke.log

The script must grep these exact PASS/classification lines:

    qemu-readonly-initramfs-vfs-smoke: final participants=8 expected=8 errors=0 classification=qemu-readonly-initramfs-vfs-smoke-complete
    qemu-readonly-initramfs-vfs-smoke: PASS

The retained log must also include these exact field names and stable values:

    qemu-readonly-initramfs-vfs-smoke: start
    qemu-readonly-initramfs-vfs-smoke: fixture name=phase8-readonly-initramfs-vfs-v1 digest-algorithm=stable-manifest digest=0x<hex>
    qemu-readonly-initramfs-vfs-smoke: lookup path=/ kind=directory entries=4 ok=true
    qemu-readonly-initramfs-vfs-smoke: lookup path=/etc/banner.txt kind=regular length=24 ok=true
    qemu-readonly-initramfs-vfs-smoke: read path=/etc/banner.txt offset-before=0 request=64 result=24 offset-after=24 data="Talos initramfs fixture\\n" ok=true
    qemu-readonly-initramfs-vfs-smoke: read path=/etc/banner.txt offset-before=24 request=64 result=0 offset-after=24 eof=true ok=true
    qemu-readonly-initramfs-vfs-smoke: read path=/empty offset-before=0 request=64 result=0 offset-after=0 eof=true ok=true
    qemu-readonly-initramfs-vfs-smoke: lookup path=/dir/nested.txt kind=regular length=15 ok=true
    qemu-readonly-initramfs-vfs-smoke: error case=missing path=/missing errno=-ENOENT ok=true
    qemu-readonly-initramfs-vfs-smoke: error case=not-directory path=/etc/banner.txt/child errno=-ENOTDIR ok=true
    qemu-readonly-initramfs-vfs-smoke: error case=is-directory path=/etc errno=-EISDIR ok=true
    qemu-readonly-initramfs-vfs-smoke: error case=name-too-long errno=-ENAMETOOLONG ok=true
    qemu-readonly-initramfs-vfs-smoke: error case=bad-descriptor errno=-EBADF offset-unchanged=true ok=true
    qemu-readonly-initramfs-vfs-smoke: error case=user-fault errno=-EFAULT offset-unchanged=true ok=true
    qemu-readonly-initramfs-vfs-smoke: error case=invalid-input errno=-EINVAL offset-unchanged=true ok=true
    qemu-readonly-initramfs-vfs-smoke: error case=unsupported-operation errno=-ENOTSUP ok=true

The implementation may print additional source-owner, fixture-node, path-limit,
or descriptor-state fields, but these required lines must stay stable enough
for the script gate. The digest value is intentionally specified as a field,
not as a fixed value in this plan, because the implementation task owns the
exact stable digest algorithm.

## Failure Classification

The smoke must distinguish target-independent contract failures from scenario
wiring failures:

- Contract failure: the VFS core returns the wrong node kind, length, bytes,
  offset, EOF result, errno, or mutation behavior for a fixture and request
  that the scenario successfully constructed.
- Scenario wiring failure: the scenario cannot select
  qemu_readonly_initramfs_vfs_smoke, cannot construct the deterministic fixture,
  cannot retain a fresh log, cannot print the fixture identity line, or cannot
  drive the planned observations in order.
- Regression failure: an accepted scalar syscall, descriptor-write, close, dup,
  or read/stdin gate required by this plan fails after implementation changes.

QEMU capture failures are not Pi 5 hardware blockers. If the smoke cannot
classify the run, keep hardwareTestLock untouched and triage only local
staging facts in this order:

1. Confirm the built kernel or substitute binary selected
   qemu_readonly_initramfs_vfs_smoke.
2. Confirm the smoke script captured a fresh retained log path.
3. Confirm the log contains qemu-readonly-initramfs-vfs-smoke: start before
   looking for PASS.
4. Confirm the fixture identity line appears before lookup/read observations.
5. Confirm the success lookup/read lines appear before the negative errno
   lines.
6. Confirm the bad-descriptor, user-fault, and invalid-input cases report
   offset-unchanged=true where required.
7. Compare the generated kernel or substitute artifact path and timestamp
   against the build command.
8. Rerun the smoke script once after cleaning only stale QEMU/substitute output
   artifacts.

The Pi 5 inconclusive-run triage sequence does not apply until a later
serialized hardware task explicitly acquires hardwareTestLock.

## Regression Gates

The implementation task must retain:

- The QEMU/substitute read-only initramfs/VFS smoke log named above.
- The command used to build and run qemu_readonly_initramfs_vfs_smoke.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- scripts/qemu-readonly-initramfs-vfs-smoke.sh or an accepted replacement
  script that retains the required log and greps the required lines.
- Existing scalar, descriptor-write, close, dup, and read/stdin QEMU/substitute
  smoke scripts only if the implementation touches shared syscall dispatch,
  descriptor table, read, user-copy, lower-EL routing, boot-scenario routing,
  or diagnostic output owners used by those smokes.
- git diff --check.
- mdbook build when docs are touched.

The evidence level is QEMU/substitute only. It must explicitly state that no
Pi 5 hardware behavior, boot archive publication, firmware initramfs delivery,
TFTP archive identity, process loading, executable /bin/init behavior, shell,
networking, or SSH support is claimed.

## Source Owners For Later Implementation

The later target-independent core task may touch only these owners unless it
records a narrow reason:

- A small VFS/initramfs module for immutable fixture nodes, metadata, lookup,
  open-file descriptions, file offsets, and regular-file reads.
- src/posix.rs only for using the accepted PosixError, normalize_path(), and
  user-copy helpers, or for narrowly exposed helper APIs needed by the
  accepted contract.
- Descriptor-table code only if needed to represent a fixture regular-file
  open-file description without broadening descriptor syscall behavior.
- Focused unit tests for the accepted fixture, success cases, error
  precedence, offset/EOF behavior, and no-mutation failures.
- Documentation and the task record needed to report evidence.

The later QEMU/substitute smoke task may also touch:

- build.rs and src/main.rs for boot-scenario routing.
- src/target/qemu_virt.rs for scenario orchestration and required output.
- scripts/qemu-readonly-initramfs-vfs-smoke.sh for retained evidence.

Existing ELF/program-loader, process table, scheduler policy, runtime
console/TTY, Pi 5, RP1/PCIe, UART interrupt, DMA/cache-driver, network, and
shell owners remain out of scope for this smoke frontier.

## Deferred Surfaces

This plan keeps these surfaces blocked:

- Pi 5 hardware proof, archive publishing, power-cycle, serial observe, TFTP
  fixture delivery, and hardware-lock acquisition.
- ELF/program loading, executable /bin/init behavior, segment validation,
  zero-fill, entry-state setup, argv/envp, process creation, exec/spawn/wait,
  process-owned address-space installation, and process-fatal loader errors.
- Open syscall ABI, directory iteration, readdir/getdents, seek syscalls,
  object final release, descriptor inheritance across exec, close-on-exec,
  dup2/fcntl, signals, wait queues, nonblocking I/O, poll/select readiness,
  mutable current working directory, chdir, per-process root, and mount
  namespaces.
- Writable filesystems, persistent storage, block devices, symlinks, device
  nodes, pipes, sockets, mmap, demand paging, copy-on-write, shared memory,
  user DMA buffers, libc/Rust std filesystem support, shell behavior,
  networking, and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Next Mechanically Derivable Task

The next target-independent implementation task should be
phase8-readonly-initramfs-vfs-core-20260530. Its goal should be to implement
only the accepted read-only initramfs/VFS object model, deterministic fixture,
lookup, regular-file open-file-description reads, offset/EOF behavior, and
focused unit tests needed by the contract and this smoke plan.

After that core is accepted, the next QEMU/substitute evidence task should be
phase8-qemu-readonly-initramfs-vfs-smoke-core-20260530. It should add only the
qemu_readonly_initramfs_vfs_smoke scenario or substitute script, required
PASS/classification output, retained log, and regression gates described here.

Neither task may add Pi 5 hardware proof, boot archive publication,
ELF/program loading, process creation, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: added this smoke plan, linked it from SUMMARY,
  updated roadmap Phase 8 status, updated the decision log, and added the task
  record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, Pi 5 hardware runs, archive publication, and
  hardwareTestLock acquisition were not required because this task changes only
  Markdown documentation and durable worker state.
