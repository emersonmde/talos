# Talos Roadmap Current Frontier Refresh

Task ID: talos-roadmap-current-frontier-refresh-20260525
Status: accepted

## Goal

Refresh the roadmap's top-level current-status wording so a reader sees the
actual Phase 6.3 frontier before the older Phase 4 and Phase 5 history.

## Scope

- Updated docs/src/roadmap.md to lead with the accepted Phase 6.3 scheduler
  facts: secondary production dispatch, cross-core IPI, remote wake,
  target-owned wake consumption, and shared scheduler metadata all have bounded
  QEMU and Pi 5 evidence.
- Made the diagnostic/productized boundary explicit: the accepted slices are
  validation gates and bounded scheduler capabilities, not a general
  multi-core runtime.
- Recorded that evidence-retention and diagnostic-surface audits are now
  accepted before the productionization boundary inventory.
- Kept later deferrals visible before Phase 7 and network-oriented roadmap
  work.

## Non-Goals

No code, boot images, hardware runs, roadmap expansion, shared run queues,
remote enqueue queues, task migration, load balancing, multi-core preemption,
userspace, filesystem, networking, SSH, or shell behavior were added.

## Evidence Reviewed

- docs/src/roadmap.md current status and Phase 6 status sections.
- docs/src/architecture/scheduler.md Phase 6.3 scheduler ownership,
  remote-wake, production secondary dispatch, and shared metadata contracts.
- docs/src/architecture/diagnostic-command-channel.md diagnostic-only command
  channel boundary.
- docs/src/project/phase6-shared-scheduler-metadata-closeout-checkpoint.md.
- docs/src/project/evidence-retention-policy.md.
- docs/src/project/diagnostic-surface-policy.md.
- docs/src/decisions/README.md latest accepted Phase 6.3 decision entries.

## Validation

- Static inspection: git status --short before edits was clean inside the
  Talos repo.
- Static review: roadmap, scheduler architecture, diagnostic command docs,
  evidence-retention policy, diagnostic-surface policy, shared metadata
  closeout, and decision log were reviewed.
- Whitespace inspection: git diff --check passed.
- Documentation: mdbook build passed.
- Rust fmt/tests and hardware runs were not required because this task changed
  only Markdown documentation and durable worker state.

## Acceptance

Accepted as a docs-only current frontier refresh. The next bounded task still
requires supervisor planning; the roadmap now points to productionization
boundary inventory rather than broad scheduler migration or later phases.
