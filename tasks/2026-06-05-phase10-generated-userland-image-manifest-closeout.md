# Phase 10 Generated Userland Image Manifest Closeout

Task: phase10-generated-userland-image-manifest-closeout-20260605

Status: accepted

## Scope

Static closeout for the accepted generated userland/initramfs manifest core.

This task reconciles the generated-root implementation evidence, retained
shell/VFS/process-control regressions, Milestone 10.3 roadmap boundary, and
deferred storage/userland-image risks.

Non-goals: no runtime behavior changes, no Pi 5 hardware proof, no boot archive
publication, no SD/USB/block driver, no writable persistence, no networking, no
SSH, and no phase transition.

## Accepted Frontier

The accepted slice proves source-code edit avoidance for adding one read-only
userland file:

- host input: `userland/generated-root.manifest`;
- generated-root identity: `phase10-generated-root-manifest-v1`;
- generated-root source: `userland/generated-root.manifest`;
- generated-root manifest sha256:
  `fcd3045b60cc061a8ca3a288cf0c50cac4cb33e89041b9a17867403c1ffd5ede`;
- generated VFS path: `/generated/manifest.txt`;
- visible behavior: `cat /generated/manifest.txt` prints
  `Talos generated-root manifest fixture` through the descriptor-backed shell
  read path;
- static boundary: `src/initramfs.rs` includes generated `OUT_DIR` constants
  and does not add a hardcoded generated file-content constant.

This is not a kernel binary no-rebuild transport claim. The manifest is
consumed at build time, so changing generated userland content still requires a
kernel rebuild until a later TFTP/initramfs or equivalent transport task proves
otherwise.

## Evidence Map

- QEMU/substitute:
  `tasks/evidence/2026-06-05-phase10-generated-userland-image-manifest-core/qemu-local-shell-generated-userland-manifest-smoke.log`
  records generated-root identity/source/digest, generated file readback,
  root visibility, retained descriptor-backed `cat /etc/banner.txt`, VFS
  exec/open/read loader controls, status/waitpid/laststatus controls,
  pipeline/redirection controls, and jobs/accounting control.
- static retained-control summary:
  `tasks/evidence/2026-06-05-phase10-generated-userland-image-manifest-core/retained-control-evidence-summary.txt`
  maps the same QEMU/substitute log to accepted Milestone 10.1/10.2 controls.
- static inspection:
  `build.rs` parses `userland/generated-root.manifest` and emits generated
  initramfs constants; `src/initramfs.rs` integrates those constants into the
  read-only VFS fixture.
- documentation:
  `tasks/2026-06-05-phase10-generated-userland-image-manifest-core.md` records
  the implementation findings, evidence, and deferred work.

## Findings

- fixed: Reconciled the generated-root manifest implementation with the
  Milestone 10.3 roadmap boundary.
- fixed: Preserved the exact accepted claim as source-code edit avoidance for
  userland content changes, not kernel binary no-rebuild, boot archive update,
  Pi 5 transport, or true persistence.
- fixed: Confirmed retained QEMU/substitute controls cover descriptor-backed
  cat, VFS/open/read exec, loader, argv/envp/status, waitpid/laststatus,
  pipeline/redirection, and jobs/accounting behavior.
- removed: Treating a generated-root build-time manifest as evidence for
  no-kernel-rebuild transport.
- deferred: Generated executable proof, TFTP-loaded initramfs/no-rebuild
  transport, boot archive publication, Pi 5 hardware proof, writable
  persistence, SD/USB/block drivers, networking, SSH, and phase transition.
- not-an-issue: hardwareTestLock stayed unlocked because this closeout is
  static docs/evidence reconciliation only.

## Recommended Next Slice

The next bounded Milestone 10.3 slice should be supervisor-planned rather than
inferred by the worker. Based on this closeout, the strongest candidate is a
transport/no-rebuild contract and proof for loading the generated root outside a
new `src/initramfs.rs` content edit and without overstating Pi 5 or
persistence behavior.

## Validation

- static inspection: reviewed accepted manifest-core task record, generated-root
  QEMU/substitute log, retained-control summary, manifest input, build-script
  generated-root parser, `src/initramfs.rs` generated VFS integration, and
  roadmap Milestone 10.3 notes.
- whitespace inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.
