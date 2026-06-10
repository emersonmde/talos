# Phase 12 RP1 Ethernet CLK_ETH_CTRL Write-Restore Pi 5 Proof

Task id: phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clk-eth-ctrl-idempotent-write-restored-with-control
Evidence level: image/archive inspection, lab-controller API, serial hardware
boot/output, TFTP/capture evidence, capture-chain-v4 replay, and restore proof.

## Goal

Run the serialized Pi 5 proof for the exact accepted CLK_ETH_CTRL pre-read raw
idempotent write/readback/restore sequence with paired no-clock-write control
and full restore evidence.

## Findings

- fixed: added the missing candidate/control boot scenarios and archive/review
  helpers for the accepted CLK_ETH_CTRL write/restore proof boundary.
- fixed: candidate rerun archive review passed with nonce
  eth-clk-ctrl-write-candidate-rerun-20260610T145000Z, archive sha256
  6565675e023002ba6f75d9e240e9b98a30fdd87dff83a7fe72f95510d6ec10d4,
  kernel sha256
  7cf07d3c885f7c6cd4195830a3ffc7e34c91dd96e55536f8d8481d559b96aef8,
  and kernel_2712.img size 50040 bytes.
- fixed: control archive review passed with nonce
  eth-clk-ctrl-write-control-20260610T143800Z, archive sha256
  c5ef9c26f849b7a475978c0728e488e25f58a0bdf09211ef04b2bc7a02a4b2a5,
  kernel sha256
  d97734361646bbcda6041aeaef0f6fa9ac627bd90e735204df2da7803e63877b,
  and kernel_2712.img size 49464 bytes.
- fixed: paired control capture-chain-v4 joined selected tree
  5c5144ce68c0537b39dcb216b2ae1343c9197ac7deb310f5c7bcc811efe31d40,
  two matching TFTP fetches of da591740/kernel_2712.img at 49464 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: paired control serial retained the same report/capture path while
  withholding writable CLK_ETH_CTRL target construction, with classification
  no-clock-write-no-ethernet-rp1-ethernet-clk-eth-ctrl-control.
- fixed: unchanged candidate rerun capture-chain-v4 joined selected tree
  8d71d54345a64913e451969b9303cd7df351baa64950dffd2fca890897cf05b3,
  two matching TFTP fetches of da591740/kernel_2712.img at 50040 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: candidate serial reported CLK_ETH_CTRL at 0x1c00018064 with pre_raw
  0x10000800, post_raw 0x10000800, restore_raw 0x10000800,
  post_eq_pre=true, restore_eq_pre=true, and classification
  rp1-ethernet-clk-eth-ctrl-idempotent-write-restored.
- fixed: final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136
  bytes before hardware lock release.
- deferred: candidate-run-short-inconclusive is retained as a short-window
  accidental capture and is not used for acceptance.
- deferred: candidate-run-staging-blocked is retained because its preflight
  staged the candidate tree, serial showed the candidate nonce, but TFTP and
  final pre-restore identity rejoined the baseline tree. The paired control
  and unchanged candidate rerun completed the required inconclusive-run triage.
- deferred: broad clock/reset ownership, shared-clock ownership,
  reset-controller ownership, GPIO32/PHY reset, MDIO/PHY, interrupts, DMA,
  descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future or rejected scope.

No findings were removed.

## Hardware Result

Accepted result:
rp1-ethernet-clk-eth-ctrl-idempotent-write-restored-with-control.

The candidate proves only the selected Ethernet-private CLK_ETH_CTRL
idempotent write/readback/restore path for one register. The paired control
proves the same report and capture path while withholding writable target
construction. This does not prove broad clock/reset ownership, shared-clock
ownership, reset-controller ownership, GPIO32/PHY reset ownership, MDIO/PHY,
DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase
12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/capture-summary.json.
- Candidate accepted rerun:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/candidate-rerun/.
- Paired control:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/control-run/.
- Retained non-acceptance candidate attempts:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/candidate-run-short-inconclusive/
  and
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/candidate-run-staging-blocked/.
- Archive reviews:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/archive-review/.
- Pre-run snapshot:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/pre-run-snapshot-create.json.
- Final restore:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-pi5-proof/final-post-restore-boot-files.json.

## Validation

- static inspection: accepted source contract, core, closeout, runtime
  scenarios, archive helpers, capture summaries, identity joins, and docs
  reviewed.
- shell syntax: bash -n on touched CLK_ETH_CTRL write/restore shell scripts
  passed.
- compile checks: candidate and control Pi 5 boot scenarios passed
  cargo -Zjson-target-spec check against targets/aarch64-talos-rpi5-bcm2712.json.
- image/archive inspection: candidate rerun and control review scripts passed.
- lab-controller API: hardwareTestLock acquired before publication; snapshot
  created and restored; final /boot/files confirmed restored tree.
- serial hardware output: candidate and control markers retained with
  run-unique nonces from direct-read serial windows.
- TFTP/capture evidence: candidate rerun and control stable deltas both
  retained two expected da591740/kernel_2712.img fetches with matching bytes.
- capture-chain replay: candidate rerun and control identity-join-v4 checks
  passed.
- JSON validation: jq empty on task-owned classification/evidence-map/
  capture-summary JSON passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clk-eth-ctrl-write-restore-proof-closeout-20260610 on the
next worker wake. The closeout must reconcile only this exact proof and must
not broaden to broad clock/reset ownership, shared-clock ownership,
reset-controller/GPIO32/PHY/MDIO/DMA/descriptor/interrupt/packet/network/
socket/SSH readiness, Phase 12.2, or a phase transition.
