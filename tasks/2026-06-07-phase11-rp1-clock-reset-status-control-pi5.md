# Phase 11 RP1 Clock/Reset Status No-MMIO Control Pi 5

Task id: phase11-rp1-clock-reset-status-control-pi5-20260607

Status: accepted

## Goal

Run the paired no-MMIO/no-RP1/no-GIC clock/reset/status control candidate on
Pi 5 to prove the output, capture, identity, TFTP, and restore path before any
real RP1 clock manager status diagnostic proof.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Checked the accepted local/static control archive before publication:
  target/talos-rpi5-rp1-clock-manager-status-no-mmio-control-core.tar.gz.
- Published only the accepted no-MMIO/no-RP1/no-GIC clock/reset/status control
  archive.
- Retained publication identity, fresh serial/TFTP cursors, serial capture,
  stable pre-restore TFTP evidence, final pre-restore identity, restore
  evidence, and the pi5-capture-transaction-v2 identity join.

## Non-Goals

No real RP1 clock manager status diagnostic candidate, RP1 clock/reset/GPIO/RIO
/pads/MSI-X/PCIe/MIP/GIC MMIO access, clock/reset writes, GPIO ownership,
event generation, interrupt delivery, GIC acknowledgement, ISR installation,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, phase transition, or RP1 clock/reset/status
hardware behavior acceptance.

## Classification

Accepted as no-mmio-clock-reset-status-control-visible.

The Pi 5 selected boot tree
eeb71c0bfc3cbd259a18c5f53403555628a5cf8f3273d764cab80656087dbb66 with
effective kernel_2712.img and a 47,120-byte da591740/kernel_2712.img. The
identity join passed with no rejection reasons: pre-power serial drain was
empty, stable pre-restore TFTP retained two served 47,120-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
serial capture retained 49 occurrences of
TALOS: rp1-clock-manager-status-control.

The retained control output classification remains simulated/control. This
accepts only the no-MMIO/no-RP1/no-GIC output-shape and capture path for the
queued real clock manager status diagnostic proof. Real RP1 clock manager
status behavior, clock/reset ownership or writes, GPIO ownership, event
generation, interrupt delivery, GIC acknowledgement, ISR/handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe behavior,
Milestone 11.3, and phase transition remain unaccepted.

The capture helper restored the pre-run snapshot
pre-clock-reset-status-control-183334, returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 control
  work.
- fixed: retained static archive identity for the accepted no-MMIO/no-RP1/no-GIC
  control archive, including archive SHA-256, kernel SHA-256, marker string,
  and forbidden real diagnostic string absence.
- fixed: retained publication identity, selected tree, effective kernel, and
  expected fetch byte count before power cycling.
- fixed: retained fresh serial/TFTP cursors, empty pre-power serial drain,
  stable same-cursor TFTP delta before restore, final pre-restore identity, and
  restore proof.
- fixed: pi5-capture-transaction-v2 identity join passed with no rejection
  reasons and 49 retained control markers.
- deferred: the real RP1 clock manager status diagnostic proof remains queued
  and must pass its own hardware lock, identity join, and classification gates.
- not-an-issue: no RP1 clock/reset/status behavior is inferred from a no-MMIO
  simulated control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-control-pi5/classification.json.
- Accepted control run:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-control-pi5/control-run/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller API: passed for preflight status, publication, power cycle,
  post-publish identity, final pre-restore identity, snapshot restore, and
  post-restore identity.
- serial hardware boot/output: passed; 49 occurrences of
  TALOS: rp1-clock-manager-status-control were retained.
- stable same-cursor TFTP evidence before restore: passed; two 47,120-byte
  candidate kernel fetches were retained.
- pi5-capture-transaction-v2 identity join: passed with no rejection reasons.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- known-good control and candidate rerun: not run; the first candidate run was
  decisive.
- git diff --check: passed.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-clock-reset-status-control-visible. The queued real RP1
clock manager status diagnostic proof is mechanically unblocked on a future
worker wake if hardwareTestLock remains unlocked/restored and
supervisorIntervention remains inactive.
