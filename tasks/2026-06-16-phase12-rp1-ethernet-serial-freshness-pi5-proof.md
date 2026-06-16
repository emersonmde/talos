# Phase 12.1 RP1 Ethernet Serial Freshness Pi 5 Proof

Task:
phase12-rp1-ethernet-serial-freshness-pi5-proof-20260616.

Status: accepted

Classification:
serial-cursor-freshness-proved.

Evidence level: serial hardware boot/output, lab-controller API,
same-power-cycle TFTP evidence, task-owned serial freshness guard replay,
static archive review, JSON validation, docs build, and diff checks. No
BCM54213PE register read, MII_CTRL1000/MII_STAT1000 retry, GPIO32 event
clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux access, interrupt
ownership, PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, or
phase transition was performed.

## Goal

Run one serialized Pi 5 proof of the accepted serial freshness guard using a
no-MDIO/no-Ethernet marker boundary, then classify whether cursor-based capture
makes post-power serial evidence decisive.

## Findings

- fixed: HardwareTestLock was acquired before publication and released only
  after restore proof. The selected tree
  f73c75438663373b3d6df4e0ce451a45f163c4a582d8ba84bd79d161cf9cc68f was
  restored to baseline
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: The marker-only kernel-entry archive retained the run nonce
  serial-freshness-20260616T085546Z, archive SHA-256
  88fb6082621de1b204e7d42a5a4a67245cee6b7825fa9704da52e30a57cc4d0e, kernel
  SHA-256 2b1d15d9d49a423c87f9b824a0a31c2f4c2d161e968b8ed0e96da8b10a1da397,
  and a 47,352-byte kernel_2712.img.
- fixed: The Pi 5 run retained selected-tree identity, two matching
  da591740/kernel_2712.img TFTP serves at 47,352 bytes, final pre-restore
  selected-tree identity, and post-restore baseline identity.
- fixed: The pre-power retained serial sample and bounded drain both ended at
  saturated cursor 4194304 without the run nonce. The post-power saturated
  direct-read fallback retained the run-unique marker and nonce 45 times,
  satisfying cursor-nonce-post-power-freshness-v1.
- fixed: The task-owned guard accepted the retained bundle as
  serial-freshness-guard-v1-ready and rejected no freshness, TFTP, final
  identity, or restore classes for this run.
- not-an-issue: A preflight-only check after restore observed baseline identity
  instead of the selected tree. That check performed no power-cycle; its
  mismatched preflight output is retained separately, and the accepted proof
  uses the retained same-power-cycle selected-tree, TFTP, serial, final
  pre-restore, and restore evidence.
- rejected: BCM54213PE register values, link readiness, Ethernet readiness,
  GPIO32/PHY reset ownership, BMCR writes, Broadcom shadow/MMD/aux access,
  interrupt ownership, packet I/O, networking, SSH, Phase 12.2, and phase
  transition remain rejected.
- removed: No task-owned source files were removed.

## Classification

serial-cursor-freshness-proved.

The accepted result is limited to serial freshness and capture-chain identity
for a marker-only no-MDIO/no-Ethernet Pi 5 proof. It does not accept Ethernet
register values, link readiness, packet I/O, networking, SSH, Phase 12.2, or a
phase transition.

## Evidence

- classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/classification.json.
- evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/evidence-map.json.
- guard replay:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/candidate/serial-freshness-guard.json.
- capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/candidate/capture-invariant-summary.json.
- TFTP delta:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/candidate/tftp-delta-stable-pre-restore.json.
- restore proof:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-serial-freshness-pi5-proof/candidate/restore-snapshot.json.

## Validation

- static archive review: scripts/rpi5-archive-review.sh passed.
- lab-controller API: PUT /boot/archive, GET /boot/files, POST
  /boot/snapshot, POST /boot/restore passed.
- serial hardware boot/output: retained marker and nonce 45 times after the
  saved cursor/saturated direct-read fallback.
- TFTP evidence: stable same-cursor delta retained two matching
  da591740/kernel_2712.img serves at 47,352 bytes.
- task-owned guard: scripts/rpi5-serial-freshness-guard-v1-check.sh accepted
  the retained bundle.
- JSON validation: jq empty on task-owned JSON evidence passed.
- diff whitespace check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed before commit.

## Acceptance Check

- Candidate identity from lab API, fresh serial cursor, TFTP delta, selected
  tree identity, expected kernel path/size/hash, post-power observe/read output
  from the saved cursor boundary, final pre-restore identity, restore evidence,
  and hardware lock release are recorded: satisfied.
- Inconclusive triage classes are distinguished by the guard and no class fired
  for this run: satisfied.
- No same-shaped hardware retry was performed; the only post-restore check was
  preflight-only and retained as non-decisive evidence: satisfied.
- Rejected Ethernet/register/networking/Phase 12.2 claims remain explicit:
  satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-serial-freshness-closeout-20260616 on the next worker wake
if dependencies remain satisfied and hardwareTestLock remains unlocked/restored.
Do not start register-read retry, packet I/O, networking, SSH, Phase 12.2, or a
phase transition from this proof.
