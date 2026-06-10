# Phase 12 RP1 Ethernet Clock/Reset Write-Restore Pi 5 Proof

Task id: phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored-with-control
Evidence level: image/archive inspection, lab-controller API, serial hardware
boot/output, TFTP/capture evidence, capture-chain-v4 replay, and restore proof.

## Goal

Run the serialized Pi 5 proof for the exact accepted Ethernet-private
CLK_ETH_TSU_CTRL idempotent write/restore sequence with paired control and
full restore evidence.

## Findings

- fixed: added the missing candidate/control boot-tree, archive, and review
  helpers for the accepted write/restore boot scenarios.
- fixed: candidate archive review passed with nonce
  eth-tsu-write-candidate-rerun-20260610T131800Z, archive sha256
  16819179b56e3b71eee2d7474d698db1ee8bcc820193b102c7068d9e6a50197c,
  kernel sha256
  c53cf4d33b26952dd5bb3eeef0697bda73689cdb01a91e46b5c03a9554cc3820,
  and kernel_2712.img size 49704 bytes.
- fixed: control archive review passed with nonce
  eth-tsu-write-control-20260610T130000Z, archive sha256
  bc9e5597cb8566b4bac055f300eb721ac912dbb6fd313208aaf06a77ee66cdbe,
  kernel sha256
  3b91f2e588435ffc0ba895c6fd989b2f49bde17f2496659c2599cde737be05e5,
  and kernel_2712.img size 49120 bytes.
- fixed: candidate capture-chain-v4 joined selected tree
  a8c5f9b18e4443887fa7a834d8ee22691f49c0c5b7f7122cfe7ed36d064377a2,
  two matching TFTP fetches of da591740/kernel_2712.img at 49704 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: candidate serial reported CLK_ETH_TSU_CTRL at 0x1c00018134 with
  pre_raw 0x10000800, post_raw 0x10000800, restore_raw 0x10000800,
  post_eq_pre=true, restore_eq_pre=true, and classification
  rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored.
- fixed: control capture-chain-v4 joined selected tree
  457859469383c34f4d3c241f46c164f0ab560e81cb275154cde4e7ad5152f458,
  two matching TFTP fetches of da591740/kernel_2712.img at 49120 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: control serial retained the same report/capture path while
  withholding writable clock target construction and candidate-only facts,
  with classification
  no-clock-write-no-ethernet-rp1-ethernet-write-restore-control.
- fixed: final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136
  bytes.
- deferred: one overlapping candidate capture attempt is retained under
  candidate-run-overlap-inconclusive and is not used for acceptance.
- deferred: broad clock/reset ownership, CLK_ETH_CTRL, GPIO32/PHY reset,
  MDIO/PHY, interrupts, DMA, descriptors, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition remain future or rejected scope.
- not-an-issue: capture-chain-v4 accepts the candidate saturated direct-read
  because the run-unique nonce was absent before power and present after power.

No findings were removed.

## Hardware Result

Accepted result:
rp1-ethernet-clk-eth-tsu-ctrl-idempotent-write-restored-with-control.

The candidate proves only the selected Ethernet-private CLK_ETH_TSU_CTRL
idempotent write/readback/restore path for one register. The paired control
proves the same report and capture path while withholding writable target
construction. This does not prove broad clock/reset ownership, shared-clock
ownership, reset-controller ownership, GPIO32/PHY reset ownership, MDIO/PHY,
DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase
12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/capture-summary.json.
- Candidate run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/candidate-run/.
- Control run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/control-run/.
- Archive reviews:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/archive-review/.
- Pre-run snapshot:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/pre-run-snapshot-create.json.
- Final restore:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-pi5-proof/final-post-restore-boot-files.json.

## Validation

- static inspection: accepted source contract, core, closeout, runtime
  scenarios, archive helpers, capture summaries, identity joins, and docs
  reviewed.
- shell syntax: bash -n on touched write/restore shell scripts passed.
- image/archive inspection: candidate and control review scripts passed.
- lab-controller API: hardwareTestLock acquired before publication; snapshot
  created and restored; final /boot/files confirmed restored tree.
- serial hardware output: candidate and control markers retained with
  run-unique nonces from direct-read serial windows.
- TFTP/capture evidence: candidate and control stable deltas both retained two
  expected da591740/kernel_2712.img fetches with matching bytes.
- capture-chain replay: candidate and control identity-join-v4 checks passed.
- JSON validation: jq empty on task-owned classification/evidence-map/
  capture-summary JSON passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout-20260610 on the
next worker wake. The closeout must reconcile only this exact proof and must
not broaden to clock/reset ownership, shared-clock ownership,
reset-controller/GPIO32/PHY/MDIO/DMA/descriptor/interrupt/packet/network/
socket/SSH readiness, Phase 12.2, or a phase transition.
