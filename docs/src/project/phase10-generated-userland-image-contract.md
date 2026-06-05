# Phase 10 Generated Userland Image Contract

Status: accepted as the documentation-only Milestone 10.3 generated
userland/initramfs image contract after the accepted
`phase10-local-storage-path-evaluation-checkpoint-20260605`.

This contract adds no Rust behavior, generated image tooling, build-script
behavior, QEMU run, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, writable filesystem, SD/USB/block driver,
networking, SSH, or phase transition. It defines the bounded implementation
policy for the next task.

## Goal

The next implementation slice should let Talos consume a host-defined
read-only userland root without adding a new hardcoded file-content constant
to `src/initramfs.rs` for every user-visible file or program change.

The accepted claim is narrow:

- accepted first: source-code edit avoidance for userland content changes;
- not accepted yet: avoiding a kernel binary rebuild;
- not accepted yet: publishing or updating a Pi 5 boot archive;
- not accepted yet: writable or durable persistent storage.

The generated image remains an immutable initramfs-style root once the kernel
consumes it. It must preserve the accepted descriptor-backed VFS/open/read,
loader, exec, status/wait, descriptor, pipeline, redirection, and jobs
frontiers.

## Input Shape

The first generated-root input is a host-side manifest or root tree that is
normalized into a deterministic image model before the kernel consumes it.
The implementation task may choose manifest text, a root directory, or a small
checked-in fixture wrapper only if the chosen form satisfies this contract
without choosing policy mid-task.

Required image fields:

| Field | Contract |
| --- | --- |
| identity | stable generated-root label plus deterministic digest printed in task evidence |
| ordering | lexical byte ordering by normalized absolute path before image construction |
| root | exactly one immutable root directory addressed as `/` |
| path | absolute UTF-8-compatible byte path, normalized through the accepted path rules |
| kind | directory or regular file only in the first slice |
| file contents | immutable bytes; zero-length files are allowed |
| executability | regular files are loader candidates only when the implementation explicitly marks them executable or places them in the accepted executable set |
| metadata | node id, kind, byte length, read-only state, and generated-source identity |

Directory entries must be unique after normalization. Parent directories may be
generated from file paths if the implementation records that policy and the
result remains deterministic.

## Path And Size Limits

Generated paths must use the accepted `normalize_path()` behavior and the
default path limits unless the implementation task records a stricter limit.
The first slice must reject:

- empty paths;
- relative paths in the generated-root input;
- embedded NUL bytes;
- `.` or `..` paths that normalize outside the generated root;
- duplicate normalized paths;
- files whose byte length exceeds the implementation's fixed generated-file
  limit; and
- total image node or byte counts that exceed the implementation's fixed
  generated-image limits.

The first executable proof must also stay within
`MAX_PROGRAM_IMAGE_BYTES`. Larger program images remain deferred.

## VFS Integration

The generated-root implementation should feed the existing read-only VFS model
rather than adding another shell-only lookup table.

Required integration behavior:

- descriptor-backed `TalosOpen` and `TalosRead` can open and read a
  generated regular file;
- lookup, open, read, EOF, directory-as-regular-file, missing path, malformed
  path, and unsupported mutation errors remain deterministic;
- accepted compiled fixtures remain regression controls unless the
  implementation deliberately migrates them behind the generated-root model;
- generated-root entries are immutable after construction; and
- writable files, directory mutation, symlinks, device nodes, sockets, pipes,
  mount points, permissions beyond read-only/executable classification,
  timestamps, credentials, and persistence metadata remain deferred.

The first implementation may keep the generated image compiled into the kernel
binary. That is enough to prove source-code edit avoidance only if the new
file or executable is defined by the manifest/root input and not by a new
`src/initramfs.rs` content constant.

## Executable Policy

The manifest-core task must include an executable only if it can do so without
broadening loader or process policy. If included, the executable must:

- be a regular generated-root file;
- be read through the accepted descriptor-backed VFS/open/read path;
- be parsed by the accepted program loader;
- launch through the accepted userspace/process lifecycle path;
- report deterministic argv/envp/status evidence; and
- preserve fixed `/bin` lookup and absolute-path exec controls.

If the first implementation includes only a generated regular file, the task
must retain VFS exec/open/read regressions against existing accepted
executables and record the executable-generated-root proof as deferred.

## Evidence Contract

The manifest-core task must retain a task-owned QEMU/substitute log that
prints:

- generated-root identity label;
- generated-root digest;
- generated-root source path or manifest identifier;
- proof that a generated file is read through descriptor-backed `TalosOpen`
  and `TalosRead`;
- proof that the generated file was not added as a new hardcoded
  `src/initramfs.rs` file-content constant;
- if executable proof is included, VFS/open/read, loader, startup ABI,
  lifecycle/status, `laststatus`, and `waitpid` lines for the generated
  executable; and
- PASS/classification lines for the generated-root scenario.

Retained controls must cover:

- descriptor-backed `cat /etc/banner.txt` or an equivalent accepted VFS read
  control;
- accepted VFS exec/open/read and loader behavior for existing executable
  controls;
- status, `laststatus`, and `waitpid` behavior;
- standard descriptor inheritance and loader temporary descriptor non-leak;
- pipeline/redirection and jobs/accounting behavior from Milestone 10.2; and
- deterministic negative cases for missing, directory, malformed, and
  non-executable paths.

Evidence must label its level precisely as QEMU/substitute, static
inspection, docs validation, or another accepted level. It must not claim Pi 5
hardware behavior, no-rebuild transport, boot archive publication, or true
persistence.

## Findings

- fixed: The next implementation has a concrete policy target: generated
  read-only userland content enters the existing VFS model and is observed
  through descriptor-backed shell/VFS behavior.
- fixed: The accepted first claim is source-code edit avoidance for userland
  file/program content changes; kernel rebuild avoidance, boot archive update,
  and true persistence are explicitly separate.
- fixed: Deterministic generated-root identity and digest reporting are
  required evidence, not optional diagnostics.
- fixed: The contract allows the manifest-core task to include a generated
  executable only if it stays within the accepted VFS/loader/userspace
  lifecycle path.
- removed: Treating another hardcoded `src/initramfs.rs` content constant as
  Milestone 10.3 progress.
- deferred: TFTP-loaded initramfs transport, no-kernel-rebuild transport, Pi 5
  boot archive publication, writable persistent filesystem mutation,
  SD/USB/block drivers, symlinks, devices, sockets, mount namespaces,
  networking, SSH, and phase transition.
- not-an-issue: No runtime code, QEMU run, Pi 5 run, or hardware lock is
  required for this contract task.

## Recommended Next Task

The next mechanically unblocked task should be
`phase10-generated-userland-image-manifest-core-20260605`.

That task should implement the smallest generated-root ingestion path that
makes a manifest/root-defined regular file visible through descriptor-backed
VFS read evidence without adding a new hardcoded `src/initramfs.rs`
file-content constant. It may include a generated executable only if it can
preserve the accepted VFS/loader/userspace lifecycle controls inside the same
bounded slice.

## Validation

- static source/doc/script review: reviewed `src/initramfs.rs`,
  `src/local_command_loop.rs`, `src/program_loader.rs`, `src/syscall.rs`,
  `src/posix.rs`, `build.rs`, QEMU local command-loop scripts, the accepted
  local-storage path evaluation checkpoint, and retained Phase 8/10 task
  records.
- whitespace inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.
- Rust fmt/tests, QEMU runs, Pi 5 hardware runs, archive publication, and
  hardwareTestLock acquisition were not required because this task changes
  only Markdown documentation and durable worker state.
