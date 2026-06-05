# Phase 10 Generated Root No-Rebuild Transport Closeout

Task: phase10-generated-root-no-rebuild-transport-closeout-20260605

Status: accepted

## Scope

- Reconcile the accepted local/QEMU no-kernel-rebuild generated-root transport
  evidence against Milestone 10.3.
- State the accepted local boundary and the still-deferred Pi 5, boot archive,
  writable persistence, storage-driver, networking, SSH, and phase-transition
  boundaries.
- Hand the next Milestone 10.3 checkpoint back to supervisor planning because
  no explicit queued follow-up remains.

## Non-Goals

- No runtime behavior changes.
- No Pi 5 hardware run, boot archive publication, SD/USB/block driver,
  writable persistence, networking, SSH, or phase transition.

## Static Evidence Map

- Core task record:
  tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-core.md
- Combined no-rebuild transport evidence:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-smoke.log
- Artifact A evidence:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-artifact-a.log
- Artifact B evidence:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-artifact-b.log
- Missing-artifact fallback evidence:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-missing.log
- Malformed-artifact fallback evidence:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-malformed.log

Accepted evidence facts:

- Same kernel ELF SHA-256 for both artifact runs:
  a0bc112bde45ef36f18d5d283cf540553827ed5dd31e1afcfb51f54505331197
- Same kernel image SHA-256 for both artifact runs:
  e8d4d60e0c3ecfc42ba8c4a8e13886a6534450b5e0a36ef2eb4b797990ed50a5
- Artifact A SHA-256:
  0341f5393502f54489acb1951633bf2773fb846a82bde89b3e4a2e82000724c6
- Artifact B SHA-256:
  1413c055cae7fde00194947b10fc69bc3da095a9cd7d78d3d5c877d7f4a1933d
- Artifact loader address:
  0x47000000
- Collision guard:
  __kernel_end=0x00000000403bb000 <= 0x47000000
- Artifact A reports source=external reason=valid-artifact, reads
  "Talos generated-root external artifact A" from
  /generated/manifest.txt, and exits /generated/status7 with status 0x7.
- Artifact B reports source=external reason=valid-artifact, reads
  "Talos generated-root external artifact B" from
  /generated/manifest.txt, and exits /generated/status7 with status 0x9.
- Missing artifact reports source=compiled-fallback reason=missing-artifact.
- Malformed artifact reports source=compiled-fallback reason=digest-mismatch.
- Retained controls cover descriptor-backed cat, VFS exec/open/read, loader,
  argv/envp/status, waitpid/laststatus, pipeline behavior, and jobs/accounting.

## Accepted Boundary

Milestone 10.3 now has a local/QEMU-substitute no-kernel-rebuild transport
proof: Talos can load a nontrivial generated-root artifact and change generated
file and executable behavior without rebuilding the kernel binary between the
two artifact runs.

This does not accept Pi 5 firmware/TFTP placement, boot archive publication,
writable persistence, SD/USB/block storage, networking, SSH, or a phase
transition. The accepted transport is QEMU loader-device evidence only.

## Findings

- fixed: Reconciled the same-kernel/two-artifact evidence against the
  Milestone 10.3 no-kernel-rebuild local criterion.
- fixed: Recorded that artifact A and B differ in digest, generated file
  content, and generated executable status while the kernel ELF/image hashes
  remain unchanged.
- fixed: Preserved the deterministic fallback boundary for missing and
  malformed artifacts.
- removed: The older wording that treated runtime transport as unaccepted after
  the core task evidence had been accepted.
- deferred: Pi 5 boot archive/TFTP transport, writable persistence,
  SD/USB/block drivers, networking, SSH, and phase transition.
- not-an-issue: No new runtime code, QEMU rerun, Pi 5 run, or hardware lock is
  required for this static closeout.

## Recommended Next Step

Supervisor planning is required for the next explicit Milestone 10.3 checkpoint.
The next work should choose between a serialized Pi 5 boot-transport contract
for placing the generated-root artifact outside the kernel image, a writable
local-storage checkpoint, or another bounded storage/userland-image step. This
closeout does not create that task or infer a phase transition.

## Validation

- static inspection: reviewed accepted no-rebuild transport task record,
  artifact A/B logs, missing and malformed fallback logs, and roadmap Milestone
  10.3 notes.
- whitespace inspection: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
