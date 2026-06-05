# Phase 10 Generated Root No-Rebuild Transport Contract

Status: accepted as the documentation-only Milestone 10.3 local/QEMU
no-kernel-rebuild generated-root transport contract after the accepted
`phase10-generated-userland-executable-closeout-20260605`.

This contract adds no Rust behavior, QEMU run, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, writable filesystem, SD/USB/block
driver, networking, SSH, or phase transition. It selects the bounded transport
shape for the next task only.

## Goal

The next implementation slice should let one built Talos kernel binary consume
two different generated-root artifacts in local/QEMU evidence. The proof must
show that userland file and executable content can change without rebuilding
the kernel binary for each artifact.

Accepted by this contract:

- local/QEMU transport policy for an external generated-root artifact;
- same-kernel-binary evidence requirements for two artifact runs; and
- deterministic missing/malformed artifact behavior.

Not accepted by this contract:

- transport implementation;
- Pi 5 firmware, TFTP, or boot-archive behavior;
- writable or durable persistent storage; and
- SD/USB/block drivers, networking, SSH, or a phase transition.

## Selected Transport

The first local transport proof uses QEMU's generic loader device to place a
self-describing generated-root artifact at a fixed physical address while
loading the same Talos kernel image with `-kernel`.

The selected local boundary is:

| Field | Contract |
| --- | --- |
| kernel image | one `target/aarch64-talos-virt/<profile>/talos` ELF and one objcopied `.img` built once for the task-owned scenario |
| artifact loader | `-device loader,file=<artifact>,addr=0x47000000` on `-M virt -m 256M` |
| artifact window | physical `0x47000000..0x47400000` |
| maximum artifact length | 4 MiB including header, entries, paths, and contents |
| collision guard | implementation must prove `__kernel_end <= 0x47000000` for the accepted build before running artifact evidence |
| ownership | the kernel copies/parses the artifact into the existing immutable generated-root/VFS model before shell-visible reads or execs use it |
| fallback source | the accepted build-time generated root remains the fallback when no valid external artifact is supplied |

The QEMU run wrapper may grow an explicit optional generated-root artifact
argument. The next implementation must not encode artifact A or B into
`src/initramfs.rs`, `build.rs`, or the kernel image between runs. Evidence
must record the same kernel image hash for both artifact runs and distinct
artifact digests.

This is deliberately a local/QEMU boundary. Pi 5 boot transport remains a later
serialized hardware task because this contract does not prove that firmware,
TFTP, config.txt, or boot-archive assembly can place the same artifact at this
address.

## Artifact Format

The external artifact is a deterministic binary generated-root image, not a
shell transcript, source patch, or host-only manifest.

The first implementation should define a compact `talos-generated-root-v1`
format with:

- magic and version;
- header length and total length;
- entry count;
- deterministic artifact digest over the complete header and entry payload;
- lexical byte ordering by normalized absolute path;
- directory and regular-file entries only;
- executable flag for regular files that may enter the accepted loader path;
- byte lengths for paths and file contents; and
- entry payloads containing normalized path bytes followed by immutable file
  contents when present.

The format must retain the accepted generated-root limits unless the
implementation records a stricter bound:

- absolute, normalized, UTF-8-compatible paths only;
- no embedded NUL bytes;
- no duplicate normalized paths;
- no symlinks, devices, sockets, pipes, mounts, credentials, timestamps, or
  writable metadata;
- generated executable contents must fit `MAX_PROGRAM_IMAGE_BYTES`; and
- the total artifact must fit the 4 MiB loader window.

## Runtime Selection

The kernel-side handoff must be all-or-nothing:

- If the artifact magic, version, length, digest, ordering, path rules, entry
  limits, or executable limits fail, Talos must ignore the external artifact
  and use the build-time generated-root fallback.
- A malformed artifact must not partially merge external entries into the VFS.
- A missing artifact must use the same fallback path without treating absence
  as a panic or hardware incident.
- Evidence must report `source=external` for valid artifact runs and
  `source=compiled-fallback` plus a reason for missing or malformed runs.

The accepted VFS, descriptor, loader, startup ABI, lifecycle/status, waitpid,
laststatus, pipeline/redirection, and jobs/accounting behavior remains the
behavioral surface. The transport must only change which immutable generated
root supplies generated file and executable nodes.

## Required Evidence For The Core Task

The next implementation must retain task-owned local/QEMU evidence with at
least two valid external artifacts:

- kernel ELF hash and kernel image hash for the single built kernel;
- artifact A path, digest, source marker, generated file path/content marker,
  generated executable path, and deterministic executable status;
- artifact B path, digest, source marker, generated file path/content marker,
  generated executable path, and deterministic executable status;
- `cat` or equivalent descriptor-backed VFS read evidence proving visible
  generated file content differs between artifact A and B;
- `exec`, `waitpid`, and `laststatus` evidence proving generated
  executable behavior differs according to artifact content while the kernel
  hash is unchanged;
- retained controls for accepted descriptor-backed VFS/open/read, existing
  loader/status behavior, pipeline/redirection, and jobs/accounting behavior;
  and
- deterministic missing or malformed artifact evidence per the fallback policy.

The evidence level remains local/QEMU-substitute only. The next implementation
must not claim Pi 5 hardware behavior, boot archive publication, writable
persistence, SD/USB/block storage, networking, SSH, or phase transition.

## Findings

- fixed: Selected a concrete local/QEMU transport shape:
  `-device loader,file=<artifact>,addr=0x47000000` plus a fixed 4 MiB
  artifact window and same-kernel hash evidence. The implementation adjusted
  the original candidate address away from QEMU's observed DTB placement at
  `0x48000000` while preserving the selected loader-device boundary.
- fixed: Defined how two generated-root artifacts prove no kernel rebuild
  occurred between userland content changes.
- fixed: Named artifact identity, digest, limits, handoff ownership, and
  deterministic fallback behavior.
- removed: Treating build-time manifest generation or source-code edit
  avoidance alone as satisfying the stronger Milestone 10.3 no-rebuild
  criterion.
- deferred: Transport implementation, Pi 5 boot archive/TFTP publication,
  writable persistence, SD/USB/block drivers, networking, SSH, and phase
  transition.
- not-an-issue: No QEMU run, Pi 5 run, runtime code change, or hardware lock is
  required for this contract task.

## Recommended Next Task

The next mechanically unblocked task is
`phase10-generated-root-no-rebuild-transport-core-20260605`.

That task should implement only the local/QEMU proof described here: build one
kernel, generate two different external generated-root artifacts, run the same
kernel image against both artifacts, retain same-kernel and different-artifact
evidence, and preserve accepted VFS/user/process/pipeline/jobs controls. It
must not publish a boot archive, run Pi 5 hardware, claim durable persistence,
add storage drivers, add networking/SSH, or transition phases.

## Validation

- static inspection: reviewed `build.rs`, `linker.ld`,
  `scripts/qemu-nographic-smoke-lib.sh`, `src/initramfs.rs`,
  `src/local_command_loop.rs`, the generated-root manifest, the accepted
  generated-root image contract, the accepted generated executable closeout,
  and roadmap Milestone 10.3 notes.
- whitespace inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.
- Rust fmt/tests, QEMU runs, Pi 5 hardware runs, archive publication, and
  hardwareTestLock acquisition were not required because this task changes only
  Markdown documentation and durable worker state.
