# Phase 12 RP1 Ethernet Clock/Reset Read-Only Baseline Pi 5 Proof

Task id: phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-clock-reset-readonly-baseline-report-visibility-control-output
Evidence level: image/archive inspection, lab-controller API, serial hardware
boot/output, TFTP/capture evidence, capture-chain-v4 replay, and restore proof.

## Goal

Run a serialized Pi 5 read-only baseline proof for the accepted Ethernet
clock/reset guard without writes or ownership claims.

## Findings

- fixed: candidate archive review passed with nonce
  clock-reset-candidate-20260610T103100Z, archive sha256
  5f9cc0471df6ac0946361eb6d30b5c429bba3e63347f7eca95ca9cb08110f1b4,
  kernel sha256
  180ca39b3dbdf18f7780d2e55061fe5f2e9b634ccfdbaabd49b45311c1d6e539,
  and kernel_2712.img size 50056 bytes.
- fixed: control archive review passed with nonce
  clock-reset-control-rerun-20260610T105000Z, archive sha256
  054456e8f042b60c8bac87b10e57b76b388a605175717cafd4abca59f76a1385,
  kernel sha256
  33e7decdadfb6929a20c46c564a6940e2edfddb6b97a5ddf4be9214c2218bc21,
  and kernel_2712.img size 49176 bytes.
- fixed: candidate capture-chain-v4 joined selected tree
  047815dc8bfde65c28be5d4a5844eb5bf83c4dc60749d7a9c76c8dce402599c3,
  two matching TFTP fetches of da591740/kernel_2712.img at 50056 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: candidate serial retained 19 required marker occurrences and printed
  observed-window MACB_MID context 0x1c001000fc/raw 0x70109/idnum 0x7/rev
  0x109, pclk/hclk/tsu_clk/tx_clk names and IDs, shared RP1_CLK_SYS policy,
  Ethernet-private clock IDs, reset-controller no-target policy, GPIO32
  context, rejected claims, retained risks, and classification
  rp1-ethernet-clock-reset-readonly-baseline-report-visible.
- fixed: control capture-chain-v4 joined selected tree
  16745426bc0d0f1cc2b1844f48d6e656a8c900afb6fcca42caee5553afc7f4fd,
  two matching TFTP fetches of da591740/kernel_2712.img at 49176 bytes,
  run-unique serial nonce freshness, final pre-restore identity, and restore
  proof.
- fixed: control serial retained 25 required marker occurrences through the
  same report path while withholding candidate-only MACB_MID context and
  baseline clock/reset facts, with classification
  no-clock-reset-no-ethernet-rp1-ethernet-clock-reset-baseline-control.
- fixed: final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img and da591740/kernel_2712.img at 104136
  bytes.
- not-an-issue: both capture-bundle summaries retain the legacy v2
  serial-drain-not-empty-before-power suggestion, but capture-chain-v4 passed
  for the accepted candidate and control because run-unique nonce freshness,
  selected-tree identity, TFTP, final identity, and restore gates all matched.
- deferred: write-backed clock/reset ownership, exact register/restore
  semantics, GPIO32/PHY reset, MDIO/PHY, interrupts, DMA, descriptor rings,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
  future tasks.

No findings were removed.

## Hardware Result

Accepted result:
rp1-ethernet-clock-reset-readonly-baseline-report-visibility-control-output.

The candidate proves that the accepted read-only clock/reset baseline report is
visible on Pi 5 serial and retains the observed-window MACB_MID identity
context plus selected baseline fields. The paired control proves the same
capture/report path while withholding candidate-only clock/reset facts. This is
report visibility/control output only; it does not prove hardware ownership,
safe writes, reset behavior, packet I/O, or Ethernet driver readiness.

## Evidence

- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/capture-summary.json.
- Candidate run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/candidate-run/.
- Control run:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/control-run/.
- Archive reviews:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/archive-review/.
- Pre-run snapshot:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/pre-run-snapshot-create.json.
- Final restore:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof/final-post-restore-boot-files.json.

## Validation

- static inspection: accepted guard closeout, runtime scenarios, archive
  helpers, capture summaries, identity joins, and docs reviewed.
- fmt check: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet rp1_ethernet passed.
- shell syntax: bash -n on touched clock/reset baseline shell scripts passed.
- image/archive inspection: candidate and control review scripts passed.
- lab-controller API: hardwareTestLock acquired before publication; snapshot
  created and restored; final /boot/files confirmed restored tree.
- serial hardware output: candidate and control markers retained with
  run-unique nonces from direct-read serial windows.
- TFTP/capture evidence: candidate and control stable deltas both retained two
  expected da591740/kernel_2712.img fetches with matching bytes.
- capture-chain replay: candidate and control identity-join-v4 checks passed.
- JSON validation: jq empty on task-owned classification/evidence-map/
  capture-summary JSON passed.

## Next Action

No mechanically unblocked follow-up task is inferred here. Supervisor planning
is required for the next explicit Phase 12.1 clock/reset ownership slice.
