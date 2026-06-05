# Phase 10 Local Storage Path Evaluation Checkpoint

Task: phase10-local-storage-path-evaluation-checkpoint-20260605
Status: accepted

## Scope

Evaluate the first Milestone 10.3 storage/userland-image path for the smallest useful capability: changing or expanding userland content without editing kernel source for every file or program.

This checkpoint does not implement runtime behavior. It compares SD, USB mass storage, generated image roots, TFTP-loaded initramfs expansion, and the existing compiled fixture model against the accepted VFS/loader ownership, QEMU/local evidence path, Pi 5 lab constraints, and roadmap risk.

## Recommendation

Primary next slice: generated userland/initramfs manifest ingestion into the existing read-only VFS model.

The first implementation should consume a host-side manifest or root tree and make at least one generated file, and optionally one generated executable, visible through the accepted descriptor-backed VFS/open/read and exec paths without adding a new hardcoded file-content constant to `src/initramfs.rs`. Evidence must report a deterministic generated-root identity.

Fallback path: TFTP-loaded generated initramfs transport after the local manifest/root ingestion contract is accepted and implemented.

This ordering deliberately separates four claims:

- source-code edit avoidance: prove this first by adding or changing userland content through a manifest/root input rather than editing `src/initramfs.rs`;
- kernel binary rebuild avoidance: deferred until a later transport boundary exists;
- boot archive update: deferred to a later Pi 5/TFTP transport task;
- true persistence: deferred because generated read-only image content is not writable persistent storage.

## Evaluation Matrix

| Path | Fit now | Reason |
| --- | --- | --- |
| Generated userland/initramfs manifest | primary | Reuses the accepted read-only VFS, descriptor open/read, loader, shell cat/exec, QEMU/local smoke scripts, and no-hardware evidence path while removing source edits for every file/program change. |
| TFTP-loaded initramfs expansion | fallback | Useful for later boot-archive and no-kernel-rebuild claims, but it crosses Pi firmware/lab boot archive policy and should wait until the generated image contract is stable. |
| Existing compiled fixture model | removed as progress path | It is the accepted baseline but still requires source edits in `src/initramfs.rs` for every new userland file/program. |
| SD card storage | deferred | Requires block-device ownership, filesystem parsing, hardware policy, and persistence semantics before it can be a thin Milestone 10.3 slice. |
| USB mass storage | deferred | Adds USB/xHCI/RP1/PCIe/DMA/cache and block stack risk before Talos has a generated local userland image boundary. |

## Findings

- fixed: Selected exactly one next Milestone 10.3 implementation slice: generated userland/initramfs manifest ingestion into the read-only VFS model.
- fixed: Defined the first useful user-visible proof as descriptor-backed `cat` and, if the contract includes it, VFS-backed `exec` of manifest/root content without adding a hardcoded `src/initramfs.rs` file-content constant.
- fixed: Separated source-code edit avoidance from kernel binary rebuild avoidance, boot archive update, and true persistence.
- fixed: Identified TFTP-loaded initramfs transport as the fallback after the local manifest/root contract, not as the first implementation slice.
- removed: Treating the current compiled fixture table as Milestone 10.3 progress; it remains a regression/control surface only.
- deferred: SD storage, USB mass storage, writable persistent filesystem mutation, block-device/FAT/ext parsing, Pi 5 hardware proof, boot archive publication, no-kernel-rebuild transport, networking, SSH, and Phase 11.
- not-an-issue: No runtime code change, QEMU run, Pi 5 run, or hardware lock was required by this evaluation checkpoint.

## Static Review Notes

- `src/initramfs.rs` currently owns immutable static nodes and hardcoded file bytes for `/etc/banner.txt`, `/bin/init`, `/bin/zero`, `/bin/status42`, `/bin/stdout`, `/bin/stdin`, and `/bin/stderr`; this is the source edit pressure Milestone 10.3 should reduce first.
- `src/local_command_loop.rs` already consumes VFS paths for descriptor-backed `cat`, fixed `/bin` exec lookup, redirection, volatile `/tmp` sinks, pipeline composition, and jobs controls.
- `src/program_loader.rs` and accepted Phase 8 task records prove that loader input already comes from the VFS regular-file boundary.
- `scripts/qemu-local-serial-command-loop-smoke.sh` has the existing local command-loop harness for task-owned QEMU/substitute evidence once the generated-image implementation exists.
- `docs/src/project/lab-controller.md` makes Pi 5 boot archive publication and TFTP proof a separate hardware/lab action.

## Evidence Boundary For Next Slice

The generated-userland contract task becomes mechanically unblocked because this checkpoint selects generated userland/initramfs image expansion as the primary path.

The contract should require manifest/root input shape, deterministic ordering, path normalization, file-kind and size limits, generated-root identity and digest reporting, proof that generated content reaches the existing read-only VFS/open/read model rather than a new `src/initramfs.rs` constant, QEMU/local descriptor-backed `cat` evidence, optional VFS-backed `exec` evidence, and retained controls for accepted Milestone 10.1/10.2 behavior.

## Validation

- static inspection: reviewed `src/initramfs.rs`, `src/local_command_loop.rs`, `src/program_loader.rs`, `build.rs`, QEMU local command-loop scripts, `docs/src/project/lab-controller.md`, accepted Phase 8 task records, and the accepted Milestone 10.2 closeout.
- static inspection: recorded the evaluation matrix and selected primary and fallback paths in this checkpoint.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final checkpoint commit recorded in supervisor state.
