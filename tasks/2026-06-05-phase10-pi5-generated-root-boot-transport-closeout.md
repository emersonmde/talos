# Phase 10 Pi 5 Generated-Root Boot Transport Closeout

Task: phase10-pi5-generated-root-boot-transport-closeout-20260605

Status: accepted

## Goal

Reconcile Pi 5 generated-root boot-transport proof evidence and freeze the
accepted Milestone 10.3 hardware boundary.

## Outcome

The closeout is accepted as a static evidence/docs reconciliation only. It does
not accept Pi 5 consumption of the external generated-root artifact and does not
close Milestone 10.3.

The accepted boundary is:

- local/QEMU generated-root no-kernel-rebuild transport remains accepted at the
  loader-device 0x47000000 boundary;
- the Pi 5 firmware-initramfs candidate archive shape is accepted as a
  non-published static candidate;
- the serialized Pi 5 run is retained as source-backed blocker evidence showing
  publication, TFTP fetch, serial capture, command-loop readiness, and restore;
- Pi 5 generated-root transport acceptance remains deferred until Talos reserves
  or copies the firmware initramfs range before early memory setup can overwrite
  it and then passes a fresh serialized hardware proof.

## Evidence

Closeout inspection:

- tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-closeout/retained-evidence-inspection.md

Contract:

- task: tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-contract.md
- evidence:
  tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-contract/static-inspection.md
- commit: 7f915dd4f5d168f0fbe1ca93b0821187d0c9b719

Candidate:

- task:
  tasks/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core.md
- evidence:
  tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core/
- commit: 3616d310dd224bd8c4c6c34b161be053205bd793
- archive SHA-256:
  8cb1d731e55f35d13328cf4f618c9dac2bf673311535ddd36038680d8a4ef60e
- kernel SHA-256:
  c44e5a55eb600a09a217c6ad23f665a43d1092a8e982423f5162099c34a42169
- generated-root artifact SHA-256:
  0341f5393502f54489acb1951633bf2773fb846a82bde89b3e4a2e82000724c6

Proof/blocker:

- task: tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-proof.md
- evidence:
  tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-proof/
- commit: 63d212d047e3e6a6647b0a0f1b7149d8518f0c7a
- TFTP proof: da591740/kernel_2712.img at 204888 bytes and
  da591740/initramfs_2712 at 662 bytes
- serial blocker: firmware initramfs range
  0x2efff000..0x2efff296 overlapped early page-frame seed/bootstrap
  reservation/translation-table memory; runtime fell back with
  source=compiled-fallback reason=missing-artifact
- restore proof: prior boot tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 was
  restored after the candidate run

Retained controls:

- tasks/2026-06-05-phase10-generated-root-no-rebuild-transport-closeout.md
- tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core/

## Findings

- fixed: Reconciled the contract, candidate archive, proof/blocker, restore
  evidence, and retained local/QEMU generated-root transport controls.
- fixed: Recorded the accepted boundary without overstating writable
  persistence, Pi 5 generated-root consumption, SD/USB/block storage,
  networking, SSH, or phase transition.
- deferred: Remediation must reserve or copy the firmware initramfs range before
  early memory setup can overwrite it, followed by a fresh serialized Pi 5
  proof.
- not-an-issue: No runtime behavior, QEMU rerun, archive publication, Pi 5
  power-cycle, or hardwareTestLock acquisition was required for this static
  closeout.

## Validation

- static evidence inspection: retained in
  tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-closeout/retained-evidence-inspection.md.
- diff hygiene: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before commit.

## Next Action

Do not promote Milestone 10.3 closeout as accepted from this task. The next
explicit task must be supervisor-planned remediation of the firmware initramfs
range overlap or another bounded task that preserves this blocker boundary.
