# Phase 10 Generated Root No-Rebuild Transport Core

Task: phase10-generated-root-no-rebuild-transport-core-20260605

Status: accepted

## Scope

- Implement the local/QEMU generated-root handoff so one built Talos kernel can
  consume an external generated-root artifact.
- Produce two task-owned generated-root artifacts with different content and
  digests, then run the same kernel binary against both.
- Retain VFS/open/read, loader, argv/envp/status, waitpid/laststatus,
  descriptor, pipeline/redirection, and jobs/accounting controls.

## Non-Goals

- No Pi 5 hardware proof, boot archive publication, writable persistence,
  SD/USB/block driver, networking, SSH, fork/signals/process groups, scheduler
  fairness proof, or phase transition.
- No claim that firmware/TFTP supports this transport.

## Implementation

- Added a deterministic talos-generated-root-v1 binary artifact parser for
  QEMU's loader-device handoff.
- Added a runtime generated-root selection that replaces only the accepted
  generated file and generated executable bytes when a valid external artifact
  is present.
- Added rootinfo as a shell-visible diagnostic line for source, reason,
  digest, lengths, and generated paths.
- Added tools/generated-root-artifact.rs and
  scripts/qemu-local-shell-generated-root-no-rebuild-transport-smoke.sh.
- Adjusted the contract-selected QEMU loader address from 0x48000000 to
  0x47000000 because QEMU places the DTB at 0x48000000 for this machine.
  The implementation records __kernel_end <= 0x47000000 before running.

## Evidence

- QEMU/substitute no-rebuild transport:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-smoke.log
- Artifact A run:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-artifact-a.log
- Artifact B run:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-artifact-b.log
- Missing fallback run:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-missing.log
- Malformed fallback run:
  tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-malformed.log

Summary evidence:

- kernel ELF SHA-256:
  a0bc112bde45ef36f18d5d283cf540553827ed5dd31e1afcfb51f54505331197
- kernel image SHA-256:
  e8d4d60e0c3ecfc42ba8c4a8e13886a6534450b5e0a36ef2eb4b797990ed50a5
- artifact A SHA-256:
  0341f5393502f54489acb1951633bf2773fb846a82bde89b3e4a2e82000724c6
- artifact B SHA-256:
  1413c055cae7fde00194947b10fc69bc3da095a9cd7d78d3d5c877d7f4a1933d
- malformed artifact SHA-256:
  adadd8535e6f6ca3c9295a1ba44ee41ee503e37e141b2f5ed725257aee6abbd2
- collision guard: __kernel_end=0x00000000403bb000 <= 0x47000000
- classification:
  qemu-local-shell-generated-root-no-rebuild-transport-complete

## Retained Controls

- Descriptor-backed cat /generated/manifest.txt reads the active generated
  file bytes.
- exec /generated/status7 alpha, waitpid, and laststatus report artifact-owned
  executable status for valid artifacts A/B.
- exec /bin/status42, exec stdout | exec stdin, jobs, and cat /etc/banner.txt
  remain accepted controls in every run.
- Missing artifact falls back to compiled generated-root with reason
  missing-artifact.
- Malformed artifact falls back to compiled generated-root with reason
  digest-mismatch.

## Findings

- fixed: Implemented the local/QEMU loader-device generated-root transport.
- fixed: Proved two generated-root artifacts change generated file and
  generated executable behavior while the kernel ELF and image hashes remain
  unchanged.
- fixed: Added deterministic missing and malformed fallback behavior with
  shell-visible source/reason reporting.
- fixed: Moved the loader window from the contract's original 0x48000000
  candidate to 0x47000000 after QEMU reported an overlap with the DTB.
- removed: Treating build-time manifest regeneration as satisfying the
  no-kernel-rebuild criterion.
- deferred: Pi 5 boot archive/TFTP transport, writable persistence,
  SD/USB/block drivers, networking, SSH, and phase transition.
- not-an-issue: The current artifact format is intentionally limited to the
  accepted generated file and generated executable paths for this first slice.

## Validation

- cargo fmt --all -- --check passed.
- cargo -Zjson-target-spec test --quiet passed.
- scripts/qemu-local-shell-generated-root-no-rebuild-transport-smoke.sh passed.
- git diff --check passed.
- /home/node/.cargo/bin/mdbook build passed.
- git diff --cached --check passed before commit.
