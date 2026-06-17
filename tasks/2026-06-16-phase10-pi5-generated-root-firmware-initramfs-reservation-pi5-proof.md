# Phase 10 Pi 5 Generated-Root Firmware Initramfs Reservation Pi 5 Proof

Task id: phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof-20260616

Status: accepted

Classification:
pi5-generated-root-firmware-initramfs-consumed-command-input-not-exercised

Evidence level: static archive/image review, lab-controller API,
same-power-cycle TFTP, serial hardware boot/output, restore proof, task-owned
JSON evidence, docs build, and diff checks.

## Goal

Run the serialized Pi 5 proof for the accepted firmware initramfs reservation
core and classify whether the firmware-loaded generated-root artifact survives
early memory setup.

## Result

The rerun proof accepted the generated-root preservation boundary. The selected
boot tree remained staged through the power cycle, TFTP served
'da591740/kernel_2712.img' twice at the expected 208984 bytes, and serial
reported:

'rpi5-generated-root-boot-transport: firmware-initramfs
start=0x000000002efff000 end=0x000000002efff296 len=0x0000000000000296
source=firmware-initramfs reason=valid-artifact ...'

This proves the firmware-loaded 'initramfs_2712' bytes were preserved through
early memory setup and consumed by Talos as the external generated-root
artifact. The Pi 5 proof does not accept interactive command injection success:
the scripted command loop reached the scenario but read empty command input and
reported 'pi5-generated-root-boot-transport-complete-incomplete'. That is
retained as a separate command-input/control-surface limitation, not a
generated-root preservation blocker.

## Findings

- fixed: The selected memory-plan exclusion allowed the firmware initramfs
  range to survive until the generated-root installer consumed it.
- fixed: Hardware proof retained selected-tree identity, same-power-cycle TFTP
  byte agreement, serial freshness, final pre-restore identity, and restore
  evidence.
- deferred: Interactive Pi 5 command injection for this scenario still needs a
  separate boundary if it becomes feature-relevant.
- not-an-issue: The accepted generated-root source is independent of the local
  command loop's failed scripted input in this run.
- rejected: persistence, SD/USB/block storage, networking, SSH, and phase
  transition claims.

## Evidence

- Archive review:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof/candidate/archive-review.txt'.
- Accepted rerun capture summary:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof/candidate-rerun/capture-invariant-summary.json'.
- Classification JSON:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof/classification.json'.
- Evidence map:
  'tasks/evidence/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof/evidence-map.json'.

The first candidate attempt is retained as non-decisive evidence because a
manual restore was issued while the capture was still running. The rerun is the
decisive proof.

## Validation

- static archive/image review: pass.
- lab-controller API selected-tree and restore evidence: pass.
- same-power-cycle TFTP: pass, two expected
  'da591740/kernel_2712.img' fetches at 208984 bytes.
- serial hardware boot/output: pass for generated-root consumption with
  'source=firmware-initramfs reason=valid-artifact'.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Hardware lock acquisition/release, candidate identity, fresh serial cursor,
  TFTP delta, final identity, and restore evidence are recorded: satisfied.
- Candidate proves firmware-loaded generated-root initramfs bounds are
  preserved/consumed: satisfied by serial source classification.
- Known-good or paired control retained when capture/staging evidence is
  inconclusive: satisfied by a clean rerun after the non-decisive first attempt.
- Terminal classification is explicit and does not infer persistence,
  SD/USB/block storage, networking, SSH, or phase transition: satisfied.
- Closeout follow-up selected:
  phase10-pi5-generated-root-firmware-initramfs-reservation-closeout-20260616.

## Next Action

Promote the closeout task after this proof is committed. The closeout should
accept Pi 5 generated-root firmware-initramfs consumption, retain command-input
limitations separately, and reject persistence, storage, networking, SSH, and
phase transition claims unless explicitly planned later.
