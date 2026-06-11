# Phase 12 RP1 Ethernet MDIO PHY ID After-MPE Pi 5 Proof

Task id: phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof-20260611

Status: accepted

Classification: mdio-phy1-physid-after-mpe-visible

Evidence level: static archive review, lab-controller API identity/restore
evidence, serial hardware boot/output, TFTP delta, and capture-chain-v4 replay.

## Goal

Run the serialized Pi 5 corrected-target after-MPE MDIO PHY-ID candidate and
paired no-MDIO/no-Ethernet control selected by the accepted guard closeout.

## Scope Performed

- Added separate after-MPE candidate/control boot scenarios and archive/review
  scripts instead of reusing the earlier wrong-target PHY-ID proof path.
- Performed static archive review before lab publication.
- Acquired hardwareTestLock before lab archive publication, power action, or
  runtime MDIO-related MMIO interaction.
- Published and captured candidate/control runs through capture-chain-v4, then
  restored the pre-run boot snapshot before releasing the lock.
- Retained final lab identity proving the boot tree was restored to the
  pre-run baseline.

## Findings

- fixed: candidate runtime now uses corrected observed-window MACB/GEM targets:
  MACB_MID context 0x1c001000fc, NCR 0x1c00100000, NSR 0x1c00100008, and MAN
  0x1c00100034.
- fixed: candidate preserved the no-NCR-write gate. It pre-read corrected NCR
  as 0x10, observed NCR.MPE bit 4 set, and performed no NCR write.
- fixed: candidate performed the accepted bounded MAN sequence only after the
  MPE gate passed. It wrote PHYSID1 frame 0x600a0000 and PHYSID2 frame
  0x600e0000, polled NSR.IDLE, and extracted MAN.DATA as physid1 0xffff and
  physid2 0xffff.
- fixed: candidate/control both passed capture-chain-v4 with selected-tree
  identity, run-unique serial markers, stable TFTP deltas, final pre-restore
  identity, and restore evidence.
- fixed: paired control constructed no MDIO target or MAN frame, performed no
  volatile load/store, and classified as
  no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control.
- deferred: PHY responsiveness beyond the returned 0xffff/0xffff data, PHY
  reset/GPIO32 ownership, broad MDIO/PHY ownership, Ethernet driver behavior,
  interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain future explicit work.
- not-an-issue: the candidate's claims-runtime-mdio-transaction=true is
  limited to the selected MAN transaction sequence; it does not claim broad
  MDIO/PHY ownership or Ethernet readiness.
- removed: no obsolete source, docs, or evidence was removed.

## Accepted Evidence

Candidate:

~~~text
classification=mdio-phy1-physid-after-mpe-visible
capture-chain-v4=capture-chain-v4-ready
observed-window-macb-mid-context-raw=0x70109
ncr-before=0x10
ncr-mpe-precondition-met=true
ncr-mpe-write-performed=false
nsr-before-physid1=0x6
man-after-physid1=0x600bffff
physid1=0xffff
physid1-valid=true
nsr-before-physid2=0x6
man-after-physid2=0x600fffff
physid2=0xffff
physid2-valid=true
man-writes-performed=true
touched-fields=MAN
~~~

Control:

~~~text
classification=no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control
capture-chain-v4=capture-chain-v4-ready
target=none
man-writes-performed=false
claims-runtime-mdio-transaction=false
touched-fields=none
~~~

This accepts only the corrected-target after-MPE MAN transaction and MAN.DATA
return boundary under capture-chain-v4. It does not accept PHY reset ownership,
link state, Ethernet driver readiness, interrupts, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/classification.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/evidence-map.json.
- Candidate capture-chain replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/candidate-run/v4-check.json.
- Control capture-chain replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/control-run/v4-check.json.
- Candidate/control archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/archive-review/.
- Final lab restore evidence:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/final-lab-status-before-lock-release.json.

## Validation

- static archive review: candidate/control archive reviews passed.
- lab-controller API: snapshot, publish, power-cycle, final identity, and
  restore evidence retained for candidate/control.
- serial hardware boot/output: candidate/control serial markers retained with
  run-unique nonces.
- TFTP delta: candidate/control stable same-cursor deltas retained with
  expected fetch path and byte counts.
- capture-chain-v4 replay: candidate/control v4-check.json passed with empty
  rejection reasons.
- JSON validation: jq empty on task-owned classification/evidence-map/
  capture-summary JSON.
- diff check: git diff --check.
- documentation build: mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- hardwareTestLock was acquired before lab interaction and released after
  restore evidence: satisfied.
- Candidate/control capture-chain-v4 identity, TFTP, serial freshness, final
  identity, restore, and JSON evidence requirements are satisfied.
- Candidate performed no operation outside the accepted sequence: satisfied;
  MAN writes occurred only after corrected NCR.MPE was observed set.
- Control performed no MDIO target construction and no volatile load/store:
  satisfied.
- Classification does not expand to broad MDIO/PHY ownership, PHY reset
  ownership, Ethernet driver readiness, interrupts, DMA/descriptors, packet
  I/O, networking, sockets, SSH, Phase 12.2, or phase transition: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout-20260611 on the next
worker wake if this proof is accepted and committed. The closeout must
reconcile the visible 0xffff/0xffff MAN.DATA result without expanding into PHY
reset, broad MDIO/PHY ownership, Ethernet driver behavior, networking, SSH, or
Phase 12.2.
