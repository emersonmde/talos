# Phase 10 Pi 5 Serial Command0 Lab Write Boundary Pi 5 Proof

Task id: phase10-pi5-serial-command0-lab-write-boundary-pi5-proof-20260617

Status: accepted

Classification:
command0-lab-write-boundary-precondition-blocked

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, prearmed live /serial/read during /serial/write,
task-owned JSON evidence, restore proof, and diff checks.

## Goal

Run the prearmed live-read discriminator selected by the accepted lab
write-boundary core and determine whether rootinfo written through
/serial/write reaches command0 at the visible prompt boundary.

## Result

The generated-root command-input candidate archive
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz was
published with archive SHA-256
8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c. Its
kernel_2712.img SHA-256 was
c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd and its
byte count was 208984.

The lab API post-publish status exposed selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212,
effective kernel kernel_2712.img, and da591740/kernel_2712.img at 208984
bytes. The same-power-cycle TFTP delta contradicted that selected identity:
the Pi fetched da591740/kernel_2712.img twice, but both served events were
104136 bytes. The final pre-restore status also reported the baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The serial side of the discriminator did retain the intended command0
delivery shape. The boot reached generated-root command0 readiness with
source=firmware-initramfs, reason=valid-artifact, ready command=0, and a
visible talos> prompt. The immediate pre-write read was empty. The prearmed
POST /serial/read was active before POST /serial/write, /serial/write accepted
rootinfo with 9 bytes, and the prearmed read retained rootinfo, line command=0,
dispatch command=0 status=handled, responses=1, and ready command=1 in order.

Because the selected-kernel/TFTP precondition failed, this task does not accept
command0 input delivery. The first failing invariant is
selected-kernel-tftp-precondition-missing. The boot tree was restored to the
baseline snapshot before releasing the hardware lock.

## Findings

- fixed: ran the selected prearmed direct-read discriminator instead of another
  same-shaped saturated cursor observe retry.
- fixed: retained positive serial delivery evidence for the prearmed-read
  shape: rootinfo reached command0 and produced ordered dispatch/response/ready
  output.
- blocked: selected-kernel/TFTP precondition regressed; TFTP served the
  104136-byte baseline kernel despite the post-publish API-selected
  208984-byte candidate tree.
- blocked: final pre-restore identity reported the baseline tree, so the
  command0 serial evidence cannot be accepted as selected-candidate proof.
- deferred: lab-boundary closeout must reconcile the selected-kernel/TFTP
  precondition regression before any source-response retention proof.
- rejected: source-response retention, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Accepted discriminator core:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-discriminator-core.md.
- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/evidence-map.json.
- Run evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/.
- Readiness summary:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/serial/readiness-summary.json.
- Pre-write freshness read:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/serial/command0-pre-write-read.json.
- /serial/write evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/serial/command0-write.json.
- Prearmed live-read evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/serial/command0-prearmed-read.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/tftp/tftp-delta-stable-pre-restore.json.
- Restore proof:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/restore/post-restore-status.json.

## Acceptance Check

- The proof records selected-kernel/TFTP precondition pass before evaluating
  command0 input delivery: not satisfied; post-publish API identity selected
  the 208984-byte kernel, but TFTP served 104136-byte baseline kernel entries
  and final pre-restore identity was baseline.
- Accepted command0 input delivery requires successful /serial/write plus
  bounded serial evidence under the selected discriminator: not satisfied
  because the selected-kernel/TFTP precondition failed, although the prearmed
  serial read retained the expected command0 delivery shape.
- Blocked proof records the precise first failing invariant without shrinking
  acceptance to source-response retention or generated-root command-input
  success: satisfied with selected-kernel-tftp-precondition-missing.
- selected_next_task is either the lab-boundary closeout task or null with
  planningNeeded=true: satisfied with the lab-boundary closeout selected for
  evidence reconciliation.
- Rejected claims remain explicit: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked at
  selected-kernel/TFTP precondition.
- candidate identity via lab API before power: pass for post-publish API
  selected tree and 208984-byte da591740/kernel_2712.img.
- fresh serial cursor before write: pass; the immediate pre-write read was
  empty.
- TFTP delta tied to selected candidate: failed; stable same-power-cycle delta
  retained two 104136-byte da591740/kernel_2712.img serves.
- /serial/write result evidence: pass; rootinfo accepted with 9 bytes.
- selected bounded post-write read/observe evidence: pass for prearmed read
  command0 delivery shape; secondary saturated observe retained zero bytes and
  is diagnostic only.
- final pre-restore boot identity: failed; baseline tree was observed.
- post-run baseline restore proof: pass.
- task-owned classifier output: pass for blocked classification.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-lab-write-boundary-closeout-20260617 on
the next worker wake if dependencies remain satisfied. The closeout must
reconcile the selected-kernel/TFTP precondition regression and must not accept
source-response retention, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or a phase transition from this proof.
