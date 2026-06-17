# Phase 10 Pi 5 Generated-Root Command-Input Proof Core

Task id: phase10-pi5-generated-root-command-input-proof-core-20260617

Status: accepted

Classification:
pi5-generated-root-command-input-proof-core-local-static

Evidence level: static/source inspection, shell syntax check, compile-only Pi 5
generated-root boot transport image/archive review, task-owned JSON evidence,
docs build, and diff checks. No Pi 5 hardware run, boot archive publication,
lab mutation, hardwareTestLock acquisition, power-cycle, persistence, storage,
networking, SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Prepare the bounded local/static proof helper for the selected Pi 5
generated-root command-input scenario without publishing or running hardware.

## Implementation

Added scripts/rpi5-generated-root-command-input-proof-review.sh. The helper
reviews a non-published generated-root boot archive and emits a JSON proof
contract for the serialized hardware follow-up.

The review first runs the accepted generated-root boot-transport candidate
review, then verifies:

- root and da591740/ copies of initramfs_2712 are present and identical;
- the external artifact contains /generated/manifest.txt and
  Talos generated-root external artifact A;
- the candidate kernel contains the generated-root proof label, final
  classification, firmware-initramfs, valid-artifact, the selected command,
  prompt-readiness, dispatch, response-count, ready-for-next, and PASS marker
  strings;
- the JSON contract records the cursor-based serial write/observe method and
  rejects prompt-only or transact-only evidence.

No runtime behavior changed. Existing Pi 5 generated-root proof harness logic
already expects cat /generated/manifest.txt as command index 1 with one handled
response; this task made that hardware proof contract mechanically checkable
before the serialized run.

## Selected Hardware Follow-Up

Selected next task:
phase10-pi5-generated-root-command-input-pi5-proof-20260617.

Selected scenario:
pi5-generated-root-manifest-command-input-v1.

The hardware proof must wait for same-boot source=firmware-initramfs
reason=valid-artifact, a rpi5-generated-root-boot-transport-proof: ready
command=N marker, and a visible talos> prompt before saving the serial cursor.
It must write exactly:

~~~text
cat /generated/manifest.txt
~~~

Expected shell-visible output:

~~~text
Talos generated-root external artifact A
~~~

Acceptance still requires retained command text, dispatch status=handled
responses=1, and a later ready-for-next prompt=true or final PASS marker from
the same boot.

## Findings

- fixed: added a task-owned proof review helper that emits a durable JSON
  contract for the selected manifest-command proof.
- fixed: made the non-published boot archive's generated-root artifact content
  and root/serial-prefixed initramfs_2712 placement mechanically checked.
- fixed: made the candidate kernel prompt, dispatch, response-count,
  ready-for-next, PASS, firmware-initramfs, and valid-artifact markers
  mechanically checked before hardware.
- fixed: selected the serialized Pi 5 hardware follow-up only after local/static
  proof helper and archive review passed.
- not-an-issue: no kernel/source behavior change was needed because the
  existing generated-root harness already contains the selected manifest command
  and expected dispatch shape.
- deferred: Pi 5 command-input acceptance remains with the serialized hardware
  proof task.
- rejected: prompt visibility alone, serial transact output, or local/static
  strings are not generated-root command-input success evidence.
- rejected: persistence, writable filesystems, SD/USB/block storage, networking,
  SSH, Phase 11/12 expansion, and phase transition claims.

## Evidence

- Selected checkpoint:
  tasks/2026-06-17-phase10-pi5-generated-root-command-input-source-checkpoint.md.
- Proof helper:
  scripts/rpi5-generated-root-command-input-proof-review.sh.
- Compile-only review JSON:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-proof-core/command-input-review.json.
- Compile-only boot tree listing:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-proof-core/boot-tree-files.txt.
- Classification JSON:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-proof-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-proof-core/evidence-map.json.

## Validation

- static/source inspection: pass.
- sh -n scripts/rpi5-generated-root-command-input-proof-review.sh: pass.
- compile-only Pi 5 generated-root boot transport image/archive review: pass,
  archive SHA-256
  8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c,
  kernel SHA-256
  c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd,
  kernel size 208984 bytes, artifact SHA-256
  0341f5393502f54489acb1951633bf2773fb846a82bde89b3e4a2e82000724c6,
  artifact size 662 bytes.
- focused local command-loop test: not applicable; no runtime behavior changed.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Proof helper/review output makes planned Pi 5 command-input evidence
  mechanically checkable before hardware: satisfied.
- Expected output ties shell-visible behavior to the external generated-root
  artifact: satisfied by artifact-content checks plus the required same-boot
  source=firmware-initramfs reason=valid-artifact hardware gate.
- Accepted local/QEMU generated-root controls and Pi 5 boot-transport archive
  review are preserved: satisfied; the helper wraps the accepted candidate
  review and no runtime code changed.
- Serialized Pi 5 proof follow-up selected only after local/static gates passed:
  satisfied with phase10-pi5-generated-root-command-input-pi5-proof-20260617.

## Next Action

Promote phase10-pi5-generated-root-command-input-pi5-proof-20260617 on the next
worker wake if dependencies remain satisfied, the repository is clean, and
hardwareTestLock is unlocked/restored. That task owns hardware publication,
power-cycle, serial write/observe, TFTP, final identity, and restore evidence.
