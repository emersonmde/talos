# Talos Review: Docs, Scripts, Evidence Hygiene

Task: talos-review-docs-scripts-evidence-hygiene-20260603
Status: accepted
Started: 2026-06-03T06:51:16Z
Completed: 2026-06-03T06:56:30Z

## Scope

Reviewed docs, scripts, task records, retained evidence references, and
hardware evidence-helper wording after the VFS/loader/userspace review.

This task did not implement new OS feature behavior and did not touch Pi 5
hardware.

## Findings

- Fixed: the previous VFS/loader/userspace review task record had a Completed
  timestamp after the supervisor acceptance evidence and recorded only the
  implementation commit. That made the durable task record disagree with state.
  Disposition: corrected the timestamp to the accepted completion time and
  recorded the acceptance/state commit.

- Fixed: scripts/rpi5-tftp-cursor.sh captured the TFTP cursor by sending a
  synthetic large cursor even though the current lab-controller contract says
  omitted cursor is tail mode and returns the authoritative cursor_end. That
  was an evidence-collection footgun for future hardware runs.
  Disposition: changed the helper to omit cursor and retain max_bytes/limit.

- Fixed: docs still described the older cursor=0 workaround as an equivalent
  pre-run cursor capture path. That conflicted with the current lab-controller
  documentation and supervisor hardware evidence rules.
  Disposition: updated reference notes and the decision-log entry to require
  omitted-cursor tail mode, while preserving the historical reason the helper
  exists.

- Not an issue: retained tasks/evidence artifacts are large and include stale
  tails and blocked-run records, but they are referenced by accepted task
  records and hardware blockers. This review did not remove evidence files
  without an explicit retention owner decision.

- Not an issue: the many per-scenario RPi5 image/boot-tree helpers are
  repetitive, but they encode accepted proof archive names and target cfg
  boundaries. A mechanical generator would be a larger tooling refactor better
  saved for the full-system review cycles if it starts blocking feature work.

## Validation

- Static inspection: rg/static review of scripts/, docs/src/project reference
  notes, docs/src/project/lab-controller.md, docs/src/decisions/README.md,
  recent 2026-06-03 review task records, and TFTP cursor references.
- shell syntax: sh -n scripts/rpi5-tftp-cursor.sh and
  scripts/rpi5-wait-tftp-delta.sh passed.
- lab-controller API: scripts/rpi5-tftp-cursor.sh returned tail cursor
  4077950 using omitted-cursor tail mode.
- static check: rg found no active cursor=999999999 or
  cursor=0&max_bytes=1048576&limit=1 guidance in scripts, reference notes, or
  decision-log active guidance.
- docs validation: /home/node/.cargo/bin/mdbook build passed; mdbook warned
  that the search index is large.
- diff hygiene: git diff --check and git diff --cached --check passed.

## Remaining Risks

- Historical task and evidence records still mention old cursor values and
  older collection behavior as facts from those runs; this task updated active
  guidance and helper behavior only.

## Commit

- Review implementation commit: d01201669b7d1bcc43f68a37b197eabc3976dae2
