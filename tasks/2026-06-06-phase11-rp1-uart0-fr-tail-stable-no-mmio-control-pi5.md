# Phase 11 RP1 UART0 FR Tail-Stable No-MMIO Control Pi 5

Task id: phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5-20260606

Status: accepted

## Goal

Run the serialized Pi 5 no-MMIO control for the tail-stable result-output
shape before any RP1 UART0 FR tail-stable result proof.

## Scope

- Acquired hardwareTestLock for the serialized Pi 5 control.
- Published only the accepted no-RP1-MMIO tail-stable control archive:
  target/talos-rpi5-rp1-uart0-fr-tail-stable-no-mmio-control-core.tar.gz.
- Retained static archive identity, publication identity, empty pre-power
  serial drain, v2 capture-transaction identity join, stable pre-restore TFTP
  evidence, final pre-restore identity, and restore proof.
- Performed the required triage after early inconclusive capture attempts:
  candidate identity, fresh serial cursor/drain evidence, TFTP delta,
  known-good control, and a candidate rerun.

## Classification

tail-stable-control-visible.

The accepted candidate rerun selected boot tree
`b4b780193281538a643aec3c17898ae59204c335f32452b90cf08b0cb8e10161` with
effective `kernel_2712.img` and a 45,728-byte
`da591740/kernel_2712.img`. The v2 identity join passed with no rejection
reasons: the pre-power `/serial/read` drain was empty, stable pre-restore
TFTP retained two served 45,728-byte candidate fetches, final pre-restore
identity still matched the selected tree, and restore returned the lab to
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

The saturated direct-read serial window retained 1,771 occurrences of
`TALOS: fr-tail-stable-control`, proving that the simulated/control
tail-stable result-output shape is capturable on Pi 5. This accepts only the
no-MMIO control output shape and proof-chain readiness for the queued RP1
tail-stable result proof. It does not accept RP1 UART0 FR mapped/read-value,
bus-fault/trap, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or a phase
transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around the serialized control
  run.
- fixed: retained static archive identity for the accepted no-MMIO control
  archive, including archive SHA-256, kernel SHA-256, image size, marker
  string, and absence of forbidden RP1 result strings.
- fixed: retained the first staging mismatch as non-feature evidence; the
  published lab tree hash is the selected hardware identity and may differ
  from the local archive tree hash while preserving kernel bytes and archive
  identity.
- fixed: retained an inconclusive candidate run and known-good control showing
  non-empty pre-power serial drain rejection under the v2 contract.
- fixed: reran the candidate after known-good control; the rerun passed the v2
  identity join and retained repeated `TALOS: fr-tail-stable-control` output.
- deferred: the RP1 UART0 FR tail-stable result proof remains queued and must
  pass its own hardware lock, v2 identity join, and classification gates.
- not-an-issue: no RP1 MMIO behavior is inferred from a no-MMIO simulated
  control.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/control-rerun-after-kg/.
- Known-good control:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/known-good-control-run/.
- Inconclusive candidate attempts retained as blocker/triage evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/control-run/ and
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/control-rerun/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted
  candidate rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 45,728-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 1,771 occurrences of
  `TALOS: fr-tail-stable-control` were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof before hardware-lock release: passed.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as tail-stable-control-visible. The queued RP1 tail-stable result
proof is mechanically unblocked on a future worker wake if hardwareTestLock
remains unlocked/restored and supervisorIntervention remains inactive.
