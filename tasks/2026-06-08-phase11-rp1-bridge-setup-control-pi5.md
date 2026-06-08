# Phase 11 RP1 Bridge/Setup Control Pi 5

Task id: phase11-rp1-bridge-setup-control-pi5-20260608

Status: accepted

Classification: no-mmio-pcie2-bridge-setup-state-control-visible

## Goal

Prove the paired no-MMIO/no-PCIe/no-RP1/no-GIC bridge/setup-state control
output shape on Pi 5 before the real bridge/setup-state proof.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Published only the accepted no-MMIO/no-PCIe/no-RP1/no-GIC control archive:
  target/talos-rpi5-rp1-bridge-setup-state-no-mmio-control-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after capture-staging evidence:
  candidate identity, fresh serial/TFTP evidence, known-good control, bounded
  serial drain, clean known-good control, and an unchanged candidate rerun.

## Non-Goals

No real bridge/setup-state candidate, no hardware MMIO path, no endpoint config
retry, BAR discovery or programming, endpoint ownership claim, bridge setup
writes, CPU-to-PCIe window programming, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Classification

Accepted as no-mmio-pcie2-bridge-setup-state-control-visible.

The accepted candidate rerun selected boot tree
2a45a5ba915f03e515c2e4a3e0aca891d25c0d99c37af10b6b6a11d5e1722e95 with
effective kernel_2712.img and a 49,496-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
49,496-byte candidate fetches, final pre-restore identity still matched the
selected tree, and restore returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The retained serial output contains 30 occurrences of
TALOS: rp1-bridge-setup-state-control and the paired-control terminal
classification no-mmio-pcie2-bridge-setup-state-control-visible. This accepts
only the no-MMIO/no-PCIe/no-RP1/no-GIC output shape and capture path for the
queued real bridge/setup-state proof. Real PCIE_MISC_PCIE_STATUS,
PCIE_MISC_MISC_CTRL, root-complex class-code, outbound-window behavior,
endpoint visibility, endpoint ownership, BAR discovery/programming, interrupt
delivery, DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3,
and phase transition remain unaccepted.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO/no-PCIe/no-RP1/no-GIC
  control archive, including archive SHA-256, kernel SHA-256, kernel size, and
  control marker.
- fixed: retained the first candidate capture as capture-staging-blocked
  evidence; it had serial marker output and matching candidate TFTP fetches,
  but the pre-power serial drain was not empty at a saturated cursor, so no
  decisive classification was taken from that run.
- fixed: ran known-good production-timer controls after the inconclusive
  candidate evidence. The clean-drain known-good control passed the v2 identity
  join with two served 104,136-byte known-good kernel fetches and retained PASS
  output.
- fixed: reran the unchanged no-MMIO/no-PCIe/no-RP1/no-GIC control candidate
  after the clean known-good control; the rerun passed the v2 identity join and
  retained repeated bridge/setup-state control output.
- deferred: the real bridge/setup-state proof remains queued and must pass its
  own hardware lock, identity join, restore, and classification gates.
- not-an-issue: no real PCIe bridge/setup-state, endpoint config, RP1 aperture,
  interrupt, GPIO, clock/reset, or DMA behavior is inferred from a no-MMIO
  simulated control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-control-pi5/control-rerun-after-clean-kg/.
- Initial candidate capture:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-control-pi5/control-run/.
- Known-good controls:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-control-pi5/known-good-control-after-inconclusive/,
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-control-pi5/known-good-control-after-serial-drain/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 49,496-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 30 occurrences of
  TALOS: rp1-bridge-setup-state-control were retained.
- known-good control and unchanged candidate rerun after inconclusive evidence:
  run and retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-pcie2-bridge-setup-state-control-visible. The queued real
bridge/setup-state proof is mechanically unblocked on a future worker wake if
hardwareTestLock remains unlocked/restored and supervisorIntervention remains
inactive.
