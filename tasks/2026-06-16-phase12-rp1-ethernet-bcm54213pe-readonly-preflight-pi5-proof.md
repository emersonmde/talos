# Phase 12 RP1 Ethernet BCM54213PE Read-Only Preflight Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof-20260616

Status: accepted

Classification: bcm54213pe-readonly-preflight-candidate-rerun-no-tftp-fetch-blocked

Evidence level: static archive/image review, lab-controller API evidence,
serial hardware boot/output, stable TFTP delta, boot-staging identity replay,
and restore proof.

## Goal

Run the serialized Pi 5 proof for the accepted BCM54213PE read-only preflight
hardware-proof core, limited to PHY1 MII_CTRL1000 0x09 and MII_STAT1000 0x0a
plus the paired no-MDIO/no-Ethernet control.

## Scope Performed

- Acquired the hardwareTestLock before archive publication and power-cycle work.
- Built run-unique control, first candidate, and one allowed candidate rerun
  boot archive from the accepted proof-core scenarios.
- Published, power-cycled, captured, and restored the lab around the hardware
  runs.
- Retained candidate/control identity, serial cursor, TFTP delta, and restore
  evidence.
- Ran the required triage order before the allowed candidate rerun: candidate
  identity, fresh serial cursor, TFTP delta, known-good control, then candidate
  rerun.

## Findings

- fixed: the no-MDIO/no-Ethernet control produced selected-tree identity
  ad6afd3218a37a509c31b587c107dedec7b33fcd48df4a43c6eab939f5cd2ad5, two
  matching 50536-byte da591740/kernel_2712.img TFTP fetches, 18 fresh serial
  marker occurrences, boot-staging-identity-ready, and restore to the baseline
  tree.
- blocked: the accepted candidate rerun published selected tree
  189219336873dd6f335fd3ad2f97bb20b8cb2f4a01e2635e4f3ae9dd5eacb5c8 with
  expected 51512-byte da591740/kernel_2712.img, but after power-cycle the TFTP
  delta was stable and empty, serial observe from the saturated cursor saw no
  fresh output, and boot-staging identity remained blocked.
- deferred: candidate raw/decoded PHY1 MII_CTRL1000 and MII_STAT1000 values
  remain deferred until the candidate power/network-fetch blocker is resolved.
- not-an-issue: GET / returns 404 in this lab deployment. The endpoint result is
  retained as evidence, and identity uses /status plus /boot/files per the
  deployed lab-controller docs.
- removed: generated boot archives were removed after upload; retained evidence
  keeps archive hashes, kernel hashes, byte counts, and lab captures.

## Control

Control classification:
no-mdio-no-ethernet-bcm54213pe-readonly-preflight-control.

Static review retained archive SHA-256
f92762ef0fe819faf7a953031fb5a8e69031e9b2d6c88f13e73fc860efd732da,
kernel SHA-256
7c0154d97f878a8867a3605ace9c86107658d42dd40ecf7415d8983dc6c4fa07,
kernel_2712.img size 50536 bytes, and nonce
bcm54213pe-preflight-20260616T023725Z-control.

Hardware capture retained two matching 50536-byte TFTP fetches for
da591740/kernel_2712.img and fresh serial output proving the no-MDIO/no-Ethernet
control shape. The runtime report constructs no MDIO target, MAN frame, MACB
target, GPIO target, RP1 Ethernet target facts, or volatile Ethernet access
intent.

## Candidate

Candidate rerun classification:
bcm54213pe-readonly-preflight-candidate-rerun-no-tftp-fetch-blocked.

Static review retained archive SHA-256
fb5983f27f3f7e80c48ce435bc6349d31a2063b1402f5233b78f1a4766970168,
kernel SHA-256
11d45951fc4454cf4ca2163b246db4277a8c02f4ffeee961247ccc0602f934f5,
kernel_2712.img size 51512 bytes, and nonce
bcm54213pe-preflight-20260616T024407Z-cand-rerun.

The candidate rerun pre-power identity selected tree
189219336873dd6f335fd3ad2f97bb20b8cb2f4a01e2635e4f3ae9dd5eacb5c8 and expected
da591740/kernel_2712.img at 51512 bytes. After power-cycle, the TFTP delta from
cursor 4516450 was stable and empty, serial observe retained no fresh bytes or
required marker, final pre-restore status still showed the selected candidate
tree, and restore returned the lab to baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Rejected Claims

The accepted result does not prove link readiness and does not authorize packet
I/O, networking, SSH, Phase 12.2, phase transition, GPIO32 reset ownership or
action, BMCR writes, Broadcom shadow/MMD/aux access, interrupt ownership, or
broad PHY/MAC configuration.

## Evidence

- Classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof/capture-summary.json.
- Control run:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof/control-run/.
- Candidate first attempt:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof/candidate-run/.
- Candidate rerun:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-pi5-proof/candidate-rerun/.

## Validation

- static archive/image review: control, first candidate, and candidate rerun
  archive reviews passed and recorded hashes/byte counts.
- lab-controller API: /status and /boot/files retained selected-tree identity,
  power-cycle records, restore records, final lab status, and final boot files.
- serial hardware boot/output: control passed with fresh marker output;
  candidate rerun blocked with stable no-output serial observe.
- stable TFTP delta: control passed with two matching 50536-byte fetches;
  candidate rerun blocked with stable empty delta.
- boot-staging identity replay: control passed; candidate rerun blocked.
- restore proof: final lab status returned to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- JSON validation: jq empty on task-owned JSON evidence.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src/roadmap.md was touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Candidate/control identity, serial cursor, TFTP delta, and restore evidence are
  retained and internally consistent: satisfied.
- Control proves no-MDIO/no-Ethernet behavior: satisfied.
- Candidate reports selected raw/decoded values or is blocked with a precise
  discriminator-backed reason: satisfied by the candidate rerun no-fresh-TFTP/no-
  serial-output blocker.
- At most one candidate rerun followed the inconclusive first candidate run:
  satisfied.
- Packet I/O, networking, SSH, Phase 12.2, phase transition, link readiness,
  GPIO32 reset ownership, BMCR writes, Broadcom shadow/MMD/aux access, interrupt
  ownership, and broad PHY/MAC configuration remain rejected: satisfied.
- Hardware lock release requires restore proof: satisfied once state is updated
  after this commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-hw-proof-closeout-20260616 on
the next worker wake if dependencies remain satisfied and hardwareTestLock is
unlocked/restored. The closeout should reconcile the control proof and candidate
no-fresh-TFTP/no-serial-output blocker without starting packet I/O, networking,
SSH, Phase 12.2, or a phase transition.
