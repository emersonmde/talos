# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Write/Restore Proof Closeout

Task id: phase12-rp1-ethernet-gpio32-phy-reset-write-restore-proof-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-gpio32-phy-reset-write-restore-lab-power-cycle-no-fetch-frontier-held
Evidence level: static inspection of blocked proof evidence, task records,
documentation, and git history. No additional hardware run was performed.

## Goal

Close out the GPIO32 / ETH_RST_N write/restore Pi 5 proof checkpoint after the
committed lab-power-cycle-no-fetch blocker and decide whether a same-shaped
retry or follow-up ownership slice is mechanically objective.

## Findings

- fixed: reconciled the committed proof blocker classification
  rp1-ethernet-gpio32-phy-reset-write-restore-lab-power-cycle-no-fetch-blocker
  from commit 14e79fa9c8ce86f10e339b6dfd73452d0d08b9ab.
- fixed: confirmed candidate and control archive reviews passed before
  publication with kernel_2712.img sizes 51536 and 49560 bytes respectively.
- blocked: the candidate staging identity joined selected tree
  365f273586277e9bc6ac886a1e65f1fa9d6209a940ba305db33950e558d9c413, but the
  post-power capture retained no accepted candidate marker and the sampled
  TFTP delta had zero events.
- blocked: the paired no-MMIO/no-GPIO control staged selected tree
  fcb0ecc1006c4015e35f8dc6faa720d330018a2c0ce2f23f75241d00858d3a02, but a
  successful /power/cycle produced zero TFTP events and only NUL+newline serial
  output.
- blocked: the restored known-good control tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 produced
  the same zero TFTP events and only NUL+newline serial output after
  /power/cycle, proving the blocker is at the lab power/boot capture layer.
- fixed: confirmed final restore returned the lab boot tree to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136
  bytes before hardwareTestLock release.
- deferred: GPIO32 write/restore success, GPIO32 ownership, PHY reset
  assertion/deassertion proof, MDIO/PHY ownership, Ethernet driver readiness,
  interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain unaccepted.

No findings were removed.

## Accepted Boundary

The accepted checkpoint is the precise lab-power-cycle-no-fetch blocker, not a
GPIO32 / ETH_RST_N ownership proof. The candidate/control proof chain reached
archive publication and selected-tree identity, but the no-MMIO control and
restored known-good control both failed the basic boot evidence gate after
successful power cycles. That makes the candidate result ambiguous at the lab
power/boot capture layer.

This closeout does not accept GPIO ownership, PHY reset assertion/deassertion,
MDIO/PHY ownership, RP1 GPIO/RIO/pad/MMIO writes, Ethernet driver readiness,
DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase
12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped GPIO32 PHY-reset write/restore hardware retries are held, not
authorized. A future retry must first prove restored known-good power cycling
again produces TFTP fetches and expected serial output, or the supervisor must
define a separate lab-recovery task with explicit scope and acceptance gates.
This closeout does not choose a non-hardware follow-up ownership slice.

## Evidence

- Proof task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-proof-closeout/evidence-map.json.

## Validation

- static inspection: proof task record, proof classification/evidence map,
  capture summary, project docs, and git history reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Supervisor planning is required before the next explicit Phase 12.1 slice. Do
not rerun the same GPIO32 write/restore proof until restored known-good
power-cycle TFTP fetches and expected serial output recover, or a separate
lab-recovery task is planned. No mechanically objective MDIO/PHY, interrupt,
DMA/descriptor, packet I/O, networking, socket, SSH, Phase 12.2, or
phase-transition follow-up is selected by this closeout.
