# Phase 10 Generated Userland Executable Core Task

Task: phase10-generated-userland-executable-core-20260605

Status: accepted

## Scope

Prove a generated-root user program can execute through the accepted VFS,
loader, startup ABI, and process lifecycle path.

This task extends the generated-root manifest/root model with one generated
executable, wires it into the existing read-only initramfs/VFS fixture without
adding a new src/initramfs.rs file-content constant for the executable bytes,
and runs it through the existing absolute exec path with deterministic
argv/envp/status plus waitpid/laststatus evidence.

Non-goals: no no-kernel-rebuild transport claim, no Pi 5 hardware proof, no
boot archive publication, no SD/USB/block driver, no writable persistence, no
networking, no SSH, no fork/signals/process groups, no scheduler fairness
proof, no PATH discovery beyond the existing fixed allowlist, and no phase
transition.

## Evidence

- static inspection: build.rs consumes userland/generated-root.manifest,
  synthesizes the generated executable ELF bytes at build time, and emits
  generated constants under OUT_DIR; src/initramfs.rs includes those constants
  and has no hardcoded generated file-content or executable byte constant.
- generated-root identity: phase10-generated-root-manifest-v1.
- generated-root source: userland/generated-root.manifest.
- generated-root manifest sha256:
  a4ee55e33293077013f9eb63e40f90abc6ac2d823bfac3d815553679cd62494d.
- generated executable path: /generated/status7.
- QEMU/substitute evidence:
  tasks/evidence/2026-06-05-phase10-generated-userland-executable-core/qemu-local-shell-generated-userland-manifest-smoke.log
  records exec /generated/status7 alpha, source=vfs-open-read, loader fixture
  phase8-program-loader-elf64-aarch64-v1, argc=2, argv0=/generated/status7,
  argv1=alpha, envp-state=empty-envp0, status=0x7, waitpid, and laststatus.
- retained-control summary:
  tasks/evidence/2026-06-05-phase10-generated-userland-executable-core/retained-control-evidence-summary.txt
  maps the same QEMU/substitute run across generated regular-file cat,
  descriptor-backed cat, retained VFS exec/open/read, loader, status/wait,
  pipeline/redirection, and jobs/accounting controls.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute:
  scripts/qemu-local-shell-generated-userland-manifest-smoke.sh --quiet passed.
- static inspection: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before commit.

## Findings

- fixed: Extended the generated-root manifest parser from one generated regular
  file to one regular file plus one generated executable with a deterministic
  exit-status field.
- fixed: Generated the executable ELF bytes from manifest/root input in build.rs
  and kept the executable content out of src/initramfs.rs file-content
  constants.
- fixed: Integrated /generated/status7 as a regular VFS file in the generated
  directory and allowed only that generated executable through the existing
  absolute VFS exec allowlist.
- fixed: Extended the task-owned QEMU/substitute smoke to prove
  /generated/status7 reaches VFS/open/read, the program loader, startup argv/envp
  setup, lifecycle status, waitpid, and laststatus while retaining generated
  regular-file cat and accepted shell controls.
- removed: Treating generated-root manifest support as regular-file-only after
  the executable proof was explicitly queued.
- deferred: Kernel binary no-rebuild transport, boot archive publication, Pi 5
  hardware proof, writable persistence, SD/USB/block drivers, networking, SSH,
  PATH discovery, and phase transition.
- not-an-issue: hardwareTestLock stayed unlocked because this task is local
  static/unit/QEMU-substitute work only.

## Result

Accepted as the generated-root executable proof for Milestone 10.3. The slice
proves that a manifest/root-defined executable can be visible as a regular VFS
node and execute through the accepted local VFS/loader/userspace lifecycle path.
It does not prove no-kernel-rebuild transport or persistent/larger storage.
