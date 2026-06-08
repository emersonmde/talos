# Phase 11 RP1 PCIe Endpoint/Config Discriminator Pi 5

Task id: phase11-rp1-pcie-endpoint-config-discriminator-pi5-20260608

Status: accepted

Classification: pcie2-host-link-up-rp1-window-sentinel

## Goal

Run the accepted real RP1 PCIe endpoint/config/decode discriminator on Pi 5
and classify whether the PCIe2 host-link/status layer is observable before any
endpoint config-space work.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 real candidate work.
- Published only the accepted real candidate archive:
  target/talos-rpi5-rp1-pcie2-host-link-status-read-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage before accepting the rerun:
  candidate identity, fresh serial/TFTP evidence, known-good control, and
  unchanged candidate rerun.

## Non-Goals

No endpoint config-space access, PCIe writes, bridge setup, PERST/link-control
changes, MSI/MIP/GIC operations, RP1 peripheral/SYSINFO/clock/GPIO retry,
clock/reset writes, GPIO/RIO/pad writes, event generation, interrupt
enablement or delivery, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, or phase transition.

## Classification

Accepted as pcie2-host-link-up-rp1-window-sentinel.

The accepted candidate rerun selected boot tree
6d1fa1cd754adf38a023909651bcdc40b6ed08a06b559e79859f545886a59393 with
effective kernel_2712.img and a 46,880-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
46,880-byte candidate fetches, final pre-restore identity still matched the
selected tree, and restore returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The retained serial output contains 120 occurrences of
TALOS: rp1-pcie2-host-link-status-result. The accepted report reads
PCIE_MISC_PCIE_STATUS at 0x1000124068 as raw=0x3e0b0, with pcie-port=true,
dl-active=true, phylinkup=true, link-in-l23=false, status-is-deaddead=false,
retained-rp1-window-sentinel=true, and terminal classification
pcie2-host-link-up-rp1-window-sentinel.

This accepts only that the read-only PCIe2 host-link/status register is
visible and link-up while the retained RP1 SYSINFO/clock-window path remains
sentinel-shaped. It does not accept endpoint config-space access, broad RP1
mapping, endpoint ownership, PCIe writes, interrupt delivery, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or a phase
transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  candidate work.
- fixed: retained static archive identity for the accepted real candidate
  archive, including archive SHA-256, kernel SHA-256, marker string, report
  shape, and accepted PCIE_MISC_PCIE_STATUS address fields.
- fixed: retained the first candidate capture as capture-staging-blocked
  evidence; it cannot support a decisive classification because the retained
  identity join records staging/serial freshness rejection reasons.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate evidence; it retained two served 104,136-byte
  known-good kernel fetches and reached its PASS marker, but the identity join
  was rejected by non-empty pre-power serial drain.
- fixed: reran the unchanged real PCIE_MISC_PCIE_STATUS candidate after the
  known-good control; the rerun passed the v2 identity join and retained the
  repeated pcie2-host-link-up-rp1-window-sentinel output.
- deferred: endpoint config-space access, bridge setup, PERST/link control,
  MSI/MIP/GIC operations, interrupt delivery, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, and phase transition.
- not-an-issue: the retained RP1 SYSINFO/clock-window sentinel context remains
  true; this proof separates PCIe2 host-link visibility from that retained
  RP1-window sentinel but does not by itself accept endpoint ownership or
  config-space probing.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-pi5/candidate-rerun-after-kg/.
- First candidate capture:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-pi5/candidate-run/.
- Known-good control:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted
  candidate rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 46,880-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 120 occurrences of
  TALOS: rp1-pcie2-host-link-status-result were retained with classification
  pcie2-host-link-up-rp1-window-sentinel.
- known-good control and unchanged candidate rerun after inconclusive evidence:
  run and retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as pcie2-host-link-up-rp1-window-sentinel. The queued closeout is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored and supervisorIntervention remains inactive.
