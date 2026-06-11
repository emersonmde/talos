# Phase 12 RP1 Ethernet MDIO MPE Enable Pi 5 Proof

Task id: phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof-20260611

Status: accepted

Classification: rp1-ethernet-mdio-mpe-enable-already-set-restored

Evidence level: static archive review, lab-controller API identity/restore
evidence, serial hardware boot/output, TFTP delta, and capture-chain-v4 replay.

## Goal

Run one serialized Pi 5 proof for the exact accepted NCR.MPE
set/readback/restore ownership sequence, with paired control and full restore
evidence.

## Scope Performed

- Added the bounded candidate and paired no-MDIO/no-Ethernet control boot
  scenarios and archive/review scripts for the accepted NCR.MPE proof.
- Acquired hardwareTestLock before archive publication, staging, power action,
  or runtime RP1 MMIO writes.
- Published and captured candidate/control runs with run-unique nonces through
  capture-chain-v4, then restored the pre-run boot tree before releasing the
  lock.
- Retained earlier incomplete/staging-contaminated captures as superseded
  evidence only; accepted evidence is candidate-rerun4 and control-rerun5.

## Findings

- fixed: candidate/control archive scripts and boot scenarios build the exact
  NCR.MPE set/readback/restore discriminator and no-MDIO control selected by
  the guard closeout.
- fixed: candidate-rerun4 passed capture-chain-v4 with selected-tree hash
  189e08f726e10ebc42c5fe198c84ac8e6fed80cf5e7a763ab02faa8be0eeee7b,
  expected fetch da591740/kernel_2712.img at 49792 bytes, run-unique serial
  nonce mpe-candidate-rerun4-20260611T1055Z, stable TFTP delta, final
  pre-restore identity, and restore evidence.
- fixed: control-rerun5 passed capture-chain-v4 with selected-tree hash
  93ff149fdd7a5565f6b6a4b5a92c054ed02879cb32ccc5b6443eb2a0e606653c,
  expected fetch da591740/kernel_2712.img at 48896 bytes, run-unique serial
  nonce mpe-control-rerun5-20260611T1058Z, stable TFTP delta, final
  pre-restore identity, and restore evidence.
- fixed: candidate-rerun4 observed MACB_MID context 0x70109 at 0x1c001000fc
  and NCR 0x10 at 0x1c00100000. The candidate performed only the accepted
  NCR.MPE write/readback/restore sequence: pre_raw 0x10, write_value 0x10,
  post_raw 0x10, restore_raw 0x10, restore_eq_pre=true.
- fixed: paired control-rerun5 constructed no NCR/MPE target, performed no
  volatile load/store, no MAN write, and no PHY-ID read, while preserving the
  same report path.
- deferred: MAN transactions, PHY-ID retry, PHY reset/GPIO32 ownership,
  broad MDIO/PHY ownership, Ethernet driver behavior, DMA/descriptors,
  interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future supervisor-owned work.
- not-an-issue: GET / returned 404 and was retained as endpoint-semantics
  evidence; capture-chain-v4 used /boot/files as the selected-tree identity
  source.
- removed: no source or evidence files were removed.

## Accepted Evidence

Candidate rerun4:

~~~text
classification=rp1-ethernet-mdio-mpe-enable-already-set-restored
capture-chain-v4=capture-chain-v4-ready
observed-window-macb-mid-context-raw=0x70109
ncr-observed-target=0x1c00100000
pre-raw=0x10
write-value=0x10
post-raw=0x10
restore-raw=0x10
restore-eq-pre=true
ncr-mpe-write-performed=true
ncr-restore-write-performed=true
man-writes-performed=false
phy-id-reads-performed=false
touched-fields=MACB/GEM_NCR.MPE
~~~

Control rerun5:

~~~text
classification=no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control
capture-chain-v4=capture-chain-v4-ready
target=none
ncr-observed-target=not-constructed
ncr-mpe-write-performed=false
ncr-restore-write-performed=false
man-writes-performed=false
phy-id-reads-performed=false
touched-fields=none
~~~

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/classification.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/evidence-map.json.
- Candidate rerun4 capture-chain replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/candidate-rerun4/v4-check.json.
- Control rerun5 capture-chain replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/control-rerun5/v4-check.json.
- Candidate/control archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/archive-review/.
- Final lab restore evidence:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/post-control-rerun5-boot-files.json.

## Validation

- static archive review: candidate-rerun4 and control-rerun5 archive reviews
  passed.
- lab-controller API: snapshot, publish, power-cycle, final identity, and
  restore evidence retained for candidate/control.
- serial hardware boot/output: candidate/control rerun serial markers retained
  with run-unique nonces.
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
  identity, restore, and JSON evidence requirements are satisfied by accepted
  reruns.
- Candidate classification is limited to the accepted NCR.MPE
  set/readback/restore boundary and records before/after/restored NCR values:
  satisfied.
- Control proves the report path without constructing NCR/MPE targets or write
  intent: satisfied.
- Classification does not expand to MAN writes, PHY-ID reads, broad MDIO/PHY
  ownership, PHY reset/GPIO32 ownership, Ethernet driver readiness, interrupts,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
  transition: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout-20260611 on the next worker
wake if this proof is accepted and committed. The closeout must reconcile the
accepted NCR.MPE already-set/restored proof and decide same-shaped retry policy
without expanding into MAN transactions, PHY-ID reads, or broad MDIO/PHY
ownership.
