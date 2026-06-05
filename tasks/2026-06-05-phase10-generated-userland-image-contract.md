# Phase 10 Generated Userland Image Contract Task

Task: phase10-generated-userland-image-contract-20260605

Status: accepted

## Scope

Documentation-only contract after the accepted local storage path evaluation
selected generated userland/initramfs manifest ingestion as the first
Milestone 10.3 slice.

The task defines manifest/root input shape, deterministic ordering,
generated-root identity, path normalization, file-kind and size limits,
read-only VFS integration, optional executable policy, evidence identities,
retained regression gates, deferred claims, and the next bounded
manifest-core implementation task.

Non-goals: no implementation, QEMU run, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, writable/persistent filesystem,
SD/USB/block driver, directory mutation, networking, SSH, process/descriptor
behavior change, or phase transition.

## Evidence

- static source/doc/script review: reviewed `src/initramfs.rs` hardcoded
  immutable node/content model; `src/local_command_loop.rs` descriptor-backed
  cat/exec, fixed-/bin lookup, process-control, redirection, pipeline, and jobs
  controls; `src/program_loader.rs` VFS-backed loader input and
  `MAX_PROGRAM_IMAGE_BYTES`; `src/syscall.rs` `TalosOpen`/`TalosRead`;
  `src/posix.rs` `normalize_path()` and default path limits; `build.rs`;
  QEMU local command-loop scripts; and accepted Phase 8/10 task records.
- documentation diff: added
  `docs/src/project/phase10-generated-userland-image-contract.md`, linked it
  from `docs/src/SUMMARY.md`, updated the Milestone 10.3 roadmap note, added
  this task record, and retained a static contract review note under task
  evidence.
- contract decision: the manifest-core task must first prove source-code edit
  avoidance for userland content changes by consuming manifest/root-defined
  read-only content through the existing descriptor-backed VFS model. It must
  not claim kernel binary rebuild avoidance, boot archive update, Pi 5
  behavior, or true persistence.
- next task: `phase10-generated-userland-image-manifest-core-20260605` is
  mechanically unblocked after this contract is accepted and committed.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

## Findings

- fixed: Defined the generated-root input contract, deterministic ordering,
  identity/digest requirements, path limits, file kinds, size limits, and
  read-only VFS integration target.
- fixed: Required task-owned QEMU/substitute evidence for generated file
  consumption through `TalosOpen`/`TalosRead` and retained shell/VFS/process
  controls.
- fixed: Preserved exact claim boundaries: source-code edit avoidance only in
  the next slice; no kernel-rebuild avoidance, boot archive update, hardware
  behavior, or true persistence claim.
- fixed: Made generated executable support conditional on preserving the
  accepted VFS/open/read loader and userspace lifecycle path.
- removed: Another hardcoded `src/initramfs.rs` file-content constant as
  acceptable Milestone 10.3 progress.
- deferred: TFTP/no-rebuild transport, Pi 5 boot archive publication,
  writable persistent storage, SD/USB/block drivers, symlinks, device nodes,
  networking, SSH, and phase transition.
- not-an-issue: No runtime code, QEMU run, Pi 5 run, or hardware lock was
  required by this contract task.

## Result

Accepted as the Milestone 10.3 generated userland/initramfs image contract.
The next task may implement only the smallest generated manifest/root
ingestion path described here and must keep every stronger storage, transport,
hardware, networking, and phase-transition claim deferred until explicitly
planned.
