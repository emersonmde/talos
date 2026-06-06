# Phase 11 RP1 UART0 FR-Read Repaired-Cursor Closeout

Task id: phase11-rp1-uart0-fr-read-repaired-cursor-closeout-20260606

Status: accepted

## Goal

Close out the repaired-cursor RP1 UART0 flag-register Pi 5 rerun without
accepting RP1 mapping behavior or broadening the Phase 11 frontier.

## Scope

- Reconciled the completed repaired-cursor Pi 5 rerun evidence.
- Recorded the accepted and unaccepted claims from the selected
  rpi5_rp1_uart0_fr_read candidate.
- Identified the next non-repetitive discriminator needed before any further
  RP1 UART0 FR-read hardware rerun.
- Did not publish a boot archive, acquire hardwareTestLock, run hardware,
  change RP1 source, change RP1 constants, or accept GPIO, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  Milestone 11.2, or a phase transition.

## Findings And Disposition

- fixed: the selected candidate archive identity was retained as SHA-256
  da35a26e817fd30b81874a701171de1b9d47c47024d5fc405a7068ca3b2e5d60.
- fixed: the candidate boot publication was retained as tree
  25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71 with
  effective_kernel=kernel_2712.img and the expected 45,832-byte
  da591740/kernel_2712.img.
- fixed: stable same-cursor TFTP evidence from cursor 4129377 retained 13
  events, including two served candidate kernel fetches before restore.
- fixed: the repaired saturated-cursor serial path read from cursor 4194304
  and retained 4,470 fresh bytes of firmware NETWORK output from the
  candidate run.
- fixed: restore evidence returned the lab boot tree to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and
  hardwareTestLock was released/restored before completion.
- removed: the prior serial-cursor-saturation failure class is no longer the
  explanation for this RP1 FR-read run; the repaired direct-read path produced
  fresh serial bytes.
- not-an-issue: no known-good control was required in the repaired-cursor
  rerun because staging, candidate fetch, serial capture, stable TFTP evidence,
  and restore proof were all retained.
- deferred: the serial window did not show TALOS: kernel_main,
  rpi5-rp1-uart0-fr-read: start,
  rpi5-rp1-uart0-fr-read: pre-mmio-read, mapped/read-value, trap/hang
  boundary text, or PASS, so RP1 UART0 FR-read readiness and RP1
  mapped/unmapped behavior remain unaccepted.

## Closeout Classification

candidate-fetch-reset-loop-without-visible-fr-marker

The accepted boundary is limited to candidate archive identity, publication,
candidate kernel fetch visibility, repaired fresh serial capture of firmware
NETWORK output, and restore hygiene. This is blocker evidence, not an RP1
mapping proof.

The smallest non-repetitive next discriminator is not another same-shaped
RP1 UART0 FR-read rerun. Supervisor planning should first separate the
FR-read candidate's source/static entry path from the accepted Rust-entry
UART10 marker-loop path, for example with a no-MMIO FR-read-shaped marker
candidate or an equivalent static/binary discriminator that proves whether the
selected FR-read scenario reaches its pre-MMIO reporting path before the
volatile load is present. That discriminator must have explicit acceptance
criteria before any new hardware run.

## Accepted Claims

- The refreshed RP1 UART0 FR-read candidate archive was selected and published.
- The Pi 5 lab served the selected candidate kernel_2712.img twice before
  restore.
- The repaired saturated-cursor serial capture path retained fresh candidate
  power-cycle serial bytes.
- The lab boot tree was restored before hardware-lock release.

## Unaccepted Claims

- TALOS: kernel_main visibility for this candidate.
- rpi5-rp1-uart0-fr-read: start or
  rpi5-rp1-uart0-fr-read: pre-mmio-read visibility.
- RP1 UART0 FR mapped/read-value.
- RP1 UART0 FR unmapped/trap or hang-at-MMIO boundary.
- Firmware-state behavior beyond candidate fetch/reset-loop evidence.
- GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH,
  broader PCIe, Milestone 11.2, or a phase transition.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-closeout/evidence-map.json.
- Completed repaired-cursor rerun:
  tasks/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun.md.
- Completed repaired-cursor rerun evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-repaired-cursor-pi5-rerun/evidence-map.json.

## Validation

- static inspection of repaired-cursor rerun task records and evidence map:
  passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

Accepted with classification candidate-fetch-reset-loop-without-visible-fr-marker.

No queued follow-up task is mechanically unblocked by this closeout. Supervisor
planning is required for the next bounded, feature-led Phase 11 discriminator.
