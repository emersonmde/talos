# Phase 12.1 RP1 Ethernet BootInfo Report Serial Visibility Pi 5 Proof

Task:
phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof-20260616.

Status: accepted with precise blocker

Classification:
staging-capture-inconclusive.

Evidence level: serial hardware boot/output, lab-controller API,
same-power-cycle TFTP evidence, static archive review, local source/test
validation, JSON validation, docs build, and diff checks. No BCM54213PE
register retry, MII_CTRL1000/MII_STAT1000 read, GPIO32 event clear/reset
recovery, BMCR write, Broadcom shadow/MMD/aux access, interrupt ownership,
PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Run the serialized Pi 5 proof for the dual-stage BootInfo/report-path serial
visibility discriminator before any Ethernet or MDIO behavior.

## Findings

- fixed: The first hardware run showed the one-shot earliest-entry marker could
  scroll out under saturated direct-read capture. The repeated control output
  retained the control nonce but not the exact earliest marker token, while the
  candidate retained the post-BootInfo marker. The proof instrumentation was
  repaired by adding a retained earliest-marker token to the repeated
  control/candidate report output and updating the static review scripts to
  require it.
- fixed: The final control archive retained the run nonce
  bootinfo-report-pi5-control-final-20260616T072750Z, archive SHA-256
  72101e6920699720c06fc9f86b6454fdce3c22997c1169be36514ed98eba369e,
  kernel SHA-256
  422de6ba1df1b690be98fe9b55273d0957daeea22c55f784868780f20931d399, and a
  55,120-byte kernel_2712.img.
- fixed: The final candidate archive retained the run nonce
  bootinfo-report-pi5-candidate-final-20260616T072750Z, archive SHA-256
  db0c701431f418803bc5edad9dde65f9fb82d8d6f22bef2f4a7aeffb57ddb551,
  kernel SHA-256
  9cc20664d3c7fce1a5816cd8e41c9033917ce40184346a3584dfcb708ecd9f70, and a
  71,168-byte kernel_2712.img.
- fixed: The final control run retained selected tree
  b886e168d26f69a943a98d77de87a40a7079938fa041aee8494e32cb98ea9178,
  two matching 55,120-byte da591740/kernel_2712.img TFTP serves, 71 earliest
  marker occurrences, zero post-BootInfo marker occurrences, final
  pre-restore identity, and restore proof.
- fixed: The final candidate run retained selected tree
  38173e8bd614d6034e09e4944e0d5e92ad80dcebafb78b260897be7f74cc8c19,
  two matching 71,168-byte da591740/kernel_2712.img TFTP serves, 69 earliest
  marker occurrences, 68 post-BootInfo marker occurrences, final pre-restore
  identity, and restore proof.
- blocked: Both final runs failed the capture-chain identity guard with
  serial-drain-not-empty-before-power. The 128-attempt pre-power serial drain
  still exhausted without an empty /serial/read response, so the marker evidence
  is retained as hardware output but not accepted as decisive proof by the
  current capture contract.
- rejected: No same-shaped retry is authorized without a new discriminator for
  the serial-drain/backlog invariant.
- rejected: BCM54213PE register values, link readiness, Ethernet readiness,
  GPIO32/PHY reset ownership, BMCR writes, Broadcom shadow/MMD/aux access,
  interrupt ownership, broad PHY/MAC configuration, packet I/O, networking,
  SSH, Phase 12.2, and phase transition remain rejected.
- removed: No task-owned source files were removed.

## Classification

staging-capture-inconclusive.

The first failing invariant is serial-drain-not-empty-before-power. Final
control and candidate runs show selected-tree identity, matching TFTP fetches,
marker serial output, final pre-restore identity, and restore proof, but the
identity guard rejects decisive classification because the pre-power serial
drain did not reach an empty read before either power-cycle.

## Evidence

- classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof/classification.json.
- evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof/evidence-map.json.
- final control capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof/control-final/capture-invariant-summary.json.
- final candidate capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bootinfo-report-serial-visibility-pi5-proof/candidate-final/capture-invariant-summary.json.
- initial and rerun evidence are retained under the same task evidence
  directory for the marker-retention and serial-backlog findings.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet
  rp1_ethernet_bootinfo_report_serial_visibility: pass.
- sh -n scripts/rpi5-rp1-ethernet-bootinfo-report-serial-visibility-*.sh:
  pass.
- static archive review for final control and candidate: pass.
- lab-controller API: PUT /boot/archive, GET /boot/files, POST
  /boot/snapshot, POST /boot/restore: pass.
- serial hardware boot/output: final control and candidate retained marker
  output, but decisive classification is blocked by
  serial-drain-not-empty-before-power.
- TFTP evidence: final control and candidate each retained two matching
  da591740/kernel_2712.img serves.
- JSON evidence validation: jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass because docs/src files were updated.
- git diff --cached --check before commit: pass after staging.

## Acceptance Check

- Hardware proof records candidate/control identity, fresh serial cursor, TFTP
  delta, final pre-restore identity, and restore evidence: satisfied.
- Candidate/control selected-tree identity and expected kernel path/size/hash
  are recorded and reconciled with served TFTP bytes: satisfied.
- Serial evidence classifies earliest-entry and post-BootInfo markers
  separately: retained, but decisive interpretation is blocked by the capture
  guard.
- Inconclusive blocker is precise and no same-shaped retry is authorized:
  satisfied.
- Rejected Ethernet/networking/phase-transition claims remain explicit:
  satisfied.
- Hardware lock was released/restored and docs/evidence/state updates are
  committed before the next task starts: satisfied after state update and
  commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bootinfo-report-serial-visibility-closeout-20260616 on the
next worker wake if the commit/state record is accepted and the hardware lock
remains unlocked/restored. The closeout must reconcile this precise blocker
before any new discriminator is planned.
