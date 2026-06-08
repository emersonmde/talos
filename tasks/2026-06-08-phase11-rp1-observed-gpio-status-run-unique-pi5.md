# Phase 11 RP1 Observed GPIO Status Run-Unique Pi 5

Task id: phase11-rp1-observed-gpio-status-run-unique-pi5-20260608

Status: completed-blocker

Classification: capture-staging-blocked

## Goal

Run the real observed GPIO14 STATUS/CTRL candidate with the accepted
run-unique capture discriminator.

## Scope

- Acquired hardwareTestLock for this task only.
- Built and static-reviewed the real observed GPIO14 STATUS/CTRL archive with
  TALOS_CAPTURE_NONCE=ru20260608T2012Z-f84941d7.
- Published only the real observed GPIO14 STATUS/CTRL archive and captured
  selected-tree identity, pre-power serial drain, serial output, stable TFTP
  delta, final identity, restore proof, and run-unique checker output.
- Attempted one clean same-shaped retry with
  TALOS_CAPTURE_NONCE=ru20260608T2025Z-f84941d7 after the first capture was
  rejected; it also failed the run-unique identity/freshness join and restored
  the lab.

## Findings And Disposition

- fixed: hardwareTestLock was held for the serialized Pi 5 work and the lab was
  restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: the primary run retained a task-owned run-unique marker absent before
  power and visible after power.
- fixed: serial output retained 41
  TALOS: rp1-observed-gpio-status-result records with capture nonce
  ru20260608T2012Z-f84941d7, gpio14-status-raw=0xabe3300,
  gpio14-ctrl-raw=0x84, ctrl-funcsel=4, and terminal
  classification=observed-aperture-gpio14-status-ctrl-visible.
- fixed: the run-unique checker rejected the run for
  tftp-expected-fetch-byte-mismatch, final-pre-restore-selected-tree-mismatch,
  and final-pre-restore-expected-fetch-byte-mismatch.
- fixed: stable TFTP evidence for the primary run showed two
  da591740/kernel_2712.img fetches of 104,136 bytes, while the selected
  candidate tree expected 49,776-byte fetches.
- fixed: final pre-restore identity pointed at the restored baseline tree
  instead of the selected candidate tree.
- fixed: a same-shaped retry with a fresh nonce drained 1,095,168 bytes without
  reaching empty-read-before-power, did not observe the required nonce marker
  after power, saw two 104,136-byte baseline TFTP fetches, and restored the lab.
- not-an-issue: marker-visible GPIO14 STATUS/CTRL serial text is retained as
  evidence but is not accepted as hardware visibility because the identity join
  failed.
- deferred: another same-shaped real retry requires supervisor planning or a
  repaired capture/staging procedure that explains the TFTP/final identity
  mismatch and saturated serial freshness failure.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-pi5/classification.json.
- Run-unique checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-pi5/run-unique-check.json.
- Primary candidate bundle:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-pi5/real-run/.
- Final clean retry blocker and checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-pi5/real-run-final/.

## Validation

- image/archive inspection: passed for both nonce-bearing real candidate
  archives.
- lab-controller API: retained publish, staged identity, power-cycle, final
  identity, and restore evidence for the primary and clean retry runs.
- serial hardware boot/output: marker-visible output retained, but not accepted
  as decisive hardware classification.
- TFTP evidence: stable delta retained; rejected because the observed fetch
  bytes matched the baseline known-good tree, not the selected candidate tree.
- run-unique checker: ran and rejected both the primary and clean retry runs as
  capture-staging-blocked.
- jq empty on classification, evidence map, run-unique checker, and retained
  primary JSON artifacts: passed.
- git diff --check: passed.
- git diff --cached --check: passed.

No docs/src files were touched, so mdbook was not required.

## Result

Completed as a committed blocker. This task does not accept GPIO14 STATUS/CTRL
visibility, GPIO ownership, event generation, interrupt delivery, broad RP1
mapping, DMA/cache, networking, SSH, Milestone 11.3, or a phase transition.
The run-unique closeout task is mechanically unblocked after this blocker is
committed and hardwareTestLock remains unlocked/restored.
