# Phase 10 Pi 5 Generated-Root Command-Input Source Checkpoint

Task id: phase10-pi5-generated-root-command-input-source-checkpoint-20260617

Status: accepted

Classification:
pi5-generated-root-command-input-source-checkpoint-selected

Evidence level: static/source/task evidence inspection, task-owned JSON
evidence, docs build, and diff checks. No runtime code change, Pi 5 hardware
run, boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, TFTP/serial capture, persistence, storage, networking, SSH, Phase
11/12 expansion, or phase transition was performed.

## Goal

Select the exact generated-root command-input proof contract that can follow the
accepted Pi 5 firmware-initramfs generated-root consumption closeout.

## Context

The accepted firmware-initramfs reservation closeout proves that Talos preserves
the firmware-loaded 'initramfs_2712' range through early memory setup and
installs it as generated-root source 'firmware-initramfs' with reason
'valid-artifact' on Pi 5 hardware.

That closeout deliberately did not accept interactive command input for the
generated-root scenario. The serialized proof reached the scenario but retained
empty command input and reported
'pi5-generated-root-boot-transport-complete-incomplete'. This checkpoint treats
that as a separate control-surface frontier, not as a generated-root transport
blocker.

Prior command-loop/input control evidence is relevant only as a control
surface:

- 'phase10-pi5-local-serial-command-loop-proof-20260531' accepted physical
  serial input, dispatch, response, and next-prompt readiness for a
  kernel-backed command-loop proof.
- 'phase10-pi5-serial-write-ingress-control-proof-20260601' accepted that the
  lab serial write endpoint can write a post-prompt command into a prompt-live
  Pi 5 control and observe the response.
- 'phase10-pi5-serial-command-response-control-discriminator-20260601'
  rejected treating a fresh prompt alone as input responsiveness when the
  proof artifact is no longer servicing post-prompt writes.

Those records do not prove generated-root command input. They only justify the
serial write/observe method selected here.

## Selected Proof Contract

Selected next task:
'phase10-pi5-generated-root-command-input-proof-core-20260617'.

Selected proof scenario:
'pi5-generated-root-manifest-command-input-v1'.

The proof must make the Pi 5 command-input evidence mechanically checkable
before hardware. The acceptance hinge is shell-visible content from the
external firmware-initramfs generated-root artifact, not the compiled fallback.

Required proof command:

~~~text
cat /generated/manifest.txt
~~~

Expected command output:

~~~text
Talos generated-root external artifact A
~~~

Prompt/readiness condition:

- the serial window for the same boot must first retain
  'source=firmware-initramfs reason=valid-artifact' for the generated-root
  selection;
- the proof helper must then wait for a generated-root proof readiness marker
  of the form 'rpi5-generated-root-boot-transport-proof: ready command=N' and
  a visible 'talos> ' prompt before saving the serial cursor used for the
  command write;
- if the proof-core implementation retains the existing multi-command generated
  root harness, it may inject any prerequisite command such as 'rootinfo' before
  the selected manifest command, but only the manifest command and output above
  can satisfy this checkpoint's generated-root command-input claim.

Serial write/observe method:

- use lab-controller '/serial/peek' or '/serial/observe' to save a post-prompt
  cursor after the prompt/readiness condition;
- write the command through 'POST /serial/write' with 'text' equal to the exact
  command and newline termination enabled;
- observe from the saved cursor with 'POST /serial/observe';
- do not use '/serial/transact' as acceptance evidence because prompt matching
  is a convenience wrapper and not the retained command-response transcript.

Minimum accepted serial response fields:

- the retained command text 'cat /generated/manifest.txt';
- the external artifact line 'Talos generated-root external artifact A';
- a generated-root proof dispatch line for that command with
  'status=handled' and 'responses=1';
- a following 'ready-for-next prompt=true' marker or final PASS marker proving
  the command loop remained serviced after the command response.

Hardware proof preconditions remain the standard boot-transport evidence:
candidate identity via lab API, fresh serial cursor, same-power-cycle TFTP
delta, final pre-restore identity, restore proof, and hardwareTestLock
serialization.

## Findings

- fixed: selected a single feature-led proof hinge: post-prompt command input
  must read '/generated/manifest.txt' from the firmware-initramfs generated-root
  artifact and print 'Talos generated-root external artifact A'.
- fixed: tied the serial method to accepted Pi 5 prompt-live serial write
  evidence while rejecting that evidence as generated-root proof by itself.
- fixed: required the generated-root source classification to appear in the
  same boot before command-input evidence can count.
- deferred: proof helper/source changes and local/static validation belong to
  the dependency-gated proof-core task.
- deferred: serialized Pi 5 hardware proof belongs to the dependency-gated Pi 5
  proof task after proof-core acceptance.
- rejected: treating the accepted firmware-initramfs consumption proof,
  kernel-backed command-loop controls, or a visible prompt as generated-root
  command-input proof.
- rejected: persistence, writable filesystems, SD/USB/block storage, broader
  filesystem mutation, networking, SSH, Phase 11/12 expansion, and phase
  transition claims.
- not-an-issue: no hardware lock or Pi 5 inconclusive-run triage was needed for
  this static checkpoint.

## Evidence

- Accepted generated-root closeout:
  'tasks/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-closeout.md'.
- Accepted generated-root Pi 5 proof:
  'tasks/2026-06-16-phase10-pi5-generated-root-firmware-initramfs-reservation-pi5-proof.md'.
- Accepted Pi 5 command-loop control:
  'tasks/2026-05-31-phase10-pi5-local-serial-command-loop-proof.md'.
- Accepted Pi 5 serial-write ingress control:
  'tasks/2026-06-01-phase10-pi5-serial-write-ingress-control-proof.md'.
- Prior prompt-only/input-blocked discriminator:
  'tasks/2026-06-01-phase10-pi5-serial-command-response-control-discriminator.md'.
- Generated-root project contract:
  'docs/src/project/phase10-pi5-generated-root-boot-transport-contract.md'.
- Task classification:
  'tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-source-checkpoint/classification.json'.
- Task evidence map:
  'tasks/evidence/2026-06-17-phase10-pi5-generated-root-command-input-source-checkpoint/evidence-map.json'.

## Acceptance Check

- Findings recorded with disposition: satisfied.
- Selected proof command, prompt-readiness condition, serial write/observe
  method, expected output, and rejected claims are explicit: satisfied.
- Harness/script fix boundedness: no fix was made in this checkpoint; any
  future fix is bounded to proof-core capture/injection mechanics.
- Next task selected only with objective acceptance criteria and evidence
  requirements: satisfied with
  'phase10-pi5-generated-root-command-input-proof-core-20260617'.

## Validation

- static/source/task evidence inspection: pass.
- sh -n on touched shell scripts: not applicable; no shell scripts were
  touched.
- focused local command-loop test or smoke: not applicable; no kernel/source
  behavior was touched.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

After this checkpoint is accepted and committed, promote
'phase10-pi5-generated-root-command-input-proof-core-20260617' if dependencies
remain satisfied. That task may only make the selected manifest-command
generated-root proof mechanically checkable and run local/static gates; it must
not acquire hardwareTestLock or run the serialized Pi 5 proof.
