# Phase 10 Pi 5 Generated-Root Command-Input Pi 5 Proof

Task id: phase10-pi5-generated-root-command-input-pi5-proof-20260617

Status: blocked

Classification:
inconclusive-command-input-capture-or-timing

Evidence level: lab-controller API, same-power-cycle TFTP, serial hardware
boot/output, serial write attempt, restore proof, and task-owned JSON evidence.

## Goal

Run the serialized Pi 5 proof that the firmware-initramfs generated-root
artifact is shell-visible through command input.

## Result

The proof did not accept generated-root command input.

The strongest rerun staged the selected generated-root command-input archive,
drained serial to an empty pre-power state, power-cycled the Pi 5, and retained
serial output showing:

- source=firmware-initramfs reason=valid-artifact;
- rpi5-generated-root-boot-transport-proof readiness and a visible talos>
  prompt;
- a successful lab /serial/write response for 28 bytes.

The retained post-write serial did not include the injected
cat /generated/manifest.txt command, the expected
Talos generated-root external artifact A output, or a handled dispatch for the
manifest command. Later retained harness output showed empty command timeouts
and ready-for-next prompt=true. The run is therefore an input timing/capture
blocker, not generated-root artifact consumption acceptance.

## Findings

- fixed: captured candidate archive identity, post-publish lab identity,
  TFTP evidence, prompt-readiness serial evidence, serial write response, and
  restore evidence under hardwareTestLock.
- fixed: after the first saturated-cursor attempt, used direct serial/read as
  the known-good capture control and reran the candidate with a drained serial
  pre-power state.
- blocked: /serial/write accepted the selected command payload after the
  prompt-readiness gate, but the retained serial stream never showed the command
  text or manifest output before the harness advanced through empty input
  timeouts.
- deferred: a follow-up needs a supervisor-planned timing/capture fix or proof
  harness adjustment before another command-input acceptance attempt.
- rejected: persistence, writable filesystems, SD/USB/block storage,
  networking, SSH, Phase 11/12 expansion, and phase transition claims.

## Evidence

- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-pi5-proof/evidence-map.json.
- Selected rerun:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-pi5-proof/candidate-rerun2-direct-read-drained/.
- Baseline direct-read control:
  tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-pi5-proof/control-baseline-direct-read/control-summary.json.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked with
  inconclusive-command-input-capture-or-timing.
- candidate identity via lab API: pass for selected archive publication.
- fresh serial cursor/read evidence: pass for rerun2 pre-power empty drain.
- TFTP delta: retained, but final selected rerun evidence is inconclusive and
  not accepted as command-input proof.
- known-good control and candidate rerun after inconclusive capture: pass.
- post-run restore proof: pass, with manual restore confirmation after
  interrupted capture handling.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Hardware lock acquisition/release, candidate identity, fresh serial cursor,
  TFTP delta, final identity, and restore evidence are recorded: satisfied for
  blocked evidence.
- Serial evidence records a nonempty injected command after prompt readiness:
  not satisfied; /serial/write accepted bytes, but retained serial did not show
  the command text.
- Shell-visible generated-root behavior from firmware-initramfs artifact:
  not satisfied.
- Terminal classification distinguishes command-input timing/capture from
  runtime failure: satisfied.
- Rejected claims remain explicit: satisfied.

## Next Action

Supervisor planning is required before another hardware attempt. Do not promote
the closeout as accepted command-input proof.
