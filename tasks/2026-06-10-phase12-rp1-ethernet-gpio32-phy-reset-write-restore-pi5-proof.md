# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Write/Restore Pi 5 Proof

Task id: phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof-20260610
Status: blocked
Owner: worker
Classification:
rp1-ethernet-gpio32-phy-reset-write-restore-lab-power-cycle-no-fetch-blocker
Evidence level: static archive/image inspection, lab-controller API, serial
hardware boot/output, TFTP/capture evidence, and restore proof. No GPIO32 /
ETH_RST_N write/restore success, PHY reset assertion/deassertion proof, MDIO
or PHY ownership, Ethernet driver readiness, packet I/O, networking, sockets,
SSH, Phase 12.2, or phase transition was accepted.

## Goal

Run the serialized Pi 5 proof for the accepted GPIO32 / ETH_RST_N
write/restore candidate with paired no-GPIO-write/no-Ethernet control and
restore evidence.

## Findings

- fixed: added the missing candidate/control boot scenarios and archive/review
  helpers for the accepted GPIO32 PHY-reset write/restore proof boundary.
- fixed: candidate archive review passed with nonce
  gpio32-write-candidate-20260610T1818Z, archive sha256
  29a1ab78e2a6ccc4df774d4deabbd150fbf2d91166c56df7050534579eb91cc5,
  kernel_2712.img sha256
  b0c6a13ca264f443090708fd39ac14e88c800b6baad5c9a6ad707448b8368021,
  and kernel_2712.img size 51536 bytes.
- fixed: control archive review passed with nonce
  gpio32-write-control-20260610T1818Z, archive sha256
  6cdd9fb0a187bd1414bfd8ecf7bee4d0cb70896be26180af1d3549bd55829325,
  kernel_2712.img sha256
  ecab917fd097411c89ed643e7991b48f6e305cd9d535863fa658ecc14d037b31,
  and kernel_2712.img size 49560 bytes.
- fixed: candidate staging identity matched selected tree
  365f273586277e9bc6ac886a1e65f1fa9d6209a940ba305db33950e558d9c413
  with expected da591740/kernel_2712.img size 51536 bytes before power.
- blocked: the candidate capture became inconclusive before accepting any
  write/restore result; the retained post-power TFTP delta sampled before
  manual restore had zero events and no accepted candidate marker was retained.
- blocked: the paired no-MMIO control staged selected tree
  fcb0ecc1006c4015e35f8dc6faa720d330018a2c0ce2f23f75241d00858d3a02
  with expected da591740/kernel_2712.img size 49560 bytes, but after
  successful /power/cycle it produced zero TFTP events and only NUL+newline
  serial output.
- blocked: the restored known-good control tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  with da591740/kernel_2712.img size 104136 bytes produced the same zero TFTP
  events and only NUL+newline serial output after successful /power/cycle.
- fixed: final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136
  bytes before hardwareTestLock release.
- deferred: GPIO32 write/restore proof, PHY reset assertion/deassertion proof,
  MDIO/PHY ownership, Ethernet driver readiness, interrupt completion,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain unaccepted until the lab power/boot capture blocker is
  cleared.

No findings were removed.

## Blocker

Precise blocker:
rp1-ethernet-gpio32-phy-reset-write-restore-lab-power-cycle-no-fetch-blocker.

The hardware proof cannot be accepted or rerun meaningfully in this wake
because the no-MMIO control and restored known-good control both failed the
basic power-cycle boot evidence gate: zero TFTP events and only NUL+newline
serial output after /power/cycle returned ok. That makes the candidate result
ambiguous at the lab/power/capture layer, not at the GPIO32 write/restore
feature layer.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/capture-summary.json.
- Candidate archive review:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/archive-review/candidate-review.txt.
- Control archive review:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/archive-review/control-review.txt.
- Candidate inconclusive run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/candidate-run/.
- Manual no-MMIO control run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/control-run-manual/.
- Restored known-good control run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/known-good-control/.
- Final restore:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof/final-post-restore-boot-files.json.

## Validation

- static archive/image inspection: candidate and control review scripts passed.
- compile/typecheck: candidate and no-MMIO control scenarios compiled during
  archive generation.
- lab-controller API: hardwareTestLock was acquired before publication and
  released only after final restore.
- serial hardware output: candidate/control/known-good runs retained blocker
  evidence; no accepted candidate/control marker was observed.
- TFTP/capture evidence: control and known-good post-power samples retained
  stable zero-event deltas.
- restore proof: final /boot/files matched restored tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- JSON validation: jq empty on task-owned classification/evidence-map/capture
  summary JSON passed.

## Next Action

Supervisor or lab-operator intervention is required before rerunning this same
hardware proof. Do not repeat the GPIO32 write/restore candidate until a
restored known-good power cycle again produces TFTP fetches and expected serial
output, or the supervisor defines a narrower lab-recovery task.
