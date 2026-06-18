# Phase 10 Pi 5 Rootinfo Tail-Stable Source-Response Pi 5 Proof

Task id: phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof-20260618

Status: accepted

Classification:
command0-tail-stable-source-response-pi5-proof-accepted

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock, lab
controller API identity/status evidence, stable TFTP delta evidence, direct
serial hardware output, prearmed command0 serial read, baseline restore proof,
task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the serialized Pi 5 proof that the tail-stable rootinfo response retains
source=firmware-initramfs and reason=valid-artifact for the same selected
command0 response.

## Result

Accepted. The selected generated-root candidate used the 208984-byte
kernel_2712.img with archive sha256
8ea2ad8199e2c317d6452cd84a4993f7c459eebc8569f7f102420b44cb645518 and kernel
sha256 ec880ee488c7e24d630f1946b72c3c41547582b1d5dbd2a239e3b25856e6101c.
The stable same-cursor TFTP delta retained two
da591740/kernel_2712.img serves at 208984 bytes, and the final pre-restore
status still matched the selected tree.

The first direct-read attempt retained only the tail after command0 and is
recorded as blocked diagnostic evidence. The accepted rerun used a prearmed
direct /serial/read before POST /serial/write. That read retained the complete
same-command0 response:

- rootinfo command text;
- source=firmware-initramfs and reason=valid-artifact on the generated-root
  response line;
- line command=0;
- dispatch command=0 status=handled responses=1;
- ready command=1 with no command1/later advancement.

The named pre-run snapshot restore returned the boot tree to baseline hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
hardwareTestLock release.

The selected next task is
phase10-pi5-rootinfo-tail-stable-source-response-closeout-20260618.

## Findings

- fixed: Pi 5 hardware retained source=firmware-initramfs and
  reason=valid-artifact for the same command0 rootinfo response after the
  tail-stable rootinfo formatting change.
- fixed: selected generated-root kernel identity, stable selected TFTP serves,
  final selected identity, and baseline restore proof were retained under
  hardwareTestLock.
- fixed: the accepted proof uses a prearmed serial read, which preserves the
  command0 response boundary that a post-write-only direct read can miss.
- not-an-issue: generated-root selection still reports the firmware initramfs
  valid-artifact path; no new generated-root source was introduced.
- deferred: generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition remain gated by follow-up
  closeout tasks.

## Evidence

- Accepted classification:
  tasks/evidence/2026-06-18-phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof/candidate-tail-stable-source-response-prearmed-20260618T090004Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof/candidate-tail-stable-source-response-prearmed-20260618T090004Z/evidence-map.json.
- Retained command0 source-response:
  tasks/evidence/2026-06-18-phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof/candidate-tail-stable-source-response-prearmed-20260618T090004Z/serial/post-command-summary.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-18-phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof/candidate-tail-stable-source-response-prearmed-20260618T090004Z/tftp/tftp-delta-after-command.json.
- Restore proof:
  tasks/evidence/2026-06-18-phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof/candidate-tail-stable-source-response-prearmed-20260618T090004Z/restore/post-restore-status.json.
- Diagnostic blocked first attempt:
  tasks/evidence/2026-06-18-phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof/candidate-tail-stable-source-response-20260618T085335Z/classification.json.

## Acceptance Check

- Accepted proof requires selected 208984-byte generated-root kernel identity,
  stable selected-candidate TFTP evidence, and final pre-restore selected
  identity: satisfied.
- Accepted proof requires fresh command0 boundary, /serial/write rootinfo
  acceptance, ordered command0 line/dispatch/ready evidence, and retained
  source=firmware-initramfs reason=valid-artifact for the same command0
  response: satisfied.
- Blocked proof records the precise first failing invariant and does not claim
  generated-root command-input success: satisfied by the first diagnostic run
  and accepted rerun classification.
- Baseline restore proof is retained before releasing hardwareTestLock:
  satisfied.
- selected_next_task is
  phase10-pi5-rootinfo-tail-stable-source-response-closeout-20260618:
  satisfied.
- Rejected claims include storage, networking, SSH, Phase 11/12 expansion, and
  phase transition: satisfied.

## Validation

- Serialized Pi 5 hardware proof under hardwareTestLock: pass, accepted
  classification.
- Candidate identity via lab API before power: pass.
- Fresh serial cursor before write: pass.
- /serial/write result evidence: pass, 9 bytes written for rootinfo.
- Bounded retained command0 response evidence: pass with prearmed direct read.
- Stable selected-candidate TFTP delta: pass, two selected 208984-byte serves.
- Final pre-restore boot identity: pass.
- Post-run baseline restore proof: pass.
- Task-owned classifier output: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-rootinfo-tail-stable-source-response-closeout-20260618 on
the next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and the repository has
no conflicting uncommitted changes. Do not claim generated-root command-input
success, storage, networking, SSH, Phase 11/12 expansion, or phase transition
from this proof.
