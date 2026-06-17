# Phase 10 Pi 5 Command0 Selected-Kernel Stability Pi 5 Proof

Task id: phase10-pi5-command0-selected-kernel-stability-pi5-proof-20260617

Status: accepted

Classification:
selected-kernel-stability-blocked-tftp-served-baseline-final-identity-regressed

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, post-publish boot-file identity, fresh serial and TFTP
cursors, stable same-cursor TFTP delta, final pre-restore identity, restore
proof, task-owned discriminator replay, JSON evidence, docs build, and diff
checks.

## Goal

Run one serialized Pi 5 proof for the selected-kernel stability contract before
any command0 write delivery or source-response retention work is retried.

## Result

The proof is accepted as a decisive blocker, not as selected-kernel stability.
The candidate archive
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz was
published with SHA-256
8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c.
Its selected kernel_2712.img SHA-256 was
c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd and its
expected byte count was 208984.

Post-publish /boot/files exposed selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212,
effective kernel_2712.img, and da591740/kernel_2712.img at 208984 bytes.
The same-power-cycle TFTP delta was stable but served two
da591740/kernel_2712.img fetches at 104136 bytes:

- Jun 17 18:03:05 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.
- Jun 17 18:03:06 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.

Final pre-restore identity exposed the baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and the
104136-byte baseline kernel, not the selected tree. Restore to snapshot
pre-selected-kernel-stability-180240 succeeded and returned the lab to the same
baseline tree.

The first failing invariant is same-power-cycle TFTP served bytes do not match
the selected kernel. Because the TFTP delta and final identity are decisive,
the inconclusive-run triage path was not triggered.

## Findings

- fixed: hardwareTestLock was held across candidate publication, power-cycle,
  TFTP/final identity capture, restore, and classification.
- fixed: post-publish lab API identity retained the selected 208984-byte tree
  before power-cycle.
- blocked: same-power-cycle stable TFTP served two baseline-sized 104136-byte
  kernel fetches instead of the selected 208984-byte kernel.
- blocked: final pre-restore identity exposed the baseline tree and
  104136-byte kernel.
- not-an-issue: restore returned the lab to the baseline tree.
- deferred: closeout must reconcile this selected-kernel stability blocker
  before any same-shaped command0 retry, source-response retention proof, or
  generated-root command-input acceptance is selected.
- rejected: command0 write-delivery success, source-response retention,
  generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Evidence

- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/evidence-map.json.
- Run evidence:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/candidate-selected-kernel-stability-20260617T180240Z/.
- Run discriminator output:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/candidate-selected-kernel-stability-20260617T180240Z/classification.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/candidate-selected-kernel-stability-20260617T180240Z/capture/tftp-delta-stable-pre-restore.json.
- Final pre-restore identity:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/candidate-selected-kernel-stability-20260617T180240Z/capture/final-pre-restore-boot-files.json.
- Restore proof:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/candidate-selected-kernel-stability-20260617T180240Z/capture/restore-snapshot.json.

## Acceptance Check

- Accepted proof retains post-publish selected identity, stable same-power-cycle
  TFTP serves matching the selected expected bytes, final pre-restore selected
  identity, and restore proof: not satisfied; the proof is blocked at
  TFTP-served byte mismatch and final identity regression.
- Blocked proof records the precise first failing invariant: satisfied as
  same-power-cycle TFTP served bytes do not match selected kernel.
- If same-power-cycle TFTP is missing or ambiguous, apply inconclusive-run
  triage: not applicable; the TFTP delta was stable and decisive.
- selected_next_task is
  phase10-pi5-command0-selected-kernel-stability-closeout-20260617: satisfied.
- Rejected claims include command0 write-delivery success, source-response
  retention, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked at
  selected-kernel stability.
- candidate identity via lab API /boot/files before power-cycle: pass for the
  selected 208984-byte tree.
- fresh serial cursor captured before power-cycle: pass.
- TFTP tail cursor captured before power-cycle: pass.
- stable same-cursor TFTP delta before restore: blocked; two selected fetch
  paths were served at 104136 bytes.
- final pre-restore boot identity: blocked; final identity was baseline.
- post-run baseline restore proof: pass.
- task-owned proof validator: expected reject for the retained blocked
  evidence.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-selected-kernel-stability-closeout-20260617 on the
next worker wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. Do not accept command0 write-delivery, source-response
retention, generated-root command-input success, storage, networking, SSH,
Phase 11/12 expansion, or phase transition from this proof.
