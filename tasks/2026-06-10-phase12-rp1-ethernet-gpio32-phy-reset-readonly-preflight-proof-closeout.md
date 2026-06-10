# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Read-Only Preflight Proof Closeout

Task id: phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-proof-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-gpio32-phy-reset-readonly-preflight-frontier-closed
Evidence level: static inspection of accepted proof evidence, task records,
documentation, and git history.

## Goal

Close out the accepted GPIO32 / ETH_RST_N read-only preflight Pi 5 proof and
decide whether a write-backed PHY reset or other Phase 12.1 ownership slice is
mechanically objective from this checkpoint.

## Findings

- fixed: reconciled the accepted proof classification
  rp1-ethernet-gpio32-phy-reset-readonly-preflight-visible-with-control from
  commit 01febe03c282f64062d908b542ce89a782f3e30e.
- fixed: confirmed the candidate capture-chain-v4 joined selected tree
  25933d095429b5b91ab2185caa1e5c2ce586346452d838a853dbebacea5c4ba7,
  two expected da591740/kernel_2712.img TFTP fetches at 49528 bytes, fresh
  serial nonce output, final pre-restore identity, and restore proof.
- fixed: confirmed candidate serial reported the accepted source-backed
  GPIO32 / ETH_RST_N preflight facts: source/report contract ids,
  observed-window MACB_MID and prerequisite frontier context, rp1_eth / phy1
  identity, rp1_gpio line 32 route, active-low assertion/deassertion mapping,
  5 ms duration, Linux MACB MDIO reset hook relationship, Phase 11 GPIO
  constraints, future write/restore invariants, rejected claims, retained
  risks, and classification
  rp1-ethernet-gpio32-phy-reset-readonly-preflight-report-visible.
- fixed: confirmed the paired no-GPIO/no-Ethernet control capture-chain-v4
  joined selected tree
  ddd753ab2040cdadde6a6b665b24a96886db2377be76bac006806ea035907bda,
  two expected da591740/kernel_2712.img TFTP fetches at 48688 bytes, fresh
  serial nonce output, final pre-restore identity, and restore proof while
  withholding candidate-only GPIO32/PHY-reset facts.
- fixed: confirmed the lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136
  bytes before hardwareTestLock release.
- not-an-issue: saturated-cursor direct serial reads were acceptable because
  run-unique capture nonce absence before power and presence after power
  proved current-run freshness for candidate and control.
- deferred: GPIO ownership, PHY reset assertion/deassertion, MDIO/PHY
  ownership, write/restore GPIO state, Ethernet driver readiness, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future or
  rejected scope.
- deferred: a write-backed PHY reset task is not mechanically objective from
  this closeout alone because no explicit queued task defines GPIO32 ownership
  preconditions, write/restore safety gates, accepted rollback behavior, or
  hardware acceptance criteria.

No findings were removed.

## Accepted Boundary

The accepted frontier is closed at read-only GPIO32 PHY-reset preflight
visibility/control. The candidate proved the Pi 5 can publish and boot a Talos
artifact that emits the accepted source-backed GPIO32 / ETH_RST_N preflight
report over serial. The paired no-GPIO/no-Ethernet control proved the same
serial/reporting path while constructing no GPIO32, ETH_RST_N, PHY reset,
MDIO, Ethernet MMIO, clock, DMA, descriptor, interrupt, packet, socket, SSH,
or phase-transition target.

This closeout does not accept GPIO ownership, PHY reset assertion/deassertion,
MDIO/PHY ownership, RP1 GPIO/RIO/pad/MMIO writes, Ethernet driver readiness,
DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase
12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped GPIO32 PHY-reset read-only preflight hardware retries are closed
for this candidate/control pair. A future task must provide materially
different scope and explicit acceptance criteria, such as GPIO32 write/restore
ownership with restore proof, an MDIO/PHY ownership boundary, interrupt
completion, DMA/descriptor ownership, or packet I/O scope. This closeout does
not choose such a task.

## Evidence

- Proof task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-proof-closeout/evidence-map.json.

## Validation

- static inspection: proof task record, proof classification/evidence map,
  capture summary, project docs, and git history reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Supervisor planning is required for the next explicit Phase 12.1 ownership
slice. No mechanically objective write-backed PHY reset, MDIO/PHY,
interrupt, DMA/descriptor, packet I/O, networking, socket, SSH, Phase 12.2,
or phase-transition follow-up is selected from this read-only GPIO32 preflight
proof alone.
