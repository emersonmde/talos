# Talos Evidence Retention Policy and Bloat Audit

Task ID: talos-evidence-retention-policy-and-bloat-audit-20260525
Status: accepted

## Goal

Reduce tracked evidence bloat risk by defining what stays in Git, what is
summarized, and what raw lab artifacts should move out of the repository.

## Scope

- Inventoried tracked task evidence, target logs, boot archives, serial
  transcripts, and generated artifacts.
- Classified evidence into keep-in-Git summaries, lightweight fixtures,
  ignored/generated artifacts, and external artifact-storage candidates.
- Added a documented evidence retention policy.
- Preserved accepted task summaries and enough digest/classification data for
  reproducibility.

## Non-Goals

No accepted hardware evidence was deleted. No build artifacts, boot archives,
hardware runs, scheduler implementation, shared run queues, migration, load
balancing, multi-core preemption, or later roadmap behavior was changed.

## Inventory Findings

- git status --short was clean before edits.
- tasks/evidence has 1,622 tracked files totaling 33,043,091 bytes by Git
  object size and 37M on disk.
- target/ is ignored and currently uses 556M locally. Its largest ignored files
  are map files and Pi 5 boot archives/boot-tree images.
- Tracked evidence is mostly raw lab metadata: 1,268 JSON files, 297 TXT files,
  15 Markdown summaries, and 42 other digest/size/cursor artifacts.
- Largest tracked evidence directories by Git object size:
  - 2026-05-24-pi5-psci-secondary-core-alive-proof: 9,563,133 bytes across
    235 files.
  - 2026-05-25-pi5-cross-core-ipi-delivery-proof: 7,212,155 bytes across
    284 files.
  - 2026-05-24-pi5-smp-lock-cache-coherence-proof: 6,822,404 bytes across
    628 files.
  - 2026-05-25-pi5-secondary-cacheable-mmu-handoff-proof: 3,870,560 bytes
    across 26 files.
  - 2026-05-25-pi5-remote-wakeup-request-proof: 1,699,610 bytes across
    41 files.
- Largest individual tracked files are raw serial-peek or TFTP-delta captures,
  including multi-megabyte JSON/TXT files. These are the main cleanup
  candidates after summaries and digests are confirmed.

## Retention Classification

Keep in Git:

- tasks/*.md task records.
- tasks/evidence/**/summary.md evidence summaries.
- Small manifest, digest, cursor, restore, status, and classification files
  needed to prove candidate identity and lab cleanup.
- Short serial/QEMU excerpts that are direct validation gates.
- Docs, roadmap, architecture, and decision-log updates that explain accepted
  design.

Retain as lightweight fixtures:

- Compact QEMU transcripts or serial excerpts.
- Minimal parser/classifier fixtures.
- Bounded negative examples that guard against repeated boot-classification
  mistakes.

Ignore/generated:

- target/, book/, boot archives, boot trees, kernel images, map files, local
  smoke logs, tarballs, disk images, ELF files, temporary logs, and local
  OpenClaw state.

Move to external artifact storage when available:

- Raw serial captures larger than 256 KiB.
- Raw TFTP deltas larger than 256 KiB.
- Duplicate before/after restore captures once one compact excerpt, digest, and
  summary prove the same state.
- Full lab-controller JSON dumps whose useful fields have been distilled into
  accepted summaries.

## Cleanup Follow-Ups

- Queue talos-evidence-archive-large-raw-lab-artifacts-20260525 to archive or
  summarize raw tracked serial/TFTP captures larger than 256 KiB without
  weakening accepted hardware claims.
- Queue talos-evidence-summary-manifest-backfill-20260525 to backfill
  summary/manifest files for older evidence directories whose review currently
  depends on raw JSON/TXT captures.

## Validation

- Static inspection: git status --short before edits was clean.
- Static inventory: git ls-files size-oriented evidence inventory completed.
- Whitespace inspection: git diff --check passed.
- Documentation: mdbook build passed.

## Acceptance

Accepted as the evidence-retention policy and bloat audit. Tracked accepted
evidence remains in place; large raw-artifact cleanup is deferred to explicit
follow-up tasks.
