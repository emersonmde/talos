# Phase 11 RP1 Bridge/Setup Pi 5

Task id: phase11-rp1-bridge-setup-pi5-20260608

Status: accepted

Classification: pcie2-bridge-setup-state-incomplete

## Goal

Run the committed real bridge/setup-state candidate on Pi 5 and classify the
result with the accepted source-contract vocabulary.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 bridge/setup-state work.
- Published only the committed real archive:
  target/talos-rpi5-rp1-bridge-setup-state-read-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and capture-transaction-v2 identity join.
- Applied inconclusive-run triage after the first real run: candidate
  identity, fresh serial/TFTP evidence, known-good control, bounded serial
  drain, clean known-good control, and unchanged candidate rerun.
- Updated roadmap and PCIe map contract docs for the accepted hardware
  frontier.

## Non-Goals

No source-contract expansion, endpoint config retry, endpoint ownership claim,
broad RP1 mapping claim, BAR discovery or programming, bridge setup writes,
CPU-to-PCIe window programming, PERST/link-control, interrupt delivery,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or phase
transition.

## Classification

Accepted as pcie2-bridge-setup-state-incomplete.

The accepted unchanged candidate rerun selected boot tree
9fbdcb57cd60519737902b9e3b85799e2479abffd8911a9ca887015a7f0f625a with
effective kernel_2712.img and a 50,736-byte da591740/kernel_2712.img. The
capture-transaction-v2 identity join passed with no rejection reasons: the
pre-power serial drain was empty, stable pre-restore TFTP retained two served
50,736-byte candidate fetches, final pre-restore identity still matched the
selected tree, and restore returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The retained serial output contains 90 occurrences of
TALOS: rp1-bridge-setup-state-result. It reports PCIE_MISC_PCIE_STATUS=0x3e0b0
with pcie_port, dl_active, and phylinkup true; PCIE_MISC_MISC_CTRL=0xa8003000
with SCB_ACCESS_EN and CFG_READ_UR_MODE true; and
PCIE_RC_CFG_PRIV1_ID_VAL3=0x30060400 with class code 0x060400. Outbound
window 0 registers are visible, but do not match the source-expected PCIe 0 ->
CPU 0x1f_0000_0000 shape: win0_lo=0x80000000, win0_hi=0x0,
win0_base_limit=0x3ff00000, win0_base_hi=0x1c, and win0_limit_hi=0x1c.
The terminal classification is pcie2-bridge-setup-state-incomplete.

This accepts only the identity-joined incomplete bridge/setup-state hardware
classification. pcie2-bridge-setup-state-visible, expected RP1 vendor/device
visibility, endpoint ownership, broad RP1 mapping, BAR discovery/programming,
bridge setup writes, PERST/link-control, interrupt delivery, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, and phase transition
remain unaccepted.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  bridge/setup-state work.
- fixed: retained static archive identity for the accepted real archive,
  including archive SHA-256 2ed822cd0b2e6491da6a6d9447456d83228a694358326b7a15a5ab663f251d17,
  kernel SHA-256 8a1c6a6cd64ecbc1228a4eda56a0cecbea041590c12bb0b60ba2674e0ac5a71b,
  kernel size 50,736 bytes, and result marker.
- fixed: retained the first real run as capture-staging-blocked evidence; it
  had serial marker output and matching candidate TFTP fetches, but pre-power
  serial drain was not empty at a saturated cursor.
- fixed: ran the required known-good production-timer controls after the
  inconclusive real run. The first known-good run was also
  capture-staging-blocked by non-empty pre-power serial drain.
- fixed: performed bounded manual serial drain and reran the known-good
  production-timer control; the clean known-good control passed identity join
  with two served 104,136-byte fetches and retained PASS output.
- fixed: reran the unchanged real bridge/setup-state candidate after the clean
  known-good control; it passed identity join and retained repeated
  bridge/setup-state result output.
- fixed: updated docs/src/roadmap.md and
  docs/src/project/phase11-rp1-pcie-map-contract.md with the accepted
  incomplete setup-state frontier and retained risks.
- deferred: endpoint visibility retry, endpoint ownership, broad RP1 mapping,
  BAR discovery/programming, bridge setup writes, interrupt delivery,
  DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, and
  phase transition.
- not-an-issue: pcie2 link/preflight and root-complex class-code visibility
  are accepted, but the outbound-window mismatch correctly prevents the
  visible setup-state classification.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-pi5/classification.json.
- Accepted real rerun:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-pi5/real-rerun-after-clean-kg/.
- Initial real run:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-pi5/real-run/.
- Known-good controls:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-pi5/known-good-control-after-inconclusive/,
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-pi5/known-good-control-after-serial-drain/.
- Manual serial drain:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-pi5/manual-serial-drain-before-clean-kg-summary.json.

## Validation

- static archive identity check: passed against the accepted core real archive.
- lab-controller serialized Pi 5 hardware run: passed on the accepted
  unchanged candidate rerun.
- capture-transaction-v2 identity join: passed on the accepted candidate rerun
  with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 50,736-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 90 occurrences of
  TALOS: rp1-bridge-setup-state-result were retained.
- known-good control and unchanged candidate rerun after inconclusive evidence:
  run and retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- docs validation: mdbook build passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as pcie2-bridge-setup-state-incomplete. The queued closeout task is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored and supervisorIntervention remains inactive.
