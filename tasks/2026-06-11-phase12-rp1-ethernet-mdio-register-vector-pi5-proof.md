# Phase 12 RP1 Ethernet MDIO Register Vector Pi 5 Proof

Task id: phase12-rp1-ethernet-mdio-register-vector-pi5-proof-20260611

Status: blocked

Classification: mdio-register-vector-publication-reverted-before-power-cycle-blocker

Evidence level: static archive review, lab-controller API identity/restore
evidence, serial hardware boot/output from a rejected capture, TFTP delta, and
capture-chain-v4 replay rejection. No accepted candidate/control hardware proof
was produced.

## Goal

Run the serialized Pi 5 corrected-target register-vector MDIO candidate and
paired no-MDIO/no-Ethernet control selected by the accepted guard closeout.

## Scope Performed

- Added separate register-vector candidate/control boot scenarios and archive
  review scripts.
- Performed static archive review for candidate, control, and candidate rerun
  archives.
- Acquired hardwareTestLock before lab archive publication, power actions, or
  runtime MDIO-related MMIO interaction.
- Published the candidate archive, captured the candidate attempt through
  capture-chain-v4, and restored the pre-run boot snapshot.
- Repeated the candidate publication with a new run-unique nonce after the
  first capture-chain rejection. The rerun used fresh identity/TFTP/serial
  cursors and recorded the same staging mismatch before any further code
  changes.
- Confirmed final lab boot files are restored to the pre-run baseline before
  releasing the hardware lock in supervisor state.

## Findings

- fixed: candidate runtime uses the corrected observed-window targets:
  MACB_MID context 0x1c001000fc, NCR 0x1c00100000, NSR 0x1c00100008, and MAN
  0x1c00100034.
- fixed: candidate preserves the no-NCR-write gate. It reads NCR first and
  performs MAN writes only when corrected NCR.MPE bit 4 is already set.
- fixed: candidate performs the selected six-register Clause 22 PHY1 vector:
  BMCR 0x60820000, BMSR 0x60860000, PHYSID1 0x608a0000, PHYSID2 0x608e0000,
  ANAR 0x60920000, and ANLPAR 0x60960000.
- fixed: paired control archive constructs no MDIO target or MAN frame and has
  no runtime volatile access intent.
- blocked: both candidate capture attempts were rejected by capture-chain-v4
  because the selected/published 52,352-byte candidate tree was not the tree
  served or reported after power. TFTP and final identity showed the restored
  104,136-byte baseline instead.
- blocked: the first candidate serial stream did contain a run marker and a
  visible register vector, but that observation is rejected as acceptance
  evidence because the capture chain tied TFTP/final identity to the wrong boot
  tree.
- deferred: control hardware execution, accepted candidate/control proof,
  PHY reset/GPIO32 ownership, broad MDIO/PHY ownership, Ethernet driver
  behavior, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain future explicit work.
- removed: no obsolete source, docs, or evidence was removed.
- not-an-issue: failed hardware captures are retained as evidence, not treated
  as accepted MDIO register-vector proof.

## Rejected Candidate Observation

The first candidate serial window included the intended marker and reported:

~~~text
classification=mdio-phy1-register-vector-visible
ncr-before=0x10
ncr-mpe-precondition-met=true
ncr-after=0x10
ncr-mpe-write-performed=false
bmcr=0x1000
bmsr=0x7949
physid1=0x600d
physid2=0x84a2
anar=0x1e1
anlpar=0x0
completed-register-count=6
man-after-vector=0x60821000,0x60867949,0x608a600d,0x608e84a2,0x609201e1,0x60960000
man-writes-performed=true
touched-fields=MAN
~~~

This observation is not accepted because both candidate v4 checks classified
the run as capture-staging-blocked with:

~~~text
tftp-expected-fetch-byte-mismatch
final-pre-restore-selected-tree-mismatch
final-pre-restore-expected-fetch-byte-mismatch
~~~

## Blocker

The lab publication path reported a selected candidate tree with a 52,352-byte
kernel_2712.img. After power, the TFTP delta and final boot identity reported
the restored baseline tree with a 104,136-byte kernel_2712.img. The rerun used
a fresh candidate archive and nonce and reproduced the same mismatch.

The worker cannot accept the proof or run the paired control as meaningful
acceptance evidence until supervisor planning provides a bounded staging/power
recovery gate or recovery task. This task stops at the precise publication to
power-cycle identity blocker and does not infer a phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof/classification.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof/capture-summary.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof/evidence-map.json.
- First candidate capture-chain replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof/candidate-run/v4-check.json.
- Candidate rerun capture-chain replay:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof/candidate-rerun/v4-check.json.
- Candidate rerun direct TFTP after power:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof/candidate-rerun/tftp-delta-direct-after-power.json.
- Candidate rerun current boot files after power:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof/candidate-rerun/current-boot-files-after-power.json.
- Candidate/control/rerun archive review:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-pi5-proof/archive-review/.

## Validation

- static archive review: candidate, control, and candidate-rerun archive reviews
  passed.
- fmt: cargo fmt --all.
- unit tests: cargo -Zjson-target-spec test --quiet mdio.
- lab-controller API: snapshot, publish, power-cycle, final identity, TFTP
  delta, and restore evidence retained.
- serial hardware boot/output: first candidate serial marker retained but
  rejected as acceptance evidence by capture-chain identity mismatch.
- capture-chain-v4 replay: candidate and candidate rerun both classified
  capture-staging-blocked with expected-fetch and selected-tree mismatches.
- JSON validation: jq empty on task-owned classification/evidence-map/
  capture-summary JSON.
- diff check: git diff --check.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- hardwareTestLock was acquired before lab interaction and released only after
  current boot files showed the restored baseline: satisfied.
- Candidate/control accepted capture-chain-v4 hardware proof: blocked by
  publication-to-power-cycle identity mismatch.
- Candidate performed no NCR write and only performed MAN writes after
  corrected NCR.MPE was observed set in the rejected serial capture: satisfied
  for the rejected observation only.
- Control hardware proof: not run because candidate staging identity was
  blocked.
- Classification does not expand to broad MDIO/PHY ownership, PHY reset
  ownership, Ethernet driver readiness, interrupts, DMA/descriptors, packet
  I/O, networking, sockets, SSH, Phase 12.2, or phase transition: satisfied.

## Next Action

Supervisor planning required. Define a bounded lab staging/power-cycle recovery
or discriminator task before retrying this candidate/control hardware proof.
