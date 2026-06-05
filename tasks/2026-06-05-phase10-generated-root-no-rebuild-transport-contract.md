# Phase 10 Generated Root No-Rebuild Transport Contract Task

Task: phase10-generated-root-no-rebuild-transport-contract-20260605

Status: accepted

## Scope

Define the local/QEMU no-kernel-rebuild generated-root transport contract for
Milestone 10.3.

This task selects the first proof shape that can run one built Talos kernel
binary against two different generated-root artifacts. It records artifact
identity/digest requirements, limits, handoff ownership, fallback behavior,
retained controls, and deferred hardware/storage work.

Non-goals: no runtime behavior changes, no transport implementation, no QEMU
smoke requirement, no Pi 5 run, no boot archive publication, no hardware lock,
no SD/USB/block driver, no writable persistence, no networking, no SSH, and no
phase transition.

## Selected Contract

The accepted contract is
`docs/src/project/phase10-generated-root-no-rebuild-transport-contract.md`.

The selected local proof shape is:

- build one kernel ELF/image for the task-owned scenario;
- use QEMU `-device loader,file=<artifact>,addr=0x48000000` to place an
  external generated-root artifact in the `0x48000000..0x48400000` physical
  window;
- require a `__kernel_end <= 0x48000000` collision guard before evidence;
- parse/copy a self-describing deterministic generated-root v1 artifact into
  the existing immutable generated-root/VFS model;
- prove artifact A and artifact B have different digests and visible generated
  file/executable behavior while the kernel image hash is unchanged; and
- fall back to the build-time generated root for missing or malformed external
  artifacts without partial VFS merges.

This is local/QEMU-substitute policy only. Pi 5 firmware/TFTP placement,
boot-archive assembly, writable persistence, and block storage remain deferred.

## Static Source And Script Review

- `build.rs` currently consumes `userland/generated-root.manifest` at build
  time and emits generated file/executable constants into OUT_DIR. This is the
  accepted source-code edit avoidance boundary but not the no-rebuild
  transport.
- `src/initramfs.rs` currently includes the generated constants and exposes
  the generated regular file and executable through the existing read-only VFS
  nodes.
- `scripts/qemu-nographic-smoke-lib.sh` currently runs QEMU with `-kernel`
  only. The next core task may add an optional generated-root artifact loader
  argument to this wrapper.
- `linker.ld` loads the QEMU kernel at `0x40200000` and defines
  `__kernel_end`; the selected `0x48000000` artifact window is outside the
  kernel load base and must be guarded against the final linked kernel size
  before evidence.
- `docs/src/project/phase10-generated-userland-image-contract.md` and
  `tasks/2026-06-05-phase10-generated-userland-executable-closeout.md`
  require the next no-rebuild step to avoid overstating Pi 5, persistence, or
  boot archive behavior.

## Findings

- fixed: Selected QEMU's generic loader device at `0x48000000` as the first
  local no-rebuild transport proof shape.
- fixed: Required same-kernel ELF/image hashes plus two distinct artifact
  digests and visible generated file/executable behavior changes.
- fixed: Defined artifact identity, digest, bounds, handoff ownership, and
  missing/malformed fallback behavior.
- fixed: Updated the roadmap and project docs to name the selected local
  no-rebuild transport boundary.
- removed: Treating the existing build-time generated-root manifest as proof
  that kernel rebuilds are avoided.
- deferred: Transport implementation, QEMU proof runs, Pi 5 boot archive/TFTP
  publication, writable persistence, SD/USB/block drivers, networking, SSH,
  and phase transition.
- not-an-issue: hardwareTestLock stayed unlocked because this task is static
  documentation and source/script inspection only.

## Result

Accepted as the local/QEMU no-kernel-rebuild generated-root transport contract
for Milestone 10.3.

The next queued task,
`phase10-generated-root-no-rebuild-transport-core-20260605`, is mechanically
unblocked for a later worker wake. That task may implement only the local/QEMU
same-kernel/two-artifact proof described by the contract. It must not publish a
boot archive, run Pi 5 hardware, claim writable persistence, add storage
drivers, add networking/SSH, or transition phases.

## Validation

- static inspection: reviewed `build.rs`, `linker.ld`,
  `scripts/qemu-nographic-smoke-lib.sh`, `src/initramfs.rs`,
  `src/local_command_loop.rs`, `userland/generated-root.manifest`,
  `docs/src/project/phase10-generated-userland-image-contract.md`,
  `tasks/2026-06-05-phase10-generated-userland-executable-closeout.md`, and
  roadmap Milestone 10.3 notes.
- diff hygiene: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.
