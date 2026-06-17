# Phase 10 Pi 5 Command0 TFTP Selected-Kernel Precondition Pi 5 Proof

Task id: phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof-20260617

Status: accepted

Classification:
command0-tftp-selected-kernel-precondition-accepted

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, serial direct-read setup, same-cursor stable TFTP delta,
final pre-restore identity, restore proof, task-owned JSON evidence, docs
build, and diff checks.

## Goal

Run one serialized Pi 5 proof that the selected command0/generated-root
candidate kernel is actually served by TFTP before command0 behavior is
evaluated.

## Result

The selected-kernel/TFTP precondition is accepted. The candidate archive
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz was
published with archive SHA-256
8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c. Its
kernel_2712.img SHA-256 was
c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd and its
expected byte count was 208984.

Before power-cycle, /boot/files reported selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212, effective
kernel kernel_2712.img, and da591740/kernel_2712.img at 208984 bytes. The
same-power-cycle TFTP delta was stable and retained two
da591740/kernel_2712.img serves, both at 208984 bytes:

- Jun 17 12:16:22 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.
- Jun 17 12:16:23 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.

Final pre-restore identity still reported the selected tree and expected
208984-byte fetch. Restore returned the lab to baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with the
104136-byte baseline kernel.

GET / returned HTTP 404 during optional endpoint metadata capture. That is not
a blocker for this task because /boot/files is the authoritative selected-tree
identity source for this deployed lab API version and supplied the required
identity evidence.

## Findings

- fixed: selected candidate publication was hardware-proven through the
  TFTP-served selected-kernel precondition before any command0 behavior retry.
- fixed: same-power-cycle TFTP delta retained two
  da591740/kernel_2712.img serves, both matching the selected candidate
  208984-byte kernel.
- fixed: final pre-restore identity retained the selected tree and post-run
  restore returned to the saved 104136-byte baseline tree.
- not-an-issue: GET / returned HTTP 404 during optional endpoint metadata
  capture; /boot/files supplied the authoritative selected-tree identity
  source.
- rejected: command0 write-delivery success, command0 source-response retention
  success, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Evidence

- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof/evidence-map.json.
- Accepted candidate run:
  tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof/candidate-selected-kernel-tftp-precondition-20260617T121556Z/.
- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof/candidate-selected-kernel-tftp-precondition-20260617T121556Z/classification.json.
- Capture summary:
  tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof/candidate-selected-kernel-tftp-precondition-20260617T121556Z/capture/capture-invariant-summary.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof/candidate-selected-kernel-tftp-precondition-20260617T121556Z/capture/tftp-delta-stable-pre-restore.json.

## Acceptance Check

- Hardware lock acquisition/release, candidate identity, expected kernel hash
  and byte count, fresh serial/direct-read setup, TFTP delta, final identity,
  and restore evidence are recorded: satisfied.
- Accepted proof shows same-power-cycle TFTP served the selected candidate
  kernel bytes required by the accepted core contract: satisfied by two
  208984-byte da591740/kernel_2712.img serves.
- Blocked proof classification is not applicable; no first failing invariant
  remains for the selected-kernel/TFTP precondition.
- Inconclusive-run triage is not required because candidate identity, serial
  setup, TFTP delta, final identity, and restore evidence were decisive.
- Rejected claims include command0 write-delivery success, command0
  source-response retention success, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition:
  satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: pass for the
  selected-kernel/TFTP precondition.
- candidate identity via lab API /boot/files before run: pass.
- fresh serial cursor/direct-read setup: pass; pre-power drain was empty and
  post-power direct-read retained the required marker.
- TFTP delta via GET /tftp/logs before restore: pass; stable same-cursor delta
  retained two selected-kernel fetches at the expected byte count.
- post-run baseline restore proof: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-tftp-selected-kernel-precondition-closeout-20260617 on the
next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not retry command0 write-delivery behavior directly from this
proof.
