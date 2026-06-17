# Phase 10 Pi 5 Serial Command 0 Source Response Retention Pi 5 Proof

Task id: phase10-pi5-serial-command0-source-response-retention-pi5-proof-20260617

Status: accepted

Classification:
serial-command0-source-response-retention-serial-readiness-capture-blocked

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, same-power-cycle TFTP, serial direct-read hardware output,
known-good control triage, candidate rerun, restore proof, task-owned JSON
evidence, docs build, and diff checks.

## Goal

Run the serialized Pi 5 hardware proof selected by the accepted command-0
source-response retention core.

## Result

The proof did not reach the command-0 source-response retention acceptance
boundary. The selected candidate archive was published and the first candidate
attempt retained same-power-cycle TFTP evidence for the expected generated-root
archive files:

- da591740/kernel_2712.img: 208984 bytes.
- da591740/initramfs_2712: 662 bytes.

The first candidate attempt did not retain the generated-root ready command=0
prompt before the command-loop timeout path advanced. It later showed
input-error timeout evidence around command=2, which makes the command-0
transaction non-evaluable. Because that is an inconclusive capture/readiness
shape, the task ran the required triage before treating the result as a
durable blocker:

- candidate identity and TFTP delta were retained;
- a known-good control power cycle was run from the restored baseline;
- a candidate rerun used a long-settle direct-read strategy before command
  write.

The known-good control and rerun retained only early firmware/RP1 serial bytes
under that strategy. The rerun still did not retain generated-root ready
command=0 before the command write, and its command-0 direct-read window
retained only early firmware bytes. The validator therefore failed. This is a
serial freshness/capture/readiness blocker, not accepted evidence about
command-0 source-response generation.

## Findings

- fixed: hardwareTestLock acquisition/release and restore were recorded.
- fixed: candidate archive publication, selected-tree identity, archive
  hashes, kernel/initramfs byte counts, final identity, and restore evidence
  were retained.
- fixed: first candidate same-power-cycle TFTP delta retained expected
  kernel_2712.img and initramfs_2712 byte counts.
- fixed: inconclusive-run triage ran a known-good control before candidate
  rerun.
- blocked: serial direct-read capture did not retain a usable ready command=0
  window for the command-0 transaction.
- deferred: command-0 source-response retention remains non-evaluable until
  the serial readiness/capture blocker is reconciled.
- rejected: generated-root command-input hardware success, prompt-only proof,
  write-only proof, persistence, storage drivers, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/evidence-map.json.
- First candidate attempt:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/candidate-command0-retention-20260617T074126Z/.
- Known-good control and candidate rerun:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-source-response-retention-pi5-proof/triage-control-and-rerun-20260617T074637Z/.

## Acceptance Check

- Hardware lock acquisition/release, candidate identity, fresh serial setup,
  TFTP delta, final identity, and restore evidence are recorded: satisfied.
- Accepted proof retains command-0 source-response evidence required by the
  core contract: not satisfied; the command-0 transaction was not evaluable.
- Blocked proof classifies the first failing invariant: satisfied as serial
  freshness/capture readiness setup.
- Known-good control and candidate rerun are used only under the documented
  inconclusive-run triage path: satisfied.
- Rejected claims remain explicit: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked with retained
  serial readiness/capture blocker evidence.
- candidate identity via lab API /status and /boot/files: pass.
- fresh serial/direct-read evidence: blocked; usable ready command=0 was not
  retained before command timeout/write.
- TFTP delta via GET /tftp/logs before restore: pass for the first candidate
  attempt; it retained the expected kernel and initramfs byte counts.
- known-good control using named boot snapshot/restore: run as inconclusive-run
  triage.
- candidate rerun after discriminator: run; still blocked at serial
  readiness/capture.
- post-run baseline restore proof: pass; restored tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- direct-read proof validator: failed as expected for the blocked hardware
  evidence.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-source-response-retention-closeout-20260617
on the next worker wake if dependencies remain satisfied, the repository is
clean, hardwareTestLock is unlocked/restored, and supervisorIntervention is
inactive. Do not infer generated-root command-input success or transition to
Phase 11/12 from this blocked proof.
