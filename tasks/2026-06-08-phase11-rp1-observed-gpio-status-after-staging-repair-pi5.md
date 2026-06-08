# Phase 11 RP1 Observed GPIO Status After Staging Repair Pi 5

Task id: phase11-rp1-observed-gpio-status-after-staging-repair-pi5-20260608

Status: accepted

Classification: observed-gpio14-status-ctrl-visible-after-staging-repair

## Goal

Retry the real observed GPIO14 STATUS/CTRL proof after the boot-staging identity
repair and known-good control made candidate identity decisive.

## Scope

- Acquired hardwareTestLock for this task only.
- Built and static-reviewed the real observed GPIO14 STATUS/CTRL archive with
  TALOS_CAPTURE_NONCE=bti-real-20260608T213533Z-122908e4.
- Published only the task-owned real observed GPIO14 STATUS/CTRL candidate
  archive.
- Captured selected-tree identity, fresh serial cursor/drain evidence, stable
  TFTP delta, final pre-restore identity, marker-visible serial output,
  run-unique checker output, boot-staging identity checker output, and restore
  proof.

## Findings And Disposition

- fixed: selected candidate identity before power matched tree
  5a499384497595de18d05f250fe146352d964953c9ff759642cc8d20384e0ea6 with
  effective kernel kernel_2712.img.
- fixed: stable TFTP evidence retained two
  da591740/kernel_2712.img fetches of 49,784 bytes, matching the selected
  candidate archive.
- fixed: final pre-restore identity still pointed at selected tree
  5a499384497595de18d05f250fe146352d964953c9ff759642cc8d20384e0ea6.
- fixed: serial hardware output retained 38 task-owned run-unique
  TALOS: rp1-observed-gpio-status-result records.
- fixed: observed GPIO14 STATUS/CTRL records reported
  gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, ctrl-funcsel=4, and
  classification=observed-aperture-gpio14-status-ctrl-visible.
- fixed: rpi5-proof-identity-join-run-unique-check.sh classified the retained
  bundle as capture-transaction-run-unique-ready with no rejection reasons.
- fixed: rpi5-boot-staging-identity-check.sh classified the retained bundle as
  boot-staging-identity-ready with no rejection reasons.
- fixed: restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- not-an-issue: the retained capture helper's v2 suggested classification is
  not the acceptance gate for this task; the accepted repair procedure requires
  the run-unique checker plus the boot-staging identity checker, and both
  passed.
- deferred: this task accepts read-only observed GPIO14 STATUS/CTRL visibility
  only. GPIO ownership, event generation, interrupt pending generation,
  interrupt delivery, GIC acknowledgement, endpoint ownership, broad RP1
  mapping, DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3,
  and phase transition remain unaccepted.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5/classification.json.
- Static archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5/static-archive-review.txt.
- Capture bundle:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5/real-run/.
- Run-unique checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5/run-unique-check.json.
- Boot-staging identity checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5/boot-staging-identity-check.json.
- Restore proof:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5/real-run/post-restore-status.json.

## Validation

- image/archive inspection: static review passed for the nonce-bearing real
  observed GPIO14 STATUS/CTRL archive.
- lab-controller API: retained snapshot, archive publish, staged identity,
  power-cycle, serial, TFTP, final identity, and restore records.
- serial hardware boot/output: passed; task-owned marker was visible after
  power and carried the expected run-unique nonce.
- TFTP evidence: passed; stable same-cursor pre-restore delta retained two
  matching 49,784-byte candidate kernel fetches.
- final pre-restore identity: passed; selected tree remained staged before
  restore.
- restore proof: passed; post-restore tree hash matched the pre-run baseline.
- run-unique checker: passed with no rejection reasons.
- boot-staging identity checker: passed with no rejection reasons.
- jq empty on evidence-map, classification, checker, and retained JSON
  artifacts: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

No docs/src files were touched, so mdbook was not required.

## Result

Accepted observed GPIO14 STATUS/CTRL visibility for the read-only STATUS/CTRL
candidate under the repaired boot-staging identity procedure. This does not
accept GPIO ownership, event generation, interrupt pending generation,
interrupt delivery, GIC acknowledgement, endpoint ownership, broad RP1 mapping,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or a phase
transition. The closeout task is mechanically unblocked after this accepted
proof is committed and hardwareTestLock remains unlocked/restored.
