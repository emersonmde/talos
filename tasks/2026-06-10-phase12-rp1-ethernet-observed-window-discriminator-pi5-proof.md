# Phase 12 RP1 Ethernet Observed-Window Discriminator Pi 5 Proof

Task: phase12-rp1-ethernet-observed-window-discriminator-pi5-proof-20260610

Status: accepted

Classification: observed-window-macb-mid-visible

Evidence level: image/archive inspection, lab-controller API, serial hardware
boot/output, TFTP/capture evidence, and restore proof. This accepts only the
read-only observed-window MACB_MID discriminator result, not Ethernet runtime
readiness.

## Scope

- Promoted the queued serialized Pi 5 proof after the accepted observed-window
  closeout selected it and hardwareTestLock was unlocked.
- Added the thinnest runtime candidate/control boot scenarios and archive
  helpers needed to run the accepted observed-window report path on Pi 5.
- Acquired hardwareTestLock before boot archive publication or Pi 5 power
  cycling.
- Rebuilt candidate and paired no-MMIO/no-Ethernet control archives with
  run-unique capture nonces.
- Published and captured candidate/control runs with capture-chain-v4
  selected-tree identity, expected TFTP fetch bytes, run-unique serial marker
  freshness, final pre-restore identity, restore proof, and task-owned JSON.
- Restored the pre-task boot tree before releasing hardwareTestLock.
- Did not implement an Ethernet driver, program RP1 MMIO writes, add DMA,
  descriptor rings, interrupts, clock/reset/PHY/MDIO ownership, packet I/O,
  networking, sockets, SSH, Phase 12.2, or a phase transition.

## Findings

- fixed: candidate archive review passed with nonce
  obswin-candidate-20260610T070042Z-fd93002b, archive sha256
  065c4b85a27395530c06fd9e7cada2e19b8e39fd49a338514643d652b006ee83,
  kernel sha256
  33e69c22bc39ed9b247ecac7a8dedf3b7ba59031f3a24aab66df35a7124916cf,
  and kernel_2712.img size 49576 bytes.
- fixed: control archive review passed with nonce
  obswin-control-20260610T070042Z-fd93002b, archive sha256
  96984812c98b004b0235fe5a23e895be4abd4372a14ede6067cc7b55a38325ec,
  kernel sha256
  9097b6232ff2240789e684214923338bbfcfcda629a871826f201cd506d4bbb9,
  and kernel_2712.img size 48848 bytes.
- fixed: candidate capture-chain-v4 joined selected tree
  ad4f367e43716532623b3d9cbcd0ad4f71a326be5672b1ca11301e37a7d1811a,
  two matching TFTP fetches of da591740/kernel_2712.img at 49576 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: candidate serial retained 27 required marker occurrences. It read
  SYSINFO_CHIP_ID at 0x1c00000000 as 0x20001927 and observed-window MACB_MID
  at 0x1c001000fc as raw 0x70109, idnum 0x7, rev 0x109, classified
  observed-window-macb-mid-visible.
- fixed: control capture-chain-v4 joined selected tree
  7a8775ea4c75e222f77d99d5d6bae1538a0f7fb67ef3026830373bab833d94c3,
  two matching TFTP fetches at 48848 bytes, run-unique serial marker
  freshness, final pre-restore identity, and restore proof without
  constructing SYSINFO, observed-window, translated-comparator, or Ethernet
  MMIO targets.
- fixed: final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img.
- not-an-issue: both capture-bundle summaries retain
  serial-drain-not-empty-before-power as the v2 summary suggestion, but the
  repaired capture-chain-v4 join accepts the runs because the run-unique nonce
  was absent before power and present after power with matching selected-tree,
  TFTP, final identity, and restore gates.
- not-an-issue: observed-window MACB_MID visibility is a read-only identity
  result only; no Ethernet runtime behavior was accepted.

No findings were removed or deferred.

## Hardware Result

Accepted result: observed-window-macb-mid-visible.

The candidate observed RP1 SYSINFO_CHIP_ID at 0x1c00000000 as 0x20001927 and
observed-window MACB_MID at 0x1c001000fc as raw 0x70109, idnum 0x7, rev 0x109.
The paired control proved the reporting path with no constructed MMIO target.
This accepts the observed-window GEM MID read-only identity discriminator only.

## Rejected Claims And Retained Risks

Rejected: Ethernet driver readiness, broad Ethernet MMIO readiness, RP1 MMIO
writes, DMA, descriptor rings, interrupts, clock/reset ownership, PHY/MDIO
ownership, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
transition.

Retained risks: clock/reset and PHY/MDIO ownership remain unaccepted;
descriptor rings, DMA, interrupts, packet I/O, networking, sockets, and SSH
remain out of scope; a driver path still requires a separate source-backed
ownership contract and implementation task.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/capture-summary.json.
- Candidate run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/candidate-run/.
- Control run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/control-run/.
- Archive reviews:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/archive-review/.
- Pre-run snapshot:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/pre-run-snapshot-create.json.
- Final restore:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/final-post-restore-boot-files.json.

## Validation

- static inspection: accepted observed-window contract/core/closeout, touched
  runtime scenarios, archive helpers, capture summaries, identity joins, and
  docs reviewed.
- fmt check: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet rp1_ethernet passed.
- image/archive inspection: candidate and control review scripts passed.
- lab-controller API: hardwareTestLock acquired before publication; snapshot
  created and restored; final /boot/files confirmed restored tree.
- serial hardware output: candidate and control markers retained with
  run-unique nonces from direct-read serial windows.
- TFTP/capture evidence: candidate and control stable deltas both retained two
  expected da591740/kernel_2712.img fetches with matching bytes.
- JSON validation: task-owned JSON passed jq empty.
- docs validation: mdbook build passed after docs/src updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-observed-window-discriminator-proof-closeout-20260610 on
the next worker wake. The closeout must reconcile the accepted visible read and
decide the next explicit Phase 12.1 task without broadening into Ethernet
driver implementation, RP1 MMIO writes, DMA, descriptor rings, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.
