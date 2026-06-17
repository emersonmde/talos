# Phase 10 Pi 5 Serial Capture Readiness Pi 5 Proof

Task id: phase10-pi5-serial-capture-readiness-pi5-proof-20260617

Status: accepted

Classification:
serial-capture-readiness-command0-write-delivery-blocked

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, same-power-cycle TFTP, serial direct-read hardware output,
known-good control triage, candidate rerun, restore proof, task-owned JSON
evidence, docs build, and diff checks.

## Goal

Run the serialized Pi 5 proof selected by the serial-capture-readiness guard
core and determine whether command 0 source-response retention is evaluable.

## Result

The proof reached the serial capture/readiness boundary that the previous Pi 5
run missed. The candidate rerun retained same-boot firmware-initramfs
valid-artifact readiness, ready command=0, and a visible talos> prompt. The
fresh command 0 pre-write direct-read was empty, and the lab serial write
endpoint accepted the 9-byte rootinfo payload.

The command 0 write did not reach the command loop in retained target output.
The post-write direct-read window did not contain rootinfo, the command 0 line
marker, generated-root source text, dispatch command=0, or ready command=1.
Instead, it retained later empty input-error timeouts for command=1 and
command=2. Command 0 source-response retention remains non-evaluable because
the first failing invariant is command0 write delivery/capture after readiness,
not source response generation.

The first candidate attempt in this task was discarded as inconclusive because
the local collection command failed after readiness and before the command 0
write, adding an artificial delay. Per the task's inconclusive-run rule, the
worker ran a restored known-good control before rerunning the candidate. The
rerun retained stable same-power-cycle TFTP evidence for:

- da591740/kernel_2712.img: 208984 bytes.
- da591740/initramfs_2712: 662 bytes.

The boot tree was restored after the rerun.

## Findings

- fixed: candidate identity, archive hashes, kernel/initramfs byte counts,
  stable same-power-cycle TFTP evidence, final identity, restore proof, and
  hardwareTestLock ownership were retained.
- fixed: the rerun proved serial readiness/capture can retain same-boot
  firmware-initramfs valid-artifact ready command=0 and a visible prompt for
  this candidate.
- fixed: the inconclusive first candidate was not treated as acceptance
  evidence; a known-good control ran before the candidate rerun.
- blocked: command 0 write delivery/capture remains blocked. The lab accepted
  the rootinfo serial write, but retained target output did not show command 0
  reaching the loop.
- deferred: command0 source-response retention remains non-evaluable until
  command0 write delivery/capture is reconciled.
- rejected: command0 source-response retention success, generated-root
  command-input success, persistence, storage drivers, networking, SSH, Phase
  11/12 expansion, and phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-pi5-proof/evidence-map.json.
- Inconclusive first candidate:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-pi5-proof/candidate-serial-capture-readiness-20260617T091840Z/.
- Known-good control and candidate rerun:
  tasks/evidence/2026-06-17-phase10-pi5-serial-capture-readiness-pi5-proof/triage-control-and-rerun-20260617T092509Z/.

## Acceptance Check

- Accepted proof must show selected candidate identity, fresh serial
  boundary/readiness, stable TFTP evidence, command0 write reaching the loop,
  retained firmware-initramfs valid-artifact response, dispatch command=0
  status=handled responses=1, ready command=1, final identity, and restore
  evidence: partially satisfied; identity, readiness, TFTP, final identity, and
  restore passed, but command0 write did not reach the loop.
- If the run is inconclusive, record the first failing invariant and run
  candidate identity, fresh serial cursor, TFTP delta, known-good control, and
  candidate rerun triage before code changes: satisfied.
- Terminal classification is a serial capture/readiness blocker with
  discriminator evidence: satisfied as command0 write-delivery/capture blocked
  after readiness.
- Rejected claims include persistence, storage, networking, SSH, Phase 11/12
  expansion, and phase transition: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked with retained
  command0 write-delivery/capture evidence.
- lab API GET /status candidate identity before power: pass.
- fresh serial cursor/readiness record: pass on candidate rerun.
- GET /tftp/logs delta for candidate/control serves: pass.
- known-good control before candidate rerun after inconclusive first candidate:
  pass.
- boot restore proof and hardwareTestLock restored=true: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-capture-readiness-closeout-20260617 on the next
worker wake if dependencies remain satisfied, the repository is clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
Do not infer generated-root command-input success or transition to Phase 11/12
from this blocked proof.
