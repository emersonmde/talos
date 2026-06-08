# Phase 11 RP1 PCIe Endpoint/Config Discriminator Control Pi 5

Task id: phase11-rp1-pcie-endpoint-config-discriminator-control-pi5-20260608

Status: accepted

Classification: no-mmio-pcie2-host-link-status-control-visible

## Goal

Prove the paired no-MMIO/no-RP1/no-GIC PCIe2 host-link-status control output
shape is visible on Pi 5 before any real endpoint/config/decode discriminator
proof.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Published only the accepted no-MMIO/no-RP1/no-GIC control archive:
  target/talos-rpi5-rp1-pcie2-host-link-status-no-mmio-control-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after an initial candidate
  capture was rejected by identity freshness: candidate identity, fresh
  serial/TFTP evidence, known-good control, and unchanged candidate rerun.

## Non-Goals

No real RP1 endpoint/config/decode candidate, RP1 peripheral/SYSINFO/clock/GPIO
or GIC retry, PCIe writes, clock/reset writes, GPIO/RIO/pad writes, event
generation, interrupt enablement or delivery, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Classification

Accepted as no-mmio-pcie2-host-link-status-control-visible.

The accepted control rerun selected boot tree
6a9df112f442b9d296a684dc7eaabac9a0fcc50ce19d96cca8d65bfddbe8813f with
effective kernel_2712.img and a 46,672-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
46,672-byte candidate fetches, final pre-restore identity still matched the
selected tree, and serial hardware output retained 118 occurrences of
TALOS: rp1-pcie2-host-link-status-control.

The retained control output classification remains
no-mmio-pcie2-host-link-status-control-visible. This accepts only the
no-MMIO/no-RP1/no-GIC output shape and capture path for the queued real
PCIE_MISC_PCIE_STATUS proof. Real RP1 endpoint/config/decode behavior, broad
RP1 mapping, endpoint ownership, PCIe writes, clock/reset ownership, GPIO
ownership, event generation, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, and phase transition remain
unaccepted.

The capture helper restored the candidate snapshot after the accepted rerun,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO/no-RP1/no-GIC
  control archive, including archive SHA-256, kernel SHA-256, marker string,
  report-shape strings, and forbidden real/RP1/GIC/MMIO strings absence.
- fixed: retained an initial candidate capture as capture-staging-blocked
  evidence; it had control marker output, but identity join rejected it because
  the pre-power serial drain was non-empty and the fresh TFTP/final identity
  fields did not join to the selected candidate tree.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate evidence; it retained two served 104,136-byte
  known-good kernel fetches and passed the v2 identity join.
- fixed: reran the same no-MMIO/no-RP1/no-GIC control candidate after the
  known-good control; the rerun passed the v2 identity join and retained
  repeated PCIe2 host-link-status control output.
- deferred: the real PCIE_MISC_PCIE_STATUS proof remains queued and must pass
  its own hardware lock, identity join, restore, and classification gates.
- not-an-issue: no real PCIe2 host-link, endpoint/config/decode, RP1 aperture,
  interrupt, or DMA behavior is inferred from a no-MMIO simulated control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-control-pi5/control-rerun-after-kg/.
- Initial candidate capture:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-control-pi5/control-run/.
- Known-good control:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-control-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 46,672-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 118 occurrences of
  TALOS: rp1-pcie2-host-link-status-control were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-pcie2-host-link-status-control-visible. The queued real
PCIE_MISC_PCIE_STATUS proof is mechanically unblocked on a future worker wake
if hardwareTestLock remains unlocked/restored and supervisorIntervention
remains inactive.
