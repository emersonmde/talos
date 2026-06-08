# Phase 11 RP1 Bridge/Config Preflight Pi 5

Task id: phase11-rp1-bridge-config-preflight-pi5-20260608

Status: accepted

Classification: pcie2-bridge-preflight-ready

## Goal

Run the accepted real bridge/config-preflight discriminator on Pi 5 under
serialized evidence rules.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 real candidate work.
- Published only the accepted real candidate archive:
  target/talos-rpi5-rp1-bridge-config-preflight-read-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after capture-staging evidence:
  candidate identity, fresh serial/TFTP evidence, known-good control, and an
  unchanged candidate rerun after a bounded serial drain.

## Non-Goals

No operation outside the accepted source contract, same-shaped endpoint config
identity rerun, broad bridge setup, BAR programming, endpoint ownership claim,
endpoint configuration mutation, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Classification

Accepted as pcie2-bridge-preflight-ready.

The accepted candidate rerun selected boot tree
e66d21ac433225c19dfa63c09a577c8ab6828ebfdf5a437b57efc5fe0e7f260a with
effective kernel_2712.img and a 48,000-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
48,000-byte candidate fetches, final pre-restore identity still matched the
selected tree, and restore returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The retained serial output contains 123 occurrences of
TALOS: rp1-bridge-config-preflight-result and 123 occurrences of
classification=pcie2-bridge-preflight-ready. The accepted report reads
PCIE_MISC_PCIE_STATUS at 0x1000124068 as raw=0x3e0b0, with pcie-port=true,
dl-active=true, phylinkup=true, link-in-l23=false, and
status-is-deaddead=false. It then reads PCIE_MISC_MISC_CTRL at 0x1000124008
as raw=0xa8003000, with scb-access-en=true, cfg-read-ur-mode=true,
rcb-mps-mode=false, rcb-64b-mode=false, max-burst-size=0x0, and
misc-ctrl-is-sentinel=false.

This accepts only that the bounded source-contract bridge/config preflight
read reaches the PCIe2 host-link-up precondition and finds a non-sentinel
PCIE_MISC_MISC_CTRL value with SCB_ACCESS_EN and CFG_READ_UR_MODE set. It does
not accept endpoint ownership, expected RP1 vendor/device visibility, broad
RP1 mapping, endpoint configuration mutation, BAR discovery or programming,
bridge setup, PERST/link-control change, interrupt delivery, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  candidate work.
- fixed: retained static archive identity for the accepted real candidate
  archive, including archive SHA-256, kernel SHA-256, marker string, report
  shape, and accepted PCIE_MISC_PCIE_STATUS/PCIE_MISC_MISC_CTRL fields.
- fixed: retained the first candidate capture as capture-staging-blocked
  evidence; it had serial marker output and matching candidate TFTP fetches,
  but the pre-power serial drain was not empty at a saturated cursor, so no
  decisive classification was taken from that run.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate evidence; the first known-good control also remained
  capture-staging-blocked because the serial drain was still non-empty.
- fixed: performed a bounded manual serial drain to remove retained capture
  noise, then reran known-good control. The clean-drain known-good control
  passed the v2 identity join with two served 104,136-byte known-good kernel
  fetches and retained PASS output.
- fixed: reran the unchanged bridge/config preflight candidate after the
  clean known-good control; the rerun passed the v2 identity join and retained
  repeated pcie2-bridge-preflight-ready output.
- deferred: endpoint ownership, expected RP1 vendor/device visibility, broad
  RP1 mapping, endpoint configuration mutation, BAR discovery or programming,
  bridge setup, PERST/link control, interrupt delivery, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, and phase transition.
- not-an-issue: PCIE_MISC_MISC_CTRL readiness bits are treated as a preflight
  state discriminator, not as proof of endpoint ownership or bridge setup.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-pi5/candidate-rerun-after-clean-kg/.
- First candidate capture:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-pi5/candidate-run/.
- Initial known-good control that remained capture-staging-blocked:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-pi5/known-good-control-after-inconclusive/.
- Clean-drain known-good control:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-pi5/known-good-control-after-serial-drain/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted
  candidate rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 48,000-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 123 occurrences of
  TALOS: rp1-bridge-config-preflight-result were retained with
  classification=pcie2-bridge-preflight-ready.
- known-good control and unchanged candidate rerun after inconclusive
  evidence: run and retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as pcie2-bridge-preflight-ready. The queued closeout is mechanically
unblocked on a future worker wake if hardwareTestLock remains unlocked/restored
and supervisorIntervention remains inactive.
