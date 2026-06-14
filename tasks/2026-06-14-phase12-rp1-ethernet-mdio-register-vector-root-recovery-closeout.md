# Phase 12 RP1 Ethernet MDIO Register-Vector Root-Recovery Closeout

Task id:
phase12-rp1-ethernet-mdio-register-vector-root-recovery-closeout-20260611

Status: accepted

Classification: root-recovery-closeout-accepted

Evidence level: static task/evidence inspection.

## Goal

Record the operator resolution and minimal sentinel follow-up evidence before
any supervisor planning considers a future MDIO register-vector retry.

## Scope Performed

- Inspected the accepted operator/lab-service resolution recorded in durable
  state.
- Inspected the accepted minimal sentinel Pi 5 proof committed at
  12605ec7263abfe7ffb46a766b4cbf5cd8c8e3e8.
- Confirmed the hardware lock was released and the lab was restored to the
  baseline boot tree after the sentinel proof.
- Recorded the recovery boundary for supervisor planning.

## Findings

- fixed: the operator resolution selected Pi hardware TFTP fetches as the
  served-root measurement and selected the minimal sentinel proof as the next
  discriminator.
- fixed: the conditional minimal sentinel task is accepted and committed. Its
  hardware evidence observed two same-cursor TFTP fetches of
  da591740/kernel_2712.img at the selected sentinel byte count, 47,832 bytes,
  before restore.
- fixed: the minimal sentinel proof restored the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  releasing the hardware lock.
- deferred: any MDIO/register-vector retry remains a supervisor planning
  decision. This closeout does not authorize or promote the retry.
- removed: no code, helper, or evidence path was removed by this closeout.
- not-an-issue: no hardware action is required for this task because its gate is
  static inspection of already accepted task evidence.

## Recovery Boundary

The previous selected-tree contradiction is closed only for the lab root
identity question addressed by the operator-selected minimal sentinel. The
accepted sentinel proof establishes that a no-MDIO/no-Ethernet selected tree can
be served by TFTP on Pi hardware with matching selected bytes and restored
afterward.

This closeout does not accept RP1 MDIO register-vector behavior, MAN.DATA
values, GPIO32/PHY reset behavior, PHY presence/absence, Ethernet runtime
behavior, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

## Evidence

- Operator/lab-service resolution in durable state:
  memory/talos-supervisor-state.json, planningResolution at
  2026-06-14T01:14:47Z.
- Minimal sentinel task record:
  tasks/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof.md.
- Minimal sentinel capture summary:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/capture-summary.json.
- Minimal sentinel staging identity gate:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/minimal-sentinel-run/staging-identity-gate-output.json.
- Minimal sentinel stable TFTP delta:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-lab-tftp-root-minimal-sentinel-pi5-proof/minimal-sentinel-run/tftp-delta-stable-pre-restore.json.
- This closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-root-recovery-closeout/classification.json.
- This closeout evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-root-recovery-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: passed. The operator resolution and
  conditional minimal sentinel result are present and committed.
- hardware lock inspection: passed. Durable state records unlocked/restored
  after the minimal sentinel proof.
- JSON validation: jq empty passed for closeout JSON evidence.
- diff check: git diff --check passed.
- docs validation: not required; docs/src files were not touched.

## Acceptance Check

- Resolution and follow-up evidence are recorded before supervisor considers
  register-vector retry planning: satisfied.
- Operator/lab-service resolution and conditional sentinel result are recorded:
  satisfied.
- No register-vector retry, new feature work, hardware action, or phase
  transition was started: satisfied.

## Next Action

Set planningNeeded for supervisor attention. The next direction must be planned
by the supervisor; this worker must not infer a retry or broader Phase 12 path
from the closeout.
