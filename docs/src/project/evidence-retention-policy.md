# Evidence Retention Policy

Talos keeps enough evidence in Git to make accepted work reviewable without
turning the repository into raw lab artifact storage. Task records and evidence
summaries are the durable source of truth; raw captures are retained only when
they are compact, uniquely decisive, or needed as a validation fixture.

## Current Inventory

The 2026-05-25 audit found:

- 1,622 tracked files under tasks/evidence, totaling 33,043,091 bytes by Git
  object size and 37M on disk.
- 556M of ignored target output, including boot archives, boot trees, map
  files, and build artifacts.
- The largest tracked evidence directories are Pi 5 PSCI secondary-core alive
  proof at 9,563,133 bytes, Pi 5 cross-core IPI delivery proof at 7,212,155
  bytes, the first Pi 5 SMP lock/cache-coherence proof at 6,822,404 bytes,
  and Pi 5 secondary cacheable-MMU handoff proof at 3,870,560 bytes.
- The largest tracked files are raw serial or TFTP captures, including
  multi-megabyte serial-peek JSON/TXT files and large TFTP delta JSON files.
- Existing .gitignore rules already keep target/, book/, logs, boot archives,
  disk images, ELF files, and local OpenClaw state out of Git.

## Keep In Git

Keep these artifacts in Git by default:

- Task records under tasks/*.md.
- Evidence summary.md files that state classification, acceptance status,
  archive and kernel digests, serial/TFTP source paths, and final PASS/FAIL
  lines.
- Small manifest, digest, size, cursor, restore, and status files that are
  needed to prove candidate identity and lab cleanup.
- Short serial or QEMU excerpts when the excerpt itself is the validation gate.
- Documentation pages, roadmap updates, architecture updates, and decision-log
  entries that explain the durable design outcome.

## Lightweight Fixtures

Retain compact fixtures in Git when they are actively useful for regression
review or deterministic tests:

- Canonical QEMU transcripts or serial excerpts that are small enough to review
  directly.
- Minimal JSON fixtures for parsers or lab-controller classification logic.
- Bounded negative examples that prevent repeating a known boot-classification
  mistake.

When a fixture grows beyond a concise review surface, replace it with a smaller
fixture plus a digest and summary of the original raw artifact.

## Generated Or Ignored

Do not commit generated artifacts unless a later task explicitly justifies an
exception:

- target/ contents, including map files, boot trees, boot archives, kernel
  images, and local smoke logs.
- book/ mdBook output.
- Local lab scratch files, temporary logs, tarballs, disk images, ELF files,
  OpenClaw state, or machine-local notes.

If a generated artifact matters for acceptance, record its SHA-256, byte size,
build command, and final classification in the task record or evidence summary.

## External Artifact Storage

Large raw lab captures should move out of Git once the accepted summary and
digests are complete. This includes:

- Raw serial captures larger than 256 KiB.
- Raw TFTP deltas larger than 256 KiB.
- Duplicate before/after restore captures where one compact excerpt proves the
  same state.
- Full lab-controller JSON dumps whose useful content has already been
  distilled into a summary, manifest, cursor, or digest file.

Until external artifact storage is available, keep existing accepted raw
evidence in place. Do not delete tracked accepted evidence as part of unrelated
feature work.

## Cleanup Rule

Cleanup must be explicit and reviewable:

- Generated ignored files may be removed locally when they are not needed for
  the active task.
- Tracked accepted evidence may be replaced only by a bounded cleanup task that
  preserves the task summary, classification, digests, artifact identity, and a
  pointer to external storage when applicable.
- If a cleanup would make an accepted hardware claim harder to audit, queue a
  follow-up instead of deleting the file.

The first cleanup follow-ups should archive or summarize the largest raw Pi 5
captures and backfill summary manifests for older evidence directories that
currently rely on raw JSON/TXT files for review.
