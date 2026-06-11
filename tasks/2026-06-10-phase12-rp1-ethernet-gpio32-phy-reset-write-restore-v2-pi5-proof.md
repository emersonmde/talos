# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Write/Restore v2 Pi 5 Proof

Task id: phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state
Evidence level: static archive/image inspection, lab-controller API, serial
hardware boot/output, stable TFTP delta, and restore proof. No GPIO32 /
ETH_RST_N write/restore success, GPIO32 ownership, PHY reset
assertion/deassertion proof, MDIO or PHY ownership, Ethernet driver readiness,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition was
accepted.

## Goal

Retry the bounded GPIO32 / ETH_RST_N write/restore Pi 5 proof after accepted
v3 known-good runtime readiness, with paired no-GPIO/no-MMIO control evidence
and final boot tree restore.

## Findings

- fixed: acquired hardwareTestLock before archive publication, staging, power
  cycling, and any task-owned runtime GPIO/RIO/pad/MMIO write path.
- fixed: candidate archive review passed with nonce
  gpio32-write-v2-candidate-20260611T013204Z, archive sha256
  730e1986ed3278016d09e7699590e11778aefd645bb6386aac6c5b4899b664d2,
  kernel_2712.img sha256
  c5c874eab5c7bf76a4d5d24306c98aa67eb0bb1b374708aa4c8aa621c555225c,
  and kernel_2712.img size 51536 bytes.
- fixed: control archive review passed with nonce
  gpio32-write-v2-control-20260611T013204Z, archive sha256
  652109230a338db3b2bc5b2cc2dff9e4e3dc49ef9dbc98d1748d21fd00e4f743,
  kernel_2712.img sha256
  cb155f165238e683ab351fb0a6865d5e5563519566980b38722b66dd9a99f49f,
  and kernel_2712.img size 49560 bytes.
- fixed: candidate selected-tree identity was retained as
  37ce442ce6984499087c555a11593f41c8199f19935b92f9979c40ce2593c2f9
  with effective kernel kernel_2712.img before power.
- fixed: candidate serial hardware output contained the run-unique marker 22
  times and the TFTP delta was stable with 13 events.
- blocked: candidate output classified the feature attempt as
  rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state with
  writes-performed=false. The observed baseline included status 0xabe3300,
  ctrl 0x85, out 0x10, oe 0x10, in 0x12, and event-bits 0xab00000, so no
  write/restore success claim is accepted.
- fixed: paired no-GPIO/no-MMIO control selected-tree identity was retained as
  8abf03022ad41ca3e1de9d59c845cf186eed62a3736938e04292b90a203da9b8
  with effective kernel kernel_2712.img before power.
- fixed: control serial hardware output contained the run-unique marker 21
  times, the TFTP delta was stable with 13 events, and the control
  classification was
  no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control.
- fixed: final lab boot tree was restored to pre-run tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img before hardwareTestLock release.
- deferred: GPIO32 write/restore success, GPIO32 ownership, PHY reset
  assertion/deassertion proof, MDIO/PHY ownership, Ethernet driver readiness,
  interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain unaccepted.

No findings were removed, and no findings were not-an-issue.

## Accepted Boundary

The accepted result is a precise blocked/no-write hardware classification. The
candidate and paired control both reached selected-tree identity, serial output,
and stable TFTP evidence, so this is no longer the earlier lab no-fetch
blocker. The feature itself did not perform the write/restore sequence because
the candidate saw an unexpected GPIO32 event state and reported
writes-performed=false.

This proof accepts the capture chain and the blocked/no-write result only. It
does not accept GPIO32 ownership, PHY reset assertion/deassertion, MDIO/PHY
ownership, Ethernet driver readiness, DMA, descriptors, interrupts, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/capture-summary.json.
- Candidate archive review:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/archive-review/candidate-review.txt.
- Control archive review:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/archive-review/control-review.txt.
- Candidate serial/TFTP/identity evidence:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/candidate-run/.
- Control serial/TFTP/identity evidence:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/control-run/.
- Final restore evidence:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/restore/final-status.json.
- Hardware lock release:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/hardware-lock-released.json.

## Validation

- static archive/image inspection: candidate and control review scripts passed.
- compile/typecheck: candidate and no-MMIO control scenarios compiled during
  archive generation.
- lab-controller API: status, boot files, snapshot, archive publication, power
  cycle, TFTP deltas, final restore, and hardwareTestLock release were
  retained.
- serial hardware boot/output: candidate and control run-unique serial markers
  were observed from saturated-cursor direct-read windows.
- TFTP/capture evidence: candidate and control TFTP deltas were stable and
  non-empty with 13 events each.
- restore proof: final status matched the pre-run tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- JSON validation: jq empty on task-owned classification/evidence-map/capture
  summary JSON passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout-20260610
on the next worker wake. Close out the precise blocked/no-write result before
any same-shaped GPIO32 write/restore rerun or any MDIO/PHY follow-up.
