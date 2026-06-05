# Phase 11 Staging/Capture Discriminator Closeout

Task id: phase11-staging-capture-discriminator-closeout-20260605

Status: accepted

## Goal

Close out the lab evidence discriminator and decide whether RP1 entry-control
candidate proof can be rerun or whether lab/staging remains blocked.

## Scope

- Reconciled the accepted lab evidence contract repair, read-only known-good
  API probe, and serialized known-good capture/staging Pi 5 discriminator.
- Distinguished proof semantics, known-good capture/staging health, and the
  remaining boot-runtime readiness blocker.
- Kept Phase 11 Milestone 11.1 boundaries explicit and did not promote any RP1
  candidate/source work.

## Non-Goals Honored

No runtime/kernel/RP1 source changes, boot archive publication, hardware run,
hardwareTestLock acquisition, GPIO ownership, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2 work, or phase
transition was performed. No RP1 candidate fetch, Rust entry, entry-control
reachability, RP1 mapped/read-value, RP1 unmapped/trap, or firmware-state
behavior is accepted by this closeout.

## Findings And Disposition

- fixed: the repaired proof semantics are accepted for future Pi 5 hardware
  bundles: use GET /status for boot identity, retain boot files/snapshots,
  collect fresh serial and TFTP cursors, and classify TFTP fetches only after
  stable pre-restore cursor replay.
- fixed: the known-good lab capture/staging path is no longer blocked by
  proof-time zero-event TFTP deltas. The accepted discriminator later observed
  13 stable pre-restore TFTP events from fresh cursor 4094251, including two
  served 104,136-byte da591740/kernel_2712.img fetches.
- deferred: Talos runtime readiness on the restored known-good tree remains
  unproven for this discriminator because serial reached Raspberry Pi
  firmware/RP1 boot output but did not reach TALOS: kernel_main,
  command-loop readiness, or PASS.
- deferred: RP1 candidate proof reuse remains blocked until supervisor plans a
  bounded next task that explains or repairs the known-good boot-runtime
  readiness gap after observed TFTP fetch.
- not-an-issue: active_name=kernel8.img in the read-only API probe is not a
  blocker because the accepted boot identity rule treats
  effective_kernel=kernel_2712.img as authoritative.
- removed: no alternate capture path, extra wait stack, candidate rerun, boot
  publication, source change, or hardware rerun was added in closeout.

## Evidence

- Closeout evidence map:
  tasks/evidence/2026-06-05-phase11-staging-capture-discriminator-closeout/evidence-map.json.
- Static evidence inspection:
  tasks/evidence/2026-06-05-phase11-staging-capture-discriminator-closeout/static-evidence-inspection.md.
- Diff hygiene:
  tasks/evidence/2026-06-05-phase11-staging-capture-discriminator-closeout/git-diff-check.log.
- Docs validation:
  tasks/evidence/2026-06-05-phase11-staging-capture-discriminator-closeout/mdbook-build.log.
- Staged diff hygiene:
  tasks/evidence/2026-06-05-phase11-staging-capture-discriminator-closeout/git-diff-cached-check.log.
- Contract repair task:
  tasks/2026-06-05-phase11-lab-evidence-contract-repair-core.md,
  commit a79b1f2d0a336077c957727e27b7f105a4f6ea29.
- Read-only API probe task:
  tasks/2026-06-05-phase11-known-good-boot-state-api-probe.md, commit
  6e9ca49ca66bfbfe1582a414dabb9e368e711fbf.
- Serialized known-good discriminator task:
  tasks/2026-06-05-phase11-known-good-capture-staging-pi5-discriminator.md,
  commit 9fea6a52fe6072aafa846926e24fe1ad5c2f25de.

## Validation

- static evidence inspection: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed with the existing large
  search-index warning.
- git diff --cached --check before commit: passed.

## Result

Accepted closeout with classification
known-good-capture-staging-accepted-runtime-readiness-blocked.

The lab capture/staging evidence path is accepted for known-good fetch
visibility under the repaired stable-log rule. RP1 candidate/source work
remains blocked because the known-good tree did not reach Talos runtime
readiness after observed TFTP fetch. Supervisor planning is required for the
next bounded Phase 11 slice; the worker must not infer an RP1 candidate rerun,
source-level handoff change, Milestone 11.2 transition, networking, SSH, GPIO,
interrupt, DMA/cache, storage, generated-root, or broader PCIe task.
