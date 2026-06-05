# Phase 10 Generated Userland Image Manifest Core Task

Task: phase10-generated-userland-image-manifest-core-20260605

Status: accepted

## Scope

Implement the smallest generated userland/initramfs manifest ingestion slice
accepted by the generated-root contract.

This task adds a host-side manifest for one read-only generated file, emits a
deterministic generated-root identity and constants at build time, wires that
generated file into the existing read-only initramfs/VFS fixture, and proves it
through the accepted descriptor-backed shell cat path.

Non-goals: no generated executable proof, no kernel binary no-rebuild
transport claim, no boot archive publication, no Pi 5 hardware run, no
writable or persistent filesystem, no SD/USB/block driver, no networking, no
SSH, and no phase transition.

## Evidence

- static inspection: build.rs consumes userland/generated-root.manifest and
  writes generated constants under OUT_DIR; src/initramfs.rs includes those
  constants and has no hardcoded generated file-content string.
- generated-root identity: phase10-generated-root-manifest-v1.
- generated-root source: userland/generated-root.manifest.
- generated-root manifest sha256:
  fcd3045b60cc061a8ca3a288cf0c50cac4cb33e89041b9a17867403c1ffd5ede.
- QEMU/substitute evidence:
  tasks/evidence/2026-06-05-phase10-generated-userland-image-manifest-core/qemu-local-shell-generated-userland-manifest-smoke.log
  records cat /generated/manifest.txt, ls / with generated, the
  generated-root identity/source/digest, and
  hardcoded-src-initramfs-constant=false.
- retained-control summary:
  tasks/evidence/2026-06-05-phase10-generated-userland-image-manifest-core/retained-control-evidence-summary.txt
  maps the same QEMU/substitute run across descriptor-backed cat, VFS/open/read
  exec, loader, argv/envp/status, waitpid/laststatus, pipeline/redirection, and
  jobs/accounting controls.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed, 421 talos no_std
  tests.
- QEMU/substitute:
  scripts/qemu-local-shell-generated-userland-manifest-smoke.sh --quiet
  passed.
- static inspection: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Findings

- fixed: Added a host-side generated-root manifest and build-time parser with
  absolute normalized path, file-content, NUL, and size checks for the thinnest
  regular-file-only slice.
- fixed: Integrated /generated/manifest.txt into the read-only initramfs/VFS
  fixture and root listing while keeping generated content out of
  src/initramfs.rs.
- fixed: Routed shell cat /generated/manifest.txt through the reusable
  descriptor-backed initramfs syscall read helper.
- fixed: Added a task-owned QEMU/substitute smoke that records generated-root
  identity/source/digest, proves generated file visibility and content, and
  retains accepted Milestone 10.1/10.2 controls.
- removed: Treating a new src/initramfs.rs hardcoded file-content constant as
  acceptable Milestone 10.3 progress.
- deferred: Generated executable proof, kernel binary no-rebuild transport,
  boot archive publication, Pi 5 hardware proof, writable persistence,
  SD/USB/block drivers, networking, SSH, and phase transition.
- not-an-issue: hardwareTestLock stayed unlocked because all required
  evidence was static inspection, unit tests, and QEMU/substitute.

## Result

Accepted as the first Milestone 10.3 generated userland/initramfs manifest
implementation slice. The slice proves source-code edit avoidance for adding a
read-only userland file through a manifest/root input and preserves the stronger
transport, hardware, persistence, networking, and phase-transition claims for
explicit later planning.
