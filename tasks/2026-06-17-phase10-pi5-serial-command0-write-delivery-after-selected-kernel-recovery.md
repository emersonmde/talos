# Phase 10 Pi 5 Serial Command0 Write-Delivery After Selected-Kernel Recovery

Task id: phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery-20260617

Status: accepted

Classification:
command0-write-delivery-blocked-after-selected-kernel-recovery

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, selected-kernel/TFTP precondition evidence,
cursor-bound post-write /serial/observe hardware output, task-owned JSON
evidence, restore proof, docs build, and diff checks.

## Goal

Run the distinct command0 write-delivery discriminator selected by the
selected-kernel recovery closeout, and determine whether rootinfo written
through /serial/write reaches command0 under the recovered selected-kernel/TFTP
boundary.

## Result

The generated-root command-input candidate archive
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz was
published with archive SHA-256
8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c. Its
kernel_2712.img SHA-256 was
c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd and its
byte count was 208984.

The selected-kernel/TFTP precondition passed. The boot files exposed selected
tree 06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212,
effective kernel kernel_2712.img, and da591740/kernel_2712.img at 208984
bytes. The same-power-cycle stable TFTP delta retained two matching
da591740/kernel_2712.img serves:

- Jun 17 20:08:31 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.
- Jun 17 20:08:32 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.

The serial capture retained generated-root command-loop readiness for command0:
rpi5-generated-root-boot-transport-proof: ready command=0 and a visible talos>
prompt. The generic readiness helper exited nonzero because its kernel_main
marker was not retained in the saturated direct-read window, but the
task-owned classification uses the command0 readiness marker and prompt for
this proof.

/serial/write accepted the 9-byte rootinfo payload, but the bounded post-write
/serial/observe from cursor 4194304 retained zero bytes. It did not retain
rootinfo or command0 line evidence, dispatch command=0 status=handled,
responses=1, or ready command=1. The first failing invariant is therefore
post-write-observe-missing-command0-delivery.

The boot tree was restored to baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with the
104136-byte baseline kernel.

## Findings

- fixed: the selected-kernel/TFTP precondition stayed recovered during the
  command0 write-delivery proof; two same-power-cycle selected-kernel TFTP
  fetches matched the 208984-byte candidate kernel.
- fixed: retained command0 readiness and a visible prompt before the write.
- blocked: /serial/write accepted rootinfo, but bounded post-write
  /serial/observe retained zero bytes and no command0 delivery evidence.
- deferred: no source-response-retention task is selected because command0
  write delivery is still blocked.
- rejected: source-response retention, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery/evidence-map.json.
- Selected run:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery/candidate-command0-write-delivery-after-recovery-20260617T200806Z/.
- Run classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery/candidate-command0-write-delivery-after-recovery-20260617T200806Z/classification.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery/candidate-command0-write-delivery-after-recovery-20260617T200806Z/tftp/tftp-delta-stable-pre-restore.json.
- Readiness summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery/candidate-command0-write-delivery-after-recovery-20260617T200806Z/serial/readiness-summary.json.
- /serial/write evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery/candidate-command0-write-delivery-after-recovery-20260617T200806Z/serial/command0-write.json.
- Post-write /serial/observe evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery/candidate-command0-write-delivery-after-recovery-20260617T200806Z/serial/command0-post-write-observe.json.
- Restore proof:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-after-selected-kernel-recovery/candidate-command0-write-delivery-after-recovery-20260617T200806Z/restore/post-restore-boot-files.json.

## Acceptance Check

- The proof records selected-kernel/TFTP precondition pass before evaluating
  command0 write delivery: satisfied.
- Accepted command0 write delivery requires successful /serial/write and
  bounded post-write observe evidence showing command0 effect: not satisfied;
  /serial/write succeeded but post-write observe retained zero bytes.
- Blocked proof records the precise first failing invariant without shrinking
  acceptance to source-response retention or generated-root command-input
  success: satisfied with
  post-write-observe-missing-command0-delivery.
- selected_next_task is either a distinct closeout task or null with
  planningNeeded=true, and source-response retention is not selected unless
  command0 write delivery is accepted: satisfied with selected_next_task=null
  and planningNeeded=true.
- Rejected claims include source-response retention unless separately proven,
  generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked at
  post-write observe missing command0 delivery.
- candidate identity via lab API before power: pass.
- fresh serial cursor before write: pass; the proof retained cursor 4194304
  before /serial/write.
- /serial/write result evidence: pass; rootinfo accepted with 9 bytes.
- bounded post-write /serial/observe evidence: blocked; zero bytes retained.
- stable same-cursor TFTP delta before restore: pass; two 208984-byte
  da591740/kernel_2712.img serves retained.
- final pre-restore boot identity: pass.
- post-run baseline restore proof: pass.
- task-owned classifier output: pass for blocked classification.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before any next worker task is promoted.
Command0 write delivery remains blocked at post-write observe missing command0
delivery after a recovered selected-kernel/TFTP precondition and accepted
rootinfo /serial/write. Do not select source-response retention, generated-root
command-input success, storage, networking, SSH, Phase 11/12 expansion, or
phase transition from this proof.
