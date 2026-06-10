# Phase 12 RP1 Ethernet GEM MID Decode Discriminator V2 Pi 5 Proof

Task: phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof-20260610

Status: accepted

Classification: observed-rp1-positive-control-gem-mid-0x1f-window-sentinel

Evidence level: image/archive inspection, lab-controller boot archive
publication/snapshot/restore evidence, serial hardware output, TFTP/capture
evidence, and post-run restore evidence. The hardware result is accepted only
as a repaired capture-chain discriminator proof of the retained 0x1f GEM MID
sentinel, not as Ethernet readiness.

## Scope

- Promoted the queued serialized Pi 5 proof after the accepted capture-chain
  repair closeout selected it and hardwareTestLock was unlocked.
- Acquired hardwareTestLock before boot archive publication or Pi 5 hardware
  interaction.
- Rebuilt the candidate and paired no-MMIO/no-Ethernet control archives with
  run-unique capture nonces.
- Published and captured the candidate/control runs with pi5-capture-chain-v4
  identity, serial, TFTP, final pre-restore, and restore gates.
- Restored snapshot pre-gem-mid-decode-discriminator-v2-proof-20260610T0516Z
  before releasing the hardware lock.
- Did not implement Ethernet behavior, program RP1 MMIO/DMA, create descriptor
  rings, claim interrupts/clock/reset/PHY ownership, perform packet I/O, add
  networking/sockets/SSH, start Phase 12.2, or claim a phase transition.

## Findings

- fixed: candidate archive review passed with nonce
  gemdisc-v2-candidate-20260610T051645Z-b4d146e9, archive sha256
  a26c7a6f71ed886bc58cc4d0e07d599c194fb81c5f4eb473fbaed650f8c367d4,
  kernel sha256
  255c89d8680b5fd6afff028e5a9aad402aecb3836021d2bc71391c574e7eecc8,
  and kernel_2712.img size 49176 bytes.
- fixed: control archive review passed with nonce
  gemdisc-v2-control-20260610T051645Z-b4d146e9, archive sha256
  fe8e3867ddb635a0efbaf34f0bcdc00529be50cda74deb726a05ac574bb49f8d,
  kernel sha256
  10b962002346cf08972d215d071a3c760a2eea0fac8aa485d5560d3682e20af3,
  and kernel_2712.img size 48432 bytes.
- fixed: candidate capture-chain-v4 identity joined selected tree
  99ba865fff0ce6829f0525fb0a20580205e8e12f5deab91139acf45d2d9f15b1,
  two matching TFTP fetches of da591740/kernel_2712.img at 49176 bytes,
  fresh serial nonce retention, final pre-restore identity, and restore proof.
- fixed: candidate serial retained 62 occurrences of the required marker, with
  observed SYSINFO positive-control raw 0x20001927 and translated MACB_MID raw
  0xdeaddead, classified as
  observed-rp1-positive-control-gem-mid-0x1f-window-sentinel.
- fixed: control capture-chain-v4 identity joined selected tree
  4a52e74fb0e9007d4689053c7296cdc6722e354fff78827f958f8c85870d6494,
  two matching TFTP fetches at 48432 bytes, nonce-fresh serial marker
  retention, final pre-restore identity, and restore proof without
  constructing RP1 or Ethernet MMIO targets.
- fixed: the final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  with effective kernel kernel_2712.img.
- deferred: the retained 0x1f001000fc GEM MID read still returns the
  0xdeaddead sentinel; PCIe/RP1 bridge/window enablement or a different
  accepted translation remains future work.
- not-an-issue: the control pre-power drain exhausted retained serial bytes,
  but pi5-capture-chain-v4 accepted the run because the run-unique control
  nonce was absent before power and present after power with matching
  identity/TFTP/final/restore gates.
- not-an-issue: no Ethernet runtime behavior was accepted.

No findings were removed.

## Hardware Result

Accepted result:
observed-rp1-positive-control-gem-mid-0x1f-window-sentinel.

The repaired capture chain decisively joined candidate identity, TFTP,
serial freshness, final pre-restore identity, restore evidence, and paired
control marker retention. The candidate observed RP1 SYSINFO_CHIP_ID at
0x1c00000000 as 0x20001927, while translated MACB_MID at 0x1f001000fc returned
0xdeaddead. This accepts the retained 0x1f window sentinel result under the
repaired evidence path. It does not accept live GEM visibility, Ethernet MMIO
readiness, or Ethernet driver readiness.

## Rejected Claims And Retained Risks

Rejected: live GEM visibility, broad Ethernet MMIO readiness, Ethernet driver
readiness, RP1 MMIO/DMA programming, descriptor rings, DMA ownership, transfer
completion, interrupt completion, clock/reset ownership, PHY reset ownership,
packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition.

Retained risks: PCIe/RP1 bridge or address-window enablement remains
unaccepted; Ethernet clock/reset and PHY/MDIO ownership remain unaccepted;
interrupts, DMA, descriptor rings, packet I/O, networking, sockets, and SSH
remain out of scope.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/capture-summary.json.
- Candidate run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/candidate-run/.
- Control run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/control-run/.
- Archive reviews:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/archive-review/.
- Pre-run snapshot:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/pre-run-snapshot-create.json.
- Final restore:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/final-restore-snapshot.json.

## Validation

- static inspection: accepted repair closeout, previous blocker, candidate and
  control archive reviews, capture summaries, and identity-join outputs
  reviewed.
- image/archive inspection: candidate and control review scripts passed.
- lab-controller API: hardwareTestLock acquired before publication; snapshot
  created and restored; final /boot/files confirmed the restored tree.
- serial hardware output: candidate and control markers retained with
  run-unique nonces from saturated direct-read serial windows.
- TFTP/capture evidence: candidate and control stable deltas both retained two
  expected fetches with matching bytes.
- JSON validation: task-owned JSON passed jq empty.
- docs validation: mdbook build passed after docs/src updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout-20260610 on the
next worker wake. The closeout must reconcile this accepted sentinel result
and decide the next explicit Phase 12.1 task without broadening into Ethernet
driver implementation, RP1 MMIO/DMA programming, descriptor rings, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.
