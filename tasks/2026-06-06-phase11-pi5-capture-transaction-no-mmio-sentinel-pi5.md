# Phase 11 Pi 5 Capture Transaction No-MMIO Sentinel

Task id: phase11-pi5-capture-transaction-no-mmio-sentinel-pi5-20260606

Status: accepted

## Goal

Validate the repaired Pi 5 capture transaction with a selected no-RP1-MMIO
long-running marker candidate before returning to the RP1 UART0 FR-read
candidate.

## Scope

- Acquired the hardware lock for the no-MMIO sentinel task.
- Used the already accepted final-preload-marker hold archive without changing
  RP1 source or constants.
- Repaired the proof helpers after the first attempt exposed proof-contract
  issues in serial-drain capture and large evidence replay.
- Published only the selected no-MMIO sentinel archive, ran one clean rerun
  through the v2 capture transaction, restored the pre-run boot tree, and
  released the hardware lock.

## Classification

no-mmio-sentinel-identity-joined.

The accepted sentinel archive was
`target/talos-rpi5-rp1-final-preload-marker-hold-core.tar.gz`, SHA-256
`07af64b86908f36c63d368589d79c76aebd492a81906a39586a2c5902d8b9287`. The
selected tree was
`101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47` with a
45,816-byte `da591740/kernel_2712.img` and effective kernel
`kernel_2712.img`.

The clean rerun proved the v2 identity join: the pre-power `/serial/read`
drain was empty, stable same-cursor TFTP evidence retained two served
45,816-byte sentinel kernel fetches, final pre-restore identity still matched
the selected tree, and the saturated-cursor direct serial window retained 7,489
occurrences of `TALOS: fr-final-preload-hold-loop`. Restore returned the lab
to tree `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

This accepts proof-chain readiness for the no-MMIO sentinel only. It does not
accept RP1 mapped/read-value behavior, RP1 unmapped/trap behavior,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, or a phase transition.

## Findings And Disposition

- fixed: `scripts/rpi5-capture-invariant-proof-bundle.sh` now uses
  `/serial/read` for the pre-power drain; `/serial/peek` is retained-tail
  evidence and cannot prove an empty device buffer.
- fixed: `scripts/rpi5-proof-identity-join-check.sh` now loads the summary
  and serial-drain JSON through `jq --slurpfile`, avoiding command-line
  length failures on large retained serial evidence.
- fixed: the clean rerun retained selected sentinel identity, stable TFTP
  evidence, final pre-restore identity, empty serial drain, marker output, and
  restore proof under one v2 run label.
- removed: the first attempt's post-restore TFTP/final-identity samples are not
  accepted because manual restore occurred while the capture session was still
  running.
- deferred: the queued capture-transaction v2 closeout must decide whether this
  proof chain is ready for a subsequent RP1 FR-read v2 proof task.
- not-an-issue: no RP1 hardware behavior is claimed from the no-MMIO marker
  sentinel.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5/evidence-map.json`.
- Classification:
  `tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5/classification.json`.
- Clean rerun bundle:
  `tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5/sentinel-rerun/`.
- Compromised first attempt retained as blocker evidence:
  `tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-no-mmio-sentinel-pi5/sentinel-run/`.

## Validation

- serialized Pi 5 hardware run through lab-controller endpoints: passed on
  clean rerun.
- static archive identity check: passed against accepted archive SHA-256.
- capture-transaction v2 summary and identity checker output: passed with
  `capture-transaction-v2-ready` and no rejection reasons on the clean rerun.
- stable same-cursor TFTP evidence before restore: passed.
- restore proof before hardware lock release: passed.
- shell syntax checks for changed scripts: passed.
- dry-run contract check for changed capture helper: passed.
- mdbook build: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as no-mmio-sentinel-identity-joined. The next queued task is the
capture-transaction v2 closeout; it must stay within the current Phase 11
Milestone 11.1 proof-chain slice.
