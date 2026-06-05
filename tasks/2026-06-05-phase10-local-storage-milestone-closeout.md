# Phase 10 Local Storage Milestone Closeout

Task: phase10-local-storage-milestone-closeout-20260605

Status: accepted

## Goal

Close the Milestone 10.3 checkpoint by reconciling generated-root source-edit
avoidance, local/QEMU no-rebuild transport, and Pi 5 boot-transport evidence or
blocker status.

## Outcome

Milestone 10.3 is accepted at the local/QEMU generated-root transport frontier
and deferred at the Pi 5 hardware boundary.

The accepted boundary includes:

- a generated-root manifest/root model for a read-only generated file,
  /generated/manifest.txt;
- a generated-root executable, /generated/status7, consumed through the
  accepted VFS/open/read, loader, argv/envp, lifecycle, waitpid, and laststatus
  path;
- local/QEMU no-kernel-rebuild transport where the same kernel ELF/image
  consumes two different talos-generated-root-v1 artifacts at 0x47000000;
- deterministic missing and malformed external-artifact fallbacks to the
  compiled generated-root source.

The deferred boundary includes:

- Pi 5 generated-root consumption, because the first serialized hardware proof
  found the firmware initramfs range at 0x2efff000 overlapping early memory
  setup and falling back to the compiled generated root;
- writable persistence, SD/USB/block storage, broader filesystem mutation,
  networking, SSH, and Phase 11 transition.

This task is a static docs/evidence checkpoint only. It does not change runtime
behavior, publish a boot archive, acquire hardwareTestLock, run Pi 5 hardware,
or infer Phase 11.

## Evidence

Retained evidence inspection:

- tasks/evidence/2026-06-05-phase10-local-storage-milestone-closeout/retained-evidence-inspection.md

Accepted local/generated-root records:

- tasks/2026-06-05-phase10-local-storage-path-evaluation-checkpoint.md
- tasks/2026-06-05-phase10-generated-userland-image-contract.md
- docs/src/project/phase10-generated-userland-image-contract.md
- tasks/2026-06-05-phase10-generated-userland-image-manifest-core.md
- tasks/2026-06-05-phase10-generated-userland-image-manifest-closeout.md
- tasks/2026-06-05-phase10-generated-userland-executable-core.md
- tasks/2026-06-05-phase10-generated-userland-executable-closeout.md
- tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-contract.md
- docs/src/project/phase10-generated-root-no-rebuild-transport-contract.md
- tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-core.md
- tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-closeout.md

Pi 5 candidate and blocker records:

- tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-contract.md
- docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md
- tasks/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core.md
- tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-proof.md
- tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-closeout.md

Key commit references are listed in the retained evidence inspection.

## Acceptance Status

Accepted:

- local/QEMU generated-root no-kernel-rebuild transport;
- generated file and generated executable consumption through accepted
  descriptor-backed VFS and process lifecycle paths;
- non-published Pi 5 candidate archive shape and static identity.

Deferred:

- Pi 5 generated-root transport acceptance, blocked by firmware initramfs range
  overlap with early memory setup;
- writable persistence, SD/USB/block drivers, broader filesystem mutation,
  networking, SSH, and phase transition.

## Findings

- fixed: Reconciled all accepted Milestone 10.3 task records and evidence paths
  into a single checkpoint.
- fixed: Made the milestone status explicit: local/QEMU accepted, Pi 5
  consumption deferred on source-backed blocker evidence.
- fixed: Updated the roadmap acceptance criteria without overstating
  persistence, SD/USB/block storage, networking, SSH, or Phase 11 readiness.
- deferred: The next feature implementation must reserve or copy the Pi 5
  firmware initramfs range before early memory setup and then rerun a serialized
  hardware proof if Pi 5 transport acceptance is desired.
- deferred: True writable persistence and block-backed storage remain future
  roadmap work.
- not-an-issue: No decision-index update was needed; this closeout consolidates
  existing accepted decisions and blocker records rather than creating a new
  expensive-to-reverse design choice.

## Validation

- static evidence inspection: retained in
  tasks/evidence/2026-06-05-phase10-local-storage-milestone-closeout/retained-evidence-inspection.md.
- diff hygiene: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before commit.

## Next Action

Do not infer Phase 11 from this milestone closeout. The queued
phase10-to-phase11-transition-checkpoint-20260605 remains the explicit gate for
deciding whether Phase 10 is ready to close or whether the Pi 5 generated-root
transport blocker keeps Phase 10 open.
