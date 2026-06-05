# Phase 10 Generated Userland Executable Closeout Task

Task: phase10-generated-userland-executable-closeout-20260605

Status: accepted

## Scope

Static closeout for the accepted generated-root executable core.

This task reconciles the generated executable proof against the generated
userland/initramfs contract and Milestone 10.3 acceptance criteria. It records
the accepted generated file and executable boundary, confirms the retained
evidence map, and identifies the next local no-kernel-rebuild transport
contract as mechanically unblocked for a later worker wake.

Non-goals: no runtime behavior changes, no transport/no-rebuild
implementation, no Pi 5 run, no boot archive publication, no SD/USB/block
driver, no writable persistence, no networking, no SSH, and no phase
transition.

## Accepted Boundary

- generated-root identity: phase10-generated-root-manifest-v1;
- generated-root source: userland/generated-root.manifest;
- generated-root manifest sha256:
  a4ee55e33293077013f9eb63e40f90abc6ac2d823bfac3d815553679cd62494d;
- accepted generated regular file: /generated/manifest.txt, read through
  descriptor-backed VFS/open/read by cat /generated/manifest.txt;
- accepted generated executable: /generated/status7, defined by the
  manifest/root input and exposed as a regular generated VFS node;
- accepted generated executable behavior: exec /generated/status7 alpha
  reaches VFS/open/read, the accepted ELF loader fixture, startup argv/envp,
  lifecycle status 0x7, waitpid, and laststatus;
- static source boundary: build.rs consumes the manifest and emits OUT_DIR
  generated constants; src/initramfs.rs includes those generated constants and
  does not add a hardcoded generated executable content constant.

This accepts source-code edit avoidance for the generated file and executable
content. It does not accept kernel binary no-rebuild transport. The generated
root is still consumed at build time, so changing userland content still
requires rebuilding the kernel binary until a later local/QEMU transport proof
changes that boundary.

## Static Evidence Map

- core task record:
  tasks/2026-06-05-phase10-generated-userland-executable-core.md records the
  implementation, evidence paths, validation gates, source boundary, and
  accepted/deferred behavior.
- QEMU/substitute evidence:
  tasks/evidence/2026-06-05-phase10-generated-userland-executable-core/qemu-local-shell-generated-userland-manifest-smoke.log
  records generated-root identity/source/digest, generated regular-file cat,
  exec /generated/status7 alpha, VFS/open/read loader source, startup ABI
  argv/envp, deterministic status=0x7, waitpid, laststatus, retained
  /bin/status42, pipeline, and jobs/accounting controls, plus PASS
  classification.
- retained-control summary:
  tasks/evidence/2026-06-05-phase10-generated-userland-executable-core/retained-control-evidence-summary.txt
  maps descriptor-backed cat, VFS exec/open/read loader behavior,
  status/wait/laststatus, pipeline/redirection, and jobs/accounting controls
  against the same QEMU/substitute run.
- static source inspection:
  build.rs parses userland/generated-root.manifest, emits generated file and
  executable constants under OUT_DIR, and synthesizes executable ELF bytes from
  the manifest exit-status field. src/initramfs.rs integrates the generated
  constants as VFS nodes and has no new hardcoded generated executable byte
  constant.
- contract alignment:
  docs/src/project/phase10-generated-userland-image-contract.md allowed a
  generated executable proof only when it stayed within the accepted VFS,
  loader, startup ABI, lifecycle/status, waitpid, and laststatus path.

## Findings

- fixed: Reconciled the generated executable proof with the generated-root
  contract and Milestone 10.3 roadmap boundary.
- fixed: Recorded the accepted generated file and generated executable content
  as source-code edit avoidance only, not kernel binary rebuild avoidance.
- fixed: Identified the local/QEMU no-kernel-rebuild transport contract as the
  next mechanically unblocked task after this closeout is accepted.
- removed: Treating the generated executable proof as evidence for boot archive
  publication, Pi 5 behavior, writable persistence, or arbitrary storage.
- deferred: No-kernel-rebuild transport implementation, boot archive
  publication, Pi 5 hardware proof, writable persistence, SD/USB/block
  drivers, networking, SSH, and phase transition.
- not-an-issue: hardwareTestLock stayed unlocked because all evidence for this
  closeout is static inspection of accepted local/QEMU-substitute records.

## Result

Accepted as the generated-root executable frontier closeout for Milestone 10.3.
The next queued task,
phase10-generated-root-no-rebuild-transport-contract-20260605, is mechanically
unblocked for the next worker wake. That task may define the local/QEMU
no-kernel-rebuild generated-root transport contract, but no transport
implementation, Pi 5 proof, boot archive publication, persistence, networking,
SSH, or phase transition is accepted by this closeout.

## Validation

- static inspection: reviewed accepted generated executable core task record,
  QEMU/substitute log, retained-control summary, build.rs, src/initramfs.rs,
  generated-root manifest, generated-root contract, and roadmap Milestone 10.3
  notes.
- diff hygiene: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
