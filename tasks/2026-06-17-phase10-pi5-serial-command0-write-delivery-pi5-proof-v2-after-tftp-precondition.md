# Phase 10 Pi 5 Serial Command 0 Write-Delivery Pi 5 Proof V2

Task id: phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition-20260617

Status: accepted

Classification:
command0-write-delivery-blocked

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, same-cursor stable TFTP log evidence, serial direct-read
hardware output, selected-kernel/TFTP precondition guard evidence, restore
proof, task-owned JSON evidence, docs build, and diff checks.

## Goal

Retry the command 0 write-delivery Pi 5 proof after the selected-kernel/TFTP
precondition was accepted, and determine whether rootinfo written through
/serial/write reaches the command 0 transaction.

## Result

The selected candidate archive
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz was
published with archive SHA-256
8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c. Its
kernel_2712.img SHA-256 was
c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd and its
expected byte count was 208984.

The selected-kernel/TFTP precondition passed during this proof. /boot/files
reported selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212, effective
kernel kernel_2712.img, and da591740/kernel_2712.img at 208984 bytes. The
same-power-cycle stable TFTP delta retained two da591740/kernel_2712.img
serves, both at 208984 bytes:

- Jun 17 12:44:49 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.
- Jun 17 12:44:50 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.

The serial readiness boundary was retained: same-boot
source=firmware-initramfs reason=valid-artifact, ready command=0, and a visible
talos> prompt. The command 0 pre-write direct read retained only two bytes and
did not already contain rootinfo or command 0 output. /serial/write accepted
9 bytes for rootinfo.

Post-write direct reads retained no command 0 output. The proof therefore does
not accept command 0 write delivery: rootinfo, command 0 line evidence,
dispatch command=0 status=handled, responses=1, and ready command=1 were not
retained after the accepted write. This is now a command0 write-delivery
blocker, not a selected-kernel/TFTP-served mismatch.

The boot tree was restored to baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with the
104136-byte baseline kernel.

## Findings

- fixed: the selected-kernel/TFTP precondition remained satisfied during the
  command0 retry; two same-power-cycle da591740/kernel_2712.img serves matched
  the selected 208984-byte kernel.
- fixed: the proof retained same-boot firmware-initramfs valid-artifact
  readiness, ready command=0, and a visible prompt before the write.
- blocked: /serial/write accepted 9 bytes for rootinfo, but post-write direct
  reads retained no rootinfo, command 0 line marker, dispatch command=0
  status=handled, responses=1, or ready command=1.
- deferred: command0 source-response retention remains non-evaluable until
  command0 write delivery is accepted or a narrower follow-up is planned.
- rejected: command0 source-response retention success, generated-root
  command-input success, storage, networking, SSH, Phase 11/12 expansion, and
  phase transition.

## Evidence

- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/evidence-map.json.
- Selected run:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/candidate-command0-write-delivery-v2-20260617T124424Z/.
- Run classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/candidate-command0-write-delivery-v2-20260617T124424Z/classification.json.
- Direct-read proof:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/candidate-command0-write-delivery-v2-20260617T124424Z/direct-read-evidence.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/candidate-command0-write-delivery-v2-20260617T124424Z/tftp/tftp-delta-stable-pre-restore.json.
- Restore proof:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/candidate-command0-write-delivery-v2-20260617T124424Z/restore/post-restore-boot-files.json.

## Acceptance Check

- Accepted proof retains ready command=0, visible prompt, fresh pre-write
  boundary, accepted rootinfo write, ordered command0 line/dispatch/responses/
  ready evidence, selected-kernel/TFTP byte agreement, final identity, and
  restore evidence: not satisfied; command0 post-write output was absent.
- Blocked proof records the first failing invariant without claiming
  generated-root command-input success: satisfied as post-write direct-read did
  not retain command0 line evidence.
- Inconclusive-run triage is not required: the selected-kernel/TFTP
  precondition, readiness boundary, accepted write, final identity, and restore
  evidence are decisive for a command0 write-delivery blocker.
- selected_next_task=phase10-pi5-serial-command0-write-delivery-v2-closeout-20260617
  after accepted or blocked proof: satisfied.
- Rejected claims include command0 source-response retention success,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked at command0
  write delivery with retained hardware evidence.
- candidate identity via lab API /boot/files before run: pass.
- fresh serial/direct-read evidence per accepted contract: pass for readiness
  and pre-write boundary; blocked for post-write command0 line retention.
- TFTP delta via GET /tftp/logs before restore: pass; stable same-cursor delta
  retained two selected-kernel fetches at the expected byte count.
- post-run baseline restore proof: pass.
- direct-read proof validator: expected fail for the blocked selected hardware
  evidence.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-write-delivery-v2-closeout-20260617 on the
next worker wake if dependencies remain satisfied, the repository remains
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not infer command0 source-response retention success,
generated-root command-input success, storage, networking, SSH, Phase 11/12
expansion, or phase transition from this blocked write-delivery proof.
