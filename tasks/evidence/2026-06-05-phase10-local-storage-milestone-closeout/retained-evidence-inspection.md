# Phase 10 Local Storage Milestone Closeout Evidence Inspection

Task: phase10-local-storage-milestone-closeout-20260605

Evidence level: static evidence inspection.

## Accepted Milestone 10.3 Records

- phase10-local-storage-path-evaluation-checkpoint-20260605
  - task: tasks/2026-06-05-phase10-local-storage-path-evaluation-checkpoint.md
  - commit: 49acd7320c746fe14cd2e31518d9e85c28c32899
  - disposition: accepted generated userland/initramfs manifest ingestion as
    the first read-only VFS-backed storage/userland-image slice.
- phase10-generated-userland-image-contract-20260605
  - task: tasks/2026-06-05-phase10-generated-userland-image-contract.md
  - project doc: docs/src/project/phase10-generated-userland-image-contract.md
  - commit: 307249d379ee53f974c4fb3e716bc8376743311b
  - disposition: accepted contract for deterministic generated-root identity,
    path normalization, read-only VFS integration, and retained controls.
- phase10-generated-userland-image-manifest-core-20260605
  - task: tasks/2026-06-05-phase10-generated-userland-image-manifest-core.md
  - evidence:
    tasks/evidence/2026-06-05-phase10-generated-userland-image-manifest-core/qemu-local-shell-generated-userland-manifest-smoke.log
  - commit: f2ee78956b2d80883f5b7a7dffed26e54ad90fe4
  - disposition: accepted generated /generated/manifest.txt through the
    descriptor-backed VFS/open/read path with generated-root identity/source/
    digest reporting and retained controls.
- phase10-generated-userland-image-manifest-closeout-20260605
  - task: tasks/2026-06-05-phase10-generated-userland-image-manifest-closeout.md
  - commit: fb30415e461a8406ec02ca888e56f9e660dc7f1d
  - disposition: accepted the source-code edit avoidance boundary for one
    generated read-only file only.
- phase10-generated-userland-executable-core-20260605
  - task: tasks/2026-06-05-phase10-generated-userland-executable-core.md
  - evidence:
    tasks/evidence/2026-06-05-phase10-generated-userland-executable-core/qemu-local-shell-generated-userland-manifest-smoke.log
  - commit: ff05777eb297c6ab9418f01b3a9c52d1841fd254
  - disposition: accepted generated /generated/status7 through VFS/open/read,
    loader, argv/envp, lifecycle status, waitpid, and laststatus controls.
- phase10-generated-userland-executable-closeout-20260605
  - task: tasks/2026-06-05-phase10-generated-userland-executable-closeout.md
  - commit: 5e9f01536b4be5fca0049c6a8db112a6748e4d69
  - disposition: accepted generated file and generated executable source-code
    edit avoidance, while leaving no-kernel-rebuild transport deferred.
- phase10-generated-root-no-rebuild-transport-contract-20260605
  - task: tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-contract.md
  - project doc:
    docs/src/project/phase10-generated-root-no-rebuild-transport-contract.md
  - commit: dc0773cbb3215f3a3535fa500e422df420c9c1e5
  - disposition: accepted the local/QEMU no-kernel-rebuild generated-root
    transport contract at QEMU loader-device address 0x47000000.
- phase10-generated-root-no-rebuild-transport-core-20260605
  - task: tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-core.md
  - evidence:
    tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/qemu-local-shell-generated-root-no-rebuild-transport-smoke.log
  - commit: a96e205eb47651e1bbba8561af7c05d5b6aa2502
  - disposition: accepted same-kernel/two-artifact local/QEMU proof. Artifact
    A and B changed generated file content and generated executable status
    without rebuilding the kernel binary between runs.
- phase10-generated-root-no-rebuild-transport-closeout-20260605
  - task: tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-closeout.md
  - commit: b12599d20c5a80bbc869141de6f087341e5678d8
  - disposition: accepted the local/QEMU no-kernel-rebuild transport frontier,
    with Pi 5 transport, writable persistence, storage drivers, networking,
    SSH, and phase transition still deferred.
- phase10-pi5-generated-root-boot-transport-contract-20260605
  - task: tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-contract.md
  - project doc:
    docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md
  - evidence:
    tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-contract/static-inspection.md
  - commit: 7f915dd4f5d168f0fbe1ca93b0821187d0c9b719
  - disposition: accepted Pi 5 firmware-initramfs transport contract only.
- phase10-pi5-generated-root-boot-archive-candidate-core-20260605
  - task:
    tasks/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core.md
  - evidence:
    tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core/
  - commit: 3616d310dd224bd8c4c6c34b161be053205bd793
  - disposition: accepted non-published candidate archive shape and static
    identity only.
- phase10-pi5-generated-root-boot-transport-proof-20260605
  - task: tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-proof.md
  - evidence:
    tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-proof/
  - commit: 63d212d047e3e6a6647b0a0f1b7149d8518f0c7a
  - disposition: completed with a source-backed blocker, not Pi 5 acceptance.
    The Pi fetched candidate kernel and initramfs_2712, but the firmware
    initramfs range at 0x2efff000 overlapped early memory setup and Talos fell
    back to the compiled generated root.
- phase10-pi5-generated-root-boot-transport-closeout-20260605
  - task:
    tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-closeout.md
  - evidence:
    tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-closeout/retained-evidence-inspection.md
  - commit: 133ebbf3c6669944b279eb6ab4596e9821942a34
  - disposition: accepted the blocked Pi 5 evidence boundary and retained
    local/QEMU controls, without accepting Pi 5 generated-root consumption.

## Milestone Status

Milestone 10.3 is accepted only at the local/QEMU generated-root transport
frontier. Talos has a same-kernel/two-artifact proof that external
talos-generated-root-v1 artifacts can change generated file content and a
generated executable status without rebuilding the kernel binary between runs.

Milestone 10.3 remains deferred at the Pi 5 hardware boundary. The candidate
archive and TFTP fetch evidence are retained, but hardware acceptance requires a
future task to reserve or copy the firmware initramfs range before early memory
setup can overwrite it and then pass a fresh serialized Pi 5 proof.

The milestone does not accept writable persistence, SD/USB/block storage,
broader filesystem mutation, networking, SSH, or a Phase 11 transition.

## Findings

- fixed: Reconciled the accepted generated-root file, executable,
  no-kernel-rebuild, candidate archive, hardware blocker, and closeout records
  into one Milestone 10.3 checkpoint.
- fixed: Recorded a precise status: local/QEMU generated-root transport
  accepted; Pi 5 consumption deferred on a source-backed blocker.
- fixed: Preserved the Phase 10 to Phase 11 transition as a separate queued
  checkpoint instead of inferring it from this milestone closeout.
- deferred: Pi 5 firmware initramfs range preservation/copying and fresh
  serialized hardware proof.
- deferred: writable persistence, SD/USB/block drivers, networking, SSH, and
  persistent filesystem policy.
- not-an-issue: No runtime change, QEMU rerun, Pi 5 power-cycle, archive
  publication, hardware lock, or decision-index update was required for this
  static evidence/docs reconciliation.
