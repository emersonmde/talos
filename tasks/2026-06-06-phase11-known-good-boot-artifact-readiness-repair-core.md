# Phase 11 Known-Good Boot Artifact Readiness Repair Core

Task id: phase11-known-good-boot-artifact-readiness-repair-core-20260606

Status: accepted

## Goal

Repair or decisively classify why the restored accepted known-good boot tree
fetches `kernel_2712.img` but does not reach Talos runtime readiness markers.

## Scope

- Performed no-hardware source, artifact, boot-file, and evidence-lineage
  inspection against the accepted direct-cursor blocker.
- Reconciled the restored known-good tree hash, effective kernel, selected
  `da591740/kernel_2712.img` size, boot config, prior accepted runtime-ready
  control evidence, and latest direct-cursor missing-readiness evidence.
- Reviewed the local production-timer boot archive and image strings without
  publishing or running hardware.
- Preserved the Phase 11 Milestone 11.1 boundary and the distinction between
  known-good runtime readiness and RP1 behavior.

## Non-Goals Honored

No Pi 5 hardware run, boot archive publication, power cycle,
hardwareTestLock acquisition, RP1 candidate rerun, RP1 source/MMIO change, new
RP1 constants, GPIO ownership, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2 work, or phase transition was
performed. This task does not accept candidate fetch, Rust entry,
entry-control reachability, RP1 mapped/read-value, unmapped/trap, or
firmware-state behavior.

## Findings And Disposition

- fixed: current/restored boot identity is still explicit. The lab reports
  tree hash
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`,
  `configured_kernel=kernel_2712.img`, `effective_kernel=kernel_2712.img`,
  and a 104,136-byte `da591740/kernel_2712.img`.
- fixed: selected-artifact lineage is tied to the accepted prior and latest
  evidence. The latest direct-cursor run fetched the same 104,136-byte
  prefixed kernel twice, while prior same-tree evidence reached
  `TALOS: kernel_main` and
  `rpi5-production-timer-preemption: PASS`.
- fixed: local archive/static image review found no build/archive/staging
  defect. The production-timer archive has the expected root and serial-
  prefixed mirrors, `kernel_2712.img` selection, matching `kernel8.img`,
  valid ARM64 Image header fields, and production-timer PASS marker strings.
- deferred: no source or artifact fix is justified from no-hardware evidence.
  The latest direct-cursor serial stopped in Raspberry Pi firmware/RP1 output
  before `TALOS: kernel_main`, while the same restored tree has prior PASS
  evidence.
- deferred: valid known-good Talos runtime readiness remains unaccepted. RP1
  entry-control candidate/source work remains blocked.
- removed: no extra waits, probes, alternate capture path, workaround script,
  boot publication, hardware rerun, RP1 source change, or phase transition was
  added.
- not-an-issue: `active_name=kernel8.img` in lab status is the configured
  fallback name; `effective_kernel=kernel_2712.img` remains the selected boot
  identity under the documented lab-controller contract.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/evidence-map.json`.
- Static artifact/evidence inspection:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/static-artifact-evidence-inspection.md`.
- Diff hygiene:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/git-diff-check.log`.
- Staged diff hygiene:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/git-diff-cached-check.log`.
- Lab API reads:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/lab-status-read.json` and
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/boot-files-read.json`.
- Archive review:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/production-timer-archive-review.log`.
- Local image/marker inspection:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/local-artifact-inspection.log`.
- Serial comparison:
  `tasks/evidence/2026-06-06-phase11-known-good-boot-artifact-readiness-repair-core/serial-comparison.log`.

## Validation

- static source/artifact/evidence inspection: passed.
- lab-controller API read: retained status and boot file listings without
  hardware lock, power cycle, restore, or publication.
- image/archive inspection: `scripts/rpi5-archive-review.sh` passed for
  `target/talos-rpi5-production-timer-preemption-boot.tar.gz`.
- cargo fmt/test: not run; no Rust runtime/source files were touched.
- archive/script dry run: not run; no build/archive/staging scripts were
  touched.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: not run; no `docs/src` files were
  touched and the accepted readiness boundary did not change.
- git diff --cached --check before commit: passed.

## Result

Accepted classification: `no-actionable-source-artifact-defect`.

The no-hardware repair task found no source, artifact-selection, boot-config,
archive, or staging mismatch that explains the latest missing readiness. The
blocker remains: known-good fetch is visible, but the latest serial window
stopped before Talos runtime readiness markers. The smallest next
discriminator is supervisor-planned serialized known-good runtime rerun or lab
firmware/serial observation discrimination. RP1 entry-control candidate/source
work remains blocked until valid known-good Talos readiness is accepted.
