# Phase 11 Known-Good Runtime Marker-Boundary Static Inspection

Task id: phase11-known-good-runtime-marker-boundary-review-core-20260606

## Boundary Reviewed

The reviewed boundary is whether the restored known-good control can accept
Talos runtime readiness when fresh serial evidence contains
`rpi5-production-timer-preemption: PASS` but omits the earlier
`TALOS: kernel_main` marker.

## Source Order

- `src/main.rs:240-265`: the non-RP1-entry-control Pi 5 path writes
  `TALOS: rust_entry`, parses boot info, initializes the target, initializes
  exceptions, then calls `kernel_main(&boot_info)`.
- `src/boot/rpi5.rs:29-83`: Pi 5 `kernel_main` writes
  `EarlyPhaseLine::KernelMain` before boot identity, memory setup, DTB reports,
  and the `rpi5_production_timer_preemption_proof` call.
- `src/target/rpi5.rs:8844-8927`: `EarlyPhaseLine::KernelMain` emits the
  literal `TALOS: kernel_main` through early UART byte writes.
- `src/target/rpi5.rs:6628-6818`: the production-timer proof prints its
  start/report/final lines and prints `rpi5-production-timer-preemption: PASS`
  only after all production-timer report predicates are true.

Together these references prove the downstream PASS marker is reachable only
after the normal known-good Talos runtime has entered `kernel_main` and run the
production-timer proof path.

## Evidence Inspection

- `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/tftp-delta-stable-pre-restore.json`:
  stable pre-restore TFTP replay contains two served
  `da591740/kernel_2712.img` fetches at 104,136 bytes for the restored
  known-good tree.
- `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/serial-readiness-observe.json`:
  fresh serial cursor 4096748 produced a 6,746-byte accumulated window, did not
  contain `TALOS: kernel_main`, and did contain
  `rpi5-production-timer-preemption: PASS`.
- `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/serial-observe-direct-large-after-manual.json`:
  the direct large observe matches the same 6,746-byte cursor window and the same
  marker mismatch.
- `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/post-restore-status.json`
  and `post-restore-boot-files.json`: the task restored the known-good boot
  tree after proof collection.

## Findings And Disposition

- fixed: the runtime-readiness boundary is narrowed for the restored known-good
  production-timer control. A fresh serial window with the downstream
  `rpi5-production-timer-preemption: PASS` marker is sufficient even if the
  earlier `TALOS: kernel_main` marker is absent, because source order makes PASS
  unreachable before `kernel_main`.
- fixed: docs now distinguish the preferred full-marker proof from the accepted
  downstream-marker fallback for this known-good control.
- not-an-issue: the missing `TALOS: kernel_main` text is a serial-window
  completeness limitation, not a runtime-readiness blocker, when the same fresh
  window contains the downstream PASS marker and TFTP/restore identity are
  accepted.
- deferred: this does not accept RP1 candidate fetch, RP1 Rust entry,
  entry-control reachability, mapped/read-value, unmapped/trap, or firmware
  state behavior.

## Classification

`valid-known-good-talos-readiness-by-downstream-marker`

The next bounded task is the marker-boundary closeout. It should reconcile this
classification and only then decide whether the existing RP1 entry-control
candidate rerun is mechanically eligible.
