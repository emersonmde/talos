# Phase 10 Pi 5 Serial Command 0 Prelude Pi 5 Proof

Task id: phase10-pi5-serial-command0-prelude-pi5-proof-20260617

Status: accepted

Classification:
serial-command0-prelude-source-response-retention-blocked

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, same-power-cycle TFTP, serial direct-read hardware output,
direct-read proof validator, restore proof, task-owned JSON evidence, docs
build, and diff checks.

## Goal

Run the hardware proof selected by the accepted command 0 prelude guard-core
contract.

## Result

The selected candidate archive booted on the Pi 5 and retained the expected
firmware-initramfs generated-root readiness boundary:

- source=firmware-initramfs reason=valid-artifact;
- ready command=0 and a visible talos> prompt;
- selected boot tree through final pre-restore identity;
- stable same-power-cycle TFTP evidence for da591740/kernel_2712.img and
  da591740/initramfs_2712;
- post-run restore to the pre-run baseline.

The atomic command attempt then wrote rootinfo through /serial/write. The
command 0 direct-read window retained:

- line command=0 hex=72 6f 6f 74 69 6e 66 6f;
- dispatch command=0 status=handled responses=1;
- ready command=1.

The proof remains blocked because the command 0 direct-read window did not
retain the firmware-initramfs valid-artifact source response required by the
accepted guard. The task-owned validator rejected the selected hardware
evidence.

## Findings

- fixed: serialized hardware evidence now proves command 0 rootinfo reached the
  command loop and dispatched as handled.
- fixed: selected-tree, TFTP, final identity, and restore evidence were
  retained for the same run.
- blocked: the direct-read window still missed the command 0 generated-root
  source response required by the guard, so generated-root command-input
  acceptance is not claimed.
- deferred: closeout/reconciliation is selected as
  phase10-pi5-serial-command0-prelude-closeout-20260617.
- rejected: persistence, writable storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Evidence

- Selected run:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof/candidate-command0-atomic-20260617T061825Z/.
- Classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof/evidence-map.json.
- Direct-read validator:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-prelude-pi5-proof/candidate-command0-atomic-20260617T061825Z/direct-read-proof-validation.json.

## Acceptance Check

- Hardware lock acquisition/release, candidate identity, serial freshness,
  TFTP, final identity, and restore evidence are recorded: satisfied.
- Accepted proof retains command 0 source response plus dispatch/readiness:
  blocked; dispatch/readiness were retained, but the source response was not.
- Command 1 manifest proof: not in scope after command 0 guard rejection.
- Blocked proof classifies the first failing invariant: satisfied.
- Rejected claims remain explicit: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked with retained
  source-response evidence gap.
- candidate identity via lab API/root/status before run: pass; GET / remained
  404 endpoint-semantics evidence, while /status and /boot/files retained the
  selected tree.
- fresh serial direct-read evidence per accepted contract: pass for readiness,
  rootinfo line, dispatch, and ready command=1; blocked for command 0 source
  response retention.
- TFTP delta via GET /tftp/logs before restore: pass.
- post-run baseline restore proof: pass.
- direct-read proof validator: expected fail for the selected hardware
  evidence.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-prelude-closeout-20260617 on the next
worker wake if dependencies remain satisfied, the repository remains clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
