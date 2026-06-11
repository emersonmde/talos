# Phase 12 RP1 Ethernet MDIO PHY-ID Pi 5 Proof

Task id: phase12-rp1-ethernet-mdio-phy-id-pi5-proof-20260611

Status: accepted

Classification: mdio-phy1-physid-source-contract-violated-blocker

Evidence level: static archive review, lab-controller API identity/restore
evidence, serial hardware boot/output, TFTP delta, and capture-chain-v4 replay.

## Goal

Run the serialized Pi 5 candidate/control MDIO/PHY-ID discriminator proof only
after the accepted guard closeout authorized exact hardware gates.

## Scope Performed

- Added the bounded candidate and paired no-MDIO/no-Ethernet control boot
  scenarios and archive/review scripts for the accepted MDIO PHY-ID proof.
- Performed local static archive review before lab publication.
- Acquired hardwareTestLock before lab archive publication, staging, power
  action, or runtime MDIO-related MMIO interaction.
- Published and captured candidate/control runs through capture-chain-v4, then
  restored the pre-run boot snapshot before releasing the lock.
- Retained the first candidate/control run as superseded evidence after it
  exposed a report-shape bug, then fixed the report and accepted rerun2.

## Findings

- fixed: candidate/control archive scripts and boot scenarios now build the
  exact MDIO PHY-ID discriminator and no-MDIO control selected by the accepted
  guard closeout.
- fixed: build.rs scenario registration and RP1 volatile helper cfg gates now
  include the MDIO PHY-ID candidate.
- fixed: the first hardware run passed capture-chain-v4 but the candidate
  report overclaimed claims-runtime-mdio-transaction=true and touched-fields=MAN
  even though NCR.MPE was clear and no MAN write occurred. Rerun2 fixed the
  report to claims-runtime-mdio-transaction=false and touched-fields=none.
- fixed: candidate rerun2 passed capture-chain-v4 with selected-tree hash
  5bc71791d508809a451aead9ebe643de5114a0647e5a8fa8670dd16d24c694a7,
  expected fetch da591740/kernel_2712.img at 50120 bytes, run-unique serial
  nonce mdio-phyid-candidate-rerun2-20260611T0915Z, stable TFTP delta, final
  pre-restore identity, and restore evidence.
- fixed: control rerun2 passed capture-chain-v4 with selected-tree hash
  e2045684027bf936386454a7a24b0debc9b70c12a3675cb2355e95feb016d546,
  expected fetch da591740/kernel_2712.img at 49192 bytes, run-unique serial
  nonce mdio-phyid-control-rerun2-20260611T0915Z, stable TFTP delta, final
  pre-restore identity, and restore evidence.
- fixed: candidate rerun2 observed MACB_MID context 0x70109 at 0x1c001000fc
  and NCR 0x20001927 at 0x1c00000000. NCR.MPE bit 4 was clear, so the
  candidate classified as mdio-phy1-physid-source-contract-violated-blocker
  without writing NCR, MAN, GPIO32, or PHY reset state.
- fixed: paired control rerun2 constructed no MDIO target or MAN frame,
  performed no MDIO transaction, and classified as
  no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control.
- fixed: docs now record the accepted no-write NCR.MPE-clear blocker and
  correct the observed-window NCR/NSR/MAN targets to 0x1c00000000,
  0x1c00000008, and 0x1c00000034.
- deferred: visible PHY-ID reads, any future NCR.MPE write authority, broad
  MDIO/PHY ownership, PHY reset ownership, Ethernet runtime behavior,
  interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain future supervisor-owned work.
- not-an-issue: GET / returned 404 and was retained as endpoint-semantics
  evidence; capture-chain-v4 used /boot/files as the selected-tree identity
  source.
- removed: no source or evidence files were removed.

## Accepted Evidence

Candidate rerun2:

~~~text
classification=mdio-phy1-physid-source-contract-violated-blocker
capture-chain-v4=capture-chain-v4-ready
observed-window-macb-mid-context-raw=0x70109
ncr-before=0x20001927
ncr-mpe-precondition-met=false
ncr-mpe-write-performed=false
man-writes-performed=false
claims-runtime-mdio-transaction=false
touched-fields=none
~~~

Control rerun2:

~~~text
classification=no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control
capture-chain-v4=capture-chain-v4-ready
target=none
man-writes-performed=false
claims-runtime-mdio-transaction=false
touched-fields=none
~~~

The first candidate/control run is retained as superseded evidence only. It is
not used for acceptance because it overclaimed runtime MDIO/touched-field state
after the no-write precondition blocker.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/classification.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/evidence-map.json.
- Candidate rerun2 capture-chain replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/candidate-rerun2/v4-check.json.
- Control rerun2 capture-chain replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/control-rerun2/v4-check.json.
- Candidate/control archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/archive-review/.
- Final lab restore evidence:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/final-lab-status-before-lock-release.json.

## Validation

- static archive review: candidate/control archive reviews passed for rerun2.
- lab-controller API: snapshot, publish, power-cycle, final identity, and
  restore evidence retained for candidate/control.
- serial hardware boot/output: candidate/control rerun2 serial markers retained
  with run-unique nonces.
- TFTP delta: candidate/control rerun2 stable same-cursor deltas retained with
  expected fetch path and byte counts.
- capture-chain-v4 replay: candidate/control rerun2 v4-check.json passed with
  empty rejection reasons.
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
  identity, restore, and JSON evidence requirements are satisfied by rerun2.
- Candidate performed no operation outside the accepted sequence: satisfied;
  the no-write precondition blocked all MAN writes because NCR.MPE was clear.
- Control performed no MDIO transaction: satisfied.
- Classification does not expand to Ethernet driver readiness, broad MDIO/PHY
  ownership, PHY reset ownership, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, or phase transition: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-phy-id-proof-closeout-20260611 on the next worker
wake if this proof is accepted and committed. The closeout must reconcile the
accepted no-write NCR.MPE-clear blocker and decide same-shaped retry policy
without expanding into NCR.MPE write ownership or broad MDIO/PHY ownership.
