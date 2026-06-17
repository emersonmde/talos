# Phase 10 Pi 5 Generated-Root Command-Input Capture Harness Pi 5 Proof

Task id: phase10-pi5-generated-root-command-input-capture-harness-pi5-proof-20260617

Status: blocked

Classification:
command-input-observe-cursor-saturated

Evidence level: lab-controller API, same-power-cycle TFTP, serial hardware
boot/output, saturated /serial/observe check, direct-read diagnostic, known-good
baseline direct-read control, restore proof, and task-owned JSON evidence.

## Goal

Run the serialized Pi 5 proof selected by the capture-harness core: wait for
same-boot firmware-initramfs generated-root readiness, then prove command-indexed
input by observing rootinfo at command 0 and cat /generated/manifest.txt at
command 1 from saved serial cursors.

## Result

The proof did not accept generated-root command input. The selected candidate
archive was published and the Pi fetched the expected candidate kernel and
initramfs before restore, but the retained serial cursor was already at the
4194304-byte saturation boundary before power. The required /serial/observe
call from the saved cursor returned zero bytes:

- cursor_start=4194304;
- cursor_end=4194304;
- bytes=0.

The direct-read diagnostic captured fresh Pi 5 serial from the same boot and
confirmed source=firmware-initramfs reason=valid-artifact, ready command=0, and
a visible talos> prompt. That direct-read evidence proves the candidate reached
the prerequisite generated-root prompt, but it is diagnostic only under the
accepted capture-harness contract and cannot replace command-indexed
/serial/observe proof.

## Findings

- fixed: retained candidate archive identity, post-publish lab identity, TFTP
  evidence, serial observe saturation evidence, direct-read diagnostic serial,
  final pre-restore identity, and restore proof under hardwareTestLock.
- fixed: ran a baseline direct-read control after restore; it retained firmware
  NETWORK/TFTP evidence on the restored baseline tree.
- blocked: command-input acceptance cannot be evaluated while the proof's saved
  serial cursor is saturated and /serial/observe returns no retained bytes.
- deferred: command-indexed rootinfo and manifest command writes remain
  unproved on Pi 5 until the serial-retention/cursor saturation blocker is
  resolved or the accepted evidence contract is replanned.
- rejected: direct-read-only output as command-input acceptance, generated-root
  command-input success, persistence, writable filesystem, SD/USB/block storage,
  networking, SSH, Phase 11/12 expansion, and phase transition claims.

## Evidence

- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-pi5-proof/evidence-map.json.
- Selected run:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-pi5-proof/candidate-observe-saturated-valid-tftp-20260617T025118Z/.
- Baseline control:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-capture-harness-pi5-proof/control-baseline-direct-read-20260617T025231Z/control-summary.json.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked with
  command-input-observe-cursor-saturated.
- candidate identity via lab API: pass for selected archive publication.
- fresh serial cursor/read evidence: blocked for /serial/observe because the
  saved cursor was saturated; diagnostic /serial/read captured fresh candidate
  prompt readiness.
- TFTP delta via GET /tftp/logs: pass before restore for expected
  da591740/kernel_2712.img and da591740/initramfs_2712 fetches.
- known-good control: pass for baseline direct-read firmware/TFTP evidence.
- post-run restore proof: pass; restored tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run because docs/src files were not touched.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Hardware lock acquisition/release, candidate identity, fresh serial/TFTP,
  final identity, and restore evidence are recorded: satisfied.
- Same-boot serial command text and manifest output are retained: not
  satisfied; /serial/observe from the saved cursor returned no bytes because the
  retained cursor was saturated.
- Blocked proof records whether the failure is publication/staging, serial
  freshness/capture, serial write delivery, runtime command input, or
  generated-root regression: satisfied as serial observe/cursor saturation.
- Known-good control was performed before further code changes: satisfied.
- Rejected claims remain explicit: satisfied.

## Next Action

Promote
phase10-pi5-generated-root-command-input-capture-harness-closeout-20260617 on a
future worker wake if dependencies remain satisfied, the repository is clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
Do not start another generated-root command-input hardware attempt, persistence,
storage, networking, SSH, Phase 11/12 expansion, or a phase transition from this
blocked proof.
