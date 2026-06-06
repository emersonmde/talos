# Phase 11 Known-Good Serial Cursor Completeness Closeout

Task id: phase11-known-good-serial-cursor-completeness-closeout-20260606

Status: accepted

## Goal

Close out the repaired serial cursor/capture path before any future RP1 UART0
FR-read rerun is considered.

## Scope

- Reconciled the serial cursor saturation repair core with the serialized
  known-good Pi 5 proof.
- Recorded the proof boundary for future RP1 UART0 FR-read hardware reruns:
  a saturated saved cursor must use the repaired direct-read serial window, and
  proof evidence must still retain selected boot identity, fresh serial/TFTP
  cursors, stable pre-restore TFTP evidence, restore proof, and exact RP1
  classification text.
- Did not publish a boot archive, acquire hardwareTestLock, run hardware,
  change RP1 source, or accept RP1 mapped/read-value, unmapped/trap,
  firmware-state, GPIO, interrupts, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.2, or a phase transition.

## Findings And Disposition

- fixed: the repair-core task identified the prior RP1 FR-read blocker as a
  cursor-4194304 serial capture saturation class, not as evidence about RP1
  UART0 mapping.
- fixed: the serial-window helper now records
  observe_contract=deadline-loop-direct-read-after-saturated-cursor and
  capture_mode=read when the saved cursor is saturated.
- fixed: the known-good Pi 5 proof selected the restored accepted tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective_kernel=kernel_2712.img and the 104,136-byte
  da591740/kernel_2712.img known-good fetch.
- fixed: direct /serial/read from saturated cursor 4194304 retained 6,347 fresh
  bytes, including firmware NETWORK output and
  rpi5-production-timer-preemption: PASS, proving fresh Talos serial bytes can
  still be observed after the prior saturation boundary.
- fixed: stable same-cursor TFTP evidence before restore retained 13 events,
  including two served known-good kernel_2712.img fetches.
- fixed: restore evidence returned the lab to the selected known-good tree and
  hardwareTestLock was released/restored before task completion.
- not-an-issue: the known-good serial window omitted TALOS: kernel_main; for
  this closeout, the downstream production-timer PASS marker is sufficient to
  prove fresh Talos serial capture after saturation and does not broaden RP1
  acceptance.
- removed: repeating the same saturated-cursor /serial/observe path is no
  longer an acceptable discriminator for RP1 FR-read proofs.
- deferred: a future RP1 UART0 FR-read rerun still needs an explicit supervisor
  task and must not relax mapped/read-value, trap/unmapped, firmware-state, or
  pre-MMIO reachability acceptance criteria.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-known-good-serial-cursor-completeness-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-known-good-serial-cursor-completeness-closeout/evidence-map.json.
- Repair core:
  tasks/2026-06-06-phase11-serial-cursor-saturation-repair-core.md.
- Known-good Pi 5 proof:
  tasks/2026-06-06-phase11-known-good-serial-cursor-completeness-pi5.md.

## Validation

- static inspection of repair-core and known-good Pi 5 proof records: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

Accepted with classification serial-cursor-capture-completeness-accepted.

The repaired proof path can support a future RP1 UART0 FR-read rerun without
repeating the cursor-4194304 saturation failure class, but only under an
explicit queued task. That rerun must use the repaired saturated-cursor
direct-read path and stable TFTP/restore rules, and it still must independently
prove any RP1 UART0 FR-read classification.
