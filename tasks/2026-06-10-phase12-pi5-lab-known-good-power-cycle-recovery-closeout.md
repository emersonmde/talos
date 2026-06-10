# Phase 12 Pi 5 Lab Known-Good Power-Cycle Recovery Closeout

Task id: phase12-pi5-lab-known-good-power-cycle-recovery-closeout-20260610
Status: accepted
Owner: worker
Classification:
known-good-power-cycle-tftp-recovered-serial-silent-frontier-held
Evidence level: static inspection of accepted lab-controller API evidence,
TFTP/capture evidence, serial hardware output, task records, and git history.
No additional hardware run was performed.

## Goal

Close out the known-good lab recovery result and decide whether the GPIO32
write/restore v2 proof is mechanically unlocked.

## Findings

- fixed: reconciled accepted recovery classification
  known-good-power-cycle-tftp-recovered-serial-silent-blocker from commit
  f241d15330551fa97c163d0828977027aee30844.
- fixed: confirmed pre-power and final `GET /status` identity matched the
  restored known-good tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10,
  effective kernel kernel_2712.img, and da591740/kernel_2712.img at 104136
  bytes.
- fixed: confirmed fresh pre-power cursors were retained: serial cursor
  4194304 and TFTP cursor 4418657.
- fixed: confirmed the one authorized known-good power cycle returned ok=true
  and recovered TFTP evidence from cursor 4418657 with 13 events, including
  two da591740/kernel_2712.img fetches at 104136 bytes.
- blocked: expected known-good serial output did not recover; serial observe
  from cursor 4194304 captured 0 bytes, no `TALOS:` marker, and no expected
  Talos output.
- deferred: the queued GPIO32 write/restore v2 Pi 5 proof remains unselected
  because the accepted recovery classification is not
  recovered-known-good-power-cycle-tftp-serial.
- deferred: GPIO32 write/restore success, GPIO32 ownership, PHY reset
  assertion/deassertion proof, MDIO/PHY ownership, Ethernet driver readiness,
  interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain unaccepted.

No findings were removed.

## Accepted Boundary

This closeout accepts only the lab evidence-gate checkpoint: known-good TFTP
fetches recovered, but expected known-good Talos serial output remained absent.
It does not accept GPIO32 write/restore, PHY reset assertion/deassertion,
MDIO/PHY, Ethernet driver behavior, packet I/O, networking, SSH, Phase 12.2,
or a phase transition.

Because the recovered classification required by the queued v2 proof is
recovered-known-good-power-cycle-tftp-serial, the v2 GPIO32 write/restore proof
is not mechanically unlocked. Same-shaped GPIO32 hardware retries remain held.

## Evidence

- Recovery task record:
  tasks/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery.md.
- Recovery classification:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/classification.json.
- Recovery evidence map:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/evidence-map.json.
- Recovery capture summary:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/capture-summary.json.
- Recovery TFTP delta:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/known-good-recovery-run/tftp-delta-after-power-from-pre-cursor.json.
- Recovery serial observe:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/known-good-recovery-run/serial-observe-after-power.json.
- Recovery final status:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery/known-good-recovery-run/final-status.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-pi5-lab-known-good-power-cycle-recovery-closeout/evidence-map.json.

## Validation

- static inspection: recovery task record, classification/evidence map,
  capture summary, lab identity snapshots, TFTP delta, serial transcript, and
  git history reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: not run; no docs/src files were touched.
- staged diff check: git diff --cached --check passed.

## Next Action

Supervisor planning is required for a bounded serial-silent known-good boot
discriminator or another explicit recovery gate. Do not promote
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof-20260610 and
do not rerun same-shaped GPIO32 write/restore hardware proof while expected
known-good Talos serial output remains absent.
