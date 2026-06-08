# Phase 11 RP1 Observed GPIO Status V3 Control Pi 5

Task id: phase11-rp1-observed-gpio-status-v3-control-pi5-20260608

Status: completed-blocker

Classification: capture-staging-blocked

## Goal

Re-establish the paired no-MMIO/no-RP1/no-GIC observed GPIO14 STATUS/CTRL
control under the accepted v3 capture freshness procedure.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 work.
- Published only the accepted no-MMIO observed GPIO status control archive:
  target/talos-rpi5-rp1-observed-gpio-status-no-mmio-control-core.tar.gz.
- Captured staged identity, serial freshness, task-owned control markers, TFTP
  delta, final selected-tree identity, and restore proof under
  pi5-capture-transaction-v3.
- Classified the task from retained raw evidence without accepting any RP1/GPIO
  hardware behavior from the control.

## Classification

Completed as capture-staging-blocked.

The retained clean control run staged the expected no-MMIO control tree
133f2a9b4a4c5c21b206d8f9eb8eba4ffe41f787a69b33bced0ea164a8cf83ab with the
expected 48,952-byte da591740/kernel_2712.img. TFTP retained two matching
48,952-byte candidate fetches, and final pre-restore status stayed on the
selected control tree.

The v3 identity/freshness join still rejected the run because serial freshness
was not proven. The pre-power drain exhausted all 96 bounded reads, retained
1,045,568 bytes, and already contained 616 occurrences of the required
TALOS: rp1-observed-gpio-status-control marker. The post-power serial window
contained 36 more control markers, but v3 requires the required marker to be
absent from all pre-power drain responses when using saturated direct-read
capture. Rejection reasons:

- v3-serial-freshness-not-proven
- required-marker-present-before-power

A later manual recovery run is retained as diagnostic context only. It captured
a fresh empty pre-power drain and one post-power marker, but TFTP and final
identity had already returned to the baseline tree, so it is not used as the
decisive blocker.

This task makes no real observed GPIO14 STATUS/CTRL, GPIO ownership, event
generation, interrupt delivery, endpoint ownership, broad RP1 mapping,
DMA/cache, storage, networking, SSH, Milestone 11.3, or phase-transition claim.

## Findings And Disposition

- fixed: hardware lock acquisition and serialized Pi 5 run evidence were
  retained.
- fixed: the accepted no-MMIO control archive identity was retained with
  SHA-256 and static archive review evidence.
- fixed: staging preflight proved the selected control tree and expected
  48,952-byte fetch before power-cycle.
- fixed: TFTP and final pre-restore identity matched the selected no-MMIO
  control tree in the decisive clean run.
- fixed: restore proof shows the lab returned to the baseline production-timer
  tree.
- fixed: v3 rejected stale saturated serial output because the required control
  marker was present before power.
- deferred: no same-shaped hardware retry is mechanically unblocked; supervisor
  planning is required before another observed GPIO status control attempt.
- not-an-issue: no RP1/GPIO behavior is inferred from the no-MMIO control
  marker.
- not-an-issue: the task completed as a blocker instead of a visible control
  proof because the v3 serial freshness discriminator rejected stale marker
  output.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-v3-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-v3-control-pi5/classification.json.
- Retained decisive v3 check:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-v3-control-pi5/clean-v3-check.json.
- Decisive raw run:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-v3-control-pi5/control-rerun-clean/.
- Static archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-v3-control-pi5/static-archive-review.txt.
- Diagnostic manual recovery run:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-v3-control-pi5/control-manual-clean/.

## Validation

- static archive inspection: the no-MMIO control archive review passed.
- lab-controller API /status: pre-run and staged status were retained.
- serial hardware boot/output: the decisive clean run captured task-owned
  no-MMIO control markers after power, but also retained matching stale markers
  before power.
- TFTP lab evidence: retained delta was stable and showed two 48,952-byte
  selected control fetches.
- pi5-capture-transaction-v3: rejected the run as capture-staging-blocked with
  v3-serial-freshness-not-proven and required-marker-present-before-power.
- lab-controller API /status: post-restore status shows baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Result

Completed with committed blocker evidence. The real observed GPIO14 STATUS/CTRL
v3 retry is not mechanically unblocked by this task.
