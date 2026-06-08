# Phase 11 RP1 Bridge/Config Preflight Control Pi 5

Task id: phase11-rp1-bridge-config-preflight-control-pi5-20260608

Status: accepted

Classification: no-mmio-pcie2-bridge-preflight-control-visible

## Goal

Prove the paired no-MMIO/no-PCIe/no-RP1/no-GIC bridge/config preflight control
output shape on Pi 5 before the real bridge/config preflight discriminator.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Published only the accepted no-MMIO/no-PCIe/no-RP1/no-GIC control archive:
  target/talos-rpi5-rp1-bridge-config-preflight-no-mmio-control-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after the first candidate
  capture was rejected as capture-staging evidence: candidate identity, fresh
  serial/TFTP evidence, known-good control, and unchanged candidate rerun.

## Non-Goals

No real discriminator hardware run, endpoint configuration mutation, BAR
programming, bridge setup, PERST/link-control change, MSI/MIP/GIC operations,
interrupt enablement or delivery, GIC IAR/EOIR acknowledgement, ISR
installation, RP1 clock/reset writes, GPIO/RIO/pad writes, event generation,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or phase
transition.

## Classification

Accepted as no-mmio-pcie2-bridge-preflight-control-visible.

The accepted candidate rerun selected boot tree
f4269bd6ed53338820122d469e56922f4741c3e0d191843090fc3eaef6819646 with
effective kernel_2712.img and a 47,504-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
47,504-byte candidate fetches, final pre-restore identity still matched the
selected tree, and serial hardware output retained 60 occurrences of
TALOS: rp1-bridge-config-preflight-control.

The retained control output classification remains
no-mmio-pcie2-bridge-preflight-control-visible. This accepts only the
no-MMIO/no-PCIe/no-RP1/no-GIC output shape and capture path for the queued real
bridge/config preflight proof. Real PCIE_MISC_MISC_CTRL behavior, endpoint
configuration mutation, broad bridge setup, interrupt delivery, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, and phase transition
remain unaccepted.

The capture helper restored the candidate snapshot after the accepted rerun,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO/no-PCIe/no-RP1/no-GIC
  control archive, including archive SHA-256, kernel SHA-256, marker string,
  report-shape strings, and forbidden real/RP1/GIC/MMIO strings absence.
- fixed: retained an initial candidate capture as capture-staging-blocked
  evidence because the pre-power serial drain was non-empty at a saturated
  cursor; no decisive classification was taken from that run.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate evidence; it passed the v2 identity join and retained
  two served 104,136-byte known-good kernel fetches.
- fixed: reran the same no-MMIO/no-PCIe/no-RP1/no-GIC control candidate after
  the known-good control; the rerun passed the v2 identity join and retained
  repeated bridge/config preflight control output.
- deferred: the real bridge/config preflight proof remains queued and must pass
  its own hardware lock, identity join, restore, and classification gates.
- not-an-issue: no real PCIe bridge config, endpoint config, RP1 aperture,
  interrupt, GPIO, clock/reset, or DMA behavior is inferred from a no-MMIO
  simulated control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-control-pi5/control-rerun-after-kg/.
- Initial candidate capture:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-control-pi5/control-run/.
- Known-good control:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-control-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,504-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 60 occurrences of
  TALOS: rp1-bridge-config-preflight-control were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-pcie2-bridge-preflight-control-visible. The queued real
bridge/config preflight proof is mechanically unblocked on a future worker wake
if hardwareTestLock remains unlocked/restored and supervisorIntervention
remains inactive.
