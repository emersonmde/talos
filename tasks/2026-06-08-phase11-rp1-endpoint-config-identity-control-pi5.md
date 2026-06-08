# Phase 11 RP1 Endpoint Config Identity Control Pi 5

Task id: phase11-rp1-endpoint-config-identity-control-pi5-20260608

Status: accepted

Classification: no-mmio-rp1-endpoint-config-id-control-visible

## Goal

Prove the paired no-MMIO/no-RP1/no-GIC endpoint config identity control output
shape is visible on Pi 5 before the real endpoint config identity proof.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Published only the accepted no-MMIO/no-RP1/no-GIC control archive:
  target/talos-rpi5-rp1-endpoint-config-identity-no-mmio-control-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after the first candidate
  capture was rejected as capture-staging evidence: candidate identity, fresh
  serial/TFTP evidence, known-good control, and unchanged candidate rerun.

## Non-Goals

No real endpoint config behavior, endpoint ownership, endpoint configuration
mutation, BAR programming, bridge setup, PERST or link-control changes,
interrupt enablement or delivery, GIC acknowledgement, ISR installation, RP1
clock/reset writes, GPIO/RIO/pad writes, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or phase transition.

## Classification

Accepted as no-mmio-rp1-endpoint-config-id-control-visible.

The accepted candidate rerun selected boot tree
def5349a2a9d4a323457f473e0371e0ed1b19a52b418df516634cfc001b88280 with
effective kernel_2712.img and a 47,608-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
47,608-byte candidate fetches, final pre-restore identity still matched the
selected tree, and serial hardware output retained 66 occurrences of
TALOS: rp1-endpoint-config-identity-control.

The retained control output classification remains
no-mmio-rp1-endpoint-config-id-control-visible. This accepts only the
no-MMIO/no-RP1/no-GIC output shape and capture path for the queued real
endpoint config identity proof. Real endpoint config behavior, broad RP1
mapping, endpoint ownership, endpoint configuration mutation, bridge setup,
interrupt delivery, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, and phase transition remain unaccepted.

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
  evidence; overlapping capture attempts produced concatenated serial JSON, so
  no decisive classification was taken from that run.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate evidence; it retained two served 104,136-byte
  known-good kernel fetches but was itself rejected by the identity join because
  the saturated serial drain did not empty before power.
- fixed: reran the same no-MMIO/no-RP1/no-GIC control candidate after the
  known-good control; the rerun passed the v2 identity join and retained
  repeated endpoint config identity control output.
- deferred: the real endpoint config identity proof remains queued and must
  pass its own hardware lock, identity join, restore, and classification gates.
- not-an-issue: no real endpoint config, RP1 aperture, interrupt, or DMA
  behavior is inferred from a no-MMIO simulated control.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-control-pi5/control-rerun-after-kg/.
- Initial candidate capture:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-control-pi5/control-run-concurrent-capture-corrupt/.
- Known-good control:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-control-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,608-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 66 occurrences of
  TALOS: rp1-endpoint-config-identity-control were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-rp1-endpoint-config-id-control-visible. The queued real
endpoint config identity proof is mechanically unblocked on a future worker
wake if hardwareTestLock remains unlocked/restored and supervisorIntervention
remains inactive.
